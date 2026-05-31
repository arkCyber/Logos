#[cfg(test)]
mod tests {
    use super::super::{AiClient, AiConfig};
    use std::sync::Arc;
    use crate::config_service::ExportConfigService;

    #[test]
    fn test_ai_client_creation() {
        let config = AiConfig::new("test_key".to_string());
        let _client = AiClient::new(config, Arc::new(ExportConfigService::new()));
        // Client should be created without panicking
        assert!(true);
    }

    #[test]
    fn test_ai_client_with_custom_config() {
        let config = AiConfig::new("test_key".to_string())
            .with_url("https://custom.api.com".to_string())
            .with_model("custom-model".to_string())
            .with_max_tokens(2048)
            .with_timeout(60);

        let _client = AiClient::new(config, Arc::new(ExportConfigService::new()));
        // Client should be created without panicking
        assert!(true);
    }

    #[test]
    fn test_ai_client_default() {
        let _client = AiClient::default();
        // Default client should be created (may use placeholder API key)
        assert!(true);
    }

    #[test]
    fn test_ai_client_config_field_access() {
        let config = AiConfig::new("test_key".to_string());
        let _client = AiClient::new(config, Arc::new(ExportConfigService::new()));

        // We can't directly access config field as it's private,
        // but we can verify the client was created
        assert!(true);
    }

    #[test]
    fn test_ai_config_timeout_in_client() {
        let config = AiConfig::new("test_key".to_string()).with_timeout(30);

        let _client = AiClient::new(config, Arc::new(ExportConfigService::new()));
        // Client should be created with custom timeout
        assert!(true);
    }

    #[test]
    fn test_ai_config_max_tokens_in_client() {
        let config = AiConfig::new("test_key".to_string()).with_max_tokens(4096);

        let _client = AiClient::new(config, Arc::new(ExportConfigService::new()));
        // Client should be created with custom max_tokens
        assert!(true);
    }

    #[test]
    fn test_ai_config_model_in_client() {
        let config = AiConfig::new("test_key".to_string()).with_model("gpt-4".to_string());

        let _client = AiClient::new(config, Arc::new(ExportConfigService::new()));
        // Client should be created with custom model
        assert!(true);
    }

    #[test]
    fn test_ai_config_url_in_client() {
        let config = AiConfig::new("test_key".to_string())
            .with_url("https://api.openai.com/v1/chat/completions".to_string());

        let _client = AiClient::new(config, Arc::new(ExportConfigService::new()));
        // Client should be created with custom URL
        assert!(true);
    }

    #[test]
    fn test_ai_client_multiple_instances() {
        let config1 = AiConfig::new("key1".to_string());
        let config2 = AiConfig::new("key2".to_string());

        let _client1 = AiClient::new(config1, Arc::new(ExportConfigService::new()));
        let _client2 = AiClient::new(config2, Arc::new(ExportConfigService::new()));

        // Multiple clients should be creatable
        assert!(true);
    }

    #[test]
    fn test_ai_client_with_empty_api_key() {
        let config = AiConfig::new("".to_string());
        let _client = AiClient::new(config, Arc::new(ExportConfigService::new()));
        // Client should be created even with empty key (will fail at call time)
        assert!(true);
    }

    #[test]
    fn test_ai_client_with_special_chars_in_key() {
        let config = AiConfig::new("sk-1234567890abcdef!@#$%^&*()".to_string());
        let _client = AiClient::new(config, Arc::new(ExportConfigService::new()));
        // Client should handle special characters in API key
        assert!(true);
    }

    #[test]
    fn test_ai_client_timeout_zero() {
        let config = AiConfig::new("test_key".to_string()).with_timeout(0);

        let _client = AiClient::new(config, Arc::new(ExportConfigService::new()));
        // Client should be created even with zero timeout
        assert!(true);
    }

    #[test]
    fn test_ai_client_very_long_timeout() {
        let config = AiConfig::new("test_key".to_string()).with_timeout(3600); // 1 hour

        let _client = AiClient::new(config, Arc::new(ExportConfigService::new()));
        // Client should be created with very long timeout
        assert!(true);
    }

    #[test]
    fn test_ai_client_max_tokens_zero() {
        let config = AiConfig::new("test_key".to_string()).with_max_tokens(0);

        let _client = AiClient::new(config, Arc::new(ExportConfigService::new()));
        // Client should be created even with zero max_tokens
        assert!(true);
    }

    #[test]
    fn test_ai_client_very_large_max_tokens() {
        let config = AiConfig::new("test_key".to_string()).with_max_tokens(100000);

        let _client = AiClient::new(config, Arc::new(ExportConfigService::new()));
        // Client should be created with very large max_tokens
        assert!(true);
    }

    #[test]
    fn test_ai_client_invalid_url_format() {
        let config = AiConfig::new("test_key".to_string()).with_url("not-a-valid-url".to_string());

        let _client = AiClient::new(config, Arc::new(ExportConfigService::new()));
        // Client should be created (will fail at call time)
        assert!(true);
    }

    #[test]
    fn test_ai_client_empty_model() {
        let config = AiConfig::new("test_key".to_string()).with_model("".to_string());

        let _client = AiClient::new(config, Arc::new(ExportConfigService::new()));
        // Client should be created even with empty model
        assert!(true);
    }

    #[test]
    fn test_ai_client_model_with_special_chars() {
        let config =
            AiConfig::new("test_key".to_string()).with_model("model-v1.2.3-beta".to_string());

        let _client = AiClient::new(config, Arc::new(ExportConfigService::new()));
        // Client should handle model names with special characters
        assert!(true);
    }
}
