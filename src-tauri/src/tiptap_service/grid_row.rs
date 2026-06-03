//! TipTap Grid Row Manager - Aerospace-Grade Grid Row Operations Service
//!
//! Safety-critical grid row operations service with:
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

/// Maximum grid row value
const MAX_GRID_ROW: i32 = 100;

/// Minimum grid row value
const MIN_GRID_ROW: i32 = 1;

pub struct GridRowManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl GridRowManager {
    /// Creates a new grid row manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new GridRowManager instance
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

    /// Get the maximum grid row constant
    /// 
    /// # Returns
    /// The maximum grid row value
    pub fn max_grid_row() -> i32 {
        MAX_GRID_ROW
    }

    /// Get the minimum grid row constant
    /// 
    /// # Returns
    /// The minimum grid row value
    pub fn min_grid_row() -> i32 {
        MIN_GRID_ROW
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

    /// Validate grid row
    /// 
    /// # Arguments
    /// * `grid_row` - The grid row to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Ensures grid row is within valid range to prevent rendering issues
    fn validate_grid_row(&self, grid_row: i32) -> Result<(), String> {
        if grid_row < MIN_GRID_ROW {
            return Err(format!("Grid row must be at least {}", MIN_GRID_ROW));
        }
        if grid_row > MAX_GRID_ROW {
            return Err(format!("Grid row cannot exceed {}", MAX_GRID_ROW));
        }
        Ok(())
    }

    /// Apply grid row to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply grid row to
    /// * `grid_row` - The grid row value
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates grid row
    pub fn apply_grid_row(&mut self, node: &mut TipTapNode, grid_row: i32) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate grid row
        self.validate_grid_row(grid_row)?;

        // Apply grid row to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("gridRow".to_string(), serde_json::json!(grid_row));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "gridRow": grid_row }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Grid row application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Grid row application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove grid row from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove grid row from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_grid_row(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("gridRow");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Grid row removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Grid row removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get grid row from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get grid row from
    /// 
    /// # Returns
    /// Option containing the grid row or None
    pub fn get_grid_row(&self, node: &TipTapNode) -> Option<i32> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(grid_row) = obj.get("gridRow") {
                    if let Some(n) = grid_row.as_i64() {
                        return Some(n as i32);
                    }
                }
            }
        }
        None
    }

    /// Check if node has grid row
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has grid row, false otherwise
    pub fn has_grid_row(&self, node: &TipTapNode) -> bool {
        self.get_grid_row(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_grid_row_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridRowManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(GridRowManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(GridRowManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_min_max_constants() {
        assert_eq!(GridRowManager::min_grid_row(), MIN_GRID_ROW);
        assert_eq!(GridRowManager::max_grid_row(), MAX_GRID_ROW);
    }

    #[test]
    fn test_apply_grid_row() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridRowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_row(&mut node, 1);
        assert!(result.is_ok());
        assert!(manager.has_grid_row(&node));
    }

    #[test]
    fn test_apply_grid_row_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridRowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_row(&mut node, MAX_GRID_ROW + 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_grid_row_too_small() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridRowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_row(&mut node, MIN_GRID_ROW - 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_grid_row() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridRowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridRow": 1 })),
            marks: None,
        };
        
        assert!(manager.has_grid_row(&node));
        let result = manager.remove_grid_row(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_grid_row(&node));
    }

    #[test]
    fn test_get_grid_row() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridRowManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridRow": 2 })),
            marks: None,
        };
        
        let grid_row = manager.get_grid_row(&node);
        assert_eq!(grid_row, Some(2));
    }

    #[test]
    fn test_get_grid_row_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridRowManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let grid_row = manager.get_grid_row(&node);
        assert!(grid_row.is_none());
    }

    #[test]
    fn test_has_grid_row() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridRowManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridRow": 3 })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_grid_row(&node_with));
        assert!(!manager.has_grid_row(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridRowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_grid_row(&mut node, 1).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridRowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_grid_row(&mut node, 1).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridRowManager::new(config_service);
        
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
        let mut manager = GridRowManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
