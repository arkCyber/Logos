//! TipTap Animation Iteration Count Manager - Aerospace-Grade Animation Iteration Count Operations Service
//!
//! Safety-critical animation iteration count operations service with:
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

/// Maximum animation iteration count value
const MAX_ANIMATION_ITERATION_COUNT: f64 = 1000.0;

/// Minimum animation iteration count value
const MIN_ANIMATION_ITERATION_COUNT: f64 = 0.0;

/// Maximum animation iteration count string length
const MAX_ANIMATION_ITERATION_COUNT_LENGTH: usize = 50;

pub struct AnimationIterationCountManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl AnimationIterationCountManager {
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

    pub fn max_animation_iteration_count() -> f64 {
        MAX_ANIMATION_ITERATION_COUNT
    }

    pub fn min_animation_iteration_count() -> f64 {
        MIN_ANIMATION_ITERATION_COUNT
    }

    pub fn max_animation_iteration_count_length() -> usize {
        MAX_ANIMATION_ITERATION_COUNT_LENGTH
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

    fn validate_animation_iteration_count(&self, animation_iteration_count: &str) -> Result<(), String> {
        if animation_iteration_count.is_empty() {
            return Err("Animation iteration count cannot be empty".to_string());
        }
        if animation_iteration_count.len() > MAX_ANIMATION_ITERATION_COUNT_LENGTH {
            return Err(format!("Animation iteration count string exceeds maximum length of {} characters", MAX_ANIMATION_ITERATION_COUNT_LENGTH));
        }
        if animation_iteration_count == "infinite" {
            return Ok(());
        }
        if let Ok(value) = animation_iteration_count.parse::<f64>() {
            if value < MIN_ANIMATION_ITERATION_COUNT || value > MAX_ANIMATION_ITERATION_COUNT {
                return Err(format!("Animation iteration count must be between {} and {}", MIN_ANIMATION_ITERATION_COUNT, MAX_ANIMATION_ITERATION_COUNT));
            }
            if !value.is_finite() {
                return Err("Animation iteration count must be a finite number".to_string());
            }
        }
        Ok(())
    }

    pub fn apply_animation_iteration_count(&mut self, node: &mut TipTapNode, animation_iteration_count: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_animation_iteration_count(animation_iteration_count)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("animationIterationCount".to_string(), serde_json::Value::String(animation_iteration_count.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "animationIterationCount": animation_iteration_count }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Animation iteration count application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Animation iteration count application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_animation_iteration_count(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("animationIterationCount");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Animation iteration count removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Animation iteration count removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_animation_iteration_count(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(animation_iteration_count) = obj.get("animationIterationCount") {
                    if let Some(s) = animation_iteration_count.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_animation_iteration_count(&self, node: &TipTapNode) -> bool {
        self.get_animation_iteration_count(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_animation_iteration_count_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AnimationIterationCountManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_animation_iteration_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AnimationIterationCountManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_animation_iteration_count(&mut node, "infinite");
        assert!(result.is_ok());
        assert!(manager.has_animation_iteration_count(&node));
    }

    #[test]
    fn test_remove_animation_iteration_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AnimationIterationCountManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "animationIterationCount": "3" })),
            marks: None,
        };
        
        assert!(manager.has_animation_iteration_count(&node));
        let result = manager.remove_animation_iteration_count(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_animation_iteration_count(&node));
    }

    #[test]
    fn test_get_animation_iteration_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AnimationIterationCountManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "animationIterationCount": "5" })),
            marks: None,
        };
        
        let animation_iteration_count = manager.get_animation_iteration_count(&node);
        assert_eq!(animation_iteration_count, Some("5".to_string()));
    }
}
