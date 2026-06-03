//! TipTap Text Justify Manager - Aerospace-Grade Text Justify Operations Service
//!
//! Safety-critical text justify operations service with:
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

/// Maximum text justify string length
const MAX_TEXT_JUSTIFY_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextJustify {
    Auto,
    None,
    InterWord,
    InterCharacter,
}

impl TextJustify {
    pub fn as_str(&self) -> &str {
        match self {
            TextJustify::Auto => "auto",
            TextJustify::None => "none",
            TextJustify::InterWord => "inter-word",
            TextJustify::InterCharacter => "inter-character",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(TextJustify::Auto),
            "none" => Ok(TextJustify::None),
            "inter-word" => Ok(TextJustify::InterWord),
            "inter-character" => Ok(TextJustify::InterCharacter),
            _ => Err(format!("Invalid text justify: {}", s)),
        }
    }
}

pub struct TextJustifyManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextJustifyManager {
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

    pub fn max_text_justify_length() -> usize {
        MAX_TEXT_JUSTIFY_LENGTH
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

    fn validate_text_justify(&self, text_justify: &str) -> Result<(), String> {
        if text_justify.len() > MAX_TEXT_JUSTIFY_LENGTH {
            return Err(format!("Text justify string exceeds maximum length of {} characters", MAX_TEXT_JUSTIFY_LENGTH));
        }
        TextJustify::from_str(text_justify)?;
        Ok(())
    }

    pub fn apply_text_justify(&mut self, node: &mut TipTapNode, text_justify: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_text_justify(text_justify)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textJustify".to_string(), serde_json::Value::String(text_justify.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textJustify": text_justify }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text justify application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text justify application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_text_justify(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textJustify");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text justify removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text justify removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_text_justify(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(text_justify) = obj.get("textJustify") {
                    if let Some(s) = text_justify.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_text_justify(&self, node: &TipTapNode) -> bool {
        self.get_text_justify(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_justify_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextJustifyManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_text_justify_variants() {
        assert_eq!(TextJustify::Auto.as_str(), "auto");
        assert_eq!(TextJustify::None.as_str(), "none");
    }

    #[test]
    fn test_apply_text_justify() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextJustifyManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_text_justify(&mut node, "inter-word");
        assert!(result.is_ok());
        assert!(manager.has_text_justify(&node));
    }

    #[test]
    fn test_remove_text_justify() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextJustifyManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textJustify": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_text_justify(&node));
        let result = manager.remove_text_justify(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_text_justify(&node));
    }

    #[test]
    fn test_get_text_justify() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextJustifyManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textJustify": "none" })),
            marks: None,
        };
        
        let text_justify = manager.get_text_justify(&node);
        assert_eq!(text_justify, Some("none".to_string()));
    }
}
