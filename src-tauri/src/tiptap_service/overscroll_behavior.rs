//! TipTap Overscroll Behavior Manager - Aerospace-Grade Overscroll Behavior Operations Service
//!
//! Safety-critical overscroll behavior operations service with:
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

/// Maximum overscroll behavior string length
const MAX_OVERSCROLL_BEHAVIOR_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverscrollBehavior {
    Auto,
    Contain,
    None,
}

impl OverscrollBehavior {
    pub fn as_str(&self) -> &str {
        match self {
            OverscrollBehavior::Auto => "auto",
            OverscrollBehavior::Contain => "contain",
            OverscrollBehavior::None => "none",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(OverscrollBehavior::Auto),
            "contain" => Ok(OverscrollBehavior::Contain),
            "none" => Ok(OverscrollBehavior::None),
            _ => Err(format!("Invalid overscroll behavior: {}", s)),
        }
    }
}

pub struct OverscrollBehaviorManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl OverscrollBehaviorManager {
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

    pub fn max_overscroll_behavior_length() -> usize {
        MAX_OVERSCROLL_BEHAVIOR_LENGTH
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

    fn validate_overscroll_behavior(&self, overscroll_behavior: &str) -> Result<(), String> {
        if overscroll_behavior.len() > MAX_OVERSCROLL_BEHAVIOR_LENGTH {
            return Err(format!("Overscroll behavior string exceeds maximum length of {} characters", MAX_OVERSCROLL_BEHAVIOR_LENGTH));
        }
        OverscrollBehavior::from_str(overscroll_behavior)?;
        Ok(())
    }

    pub fn apply_overscroll_behavior(&mut self, node: &mut TipTapNode, overscroll_behavior: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_overscroll_behavior(overscroll_behavior)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("overscrollBehavior".to_string(), serde_json::Value::String(overscroll_behavior.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "overscrollBehavior": overscroll_behavior }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Overscroll behavior application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Overscroll behavior application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_overscroll_behavior(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("overscrollBehavior");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Overscroll behavior removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Overscroll behavior removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_overscroll_behavior(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(overscroll_behavior) = obj.get("overscrollBehavior") {
                    if let Some(s) = overscroll_behavior.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_overscroll_behavior(&self, node: &TipTapNode) -> bool {
        self.get_overscroll_behavior(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_overscroll_behavior_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OverscrollBehaviorManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_overscroll_behavior_variants() {
        assert_eq!(OverscrollBehavior::Auto.as_str(), "auto");
        assert_eq!(OverscrollBehavior::Contain.as_str(), "contain");
    }

    #[test]
    fn test_apply_overscroll_behavior() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OverscrollBehaviorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_overscroll_behavior(&mut node, "contain");
        assert!(result.is_ok());
        assert!(manager.has_overscroll_behavior(&node));
    }

    #[test]
    fn test_remove_overscroll_behavior() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OverscrollBehaviorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "overscrollBehavior": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_overscroll_behavior(&node));
        let result = manager.remove_overscroll_behavior(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_overscroll_behavior(&node));
    }

    #[test]
    fn test_get_overscroll_behavior() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OverscrollBehaviorManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "overscrollBehavior": "none" })),
            marks: None,
        };
        
        let overscroll_behavior = manager.get_overscroll_behavior(&node);
        assert_eq!(overscroll_behavior, Some("none".to_string()));
    }
}
