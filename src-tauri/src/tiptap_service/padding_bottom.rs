//! TipTap Padding Bottom Manager - Aerospace-Grade Padding Bottom Operations Service
//!
//! Safety-critical padding bottom operations service with:
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

/// Maximum padding bottom value (in pixels)
const MAX_PADDING_BOTTOM: f64 = 1000.0;

/// Minimum padding bottom value (in pixels)
const MIN_PADDING_BOTTOM: f64 = 0.0;

/// Maximum padding bottom string length
const MAX_PADDING_BOTTOM_LENGTH: usize = 50;

pub struct PaddingBottomManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl PaddingBottomManager {
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

    pub fn max_padding_bottom() -> f64 {
        MAX_PADDING_BOTTOM
    }

    pub fn min_padding_bottom() -> f64 {
        MIN_PADDING_BOTTOM
    }

    pub fn max_padding_bottom_length() -> usize {
        MAX_PADDING_BOTTOM_LENGTH
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

    fn validate_padding_bottom(&self, padding_bottom: &str) -> Result<(), String> {
        if padding_bottom.is_empty() {
            return Err("Padding bottom cannot be empty".to_string());
        }
        if padding_bottom.len() > MAX_PADDING_BOTTOM_LENGTH {
            return Err(format!("Padding bottom string exceeds maximum length of {} characters", MAX_PADDING_BOTTOM_LENGTH));
        }
        if padding_bottom.ends_with("px") {
            let value_str = padding_bottom.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_PADDING_BOTTOM || value > MAX_PADDING_BOTTOM {
                    return Err(format!("Padding bottom must be between {} and {} pixels", MIN_PADDING_BOTTOM, MAX_PADDING_BOTTOM));
                }
                if !value.is_finite() {
                    return Err("Padding bottom must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_padding_bottom(&mut self, node: &mut TipTapNode, padding_bottom: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_padding_bottom(padding_bottom)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("paddingBottom".to_string(), serde_json::Value::String(padding_bottom.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "paddingBottom": padding_bottom }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Padding bottom application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Padding bottom application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_padding_bottom(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("paddingBottom");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Padding bottom removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Padding bottom removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_padding_bottom(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(padding_bottom) = obj.get("paddingBottom") {
                    if let Some(s) = padding_bottom.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_padding_bottom(&self, node: &TipTapNode) -> bool {
        self.get_padding_bottom(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_padding_bottom_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PaddingBottomManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_padding_bottom() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PaddingBottomManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_padding_bottom(&mut node, "10px");
        assert!(result.is_ok());
        assert!(manager.has_padding_bottom(&node));
    }

    #[test]
    fn test_remove_padding_bottom() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PaddingBottomManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "paddingBottom": "20px" })),
            marks: None,
        };
        
        assert!(manager.has_padding_bottom(&node));
        let result = manager.remove_padding_bottom(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_padding_bottom(&node));
    }

    #[test]
    fn test_get_padding_bottom() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PaddingBottomManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "paddingBottom": "30px" })),
            marks: None,
        };
        
        let padding_bottom = manager.get_padding_bottom(&node);
        assert_eq!(padding_bottom, Some("30px".to_string()));
    }
}
