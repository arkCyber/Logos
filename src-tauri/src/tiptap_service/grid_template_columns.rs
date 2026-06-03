//! TipTap Grid Template Columns Manager - Aerospace-Grade Grid Template Columns Operations Service
//!
//! Safety-critical grid template columns operations service with:
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

/// Maximum grid template columns string length
const MAX_GRID_TEMPLATE_COLUMNS_LENGTH: usize = 200;

pub struct GridTemplateColumnsManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl GridTemplateColumnsManager {
    /// Creates a new grid template columns manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new GridTemplateColumnsManager instance
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

    /// Get the maximum grid template columns length constant
    /// 
    /// # Returns
    /// The maximum grid template columns string length
    pub fn max_grid_template_columns_length() -> usize {
        MAX_GRID_TEMPLATE_COLUMNS_LENGTH
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

    /// Validate grid template columns string
    /// 
    /// # Arguments
    /// * `grid_template_columns` - The grid template columns string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting grid template columns string length
    fn validate_grid_template_columns(&self, grid_template_columns: &str) -> Result<(), String> {
        if grid_template_columns.is_empty() {
            return Err("Grid template columns cannot be empty".to_string());
        }
        if grid_template_columns.len() > MAX_GRID_TEMPLATE_COLUMNS_LENGTH {
            return Err(format!("Grid template columns string exceeds maximum length of {} characters", MAX_GRID_TEMPLATE_COLUMNS_LENGTH));
        }
        // Check for unmatched parentheses first
        if grid_template_columns.contains('(') && !grid_template_columns.contains(')') {
            return Err("Invalid grid template columns: unmatched parentheses".to_string());
        }
        // Basic validation for common grid template values
        let valid_patterns = ["repeat", "minmax", "auto", "fr", "px", "%"];
        if !valid_patterns.iter().any(|pattern| grid_template_columns.contains(pattern)) 
            && grid_template_columns != "none" 
            && grid_template_columns != "subgrid" {
            // Allow custom values but validate basic structure
            if grid_template_columns.contains('(') && !grid_template_columns.contains(')') {
                return Err("Invalid grid template columns: unmatched parentheses".to_string());
            }
        }
        Ok(())
    }

    /// Apply grid template columns to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply grid template columns to
    /// * `grid_template_columns` - The grid template columns to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates grid template columns string
    pub fn apply_grid_template_columns(&mut self, node: &mut TipTapNode, grid_template_columns: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate grid template columns
        self.validate_grid_template_columns(grid_template_columns)?;

        // Apply grid template columns to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("gridTemplateColumns".to_string(), serde_json::Value::String(grid_template_columns.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "gridTemplateColumns": grid_template_columns }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Grid template columns application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Grid template columns application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove grid template columns from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove grid template columns from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_grid_template_columns(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("gridTemplateColumns");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Grid template columns removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Grid template columns removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get grid template columns from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get grid template columns from
    /// 
    /// # Returns
    /// Option containing the grid template columns string or None
    pub fn get_grid_template_columns(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(grid_template_columns) = obj.get("gridTemplateColumns") {
                    if let Some(s) = grid_template_columns.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has grid template columns
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has grid template columns, false otherwise
    pub fn has_grid_template_columns(&self, node: &TipTapNode) -> bool {
        self.get_grid_template_columns(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_grid_template_columns_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridTemplateColumnsManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(GridTemplateColumnsManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(GridTemplateColumnsManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(GridTemplateColumnsManager::max_grid_template_columns_length(), MAX_GRID_TEMPLATE_COLUMNS_LENGTH);
    }

    #[test]
    fn test_apply_grid_template_columns() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridTemplateColumnsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_template_columns(&mut node, "repeat(3, 1fr)");
        assert!(result.is_ok());
        assert!(manager.has_grid_template_columns(&node));
    }

    #[test]
    fn test_apply_grid_template_columns_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridTemplateColumnsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_template_columns(&mut node, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_grid_template_columns_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridTemplateColumnsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_grid_template_columns = "a".repeat(MAX_GRID_TEMPLATE_COLUMNS_LENGTH + 1);
        let result = manager.apply_grid_template_columns(&mut node, &long_grid_template_columns);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_grid_template_columns_invalid_parentheses() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridTemplateColumnsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_template_columns(&mut node, "repeat(3, 1fr");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_grid_template_columns() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridTemplateColumnsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridTemplateColumns": "repeat(3, 1fr)" })),
            marks: None,
        };
        
        assert!(manager.has_grid_template_columns(&node));
        let result = manager.remove_grid_template_columns(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_grid_template_columns(&node));
    }

    #[test]
    fn test_get_grid_template_columns() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridTemplateColumnsManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridTemplateColumns": "1fr 2fr 1fr" })),
            marks: None,
        };
        
        let grid_template_columns = manager.get_grid_template_columns(&node);
        assert_eq!(grid_template_columns, Some("1fr 2fr 1fr".to_string()));
    }

    #[test]
    fn test_get_grid_template_columns_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridTemplateColumnsManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let grid_template_columns = manager.get_grid_template_columns(&node);
        assert!(grid_template_columns.is_none());
    }

    #[test]
    fn test_has_grid_template_columns() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridTemplateColumnsManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridTemplateColumns": "auto" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_grid_template_columns(&node_with));
        assert!(!manager.has_grid_template_columns(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridTemplateColumnsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_grid_template_columns(&mut node, "repeat(3, 1fr)").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridTemplateColumnsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_grid_template_columns(&mut node, "repeat(3, 1fr)").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridTemplateColumnsManager::new(config_service);
        
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
        let mut manager = GridTemplateColumnsManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
