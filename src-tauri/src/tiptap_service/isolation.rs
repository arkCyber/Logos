//! TipTap Isolation Manager - Aerospace-Grade Isolation Operations Service
//!
//! Safety-critical isolation operations service with:
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

/// Maximum isolation string length
const MAX_ISOLATION_LENGTH: usize = 50;

/// Isolation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Isolation {
    Auto,
    Isolate,
}

impl Isolation {
    /// Convert isolation to string
    pub fn as_str(&self) -> &str {
        match self {
            Isolation::Auto => "auto",
            Isolation::Isolate => "isolate",
        }
    }

    /// Parse isolation from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(Isolation::Auto),
            "isolate" => Ok(Isolation::Isolate),
            _ => Err(format!("Invalid isolation: {}", s)),
        }
    }
}

pub struct IsolationManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl IsolationManager {
    /// Creates a new isolation manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new IsolationManager instance
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

    /// Get the maximum isolation length constant
    /// 
    /// # Returns
    /// The maximum isolation string length
    pub fn max_isolation_length() -> usize {
        MAX_ISOLATION_LENGTH
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

    /// Validate isolation string
    /// 
    /// # Arguments
    /// * `isolation` - The isolation string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting isolation string length
    fn validate_isolation(&self, isolation: &str) -> Result<(), String> {
        if isolation.len() > MAX_ISOLATION_LENGTH {
            return Err(format!("Isolation string exceeds maximum length of {} characters", MAX_ISOLATION_LENGTH));
        }
        
        // Validate isolation value
        Isolation::from_str(isolation)?;
        
        Ok(())
    }

    /// Apply isolation to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply isolation to
    /// * `isolation` - The isolation to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates isolation string
    pub fn apply_isolation(&mut self, node: &mut TipTapNode, isolation: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate isolation
        self.validate_isolation(isolation)?;

        // Apply isolation to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("isolation".to_string(), serde_json::Value::String(isolation.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "isolation": isolation }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Isolation application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Isolation application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove isolation from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove isolation from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_isolation(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("isolation");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Isolation removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Isolation removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get isolation from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get isolation from
    /// 
    /// # Returns
    /// Option containing the isolation string or None
    pub fn get_isolation(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(isolation) = obj.get("isolation") {
                    if let Some(s) = isolation.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has isolation
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has isolation, false otherwise
    pub fn has_isolation(&self, node: &TipTapNode) -> bool {
        self.get_isolation(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_isolation_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = IsolationManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(IsolationManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(IsolationManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(IsolationManager::max_isolation_length(), MAX_ISOLATION_LENGTH);
    }

    #[test]
    fn test_isolation_variants() {
        assert_eq!(Isolation::Auto.as_str(), "auto");
        assert_eq!(Isolation::Isolate.as_str(), "isolate");
    }

    #[test]
    fn test_isolation_from_str() {
        assert!(matches!(Isolation::from_str("auto"), Ok(Isolation::Auto)));
        assert!(matches!(Isolation::from_str("isolate"), Ok(Isolation::Isolate)));
        assert!(Isolation::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_isolation() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = IsolationManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_isolation(&mut node, "isolate");
        assert!(result.is_ok());
        assert!(manager.has_isolation(&node));
    }

    #[test]
    fn test_apply_isolation_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = IsolationManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_isolation(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_isolation_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = IsolationManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_isolation = "a".repeat(MAX_ISOLATION_LENGTH + 1);
        let result = manager.apply_isolation(&mut node, &long_isolation);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_isolation() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = IsolationManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "isolation": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_isolation(&node));
        let result = manager.remove_isolation(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_isolation(&node));
    }

    #[test]
    fn test_get_isolation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = IsolationManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "isolation": "isolate" })),
            marks: None,
        };
        
        let isolation = manager.get_isolation(&node);
        assert_eq!(isolation, Some("isolate".to_string()));
    }

    #[test]
    fn test_get_isolation_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = IsolationManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let isolation = manager.get_isolation(&node);
        assert!(isolation.is_none());
    }

    #[test]
    fn test_has_isolation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = IsolationManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "isolation": "auto" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_isolation(&node_with));
        assert!(!manager.has_isolation(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = IsolationManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_isolation(&mut node, "isolate").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = IsolationManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_isolation(&mut node, "isolate").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = IsolationManager::new(config_service);
        
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
        let mut manager = IsolationManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
