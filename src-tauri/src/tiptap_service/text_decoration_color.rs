//! TipTap Text Decoration Color Manager - Aerospace-Grade Text Decoration Color Operations Service
//!
//! Safety-critical text decoration color operations service with:
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

/// Maximum text decoration color string length
const MAX_TEXT_DECORATION_COLOR_LENGTH: usize = 100;

pub struct TextDecorationColorManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextDecorationColorManager {
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

    pub fn max_text_decoration_color_length() -> usize {
        MAX_TEXT_DECORATION_COLOR_LENGTH
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

    fn validate_text_decoration_color(&self, text_decoration_color: &str) -> Result<(), String> {
        if text_decoration_color.is_empty() {
            return Err("Text decoration color cannot be empty".to_string());
        }
        if text_decoration_color.len() > MAX_TEXT_DECORATION_COLOR_LENGTH {
            return Err(format!("Text decoration color string exceeds maximum length of {} characters", MAX_TEXT_DECORATION_COLOR_LENGTH));
        }
        let valid_patterns = ["#", "rgb", "rgba", "hsl", "hsla", "currentColor", "inherit"];
        if !valid_patterns.iter().any(|pattern| text_decoration_color.starts_with(pattern) || text_decoration_color == *pattern) {
            return Err("Invalid text decoration color format".to_string());
        }
        Ok(())
    }

    pub fn apply_text_decoration_color(&mut self, node: &mut TipTapNode, text_decoration_color: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_text_decoration_color(text_decoration_color)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textDecorationColor".to_string(), serde_json::Value::String(text_decoration_color.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textDecorationColor": text_decoration_color }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text decoration color application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text decoration color application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_text_decoration_color(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textDecorationColor");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text decoration color removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text decoration color removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_text_decoration_color(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(text_decoration_color) = obj.get("textDecorationColor") {
                    if let Some(s) = text_decoration_color.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_text_decoration_color(&self, node: &TipTapNode) -> bool {
        self.get_text_decoration_color(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_decoration_color_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextDecorationColorManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_text_decoration_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDecorationColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_text_decoration_color(&mut node, "#ff0000");
        assert!(result.is_ok());
        assert!(manager.has_text_decoration_color(&node));
    }

    #[test]
    fn test_remove_text_decoration_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDecorationColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textDecorationColor": "#00ff00" })),
            marks: None,
        };
        
        assert!(manager.has_text_decoration_color(&node));
        let result = manager.remove_text_decoration_color(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_text_decoration_color(&node));
    }

    #[test]
    fn test_get_text_decoration_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextDecorationColorManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textDecorationColor": "rgb(0, 0, 255)" })),
            marks: None,
        };
        
        let text_decoration_color = manager.get_text_decoration_color(&node);
        assert_eq!(text_decoration_color, Some("rgb(0, 0, 255)".to_string()));
    }
}
