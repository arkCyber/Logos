//! TipTap Flex Wrap Manager - Aerospace-Grade Flex Wrap Operations Service
//!
//! Safety-critical flex wrap operations service with:
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

/// Maximum flex wrap string length
const MAX_FLEX_WRAP_LENGTH: usize = 50;

/// Flex wrap type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl FlexWrap {
    /// Convert flex wrap to string
    pub fn as_str(&self) -> &str {
        match self {
            FlexWrap::NoWrap => "nowrap",
            FlexWrap::Wrap => "wrap",
            FlexWrap::WrapReverse => "wrap-reverse",
        }
    }

    /// Parse flex wrap from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "nowrap" => Ok(FlexWrap::NoWrap),
            "wrap" => Ok(FlexWrap::Wrap),
            "wrap-reverse" => Ok(FlexWrap::WrapReverse),
            _ => Err(format!("Invalid flex wrap: {}", s)),
        }
    }
}

pub struct FlexWrapManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FlexWrapManager {
    /// Creates a new flex wrap manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new FlexWrapManager instance
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

    /// Get the maximum flex wrap length constant
    /// 
    /// # Returns
    /// The maximum flex wrap string length
    pub fn max_flex_wrap_length() -> usize {
        MAX_FLEX_WRAP_LENGTH
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

    /// Validate flex wrap string
    /// 
    /// # Arguments
    /// * `flex_wrap` - The flex wrap string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting flex wrap string length
    fn validate_flex_wrap(&self, flex_wrap: &str) -> Result<(), String> {
        if flex_wrap.len() > MAX_FLEX_WRAP_LENGTH {
            return Err(format!("Flex wrap string exceeds maximum length of {} characters", MAX_FLEX_WRAP_LENGTH));
        }
        
        // Validate flex wrap value
        FlexWrap::from_str(flex_wrap)?;
        
        Ok(())
    }

    /// Apply flex wrap to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply flex wrap to
    /// * `flex_wrap` - The flex wrap to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates flex wrap string
    pub fn apply_flex_wrap(&mut self, node: &mut TipTapNode, flex_wrap: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate flex wrap
        self.validate_flex_wrap(flex_wrap)?;

        // Apply flex wrap to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("flexWrap".to_string(), serde_json::Value::String(flex_wrap.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "flexWrap": flex_wrap }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Flex wrap application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Flex wrap application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove flex wrap from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove flex wrap from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_flex_wrap(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("flexWrap");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Flex wrap removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Flex wrap removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get flex wrap from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get flex wrap from
    /// 
    /// # Returns
    /// Option containing the flex wrap string or None
    pub fn get_flex_wrap(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(flex_wrap) = obj.get("flexWrap") {
                    if let Some(s) = flex_wrap.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has flex wrap
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has flex wrap, false otherwise
    pub fn has_flex_wrap(&self, node: &TipTapNode) -> bool {
        self.get_flex_wrap(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_flex_wrap_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexWrapManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(FlexWrapManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(FlexWrapManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(FlexWrapManager::max_flex_wrap_length(), MAX_FLEX_WRAP_LENGTH);
    }

    #[test]
    fn test_flex_wrap_variants() {
        assert_eq!(FlexWrap::NoWrap.as_str(), "nowrap");
        assert_eq!(FlexWrap::Wrap.as_str(), "wrap");
        assert_eq!(FlexWrap::WrapReverse.as_str(), "wrap-reverse");
    }

    #[test]
    fn test_flex_wrap_from_str() {
        assert!(matches!(FlexWrap::from_str("nowrap"), Ok(FlexWrap::NoWrap)));
        assert!(matches!(FlexWrap::from_str("wrap"), Ok(FlexWrap::Wrap)));
        assert!(matches!(FlexWrap::from_str("wrap-reverse"), Ok(FlexWrap::WrapReverse)));
        assert!(FlexWrap::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_flex_wrap() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexWrapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_flex_wrap(&mut node, "wrap");
        assert!(result.is_ok());
        assert!(manager.has_flex_wrap(&node));
    }

    #[test]
    fn test_apply_flex_wrap_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexWrapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_flex_wrap(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_flex_wrap_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexWrapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_flex_wrap = "a".repeat(MAX_FLEX_WRAP_LENGTH + 1);
        let result = manager.apply_flex_wrap(&mut node, &long_flex_wrap);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_flex_wrap() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexWrapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "flexWrap": "nowrap" })),
            marks: None,
        };
        
        assert!(manager.has_flex_wrap(&node));
        let result = manager.remove_flex_wrap(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_flex_wrap(&node));
    }

    #[test]
    fn test_get_flex_wrap() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexWrapManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "flexWrap": "wrap-reverse" })),
            marks: None,
        };
        
        let flex_wrap = manager.get_flex_wrap(&node);
        assert_eq!(flex_wrap, Some("wrap-reverse".to_string()));
    }

    #[test]
    fn test_get_flex_wrap_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexWrapManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let flex_wrap = manager.get_flex_wrap(&node);
        assert!(flex_wrap.is_none());
    }

    #[test]
    fn test_has_flex_wrap() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexWrapManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "flexWrap": "wrap" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_flex_wrap(&node_with));
        assert!(!manager.has_flex_wrap(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexWrapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_flex_wrap(&mut node, "wrap").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexWrapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_flex_wrap(&mut node, "wrap").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexWrapManager::new(config_service);
        
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
        let mut manager = FlexWrapManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
