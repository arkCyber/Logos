//! TipTap Text Emphasis Style Manager - Aerospace-Grade Text Emphasis Style Operations Service
//!
//! Safety-critical text emphasis style operations service with:
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

/// Maximum text emphasis style string length
const MAX_TEXT_EMPHASIS_STYLE_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEmphasisStyle {
    None,
    Filled,
    Open,
}

impl TextEmphasisStyle {
    pub fn as_str(&self) -> &str {
        match self {
            TextEmphasisStyle::None => "none",
            TextEmphasisStyle::Filled => "filled",
            TextEmphasisStyle::Open => "open",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" => Ok(TextEmphasisStyle::None),
            "filled" => Ok(TextEmphasisStyle::Filled),
            "open" => Ok(TextEmphasisStyle::Open),
            _ => Err(format!("Invalid text emphasis style: {}", s)),
        }
    }
}

pub struct TextEmphasisStyleManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextEmphasisStyleManager {
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

    pub fn max_text_emphasis_style_length() -> usize {
        MAX_TEXT_EMPHASIS_STYLE_LENGTH
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

    fn validate_text_emphasis_style(&self, text_emphasis_style: &str) -> Result<(), String> {
        if text_emphasis_style.len() > MAX_TEXT_EMPHASIS_STYLE_LENGTH {
            return Err(format!("Text emphasis style string exceeds maximum length of {} characters", MAX_TEXT_EMPHASIS_STYLE_LENGTH));
        }
        TextEmphasisStyle::from_str(text_emphasis_style)?;
        Ok(())
    }

    pub fn apply_text_emphasis_style(&mut self, node: &mut TipTapNode, text_emphasis_style: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_text_emphasis_style(text_emphasis_style)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textEmphasisStyle".to_string(), serde_json::Value::String(text_emphasis_style.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textEmphasisStyle": text_emphasis_style }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text emphasis style application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text emphasis style application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_text_emphasis_style(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textEmphasisStyle");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text emphasis style removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text emphasis style removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_text_emphasis_style(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(text_emphasis_style) = obj.get("textEmphasisStyle") {
                    if let Some(s) = text_emphasis_style.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_text_emphasis_style(&self, node: &TipTapNode) -> bool {
        self.get_text_emphasis_style(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_emphasis_style_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextEmphasisStyleManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_text_emphasis_style_variants() {
        assert_eq!(TextEmphasisStyle::None.as_str(), "none");
        assert_eq!(TextEmphasisStyle::Filled.as_str(), "filled");
    }

    #[test]
    fn test_apply_text_emphasis_style() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextEmphasisStyleManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_text_emphasis_style(&mut node, "filled");
        assert!(result.is_ok());
        assert!(manager.has_text_emphasis_style(&node));
    }

    #[test]
    fn test_remove_text_emphasis_style() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextEmphasisStyleManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textEmphasisStyle": "open" })),
            marks: None,
        };
        
        assert!(manager.has_text_emphasis_style(&node));
        let result = manager.remove_text_emphasis_style(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_text_emphasis_style(&node));
    }

    #[test]
    fn test_get_text_emphasis_style() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextEmphasisStyleManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textEmphasisStyle": "none" })),
            marks: None,
        };
        
        let text_emphasis_style = manager.get_text_emphasis_style(&node);
        assert_eq!(text_emphasis_style, Some("none".to_string()));
    }
}
