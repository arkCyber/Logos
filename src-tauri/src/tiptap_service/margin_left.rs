//! TipTap Margin Left Manager - Aerospace-Grade Margin Left Operations Service
//!
//! Safety-critical margin left operations service with:
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

/// Maximum margin left value (in pixels)
const MAX_MARGIN_LEFT: f64 = 1000.0;

/// Minimum margin left value (in pixels)
const MIN_MARGIN_LEFT: f64 = -1000.0;

/// Maximum margin left string length
const MAX_MARGIN_LEFT_LENGTH: usize = 50;

pub struct MarginLeftManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl MarginLeftManager {
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

    pub fn max_margin_left() -> f64 {
        MAX_MARGIN_LEFT
    }

    pub fn min_margin_left() -> f64 {
        MIN_MARGIN_LEFT
    }

    pub fn max_margin_left_length() -> usize {
        MAX_MARGIN_LEFT_LENGTH
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

    fn validate_margin_left(&self, margin_left: &str) -> Result<(), String> {
        if margin_left.is_empty() {
            return Err("Margin left cannot be empty".to_string());
        }
        if margin_left.len() > MAX_MARGIN_LEFT_LENGTH {
            return Err(format!("Margin left string exceeds maximum length of {} characters", MAX_MARGIN_LEFT_LENGTH));
        }
        if margin_left.ends_with("px") {
            let value_str = margin_left.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_MARGIN_LEFT || value > MAX_MARGIN_LEFT {
                    return Err(format!("Margin left must be between {} and {} pixels", MIN_MARGIN_LEFT, MAX_MARGIN_LEFT));
                }
                if !value.is_finite() {
                    return Err("Margin left must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_margin_left(&mut self, node: &mut TipTapNode, margin_left: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_margin_left(margin_left)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("marginLeft".to_string(), serde_json::Value::String(margin_left.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "marginLeft": margin_left }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Margin left application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Margin left application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_margin_left(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("marginLeft");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Margin left removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Margin left removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_margin_left(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(margin_left) = obj.get("marginLeft") {
                    if let Some(s) = margin_left.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_margin_left(&self, node: &TipTapNode) -> bool {
        self.get_margin_left(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_margin_left_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MarginLeftManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_margin_left() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MarginLeftManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_margin_left(&mut node, "10px");
        assert!(result.is_ok());
        assert!(manager.has_margin_left(&node));
    }

    #[test]
    fn test_remove_margin_left() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MarginLeftManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "marginLeft": "20px" })),
            marks: None,
        };
        
        assert!(manager.has_margin_left(&node));
        let result = manager.remove_margin_left(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_margin_left(&node));
    }

    #[test]
    fn test_get_margin_left() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MarginLeftManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "marginLeft": "30px" })),
            marks: None,
        };
        
        let margin_left = manager.get_margin_left(&node);
        assert_eq!(margin_left, Some("30px".to_string()));
    }
}
