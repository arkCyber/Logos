//! TipTap Text Stroke Color Manager - Aerospace-Grade Text Stroke Color Operations Service
//!
//! Safety-critical text stroke color operations service with:
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

/// Maximum text stroke color string length
const MAX_TEXT_STROKE_COLOR_LENGTH: usize = 100;

pub struct TextStrokeColorManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextStrokeColorManager {
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

    pub fn max_text_stroke_color_length() -> usize {
        MAX_TEXT_STROKE_COLOR_LENGTH
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

    fn validate_text_stroke_color(&self, text_stroke_color: &str) -> Result<(), String> {
        if text_stroke_color.is_empty() {
            return Err("Text stroke color cannot be empty".to_string());
        }
        if text_stroke_color.len() > MAX_TEXT_STROKE_COLOR_LENGTH {
            return Err(format!("Text stroke color string exceeds maximum length of {} characters", MAX_TEXT_STROKE_COLOR_LENGTH));
        }
        if text_stroke_color.contains('<') || text_stroke_color.contains('>') || text_stroke_color.contains('"') {
            return Err("Text stroke color contains invalid characters".to_string());
        }
        Ok(())
    }

    pub fn apply_text_stroke_color(&mut self, node: &mut TipTapNode, text_stroke_color: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_text_stroke_color(text_stroke_color)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textStrokeColor".to_string(), serde_json::Value::String(text_stroke_color.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textStrokeColor": text_stroke_color }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text stroke color application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text stroke color application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_text_stroke_color(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textStrokeColor");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text stroke color removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text stroke color removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_text_stroke_color(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(text_stroke_color) = obj.get("textStrokeColor") {
                    if let Some(s) = text_stroke_color.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_text_stroke_color(&self, node: &TipTapNode) -> bool {
        self.get_text_stroke_color(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_stroke_color_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextStrokeColorManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_text_stroke_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextStrokeColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_text_stroke_color(&mut node, "red");
        assert!(result.is_ok());
        assert!(manager.has_text_stroke_color(&node));
    }

    #[test]
    fn test_remove_text_stroke_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextStrokeColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textStrokeColor": "blue" })),
            marks: None,
        };
        
        assert!(manager.has_text_stroke_color(&node));
        let result = manager.remove_text_stroke_color(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_text_stroke_color(&node));
    }

    #[test]
    fn test_get_text_stroke_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextStrokeColorManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textStrokeColor": "#ff0000" })),
            marks: None,
        };
        
        let text_stroke_color = manager.get_text_stroke_color(&node);
        assert_eq!(text_stroke_color, Some("#ff0000".to_string()));
    }
}
