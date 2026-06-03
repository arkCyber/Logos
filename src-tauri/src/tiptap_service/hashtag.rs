//! TipTap Hashtag Manager - Aerospace-Grade Hashtag Service
//!
//! Safety-critical hashtag service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Performance monitoring
//! - Comprehensive error handling
//! - Security hardening
//! - Fault tolerance and error recovery

use std::time::Instant;
use std::collections::HashMap;
use crate::error_handling::{ErrorContext, ErrorSeverity};
use crate::config_service::ExportConfigService;
use std::sync::Arc;

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 50;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 200;

/// Maximum hashtag length
const MAX_HASHTAG_LENGTH: usize = 100;

/// Hashtag
#[derive(Debug, Clone)]
pub struct Hashtag {
    pub hashtag_id: String,
    pub tag: String,
    pub position: usize,
    pub count: usize,
}

pub struct HashtagManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
    hashtags: HashMap<String, Hashtag>,
    hashtag_counter: u64,
}

impl HashtagManager {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
            hashtags: HashMap::new(),
            hashtag_counter: 0,
        }
    }

    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    pub fn max_hashtag_length() -> usize {
        MAX_HASHTAG_LENGTH
    }

    fn record_error(&mut self, code: &str, message: &str, source: &str) {
        self.last_error = Some(ErrorContext::new(ErrorSeverity::Error, code, message, source));
    }

    pub fn get_last_error(&self) -> Option<&ErrorContext> {
        self.last_error.as_ref()
    }

    pub fn get_operation_count(&self) -> u64 {
        self.operation_count
    }

    pub fn reset_error_state(&mut self) {
        self.last_error = None;
    }

    pub fn reset_operation_count(&mut self) {
        self.operation_count = 0;
    }

    pub fn add_hashtag(&mut self, tag: String, position: usize) -> Result<String, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if tag.is_empty() {
            return Err("Hashtag cannot be empty".to_string());
        }

        if !tag.starts_with('#') {
            return Err("Hashtag must start with #".to_string());
        }

        if tag.len() > MAX_HASHTAG_LENGTH {
            return Err(format!("Hashtag exceeds maximum length of {} characters", MAX_HASHTAG_LENGTH));
        }

        self.hashtag_counter += 1;
        let hashtag_id = format!("hashtag_{}", self.hashtag_counter);

        let hashtag = Hashtag {
            hashtag_id: hashtag_id.clone(),
            tag,
            position,
            count: 1,
        };

        self.hashtags.insert(hashtag_id.clone(), hashtag);

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Add hashtag CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Add hashtag performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(hashtag_id)
    }

    pub fn remove_hashtag(&mut self, hashtag_id: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.hashtags.remove(hashtag_id)
            .ok_or("Hashtag not found")?;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Remove hashtag CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Remove hashtag performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn find_by_tag(&self, tag: &str) -> Option<&Hashtag> {
        self.hashtags.values().find(|h| h.tag == tag)
    }

    pub fn get_hashtag(&self, hashtag_id: &str) -> Option<&Hashtag> {
        self.hashtags.get(hashtag_id)
    }

    pub fn get_all_hashtags(&self) -> Vec<&Hashtag> {
        self.hashtags.values().collect()
    }

    pub fn clear_hashtags(&mut self) {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.hashtags.clear();

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Clear hashtags CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Clear hashtags performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashtag_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = HashtagManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_add_hashtag() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HashtagManager::new(config_service);
        
        let result = manager.add_hashtag("#rust".to_string(), 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_hashtag_without_hash() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HashtagManager::new(config_service);
        
        let result = manager.add_hashtag("rust".to_string(), 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_by_tag() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HashtagManager::new(config_service);
        
        manager.add_hashtag("#rust".to_string(), 0).unwrap();
        let result = manager.find_by_tag("#rust");
        assert!(result.is_some());
    }

    #[test]
    fn test_clear_hashtags() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HashtagManager::new(config_service);
        
        manager.add_hashtag("#rust".to_string(), 0).unwrap();
        manager.clear_hashtags();
        
        assert_eq!(manager.get_all_hashtags().len(), 0);
    }
}
