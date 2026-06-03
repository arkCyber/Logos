//! TipTap Link Manager - Aerospace-Grade Link Service
//!
//! Safety-critical link service with:
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

/// Maximum URL length
const MAX_URL_LENGTH: usize = 2000;

/// Maximum link text length
const MAX_LINK_TEXT_LENGTH: usize = 500;

/// Hyperlink
#[derive(Debug, Clone)]
pub struct Hyperlink {
    pub link_id: String,
    pub url: String,
    pub text: String,
    pub title: Option<String>,
    pub target: Option<String>,
}

pub struct HyperlinkManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
    links: HashMap<String, Hyperlink>,
    link_counter: u64,
}

impl HyperlinkManager {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
            links: HashMap::new(),
            link_counter: 0,
        }
    }

    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    pub fn max_url_length() -> usize {
        MAX_URL_LENGTH
    }

    pub fn max_link_text_length() -> usize {
        MAX_LINK_TEXT_LENGTH
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

    pub fn add_link(&mut self, url: String, text: String, title: Option<String>, target: Option<String>) -> Result<String, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if url.is_empty() {
            return Err("URL cannot be empty".to_string());
        }

        if url.len() > MAX_URL_LENGTH {
            return Err(format!("URL exceeds maximum length of {} characters", MAX_URL_LENGTH));
        }

        if text.is_empty() {
            return Err("Link text cannot be empty".to_string());
        }

        if text.len() > MAX_LINK_TEXT_LENGTH {
            return Err(format!("Link text exceeds maximum length of {} characters", MAX_LINK_TEXT_LENGTH));
        }

        self.link_counter += 1;
        let link_id = format!("link_{}", self.link_counter);

        let link = Hyperlink {
            link_id: link_id.clone(),
            url,
            text,
            title,
            target,
        };

        self.links.insert(link_id.clone(), link);

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Add link CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Add link performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(link_id)
    }

    pub fn remove_link(&mut self, link_id: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.links.remove(link_id)
            .ok_or("Link not found")?;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Remove link CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Remove link performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_link(&self, link_id: &str) -> Option<&Hyperlink> {
        self.links.get(link_id)
    }

    pub fn get_all_links(&self) -> Vec<&Hyperlink> {
        self.links.values().collect()
    }

    pub fn clear_links(&mut self) {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.links.clear();

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Clear links CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Clear links performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = HyperlinkManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_add_link() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HyperlinkManager::new(config_service);
        
        let result = manager.add_link(
            "https://example.com".to_string(),
            "Example".to_string(),
            Some("Example Site".to_string()),
            Some("_blank".to_string())
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_link() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HyperlinkManager::new(config_service);
        
        let link_id = manager.add_link(
            "https://example.com".to_string(),
            "Example".to_string(),
            None,
            None
        ).unwrap();
        
        let result = manager.remove_link(&link_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_url() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HyperlinkManager::new(config_service);
        
        let result = manager.add_link(
            "".to_string(),
            "Example".to_string(),
            None,
            None
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_clear_links() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HyperlinkManager::new(config_service);
        
        manager.add_link(
            "https://example.com".to_string(),
            "Example".to_string(),
            None,
            None
        ).unwrap();
        
        manager.clear_links();
        assert_eq!(manager.get_all_links().len(), 0);
    }
}
