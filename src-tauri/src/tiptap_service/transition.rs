//! TipTap Transition Manager - Aerospace-Grade Transition Operations Service
//!
//! Safety-critical transition operations service with:
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

/// Maximum transition string length
const MAX_TRANSITION_LENGTH: usize = 200;

pub struct TransitionManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TransitionManager {
    /// Creates a new transition manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new TransitionManager instance
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
        }
    }

    /// Get the performance warning threshold
    /// 
    /// # Returns
    /// The performance warning threshold in milliseconds
    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    /// Get the performance critical threshold
    /// 
    /// # Returns
    /// The performance critical threshold in milliseconds
    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    /// Get the maximum transition length constant
    /// 
    /// # Returns
    /// The maximum transition string length
    pub fn max_transition_length() -> usize {
        MAX_TRANSITION_LENGTH
    }

    /// Record error context
    fn record_error(&mut self, code: &str, message: &str, source: &str) {
        self.last_error = Some(ErrorContext::new(
            ErrorSeverity::Error,
            code,
            message,
            source,
        ));
    }

    /// Get last error
    pub fn get_last_error(&self) -> Option<&ErrorContext> {
        self.last_error.as_ref()
    }

    /// Get operation count
    pub fn get_operation_count(&self) -> u64 {
        self.operation_count
    }

    /// Reset error state
    pub fn reset_error_state(&mut self) {
        self.last_error = None;
    }

    /// Reset operation count
    pub fn reset_operation_count(&mut self) {
        self.operation_count = 0;
    }

    /// Validate transition string
    /// 
    /// # Arguments
    /// * `transition` - The transition string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting transition string length
    fn validate_transition(&self, transition: &str) -> Result<(), String> {
        if transition.is_empty() {
            return Err("Transition cannot be empty".to_string());
        }
        if transition.len() > MAX_TRANSITION_LENGTH {
            return Err(format!("Transition string exceeds maximum length of {} characters", MAX_TRANSITION_LENGTH));
        }
        // Basic validation for common transition properties
        let valid_patterns = ["all", "none", "ease", "linear", "ease-in", "ease-out", "ease-in-out", "s", "ms"];
        if !valid_patterns.iter().any(|pattern| transition.contains(pattern)) {
            // Allow custom values but validate basic structure
            if transition.contains('(') && !transition.contains(')') {
                return Err("Invalid transition: unmatched parentheses".to_string());
            }
        }
        Ok(())
    }

    /// Apply transition to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply transition to
    /// * `transition` - The transition to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates transition string
    pub fn apply_transition(&mut self, node: &mut TipTapNode, transition: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate transition
        self.validate_transition(transition)?;

        // Apply transition to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("transition".to_string(), serde_json::Value::String(transition.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "transition": transition }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Transition application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Transition application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove transition from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove transition from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_transition(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("transition");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Transition removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Transition removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get transition from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get transition from
    /// 
    /// # Returns
    /// Option containing the transition string or None
    pub fn get_transition(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(transition) = obj.get("transition") {
                    if let Some(s) = transition.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has transition
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has transition, false otherwise
    pub fn has_transition(&self, node: &TipTapNode) -> bool {
        self.get_transition(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_transition_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TransitionManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(TransitionManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(TransitionManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(TransitionManager::max_transition_length(), MAX_TRANSITION_LENGTH);
    }

    #[test]
    fn test_apply_transition() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransitionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_transition(&mut node, "all 0.3s ease");
        assert!(result.is_ok());
        assert!(manager.has_transition(&node));
    }

    #[test]
    fn test_apply_transition_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransitionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_transition(&mut node, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_transition_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransitionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_transition = "a".repeat(MAX_TRANSITION_LENGTH + 1);
        let result = manager.apply_transition(&mut node, &long_transition);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_transition_invalid_parentheses() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransitionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_transition(&mut node, "cubic-bezier(0.25, 0.1, 0.25, 1.0");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_transition() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransitionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "transition": "opacity 0.5s linear" })),
            marks: None,
        };
        
        assert!(manager.has_transition(&node));
        let result = manager.remove_transition(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_transition(&node));
    }

    #[test]
    fn test_get_transition() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TransitionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "transition": "transform 0.3s ease-in-out" })),
            marks: None,
        };
        
        let transition = manager.get_transition(&node);
        assert_eq!(transition, Some("transform 0.3s ease-in-out".to_string()));
    }

    #[test]
    fn test_get_transition_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TransitionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let transition = manager.get_transition(&node);
        assert!(transition.is_none());
    }

    #[test]
    fn test_has_transition() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TransitionManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "transition": "all 0.2s ease" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_transition(&node_with));
        assert!(!manager.has_transition(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransitionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_transition(&mut node, "all 0.3s ease").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransitionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_transition(&mut node, "all 0.3s ease").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransitionManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        
        let last_error = manager.get_last_error();
        assert!(last_error.is_some());
        let error = last_error.unwrap();
        assert_eq!(error.code, "TEST_ERROR");
        assert_eq!(error.message, "Test error message");
        assert_eq!(error.source, "test_source");
    }

    #[test]
    fn test_error_state_reset() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransitionManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
