use std::env;

#[derive(Debug, Clone)]
pub struct AiConfig {
    pub api_key: String,
    pub api_url: String,
    pub model: String,
    pub max_tokens: u32,
    pub timeout_seconds: u64,
}

impl AiConfig {
    pub fn from_env() -> Result<Self, String> {
        let api_key = env::var("DEEPSEEK_API_KEY")
            .or_else(|_| env::var("AI_API_KEY"))
            .map_err(|_| "AI_API_KEY environment variable not set".to_string())?;

        Ok(Self {
            api_key,
            api_url: env::var("AI_API_URL")
                .unwrap_or_else(|_| "https://api.deepseek.com/v1/chat/completions".to_string()),
            model: env::var("AI_MODEL").unwrap_or_else(|_| "deepseek-chat".to_string()),
            max_tokens: env::var("AI_MAX_TOKENS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1024),
            timeout_seconds: env::var("AI_TIMEOUT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
        })
    }

    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            api_url: "https://api.deepseek.com/v1/chat/completions".to_string(),
            model: "deepseek-chat".to_string(),
            max_tokens: 1024,
            timeout_seconds: 30,
        }
    }

    #[allow(dead_code)]
    pub fn with_url(mut self, url: String) -> Self {
        self.api_url = url;
        self
    }

    #[allow(dead_code)]
    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    #[allow(dead_code)]
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    #[allow(dead_code)]
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = timeout_seconds;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = AiConfig::new("test_key".to_string());
        assert_eq!(config.api_key, "test_key");
        assert_eq!(
            config.api_url,
            "https://api.deepseek.com/v1/chat/completions"
        );
        assert_eq!(config.model, "deepseek-chat");
        assert_eq!(config.max_tokens, 1024);
        assert_eq!(config.timeout_seconds, 30);
    }

    #[test]
    fn test_config_with_url() {
        let config =
            AiConfig::new("test_key".to_string()).with_url("https://custom.api.com".to_string());
        assert_eq!(config.api_url, "https://custom.api.com");
    }

    #[test]
    fn test_config_with_model() {
        let config = AiConfig::new("test_key".to_string()).with_model("gpt-4".to_string());
        assert_eq!(config.model, "gpt-4");
    }

    #[test]
    fn test_config_with_max_tokens() {
        let config = AiConfig::new("test_key".to_string()).with_max_tokens(2048);
        assert_eq!(config.max_tokens, 2048);
    }

    #[test]
    fn test_config_with_timeout() {
        let config = AiConfig::new("test_key".to_string()).with_timeout(60);
        assert_eq!(config.timeout_seconds, 60);
    }

    #[test]
    fn test_config_chained_builders() {
        let config = AiConfig::new("test_key".to_string())
            .with_url("https://custom.api.com".to_string())
            .with_model("gpt-4".to_string())
            .with_max_tokens(4096)
            .with_timeout(120);

        assert_eq!(config.api_key, "test_key");
        assert_eq!(config.api_url, "https://custom.api.com");
        assert_eq!(config.model, "gpt-4");
        assert_eq!(config.max_tokens, 4096);
        assert_eq!(config.timeout_seconds, 120);
    }

    #[test]
    fn test_config_clone() {
        let config = AiConfig::new("test_key".to_string());
        let cloned = config.clone();
        assert_eq!(config.api_key, cloned.api_key);
        assert_eq!(config.api_url, cloned.api_url);
    }

    #[test]
    fn test_config_zero_max_tokens() {
        let config = AiConfig::new("test_key".to_string()).with_max_tokens(0);
        assert_eq!(config.max_tokens, 0);
    }

    #[test]
    fn test_config_zero_timeout() {
        let config = AiConfig::new("test_key".to_string()).with_timeout(0);
        assert_eq!(config.timeout_seconds, 0);
    }

    #[test]
    fn test_config_large_max_tokens() {
        let config = AiConfig::new("test_key".to_string()).with_max_tokens(100000);
        assert_eq!(config.max_tokens, 100000);
    }

    #[test]
    fn test_config_large_timeout() {
        let config = AiConfig::new("test_key".to_string()).with_timeout(3600);
        assert_eq!(config.timeout_seconds, 3600);
    }

    #[test]
    fn test_config_empty_api_key() {
        let config = AiConfig::new("".to_string());
        assert_eq!(config.api_key, "");
    }

    #[test]
    fn test_config_empty_url() {
        let config = AiConfig::new("test_key".to_string()).with_url("".to_string());
        assert_eq!(config.api_url, "");
    }

    #[test]
    fn test_config_empty_model() {
        let config = AiConfig::new("test_key".to_string()).with_model("".to_string());
        assert_eq!(config.model, "");
    }

    #[test]
    fn test_config_default_values() {
        let config = AiConfig::new("test_key".to_string());
        assert_eq!(
            config.api_url,
            "https://api.deepseek.com/v1/chat/completions"
        );
        assert_eq!(config.model, "deepseek-chat");
        assert_eq!(config.max_tokens, 1024);
        assert_eq!(config.timeout_seconds, 30);
    }

    #[test]
    fn test_config_builder_pattern_immutability() {
        let config1 = AiConfig::new("test_key".to_string());
        let config2 = config1
            .clone()
            .with_url("https://other.api.com".to_string());

        // Original config should be unchanged
        assert_eq!(
            config1.api_url,
            "https://api.deepseek.com/v1/chat/completions"
        );
        // New config should have new value
        assert_eq!(config2.api_url, "https://other.api.com");
    }
}
