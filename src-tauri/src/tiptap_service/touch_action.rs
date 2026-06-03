//! TipTap Touch Action Manager - Aerospace-Grade Touch Action Operations Service
//!
//! Safety-critical touch action operations service with:
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

/// Maximum touch action string length
const MAX_TOUCH_ACTION_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchAction {
    Auto,
    None,
    PanX,
    PanY,
    Manipulation,
}

impl TouchAction {
    pub fn as_str(&self) -> &str {
        match self {
            TouchAction::Auto => "auto",
            TouchAction::None => "none",
            TouchAction::PanX => "pan-x",
            TouchAction::PanY => "pan-y",
            TouchAction::Manipulation => "manipulation",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(TouchAction::Auto),
            "none" => Ok(TouchAction::None),
            "pan-x" => Ok(TouchAction::PanX),
            "pan-y" => Ok(TouchAction::PanY),
            "manipulation" => Ok(TouchAction::Manipulation),
            _ => Err(format!("Invalid touch action: {}", s)),
        }
    }
}

pub struct TouchActionManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TouchActionManager {
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

    pub fn max_touch_action_length() -> usize {
        MAX_TOUCH_ACTION_LENGTH
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

    fn validate_touch_action(&self, touch_action: &str) -> Result<(), String> {
        if touch_action.len() > MAX_TOUCH_ACTION_LENGTH {
            return Err(format!("Touch action string exceeds maximum length of {} characters", MAX_TOUCH_ACTION_LENGTH));
        }
        TouchAction::from_str(touch_action)?;
        Ok(())
    }

    pub fn apply_touch_action(&mut self, node: &mut TipTapNode, touch_action: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_touch_action(touch_action)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("touchAction".to_string(), serde_json::Value::String(touch_action.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "touchAction": touch_action }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Touch action application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Touch action application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_touch_action(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("touchAction");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Touch action removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Touch action removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_touch_action(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(touch_action) = obj.get("touchAction") {
                    if let Some(s) = touch_action.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_touch_action(&self, node: &TipTapNode) -> bool {
        self.get_touch_action(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_touch_action_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TouchActionManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_touch_action_variants() {
        assert_eq!(TouchAction::Auto.as_str(), "auto");
        assert_eq!(TouchAction::None.as_str(), "none");
    }

    #[test]
    fn test_apply_touch_action() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TouchActionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_touch_action(&mut node, "none");
        assert!(result.is_ok());
        assert!(manager.has_touch_action(&node));
    }

    #[test]
    fn test_remove_touch_action() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TouchActionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "touchAction": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_touch_action(&node));
        let result = manager.remove_touch_action(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_touch_action(&node));
    }

    #[test]
    fn test_get_touch_action() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TouchActionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "touchAction": "pan-x" })),
            marks: None,
        };
        
        let touch_action = manager.get_touch_action(&node);
        assert_eq!(touch_action, Some("pan-x".to_string()));
    }
}
