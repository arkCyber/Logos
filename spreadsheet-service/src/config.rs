//! Aerospace-grade configuration management module
//! Provides centralized configuration with validation and environment variable support

use crate::error::{SpreadsheetError, SpreadsheetResult};
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing::{info, warn};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub security: SecurityConfig,
    pub excel: ExcelConfig,
    pub rate_limit: RateLimitConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub file: Option<String>,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub jwt_expiration: u64,
    pub bcrypt_cost: u32,
    pub enable_cors: bool,
    pub allowed_origins: Vec<String>,
}

/// Excel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExcelConfig {
    pub max_file_size: usize,
    pub max_rows: usize,
    pub max_columns: usize,
    pub temp_dir: String,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub enabled: bool,
    pub requests_per_second: u64,
    pub burst_size: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                workers: None,
            },
            database: DatabaseConfig {
                url: "sqlite:./data/spreadsheet.db".to_string(),
                max_connections: 10,
                min_connections: 2,
                connection_timeout: 30,
                idle_timeout: 600,
                max_lifetime: 1800,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
                file: None,
            },
            security: SecurityConfig {
                jwt_secret: "change-me-in-production".to_string(),
                jwt_expiration: 86400,
                bcrypt_cost: 12,
                enable_cors: true,
                allowed_origins: vec!["*".to_string()],
            },
            excel: ExcelConfig {
                max_file_size: 10 * 1024 * 1024, // 10MB
                max_rows: 1048576,
                max_columns: 16384,
                temp_dir: "./temp".to_string(),
            },
            rate_limit: RateLimitConfig {
                enabled: true,
                requests_per_second: 100,
                burst_size: 200,
            },
        }
    }
}

impl AppConfig {
    /// Load configuration from file and environment variables
    pub fn load() -> SpreadsheetResult<Self> {
        let config = Config::builder()
            // Start with default configuration
            .add_source(Config::try_from(&AppConfig::default())?)
            // Add configuration file (optional)
            .add_source(File::with_name("config/spreadsheet").required(false))
            // Add environment variables with prefix "SPREADSHEET_"
            .add_source(
                Environment::with_prefix("SPREADSHEET")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;

        let app_config: AppConfig = config.try_deserialize()?;

        // Validate configuration
        app_config.validate()?;

        info!(
            host = %app_config.server.host,
            port = %app_config.server.port,
            "Configuration loaded successfully"
        );

        Ok(app_config)
    }

    /// Validate configuration values
    fn validate(&self) -> SpreadsheetResult<()> {
        // Validate server configuration
        if self.server.port == 0 {
            return Err(SpreadsheetError::Configuration(
                "Invalid port number".to_string(),
            ));
        }

        // Validate database configuration
        if self.database.max_connections < self.database.min_connections {
            return Err(SpreadsheetError::Configuration(
                "max_connections must be >= min_connections".to_string(),
            ));
        }

        if self.database.max_connections == 0 {
            return Err(SpreadsheetError::Configuration(
                "max_connections must be > 0".to_string(),
            ));
        }

        // Validate security configuration
        if self.security.jwt_secret == "change-me-in-production" {
            warn!("Using default JWT secret - change in production!");
        }

        if self.security.jwt_secret.len() < 32 {
            return Err(SpreadsheetError::Configuration(
                "JWT secret must be at least 32 characters".to_string(),
            ));
        }

        if self.security.bcrypt_cost < 4 || self.security.bcrypt_cost > 12 {
            return Err(SpreadsheetError::Configuration(
                "bcrypt_cost must be between 4 and 12".to_string(),
            ));
        }

        // Validate Excel configuration
        if self.excel.max_file_size == 0 {
            return Err(SpreadsheetError::Configuration(
                "max_file_size must be > 0".to_string(),
            ));
        }

        // Validate rate limiting configuration
        if self.rate_limit.enabled && self.rate_limit.requests_per_second == 0 {
            return Err(SpreadsheetError::Configuration(
                "requests_per_second must be > 0 when rate limiting is enabled".to_string(),
            ));
        }

        Ok(())
    }

    /// Get server address
    pub fn server_address(&self) -> SpreadsheetResult<SocketAddr> {
        let addr_str = format!("{}:{}", self.server.host, self.server.port);
        addr_str.parse().map_err(|_| {
            SpreadsheetError::Configuration(format!("Invalid server address: {}", addr_str))
        })
    }

    /// Get database URL
    pub fn database_url(&self) -> &str {
        &self.database.url
    }

    /// Check if CORS is enabled
    pub fn cors_enabled(&self) -> bool {
        self.security.enable_cors
    }

    /// Check if rate limiting is enabled
    pub fn rate_limiting_enabled(&self) -> bool {
        self.rate_limit.enabled
    }
}

/// Configuration error conversion
impl From<ConfigError> for SpreadsheetError {
    fn from(err: ConfigError) -> Self {
        SpreadsheetError::Configuration(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 8080);
    }

    #[test]
    fn test_server_address() {
        let config = AppConfig::default();
        let addr = config.server_address().unwrap();
        assert_eq!(addr.port(), 8080);
    }

    #[test]
    fn test_validate_invalid_port() {
        let mut config = AppConfig::default();
        config.server.port = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_jwt_secret() {
        let mut config = AppConfig::default();
        config.security.jwt_secret = "short".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_bcrypt_cost() {
        let mut config = AppConfig::default();
        config.security.bcrypt_cost = 15;
        assert!(config.validate().is_err());
    }
}
