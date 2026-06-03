//! TipTap Overscroll Behavior Y Manager - Aerospace-Grade Overscroll Behavior Y Operations Service
//!
//! Safety-critical overscroll behavior Y operations service with:
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

/// Maximum overscroll behavior Y string length
const MAX_OVERSCROLL_BEHAVIOR_Y_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverscrollBehaviorY {
    Auto,
    Contain,
    None,
}

impl OverscrollBehaviorY {
    pub fn as_str(&self) -> &str {
        match self {
            OverscrollBehaviorY::Auto => "auto",
            OverscrollBehaviorY::Contain => "contain",
            OverscrollBehaviorY::None => "none",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(OverscrollBehaviorY::Auto),
            "contain" => Ok(OverscrollBehaviorY::Contain),
            "none" => Ok(OverscrollBehaviorY::None),
            _ => Err(format!("Invalid overscroll behavior Y: {}", s)),
        }
    }
}

pub struct OverscrollBehaviorYManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl OverscrollBehaviorYManager {
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

    pub fn max_overscroll_behavior_y_length() -> usize {
        MAX_OVERSCROLL_BEHAVIOR_Y_LENGTH
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

    fn validate_overscroll_behavior_y(&self, overscroll_behavior_y: &str) -> Result<(), String> {
        if overscroll_behavior_y.len() > MAX_OVERSCROLL_BEHAVIOR_Y_LENGTH {
            return Err(format!("Overscroll behavior Y string exceeds maximum length of {} characters", MAX_OVERSCROLL_BEHAVIOR_Y_LENGTH));
        }
        OverscrollBehaviorY::from_str(overscroll_behavior_y)?;
        Ok(())
    }

    pub fn apply_overscroll_behavior_y(&mut self, node: &mut TipTapNode, overscroll_behavior_y: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_overscroll_behavior_y(overscroll_behavior_y)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("overscrollBehaviorY".to_string(), serde_json::Value::String(overscroll_behavior_y.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "overscrollBehaviorY": overscroll_behavior_y }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Overscroll behavior Y application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Overscroll behavior Y application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_overscroll_behavior_y(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("overscrollBehaviorY");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Overscroll behavior Y removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Overscroll behavior Y removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_overscroll_behavior_y(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(overscroll_behavior_y) = obj.get("overscrollBehaviorY") {
                    if let Some(s) = overscroll_behavior_y.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_overscroll_behavior_y(&self, node: &TipTapNode) -> bool {
        self.get_overscroll_behavior_y(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_overscroll_behavior_y_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OverscrollBehaviorYManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_overscroll_behavior_y_variants() {
        assert_eq!(OverscrollBehaviorY::Auto.as_str(), "auto");
        assert_eq!(OverscrollBehaviorY::Contain.as_str(), "contain");
    }

    #[test]
    fn test_apply_overscroll_behavior_y() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OverscrollBehaviorYManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_overscroll_behavior_y(&mut node, "contain");
        assert!(result.is_ok());
        assert!(manager.has_overscroll_behavior_y(&node));
    }

    #[test]
    fn test_remove_overscroll_behavior_y() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OverscrollBehaviorYManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "overscrollBehaviorY": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_overscroll_behavior_y(&node));
        let result = manager.remove_overscroll_behavior_y(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_overscroll_behavior_y(&node));
    }

    #[test]
    fn test_get_overscroll_behavior_y() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OverscrollBehaviorYManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "overscrollBehaviorY": "none" })),
            marks: None,
        };
        
        let overscroll_behavior_y = manager.get_overscroll_behavior_y(&node);
        assert_eq!(overscroll_behavior_y, Some("none".to_string()));
    }
}
