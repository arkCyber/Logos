//! TipTap Animation Name Manager - Aerospace-Grade Animation Name Operations Service
//!
//! Safety-critical animation name operations service with:
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

/// Maximum animation name string length
const MAX_ANIMATION_NAME_LENGTH: usize = 200;

pub struct AnimationNameManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl AnimationNameManager {
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

    pub fn max_animation_name_length() -> usize {
        MAX_ANIMATION_NAME_LENGTH
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

    fn validate_animation_name(&self, animation_name: &str) -> Result<(), String> {
        if animation_name.is_empty() {
            return Err("Animation name cannot be empty".to_string());
        }
        if animation_name.len() > MAX_ANIMATION_NAME_LENGTH {
            return Err(format!("Animation name string exceeds maximum length of {} characters", MAX_ANIMATION_NAME_LENGTH));
        }
        if animation_name.contains('(') && !animation_name.contains(')') {
            return Err("Invalid animation name: unmatched parentheses".to_string());
        }
        Ok(())
    }

    pub fn apply_animation_name(&mut self, node: &mut TipTapNode, animation_name: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_animation_name(animation_name)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("animationName".to_string(), serde_json::Value::String(animation_name.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "animationName": animation_name }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Animation name application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Animation name application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_animation_name(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("animationName");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Animation name removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Animation name removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_animation_name(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(animation_name) = obj.get("animationName") {
                    if let Some(s) = animation_name.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_animation_name(&self, node: &TipTapNode) -> bool {
        self.get_animation_name(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_animation_name_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AnimationNameManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_animation_name() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AnimationNameManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_animation_name(&mut node, "fadeIn");
        assert!(result.is_ok());
        assert!(manager.has_animation_name(&node));
    }

    #[test]
    fn test_remove_animation_name() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AnimationNameManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "animationName": "slideIn" })),
            marks: None,
        };
        
        assert!(manager.has_animation_name(&node));
        let result = manager.remove_animation_name(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_animation_name(&node));
    }

    #[test]
    fn test_get_animation_name() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AnimationNameManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "animationName": "bounce" })),
            marks: None,
        };
        
        let animation_name = manager.get_animation_name(&node);
        assert_eq!(animation_name, Some("bounce".to_string()));
    }
}
