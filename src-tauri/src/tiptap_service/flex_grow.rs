//! TipTap Flex Grow Manager - Aerospace-Grade Flex Grow Operations Service
//!
//! Safety-critical flex grow operations service with:
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

/// Minimum flex grow value
const MIN_FLEX_GROW: f64 = 0.0;

/// Maximum flex grow value
const MAX_FLEX_GROW: f64 = 10.0;

pub struct FlexGrowManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FlexGrowManager {
    /// Creates a new flex grow manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new FlexGrowManager instance
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

    /// Get the minimum flex grow constant
    /// 
    /// # Returns
    /// The minimum flex grow value
    pub fn min_flex_grow() -> f64 {
        MIN_FLEX_GROW
    }

    /// Get the maximum flex grow constant
    /// 
    /// # Returns
    /// The maximum flex grow value
    pub fn max_flex_grow() -> f64 {
        MAX_FLEX_GROW
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

    /// Validate flex grow
    /// 
    /// # Arguments
    /// * `flex_grow` - The flex grow to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Ensures flex grow is within valid range to prevent rendering issues
    fn validate_flex_grow(&self, flex_grow: f64) -> Result<(), String> {
        if flex_grow < MIN_FLEX_GROW {
            return Err(format!("Flex grow must be at least {}", MIN_FLEX_GROW));
        }
        if flex_grow > MAX_FLEX_GROW {
            return Err(format!("Flex grow cannot exceed {}", MAX_FLEX_GROW));
        }
        if !flex_grow.is_finite() {
            return Err("Flex grow must be a finite number".to_string());
        }
        Ok(())
    }

    /// Apply flex grow to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply flex grow to
    /// * `flex_grow` - The flex grow value
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates flex grow
    pub fn apply_flex_grow(&mut self, node: &mut TipTapNode, flex_grow: f64) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate flex grow
        self.validate_flex_grow(flex_grow)?;

        // Apply flex grow to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("flexGrow".to_string(), serde_json::json!(flex_grow));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "flexGrow": flex_grow }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Flex grow application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Flex grow application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove flex grow from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove flex grow from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_flex_grow(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("flexGrow");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Flex grow removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Flex grow removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get flex grow from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get flex grow from
    /// 
    /// # Returns
    /// Option containing the flex grow or None
    pub fn get_flex_grow(&self, node: &TipTapNode) -> Option<f64> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(flex_grow) = obj.get("flexGrow") {
                    if let Some(n) = flex_grow.as_f64() {
                        return Some(n);
                    }
                }
            }
        }
        None
    }

    /// Check if node has flex grow
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has flex grow, false otherwise
    pub fn has_flex_grow(&self, node: &TipTapNode) -> bool {
        self.get_flex_grow(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_flex_grow_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexGrowManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(FlexGrowManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(FlexGrowManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_min_max_constants() {
        assert_eq!(FlexGrowManager::min_flex_grow(), MIN_FLEX_GROW);
        assert_eq!(FlexGrowManager::max_flex_grow(), MAX_FLEX_GROW);
    }

    #[test]
    fn test_apply_flex_grow() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexGrowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_flex_grow(&mut node, 1.0);
        assert!(result.is_ok());
        assert!(manager.has_flex_grow(&node));
    }

    #[test]
    fn test_apply_flex_grow_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexGrowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_flex_grow(&mut node, MAX_FLEX_GROW + 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_flex_grow_too_small() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexGrowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_flex_grow(&mut node, MIN_FLEX_GROW - 0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_flex_grow_infinite() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexGrowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_flex_grow(&mut node, f64::INFINITY);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_flex_grow() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexGrowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "flexGrow": 1.0 })),
            marks: None,
        };
        
        assert!(manager.has_flex_grow(&node));
        let result = manager.remove_flex_grow(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_flex_grow(&node));
    }

    #[test]
    fn test_get_flex_grow() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexGrowManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "flexGrow": 2.0 })),
            marks: None,
        };
        
        let flex_grow = manager.get_flex_grow(&node);
        assert_eq!(flex_grow, Some(2.0));
    }

    #[test]
    fn test_get_flex_grow_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexGrowManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let flex_grow = manager.get_flex_grow(&node);
        assert!(flex_grow.is_none());
    }

    #[test]
    fn test_has_flex_grow() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexGrowManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "flexGrow": 1.5 })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_flex_grow(&node_with));
        assert!(!manager.has_flex_grow(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexGrowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_flex_grow(&mut node, 1.0).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexGrowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_flex_grow(&mut node, 1.0).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexGrowManager::new(config_service);
        
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
        let mut manager = FlexGrowManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
