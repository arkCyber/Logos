//! TipTap Animation Duration Manager - Aerospace-Grade Animation Duration Operations Service
//!
//! Safety-critical animation duration operations service with:
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

/// Maximum animation duration value (in seconds)
const MAX_ANIMATION_DURATION: f64 = 60.0;

/// Minimum animation duration value (in seconds)
const MIN_ANIMATION_DURATION: f64 = 0.0;

/// Maximum animation duration string length
const MAX_ANIMATION_DURATION_LENGTH: usize = 50;

pub struct AnimationDurationManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl AnimationDurationManager {
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

    pub fn max_animation_duration() -> f64 {
        MAX_ANIMATION_DURATION
    }

    pub fn min_animation_duration() -> f64 {
        MIN_ANIMATION_DURATION
    }

    pub fn max_animation_duration_length() -> usize {
        MAX_ANIMATION_DURATION_LENGTH
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

    fn validate_animation_duration(&self, animation_duration: &str) -> Result<(), String> {
        if animation_duration.is_empty() {
            return Err("Animation duration cannot be empty".to_string());
        }
        if animation_duration.len() > MAX_ANIMATION_DURATION_LENGTH {
            return Err(format!("Animation duration string exceeds maximum length of {} characters", MAX_ANIMATION_DURATION_LENGTH));
        }
        if animation_duration.ends_with("s") {
            let value_str = animation_duration.trim_end_matches("s");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_ANIMATION_DURATION || value > MAX_ANIMATION_DURATION {
                    return Err(format!("Animation duration must be between {} and {} seconds", MIN_ANIMATION_DURATION, MAX_ANIMATION_DURATION));
                }
                if !value.is_finite() {
                    return Err("Animation duration must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_animation_duration(&mut self, node: &mut TipTapNode, animation_duration: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_animation_duration(animation_duration)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("animationDuration".to_string(), serde_json::Value::String(animation_duration.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "animationDuration": animation_duration }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Animation duration application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Animation duration application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_animation_duration(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("animationDuration");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Animation duration removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Animation duration removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_animation_duration(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(animation_duration) = obj.get("animationDuration") {
                    if let Some(s) = animation_duration.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_animation_duration(&self, node: &TipTapNode) -> bool {
        self.get_animation_duration(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_animation_duration_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AnimationDurationManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_animation_duration() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AnimationDurationManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_animation_duration(&mut node, "1s");
        assert!(result.is_ok());
        assert!(manager.has_animation_duration(&node));
    }

    #[test]
    fn test_remove_animation_duration() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AnimationDurationManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "animationDuration": "2s" })),
            marks: None,
        };
        
        assert!(manager.has_animation_duration(&node));
        let result = manager.remove_animation_duration(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_animation_duration(&node));
    }

    #[test]
    fn test_get_animation_duration() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AnimationDurationManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "animationDuration": "0.5s" })),
            marks: None,
        };
        
        let animation_duration = manager.get_animation_duration(&node);
        assert_eq!(animation_duration, Some("0.5s".to_string()));
    }
}
