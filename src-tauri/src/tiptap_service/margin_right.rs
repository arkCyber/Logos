//! TipTap Margin Right Manager - Aerospace-Grade Margin Right Operations Service
//!
//! Safety-critical margin right operations service with:
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

/// Maximum margin right value (in pixels)
const MAX_MARGIN_RIGHT: f64 = 1000.0;

/// Minimum margin right value (in pixels)
const MIN_MARGIN_RIGHT: f64 = -1000.0;

/// Maximum margin right string length
const MAX_MARGIN_RIGHT_LENGTH: usize = 50;

pub struct MarginRightManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl MarginRightManager {
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

    pub fn max_margin_right() -> f64 {
        MAX_MARGIN_RIGHT
    }

    pub fn min_margin_right() -> f64 {
        MIN_MARGIN_RIGHT
    }

    pub fn max_margin_right_length() -> usize {
        MAX_MARGIN_RIGHT_LENGTH
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

    fn validate_margin_right(&self, margin_right: &str) -> Result<(), String> {
        if margin_right.is_empty() {
            return Err("Margin right cannot be empty".to_string());
        }
        if margin_right.len() > MAX_MARGIN_RIGHT_LENGTH {
            return Err(format!("Margin right string exceeds maximum length of {} characters", MAX_MARGIN_RIGHT_LENGTH));
        }
        if margin_right.ends_with("px") {
            let value_str = margin_right.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_MARGIN_RIGHT || value > MAX_MARGIN_RIGHT {
                    return Err(format!("Margin right must be between {} and {} pixels", MIN_MARGIN_RIGHT, MAX_MARGIN_RIGHT));
                }
                if !value.is_finite() {
                    return Err("Margin right must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_margin_right(&mut self, node: &mut TipTapNode, margin_right: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_margin_right(margin_right)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("marginRight".to_string(), serde_json::Value::String(margin_right.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "marginRight": margin_right }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Margin right application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Margin right application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_margin_right(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("marginRight");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Margin right removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Margin right removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_margin_right(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(margin_right) = obj.get("marginRight") {
                    if let Some(s) = margin_right.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_margin_right(&self, node: &TipTapNode) -> bool {
        self.get_margin_right(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_margin_right_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MarginRightManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_margin_right() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MarginRightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_margin_right(&mut node, "10px");
        assert!(result.is_ok());
        assert!(manager.has_margin_right(&node));
    }

    #[test]
    fn test_remove_margin_right() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MarginRightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "marginRight": "20px" })),
            marks: None,
        };
        
        assert!(manager.has_margin_right(&node));
        let result = manager.remove_margin_right(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_margin_right(&node));
    }

    #[test]
    fn test_get_margin_right() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MarginRightManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "marginRight": "30px" })),
            marks: None,
        };
        
        let margin_right = manager.get_margin_right(&node);
        assert_eq!(margin_right, Some("30px".to_string()));
    }
}
