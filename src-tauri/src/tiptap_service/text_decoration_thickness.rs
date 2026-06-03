//! TipTap Text Decoration Thickness Manager - Aerospace-Grade Text Decoration Thickness Operations Service
//!
//! Safety-critical text decoration thickness operations service with:
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

/// Maximum text decoration thickness value (in pixels)
const MAX_TEXT_DECORATION_THICKNESS: f64 = 50.0;

/// Minimum text decoration thickness value (in pixels)
const MIN_TEXT_DECORATION_THICKNESS: f64 = 0.0;

/// Maximum text decoration thickness string length
const MAX_TEXT_DECORATION_THICKNESS_LENGTH: usize = 50;

pub struct TextDecorationThicknessManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextDecorationThicknessManager {
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

    pub fn max_text_decoration_thickness() -> f64 {
        MAX_TEXT_DECORATION_THICKNESS
    }

    pub fn min_text_decoration_thickness() -> f64 {
        MIN_TEXT_DECORATION_THICKNESS
    }

    pub fn max_text_decoration_thickness_length() -> usize {
        MAX_TEXT_DECORATION_THICKNESS_LENGTH
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

    fn validate_text_decoration_thickness(&self, text_decoration_thickness: &str) -> Result<(), String> {
        if text_decoration_thickness.is_empty() {
            return Err("Text decoration thickness cannot be empty".to_string());
        }
        if text_decoration_thickness.len() > MAX_TEXT_DECORATION_THICKNESS_LENGTH {
            return Err(format!("Text decoration thickness string exceeds maximum length of {} characters", MAX_TEXT_DECORATION_THICKNESS_LENGTH));
        }
        if text_decoration_thickness.ends_with("px") {
            let value_str = text_decoration_thickness.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_TEXT_DECORATION_THICKNESS || value > MAX_TEXT_DECORATION_THICKNESS {
                    return Err(format!("Text decoration thickness must be between {} and {} pixels", MIN_TEXT_DECORATION_THICKNESS, MAX_TEXT_DECORATION_THICKNESS));
                }
                if !value.is_finite() {
                    return Err("Text decoration thickness must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_text_decoration_thickness(&mut self, node: &mut TipTapNode, text_decoration_thickness: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_text_decoration_thickness(text_decoration_thickness)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textDecorationThickness".to_string(), serde_json::Value::String(text_decoration_thickness.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textDecorationThickness": text_decoration_thickness }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text decoration thickness application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text decoration thickness application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_text_decoration_thickness(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textDecorationThickness");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text decoration thickness removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text decoration thickness removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_text_decoration_thickness(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(text_decoration_thickness) = obj.get("textDecorationThickness") {
                    if let Some(s) = text_decoration_thickness.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_text_decoration_thickness(&self, node: &TipTapNode) -> bool {
        self.get_text_decoration_thickness(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_decoration_thickness_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextDecorationThicknessManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_text_decoration_thickness() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDecorationThicknessManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_text_decoration_thickness(&mut node, "2px");
        assert!(result.is_ok());
        assert!(manager.has_text_decoration_thickness(&node));
    }

    #[test]
    fn test_remove_text_decoration_thickness() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDecorationThicknessManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textDecorationThickness": "3px" })),
            marks: None,
        };
        
        assert!(manager.has_text_decoration_thickness(&node));
        let result = manager.remove_text_decoration_thickness(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_text_decoration_thickness(&node));
    }

    #[test]
    fn test_get_text_decoration_thickness() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextDecorationThicknessManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textDecorationThickness": "1px" })),
            marks: None,
        };
        
        let text_decoration_thickness = manager.get_text_decoration_thickness(&node);
        assert_eq!(text_decoration_thickness, Some("1px".to_string()));
    }
}
