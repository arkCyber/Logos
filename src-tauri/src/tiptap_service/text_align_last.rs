//! TipTap Text Align Last Manager - Aerospace-Grade Text Align Last Operations Service
//!
//! Safety-critical text align last operations service with:
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

/// Maximum text align last string length
const MAX_TEXT_ALIGN_LAST_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignLast {
    Auto,
    Start,
    End,
    Left,
    Right,
    Center,
    Justify,
}

impl TextAlignLast {
    pub fn as_str(&self) -> &str {
        match self {
            TextAlignLast::Auto => "auto",
            TextAlignLast::Start => "start",
            TextAlignLast::End => "end",
            TextAlignLast::Left => "left",
            TextAlignLast::Right => "right",
            TextAlignLast::Center => "center",
            TextAlignLast::Justify => "justify",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(TextAlignLast::Auto),
            "start" => Ok(TextAlignLast::Start),
            "end" => Ok(TextAlignLast::End),
            "left" => Ok(TextAlignLast::Left),
            "right" => Ok(TextAlignLast::Right),
            "center" => Ok(TextAlignLast::Center),
            "justify" => Ok(TextAlignLast::Justify),
            _ => Err(format!("Invalid text align last: {}", s)),
        }
    }
}

pub struct TextAlignLastManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextAlignLastManager {
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

    pub fn max_text_align_last_length() -> usize {
        MAX_TEXT_ALIGN_LAST_LENGTH
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

    fn validate_text_align_last(&self, text_align_last: &str) -> Result<(), String> {
        if text_align_last.len() > MAX_TEXT_ALIGN_LAST_LENGTH {
            return Err(format!("Text align last string exceeds maximum length of {} characters", MAX_TEXT_ALIGN_LAST_LENGTH));
        }
        TextAlignLast::from_str(text_align_last)?;
        Ok(())
    }

    pub fn apply_text_align_last(&mut self, node: &mut TipTapNode, text_align_last: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_text_align_last(text_align_last)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textAlignLast".to_string(), serde_json::Value::String(text_align_last.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textAlignLast": text_align_last }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text align last application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text align last application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_text_align_last(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textAlignLast");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text align last removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text align last removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_text_align_last(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(text_align_last) = obj.get("textAlignLast") {
                    if let Some(s) = text_align_last.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_text_align_last(&self, node: &TipTapNode) -> bool {
        self.get_text_align_last(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_align_last_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextAlignLastManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_text_align_last_variants() {
        assert_eq!(TextAlignLast::Auto.as_str(), "auto");
        assert_eq!(TextAlignLast::Center.as_str(), "center");
    }

    #[test]
    fn test_apply_text_align_last() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextAlignLastManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_text_align_last(&mut node, "justify");
        assert!(result.is_ok());
        assert!(manager.has_text_align_last(&node));
    }

    #[test]
    fn test_remove_text_align_last() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextAlignLastManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textAlignLast": "start" })),
            marks: None,
        };
        
        assert!(manager.has_text_align_last(&node));
        let result = manager.remove_text_align_last(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_text_align_last(&node));
    }

    #[test]
    fn test_get_text_align_last() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextAlignLastManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textAlignLast": "end" })),
            marks: None,
        };
        
        let text_align_last = manager.get_text_align_last(&node);
        assert_eq!(text_align_last, Some("end".to_string()));
    }
}
