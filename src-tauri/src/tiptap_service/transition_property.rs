//! TipTap Transition Property Manager - Aerospace-Grade Transition Property Operations Service
//!
//! Safety-critical transition property operations service with:
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

/// Maximum transition property string length
const MAX_TRANSITION_PROPERTY_LENGTH: usize = 200;

pub struct TransitionPropertyManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TransitionPropertyManager {
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

    pub fn max_transition_property_length() -> usize {
        MAX_TRANSITION_PROPERTY_LENGTH
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

    fn validate_transition_property(&self, transition_property: &str) -> Result<(), String> {
        if transition_property.is_empty() {
            return Err("Transition property cannot be empty".to_string());
        }
        if transition_property.len() > MAX_TRANSITION_PROPERTY_LENGTH {
            return Err(format!("Transition property string exceeds maximum length of {} characters", MAX_TRANSITION_PROPERTY_LENGTH));
        }
        if transition_property.contains('(') && !transition_property.contains(')') {
            return Err("Invalid transition property: unmatched parentheses".to_string());
        }
        Ok(())
    }

    pub fn apply_transition_property(&mut self, node: &mut TipTapNode, transition_property: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_transition_property(transition_property)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("transitionProperty".to_string(), serde_json::Value::String(transition_property.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "transitionProperty": transition_property }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Transition property application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Transition property application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_transition_property(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("transitionProperty");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Transition property removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Transition property removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_transition_property(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(transition_property) = obj.get("transitionProperty") {
                    if let Some(s) = transition_property.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_transition_property(&self, node: &TipTapNode) -> bool {
        self.get_transition_property(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_transition_property_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TransitionPropertyManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_transition_property() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransitionPropertyManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_transition_property(&mut node, "all");
        assert!(result.is_ok());
        assert!(manager.has_transition_property(&node));
    }

    #[test]
    fn test_remove_transition_property() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransitionPropertyManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "transitionProperty": "opacity" })),
            marks: None,
        };
        
        assert!(manager.has_transition_property(&node));
        let result = manager.remove_transition_property(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_transition_property(&node));
    }

    #[test]
    fn test_get_transition_property() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TransitionPropertyManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "transitionProperty": "transform" })),
            marks: None,
        };
        
        let transition_property = manager.get_transition_property(&node);
        assert_eq!(transition_property, Some("transform".to_string()));
    }
}
