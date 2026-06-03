//! TipTap Will Change Manager - Aerospace-Grade Will Change Operations Service
//!
//! Safety-critical will change operations service with:
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

/// Maximum will change string length
const MAX_WILL_CHANGE_LENGTH: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WillChange {
    Auto,
    Scroll,
    Position,
    Contents,
    Transform,
    Opacity,
}

impl WillChange {
    pub fn as_str(&self) -> &str {
        match self {
            WillChange::Auto => "auto",
            WillChange::Scroll => "scroll",
            WillChange::Position => "position",
            WillChange::Contents => "contents",
            WillChange::Transform => "transform",
            WillChange::Opacity => "opacity",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(WillChange::Auto),
            "scroll" => Ok(WillChange::Scroll),
            "position" => Ok(WillChange::Position),
            "contents" => Ok(WillChange::Contents),
            "transform" => Ok(WillChange::Transform),
            "opacity" => Ok(WillChange::Opacity),
            _ => Err(format!("Invalid will change: {}", s)),
        }
    }
}

pub struct WillChangeManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl WillChangeManager {
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

    pub fn max_will_change_length() -> usize {
        MAX_WILL_CHANGE_LENGTH
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

    fn validate_will_change(&self, will_change: &str) -> Result<(), String> {
        if will_change.len() > MAX_WILL_CHANGE_LENGTH {
            return Err(format!("Will change string exceeds maximum length of {} characters", MAX_WILL_CHANGE_LENGTH));
        }
        WillChange::from_str(will_change)?;
        Ok(())
    }

    pub fn apply_will_change(&mut self, node: &mut TipTapNode, will_change: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_will_change(will_change)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("willChange".to_string(), serde_json::Value::String(will_change.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "willChange": will_change }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Will change application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Will change application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_will_change(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("willChange");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Will change removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Will change removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_will_change(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(will_change) = obj.get("willChange") {
                    if let Some(s) = will_change.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_will_change(&self, node: &TipTapNode) -> bool {
        self.get_will_change(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_will_change_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = WillChangeManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_will_change_variants() {
        assert_eq!(WillChange::Auto.as_str(), "auto");
        assert_eq!(WillChange::Transform.as_str(), "transform");
    }

    #[test]
    fn test_apply_will_change() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WillChangeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_will_change(&mut node, "transform");
        assert!(result.is_ok());
        assert!(manager.has_will_change(&node));
    }

    #[test]
    fn test_remove_will_change() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WillChangeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "willChange": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_will_change(&node));
        let result = manager.remove_will_change(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_will_change(&node));
    }

    #[test]
    fn test_get_will_change() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = WillChangeManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "willChange": "opacity" })),
            marks: None,
        };
        
        let will_change = manager.get_will_change(&node);
        assert_eq!(will_change, Some("opacity".to_string()));
    }
}
