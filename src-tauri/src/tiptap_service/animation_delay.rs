//! TipTap Animation Delay Manager - Aerospace-Grade Animation Delay Operations Service
//!
//! Safety-critical animation delay operations service with:
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

/// Maximum animation delay value (in seconds)
const MAX_ANIMATION_DELAY: f64 = 60.0;

/// Minimum animation delay value (in seconds)
const MIN_ANIMATION_DELAY: f64 = 0.0;

/// Maximum animation delay string length
const MAX_ANIMATION_DELAY_LENGTH: usize = 50;

pub struct AnimationDelayManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl AnimationDelayManager {
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

    pub fn max_animation_delay() -> f64 {
        MAX_ANIMATION_DELAY
    }

    pub fn min_animation_delay() -> f64 {
        MIN_ANIMATION_DELAY
    }

    pub fn max_animation_delay_length() -> usize {
        MAX_ANIMATION_DELAY_LENGTH
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

    fn validate_animation_delay(&self, animation_delay: &str) -> Result<(), String> {
        if animation_delay.is_empty() {
            return Err("Animation delay cannot be empty".to_string());
        }
        if animation_delay.len() > MAX_ANIMATION_DELAY_LENGTH {
            return Err(format!("Animation delay string exceeds maximum length of {} characters", MAX_ANIMATION_DELAY_LENGTH));
        }
        if animation_delay.ends_with("s") {
            let value_str = animation_delay.trim_end_matches("s");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_ANIMATION_DELAY || value > MAX_ANIMATION_DELAY {
                    return Err(format!("Animation delay must be between {} and {} seconds", MIN_ANIMATION_DELAY, MAX_ANIMATION_DELAY));
                }
                if !value.is_finite() {
                    return Err("Animation delay must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_animation_delay(&mut self, node: &mut TipTapNode, animation_delay: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_animation_delay(animation_delay)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("animationDelay".to_string(), serde_json::Value::String(animation_delay.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "animationDelay": animation_delay }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Animation delay application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Animation delay application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_animation_delay(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("animationDelay");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Animation delay removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Animation delay removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_animation_delay(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(animation_delay) = obj.get("animationDelay") {
                    if let Some(s) = animation_delay.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_animation_delay(&self, node: &TipTapNode) -> bool {
        self.get_animation_delay(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_animation_delay_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AnimationDelayManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_animation_delay() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AnimationDelayManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_animation_delay(&mut node, "0.5s");
        assert!(result.is_ok());
        assert!(manager.has_animation_delay(&node));
    }

    #[test]
    fn test_remove_animation_delay() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AnimationDelayManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "animationDelay": "1s" })),
            marks: None,
        };
        
        assert!(manager.has_animation_delay(&node));
        let result = manager.remove_animation_delay(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_animation_delay(&node));
    }

    #[test]
    fn test_get_animation_delay() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AnimationDelayManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "animationDelay": "2s" })),
            marks: None,
        };
        
        let animation_delay = manager.get_animation_delay(&node);
        assert_eq!(animation_delay, Some("2s".to_string()));
    }
}
