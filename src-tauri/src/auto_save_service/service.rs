use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;
use crate::error_handling::CircuitBreaker;
use crate::config_service::ExportConfigService;
use std::sync::Arc;

/// Maximum document content size to prevent memory issues
const MAX_CONTENT_SIZE: usize = 10 * 1024 * 1024; // 10MB

/// Maximum document ID length
const MAX_DOCUMENT_ID_LENGTH: usize = 256;

/// Maximum number of documents to prevent memory issues
const MAX_DOCUMENTS: usize = 10_000;

/// Minimum save interval (milliseconds)
const MIN_INTERVAL_MS: u64 = 1000; // 1 second

/// Maximum save interval (milliseconds)
const MAX_INTERVAL_MS: u64 = 3600_000; // 1 hour

/// Minimum debounce time (milliseconds)
const MIN_DEBOUNCE_MS: u64 = 100; // 100ms

/// Maximum debounce time (milliseconds)
const MAX_DEBOUNCE_MS: u64 = 60_000; // 1 minute

/// Maximum versions per document
const MAX_VERSIONS_PER_DOCUMENT: usize = 100;

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 100;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 500;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveConfig {
    pub enabled: bool,
    pub interval_ms: u64,
    pub debounce_ms: u64,
    pub max_versions: usize,
}

