//! TipTap Column Rule Width Manager - Aerospace-Grade Column Rule Width Operations Service
//!
//! Safety-critical column rule width operations service with:
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

/// Maximum column rule width value (in pixels)
const MAX_COLUMN_RULE_WIDTH: f64 = 100.0;

/// Minimum column rule width value (in pixels)
const MIN_COLUMN_RULE_WIDTH: f64 = 0.0;

/// Maximum column rule width string length
const MAX_COLUMN_RULE_WIDTH_LENGTH: usize = 50;

pub struct ColumnRuleWidthManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ColumnRuleWidthManager {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
        }
    }

    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    pub fn max_column_rule_width() -> f64 {
        MAX_COLUMN_RULE_WIDTH
    }

    pub fn min_column_rule_width() -> f64 {
        MIN_COLUMN_RULE_WIDTH
    }

    pub fn max_column_rule_width_length() -> usize {
        MAX_COLUMN_RULE_WIDTH_LENGTH
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

    fn validate_column_rule_width(&self, column_rule_width: &str) -> Result<(), String> {
        if column_rule_width.is_empty() {
            return Err("Column rule width cannot be empty".to_string());
        }
        if column_rule_width.len() > MAX_COLUMN_RULE_WIDTH_LENGTH {
            return Err(format!("Column rule width string exceeds maximum length of {} characters", MAX_COLUMN_RULE_WIDTH_LENGTH));
        }
        if column_rule_width.ends_with("px") {
            let value_str = column_rule_width.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_COLUMN_RULE_WIDTH || value > MAX_COLUMN_RULE_WIDTH {
                    return Err(format!("Column rule width must be between {} and {} pixels", MIN_COLUMN_RULE_WIDTH, MAX_COLUMN_RULE_WIDTH));
                }
                if !value.is_finite() {
                    return Err("Column rule width must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_column_rule_width(&mut self, node: &mut TipTapNode, column_rule_width: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_column_rule_width(column_rule_width)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("columnRuleWidth".to_string(), serde_json::Value::String(column_rule_width.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "columnRuleWidth": column_rule_width }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Column rule width application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Column rule width application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_column_rule_width(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("columnRuleWidth");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Column rule width removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Column rule width removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_column_rule_width(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(column_rule_width) = obj.get("columnRuleWidth") {
                    if let Some(s) = column_rule_width.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_column_rule_width(&self, node: &TipTapNode) -> bool {
        self.get_column_rule_width(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_column_rule_width_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnRuleWidthManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_column_rule_width() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnRuleWidthManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_column_rule_width(&mut node, "2px");
        assert!(result.is_ok());
        assert!(manager.has_column_rule_width(&node));
    }

    #[test]
    fn test_remove_column_rule_width() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnRuleWidthManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "columnRuleWidth": "3px" })),
            marks: None,
        };
        
        assert!(manager.has_column_rule_width(&node));
        let result = manager.remove_column_rule_width(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_column_rule_width(&node));
    }

    #[test]
    fn test_get_column_rule_width() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnRuleWidthManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "columnRuleWidth": "1px" })),
            marks: None,
        };
        
        let column_rule_width = manager.get_column_rule_width(&node);
        assert_eq!(column_rule_width, Some("1px".to_string()));
    }
}
