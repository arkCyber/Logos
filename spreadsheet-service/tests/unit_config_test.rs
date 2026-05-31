//! Aerospace-grade unit tests for configuration module
//! Tests configuration loading and validation

#[cfg(test)]
mod tests {
    use spreadsheet_service::config::{AppConfig, ServerConfig, DatabaseConfig, SecurityConfig};

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 8080);
    }

    #[test]
    fn test_config_validation_valid() {
        let config = AppConfig::default();
        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_validation_invalid_port() {
        let mut config = AppConfig::default();
        config.server.port = 70000; // Invalid port
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_negative_port() {
        let mut config = AppConfig::default();
        config.server.port = -1;
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_empty_database_url() {
        let mut config = AppConfig::default();
        config.database.url = "";
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_weak_jwt_secret() {
        let mut config = AppConfig::default();
        config.security.jwt_secret = "weak".to_string();
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_invalid_bcrypt_cost() {
        let mut config = AppConfig::default();
        config.security.bcrypt_cost = 20; // Too high
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_negative_bcrypt_cost() {
        let mut config = AppConfig::default();
        config.security.bcrypt_cost = -1;
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_invalid_rate_limit() {
        let mut config = AppConfig::default();
        config.rate_limit.requests_per_second = 0;
        let result = config.validate();
        assert!(result.is_err());
    }
}