impl SaveConfig {
    /// Validates the save configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.interval_ms < MIN_INTERVAL_MS || self.interval_ms > MAX_INTERVAL_MS {
            return Err(format!("interval_ms must be between {} and {}", MIN_INTERVAL_MS, MAX_INTERVAL_MS));
        }
        if self.debounce_ms < MIN_DEBOUNCE_MS || self.debounce_ms > MAX_DEBOUNCE_MS {
            return Err(format!("debounce_ms must be between {} and {}", MIN_DEBOUNCE_MS, MAX_DEBOUNCE_MS));
        }
        if self.max_versions == 0 || self.max_versions > MAX_VERSIONS_PER_DOCUMENT {
            return Err(format!("max_versions must be between 1 and {}", MAX_VERSIONS_PER_DOCUMENT));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveResult {
    pub success: bool,
    pub document_id: String,
    pub version: usize,
    pub timestamp: u64,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
struct DocumentVersion {
    content: String,
    timestamp: Instant,
    version: usize,
}

pub struct AutoSaveService {
    documents: Mutex<HashMap<String, DocumentVersion>>,
    config: SaveConfig,
    last_save: Mutex<HashMap<String, Instant>>,
    config_service: Arc<ExportConfigService>,
    circuit_breaker: CircuitBreaker,
}

impl AutoSaveService {
    /// Creates a new auto save service with the given configuration
    /// 
    /// # Arguments
    /// * `config` - Save configuration
    /// 
    /// # Returns
    /// A new AutoSaveService instance
    pub fn new(config: SaveConfig) -> Self {
        let config_service = Arc::new(ExportConfigService::new());
        let circuit_breaker = CircuitBreaker::new(config_service.clone());
        Self {
            documents: Mutex::new(HashMap::new()),
            config,
            last_save: Mutex::new(HashMap::new()),
            config_service,
            circuit_breaker,
        }
    }

    /// Checks if a document should be saved based on debounce settings
    /// 
    /// # Arguments
    /// * `document_id` - The document ID to check
    /// 
    /// # Returns
    /// true if the document should be saved, false otherwise
    pub fn should_save(&self, document_id: &str) -> bool {
        if !self.config.enabled {
            return false;
        }

        // Validate document ID
        if document_id.len() > MAX_DOCUMENT_ID_LENGTH {
            eprintln!("Should save: document_id exceeds maximum length of {}", MAX_DOCUMENT_ID_LENGTH);
            return false;
        }

        let last_saves = self.last_save.lock().unwrap();
        if let Some(last_save) = last_saves.get(document_id) {
            let elapsed = last_save.elapsed();
            elapsed.as_millis() as u64 >= self.config.debounce_ms
        } else {
            true
        }
    }

    /// Saves a document with the given content
    /// 
    /// # Arguments
    /// * `document_id` - The document ID
    /// * `content` - The document content
    /// 
    /// # Returns
    /// SaveResult containing the save status and metadata
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates input sizes to prevent DoS attacks and memory issues
    pub fn save_document(&self, document_id: &str, content: &str) -> SaveResult {
        let start_time = Instant::now();
        
        // Check circuit breaker
        if !self.circuit_breaker.allow_operation() {
            return SaveResult {
                success: false,
                document_id: document_id.to_string(),
                version: 0,
                timestamp: 0,
                error: Some("Circuit breaker is open, blocking save operations".to_string()),
            };
        }
        
        // Input validation
        if document_id.is_empty() {
            self.circuit_breaker.record_failure();
            return SaveResult {
                success: false,
                document_id: document_id.to_string(),
                version: 0,
                timestamp: 0,
                error: Some("Document ID cannot be empty".to_string()),
            };
        }

        // Security check: prevent DoS with oversized document ID
        if document_id.len() > MAX_DOCUMENT_ID_LENGTH {
            eprintln!("Save document: document_id exceeds maximum length of {}", MAX_DOCUMENT_ID_LENGTH);
            self.circuit_breaker.record_failure();
            return SaveResult {
                success: false,
                document_id: document_id.to_string(),
                version: 0,
                timestamp: 0,
                error: Some(format!("Document ID exceeds maximum length of {}", MAX_DOCUMENT_ID_LENGTH)),
            };
        }

        // Security check: prevent DoS with oversized content
        if content.len() > MAX_CONTENT_SIZE {
            eprintln!("Save document: content exceeds maximum size of {} bytes", MAX_CONTENT_SIZE);
            self.circuit_breaker.record_failure();
            return SaveResult {
                success: false,
                document_id: document_id.to_string(),
                version: 0,
                timestamp: 0,
                error: Some(format!("Content exceeds maximum size of {} bytes", MAX_CONTENT_SIZE)),
            };
        }

        let mut documents = self.documents.lock().unwrap();
        let mut last_saves = self.last_save.lock().unwrap();

        // Safety check: prevent too many documents
        if !documents.contains_key(document_id) && documents.len() >= MAX_DOCUMENTS {
            eprintln!("Save document: reached maximum document limit of {}", MAX_DOCUMENTS);
            self.circuit_breaker.record_failure();
            return SaveResult {
                success: false,
                document_id: document_id.to_string(),
                version: 0,
                timestamp: 0,
                error: Some(format!("Maximum document limit of {} reached", MAX_DOCUMENTS)),
            };
        }

        let current_version = documents
            .get(document_id)
            .map(|v| v.version)
            .unwrap_or(0);

        let new_version = current_version + 1;
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Check if content changed
        if let Some(existing) = documents.get(document_id) {
            if existing.content == content {
                // Performance monitoring
                let elapsed = start_time.elapsed();
                if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
                    eprintln!("Save document CRITICAL performance warning: took {}ms (no change)", elapsed.as_millis());
                } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
                    eprintln!("Save document performance warning: took {}ms (no change)", elapsed.as_millis());
                }
                
                return SaveResult {
                    success: true,
                    document_id: document_id.to_string(),
                    version: existing.version,
                    timestamp,
                    error: None,
                };
            }
        }

        // Create new version
        let version = DocumentVersion {
            content: content.to_string(),
            timestamp: Instant::now(),
            version: new_version,
        };

        documents.insert(document_id.to_string(), version);
        last_saves.insert(document_id.to_string(), Instant::now());

        // Clean up old versions
        self.cleanup_old_versions(&mut documents);

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Save document CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Save document performance warning: took {}ms", elapsed.as_millis());
        }

        self.circuit_breaker.record_success();

        SaveResult {
            success: true,
            document_id: document_id.to_string(),
            version: new_version,
            timestamp,
            error: None,
        }
    }

    /// Gets the current content of a document
    /// 
    /// # Arguments
    /// * `document_id` - The document ID
    /// 
    /// # Returns
    /// The document content if found, None otherwise
    pub fn get_document(&self, document_id: &str) -> Option<String> {
        let documents = self.documents.lock().unwrap();
        documents.get(document_id).map(|v| v.content.clone())
    }

    /// Gets a specific version of a document
    /// 
    /// # Arguments
    /// * `document_id` - The document ID
    /// * `version` - The version number
    /// 
    /// # Returns
    /// The document content if the version exists, None otherwise
    /// 
    /// # Note
    /// In the current implementation, only the current version is stored
    pub fn get_version(&self, document_id: &str, version: usize) -> Option<String> {
        let documents = self.documents.lock().unwrap();
        // In a real implementation, you'd store multiple versions
        // For now, just return the current version
        documents.get(document_id).map(|v| {
            if v.version == version {
                Some(v.content.clone())
            } else {
                None
            }
        }).flatten()
    }

    /// Deletes a document from the auto save service
    /// 
    /// # Arguments
    /// * `document_id` - The document ID to delete
    pub fn delete_document(&self, document_id: &str) {
        let mut documents = self.documents.lock().unwrap();
        let mut last_saves = self.last_save.lock().unwrap();
        documents.remove(document_id);
        last_saves.remove(document_id);
    }

    /// Lists all document IDs in the auto save service
    /// 
    /// # Returns
    /// A vector of document IDs
    pub fn list_documents(&self) -> Vec<String> {
        let documents = self.documents.lock().unwrap();
        documents.keys().cloned().collect()
    }

    /// Gets the number of documents stored
    /// 
    /// # Returns
    /// The number of documents
    pub fn document_count(&self) -> usize {
        let documents = self.documents.lock().unwrap();
        documents.len()
    }

    /// Cleans up old versions based on configuration
    /// 
    /// # Arguments
    /// * `documents` - The documents HashMap to clean up
    /// 
    /// # Algorithm
    /// Removes oldest documents when the count exceeds max_versions
    fn cleanup_old_versions(&self, documents: &mut HashMap<String, DocumentVersion>) {
        // In a real implementation, you'd keep multiple versions
        // For now, this is a placeholder
        if documents.len() > self.config.max_versions {
            // Remove oldest documents
            let mut doc_list: Vec<(String, std::time::Instant)> = documents
                .iter()
                .map(|(id, v)| (id.clone(), v.timestamp))
                .collect();
            doc_list.sort_by_key(|(_, t)| *t);
            
            let to_remove = documents.len() - self.config.max_versions;
            for (doc_id, _) in doc_list.into_iter().take(to_remove) {
                documents.remove(&doc_id);
            }
        }
    }

    /// Updates the save configuration
    /// 
    /// # Arguments
    /// * `config` - The new configuration
    /// 
    /// # Returns
    /// Ok(()) if the configuration is valid, Err otherwise
    pub fn update_config(&mut self, config: SaveConfig) -> Result<(), String> {
        if let Err(e) = config.validate() {
            return Err(e);
        }
        self.config = config;
        Ok(())
    }

    /// Gets the current save configuration
    /// 
    /// # Returns
    /// The current SaveConfig
    pub fn get_config(&self) -> SaveConfig {
        self.config.clone()
    }

    /// Clears all documents from the auto save service
    /// 
    /// # Warning
    /// This will delete all saved documents
    pub fn clear_all(&self) {
        let mut documents = self.documents.lock().unwrap();
        let mut last_saves = self.last_save.lock().unwrap();
        documents.clear();
        last_saves.clear();
    }
}

