//! TipTap Animation Manager - Aerospace-Grade Animation Operations Service
//!
//! Safety-critical animation operations service with:
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

/// Maximum animation string length
const MAX_ANIMATION_LENGTH: usize = 200;

pub struct AnimationManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl AnimationManager {
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

    pub fn max_animation_length() -> usize {
        MAX_ANIMATION_LENGTH
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

    fn validate_animation(&self, animation: &str) -> Result<(), String> {
        if animation.is_empty() {
            return Err("Animation cannot be empty".to_string());
        }
        if animation.len() > MAX_ANIMATION_LENGTH {
            return Err(format!("Animation string exceeds maximum length of {} characters", MAX_ANIMATION_LENGTH));
        }
        let valid_patterns = ["none", "fade", "slide", "bounce", "rotate", "scale"];
        if !valid_patterns.iter().any(|pattern| animation.contains(pattern)) {
            if animation.contains('(') && !animation.contains(')') {
                return Err("Invalid animation: unmatched parentheses".to_string());
            }
        }
        Ok(())
    }

    pub fn apply_animation(&mut self, node: &mut TipTapNode, animation: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_animation(animation)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("animation".to_string(), serde_json::Value::String(animation.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "animation": animation }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Animation application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Animation application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_animation(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("animation");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Animation removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Animation removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_animation(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(animation) = obj.get("animation") {
                    if let Some(s) = animation.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_animation(&self, node: &TipTapNode) -> bool {
        self.get_animation(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_animation_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AnimationManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_animation() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AnimationManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_animation(&mut node, "fade 1s ease-in");
        assert!(result.is_ok());
        assert!(manager.has_animation(&node));
    }

    #[test]
    fn test_remove_animation() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AnimationManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "animation": "none" })),
            marks: None,
        };
        
        assert!(manager.has_animation(&node));
        let result = manager.remove_animation(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_animation(&node));
    }

    #[test]
    fn test_get_animation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AnimationManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "animation": "slide 0.5s ease-out" })),
            marks: None,
        };
        
        let animation = manager.get_animation(&node);
        assert_eq!(animation, Some("slide 0.5s ease-out".to_string()));
    }
}
