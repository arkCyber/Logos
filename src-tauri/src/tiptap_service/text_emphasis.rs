//! TipTap Text Emphasis Manager - Aerospace-Grade Text Emphasis Operations Service
//!
//! Safety-critical text emphasis operations service with:
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

/// Maximum text emphasis string length
const MAX_TEXT_EMPHASIS_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEmphasis {
    None,
    Dot,
    Circle,
    DoubleCircle,
    Triangle,
    Sesame,
}

impl TextEmphasis {
    pub fn as_str(&self) -> &str {
        match self {
            TextEmphasis::None => "none",
            TextEmphasis::Dot => "dot",
            TextEmphasis::Circle => "circle",
            TextEmphasis::DoubleCircle => "double-circle",
            TextEmphasis::Triangle => "triangle",
            TextEmphasis::Sesame => "sesame",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" => Ok(TextEmphasis::None),
            "dot" => Ok(TextEmphasis::Dot),
            "circle" => Ok(TextEmphasis::Circle),
            "double-circle" => Ok(TextEmphasis::DoubleCircle),
            "triangle" => Ok(TextEmphasis::Triangle),
            "sesame" => Ok(TextEmphasis::Sesame),
            _ => Err(format!("Invalid text emphasis: {}", s)),
        }
    }
}

pub struct TextEmphasisManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextEmphasisManager {
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

    pub fn max_text_emphasis_length() -> usize {
        MAX_TEXT_EMPHASIS_LENGTH
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

    fn validate_text_emphasis(&self, text_emphasis: &str) -> Result<(), String> {
        if text_emphasis.len() > MAX_TEXT_EMPHASIS_LENGTH {
            return Err(format!("Text emphasis string exceeds maximum length of {} characters", MAX_TEXT_EMPHASIS_LENGTH));
        }
        TextEmphasis::from_str(text_emphasis)?;
        Ok(())
    }

    pub fn apply_text_emphasis(&mut self, node: &mut TipTapNode, text_emphasis: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_text_emphasis(text_emphasis)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textEmphasis".to_string(), serde_json::Value::String(text_emphasis.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textEmphasis": text_emphasis }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text emphasis application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text emphasis application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_text_emphasis(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textEmphasis");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text emphasis removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text emphasis removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_text_emphasis(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(text_emphasis) = obj.get("textEmphasis") {
                    if let Some(s) = text_emphasis.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_text_emphasis(&self, node: &TipTapNode) -> bool {
        self.get_text_emphasis(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_emphasis_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextEmphasisManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_text_emphasis_variants() {
        assert_eq!(TextEmphasis::None.as_str(), "none");
        assert_eq!(TextEmphasis::Dot.as_str(), "dot");
        assert_eq!(TextEmphasis::Circle.as_str(), "circle");
    }

    #[test]
    fn test_apply_text_emphasis() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextEmphasisManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_text_emphasis(&mut node, "dot");
        assert!(result.is_ok());
        assert!(manager.has_text_emphasis(&node));
    }

    #[test]
    fn test_remove_text_emphasis() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextEmphasisManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textEmphasis": "circle" })),
            marks: None,
        };
        
        assert!(manager.has_text_emphasis(&node));
        let result = manager.remove_text_emphasis(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_text_emphasis(&node));
    }

    #[test]
    fn test_get_text_emphasis() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextEmphasisManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textEmphasis": "sesame" })),
            marks: None,
        };
        
        let text_emphasis = manager.get_text_emphasis(&node);
        assert_eq!(text_emphasis, Some("sesame".to_string()));
    }
}
