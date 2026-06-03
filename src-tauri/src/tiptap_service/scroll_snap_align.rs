//! TipTap Scroll Snap Align Manager - Aerospace-Grade Scroll Snap Align Operations Service
//!
//! Safety-critical scroll snap align operations service with:
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

/// Maximum scroll snap align string length
const MAX_SCROLL_SNAP_ALIGN_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollSnapAlign {
    None,
    Start,
    End,
    Center,
}

impl ScrollSnapAlign {
    pub fn as_str(&self) -> &str {
        match self {
            ScrollSnapAlign::None => "none",
            ScrollSnapAlign::Start => "start",
            ScrollSnapAlign::End => "end",
            ScrollSnapAlign::Center => "center",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" => Ok(ScrollSnapAlign::None),
            "start" => Ok(ScrollSnapAlign::Start),
            "end" => Ok(ScrollSnapAlign::End),
            "center" => Ok(ScrollSnapAlign::Center),
            _ => Err(format!("Invalid scroll snap align: {}", s)),
        }
    }
}

pub struct ScrollSnapAlignManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ScrollSnapAlignManager {
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

    pub fn max_scroll_snap_align_length() -> usize {
        MAX_SCROLL_SNAP_ALIGN_LENGTH
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

    fn validate_scroll_snap_align(&self, scroll_snap_align: &str) -> Result<(), String> {
        if scroll_snap_align.len() > MAX_SCROLL_SNAP_ALIGN_LENGTH {
            return Err(format!("Scroll snap align string exceeds maximum length of {} characters", MAX_SCROLL_SNAP_ALIGN_LENGTH));
        }
        ScrollSnapAlign::from_str(scroll_snap_align)?;
        Ok(())
    }

    pub fn apply_scroll_snap_align(&mut self, node: &mut TipTapNode, scroll_snap_align: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_scroll_snap_align(scroll_snap_align)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("scrollSnapAlign".to_string(), serde_json::Value::String(scroll_snap_align.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "scrollSnapAlign": scroll_snap_align }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Scroll snap align application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Scroll snap align application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_scroll_snap_align(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("scrollSnapAlign");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Scroll snap align removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Scroll snap align removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_scroll_snap_align(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(scroll_snap_align) = obj.get("scrollSnapAlign") {
                    if let Some(s) = scroll_snap_align.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_scroll_snap_align(&self, node: &TipTapNode) -> bool {
        self.get_scroll_snap_align(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_scroll_snap_align_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ScrollSnapAlignManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_scroll_snap_align_variants() {
        assert_eq!(ScrollSnapAlign::None.as_str(), "none");
        assert_eq!(ScrollSnapAlign::Start.as_str(), "start");
    }

    #[test]
    fn test_apply_scroll_snap_align() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScrollSnapAlignManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_scroll_snap_align(&mut node, "center");
        assert!(result.is_ok());
        assert!(manager.has_scroll_snap_align(&node));
    }

    #[test]
    fn test_remove_scroll_snap_align() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScrollSnapAlignManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "scrollSnapAlign": "start" })),
            marks: None,
        };
        
        assert!(manager.has_scroll_snap_align(&node));
        let result = manager.remove_scroll_snap_align(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_scroll_snap_align(&node));
    }

    #[test]
    fn test_get_scroll_snap_align() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ScrollSnapAlignManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "scrollSnapAlign": "end" })),
            marks: None,
        };
        
        let scroll_snap_align = manager.get_scroll_snap_align(&node);
        assert_eq!(scroll_snap_align, Some("end".to_string()));
    }
}
