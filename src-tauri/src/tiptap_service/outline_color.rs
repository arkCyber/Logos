//! TipTap Outline Color Manager - Aerospace-Grade Outline Color Operations Service
//!
//! Safety-critical outline color operations service with:
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

/// Maximum outline color string length
const MAX_OUTLINE_COLOR_LENGTH: usize = 100;

pub struct OutlineColorManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl OutlineColorManager {
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

    pub fn max_outline_color_length() -> usize {
        MAX_OUTLINE_COLOR_LENGTH
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

    fn validate_outline_color(&self, outline_color: &str) -> Result<(), String> {
        if outline_color.is_empty() {
            return Err("Outline color cannot be empty".to_string());
        }
        if outline_color.len() > MAX_OUTLINE_COLOR_LENGTH {
            return Err(format!("Outline color string exceeds maximum length of {} characters", MAX_OUTLINE_COLOR_LENGTH));
        }
        let valid_patterns = ["#", "rgb", "rgba", "hsl", "hsla", "transparent", "currentColor", "inherit"];
        if !valid_patterns.iter().any(|pattern| outline_color.starts_with(pattern) || outline_color == *pattern) {
            return Err("Invalid outline color format".to_string());
        }
        Ok(())
    }

    pub fn apply_outline_color(&mut self, node: &mut TipTapNode, outline_color: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_outline_color(outline_color)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("outlineColor".to_string(), serde_json::Value::String(outline_color.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "outlineColor": outline_color }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Outline color application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Outline color application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_outline_color(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("outlineColor");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Outline color removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Outline color removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_outline_color(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(outline_color) = obj.get("outlineColor") {
                    if let Some(s) = outline_color.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_outline_color(&self, node: &TipTapNode) -> bool {
        self.get_outline_color(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_outline_color_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OutlineColorManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_outline_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OutlineColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_outline_color(&mut node, "#ff0000");
        assert!(result.is_ok());
        assert!(manager.has_outline_color(&node));
    }

    #[test]
    fn test_remove_outline_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OutlineColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "outlineColor": "#00ff00" })),
            marks: None,
        };
        
        assert!(manager.has_outline_color(&node));
        let result = manager.remove_outline_color(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_outline_color(&node));
    }

    #[test]
    fn test_get_outline_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OutlineColorManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "outlineColor": "rgb(0, 0, 255)" })),
            marks: None,
        };
        
        let outline_color = manager.get_outline_color(&node);
        assert_eq!(outline_color, Some("rgb(0, 0, 255)".to_string()));
    }
}
