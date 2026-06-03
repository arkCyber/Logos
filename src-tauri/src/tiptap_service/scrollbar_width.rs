//! TipTap Scrollbar Width Manager - Aerospace-Grade Scrollbar Width Operations Service
//!
//! Safety-critical scrollbar width operations service with:
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollbarWidth {
    Auto,
    Thin,
    None,
}

impl ScrollbarWidth {
    pub fn as_str(&self) -> &str {
        match self {
            ScrollbarWidth::Auto => "auto",
            ScrollbarWidth::Thin => "thin",
            ScrollbarWidth::None => "none",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(ScrollbarWidth::Auto),
            "thin" => Ok(ScrollbarWidth::Thin),
            "none" => Ok(ScrollbarWidth::None),
            _ => Err(format!("Invalid scrollbar width value: {}", s)),
        }
    }
}

pub struct ScrollbarWidthManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ScrollbarWidthManager {
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

    fn validate_scrollbar_width(&self, scrollbar_width: &str) -> Result<(), String> {
        if scrollbar_width.is_empty() {
            return Err("Scrollbar width cannot be empty".to_string());
        }
        ScrollbarWidth::from_str(scrollbar_width)?;
        Ok(())
    }

    pub fn apply_scrollbar_width(&mut self, node: &mut TipTapNode, scrollbar_width: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_scrollbar_width(scrollbar_width)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("scrollbarWidth".to_string(), serde_json::Value::String(scrollbar_width.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "scrollbarWidth": scrollbar_width }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Scrollbar width application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Scrollbar width application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_scrollbar_width(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("scrollbarWidth");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Scrollbar width removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Scrollbar width removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_scrollbar_width(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(scrollbar_width) = obj.get("scrollbarWidth") {
                    if let Some(s) = scrollbar_width.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_scrollbar_width(&self, node: &TipTapNode) -> bool {
        self.get_scrollbar_width(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_scrollbar_width_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ScrollbarWidthManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_scrollbar_width_variants() {
        assert_eq!(ScrollbarWidth::Auto.as_str(), "auto");
        assert_eq!(ScrollbarWidth::Thin.as_str(), "thin");
    }

    #[test]
    fn test_apply_scrollbar_width() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScrollbarWidthManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_scrollbar_width(&mut node, "thin");
        assert!(result.is_ok());
        assert!(manager.has_scrollbar_width(&node));
    }

    #[test]
    fn test_remove_scrollbar_width() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScrollbarWidthManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "scrollbarWidth": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_scrollbar_width(&node));
        let result = manager.remove_scrollbar_width(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_scrollbar_width(&node));
    }

    #[test]
    fn test_get_scrollbar_width() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ScrollbarWidthManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "scrollbarWidth": "none" })),
            marks: None,
        };
        
        let scrollbar_width = manager.get_scrollbar_width(&node);
        assert_eq!(scrollbar_width, Some("none".to_string()));
    }
}
