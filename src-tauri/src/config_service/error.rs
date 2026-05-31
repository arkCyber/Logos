//! Configuration Error Handling - Aerospace-Grade Error Management

use std::fmt;

/// Configuration error types
#[derive(Debug, Clone)]
pub enum ConfigError {
    /// Configuration file not found
    FileNotFound(String),
    /// Configuration file parsing failed
    ParseFailed(String),
    /// Invalid configuration value
    InvalidValue(String),
    /// Missing required configuration field
    MissingField(String),
    /// Configuration validation failed
    ValidationFailed(String),
    /// IO error during configuration loading
    IoError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileNotFound(path) => write!(f, "Configuration file not found: {}", path),
            ConfigError::ParseFailed(msg) => write!(f, "Configuration parsing failed: {}", msg),
            ConfigError::InvalidValue(msg) => write!(f, "Invalid configuration value: {}", msg),
            ConfigError::MissingField(field) => write!(f, "Missing required configuration field: {}", field),
            ConfigError::ValidationFailed(msg) => write!(f, "Configuration validation failed: {}", msg),
            ConfigError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<ConfigError> for String {
    fn from(error: ConfigError) -> Self {
        error.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = ConfigError::FileNotFound("test.toml".to_string());
        assert_eq!(error.to_string(), "Configuration file not found: test.toml");
    }

    #[test]
    fn test_error_to_string() {
        let error = ConfigError::InvalidValue("test error".to_string());
        let s: String = error.into();
        assert_eq!(s, "Invalid configuration value: test error");
    }
}
