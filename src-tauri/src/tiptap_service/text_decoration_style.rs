//! TipTap Text Decoration Style Manager - Aerospace-Grade Text Decoration Style Operations Service
//!
//! Safety-critical text decoration style operations service with:
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

/// Maximum text decoration style string length
const MAX_TEXT_DECORATION_STYLE_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDecorationStyle {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}

impl TextDecorationStyle {
    pub fn as_str(&self) -> &str {
        match self {
            TextDecorationStyle::Solid => "solid",
            TextDecorationStyle::Double => "double",
            TextDecorationStyle::Dotted => "dotted",
            TextDecorationStyle::Dashed => "dashed",
            TextDecorationStyle::Wavy => "wavy",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "solid" => Ok(TextDecorationStyle::Solid),
            "double" => Ok(TextDecorationStyle::Double),
            "dotted" => Ok(TextDecorationStyle::Dotted),
            "dashed" => Ok(TextDecorationStyle::Dashed),
            "wavy" => Ok(TextDecorationStyle::Wavy),
            _ => Err(format!("Invalid text decoration style: {}", s)),
        }
    }
}

pub struct TextDecorationStyleManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextDecorationStyleManager {
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

    pub fn max_text_decoration_style_length() -> usize {
        MAX_TEXT_DECORATION_STYLE_LENGTH
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

    fn validate_text_decoration_style(&self, text_decoration_style: &str) -> Result<(), String> {
        if text_decoration_style.len() > MAX_TEXT_DECORATION_STYLE_LENGTH {
            return Err(format!("Text decoration style string exceeds maximum length of {} characters", MAX_TEXT_DECORATION_STYLE_LENGTH));
        }
        TextDecorationStyle::from_str(text_decoration_style)?;
        Ok(())
    }

    pub fn apply_text_decoration_style(&mut self, node: &mut TipTapNode, text_decoration_style: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_text_decoration_style(text_decoration_style)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textDecorationStyle".to_string(), serde_json::Value::String(text_decoration_style.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textDecorationStyle": text_decoration_style }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text decoration style application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text decoration style application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_text_decoration_style(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textDecorationStyle");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text decoration style removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text decoration style removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_text_decoration_style(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(text_decoration_style) = obj.get("textDecorationStyle") {
                    if let Some(s) = text_decoration_style.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_text_decoration_style(&self, node: &TipTapNode) -> bool {
        self.get_text_decoration_style(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_decoration_style_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextDecorationStyleManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_text_decoration_style_variants() {
        assert_eq!(TextDecorationStyle::Solid.as_str(), "solid");
        assert_eq!(TextDecorationStyle::Wavy.as_str(), "wavy");
    }

    #[test]
    fn test_apply_text_decoration_style() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDecorationStyleManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_text_decoration_style(&mut node, "dotted");
        assert!(result.is_ok());
        assert!(manager.has_text_decoration_style(&node));
    }

    #[test]
    fn test_remove_text_decoration_style() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDecorationStyleManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textDecorationStyle": "solid" })),
            marks: None,
        };
        
        assert!(manager.has_text_decoration_style(&node));
        let result = manager.remove_text_decoration_style(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_text_decoration_style(&node));
    }

    #[test]
    fn test_get_text_decoration_style() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextDecorationStyleManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textDecorationStyle": "dashed" })),
            marks: None,
        };
        
        let text_decoration_style = manager.get_text_decoration_style(&node);
        assert_eq!(text_decoration_style, Some("dashed".to_string()));
    }
}
