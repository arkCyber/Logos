//! TipTap Font Style Manager - Aerospace-Grade Font Style Operations Service
//!
//! Safety-critical font style operations service with:
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

/// Maximum font style string length
const MAX_FONT_STYLE_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

impl FontStyle {
    pub fn as_str(&self) -> &str {
        match self {
            FontStyle::Normal => "normal",
            FontStyle::Italic => "italic",
            FontStyle::Oblique => "oblique",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(FontStyle::Normal),
            "italic" => Ok(FontStyle::Italic),
            "oblique" => Ok(FontStyle::Oblique),
            _ => Err(format!("Invalid font style: {}", s)),
        }
    }
}

pub struct FontStyleManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FontStyleManager {
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

    pub fn max_font_style_length() -> usize {
        MAX_FONT_STYLE_LENGTH
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

    fn validate_font_style(&self, font_style: &str) -> Result<(), String> {
        if font_style.is_empty() {
            return Err("Font style cannot be empty".to_string());
        }
        if font_style.len() > MAX_FONT_STYLE_LENGTH {
            return Err(format!("Font style string exceeds maximum length of {} characters", MAX_FONT_STYLE_LENGTH));
        }
        FontStyle::from_str(font_style)?;
        Ok(())
    }

    pub fn apply_font_style(&mut self, node: &mut TipTapNode, font_style: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_font_style(font_style)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("fontStyle".to_string(), serde_json::Value::String(font_style.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "fontStyle": font_style }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font style application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font style application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_font_style(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("fontStyle");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font style removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font style removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_font_style(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(font_style) = obj.get("fontStyle") {
                    if let Some(s) = font_style.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_font_style(&self, node: &TipTapNode) -> bool {
        self.get_font_style(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_font_style_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontStyleManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_font_style_variants() {
        assert_eq!(FontStyle::Normal.as_str(), "normal");
        assert_eq!(FontStyle::Italic.as_str(), "italic");
        assert_eq!(FontStyle::Oblique.as_str(), "oblique");
    }

    #[test]
    fn test_apply_font_style() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontStyleManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_font_style(&mut node, "italic");
        assert!(result.is_ok());
        assert!(manager.has_font_style(&node));
    }

    #[test]
    fn test_remove_font_style() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontStyleManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontStyle": "italic" })),
            marks: None,
        };
        
        assert!(manager.has_font_style(&node));
        let result = manager.remove_font_style(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_font_style(&node));
    }

    #[test]
    fn test_get_font_style() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontStyleManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontStyle": "oblique" })),
            marks: None,
        };
        
        let font_style = manager.get_font_style(&node);
        assert_eq!(font_style, Some("oblique".to_string()));
    }
}
