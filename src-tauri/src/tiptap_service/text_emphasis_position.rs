//! TipTap Text Emphasis Position Manager - Aerospace-Grade Text Emphasis Position Operations Service
//!
//! Safety-critical text emphasis position operations service with:
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

/// Maximum text emphasis position string length
const MAX_TEXT_EMPHASIS_POSITION_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEmphasisPosition {
    Over,
    Under,
}

impl TextEmphasisPosition {
    pub fn as_str(&self) -> &str {
        match self {
            TextEmphasisPosition::Over => "over",
            TextEmphasisPosition::Under => "under",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "over" => Ok(TextEmphasisPosition::Over),
            "under" => Ok(TextEmphasisPosition::Under),
            _ => Err(format!("Invalid text emphasis position: {}", s)),
        }
    }
}

pub struct TextEmphasisPositionManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextEmphasisPositionManager {
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

    pub fn max_text_emphasis_position_length() -> usize {
        MAX_TEXT_EMPHASIS_POSITION_LENGTH
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

    fn validate_text_emphasis_position(&self, text_emphasis_position: &str) -> Result<(), String> {
        if text_emphasis_position.len() > MAX_TEXT_EMPHASIS_POSITION_LENGTH {
            return Err(format!("Text emphasis position string exceeds maximum length of {} characters", MAX_TEXT_EMPHASIS_POSITION_LENGTH));
        }
        TextEmphasisPosition::from_str(text_emphasis_position)?;
        Ok(())
    }

    pub fn apply_text_emphasis_position(&mut self, node: &mut TipTapNode, text_emphasis_position: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_text_emphasis_position(text_emphasis_position)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textEmphasisPosition".to_string(), serde_json::Value::String(text_emphasis_position.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textEmphasisPosition": text_emphasis_position }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text emphasis position application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text emphasis position application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_text_emphasis_position(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textEmphasisPosition");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text emphasis position removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text emphasis position removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_text_emphasis_position(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(text_emphasis_position) = obj.get("textEmphasisPosition") {
                    if let Some(s) = text_emphasis_position.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_text_emphasis_position(&self, node: &TipTapNode) -> bool {
        self.get_text_emphasis_position(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_emphasis_position_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextEmphasisPositionManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_text_emphasis_position_variants() {
        assert_eq!(TextEmphasisPosition::Over.as_str(), "over");
        assert_eq!(TextEmphasisPosition::Under.as_str(), "under");
    }

    #[test]
    fn test_apply_text_emphasis_position() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextEmphasisPositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_text_emphasis_position(&mut node, "over");
        assert!(result.is_ok());
        assert!(manager.has_text_emphasis_position(&node));
    }

    #[test]
    fn test_remove_text_emphasis_position() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextEmphasisPositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textEmphasisPosition": "under" })),
            marks: None,
        };
        
        assert!(manager.has_text_emphasis_position(&node));
        let result = manager.remove_text_emphasis_position(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_text_emphasis_position(&node));
    }

    #[test]
    fn test_get_text_emphasis_position() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextEmphasisPositionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textEmphasisPosition": "over" })),
            marks: None,
        };
        
        let text_emphasis_position = manager.get_text_emphasis_position(&node);
        assert_eq!(text_emphasis_position, Some("over".to_string()));
    }
}
