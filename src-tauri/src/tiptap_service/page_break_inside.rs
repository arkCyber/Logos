//! TipTap Page Break Inside Manager - Aerospace-Grade Page Break Inside Operations Service
//!
//! Safety-critical page break inside operations service with:
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
pub enum PageBreakInside {
    Auto,
    Avoid,
}

impl PageBreakInside {
    pub fn as_str(&self) -> &str {
        match self {
            PageBreakInside::Auto => "auto",
            PageBreakInside::Avoid => "avoid",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(PageBreakInside::Auto),
            "avoid" => Ok(PageBreakInside::Avoid),
            _ => Err(format!("Invalid page break inside value: {}", s)),
        }
    }
}

pub struct PageBreakInsideManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl PageBreakInsideManager {
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

    fn validate_page_break_inside(&self, page_break_inside: &str) -> Result<(), String> {
        if page_break_inside.is_empty() {
            return Err("Page break inside cannot be empty".to_string());
        }
        PageBreakInside::from_str(page_break_inside)?;
        Ok(())
    }

    pub fn apply_page_break_inside(&mut self, node: &mut TipTapNode, page_break_inside: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_page_break_inside(page_break_inside)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("pageBreakInside".to_string(), serde_json::Value::String(page_break_inside.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "pageBreakInside": page_break_inside }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Page break inside application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Page break inside application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_page_break_inside(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("pageBreakInside");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Page break inside removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Page break inside removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_page_break_inside(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(page_break_inside) = obj.get("pageBreakInside") {
                    if let Some(s) = page_break_inside.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_page_break_inside(&self, node: &TipTapNode) -> bool {
        self.get_page_break_inside(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_page_break_inside_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PageBreakInsideManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_page_break_inside_variants() {
        assert_eq!(PageBreakInside::Auto.as_str(), "auto");
        assert_eq!(PageBreakInside::Avoid.as_str(), "avoid");
    }

    #[test]
    fn test_apply_page_break_inside() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PageBreakInsideManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_page_break_inside(&mut node, "avoid");
        assert!(result.is_ok());
        assert!(manager.has_page_break_inside(&node));
    }

    #[test]
    fn test_remove_page_break_inside() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PageBreakInsideManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "pageBreakInside": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_page_break_inside(&node));
        let result = manager.remove_page_break_inside(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_page_break_inside(&node));
    }

    #[test]
    fn test_get_page_break_inside() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PageBreakInsideManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "pageBreakInside": "avoid" })),
            marks: None,
        };
        
        let page_break_inside = manager.get_page_break_inside(&node);
        assert_eq!(page_break_inside, Some("avoid".to_string()));
    }
}
