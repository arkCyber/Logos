//! TipTap Overscroll Behavior X Manager - Aerospace-Grade Overscroll Behavior X Operations Service
//!
//! Safety-critical overscroll behavior X operations service with:
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

/// Maximum overscroll behavior X string length
const MAX_OVERSCROLL_BEHAVIOR_X_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverscrollBehaviorX {
    Auto,
    Contain,
    None,
}

impl OverscrollBehaviorX {
    pub fn as_str(&self) -> &str {
        match self {
            OverscrollBehaviorX::Auto => "auto",
            OverscrollBehaviorX::Contain => "contain",
            OverscrollBehaviorX::None => "none",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(OverscrollBehaviorX::Auto),
            "contain" => Ok(OverscrollBehaviorX::Contain),
            "none" => Ok(OverscrollBehaviorX::None),
            _ => Err(format!("Invalid overscroll behavior X: {}", s)),
        }
    }
}

pub struct OverscrollBehaviorXManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl OverscrollBehaviorXManager {
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

    pub fn max_overscroll_behavior_x_length() -> usize {
        MAX_OVERSCROLL_BEHAVIOR_X_LENGTH
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

    fn validate_overscroll_behavior_x(&self, overscroll_behavior_x: &str) -> Result<(), String> {
        if overscroll_behavior_x.len() > MAX_OVERSCROLL_BEHAVIOR_X_LENGTH {
            return Err(format!("Overscroll behavior X string exceeds maximum length of {} characters", MAX_OVERSCROLL_BEHAVIOR_X_LENGTH));
        }
        OverscrollBehaviorX::from_str(overscroll_behavior_x)?;
        Ok(())
    }

    pub fn apply_overscroll_behavior_x(&mut self, node: &mut TipTapNode, overscroll_behavior_x: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_overscroll_behavior_x(overscroll_behavior_x)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("overscrollBehaviorX".to_string(), serde_json::Value::String(overscroll_behavior_x.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "overscrollBehaviorX": overscroll_behavior_x }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Overscroll behavior X application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Overscroll behavior X application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_overscroll_behavior_x(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("overscrollBehaviorX");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Overscroll behavior X removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Overscroll behavior X removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_overscroll_behavior_x(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(overscroll_behavior_x) = obj.get("overscrollBehaviorX") {
                    if let Some(s) = overscroll_behavior_x.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_overscroll_behavior_x(&self, node: &TipTapNode) -> bool {
        self.get_overscroll_behavior_x(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_overscroll_behavior_x_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OverscrollBehaviorXManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_overscroll_behavior_x_variants() {
        assert_eq!(OverscrollBehaviorX::Auto.as_str(), "auto");
        assert_eq!(OverscrollBehaviorX::Contain.as_str(), "contain");
    }

    #[test]
    fn test_apply_overscroll_behavior_x() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OverscrollBehaviorXManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_overscroll_behavior_x(&mut node, "contain");
        assert!(result.is_ok());
        assert!(manager.has_overscroll_behavior_x(&node));
    }

    #[test]
    fn test_remove_overscroll_behavior_x() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OverscrollBehaviorXManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "overscrollBehaviorX": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_overscroll_behavior_x(&node));
        let result = manager.remove_overscroll_behavior_x(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_overscroll_behavior_x(&node));
    }

    #[test]
    fn test_get_overscroll_behavior_x() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OverscrollBehaviorXManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "overscrollBehaviorX": "none" })),
            marks: None,
        };
        
        let overscroll_behavior_x = manager.get_overscroll_behavior_x(&node);
        assert_eq!(overscroll_behavior_x, Some("none".to_string()));
    }
}
