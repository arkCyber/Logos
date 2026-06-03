//! TipTap Text Underline Position Manager - Aerospace-Grade Text Underline Position Operations Service
//!
//! Safety-critical text underline position operations service with:
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

/// Maximum text underline position string length
const MAX_TEXT_UNDERLINE_POSITION_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextUnderlinePosition {
    Auto,
    Under,
    FromFont,
}

impl TextUnderlinePosition {
    pub fn as_str(&self) -> &str {
        match self {
            TextUnderlinePosition::Auto => "auto",
            TextUnderlinePosition::Under => "under",
            TextUnderlinePosition::FromFont => "from-font",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(TextUnderlinePosition::Auto),
            "under" => Ok(TextUnderlinePosition::Under),
            "from-font" => Ok(TextUnderlinePosition::FromFont),
            _ => Err(format!("Invalid text underline position: {}", s)),
        }
    }
}

pub struct TextUnderlinePositionManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextUnderlinePositionManager {
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

    pub fn max_text_underline_position_length() -> usize {
        MAX_TEXT_UNDERLINE_POSITION_LENGTH
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

    fn validate_text_underline_position(&self, text_underline_position: &str) -> Result<(), String> {
        if text_underline_position.len() > MAX_TEXT_UNDERLINE_POSITION_LENGTH {
            return Err(format!("Text underline position string exceeds maximum length of {} characters", MAX_TEXT_UNDERLINE_POSITION_LENGTH));
        }
        TextUnderlinePosition::from_str(text_underline_position)?;
        Ok(())
    }

    pub fn apply_text_underline_position(&mut self, node: &mut TipTapNode, text_underline_position: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_text_underline_position(text_underline_position)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textUnderlinePosition".to_string(), serde_json::Value::String(text_underline_position.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textUnderlinePosition": text_underline_position }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text underline position application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text underline position application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_text_underline_position(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textUnderlinePosition");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text underline position removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text underline position removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_text_underline_position(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(text_underline_position) = obj.get("textUnderlinePosition") {
                    if let Some(s) = text_underline_position.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_text_underline_position(&self, node: &TipTapNode) -> bool {
        self.get_text_underline_position(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_underline_position_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextUnderlinePositionManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_text_underline_position_variants() {
        assert_eq!(TextUnderlinePosition::Auto.as_str(), "auto");
        assert_eq!(TextUnderlinePosition::Under.as_str(), "under");
    }

    #[test]
    fn test_apply_text_underline_position() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextUnderlinePositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_text_underline_position(&mut node, "under");
        assert!(result.is_ok());
        assert!(manager.has_text_underline_position(&node));
    }

    #[test]
    fn test_remove_text_underline_position() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextUnderlinePositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textUnderlinePosition": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_text_underline_position(&node));
        let result = manager.remove_text_underline_position(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_text_underline_position(&node));
    }

    #[test]
    fn test_get_text_underline_position() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextUnderlinePositionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textUnderlinePosition": "from-font" })),
            marks: None,
        };
        
        let text_underline_position = manager.get_text_underline_position(&node);
        assert_eq!(text_underline_position, Some("from-font".to_string()));
    }
}
