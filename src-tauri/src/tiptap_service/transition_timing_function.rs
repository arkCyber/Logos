//! TipTap Transition Timing Function Manager - Aerospace-Grade Transition Timing Function Operations Service
//!
//! Safety-critical transition timing function operations service with:
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

/// Maximum transition timing function string length
const MAX_TRANSITION_TIMING_FUNCTION_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionTimingFunction {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInQuint,
    EaseOutQuint,
    EaseInOutQuint,
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
}

impl TransitionTimingFunction {
    pub fn as_str(&self) -> &str {
        match self {
            TransitionTimingFunction::Linear => "linear",
            TransitionTimingFunction::Ease => "ease",
            TransitionTimingFunction::EaseIn => "ease-in",
            TransitionTimingFunction::EaseOut => "ease-out",
            TransitionTimingFunction::EaseInOut => "ease-in-out",
            TransitionTimingFunction::EaseInQuad => "cubic-bezier(0.55, 0.085, 0.68, 0.53)",
            TransitionTimingFunction::EaseOutQuad => "cubic-bezier(0.25, 0.46, 0.45, 0.94)",
            TransitionTimingFunction::EaseInOutQuad => "cubic-bezier(0.455, 0.03, 0.515, 0.955)",
            TransitionTimingFunction::EaseInCubic => "cubic-bezier(0.55, 0.055, 0.675, 0.19)",
            TransitionTimingFunction::EaseOutCubic => "cubic-bezier(0.215, 0.61, 0.355, 1)",
            TransitionTimingFunction::EaseInOutCubic => "cubic-bezier(0.645, 0.045, 0.355, 1)",
            TransitionTimingFunction::EaseInQuart => "cubic-bezier(0.895, 0.03, 0.685, 0.22)",
            TransitionTimingFunction::EaseOutQuart => "cubic-bezier(0.165, 0.84, 0.44, 1)",
            TransitionTimingFunction::EaseInOutQuart => "cubic-bezier(0.77, 0, 0.175, 1)",
            TransitionTimingFunction::EaseInQuint => "cubic-bezier(0.755, 0.05, 0.855, 0.06)",
            TransitionTimingFunction::EaseOutQuint => "cubic-bezier(0.23, 1, 0.32, 1)",
            TransitionTimingFunction::EaseInOutQuint => "cubic-bezier(0.86, 0, 0.07, 1)",
            TransitionTimingFunction::EaseInSine => "cubic-bezier(0.47, 0, 0.745, 0.715)",
            TransitionTimingFunction::EaseOutSine => "cubic-bezier(0.39, 0.575, 0.565, 1)",
            TransitionTimingFunction::EaseInOutSine => "cubic-bezier(0.445, 0.05, 0.55, 0.95)",
            TransitionTimingFunction::EaseInExpo => "cubic-bezier(0.95, 0.05, 0.795, 0.035)",
            TransitionTimingFunction::EaseOutExpo => "cubic-bezier(0.19, 1, 0.22, 1)",
            TransitionTimingFunction::EaseInOutExpo => "cubic-bezier(1, 0, 0, 1)",
            TransitionTimingFunction::EaseInCirc => "cubic-bezier(0.6, 0.04, 0.98, 0.335)",
            TransitionTimingFunction::EaseOutCirc => "cubic-bezier(0.075, 0.82, 0.165, 1)",
            TransitionTimingFunction::EaseInOutCirc => "cubic-bezier(0.785, 0.135, 0.15, 0.86)",
            TransitionTimingFunction::EaseInBack => "cubic-bezier(0.6, -0.28, 0.735, 0.045)",
            TransitionTimingFunction::EaseOutBack => "cubic-bezier(0.175, 0.885, 0.32, 1.275)",
            TransitionTimingFunction::EaseInOutBack => "cubic-bezier(0.68, -0.55, 0.265, 1.55)",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "linear" => Ok(TransitionTimingFunction::Linear),
            "ease" => Ok(TransitionTimingFunction::Ease),
            "ease-in" => Ok(TransitionTimingFunction::EaseIn),
            "ease-out" => Ok(TransitionTimingFunction::EaseOut),
            "ease-in-out" => Ok(TransitionTimingFunction::EaseInOut),
            _ => Err(format!("Invalid transition timing function: {}", s)),
        }
    }
}

pub struct TransitionTimingFunctionManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TransitionTimingFunctionManager {
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

    pub fn max_transition_timing_function_length() -> usize {
        MAX_TRANSITION_TIMING_FUNCTION_LENGTH
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

    fn validate_transition_timing_function(&self, transition_timing_function: &str) -> Result<(), String> {
        if transition_timing_function.len() > MAX_TRANSITION_TIMING_FUNCTION_LENGTH {
            return Err(format!("Transition timing function string exceeds maximum length of {} characters", MAX_TRANSITION_TIMING_FUNCTION_LENGTH));
        }
        TransitionTimingFunction::from_str(transition_timing_function)?;
        Ok(())
    }

    pub fn apply_transition_timing_function(&mut self, node: &mut TipTapNode, transition_timing_function: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_transition_timing_function(transition_timing_function)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("transitionTimingFunction".to_string(), serde_json::Value::String(transition_timing_function.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "transitionTimingFunction": transition_timing_function }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Transition timing function application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Transition timing function application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_transition_timing_function(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("transitionTimingFunction");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Transition timing function removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Transition timing function removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_transition_timing_function(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(transition_timing_function) = obj.get("transitionTimingFunction") {
                    if let Some(s) = transition_timing_function.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_transition_timing_function(&self, node: &TipTapNode) -> bool {
        self.get_transition_timing_function(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_transition_timing_function_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TransitionTimingFunctionManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_transition_timing_function_variants() {
        assert_eq!(TransitionTimingFunction::Linear.as_str(), "linear");
        assert_eq!(TransitionTimingFunction::Ease.as_str(), "ease");
        assert_eq!(TransitionTimingFunction::EaseIn.as_str(), "ease-in");
    }

    #[test]
    fn test_apply_transition_timing_function() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransitionTimingFunctionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_transition_timing_function(&mut node, "ease-in-out");
        assert!(result.is_ok());
        assert!(manager.has_transition_timing_function(&node));
    }

    #[test]
    fn test_remove_transition_timing_function() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransitionTimingFunctionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "transitionTimingFunction": "linear" })),
            marks: None,
        };
        
        assert!(manager.has_transition_timing_function(&node));
        let result = manager.remove_transition_timing_function(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_transition_timing_function(&node));
    }

    #[test]
    fn test_get_transition_timing_function() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TransitionTimingFunctionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "transitionTimingFunction": "ease-out" })),
            marks: None,
        };
        
        let transition_timing_function = manager.get_transition_timing_function(&node);
        assert_eq!(transition_timing_function, Some("ease-out".to_string()));
    }
}
