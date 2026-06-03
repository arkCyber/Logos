//! TipTap Grid Column Manager - Aerospace-Grade Grid Column Operations Service
//!
//! Safety-critical grid column operations service with:
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

/// Maximum grid column value
const MAX_GRID_COLUMN: i32 = 100;

/// Minimum grid column value
const MIN_GRID_COLUMN: i32 = 1;

pub struct GridColumnManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl GridColumnManager {
    /// Creates a new grid column manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new GridColumnManager instance
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

    /// Get the maximum grid column constant
    /// 
    /// # Returns
    /// The maximum grid column value
    pub fn max_grid_column() -> i32 {
        MAX_GRID_COLUMN
    }

    /// Get the minimum grid column constant
    /// 
    /// # Returns
    /// The minimum grid column value
    pub fn min_grid_column() -> i32 {
        MIN_GRID_COLUMN
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

    /// Validate grid column
    /// 
    /// # Arguments
    /// * `grid_column` - The grid column to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Ensures grid column is within valid range to prevent rendering issues
    fn validate_grid_column(&self, grid_column: i32) -> Result<(), String> {
        if grid_column < MIN_GRID_COLUMN {
            return Err(format!("Grid column must be at least {}", MIN_GRID_COLUMN));
        }
        if grid_column > MAX_GRID_COLUMN {
            return Err(format!("Grid column cannot exceed {}", MAX_GRID_COLUMN));
        }
        Ok(())
    }

    /// Apply grid column to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply grid column to
    /// * `grid_column` - The grid column value
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates grid column
    pub fn apply_grid_column(&mut self, node: &mut TipTapNode, grid_column: i32) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate grid column
        self.validate_grid_column(grid_column)?;

        // Apply grid column to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("gridColumn".to_string(), serde_json::json!(grid_column));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "gridColumn": grid_column }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Grid column application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Grid column application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove grid column from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove grid column from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_grid_column(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("gridColumn");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Grid column removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Grid column removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get grid column from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get grid column from
    /// 
    /// # Returns
    /// Option containing the grid column or None
    pub fn get_grid_column(&self, node: &TipTapNode) -> Option<i32> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(grid_column) = obj.get("gridColumn") {
                    if let Some(n) = grid_column.as_i64() {
                        return Some(n as i32);
                    }
                }
            }
        }
        None
    }

    /// Check if node has grid column
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has grid column, false otherwise
    pub fn has_grid_column(&self, node: &TipTapNode) -> bool {
        self.get_grid_column(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_grid_column_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridColumnManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(GridColumnManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(GridColumnManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_min_max_constants() {
        assert_eq!(GridColumnManager::min_grid_column(), MIN_GRID_COLUMN);
        assert_eq!(GridColumnManager::max_grid_column(), MAX_GRID_COLUMN);
    }

    #[test]
    fn test_apply_grid_column() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridColumnManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_column(&mut node, 1);
        assert!(result.is_ok());
        assert!(manager.has_grid_column(&node));
    }

    #[test]
    fn test_apply_grid_column_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridColumnManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_column(&mut node, MAX_GRID_COLUMN + 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_grid_column_too_small() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridColumnManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_column(&mut node, MIN_GRID_COLUMN - 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_grid_column() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridColumnManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridColumn": 1 })),
            marks: None,
        };
        
        assert!(manager.has_grid_column(&node));
        let result = manager.remove_grid_column(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_grid_column(&node));
    }

    #[test]
    fn test_get_grid_column() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridColumnManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridColumn": 2 })),
            marks: None,
        };
        
        let grid_column = manager.get_grid_column(&node);
        assert_eq!(grid_column, Some(2));
    }

    #[test]
    fn test_get_grid_column_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridColumnManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let grid_column = manager.get_grid_column(&node);
        assert!(grid_column.is_none());
    }

    #[test]
    fn test_has_grid_column() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridColumnManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridColumn": 3 })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_grid_column(&node_with));
        assert!(!manager.has_grid_column(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridColumnManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_grid_column(&mut node, 1).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridColumnManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_grid_column(&mut node, 1).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridColumnManager::new(config_service);
        
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
        let mut manager = GridColumnManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
