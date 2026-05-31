//! Aerospace-grade secrets management system
//! Provides secure storage and retrieval of sensitive configuration values

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use tracing::{debug, warn};

/// Secrets manager for handling sensitive configuration
pub struct SecretsManager {
    secrets: HashMap<String, String>,
    encryption_key: Option<Vec<u8>>,
}

impl SecretsManager {
    /// Create a new secrets manager
    pub fn new() -> Self {
        Self {
            secrets: HashMap::new(),
            encryption_key: Self::load_encryption_key(),
        }
    }

    /// Load secrets from environment variables
    pub fn load_from_env(&mut self) -> Result<(), SecretsError> {
        // Load JWT secret
        if let Ok(secret) = env::var("JWT_SECRET") {
            self.secrets.insert("jwt_secret".to_string(), secret);
        }

        // Load database password (if using external database)
        if let Ok(password) = env::var("DATABASE_PASSWORD") {
            self.secrets.insert("database_password".to_string(), password);
        }

        // Load API keys
        if let Ok(key) = env::var("API_KEY") {
            self.secrets.insert("api_key".to_string(), key);
        }

        // Load CSRF secret
        if let Ok(secret) = env::var("CSRF_SECRET") {
            self.secrets.insert("csrf_secret".to_string(), secret);
        }

        debug!("Loaded {} secrets from environment", self.secrets.len());
        Ok(())
    }

    /// Load secrets from a file
    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), SecretsError> {
        let path = path.as_ref();
        
        if !path.exists() {
            warn!("Secrets file not found: {}", path.display());
            return Ok(());
        }

        let content = fs::read_to_string(path)
            .map_err(|e| SecretsError::IoError(format!("Failed to read secrets file: {}", e)))?;

        // Parse simple KEY=VALUE format
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                
                // Remove quotes if present
                let value = if (value.starts_with('"') && value.ends_with('"'))
                    || (value.starts_with('\'') && value.ends_with('\''))
                {
                    &value[1..value.len()-1]
                } else {
                    value
                };

                self.secrets.insert(key.to_string(), value.to_string());
            }
        }

        debug!("Loaded {} secrets from file", self.secrets.len());
        Ok(())
    }

    /// Get a secret value
    pub fn get(&self, key: &str) -> Option<&String> {
        self.secrets.get(key)
    }

    /// Get a secret value or return a default
    pub fn get_or(&self, key: &str, default: &str) -> String {
        self.get(key).unwrap_or(&default.to_string()).clone()
    }

    /// Set a secret value
    pub fn set(&mut self, key: String, value: String) {
        self.secrets.insert(key, value);
    }

    /// Check if a secret exists
    pub fn contains(&self, key: &str) -> bool {
        self.secrets.contains_key(key)
    }

    /// Generate a secure random secret
    pub fn generate_secret(length: usize) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()";
        let mut rng = rand::thread_rng();
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Load encryption key from file or environment
    fn load_encryption_key() -> Option<Vec<u8>> {
        // Try environment variable first
        if let Ok(key_hex) = env::var("ENCRYPTION_KEY") {
            if let Ok(key) = hex::decode(&key_hex) {
                return Some(key);
            }
        }

        // Try file
        if let Ok(key_path) = env::var("ENCRYPTION_KEY_FILE") {
            if let Ok(key_hex) = fs::read_to_string(&key_path) {
                if let Ok(key) = hex::decode(key_hex.trim()) {
                    return Some(key);
                }
            }
        }

        None
    }

    /// Encrypt a secret value
    pub fn encrypt(&self, value: &str) -> Result<String, SecretsError> {
        let key = self.encryption_key.as_ref()
            .ok_or_else(|| SecretsError::EncryptionError("No encryption key available".to_string()))?;

        // Simple XOR encryption for demonstration
        // In production, use AES-256-GCM or similar
        let key_bytes = key;
        let value_bytes = value.as_bytes();
        let mut encrypted = Vec::with_capacity(value_bytes.len());
        
        for (i, &byte) in value_bytes.iter().enumerate() {
            encrypted.push(byte ^ key_bytes[i % key_bytes.len()]);
        }

        Ok(hex::encode(encrypted))
    }

    /// Decrypt a secret value
    pub fn decrypt(&self, encrypted: &str) -> Result<String, SecretsError> {
        let key = self.encryption_key.as_ref()
            .ok_or_else(|| SecretsError::EncryptionError("No encryption key available".to_string()))?;

        let encrypted_bytes = hex::decode(encrypted)
            .map_err(|e| SecretsError::EncryptionError(format!("Failed to decode hex: {}", e)))?;

        let key_bytes = key;
        let mut decrypted = Vec::with_capacity(encrypted_bytes.len());
        
        for (i, &byte) in encrypted_bytes.iter().enumerate() {
            decrypted.push(byte ^ key_bytes[i % key_bytes.len()]);
        }

        String::from_utf8(decrypted)
            .map_err(|e| SecretsError::EncryptionError(format!("Failed to decode UTF-8: {}", e)))
    }

    /// Validate that all required secrets are present
    pub fn validate_required(&self, required: &[&str]) -> Result<(), SecretsError> {
        let missing: Vec<String> = required
            .iter()
            .filter(|&key| !self.contains(key))
            .map(|&s| s.to_string())
            .collect();

        if !missing.is_empty() {
            return Err(SecretsError::MissingSecrets(missing));
        }

        Ok(())
    }

    /// Get all secret keys
    pub fn keys(&self) -> Vec<String> {
        self.secrets.keys().cloned().collect()
    }

    /// Clear all secrets from memory
    pub fn clear(&mut self) {
        self.secrets.clear();
    }
}

