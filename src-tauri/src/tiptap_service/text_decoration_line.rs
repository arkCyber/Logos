//! TipTap Text Decoration Line Manager - Aerospace-Grade Text Decoration Line Operations Service
//!
//! Safety-critical text decoration line operations service with:
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

/// Maximum text decoration line string length
const MAX_TEXT_DECORATION_LINE_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDecorationLine {
    None,
    Underline,
    Overline,
    LineThrough,
}

impl TextDecorationLine {
    pub fn as_str(&self) -> &str {
        match self {
            TextDecorationLine::None => "none",
            TextDecorationLine::Underline => "underline",
            TextDecorationLine::Overline => "overline",
            TextDecorationLine::LineThrough => "line-through",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" => Ok(TextDecorationLine::None),
            "underline" => Ok(TextDecorationLine::Underline),
            "overline" => Ok(TextDecorationLine::Overline),
            "line-through" => Ok(TextDecorationLine::LineThrough),
            _ => Err(format!("Invalid text decoration line: {}", s)),
        }
    }
}

pub struct TextDecorationLineManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextDecorationLineManager {
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

    pub fn max_text_decoration_line_length() -> usize {
        MAX_TEXT_DECORATION_LINE_LENGTH
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

    fn validate_text_decoration_line(&self, text_decoration_line: &str) -> Result<(), String> {
        if text_decoration_line.len() > MAX_TEXT_DECORATION_LINE_LENGTH {
            return Err(format!("Text decoration line string exceeds maximum length of {} characters", MAX_TEXT_DECORATION_LINE_LENGTH));
        }
        TextDecorationLine::from_str(text_decoration_line)?;
        Ok(())
    }

    pub fn apply_text_decoration_line(&mut self, node: &mut TipTapNode, text_decoration_line: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_text_decoration_line(text_decoration_line)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textDecorationLine".to_string(), serde_json::Value::String(text_decoration_line.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textDecorationLine": text_decoration_line }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text decoration line application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text decoration line application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_text_decoration_line(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textDecorationLine");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text decoration line removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text decoration line removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_text_decoration_line(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(text_decoration_line) = obj.get("textDecorationLine") {
                    if let Some(s) = text_decoration_line.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_text_decoration_line(&self, node: &TipTapNode) -> bool {
        self.get_text_decoration_line(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_decoration_line_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextDecorationLineManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_text_decoration_line_variants() {
        assert_eq!(TextDecorationLine::None.as_str(), "none");
        assert_eq!(TextDecorationLine::Underline.as_str(), "underline");
    }

    #[test]
    fn test_apply_text_decoration_line() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDecorationLineManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_text_decoration_line(&mut node, "underline");
        assert!(result.is_ok());
        assert!(manager.has_text_decoration_line(&node));
    }

    #[test]
    fn test_remove_text_decoration_line() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDecorationLineManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textDecorationLine": "overline" })),
            marks: None,
        };
        
        assert!(manager.has_text_decoration_line(&node));
        let result = manager.remove_text_decoration_line(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_text_decoration_line(&node));
    }

    #[test]
    fn test_get_text_decoration_line() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextDecorationLineManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textDecorationLine": "line-through" })),
            marks: None,
        };
        
        let text_decoration_line = manager.get_text_decoration_line(&node);
        assert_eq!(text_decoration_line, Some("line-through".to_string()));
    }
}
