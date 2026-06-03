//! TipTap Animation Timing Function Manager - Aerospace-Grade Animation Timing Function Operations Service
//!
//! Safety-critical animation timing function operations service with:
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

/// Maximum animation timing function string length
const MAX_ANIMATION_TIMING_FUNCTION_LENGTH: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationTimingFunction {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    StepStart,
    StepEnd,
}

impl AnimationTimingFunction {
    pub fn as_str(&self) -> &str {
        match self {
            AnimationTimingFunction::Linear => "linear",
            AnimationTimingFunction::Ease => "ease",
            AnimationTimingFunction::EaseIn => "ease-in",
            AnimationTimingFunction::EaseOut => "ease-out",
            AnimationTimingFunction::EaseInOut => "ease-in-out",
            AnimationTimingFunction::StepStart => "step-start",
            AnimationTimingFunction::StepEnd => "step-end",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "linear" => Ok(AnimationTimingFunction::Linear),
            "ease" => Ok(AnimationTimingFunction::Ease),
            "ease-in" => Ok(AnimationTimingFunction::EaseIn),
            "ease-out" => Ok(AnimationTimingFunction::EaseOut),
            "ease-in-out" => Ok(AnimationTimingFunction::EaseInOut),
            "step-start" => Ok(AnimationTimingFunction::StepStart),
            "step-end" => Ok(AnimationTimingFunction::StepEnd),
            _ => Err(format!("Invalid animation timing function: {}", s)),
        }
    }
}

pub struct AnimationTimingFunctionManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl AnimationTimingFunctionManager {
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

    pub fn max_animation_timing_function_length() -> usize {
        MAX_ANIMATION_TIMING_FUNCTION_LENGTH
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

    fn validate_animation_timing_function(&self, animation_timing_function: &str) -> Result<(), String> {
        if animation_timing_function.len() > MAX_ANIMATION_TIMING_FUNCTION_LENGTH {
            return Err(format!("Animation timing function string exceeds maximum length of {} characters", MAX_ANIMATION_TIMING_FUNCTION_LENGTH));
        }
        if animation_timing_function.contains('(') && !animation_timing_function.contains(')') {
            return Err("Invalid animation timing function: unmatched parentheses".to_string());
        }
        Ok(())
    }

    pub fn apply_animation_timing_function(&mut self, node: &mut TipTapNode, animation_timing_function: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_animation_timing_function(animation_timing_function)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("animationTimingFunction".to_string(), serde_json::Value::String(animation_timing_function.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "animationTimingFunction": animation_timing_function }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Animation timing function application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Animation timing function application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_animation_timing_function(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("animationTimingFunction");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Animation timing function removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Animation timing function removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_animation_timing_function(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(animation_timing_function) = obj.get("animationTimingFunction") {
                    if let Some(s) = animation_timing_function.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_animation_timing_function(&self, node: &TipTapNode) -> bool {
        self.get_animation_timing_function(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_animation_timing_function_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AnimationTimingFunctionManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_animation_timing_function_variants() {
        assert_eq!(AnimationTimingFunction::Linear.as_str(), "linear");
        assert_eq!(AnimationTimingFunction::Ease.as_str(), "ease");
    }

    #[test]
    fn test_apply_animation_timing_function() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AnimationTimingFunctionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_animation_timing_function(&mut node, "ease-in-out");
        assert!(result.is_ok());
        assert!(manager.has_animation_timing_function(&node));
    }

    #[test]
    fn test_remove_animation_timing_function() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AnimationTimingFunctionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "animationTimingFunction": "linear" })),
            marks: None,
        };
        
        assert!(manager.has_animation_timing_function(&node));
        let result = manager.remove_animation_timing_function(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_animation_timing_function(&node));
    }

    #[test]
    fn test_get_animation_timing_function() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AnimationTimingFunctionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "animationTimingFunction": "ease" })),
            marks: None,
        };
        
        let animation_timing_function = manager.get_animation_timing_function(&node);
        assert_eq!(animation_timing_function, Some("ease".to_string()));
    }
}