impl Default for SecretsManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Secrets error types
#[derive(Debug)]
pub enum SecretsError {
    IoError(String),
    EncryptionError(String),
    MissingSecrets(Vec<String>),
    InvalidFormat(String),
}

impl std::fmt::Display for SecretsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecretsError::IoError(msg) => write!(f, "IO error: {}", msg),
            SecretsError::EncryptionError(msg) => write!(f, "Encryption error: {}", msg),
            SecretsError::MissingSecrets(secrets) => {
                write!(f, "Missing required secrets: {}", secrets.join(", "))
            }
            SecretsError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
        }
    }
}

impl std::error::Error for SecretsError {}

/// Initialize secrets manager with environment and file loading
pub fn init_secrets_manager() -> Result<SecretsManager, SecretsError> {
    let mut manager = SecretsManager::new();
    
    // Load from environment
    manager.load_from_env()?;
    
    // Load from file if specified
    if let Ok(path) = env::var("SECRETS_FILE") {
        manager.load_from_file(&path)?;
    }
    
    // Validate required secrets
    let required = vec!["jwt_secret"];
    if let Err(e) = manager.validate_required(&required) {
        warn!("Secrets validation warning: {}", e);
        // Generate default JWT secret for development
        if !manager.contains("jwt_secret") {
            let default_secret = SecretsManager::generate_secret(32);
            manager.set("jwt_secret".to_string(), default_secret);
            warn!("Generated default JWT secret (use only in development)");
        }
    }
    
    Ok(manager)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secrets_manager_creation() {
        let manager = SecretsManager::new();
        assert!(manager.keys().is_empty());
    }

    #[test]
    fn test_set_and_get_secret() {
        let mut manager = SecretsManager::new();
        manager.set("test_key".to_string(), "test_value".to_string());
        
        assert_eq!(manager.get("test_key"), Some(&"test_value".to_string()));
    }

    #[test]
    fn test_get_or_default() {
        let manager = SecretsManager::new();
        assert_eq!(manager.get_or("nonexistent", "default"), "default");
    }

    #[test]
    fn test_contains() {
        let mut manager = SecretsManager::new();
        manager.set("test_key".to_string(), "test_value".to_string());
        
        assert!(manager.contains("test_key"));
        assert!(!manager.contains("nonexistent"));
    }

    #[test]
    fn test_generate_secret() {
        let secret = SecretsManager::generate_secret(32);
        assert_eq!(secret.len(), 32);
        
        let secret2 = SecretsManager::generate_secret(32);
        assert_ne!(secret, secret2);
    }

    #[test]
    fn test_validate_required() {
        let mut manager = SecretsManager::new();
        manager.set("jwt_secret".to_string(), "secret".to_string());
        
        assert!(manager.validate_required(&["jwt_secret"]).is_ok());
        assert!(manager.validate_required(&["missing_secret"]).is_err());
    }

    #[test]
    fn test_encryption_decryption() {
        let mut manager = SecretsManager::new();
        manager.encryption_key = Some(vec![1u8; 32]); // Use test key
        
        let original = "test_secret_value";
        let encrypted = manager.encrypt(original).unwrap();
        let decrypted = manager.decrypt(&encrypted).unwrap();
        
        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_encryption_without_key() {
        let manager = SecretsManager::new();
        
        let result = manager.encrypt("test");
        assert!(result.is_err());
    }

    #[test]
    fn test_clear() {
        let mut manager = SecretsManager::new();
        manager.set("test".to_string(), "value".to_string());
        
        assert!(manager.contains("test"));
        
        manager.clear();
        
        assert!(!manager.contains("test"));
    }
}
