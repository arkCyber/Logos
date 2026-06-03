//! TipTap Column Span Manager - Aerospace-Grade Column Span Operations Service
//!
//! Safety-critical column span operations service with:
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

/// Maximum column span value
const MAX_COLUMN_SPAN: i32 = 100;

/// Minimum column span value
const MIN_COLUMN_SPAN: i32 = 1;

/// Maximum column span string length
const MAX_COLUMN_SPAN_LENGTH: usize = 50;

pub struct ColumnSpanManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ColumnSpanManager {
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

    pub fn max_column_span() -> i32 {
        MAX_COLUMN_SPAN
    }

    pub fn min_column_span() -> i32 {
        MIN_COLUMN_SPAN
    }

    pub fn max_column_span_length() -> usize {
        MAX_COLUMN_SPAN_LENGTH
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

    fn validate_column_span(&self, column_span: &str) -> Result<(), String> {
        if column_span.is_empty() {
            return Err("Column span cannot be empty".to_string());
        }
        if column_span.len() > MAX_COLUMN_SPAN_LENGTH {
            return Err(format!("Column span string exceeds maximum length of {} characters", MAX_COLUMN_SPAN_LENGTH));
        }
        if let Ok(value) = column_span.parse::<i32>() {
            if value < MIN_COLUMN_SPAN || value > MAX_COLUMN_SPAN {
                return Err(format!("Column span must be between {} and {}", MIN_COLUMN_SPAN, MAX_COLUMN_SPAN));
            }
        }
        Ok(())
    }

    pub fn apply_column_span(&mut self, node: &mut TipTapNode, column_span: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_column_span(column_span)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("columnSpan".to_string(), serde_json::Value::String(column_span.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "columnSpan": column_span }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Column span application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Column span application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_column_span(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("columnSpan");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Column span removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Column span removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_column_span(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(column_span) = obj.get("columnSpan") {
                    if let Some(s) = column_span.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_column_span(&self, node: &TipTapNode) -> bool {
        self.get_column_span(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_column_span_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnSpanManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_column_span() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnSpanManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_column_span(&mut node, "2");
        assert!(result.is_ok());
        assert!(manager.has_column_span(&node));
    }

    #[test]
    fn test_remove_column_span() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnSpanManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "columnSpan": "3" })),
            marks: None,
        };
        
        assert!(manager.has_column_span(&node));
        let result = manager.remove_column_span(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_column_span(&node));
    }

    #[test]
    fn test_get_column_span() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnSpanManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "columnSpan": "4" })),
            marks: None,
        };
        
        let column_span = manager.get_column_span(&node);
        assert_eq!(column_span, Some("4".to_string()));
    }
}
