//! TipTap Auto Wrap Manager - Aerospace-Grade Auto Wrap Operations Service
//!
//! Safety-critical auto wrap operations service with:
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

/// Maximum auto wrap string length
const MAX_AUTO_WRAP_LENGTH: usize = 50;

/// Auto wrap type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoWrap {
    Normal,
    BreakWord,
    Balance,
}

impl AutoWrap {
    /// Convert auto wrap to string
    pub fn as_str(&self) -> &str {
        match self {
            AutoWrap::Normal => "normal",
            AutoWrap::BreakWord => "break-word",
            AutoWrap::Balance => "balance",
        }
    }

    /// Parse auto wrap from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(AutoWrap::Normal),
            "break-word" => Ok(AutoWrap::BreakWord),
            "balance" => Ok(AutoWrap::Balance),
            _ => Err(format!("Invalid auto wrap: {}", s)),
        }
    }
}

pub struct AutoWrapManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl AutoWrapManager {
    /// Creates a new auto wrap manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new AutoWrapManager instance
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

    /// Get the maximum auto wrap length constant
    /// 
    /// # Returns
    /// The maximum auto wrap string length
    pub fn max_auto_wrap_length() -> usize {
        MAX_AUTO_WRAP_LENGTH
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

    /// Validate auto wrap string
    /// 
    /// # Arguments
    /// * `auto_wrap` - The auto wrap string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting auto wrap string length
    fn validate_auto_wrap(&self, auto_wrap: &str) -> Result<(), String> {
        if auto_wrap.len() > MAX_AUTO_WRAP_LENGTH {
            return Err(format!("Auto wrap string exceeds maximum length of {} characters", MAX_AUTO_WRAP_LENGTH));
        }
        
        // Validate auto wrap value
        AutoWrap::from_str(auto_wrap)?;
        
        Ok(())
    }

    /// Apply auto wrap to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply auto wrap to
    /// * `auto_wrap` - The auto wrap to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates auto wrap string
    pub fn apply_auto_wrap(&mut self, node: &mut TipTapNode, auto_wrap: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate auto wrap
        self.validate_auto_wrap(auto_wrap)?;

        // Apply auto wrap to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("autoWrap".to_string(), serde_json::Value::String(auto_wrap.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "autoWrap": auto_wrap }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Auto wrap application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Auto wrap application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove auto wrap from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove auto wrap from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_auto_wrap(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("autoWrap");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Auto wrap removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Auto wrap removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get auto wrap from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get auto wrap from
    /// 
    /// # Returns
    /// Option containing the auto wrap string or None
    pub fn get_auto_wrap(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(auto_wrap) = obj.get("autoWrap") {
                    if let Some(s) = auto_wrap.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has auto wrap
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has auto wrap, false otherwise
    pub fn has_auto_wrap(&self, node: &TipTapNode) -> bool {
        self.get_auto_wrap(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_auto_wrap_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AutoWrapManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(AutoWrapManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(AutoWrapManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(AutoWrapManager::max_auto_wrap_length(), MAX_AUTO_WRAP_LENGTH);
    }

    #[test]
    fn test_auto_wrap_variants() {
        assert_eq!(AutoWrap::Normal.as_str(), "normal");
        assert_eq!(AutoWrap::BreakWord.as_str(), "break-word");
        assert_eq!(AutoWrap::Balance.as_str(), "balance");
    }

    #[test]
    fn test_auto_wrap_from_str() {
        assert!(matches!(AutoWrap::from_str("normal"), Ok(AutoWrap::Normal)));
        assert!(matches!(AutoWrap::from_str("break-word"), Ok(AutoWrap::BreakWord)));
        assert!(matches!(AutoWrap::from_str("balance"), Ok(AutoWrap::Balance)));
        assert!(AutoWrap::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_auto_wrap() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AutoWrapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_auto_wrap(&mut node, "break-word");
        assert!(result.is_ok());
        assert!(manager.has_auto_wrap(&node));
    }

    #[test]
    fn test_apply_auto_wrap_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AutoWrapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_auto_wrap(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_auto_wrap_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AutoWrapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_auto_wrap = "a".repeat(MAX_AUTO_WRAP_LENGTH + 1);
        let result = manager.apply_auto_wrap(&mut node, &long_auto_wrap);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_auto_wrap() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AutoWrapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "autoWrap": "break-word" })),
            marks: None,
        };
        
        assert!(manager.has_auto_wrap(&node));
        let result = manager.remove_auto_wrap(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_auto_wrap(&node));
    }

    #[test]
    fn test_get_auto_wrap() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AutoWrapManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "autoWrap": "balance" })),
            marks: None,
        };
        
        let auto_wrap = manager.get_auto_wrap(&node);
        assert_eq!(auto_wrap, Some("balance".to_string()));
    }

    #[test]
    fn test_get_auto_wrap_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AutoWrapManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let auto_wrap = manager.get_auto_wrap(&node);
        assert!(auto_wrap.is_none());
    }

    #[test]
    fn test_has_auto_wrap() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AutoWrapManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "autoWrap": "normal" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_auto_wrap(&node_with));
        assert!(!manager.has_auto_wrap(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AutoWrapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_auto_wrap(&mut node, "break-word").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AutoWrapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_auto_wrap(&mut node, "break-word").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AutoWrapManager::new(config_service);
        
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
        let mut manager = AutoWrapManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
