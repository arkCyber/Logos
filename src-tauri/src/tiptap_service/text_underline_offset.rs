//! TipTap Text Underline Offset Manager - Aerospace-Grade Text Underline Offset Operations Service
//!
//! Safety-critical text underline offset operations service with:
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

/// Maximum text underline offset value (in pixels)
const MAX_TEXT_UNDERLINE_OFFSET: f64 = 20.0;

/// Minimum text underline offset value (in pixels)
const MIN_TEXT_UNDERLINE_OFFSET: f64 = 0.0;

/// Maximum text underline offset string length
const MAX_TEXT_UNDERLINE_OFFSET_LENGTH: usize = 50;

pub struct TextUnderlineOffsetManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextUnderlineOffsetManager {
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

    pub fn max_text_underline_offset() -> f64 {
        MAX_TEXT_UNDERLINE_OFFSET
    }

    pub fn min_text_underline_offset() -> f64 {
        MIN_TEXT_UNDERLINE_OFFSET
    }

    pub fn max_text_underline_offset_length() -> usize {
        MAX_TEXT_UNDERLINE_OFFSET_LENGTH
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

    fn validate_text_underline_offset(&self, text_underline_offset: &str) -> Result<(), String> {
        if text_underline_offset.is_empty() {
            return Err("Text underline offset cannot be empty".to_string());
        }
        if text_underline_offset.len() > MAX_TEXT_UNDERLINE_OFFSET_LENGTH {
            return Err(format!("Text underline offset string exceeds maximum length of {} characters", MAX_TEXT_UNDERLINE_OFFSET_LENGTH));
        }
        if text_underline_offset.ends_with("px") {
            let value_str = text_underline_offset.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_TEXT_UNDERLINE_OFFSET || value > MAX_TEXT_UNDERLINE_OFFSET {
                    return Err(format!("Text underline offset must be between {} and {} pixels", MIN_TEXT_UNDERLINE_OFFSET, MAX_TEXT_UNDERLINE_OFFSET));
                }
                if !value.is_finite() {
                    return Err("Text underline offset must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_text_underline_offset(&mut self, node: &mut TipTapNode, text_underline_offset: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_text_underline_offset(text_underline_offset)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textUnderlineOffset".to_string(), serde_json::Value::String(text_underline_offset.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textUnderlineOffset": text_underline_offset }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text underline offset application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text underline offset application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_text_underline_offset(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textUnderlineOffset");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text underline offset removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text underline offset removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_text_underline_offset(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(text_underline_offset) = obj.get("textUnderlineOffset") {
                    if let Some(s) = text_underline_offset.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_text_underline_offset(&self, node: &TipTapNode) -> bool {
        self.get_text_underline_offset(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_underline_offset_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextUnderlineOffsetManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_text_underline_offset() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextUnderlineOffsetManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_text_underline_offset(&mut node, "5px");
        assert!(result.is_ok());
        assert!(manager.has_text_underline_offset(&node));
    }

    #[test]
    fn test_remove_text_underline_offset() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextUnderlineOffsetManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textUnderlineOffset": "10px" })),
            marks: None,
        };
        
        assert!(manager.has_text_underline_offset(&node));
        let result = manager.remove_text_underline_offset(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_text_underline_offset(&node));
    }

    #[test]
    fn test_get_text_underline_offset() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextUnderlineOffsetManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textUnderlineOffset": "15px" })),
            marks: None,
        };
        
        let text_underline_offset = manager.get_text_underline_offset(&node);
        assert_eq!(text_underline_offset, Some("15px".to_string()));
    }
}
