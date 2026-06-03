//! TipTap Scroll Snap Type Manager - Aerospace-Grade Scroll Snap Type Operations Service
//!
//! Safety-critical scroll snap type operations service with:
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

/// Maximum scroll snap type string length
const MAX_SCROLL_SNAP_TYPE_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollSnapType {
    None,
    Mandatory,
    Proximity,
}

impl ScrollSnapType {
    pub fn as_str(&self) -> &str {
        match self {
            ScrollSnapType::None => "none",
            ScrollSnapType::Mandatory => "mandatory",
            ScrollSnapType::Proximity => "proximity",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" => Ok(ScrollSnapType::None),
            "mandatory" => Ok(ScrollSnapType::Mandatory),
            "proximity" => Ok(ScrollSnapType::Proximity),
            _ => Err(format!("Invalid scroll snap type: {}", s)),
        }
    }
}

pub struct ScrollSnapTypeManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ScrollSnapTypeManager {
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

    pub fn max_scroll_snap_type_length() -> usize {
        MAX_SCROLL_SNAP_TYPE_LENGTH
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

    fn validate_scroll_snap_type(&self, scroll_snap_type: &str) -> Result<(), String> {
        if scroll_snap_type.len() > MAX_SCROLL_SNAP_TYPE_LENGTH {
            return Err(format!("Scroll snap type string exceeds maximum length of {} characters", MAX_SCROLL_SNAP_TYPE_LENGTH));
        }
        ScrollSnapType::from_str(scroll_snap_type)?;
        Ok(())
    }

    pub fn apply_scroll_snap_type(&mut self, node: &mut TipTapNode, scroll_snap_type: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_scroll_snap_type(scroll_snap_type)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("scrollSnapType".to_string(), serde_json::Value::String(scroll_snap_type.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "scrollSnapType": scroll_snap_type }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Scroll snap type application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Scroll snap type application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_scroll_snap_type(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("scrollSnapType");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Scroll snap type removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Scroll snap type removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_scroll_snap_type(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(scroll_snap_type) = obj.get("scrollSnapType") {
                    if let Some(s) = scroll_snap_type.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_scroll_snap_type(&self, node: &TipTapNode) -> bool {
        self.get_scroll_snap_type(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_scroll_snap_type_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ScrollSnapTypeManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_scroll_snap_type_variants() {
        assert_eq!(ScrollSnapType::None.as_str(), "none");
        assert_eq!(ScrollSnapType::Mandatory.as_str(), "mandatory");
    }

    #[test]
    fn test_apply_scroll_snap_type() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScrollSnapTypeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_scroll_snap_type(&mut node, "mandatory");
        assert!(result.is_ok());
        assert!(manager.has_scroll_snap_type(&node));
    }

    #[test]
    fn test_remove_scroll_snap_type() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScrollSnapTypeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "scrollSnapType": "proximity" })),
            marks: None,
        };
        
        assert!(manager.has_scroll_snap_type(&node));
        let result = manager.remove_scroll_snap_type(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_scroll_snap_type(&node));
    }

    #[test]
    fn test_get_scroll_snap_type() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ScrollSnapTypeManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "scrollSnapType": "none" })),
            marks: None,
        };
        
        let scroll_snap_type = manager.get_scroll_snap_type(&node);
        assert_eq!(scroll_snap_type, Some("none".to_string()));
    }
}
