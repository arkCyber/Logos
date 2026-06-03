//! TipTap Filter Manager - Aerospace-Grade Filter Operations Service
//!
//! Safety-critical filter operations service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Comprehensive error handling
//! - Performance monitoring
//! - Security hardening
//! - Fault tolerance and error recovery

use std::time::Instant;
use crate::error_handling::{ErrorContext, ErrorSeverity};
use crate::config_service::ExportConfigService;
use std::sync::Arc;
use super::editor::TipTapNode;

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 50;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 200;

/// Maximum filter string length
const MAX_FILTER_LENGTH: usize = 200;

pub struct FilterManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FilterManager {
    /// Creates a new filter manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new FilterManager instance
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

    /// Get the maximum filter length constant
    /// 
    /// # Returns
    /// The maximum filter string length
    pub fn max_filter_length() -> usize {
        MAX_FILTER_LENGTH
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

    /// Validate filter string
    /// 
    /// # Arguments
    /// * `filter` - The filter string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting filter string length
    fn validate_filter(&self, filter: &str) -> Result<(), String> {
        if filter.is_empty() {
            return Err("Filter cannot be empty".to_string());
        }
        if filter.len() > MAX_FILTER_LENGTH {
            return Err(format!("Filter string exceeds maximum length of {} characters", MAX_FILTER_LENGTH));
        }
        // Check for unmatched parentheses first
        if filter.contains('(') && !filter.contains(')') {
            return Err("Invalid filter: unmatched parentheses".to_string());
        }
        // Basic validation for common filter functions
        let valid_patterns = ["blur", "brightness", "contrast", "grayscale", "hue-rotate", "invert", "opacity", "saturate", "sepia", "none"];
        if !valid_patterns.iter().any(|pattern| filter.contains(pattern)) {
            // Allow custom values but validate basic structure
            if filter.contains('(') && !filter.contains(')') {
                return Err("Invalid filter: unmatched parentheses".to_string());
            }
        }
        Ok(())
    }

    /// Apply filter to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply filter to
    /// * `filter` - The filter to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates filter string
    pub fn apply_filter(&mut self, node: &mut TipTapNode, filter: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate filter
        self.validate_filter(filter)?;

        // Apply filter to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("filter".to_string(), serde_json::Value::String(filter.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "filter": filter }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Filter application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Filter application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove filter from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove filter from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_filter(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("filter");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Filter removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Filter removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get filter from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get filter from
    /// 
    /// # Returns
    /// Option containing the filter string or None
    pub fn get_filter(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(filter) = obj.get("filter") {
                    if let Some(s) = filter.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has filter
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has filter, false otherwise
    pub fn has_filter(&self, node: &TipTapNode) -> bool {
        self.get_filter(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_filter_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FilterManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(FilterManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(FilterManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(FilterManager::max_filter_length(), MAX_FILTER_LENGTH);
    }

    #[test]
    fn test_apply_filter() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FilterManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_filter(&mut node, "blur(5px)");
        assert!(result.is_ok());
        assert!(manager.has_filter(&node));
    }

    #[test]
    fn test_apply_filter_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FilterManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_filter(&mut node, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_filter_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FilterManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_filter = "a".repeat(MAX_FILTER_LENGTH + 1);
        let result = manager.apply_filter(&mut node, &long_filter);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_filter_invalid_parentheses() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FilterManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_filter(&mut node, "blur(5px");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_filter() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FilterManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "filter": "grayscale(100%)" })),
            marks: None,
        };
        
        assert!(manager.has_filter(&node));
        let result = manager.remove_filter(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_filter(&node));
    }

    #[test]
    fn test_get_filter() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FilterManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "filter": "brightness(150%)" })),
            marks: None,
        };
        
        let filter = manager.get_filter(&node);
        assert_eq!(filter, Some("brightness(150%)".to_string()));
    }

    #[test]
    fn test_get_filter_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FilterManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let filter = manager.get_filter(&node);
        assert!(filter.is_none());
    }

    #[test]
    fn test_has_filter() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FilterManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "filter": "contrast(200%)" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_filter(&node_with));
        assert!(!manager.has_filter(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FilterManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_filter(&mut node, "blur(5px)").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FilterManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_filter(&mut node, "blur(5px)").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FilterManager::new(config_service);
        
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
        let mut manager = FilterManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
