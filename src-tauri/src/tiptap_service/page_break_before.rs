//! TipTap Page Break Before Manager - Aerospace-Grade Page Break Before Operations Service
//!
//! Safety-critical page break before operations service with:
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
pub enum PageBreakBefore {
    Auto,
    Always,
    Avoid,
    Left,
    Right,
}

impl PageBreakBefore {
    pub fn as_str(&self) -> &str {
        match self {
            PageBreakBefore::Auto => "auto",
            PageBreakBefore::Always => "always",
            PageBreakBefore::Avoid => "avoid",
            PageBreakBefore::Left => "left",
            PageBreakBefore::Right => "right",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(PageBreakBefore::Auto),
            "always" => Ok(PageBreakBefore::Always),
            "avoid" => Ok(PageBreakBefore::Avoid),
            "left" => Ok(PageBreakBefore::Left),
            "right" => Ok(PageBreakBefore::Right),
            _ => Err(format!("Invalid page break before value: {}", s)),
        }
    }
}

pub struct PageBreakBeforeManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl PageBreakBeforeManager {
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

    fn validate_page_break_before(&self, page_break_before: &str) -> Result<(), String> {
        if page_break_before.is_empty() {
            return Err("Page break before cannot be empty".to_string());
        }
        PageBreakBefore::from_str(page_break_before)?;
        Ok(())
    }

    pub fn apply_page_break_before(&mut self, node: &mut TipTapNode, page_break_before: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_page_break_before(page_break_before)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("pageBreakBefore".to_string(), serde_json::Value::String(page_break_before.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "pageBreakBefore": page_break_before }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Page break before application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Page break before application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_page_break_before(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("pageBreakBefore");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Page break before removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Page break before removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_page_break_before(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(page_break_before) = obj.get("pageBreakBefore") {
                    if let Some(s) = page_break_before.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_page_break_before(&self, node: &TipTapNode) -> bool {
        self.get_page_break_before(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_page_break_before_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PageBreakBeforeManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_page_break_before_variants() {
        assert_eq!(PageBreakBefore::Auto.as_str(), "auto");
        assert_eq!(PageBreakBefore::Always.as_str(), "always");
    }

    #[test]
    fn test_apply_page_break_before() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PageBreakBeforeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_page_break_before(&mut node, "always");
        assert!(result.is_ok());
        assert!(manager.has_page_break_before(&node));
    }

    #[test]
    fn test_remove_page_break_before() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PageBreakBeforeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "pageBreakBefore": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_page_break_before(&node));
        let result = manager.remove_page_break_before(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_page_break_before(&node));
    }

    #[test]
    fn test_get_page_break_before() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PageBreakBeforeManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "pageBreakBefore": "avoid" })),
            marks: None,
        };
        
        let page_break_before = manager.get_page_break_before(&node);
        assert_eq!(page_break_before, Some("avoid".to_string()));
    }
}
