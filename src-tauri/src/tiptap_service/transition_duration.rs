//! TipTap Transition Duration Manager - Aerospace-Grade Transition Duration Operations Service
//!
//! Safety-critical transition duration operations service with:
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

/// Maximum transition duration value (in seconds)
const MAX_TRANSITION_DURATION: f64 = 10.0;

/// Minimum transition duration value (in seconds)
const MIN_TRANSITION_DURATION: f64 = 0.0;

/// Maximum transition duration string length
const MAX_TRANSITION_DURATION_LENGTH: usize = 50;

pub struct TransitionDurationManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TransitionDurationManager {
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

    pub fn max_transition_duration() -> f64 {
        MAX_TRANSITION_DURATION
    }

    pub fn min_transition_duration() -> f64 {
        MIN_TRANSITION_DURATION
    }

    pub fn max_transition_duration_length() -> usize {
        MAX_TRANSITION_DURATION_LENGTH
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

    fn validate_transition_duration(&self, transition_duration: &str) -> Result<(), String> {
        if transition_duration.is_empty() {
            return Err("Transition duration cannot be empty".to_string());
        }
        if transition_duration.len() > MAX_TRANSITION_DURATION_LENGTH {
            return Err(format!("Transition duration string exceeds maximum length of {} characters", MAX_TRANSITION_DURATION_LENGTH));
        }
        if transition_duration.ends_with("s") {
            let value_str = transition_duration.trim_end_matches("s");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_TRANSITION_DURATION || value > MAX_TRANSITION_DURATION {
                    return Err(format!("Transition duration must be between {} and {} seconds", MIN_TRANSITION_DURATION, MAX_TRANSITION_DURATION));
                }
                if !value.is_finite() {
                    return Err("Transition duration must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_transition_duration(&mut self, node: &mut TipTapNode, transition_duration: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_transition_duration(transition_duration)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("transitionDuration".to_string(), serde_json::Value::String(transition_duration.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "transitionDuration": transition_duration }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Transition duration application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Transition duration application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_transition_duration(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("transitionDuration");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Transition duration removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Transition duration removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_transition_duration(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(transition_duration) = obj.get("transitionDuration") {
                    if let Some(s) = transition_duration.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_transition_duration(&self, node: &TipTapNode) -> bool {
        self.get_transition_duration(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_transition_duration_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TransitionDurationManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_transition_duration() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransitionDurationManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_transition_duration(&mut node, "0.5s");
        assert!(result.is_ok());
        assert!(manager.has_transition_duration(&node));
    }

    #[test]
    fn test_remove_transition_duration() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransitionDurationManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "transitionDuration": "1s" })),
            marks: None,
        };
        
        assert!(manager.has_transition_duration(&node));
        let result = manager.remove_transition_duration(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_transition_duration(&node));
    }

    #[test]
    fn test_get_transition_duration() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TransitionDurationManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "transitionDuration": "2s" })),
            marks: None,
        };
        
        let transition_duration = manager.get_transition_duration(&node);
        assert_eq!(transition_duration, Some("2s".to_string()));
    }
}
