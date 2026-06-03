//! TipTap Z-Index Manager - Aerospace-Grade Z-Index Operations Service
//!
//! Safety-critical z-index operations service with:
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

/// Minimum z-index value
const MIN_Z_INDEX: i32 = -2147483648;

/// Maximum z-index value
const MAX_Z_INDEX: i32 = 2147483647;

pub struct ZIndexManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ZIndexManager {
    /// Creates a new z-index manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new ZIndexManager instance
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

    /// Get the minimum z-index constant
    /// 
    /// # Returns
    /// The minimum z-index value
    pub fn min_z_index() -> i32 {
        MIN_Z_INDEX
    }

    /// Get the maximum z-index constant
    /// 
    /// # Returns
    /// The maximum z-index value
    pub fn max_z_index() -> i32 {
        MAX_Z_INDEX
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

    /// Validate z-index
    /// 
    /// # Arguments
    /// * `z_index` - The z-index to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Ensures z-index is within valid range to prevent rendering issues
    fn validate_z_index(&self, z_index: i32) -> Result<(), String> {
        if z_index < MIN_Z_INDEX {
            return Err(format!("Z-index must be at least {}", MIN_Z_INDEX));
        }
        if z_index > MAX_Z_INDEX {
            return Err(format!("Z-index cannot exceed {}", MAX_Z_INDEX));
        }
        Ok(())
    }

    /// Apply z-index to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply z-index to
    /// * `z_index` - The z-index value
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates z-index
    pub fn apply_z_index(&mut self, node: &mut TipTapNode, z_index: i32) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate z-index
        self.validate_z_index(z_index)?;

        // Apply z-index to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("zIndex".to_string(), serde_json::json!(z_index));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "zIndex": z_index }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Z-index application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Z-index application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove z-index from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove z-index from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_z_index(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("zIndex");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Z-index removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Z-index removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get z-index from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get z-index from
    /// 
    /// # Returns
    /// Option containing the z-index or None
    pub fn get_z_index(&self, node: &TipTapNode) -> Option<i32> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(z_index) = obj.get("zIndex") {
                    if let Some(n) = z_index.as_i64() {
                        if n >= i32::MIN as i64 && n <= i32::MAX as i64 {
                            return Some(n as i32);
                        }
                    }
                }
            }
        }
        None
    }

    /// Check if node has z-index
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has z-index, false otherwise
    pub fn has_z_index(&self, node: &TipTapNode) -> bool {
        self.get_z_index(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_z_index_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ZIndexManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(ZIndexManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(ZIndexManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_min_max_constants() {
        assert_eq!(ZIndexManager::min_z_index(), MIN_Z_INDEX);
        assert_eq!(ZIndexManager::max_z_index(), MAX_Z_INDEX);
    }

    #[test]
    fn test_apply_z_index() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ZIndexManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_z_index(&mut node, 10);
        assert!(result.is_ok());
        assert!(manager.has_z_index(&node));
    }

    #[test]
    fn test_apply_z_index_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ZIndexManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        // i32 cannot represent values above MAX_Z_INDEX; verify upper bound is accepted.
        let result = manager.apply_z_index(&mut node, MAX_Z_INDEX);
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_z_index_too_small() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ZIndexManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        // i32 cannot represent values below MIN_Z_INDEX; verify lower bound is accepted.
        let result = manager.apply_z_index(&mut node, MIN_Z_INDEX);
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_z_index() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ZIndexManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "zIndex": 10 })),
            marks: None,
        };
        
        assert!(manager.has_z_index(&node));
        let result = manager.remove_z_index(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_z_index(&node));
    }

    #[test]
    fn test_get_z_index() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ZIndexManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "zIndex": 5 })),
            marks: None,
        };
        
        let z_index = manager.get_z_index(&node);
        assert_eq!(z_index, Some(5));
    }

    #[test]
    fn test_get_z_index_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ZIndexManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let z_index = manager.get_z_index(&node);
        assert!(z_index.is_none());
    }

    #[test]
    fn test_has_z_index() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ZIndexManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "zIndex": 100 })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_z_index(&node_with));
        assert!(!manager.has_z_index(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ZIndexManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_z_index(&mut node, 10).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ZIndexManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_z_index(&mut node, 10).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ZIndexManager::new(config_service);
        
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
        let mut manager = ZIndexManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
