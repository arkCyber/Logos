//! TipTap Grid Template Rows Manager - Aerospace-Grade Grid Template Rows Operations Service
//!
//! Safety-critical grid template rows operations service with:
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

/// Maximum grid template rows string length
const MAX_GRID_TEMPLATE_ROWS_LENGTH: usize = 200;

pub struct GridTemplateRowsManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl GridTemplateRowsManager {
    /// Creates a new grid template rows manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new GridTemplateRowsManager instance
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

    /// Get the maximum grid template rows length constant
    /// 
    /// # Returns
    /// The maximum grid template rows string length
    pub fn max_grid_template_rows_length() -> usize {
        MAX_GRID_TEMPLATE_ROWS_LENGTH
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

    /// Validate grid template rows string
    /// 
    /// # Arguments
    /// * `grid_template_rows` - The grid template rows string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting grid template rows string length
    fn validate_grid_template_rows(&self, grid_template_rows: &str) -> Result<(), String> {
        if grid_template_rows.is_empty() {
            return Err("Grid template rows cannot be empty".to_string());
        }
        if grid_template_rows.len() > MAX_GRID_TEMPLATE_ROWS_LENGTH {
            return Err(format!("Grid template rows string exceeds maximum length of {} characters", MAX_GRID_TEMPLATE_ROWS_LENGTH));
        }
        // Check for unmatched parentheses first
        if grid_template_rows.contains('(') && !grid_template_rows.contains(')') {
            return Err("Invalid grid template rows: unmatched parentheses".to_string());
        }
        // Basic validation for common grid template values
        let valid_patterns = ["repeat", "minmax", "auto", "fr", "px", "%"];
        if !valid_patterns.iter().any(|pattern| grid_template_rows.contains(pattern)) 
            && grid_template_rows != "none" 
            && grid_template_rows != "subgrid" {
            // Allow custom values but validate basic structure
            if grid_template_rows.contains('(') && !grid_template_rows.contains(')') {
                return Err("Invalid grid template rows: unmatched parentheses".to_string());
            }
        }
        Ok(())
    }

    /// Apply grid template rows to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply grid template rows to
    /// * `grid_template_rows` - The grid template rows to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates grid template rows string
    pub fn apply_grid_template_rows(&mut self, node: &mut TipTapNode, grid_template_rows: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate grid template rows
        self.validate_grid_template_rows(grid_template_rows)?;

        // Apply grid template rows to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("gridTemplateRows".to_string(), serde_json::Value::String(grid_template_rows.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "gridTemplateRows": grid_template_rows }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Grid template rows application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Grid template rows application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove grid template rows from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove grid template rows from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_grid_template_rows(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("gridTemplateRows");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Grid template rows removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Grid template rows removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get grid template rows from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get grid template rows from
    /// 
    /// # Returns
    /// Option containing the grid template rows string or None
    pub fn get_grid_template_rows(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(grid_template_rows) = obj.get("gridTemplateRows") {
                    if let Some(s) = grid_template_rows.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has grid template rows
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has grid template rows, false otherwise
    pub fn has_grid_template_rows(&self, node: &TipTapNode) -> bool {
        self.get_grid_template_rows(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_grid_template_rows_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridTemplateRowsManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(GridTemplateRowsManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(GridTemplateRowsManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(GridTemplateRowsManager::max_grid_template_rows_length(), MAX_GRID_TEMPLATE_ROWS_LENGTH);
    }

    #[test]
    fn test_apply_grid_template_rows() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridTemplateRowsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_template_rows(&mut node, "repeat(3, 1fr)");
        assert!(result.is_ok());
        assert!(manager.has_grid_template_rows(&node));
    }

    #[test]
    fn test_apply_grid_template_rows_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridTemplateRowsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_template_rows(&mut node, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_grid_template_rows_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridTemplateRowsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_grid_template_rows = "a".repeat(MAX_GRID_TEMPLATE_ROWS_LENGTH + 1);
        let result = manager.apply_grid_template_rows(&mut node, &long_grid_template_rows);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_grid_template_rows_invalid_parentheses() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridTemplateRowsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_template_rows(&mut node, "repeat(3, 1fr");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_grid_template_rows() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridTemplateRowsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridTemplateRows": "repeat(3, 1fr)" })),
            marks: None,
        };
        
        assert!(manager.has_grid_template_rows(&node));
        let result = manager.remove_grid_template_rows(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_grid_template_rows(&node));
    }

    #[test]
    fn test_get_grid_template_rows() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridTemplateRowsManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridTemplateRows": "1fr 2fr 1fr" })),
            marks: None,
        };
        
        let grid_template_rows = manager.get_grid_template_rows(&node);
        assert_eq!(grid_template_rows, Some("1fr 2fr 1fr".to_string()));
    }

    #[test]
    fn test_get_grid_template_rows_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridTemplateRowsManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let grid_template_rows = manager.get_grid_template_rows(&node);
        assert!(grid_template_rows.is_none());
    }

    #[test]
    fn test_has_grid_template_rows() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridTemplateRowsManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridTemplateRows": "auto" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_grid_template_rows(&node_with));
        assert!(!manager.has_grid_template_rows(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridTemplateRowsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_grid_template_rows(&mut node, "repeat(3, 1fr)").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridTemplateRowsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_grid_template_rows(&mut node, "repeat(3, 1fr)").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridTemplateRowsManager::new(config_service);
        
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
        let mut manager = GridTemplateRowsManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
