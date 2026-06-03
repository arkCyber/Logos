//! TipTap Scroll Behavior Manager - Aerospace-Grade Scroll Behavior Operations Service
//!
//! Safety-critical scroll behavior operations service with:
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

/// Maximum scroll behavior string length
const MAX_SCROLL_BEHAVIOR_LENGTH: usize = 50;

/// Scroll behavior type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollBehavior {
    Auto,
    Smooth,
    Instant,
}

impl ScrollBehavior {
    /// Convert scroll behavior to string
    pub fn as_str(&self) -> &str {
        match self {
            ScrollBehavior::Auto => "auto",
            ScrollBehavior::Smooth => "smooth",
            ScrollBehavior::Instant => "instant",
        }
    }

    /// Parse scroll behavior from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(ScrollBehavior::Auto),
            "smooth" => Ok(ScrollBehavior::Smooth),
            "instant" => Ok(ScrollBehavior::Instant),
            _ => Err(format!("Invalid scroll behavior: {}", s)),
        }
    }
}

pub struct ScrollBehaviorManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ScrollBehaviorManager {
    /// Creates a new scroll behavior manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new ScrollBehaviorManager instance
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

    /// Get the maximum scroll behavior length constant
    /// 
    /// # Returns
    /// The maximum scroll behavior string length
    pub fn max_scroll_behavior_length() -> usize {
        MAX_SCROLL_BEHAVIOR_LENGTH
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

    /// Validate scroll behavior string
    /// 
    /// # Arguments
    /// * `scroll_behavior` - The scroll behavior string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting scroll behavior string length
    fn validate_scroll_behavior(&self, scroll_behavior: &str) -> Result<(), String> {
        if scroll_behavior.len() > MAX_SCROLL_BEHAVIOR_LENGTH {
            return Err(format!("Scroll behavior string exceeds maximum length of {} characters", MAX_SCROLL_BEHAVIOR_LENGTH));
        }
        
        // Validate scroll behavior value
        ScrollBehavior::from_str(scroll_behavior)?;
        
        Ok(())
    }

    /// Apply scroll behavior to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply scroll behavior to
    /// * `scroll_behavior` - The scroll behavior to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates scroll behavior string
    pub fn apply_scroll_behavior(&mut self, node: &mut TipTapNode, scroll_behavior: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate scroll behavior
        self.validate_scroll_behavior(scroll_behavior)?;

        // Apply scroll behavior to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("scrollBehavior".to_string(), serde_json::Value::String(scroll_behavior.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "scrollBehavior": scroll_behavior }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Scroll behavior application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Scroll behavior application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove scroll behavior from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove scroll behavior from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_scroll_behavior(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("scrollBehavior");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Scroll behavior removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Scroll behavior removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get scroll behavior from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get scroll behavior from
    /// 
    /// # Returns
    /// Option containing the scroll behavior string or None
    pub fn get_scroll_behavior(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(scroll_behavior) = obj.get("scrollBehavior") {
                    if let Some(s) = scroll_behavior.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has scroll behavior
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has scroll behavior, false otherwise
    pub fn has_scroll_behavior(&self, node: &TipTapNode) -> bool {
        self.get_scroll_behavior(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_scroll_behavior_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ScrollBehaviorManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(ScrollBehaviorManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(ScrollBehaviorManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(ScrollBehaviorManager::max_scroll_behavior_length(), MAX_SCROLL_BEHAVIOR_LENGTH);
    }

    #[test]
    fn test_scroll_behavior_variants() {
        assert_eq!(ScrollBehavior::Auto.as_str(), "auto");
        assert_eq!(ScrollBehavior::Smooth.as_str(), "smooth");
        assert_eq!(ScrollBehavior::Instant.as_str(), "instant");
    }

    #[test]
    fn test_scroll_behavior_from_str() {
        assert!(matches!(ScrollBehavior::from_str("auto"), Ok(ScrollBehavior::Auto)));
        assert!(matches!(ScrollBehavior::from_str("smooth"), Ok(ScrollBehavior::Smooth)));
        assert!(matches!(ScrollBehavior::from_str("instant"), Ok(ScrollBehavior::Instant)));
        assert!(ScrollBehavior::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_scroll_behavior() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScrollBehaviorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_scroll_behavior(&mut node, "smooth");
        assert!(result.is_ok());
        assert!(manager.has_scroll_behavior(&node));
    }

    #[test]
    fn test_apply_scroll_behavior_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScrollBehaviorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_scroll_behavior(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_scroll_behavior_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScrollBehaviorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_scroll_behavior = "a".repeat(MAX_SCROLL_BEHAVIOR_LENGTH + 1);
        let result = manager.apply_scroll_behavior(&mut node, &long_scroll_behavior);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_scroll_behavior() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScrollBehaviorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "scrollBehavior": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_scroll_behavior(&node));
        let result = manager.remove_scroll_behavior(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_scroll_behavior(&node));
    }

    #[test]
    fn test_get_scroll_behavior() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ScrollBehaviorManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "scrollBehavior": "instant" })),
            marks: None,
        };
        
        let scroll_behavior = manager.get_scroll_behavior(&node);
        assert_eq!(scroll_behavior, Some("instant".to_string()));
    }

    #[test]
    fn test_get_scroll_behavior_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ScrollBehaviorManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let scroll_behavior = manager.get_scroll_behavior(&node);
        assert!(scroll_behavior.is_none());
    }

    #[test]
    fn test_has_scroll_behavior() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ScrollBehaviorManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "scrollBehavior": "smooth" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_scroll_behavior(&node_with));
        assert!(!manager.has_scroll_behavior(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScrollBehaviorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_scroll_behavior(&mut node, "smooth").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScrollBehaviorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_scroll_behavior(&mut node, "smooth").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScrollBehaviorManager::new(config_service);
        
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
        let mut manager = ScrollBehaviorManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
