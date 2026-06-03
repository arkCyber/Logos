//! TipTap Scrollbar Color Manager - Aerospace-Grade Scrollbar Color Operations Service
//!
//! Safety-critical scrollbar color operations service with:
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

/// Maximum scrollbar color string length
const MAX_SCROLLBAR_COLOR_LENGTH: usize = 100;

pub struct ScrollbarColorManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ScrollbarColorManager {
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

    pub fn max_scrollbar_color_length() -> usize {
        MAX_SCROLLBAR_COLOR_LENGTH
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

    fn validate_scrollbar_color(&self, scrollbar_color: &str) -> Result<(), String> {
        if scrollbar_color.is_empty() {
            return Err("Scrollbar color cannot be empty".to_string());
        }
        if scrollbar_color.len() > MAX_SCROLLBAR_COLOR_LENGTH {
            return Err(format!("Scrollbar color string exceeds maximum length of {} characters", MAX_SCROLLBAR_COLOR_LENGTH));
        }
        if scrollbar_color == "auto" {
            return Ok(());
        }
        if scrollbar_color.contains('<') || scrollbar_color.contains('>') || scrollbar_color.contains('"') {
            return Err("Scrollbar color contains invalid characters".to_string());
        }
        Ok(())
    }

    pub fn apply_scrollbar_color(&mut self, node: &mut TipTapNode, scrollbar_color: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_scrollbar_color(scrollbar_color)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("scrollbarColor".to_string(), serde_json::Value::String(scrollbar_color.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "scrollbarColor": scrollbar_color }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Scrollbar color application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Scrollbar color application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_scrollbar_color(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("scrollbarColor");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Scrollbar color removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Scrollbar color removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_scrollbar_color(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(scrollbar_color) = obj.get("scrollbarColor") {
                    if let Some(s) = scrollbar_color.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_scrollbar_color(&self, node: &TipTapNode) -> bool {
        self.get_scrollbar_color(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_scrollbar_color_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ScrollbarColorManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_scrollbar_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScrollbarColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_scrollbar_color(&mut node, "auto");
        assert!(result.is_ok());
        assert!(manager.has_scrollbar_color(&node));
    }

    #[test]
    fn test_remove_scrollbar_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScrollbarColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "scrollbarColor": "#ff0000 #0000ff" })),
            marks: None,
        };
        
        assert!(manager.has_scrollbar_color(&node));
        let result = manager.remove_scrollbar_color(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_scrollbar_color(&node));
    }

    #[test]
    fn test_get_scrollbar_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ScrollbarColorManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "scrollbarColor": "dark dark" })),
            marks: None,
        };
        
        let scrollbar_color = manager.get_scrollbar_color(&node);
        assert_eq!(scrollbar_color, Some("dark dark".to_string()));
    }
}
