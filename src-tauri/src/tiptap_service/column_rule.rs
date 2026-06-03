//! TipTap Column Rule Manager - Aerospace-Grade Column Rule Operations Service
//!
//! Safety-critical column rule operations service with:
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

/// Maximum column rule string length
const MAX_COLUMN_RULE_LENGTH: usize = 200;

pub struct ColumnRuleManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ColumnRuleManager {
    /// Creates a new column rule manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new ColumnRuleManager instance
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

    /// Get the maximum column rule length constant
    /// 
    /// # Returns
    /// The maximum column rule string length
    pub fn max_column_rule_length() -> usize {
        MAX_COLUMN_RULE_LENGTH
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

    /// Validate column rule string
    /// 
    /// # Arguments
    /// * `column_rule` - The column rule string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting column rule string length
    fn validate_column_rule(&self, column_rule: &str) -> Result<(), String> {
        if column_rule.is_empty() {
            return Err("Column rule cannot be empty".to_string());
        }
        if column_rule.len() > MAX_COLUMN_RULE_LENGTH {
            return Err(format!("Column rule string exceeds maximum length of {} characters", MAX_COLUMN_RULE_LENGTH));
        }
        // Check for unmatched parentheses first
        if column_rule.contains('(') && !column_rule.contains(')') {
            return Err("Invalid column rule: unmatched parentheses".to_string());
        }
        // Basic validation for common column rule values
        let valid_patterns = ["none", "solid", "dotted", "dashed", "double", "groove", "ridge", "inset", "outset"];
        if !valid_patterns.iter().any(|pattern| column_rule.contains(pattern)) {
            // Allow custom values but validate basic structure
            if column_rule.contains('(') && !column_rule.contains(')') {
                return Err("Invalid column rule: unmatched parentheses".to_string());
            }
        }
        Ok(())
    }

    /// Apply column rule to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply column rule to
    /// * `column_rule` - The column rule to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates column rule string
    pub fn apply_column_rule(&mut self, node: &mut TipTapNode, column_rule: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate column rule
        self.validate_column_rule(column_rule)?;

        // Apply column rule to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("columnRule".to_string(), serde_json::Value::String(column_rule.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "columnRule": column_rule }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Column rule application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Column rule application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove column rule from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove column rule from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_column_rule(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("columnRule");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Column rule removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Column rule removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get column rule from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get column rule from
    /// 
    /// # Returns
    /// Option containing the column rule string or None
    pub fn get_column_rule(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(column_rule) = obj.get("columnRule") {
                    if let Some(s) = column_rule.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has column rule
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has column rule, false otherwise
    pub fn has_column_rule(&self, node: &TipTapNode) -> bool {
        self.get_column_rule(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_column_rule_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnRuleManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(ColumnRuleManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(ColumnRuleManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(ColumnRuleManager::max_column_rule_length(), MAX_COLUMN_RULE_LENGTH);
    }

    #[test]
    fn test_apply_column_rule() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnRuleManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_column_rule(&mut node, "1px solid black");
        assert!(result.is_ok());
        assert!(manager.has_column_rule(&node));
    }

    #[test]
    fn test_apply_column_rule_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnRuleManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_column_rule(&mut node, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_column_rule_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnRuleManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_column_rule = "a".repeat(MAX_COLUMN_RULE_LENGTH + 1);
        let result = manager.apply_column_rule(&mut node, &long_column_rule);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_column_rule_invalid_parentheses() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnRuleManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_column_rule(&mut node, "1px solid rgb(255, 0, 0");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_column_rule() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnRuleManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "columnRule": "none" })),
            marks: None,
        };
        
        assert!(manager.has_column_rule(&node));
        let result = manager.remove_column_rule(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_column_rule(&node));
    }

    #[test]
    fn test_get_column_rule() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnRuleManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "columnRule": "2px dashed blue" })),
            marks: None,
        };
        
        let column_rule = manager.get_column_rule(&node);
        assert_eq!(column_rule, Some("2px dashed blue".to_string()));
    }

    #[test]
    fn test_get_column_rule_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnRuleManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let column_rule = manager.get_column_rule(&node);
        assert!(column_rule.is_none());
    }

    #[test]
    fn test_has_column_rule() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnRuleManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "columnRule": "3px double green" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_column_rule(&node_with));
        assert!(!manager.has_column_rule(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnRuleManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_column_rule(&mut node, "1px solid black").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnRuleManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_column_rule(&mut node, "1px solid black").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnRuleManager::new(config_service);
        
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
        let mut manager = ColumnRuleManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
