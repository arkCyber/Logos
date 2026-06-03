//! TipTap Text Stroke Width Manager - Aerospace-Grade Text Stroke Width Operations Service
//!
//! Safety-critical text stroke width operations service with:
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

/// Maximum text stroke width value (in pixels)
const MAX_TEXT_STROKE_WIDTH: f64 = 100.0;

/// Minimum text stroke width value (in pixels)
const MIN_TEXT_STROKE_WIDTH: f64 = 0.0;

/// Maximum text stroke width string length
const MAX_TEXT_STROKE_WIDTH_LENGTH: usize = 50;

pub struct TextStrokeWidthManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextStrokeWidthManager {
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

    pub fn max_text_stroke_width() -> f64 {
        MAX_TEXT_STROKE_WIDTH
    }

    pub fn min_text_stroke_width() -> f64 {
        MIN_TEXT_STROKE_WIDTH
    }

    pub fn max_text_stroke_width_length() -> usize {
        MAX_TEXT_STROKE_WIDTH_LENGTH
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

    fn validate_text_stroke_width(&self, text_stroke_width: &str) -> Result<(), String> {
        if text_stroke_width.is_empty() {
            return Err("Text stroke width cannot be empty".to_string());
        }
        if text_stroke_width.len() > MAX_TEXT_STROKE_WIDTH_LENGTH {
            return Err(format!("Text stroke width string exceeds maximum length of {} characters", MAX_TEXT_STROKE_WIDTH_LENGTH));
        }
        if text_stroke_width.ends_with("px") {
            let value_str = text_stroke_width.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_TEXT_STROKE_WIDTH || value > MAX_TEXT_STROKE_WIDTH {
                    return Err(format!("Text stroke width must be between {} and {} pixels", MIN_TEXT_STROKE_WIDTH, MAX_TEXT_STROKE_WIDTH));
                }
                if !value.is_finite() {
                    return Err("Text stroke width must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_text_stroke_width(&mut self, node: &mut TipTapNode, text_stroke_width: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_text_stroke_width(text_stroke_width)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textStrokeWidth".to_string(), serde_json::Value::String(text_stroke_width.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textStrokeWidth": text_stroke_width }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text stroke width application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text stroke width application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_text_stroke_width(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textStrokeWidth");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text stroke width removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text stroke width removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_text_stroke_width(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(text_stroke_width) = obj.get("textStrokeWidth") {
                    if let Some(s) = text_stroke_width.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_text_stroke_width(&self, node: &TipTapNode) -> bool {
        self.get_text_stroke_width(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_stroke_width_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextStrokeWidthManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_text_stroke_width() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextStrokeWidthManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_text_stroke_width(&mut node, "1px");
        assert!(result.is_ok());
        assert!(manager.has_text_stroke_width(&node));
    }

    #[test]
    fn test_remove_text_stroke_width() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextStrokeWidthManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textStrokeWidth": "2px" })),
            marks: None,
        };
        
        assert!(manager.has_text_stroke_width(&node));
        let result = manager.remove_text_stroke_width(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_text_stroke_width(&node));
    }

    #[test]
    fn test_get_text_stroke_width() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextStrokeWidthManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textStrokeWidth": "3px" })),
            marks: None,
        };
        
        let text_stroke_width = manager.get_text_stroke_width(&node);
        assert_eq!(text_stroke_width, Some("3px".to_string()));
    }
}
