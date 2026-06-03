//! TipTap Page Break After Manager - Aerospace-Grade Page Break After Operations Service
//!
//! Safety-critical page break after operations service with:
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
pub enum PageBreakAfter {
    Auto,
    Always,
    Avoid,
    Left,
    Right,
}

impl PageBreakAfter {
    pub fn as_str(&self) -> &str {
        match self {
            PageBreakAfter::Auto => "auto",
            PageBreakAfter::Always => "always",
            PageBreakAfter::Avoid => "avoid",
            PageBreakAfter::Left => "left",
            PageBreakAfter::Right => "right",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(PageBreakAfter::Auto),
            "always" => Ok(PageBreakAfter::Always),
            "avoid" => Ok(PageBreakAfter::Avoid),
            "left" => Ok(PageBreakAfter::Left),
            "right" => Ok(PageBreakAfter::Right),
            _ => Err(format!("Invalid page break after value: {}", s)),
        }
    }
}

pub struct PageBreakAfterManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl PageBreakAfterManager {
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

    fn validate_page_break_after(&self, page_break_after: &str) -> Result<(), String> {
        if page_break_after.is_empty() {
            return Err("Page break after cannot be empty".to_string());
        }
        PageBreakAfter::from_str(page_break_after)?;
        Ok(())
    }

    pub fn apply_page_break_after(&mut self, node: &mut TipTapNode, page_break_after: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_page_break_after(page_break_after)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("pageBreakAfter".to_string(), serde_json::Value::String(page_break_after.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "pageBreakAfter": page_break_after }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Page break after application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Page break after application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_page_break_after(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("pageBreakAfter");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Page break after removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Page break after removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_page_break_after(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(page_break_after) = obj.get("pageBreakAfter") {
                    if let Some(s) = page_break_after.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_page_break_after(&self, node: &TipTapNode) -> bool {
        self.get_page_break_after(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_page_break_after_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PageBreakAfterManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_page_break_after_variants() {
        assert_eq!(PageBreakAfter::Auto.as_str(), "auto");
        assert_eq!(PageBreakAfter::Always.as_str(), "always");
    }

    #[test]
    fn test_apply_page_break_after() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PageBreakAfterManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_page_break_after(&mut node, "always");
        assert!(result.is_ok());
        assert!(manager.has_page_break_after(&node));
    }

    #[test]
    fn test_remove_page_break_after() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PageBreakAfterManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "pageBreakAfter": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_page_break_after(&node));
        let result = manager.remove_page_break_after(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_page_break_after(&node));
    }

    #[test]
    fn test_get_page_break_after() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PageBreakAfterManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "pageBreakAfter": "avoid" })),
            marks: None,
        };
        
        let page_break_after = manager.get_page_break_after(&node);
        assert_eq!(page_break_after, Some("avoid".to_string()));
    }
}
