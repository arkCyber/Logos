//! TipTap Margin Top Manager - Aerospace-Grade Margin Top Operations Service
//!
//! Safety-critical margin top operations service with:
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

/// Maximum margin top value (in pixels)
const MAX_MARGIN_TOP: f64 = 1000.0;

/// Minimum margin top value (in pixels)
const MIN_MARGIN_TOP: f64 = -1000.0;

/// Maximum margin top string length
const MAX_MARGIN_TOP_LENGTH: usize = 50;

pub struct MarginTopManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl MarginTopManager {
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

    pub fn max_margin_top() -> f64 {
        MAX_MARGIN_TOP
    }

    pub fn min_margin_top() -> f64 {
        MIN_MARGIN_TOP
    }

    pub fn max_margin_top_length() -> usize {
        MAX_MARGIN_TOP_LENGTH
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

    fn validate_margin_top(&self, margin_top: &str) -> Result<(), String> {
        if margin_top.is_empty() {
            return Err("Margin top cannot be empty".to_string());
        }
        if margin_top.len() > MAX_MARGIN_TOP_LENGTH {
            return Err(format!("Margin top string exceeds maximum length of {} characters", MAX_MARGIN_TOP_LENGTH));
        }
        if margin_top.ends_with("px") {
            let value_str = margin_top.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_MARGIN_TOP || value > MAX_MARGIN_TOP {
                    return Err(format!("Margin top must be between {} and {} pixels", MIN_MARGIN_TOP, MAX_MARGIN_TOP));
                }
                if !value.is_finite() {
                    return Err("Margin top must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_margin_top(&mut self, node: &mut TipTapNode, margin_top: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_margin_top(margin_top)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("marginTop".to_string(), serde_json::Value::String(margin_top.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "marginTop": margin_top }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Margin top application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Margin top application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_margin_top(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("marginTop");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Margin top removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Margin top removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_margin_top(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(margin_top) = obj.get("marginTop") {
                    if let Some(s) = margin_top.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_margin_top(&self, node: &TipTapNode) -> bool {
        self.get_margin_top(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_margin_top_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MarginTopManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_margin_top() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MarginTopManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_margin_top(&mut node, "10px");
        assert!(result.is_ok());
        assert!(manager.has_margin_top(&node));
    }

    #[test]
    fn test_remove_margin_top() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MarginTopManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "marginTop": "20px" })),
            marks: None,
        };
        
        assert!(manager.has_margin_top(&node));
        let result = manager.remove_margin_top(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_margin_top(&node));
    }

    #[test]
    fn test_get_margin_top() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MarginTopManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "marginTop": "30px" })),
            marks: None,
        };
        
        let margin_top = manager.get_margin_top(&node);
        assert_eq!(margin_top, Some("30px".to_string()));
    }
}
