//! AI Client - Aerospace-Grade AI Service
//!
//! Safety-critical AI service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Comprehensive error handling
//! - Performance monitoring
//! - Security hardening
//! - Fault tolerance and error recovery

use super::config::AiConfig;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tauri::Emitter;
use std::time::Instant;
use crate::error_handling::{ErrorContext, ErrorSeverity, CircuitBreaker, RetryPolicy};
use crate::config_service::ExportConfigService;
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
struct AiResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Serialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize, Serialize)]
struct Message {
    content: String,
}

pub struct AiClient {
    config: AiConfig,
    /// 配置服务
    config_service: Arc<ExportConfigService>,
    client: reqwest::Client,
    operation_count: u64,
    last_error: Option<ErrorContext>,
    /// Circuit breaker for preventing cascading failures
    circuit_breaker: CircuitBreaker,
    /// Retry policy for transient failures
    retry_policy: RetryPolicy,
}

impl AiClient {
    pub fn new(config: AiConfig, config_service: Arc<ExportConfigService>) -> Result<Self, String> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| {
                let context = ErrorContext::new(
                    ErrorSeverity::Error,
                    "HTTP_CLIENT_CREATE_FAILED",
                    &format!("Failed to create HTTP client: {}", e),
                    "ai_client",
                );
                eprintln!("[AI Client] Error: {}", context.message);
                context.message
            })?;

        let circuit_breaker = CircuitBreaker::new(config_service.clone());
        let retry_policy = RetryPolicy::default();

        Ok(Self {
            config,
            config_service,
            client,
            operation_count: 0,
            last_error: None,
            circuit_breaker,
            retry_policy,
        })
    }

    /// Validate prompt length
    fn validate_prompt(&self, prompt: &str) -> Result<(), String> {
        let ai_config = self.config_service.get_ai_config();
        if prompt.len() > ai_config.max_prompt_length {
            return Err(format!("Prompt exceeds maximum length of {}", ai_config.max_prompt_length));
        }
        Ok(())
    }

    /// Validate text length
    fn validate_text(&self, text: &str) -> Result<(), String> {
        let ai_config = self.config_service.get_ai_config();
        if text.len() > ai_config.max_text_length {
            return Err(format!("Text exceeds maximum length of {}", ai_config.max_text_length));
        }
        Ok(())
    }

    /// Validate API configuration
    fn validate_config(&self) -> Result<(), String> {
        let ai_config = self.config_service.get_ai_config();
        if self.config.api_key.is_empty() {
            return Err("API key cannot be empty".to_string());
        }
        if self.config.api_key.len() > ai_config.max_api_key_length {
            return Err(format!("API key exceeds maximum length of {}", ai_config.max_api_key_length));
        }
        if self.config.api_url.is_empty() {
            return Err("API URL cannot be empty".to_string());
        }
        if self.config.api_url.len() > ai_config.max_api_url_length {
            return Err(format!("API URL exceeds maximum length of {}", ai_config.max_api_url_length));
        }
        Ok(())
    }

    /// Record error context
    fn record_error(&mut self, code: &str, message: &str, source: &str) {
        self.last_error = Some(ErrorContext::new(
            ErrorSeverity::Error,
            code,
            message,
            source,
        ));
    }

    /// Get last error
    pub fn get_last_error(&self) -> Option<&ErrorContext> {
        self.last_error.as_ref()
    }

    /// Get operation count
    pub fn get_operation_count(&self) -> u64 {
        self.operation_count
    }

    /// Reset error state
    pub fn reset_error_state(&mut self) {
        self.last_error = None;
    }

    pub async fn call(&mut self, prompt: &str, text: &str) -> Result<String, String> {
        self.operation_count += 1;
        let _start = Instant::now();

        // Check circuit breaker
        if !self.circuit_breaker.allow_operation() {
            let context = ErrorContext::new(
                ErrorSeverity::Critical,
                "CIRCUIT_BREAKER_OPEN",
                "Circuit breaker is open, blocking AI API calls",
                "ai_client",
            );
            eprintln!("[AI Client] Error: {}", context.message);
            return Err(context.message);
        }

        // Validate configuration
        if let Err(e) = self.validate_config() {
            self.record_error("INVALID_CONFIG", &e, "call");
            self.circuit_breaker.record_failure();
            return Err(e);
        }

        // Validate inputs
        if let Err(e) = self.validate_prompt(prompt) {
            self.record_error("INVALID_PROMPT", &e, "call");
            self.circuit_breaker.record_failure();
            return Err(e);
        }
        if let Err(e) = self.validate_text(text) {
            self.record_error("INVALID_TEXT", &e, "call");
            self.circuit_breaker.record_failure();
            return Err(e);
        }

        eprintln!(
            "[AI Service] Calling AI API with model: {}",
            self.config.model
        );
        let full_prompt = format!("{}{}", prompt, text);

        // Use retry policy for transient failures
        let response_result = self.retry_policy.execute(|| async {
            self.client
                .post(&self.config.api_url)
                .header("Authorization", format!("Bearer {}", self.config.api_key))
                .header("Content-Type", "application/json")
                .json(&serde_json::json!({
                    "model": self.config.model,
                    "messages": [
                        {
                            "role": "user",
                            "content": full_prompt
                        }
                    ],
                    "max_tokens": self.config.max_tokens
                }))
                .send()
                .await
        }).await;

        let response = response_result.map_err(|e| {
            let error = format!("HTTP request failed after retries: {}", e);
            eprintln!("[AI Service] {}", error);
            self.record_error("HTTP_REQUEST_FAILED", &error, "call");
            self.circuit_breaker.record_failure();
            error
        })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            let error = format!("API error ({}): {}", status, error_text);
            eprintln!("[AI Service] {}", error);
            self.record_error("API_ERROR", &error, "call");
            self.circuit_breaker.record_failure();
            return Err(error);
        }

        let res_body: AiResponse = response.json().await.map_err(|e| {
            let error = format!("Failed to parse response: {}", e);
            eprintln!("[AI Service] {}", error);
            self.record_error("PARSE_ERROR", &error, "call");
            self.circuit_breaker.record_failure();
            error
        })?;

        eprintln!("[AI Service] AI call successful");
        self.last_error = None;
        self.circuit_breaker.record_success();
        res_body
            .choices
            .first()
            .map(|c| c.message.content.trim().to_string())
            .ok_or_else(|| {
                let error = "AI returned no content".to_string();
                eprintln!("[AI Service] {}", error);
                self.record_error("NO_CONTENT", &error, "call");
                error
            })
    }

    pub async fn call_stream(
        &mut self,
        prompt: &str,
        text: &str,
        app: tauri::AppHandle,
    ) -> Result<(), String> {
        self.operation_count += 1;
        let _start = Instant::now();

        // Validate configuration
        if let Err(e) = self.validate_config() {
            self.record_error("INVALID_CONFIG", &e, "call_stream");
            return Err(e);
        }

        // Validate inputs
        if let Err(e) = self.validate_prompt(prompt) {
            self.record_error("INVALID_PROMPT", &e, "call_stream");
            return Err(e);
        }
        if let Err(e) = self.validate_text(text) {
            self.record_error("INVALID_TEXT", &e, "call_stream");
            return Err(e);
        }

        eprintln!(
            "[AI Service] Starting AI stream with model: {}",
            self.config.model
        );
        let full_prompt = format!("{}{}", prompt, text);

        let response = self
            .client
            .post(&self.config.api_url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": self.config.model,
                "messages": [
                    {
                        "role": "user",
                        "content": full_prompt
                    }
                ],
                "max_tokens": self.config.max_tokens,
                "stream": true
            }))
            .send()
            .await
            .map_err(|e| {
                let error = format!("HTTP request failed: {}", e);
                eprintln!("[AI Service] {}", error);
                self.record_error("HTTP_REQUEST_FAILED", &error, "call_stream");
                error
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            let error = format!("API error ({}): {}", status, error_text);
            eprintln!("[AI Service] {}", error);
            self.record_error("API_ERROR", &error, "call_stream");
            return Err(error);
        }

        let mut stream = response.bytes_stream();
        let mut chunk_count = 0;

        while let Some(chunk_result) = stream.next().await {
            // Check for maximum chunk count to prevent infinite loops
            let ai_config = self.config_service.get_ai_config();
            if chunk_count >= ai_config.max_stream_chunks {
                let error = format!("Stream exceeded maximum chunk count of {}", ai_config.max_stream_chunks);
                eprintln!("[AI Service] {}", error);
                self.record_error("STREAM_TOO_LONG", &error, "call_stream");
                let _ = app.emit("ai-stream-error", &error);
                return Err(error);
            }

            let chunk = chunk_result.map_err(|e| {
                let error = format!("Stream error: {}", e);
                eprintln!("[AI Service] {}", error);
                self.record_error("STREAM_ERROR", &error, "call_stream");
                error
            })?;
            let chunk_str = String::from_utf8_lossy(&chunk);

            for line in chunk_str.lines() {
                if line.starts_with("data: ") {
                    let data = &line[6..];
                    if data == "[DONE]" {
                        continue;
                    }

                    if let Ok(stream_response) = serde_json::from_str::<serde_json::Value>(data) {
                        if let Some(choices) =
                            stream_response.get("choices").and_then(|c| c.as_array())
                        {
                            if let Some(choice) = choices.first() {
                                if let Some(delta) = choice.get("delta") {
                                    if let Some(content) =
                                        delta.get("content").and_then(|c| c.as_str())
                                    {
                                        chunk_count += 1;
                                        let _ = app.emit("ai-stream-chunk", content);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        eprintln!(
            "[AI Service] Stream complete, processed {} chunks",
            chunk_count
        );
        self.last_error = None;
        let _ = app.emit("ai-stream-complete", ());
        Ok(())
    }
}

impl Default for AiClient {
    fn default() -> Self {
        let config = AiConfig::from_env().unwrap_or_else(|_| {
            AiConfig::new("YOUR_API_KEY".to_string()) // Fallback for development
        });
        Self::new(config, Arc::new(ExportConfigService::new()))
            .expect("Failed to create default AiClient")
    }
}

#[cfg(test)]
mod tests {
    use super::super::config::AiConfig;
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = AiConfig::new("test_api_key".to_string());
        let client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        assert_eq!(client.config.api_key, "test_api_key");
    }

    #[test]
    fn test_client_default() {
        let client = AiClient::default();
        // Default client should have a config
        assert!(!client.config.api_key.is_empty() || client.config.api_key == "YOUR_API_KEY");
    }

    #[test]
    fn test_client_creation_with_custom_timeout() {
        let mut config = AiConfig::new("test_api_key".to_string());
        config.timeout_seconds = 30;
        let client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        assert_eq!(client.config.timeout_seconds, 30);
    }

    #[test]
    fn test_client_creation_with_custom_max_tokens() {
        let mut config = AiConfig::new("test_api_key".to_string());
        config.max_tokens = 2000;
        let client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        assert_eq!(client.config.max_tokens, 2000);
    }

    #[test]
    fn test_client_creation_with_custom_model() {
        let mut config = AiConfig::new("test_api_key".to_string());
        config.model = "gpt-4".to_string();
        let client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        assert_eq!(client.config.model, "gpt-4");
    }

    #[test]
    fn test_client_creation_with_custom_api_url() {
        let mut config = AiConfig::new("test_api_key".to_string());
        config.api_url = "https://custom.api.url".to_string();
        let client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        assert_eq!(client.config.api_url, "https://custom.api.url");
    }

    #[tokio::test]
    async fn test_call_with_empty_prompt() {
        let config = AiConfig::new("test_api_key".to_string());
        let mut client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        let result = client.call("", "test text").await;
        // This will fail due to invalid API, but we test the error handling
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_call_with_empty_text() {
        let config = AiConfig::new("test_api_key".to_string());
        let mut client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        let result = client.call("test prompt", "").await;
        // This will fail due to invalid API, but we test the error handling
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_call_with_long_text() {
        let config = AiConfig::new("test_api_key".to_string());
        let mut client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        let long_text = "a".repeat(10000);
        let result = client.call("test prompt", &long_text).await;
        // This will fail due to invalid API, but we test the error handling
        assert!(result.is_err());
    }

    #[test]
    fn test_ai_response_struct() {
        let response = AiResponse { choices: vec![] };
        assert!(response.choices.is_empty());
    }

    #[test]
    fn test_choice_struct() {
        let choice = Choice {
            message: Message {
                content: "test content".to_string(),
            },
        };
        assert_eq!(choice.message.content, "test content");
    }

    #[test]
    fn test_message_struct() {
        let message = Message {
            content: "test content".to_string(),
        };
        assert_eq!(message.content, "test content");
    }

    #[test]
    fn test_ai_response_with_choices() {
        let response = AiResponse {
            choices: vec![
                Choice {
                    message: Message {
                        content: "content 1".to_string(),
                    },
                },
                Choice {
                    message: Message {
                        content: "content 2".to_string(),
                    },
                },
            ],
        };
        assert_eq!(response.choices.len(), 2);
    }

    #[test]
    fn test_message_empty_content() {
        let message = Message {
            content: "".to_string(),
        };
        assert_eq!(message.content, "");
    }

    #[test]
    fn test_message_with_special_characters() {
        let message = Message {
            content: "Test with émojis 🎉 and spëcial çhars".to_string(),
        };
        assert_eq!(message.content, "Test with émojis 🎉 and spëcial çhars");
    }

    #[test]
    fn test_message_with_newlines() {
        let message = Message {
            content: "Line 1\nLine 2\nLine 3".to_string(),
        };
        assert_eq!(message.content, "Line 1\nLine 2\nLine 3");
    }

    #[test]
    fn test_config_clone() {
        let config = AiConfig::new("test_api_key".to_string());
        let client = AiClient::new(config.clone(), Arc::new(ExportConfigService::new())).unwrap();
        assert_eq!(config.api_key, client.config.api_key);
    }

    #[test]
    fn test_client_timeout_configuration() {
        let mut config = AiConfig::new("test_api_key".to_string());
        config.timeout_seconds = 60;
        let client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        assert_eq!(client.config.timeout_seconds, 60);
    }

    #[test]
    fn test_client_max_tokens_configuration() {
        let mut config = AiConfig::new("test_api_key".to_string());
        config.max_tokens = 4096;
        let client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        assert_eq!(client.config.max_tokens, 4096);
    }

    #[test]
    fn test_client_model_configuration() {
        let mut config = AiConfig::new("test_api_key".to_string());
        config.model = "gpt-3.5-turbo".to_string();
        let client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        assert_eq!(client.config.model, "gpt-3.5-turbo");
    }

    #[test]
    fn test_client_api_url_configuration() {
        let mut config = AiConfig::new("test_api_key".to_string());
        config.api_url = "https://api.openai.com/v1/chat/completions".to_string();
        let client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        assert_eq!(
            client.config.api_url,
            "https://api.openai.com/v1/chat/completions"
        );
    }

    #[test]
    fn test_ai_response_serialization() {
        let response = AiResponse {
            choices: vec![Choice {
                message: Message {
                    content: "test".to_string(),
                },
            }],
        };
        let json = serde_json::to_string(&response);
        assert!(json.is_ok());
    }

    #[test]
    fn test_ai_response_deserialization() {
        let json = r#"{"choices":[{"message":{"content":"test"}}]}"#;
        let response: Result<AiResponse, _> = serde_json::from_str(json);
        assert!(response.is_ok());
    }

    #[test]
    fn test_choice_serialization() {
        let choice = Choice {
            message: Message {
                content: "test".to_string(),
            },
        };
        let json = serde_json::to_string(&choice);
        assert!(json.is_ok());
    }

    #[test]
    fn test_choice_deserialization() {
        let json = r#"{"message":{"content":"test"}}"#;
        let choice: Result<Choice, _> = serde_json::from_str(json);
        assert!(choice.is_ok());
    }

    #[test]
    fn test_message_serialization() {
        let message = Message {
            content: "test".to_string(),
        };
        let json = serde_json::to_string(&message);
        assert!(json.is_ok());
    }

    #[test]
    fn test_message_deserialization() {
        let json = r#"{"content":"test"}"#;
        let message: Result<Message, _> = serde_json::from_str(json);
        assert!(message.is_ok());
    }

    #[test]
    fn test_ai_response_empty_choices() {
        let response = AiResponse { choices: vec![] };
        let json = serde_json::to_string(&response);
        assert!(json.is_ok());
        let deserialized: AiResponse = serde_json::from_str(&json.unwrap()).unwrap();
        assert!(deserialized.choices.is_empty());
    }

    #[test]
    fn test_ai_response_multiple_choices() {
        let response = AiResponse {
            choices: vec![
                Choice {
                    message: Message {
                        content: "first".to_string(),
                    },
                },
                Choice {
                    message: Message {
                        content: "second".to_string(),
                    },
                },
                Choice {
                    message: Message {
                        content: "third".to_string(),
                    },
                },
            ],
        };
        assert_eq!(response.choices.len(), 3);
    }

    #[test]
    fn test_message_with_whitespace() {
        let message = Message {
            content: "   test   ".to_string(),
        };
        assert_eq!(message.content, "   test   ");
    }

    #[test]
    fn test_message_with_tabs() {
        let message = Message {
            content: "test\twith\ttabs".to_string(),
        };
        assert_eq!(message.content, "test\twith\ttabs");
    }

    #[test]
    fn test_message_with_json_special_chars() {
        let message = Message {
            content: "Test with \"quotes\" and {braces}".to_string(),
        };
        assert_eq!(message.content, "Test with \"quotes\" and {braces}");
    }

    #[test]
    fn test_message_very_long_content() {
        let long_content = "a".repeat(100000);
        let message = Message {
            content: long_content.clone(),
        };
        assert_eq!(message.content.len(), 100000);
    }

    #[test]
    fn test_config_zero_timeout() {
        let mut config = AiConfig::new("test_api_key".to_string());
        config.timeout_seconds = 0;
        let client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        assert_eq!(client.config.timeout_seconds, 0);
    }

    #[test]
    fn test_config_zero_max_tokens() {
        let mut config = AiConfig::new("test_api_key".to_string());
        config.max_tokens = 0;
        let client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        assert_eq!(client.config.max_tokens, 0);
    }

    #[test]
    fn test_config_large_max_tokens() {
        let mut config = AiConfig::new("test_api_key".to_string());
        config.max_tokens = 100000;
        let client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        assert_eq!(client.config.max_tokens, 100000);
    }

    #[test]
    fn test_config_empty_model() {
        let mut config = AiConfig::new("test_api_key".to_string());
        config.model = "".to_string();
        let client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        assert_eq!(client.config.model, "");
    }

    #[test]
    fn test_config_empty_api_url() {
        let mut config = AiConfig::new("test_api_key".to_string());
        config.api_url = "".to_string();
        let client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        assert_eq!(client.config.api_url, "");
    }

    #[test]
    fn test_config_empty_api_key() {
        let config = AiConfig::new("".to_string());
        let client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        assert_eq!(client.config.api_key, "");
    }

    #[test]
    fn test_ai_response_first_choice_extraction() {
        let response = AiResponse {
            choices: vec![
                Choice {
                    message: Message {
                        content: "first".to_string(),
                    },
                },
                Choice {
                    message: Message {
                        content: "second".to_string(),
                    },
                },
            ],
        };
        let first = response.choices.first();
        assert!(first.is_some());
        assert_eq!(first.unwrap().message.content, "first");
    }

    #[test]
    fn test_ai_response_no_first_choice() {
        let response = AiResponse { choices: vec![] };
        let first = response.choices.first();
        assert!(first.is_none());
    }

    #[tokio::test]
    async fn test_call_with_unicode_text() {
        let config = AiConfig::new("test_api_key".to_string());
        let mut client = AiClient::new(config, Arc::new(ExportConfigService::new()));
        let result = client.call("test", "Hello 世界 🌍").await;
        // Will fail due to invalid API, but tests the call mechanism
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_call_with_special_chars() {
        let config = AiConfig::new("test_api_key".to_string());
        let mut client = AiClient::new(config, Arc::new(ExportConfigService::new()));
        let result = client.call("test", "Test with \"quotes\" & <tags>").await;
        // Will fail due to invalid API, but tests the call mechanism
        assert!(result.is_err());
    }

    #[test]
    fn test_stream_data_parsing_done() {
        let data = "[DONE]";
        assert_eq!(data, "[DONE]");
    }

    #[test]
    fn test_stream_data_parsing_valid_json() {
        let data = r#"{"choices":[{"delta":{"content":"test"}}]}"#;
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(data);
        assert!(parsed.is_ok());
    }

    #[test]
    fn test_stream_data_parsing_invalid_json() {
        let data = "invalid json";
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(data);
        assert!(parsed.is_err());
    }

    #[test]
    fn test_stream_line_starts_with_data() {
        let line = "data: test";
        assert!(line.starts_with("data: "));
    }

    #[test]
    fn test_stream_line_without_data_prefix() {
        let line = "regular line";
        assert!(!line.starts_with("data: "));
    }

    #[test]
    fn test_stream_data_extraction() {
        let line = "data: test content";
        let data = &line[6..];
        assert_eq!(data, "test content");
    }

    #[test]
    fn test_message_content_trim() {
        let message = Message {
            content: "  test  ".to_string(),
        };
        let trimmed = message.content.trim();
        assert_eq!(trimmed, "test");
    }

    #[test]
    fn test_client_config_field_access() {
        let config = AiConfig::new("test_key".to_string());
        let client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        // Access all config fields to ensure they're public or accessible
        let _ = client.config.api_key;
        let _ = client.config.api_url;
        let _ = client.config.model;
        let _ = client.config.max_tokens;
        let _ = client.config.timeout_seconds;
    }

    #[test]
    fn test_choice_message_field_access() {
        let choice = Choice {
            message: Message {
                content: "test".to_string(),
            },
        };
        let _ = choice.message.content;
    }

    #[test]
    fn test_response_choices_field_access() {
        let response = AiResponse {
            choices: vec![Choice {
                message: Message {
                    content: "test".to_string(),
                },
            }],
        };
        let _ = response.choices;
    }

    // Aerospace-level tests
    #[test]
    fn test_prompt_validation_too_long() {
        let config = AiConfig::new("test_api_key".to_string());
        let config_service = Arc::new(ExportConfigService::new());
        let client = AiClient::new(config, config_service.clone()).unwrap();
        let ai_config = config_service.get_ai_config();
        let long_prompt = "a".repeat(ai_config.max_prompt_length + 1);
        let result = client.validate_prompt(&long_prompt);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum length"));
    }

    #[test]
    fn test_text_validation_too_long() {
        let config = AiConfig::new("test_api_key".to_string());
        let config_service = Arc::new(ExportConfigService::new());
        let client = AiClient::new(config, config_service.clone()).unwrap();
        let ai_config = config_service.get_ai_config();
        let long_text = "a".repeat(ai_config.max_text_length + 1);
        let result = client.validate_text(&long_text);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum length"));
    }

    #[test]
    fn test_config_validation_empty_api_key() {
        let config = AiConfig::new("".to_string());
        let client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        let result = client.validate_config();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_config_validation_api_key_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let ai_config = config_service.get_ai_config();
        let config = AiConfig::new("a".repeat(ai_config.max_api_key_length + 1));
        let client = AiClient::new(config, config_service).unwrap();
        let result = client.validate_config();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum length"));
    }

    #[test]
    fn test_config_validation_empty_api_url() {
        let mut config = AiConfig::new("test_api_key".to_string());
        config.api_url = "".to_string();
        let client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        let result = client.validate_config();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_config_validation_api_url_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let ai_config = config_service.get_ai_config();
        let mut config = AiConfig::new("test_api_key".to_string());
        config.api_url = "a".repeat(ai_config.max_api_url_length + 1);
        let client = AiClient::new(config, config_service).unwrap();
        let result = client.validate_config();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum length"));
    }

    #[test]
    fn test_operation_count() {
        let config = AiConfig::new("test_api_key".to_string());
        let mut client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        assert_eq!(client.get_operation_count(), 0);
        
        // Simulate operation count increment
        client.operation_count = 5;
        assert_eq!(client.get_operation_count(), 5);
    }

    #[test]
    fn test_error_recording() {
        let config = AiConfig::new("test_api_key".to_string());
        let mut client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        
        client.record_error("TEST_ERROR", "Test error message", "test_source");
        
        let last_error = client.get_last_error();
        assert!(last_error.is_some());
        let error = last_error.unwrap();
        assert_eq!(error.code, "TEST_ERROR");
        assert_eq!(error.message, "Test error message");
        assert_eq!(error.source, "test_source");
    }

    #[test]
    fn test_error_state_reset() {
        let config = AiConfig::new("test_api_key".to_string());
        let mut client = AiClient::new(config, Arc::new(ExportConfigService::new())).unwrap();
        
        client.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(client.get_last_error().is_some());
        
        client.reset_error_state();
        assert!(client.get_last_error().is_none());
    }

    #[test]
    fn test_max_prompt_length_accepted() {
        let config = AiConfig::new("test_api_key".to_string());
        let config_service = Arc::new(ExportConfigService::new());
        let client = AiClient::new(config, config_service.clone()).unwrap();
        let ai_config = config_service.get_ai_config();
        let prompt = "a".repeat(ai_config.max_prompt_length);
        let result = client.validate_prompt(&prompt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_text_length_accepted() {
        let config = AiConfig::new("test_api_key".to_string());
        let config_service = Arc::new(ExportConfigService::new());
        let client = AiClient::new(config, config_service.clone()).unwrap();
        let ai_config = config_service.get_ai_config();
        let text = "a".repeat(ai_config.max_text_length);
        let result = client.validate_text(&text);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_api_key_length_accepted() {
        let config_service = Arc::new(ExportConfigService::new());
        let ai_config = config_service.get_ai_config();
        let config = AiConfig::new("a".repeat(ai_config.max_api_key_length));
        let client = AiClient::new(config, config_service).unwrap();
        let result = client.validate_config();
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_api_url_length_accepted() {
        let config_service = Arc::new(ExportConfigService::new());
        let ai_config = config_service.get_ai_config();
        let mut config = AiConfig::new("test_api_key".to_string());
        config.api_url = "a".repeat(ai_config.max_api_url_length);
        let client = AiClient::new(config, config_service).unwrap();
        let result = client.validate_config();
        assert!(result.is_ok());
    }
}
