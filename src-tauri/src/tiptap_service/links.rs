//! TipTap Link Manager - Aerospace-Grade Link Operations Service
//!
//! Safety-critical link operations service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Comprehensive error handling
//! - Performance monitoring
//! - Security hardening
//! - Fault tolerance and error recovery

use serde::{Deserialize, Serialize};
use std::time::Instant;
use crate::error_handling::{ErrorContext, ErrorSeverity};
use crate::config_service::ExportConfigService;
use std::sync::Arc;
use super::editor::{TipTapNode, Mark};

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 50;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 200;

/// Maximum link URL length
const MAX_LINK_URL_LENGTH: usize = 2048;

/// Maximum link text length
const MAX_LINK_TEXT_LENGTH: usize = 10000;

/// Maximum link title length
const MAX_LINK_TITLE_LENGTH: usize = 500;

/// Link attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkAttributes {
    pub href: String,
    pub title: Option<String>,
    pub target: Option<String>,
    pub rel: Option<String>,
}

pub struct LinkManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl LinkManager {
    /// Creates a new link manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new LinkManager instance
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
        }
    }

    /// Get the performance warning threshold
    /// 
    /// # Returns
    /// The performance warning threshold in milliseconds
    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    /// Get the performance critical threshold
    /// 
    /// # Returns
    /// The performance critical threshold in milliseconds
    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    /// Get the maximum link URL length constant
    /// 
    /// # Returns
    /// The maximum link URL length
    pub fn max_link_url_length() -> usize {
        MAX_LINK_URL_LENGTH
    }

    /// Get the maximum link text length constant
    /// 
    /// # Returns
    /// The maximum link text length
    pub fn max_link_text_length() -> usize {
        MAX_LINK_TEXT_LENGTH
    }

    /// Get the maximum link title length constant
    /// 
    /// # Returns
    /// The maximum link title length
    pub fn max_link_title_length() -> usize {
        MAX_LINK_TITLE_LENGTH
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

    /// Reset operation count
    pub fn reset_operation_count(&mut self) {
        self.operation_count = 0;
    }

    /// Validate link URL
    /// 
    /// # Arguments
    /// * `url` - The URL to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting URL length and validating protocol
    fn validate_link_url(&self, url: &str) -> Result<(), String> {
        if url.is_empty() {
            return Err("Link URL cannot be empty".to_string());
        }
        if url.len() > MAX_LINK_URL_LENGTH {
            return Err(format!("Link URL exceeds maximum length of {} characters", MAX_LINK_URL_LENGTH));
        }
        
        // Basic URL validation
        if !url.starts_with("http://") && !url.starts_with("https://") && !url.starts_with("mailto:") {
            return Err("Link URL must start with http://, https://, or mailto:".to_string());
        }
        
        Ok(())
    }

    /// Validate link text
    /// 
    /// # Arguments
    /// * `text` - The text to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting text length
    fn validate_link_text(&self, text: &str) -> Result<(), String> {
        if text.len() > MAX_LINK_TEXT_LENGTH {
            return Err(format!("Link text exceeds maximum length of {} characters", MAX_LINK_TEXT_LENGTH));
        }
        Ok(())
    }

    /// Validate link title
    /// 
    /// # Arguments
    /// * `title` - The title to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting title length
    fn validate_link_title(&self, title: &str) -> Result<(), String> {
        if title.len() > MAX_LINK_TITLE_LENGTH {
            return Err(format!("Link title exceeds maximum length of {} characters", MAX_LINK_TITLE_LENGTH));
        }
        Ok(())
    }

    /// Create a link mark
    /// 
    /// # Arguments
    /// * `attributes` - The link attributes
    /// 
    /// # Returns
    /// Result containing the link mark or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates URL length and protocol
    pub fn create_link_mark(&mut self, attributes: LinkAttributes) -> Result<Mark, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate URL
        self.validate_link_url(&attributes.href)?;

        // Validate title
        if let Some(ref title) = attributes.title {
            self.validate_link_title(title)?;
        }

        let attrs_json = serde_json::to_value(&attributes).map_err(|e| {
            let error = format!("Failed to serialize link attributes: {}", e);
            self.record_error("SERIALIZE_ERROR", &error, "create_link_mark");
            error
        })?;

        let mark = Mark {
            mark_type: "link".to_string(),
            attrs: Some(attrs_json),
        };

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Link mark creation CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Link mark creation performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(mark)
    }

    /// Apply link to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply link to
    /// * `attributes` - The link attributes
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn apply_link(&mut self, node: &mut TipTapNode, attributes: LinkAttributes) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate URL
        self.validate_link_url(&attributes.href)?;

        // Validate title
        if let Some(ref title) = attributes.title {
            self.validate_link_title(title)?;
        }

        let mark = self.create_link_mark(attributes)?;

        if node.marks.is_none() {
            node.marks = Some(vec![mark]);
        } else {
            node.marks.as_mut().unwrap().push(mark);
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Link apply CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Link apply performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove link from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove link from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_link(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut marks) = node.marks {
            marks.retain(|m| m.mark_type != "link");
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Link remove CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Link remove performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Check if a node has a link
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if the node has a link, false otherwise
    pub fn has_link(&self, node: &TipTapNode) -> bool {
        if let Some(ref marks) = node.marks {
            marks.iter().any(|m| m.mark_type == "link")
        } else {
            false
        }
    }

    /// Get link attributes from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get link from
    /// 
    /// # Returns
    /// Option containing the link attributes
    pub fn get_link_attributes(&self, node: &TipTapNode) -> Option<LinkAttributes> {
        if let Some(ref marks) = node.marks {
            for mark in marks {
                if mark.mark_type == "link" {
                    if let Some(ref attrs) = mark.attrs {
                        if let Ok(attributes) = serde_json::from_value::<LinkAttributes>(attrs.clone()) {
                            return Some(attributes);
                        }
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_link_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = LinkManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(LinkManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(LinkManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(LinkManager::max_link_url_length(), MAX_LINK_URL_LENGTH);
        assert_eq!(LinkManager::max_link_text_length(), MAX_LINK_TEXT_LENGTH);
        assert_eq!(LinkManager::max_link_title_length(), MAX_LINK_TITLE_LENGTH);
    }

    #[test]
    fn test_create_link_mark() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LinkManager::new(config_service);
        
        let attributes = LinkAttributes {
            href: "https://example.com".to_string(),
            title: Some("Example".to_string()),
            target: Some("_blank".to_string()),
            rel: None,
        };
        
        let result = manager.create_link_mark(attributes);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_link_mark_empty_url() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LinkManager::new(config_service);
        
        let attributes = LinkAttributes {
            href: "".to_string(),
            title: None,
            target: None,
            rel: None,
        };
        
        let result = manager.create_link_mark(attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_link_mark_url_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LinkManager::new(config_service);
        
        let long_url = "a".repeat(MAX_LINK_URL_LENGTH + 1);
        let attributes = LinkAttributes {
            href: long_url,
            title: None,
            target: None,
            rel: None,
        };
        
        let result = manager.create_link_mark(attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_link_mark_invalid_protocol() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LinkManager::new(config_service);
        
        let attributes = LinkAttributes {
            href: "ftp://example.com".to_string(),
            title: None,
            target: None,
            rel: None,
        };
        
        let result = manager.create_link_mark(attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_link_mark_title_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LinkManager::new(config_service);
        
        let long_title = "a".repeat(MAX_LINK_TITLE_LENGTH + 1);
        let attributes = LinkAttributes {
            href: "https://example.com".to_string(),
            title: Some(long_title),
            target: None,
            rel: None,
        };
        
        let result = manager.create_link_mark(attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_link() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LinkManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Click here".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = LinkAttributes {
            href: "https://example.com".to_string(),
            title: None,
            target: None,
            rel: None,
        };
        
        let result = manager.apply_link(&mut node, attributes);
        assert!(result.is_ok());
        assert!(manager.has_link(&node));
    }

    #[test]
    fn test_remove_link() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LinkManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Click here".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = LinkAttributes {
            href: "https://example.com".to_string(),
            title: None,
            target: None,
            rel: None,
        };
        
        manager.apply_link(&mut node, attributes).unwrap();
        let result = manager.remove_link(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_link(&node));
    }

    #[test]
    fn test_has_link() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LinkManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Click here".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(!manager.has_link(&node));
        
        let attributes = LinkAttributes {
            href: "https://example.com".to_string(),
            title: None,
            target: None,
            rel: None,
        };
        
        manager.apply_link(&mut node, attributes).unwrap();
        assert!(manager.has_link(&node));
    }

    #[test]
    fn test_get_link_attributes() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LinkManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Click here".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = LinkAttributes {
            href: "https://example.com".to_string(),
            title: Some("Example".to_string()),
            target: Some("_blank".to_string()),
            rel: None,
        };
        
        manager.apply_link(&mut node, attributes.clone()).unwrap();
        
        let retrieved = manager.get_link_attributes(&node);
        assert!(retrieved.is_some());
        let retrieved_attrs = retrieved.unwrap();
        assert_eq!(retrieved_attrs.href, "https://example.com");
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LinkManager::new(config_service);
        
        let attributes = LinkAttributes {
            href: "https://example.com".to_string(),
            title: None,
            target: None,
            rel: None,
        };
        
        manager.create_link_mark(attributes).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LinkManager::new(config_service);
        
        let attributes = LinkAttributes {
            href: "https://example.com".to_string(),
            title: None,
            target: None,
            rel: None,
        };
        
        manager.create_link_mark(attributes).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LinkManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        
        let last_error = manager.get_last_error();
        assert!(last_error.is_some());
        let error = last_error.unwrap();
        assert_eq!(error.code, "TEST_ERROR");
        assert_eq!(error.message, "Test error message");
        assert_eq!(error.source, "test_source");
    }

    #[test]
    fn test_error_state_reset() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LinkManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }

    #[test]
    fn test_link_attributes_creation() {
        let attributes = LinkAttributes {
            href: "https://example.com".to_string(),
            title: Some("Example".to_string()),
            target: Some("_blank".to_string()),
            rel: Some("noopener".to_string()),
        };
        
        assert_eq!(attributes.href, "https://example.com");
        assert_eq!(attributes.title, Some("Example".to_string()));
        assert_eq!(attributes.target, Some("_blank".to_string()));
        assert_eq!(attributes.rel, Some("noopener".to_string()));
    }
}
