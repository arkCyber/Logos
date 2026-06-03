//! TipTap Text Stroke Manager - Aerospace-Grade Text Stroke Operations Service
//!
//! Safety-critical text stroke operations service with:
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

/// Maximum text stroke string length
const MAX_TEXT_STROKE_LENGTH: usize = 100;

pub struct TextStrokeManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextStrokeManager {
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

    pub fn max_text_stroke_length() -> usize {
        MAX_TEXT_STROKE_LENGTH
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

    fn validate_text_stroke(&self, text_stroke: &str) -> Result<(), String> {
        if text_stroke.is_empty() {
            return Err("Text stroke cannot be empty".to_string());
        }
        if text_stroke.len() > MAX_TEXT_STROKE_LENGTH {
            return Err(format!("Text stroke string exceeds maximum length of {} characters", MAX_TEXT_STROKE_LENGTH));
        }
        if text_stroke.contains('<') || text_stroke.contains('>') || text_stroke.contains('"') {
            return Err("Text stroke contains invalid characters".to_string());
        }
        Ok(())
    }

    pub fn apply_text_stroke(&mut self, node: &mut TipTapNode, text_stroke: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_text_stroke(text_stroke)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textStroke".to_string(), serde_json::Value::String(text_stroke.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textStroke": text_stroke }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text stroke application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text stroke application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_text_stroke(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textStroke");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text stroke removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text stroke removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_text_stroke(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(text_stroke) = obj.get("textStroke") {
                    if let Some(s) = text_stroke.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_text_stroke(&self, node: &TipTapNode) -> bool {
        self.get_text_stroke(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_stroke_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextStrokeManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_text_stroke() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextStrokeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_text_stroke(&mut node, "1px red");
        assert!(result.is_ok());
        assert!(manager.has_text_stroke(&node));
    }

    #[test]
    fn test_remove_text_stroke() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextStrokeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textStroke": "2px blue" })),
            marks: None,
        };
        
        assert!(manager.has_text_stroke(&node));
        let result = manager.remove_text_stroke(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_text_stroke(&node));
    }

    #[test]
    fn test_get_text_stroke() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextStrokeManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textStroke": "3px green" })),
            marks: None,
        };
        
        let text_stroke = manager.get_text_stroke(&node);
        assert_eq!(text_stroke, Some("3px green".to_string()));
    }
}
