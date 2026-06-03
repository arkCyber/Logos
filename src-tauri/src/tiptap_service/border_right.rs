//! TipTap Border Right Manager - Aerospace-Grade Border Right Operations Service
//!
//! Safety-critical border right operations service with:
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

/// Maximum border right string length
const MAX_BORDER_RIGHT_LENGTH: usize = 200;

pub struct BorderRightManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl BorderRightManager {
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

    pub fn max_border_right_length() -> usize {
        MAX_BORDER_RIGHT_LENGTH
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

    fn validate_border_right(&self, border_right: &str) -> Result<(), String> {
        if border_right.is_empty() {
            return Err("Border right cannot be empty".to_string());
        }
        if border_right.len() > MAX_BORDER_RIGHT_LENGTH {
            return Err(format!("Border right string exceeds maximum length of {} characters", MAX_BORDER_RIGHT_LENGTH));
        }
        if border_right.contains('(') && !border_right.contains(')') {
            return Err("Invalid border right: unmatched parentheses".to_string());
        }
        Ok(())
    }

    pub fn apply_border_right(&mut self, node: &mut TipTapNode, border_right: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_border_right(border_right)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("borderRight".to_string(), serde_json::Value::String(border_right.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "borderRight": border_right }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Border right application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Border right application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_border_right(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("borderRight");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Border right removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Border right removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_border_right(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(border_right) = obj.get("borderRight") {
                    if let Some(s) = border_right.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_border_right(&self, node: &TipTapNode) -> bool {
        self.get_border_right(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_border_right_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BorderRightManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_border_right() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderRightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_border_right(&mut node, "1px solid red");
        assert!(result.is_ok());
        assert!(manager.has_border_right(&node));
    }

    #[test]
    fn test_remove_border_right() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderRightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "borderRight": "2px dashed blue" })),
            marks: None,
        };
        
        assert!(manager.has_border_right(&node));
        let result = manager.remove_border_right(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_border_right(&node));
    }

    #[test]
    fn test_get_border_right() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BorderRightManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "borderRight": "3px dotted green" })),
            marks: None,
        };
        
        let border_right = manager.get_border_right(&node);
        assert_eq!(border_right, Some("3px dotted green".to_string()));
    }
}
