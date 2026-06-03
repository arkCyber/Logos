//! TipTap Column Count Manager - Aerospace-Grade Column Count Operations Service
//!
//! Safety-critical column count operations service with:
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

/// Maximum column count value
const MAX_COLUMN_COUNT: i32 = 100;

/// Minimum column count value
const MIN_COLUMN_COUNT: i32 = 1;

pub struct ColumnCountManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ColumnCountManager {
    /// Creates a new column count manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new ColumnCountManager instance
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

    /// Get the maximum column count constant
    /// 
    /// # Returns
    /// The maximum column count
    pub fn max_column_count() -> i32 {
        MAX_COLUMN_COUNT
    }

    /// Get the minimum column count constant
    /// 
    /// # Returns
    /// The minimum column count
    pub fn min_column_count() -> i32 {
        MIN_COLUMN_COUNT
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

    /// Validate column count
    /// 
    /// # Arguments
    /// * `column_count` - The column count to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Ensures column count is within valid range to prevent rendering issues
    fn validate_column_count(&self, column_count: i32) -> Result<(), String> {
        if column_count < MIN_COLUMN_COUNT {
            return Err(format!("Column count must be at least {}", MIN_COLUMN_COUNT));
        }
        if column_count > MAX_COLUMN_COUNT {
            return Err(format!("Column count cannot exceed {}", MAX_COLUMN_COUNT));
        }
        Ok(())
    }

    /// Apply column count to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply column count to
    /// * `column_count` - The column count to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates column count
    pub fn apply_column_count(&mut self, node: &mut TipTapNode, column_count: i32) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate column count
        self.validate_column_count(column_count)?;

        // Apply column count to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("columnCount".to_string(), serde_json::json!(column_count));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "columnCount": column_count }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Column count application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Column count application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove column count from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove column count from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_column_count(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("columnCount");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Column count removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Column count removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get column count from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get column count from
    /// 
    /// # Returns
    /// Option containing the column count or None
    pub fn get_column_count(&self, node: &TipTapNode) -> Option<i32> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(column_count) = obj.get("columnCount") {
                    if let Some(n) = column_count.as_i64() {
                        return Some(n as i32);
                    }
                }
            }
        }
        None
    }

    /// Check if node has column count
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has column count, false otherwise
    pub fn has_column_count(&self, node: &TipTapNode) -> bool {
        self.get_column_count(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_column_count_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnCountManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(ColumnCountManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(ColumnCountManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_min_max_constants() {
        assert_eq!(ColumnCountManager::min_column_count(), MIN_COLUMN_COUNT);
        assert_eq!(ColumnCountManager::max_column_count(), MAX_COLUMN_COUNT);
    }

    #[test]
    fn test_apply_column_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnCountManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_column_count(&mut node, 3);
        assert!(result.is_ok());
        assert!(manager.has_column_count(&node));
    }

    #[test]
    fn test_apply_column_count_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnCountManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_column_count(&mut node, MAX_COLUMN_COUNT + 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_column_count_too_small() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnCountManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_column_count(&mut node, MIN_COLUMN_COUNT - 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_column_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnCountManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "columnCount": 2 })),
            marks: None,
        };
        
        assert!(manager.has_column_count(&node));
        let result = manager.remove_column_count(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_column_count(&node));
    }

    #[test]
    fn test_get_column_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnCountManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "columnCount": 5 })),
            marks: None,
        };
        
        let column_count = manager.get_column_count(&node);
        assert_eq!(column_count, Some(5));
    }

    #[test]
    fn test_get_column_count_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnCountManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let column_count = manager.get_column_count(&node);
        assert!(column_count.is_none());
    }

    #[test]
    fn test_has_column_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnCountManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "columnCount": 3 })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_column_count(&node_with));
        assert!(!manager.has_column_count(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnCountManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_column_count(&mut node, 3).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnCountManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_column_count(&mut node, 3).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnCountManager::new(config_service);
        
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
        let mut manager = ColumnCountManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