impl Default for AutoSaveService {
    fn default() -> Self {
        Self::new(SaveConfig {
            enabled: true,
            interval_ms: 30000, // 30 seconds
            debounce_ms: 2000,  // 2 seconds
            max_versions: 10,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_save_service_creation() {
        let service = AutoSaveService::default();
        assert!(service.config.enabled);
    }

    #[test]
    fn test_save_document() {
        let service = AutoSaveService::default();
        let result = service.save_document("doc1", "Hello world");
        assert!(result.success);
        assert_eq!(result.version, 1);
    }

    #[test]
    fn test_get_document() {
        let service = AutoSaveService::default();
        service.save_document("doc1", "Hello world");
        let content = service.get_document("doc1");
        assert_eq!(content, Some("Hello world".to_string()));
    }

    #[test]
    fn test_should_save_debounce() {
        let config = SaveConfig {
            enabled: true,
            interval_ms: 30000,
            debounce_ms: 1000,
            max_versions: 10,
        };
        let service = AutoSaveService::new(config);
        
        assert!(service.should_save("doc1"));
        service.save_document("doc1", "Hello");
        assert!(!service.should_save("doc1")); // Should not save immediately
    }

    #[test]
    fn test_delete_document() {
        let service = AutoSaveService::default();
        service.save_document("doc1", "Hello world");
        service.delete_document("doc1");
        assert_eq!(service.get_document("doc1"), None);
    }

    #[test]
    fn test_list_documents() {
        let service = AutoSaveService::default();
        service.save_document("doc1", "Hello");
        service.save_document("doc2", "World");
        let docs = service.list_documents();
        assert_eq!(docs.len(), 2);
    }

    #[test]
    fn test_duplicate_save_no_version_increment() {
        let service = AutoSaveService::default();
        service.save_document("doc1", "Hello");
        let result1 = service.save_document("doc1", "Hello");
        let result2 = service.save_document("doc1", "Hello");
        assert_eq!(result1.version, result2.version); // Same content, no increment
    }

    #[test]
    fn test_config_update() {
        let mut service = AutoSaveService::default();
        let new_config = SaveConfig {
            enabled: false,
            interval_ms: 60000,
            debounce_ms: 5000,
            max_versions: 20,
        };
        assert!(service.update_config(new_config.clone()).is_ok());
        assert_eq!(service.get_config().enabled, false);
    }

    #[test]
    fn test_config_validation() {
        let config = SaveConfig {
            enabled: true,
            interval_ms: 500, // Below minimum
            debounce_ms: 2000,
            max_versions: 10,
        };
        assert!(config.validate().is_err());

        let config = SaveConfig {
            enabled: true,
            interval_ms: 30000,
            debounce_ms: 2000,
            max_versions: 0, // Invalid
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_max_content_size() {
        let service = AutoSaveService::default();
        let large_content = "a".repeat(MAX_CONTENT_SIZE + 1);
        let result = service.save_document("doc1", &large_content);
        assert!(!result.success);
    }

    #[test]
    fn test_max_document_id_length() {
        let service = AutoSaveService::default();
        let long_id = "a".repeat(MAX_DOCUMENT_ID_LENGTH + 1);
        let result = service.save_document(&long_id, "content");
        assert!(!result.success);
    }

    #[test]
    fn test_empty_document_id() {
        let service = AutoSaveService::default();
        let result = service.save_document("", "content");
        assert!(!result.success);
    }

    #[test]
    fn test_max_documents_limit() {
        let config = SaveConfig {
            enabled: true,
            interval_ms: 30000,
            debounce_ms: 1000,
            max_versions: MAX_DOCUMENTS, // Set high to avoid cleanup
        };
        let service = AutoSaveService::new(config);
        
        // Try to add more documents than MAX_DOCUMENTS
        for i in 0..=MAX_DOCUMENTS {
            let result = service.save_document(&format!("doc{}", i), "content");
            if i < MAX_DOCUMENTS {
                assert!(result.success);
            } else {
                assert!(!result.success);
            }
        }
    }

    #[test]
    fn test_document_count() {
        let service = AutoSaveService::default();
        assert_eq!(service.document_count(), 0);
        
        service.save_document("doc1", "content");
        assert_eq!(service.document_count(), 1);
        
        service.save_document("doc2", "content");
        assert_eq!(service.document_count(), 2);
        
        service.delete_document("doc1");
        assert_eq!(service.document_count(), 1);
    }

    #[test]
    fn test_clear_all() {
        let service = AutoSaveService::default();
        service.save_document("doc1", "content");
        service.save_document("doc2", "content");
        
        assert_eq!(service.document_count(), 2);
        
        service.clear_all();
        
        assert_eq!(service.document_count(), 0);
        assert_eq!(service.get_document("doc1"), None);
        assert_eq!(service.get_document("doc2"), None);
    }

    #[test]
    fn test_should_save_disabled() {
        let config = SaveConfig {
            enabled: false,
            interval_ms: 30000,
            debounce_ms: 1000,
            max_versions: 10,
        };
        let service = AutoSaveService::new(config);
        
        assert!(!service.should_save("doc1"));
    }

    #[test]
    fn test_cleanup_old_versions() {
        let config = SaveConfig {
            enabled: true,
            interval_ms: 30000,
            debounce_ms: 1000,
            max_versions: 2,
        };
        let service = AutoSaveService::new(config);
        
        service.save_document("doc1", "content1");
        service.save_document("doc2", "content2");
        service.save_document("doc3", "content3");
        
        // Should only keep 2 documents (max_versions)
        assert!(service.document_count() <= 2);
    }

    #[test]
    fn test_performance_large_content() {
        let service = AutoSaveService::default();
        let large_content = "a".repeat(1_000_000); // 1MB
        let result = service.save_document("doc1", &large_content);
        assert!(result.success);
    }
}
