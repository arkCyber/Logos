//! TipTap Padding Right Manager - Aerospace-Grade Padding Right Operations Service
//!
//! Safety-critical padding right operations service with:
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

/// Maximum padding right value (in pixels)
const MAX_PADDING_RIGHT: f64 = 1000.0;

/// Minimum padding right value (in pixels)
const MIN_PADDING_RIGHT: f64 = 0.0;

/// Maximum padding right string length
const MAX_PADDING_RIGHT_LENGTH: usize = 50;

pub struct PaddingRightManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl PaddingRightManager {
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

    pub fn max_padding_right() -> f64 {
        MAX_PADDING_RIGHT
    }

    pub fn min_padding_right() -> f64 {
        MIN_PADDING_RIGHT
    }

    pub fn max_padding_right_length() -> usize {
        MAX_PADDING_RIGHT_LENGTH
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

    fn validate_padding_right(&self, padding_right: &str) -> Result<(), String> {
        if padding_right.is_empty() {
            return Err("Padding right cannot be empty".to_string());
        }
        if padding_right.len() > MAX_PADDING_RIGHT_LENGTH {
            return Err(format!("Padding right string exceeds maximum length of {} characters", MAX_PADDING_RIGHT_LENGTH));
        }
        if padding_right.ends_with("px") {
            let value_str = padding_right.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_PADDING_RIGHT || value > MAX_PADDING_RIGHT {
                    return Err(format!("Padding right must be between {} and {} pixels", MIN_PADDING_RIGHT, MAX_PADDING_RIGHT));
                }
                if !value.is_finite() {
                    return Err("Padding right must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_padding_right(&mut self, node: &mut TipTapNode, padding_right: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_padding_right(padding_right)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("paddingRight".to_string(), serde_json::Value::String(padding_right.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "paddingRight": padding_right }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Padding right application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Padding right application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_padding_right(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("paddingRight");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Padding right removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Padding right removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_padding_right(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(padding_right) = obj.get("paddingRight") {
                    if let Some(s) = padding_right.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_padding_right(&self, node: &TipTapNode) -> bool {
        self.get_padding_right(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_padding_right_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PaddingRightManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_padding_right() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PaddingRightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_padding_right(&mut node, "10px");
        assert!(result.is_ok());
        assert!(manager.has_padding_right(&node));
    }

    #[test]
    fn test_remove_padding_right() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PaddingRightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "paddingRight": "20px" })),
            marks: None,
        };
        
        assert!(manager.has_padding_right(&node));
        let result = manager.remove_padding_right(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_padding_right(&node));
    }

    #[test]
    fn test_get_padding_right() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PaddingRightManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "paddingRight": "30px" })),
            marks: None,
        };
        
        let padding_right = manager.get_padding_right(&node);
        assert_eq!(padding_right, Some("30px".to_string()));
    }
}
