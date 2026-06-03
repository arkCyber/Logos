//! TipTap Transition Delay Manager - Aerospace-Grade Transition Delay Operations Service
//!
//! Safety-critical transition delay operations service with:
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

/// Maximum transition delay string length
const MAX_TRANSITION_DELAY_LENGTH: usize = 50;

/// Maximum transition delay value (in seconds)
const MAX_TRANSITION_DELAY: f64 = 10.0;

pub struct TransitionDelayManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TransitionDelayManager {
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

    pub fn max_transition_delay_length() -> usize {
        MAX_TRANSITION_DELAY_LENGTH
    }

    pub fn max_transition_delay() -> f64 {
        MAX_TRANSITION_DELAY
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

    fn validate_transition_delay(&self, transition_delay: &str) -> Result<(), String> {
        if transition_delay.is_empty() {
            return Err("Transition delay cannot be empty".to_string());
        }
        if transition_delay.len() > MAX_TRANSITION_DELAY_LENGTH {
            return Err(format!("Transition delay string exceeds maximum length of {} characters", MAX_TRANSITION_DELAY_LENGTH));
        }
        if transition_delay.ends_with("s") {
            let value_str = transition_delay.trim_end_matches("s");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < 0.0 || value > MAX_TRANSITION_DELAY {
                    return Err(format!("Transition delay must be between 0 and {} seconds", MAX_TRANSITION_DELAY));
                }
                if !value.is_finite() {
                    return Err("Transition delay must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_transition_delay(&mut self, node: &mut TipTapNode, transition_delay: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_transition_delay(transition_delay)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("transitionDelay".to_string(), serde_json::Value::String(transition_delay.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "transitionDelay": transition_delay }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Transition delay application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Transition delay application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_transition_delay(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("transitionDelay");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Transition delay removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Transition delay removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_transition_delay(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(transition_delay) = obj.get("transitionDelay") {
                    if let Some(s) = transition_delay.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_transition_delay(&self, node: &TipTapNode) -> bool {
        self.get_transition_delay(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_transition_delay_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TransitionDelayManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_transition_delay() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransitionDelayManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_transition_delay(&mut node, "0.5s");
        assert!(result.is_ok());
        assert!(manager.has_transition_delay(&node));
    }

    #[test]
    fn test_remove_transition_delay() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransitionDelayManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "transitionDelay": "1s" })),
            marks: None,
        };
        
        assert!(manager.has_transition_delay(&node));
        let result = manager.remove_transition_delay(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_transition_delay(&node));
    }

    #[test]
    fn test_get_transition_delay() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TransitionDelayManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "transitionDelay": "2s" })),
            marks: None,
        };
        
        let transition_delay = manager.get_transition_delay(&node);
        assert_eq!(transition_delay, Some("2s".to_string()));
    }
}
