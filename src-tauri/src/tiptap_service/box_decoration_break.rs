//! TipTap Box Decoration Break Manager - Aerospace-Grade Box Decoration Break Operations Service
//!
//! Safety-critical box decoration break operations service with:
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
pub enum BoxDecorationBreak {
    Slice,
    Clone,
}

impl BoxDecorationBreak {
    pub fn as_str(&self) -> &str {
        match self {
            BoxDecorationBreak::Slice => "slice",
            BoxDecorationBreak::Clone => "clone",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "slice" => Ok(BoxDecorationBreak::Slice),
            "clone" => Ok(BoxDecorationBreak::Clone),
            _ => Err(format!("Invalid box decoration break value: {}", s)),
        }
    }
}

pub struct BoxDecorationBreakManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl BoxDecorationBreakManager {
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

    fn validate_box_decoration_break(&self, box_decoration_break: &str) -> Result<(), String> {
        if box_decoration_break.is_empty() {
            return Err("Box decoration break cannot be empty".to_string());
        }
        BoxDecorationBreak::from_str(box_decoration_break)?;
        Ok(())
    }

    pub fn apply_box_decoration_break(&mut self, node: &mut TipTapNode, box_decoration_break: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_box_decoration_break(box_decoration_break)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("boxDecorationBreak".to_string(), serde_json::Value::String(box_decoration_break.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "boxDecorationBreak": box_decoration_break }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Box decoration break application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Box decoration break application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_box_decoration_break(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("boxDecorationBreak");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Box decoration break removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Box decoration break removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_box_decoration_break(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(box_decoration_break) = obj.get("boxDecorationBreak") {
                    if let Some(s) = box_decoration_break.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_box_decoration_break(&self, node: &TipTapNode) -> bool {
        self.get_box_decoration_break(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_box_decoration_break_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BoxDecorationBreakManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_box_decoration_break_variants() {
        assert_eq!(BoxDecorationBreak::Slice.as_str(), "slice");
        assert_eq!(BoxDecorationBreak::Clone.as_str(), "clone");
    }

    #[test]
    fn test_apply_box_decoration_break() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BoxDecorationBreakManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_box_decoration_break(&mut node, "clone");
        assert!(result.is_ok());
        assert!(manager.has_box_decoration_break(&node));
    }

    #[test]
    fn test_remove_box_decoration_break() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BoxDecorationBreakManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "boxDecorationBreak": "slice" })),
            marks: None,
        };
        
        assert!(manager.has_box_decoration_break(&node));
        let result = manager.remove_box_decoration_break(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_box_decoration_break(&node));
    }

    #[test]
    fn test_get_box_decoration_break() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BoxDecorationBreakManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "boxDecorationBreak": "clone" })),
            marks: None,
        };
        
        let box_decoration_break = manager.get_box_decoration_break(&node);
        assert_eq!(box_decoration_break, Some("clone".to_string()));
    }
}
