//! TipTap Grid Area Manager - Aerospace-Grade Grid Area Operations Service
//!
//! Safety-critical grid area operations service with:
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

/// Maximum grid area string length
const MAX_GRID_AREA_LENGTH: usize = 100;

pub struct GridAreaManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl GridAreaManager {
    /// Creates a new grid area manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new GridAreaManager instance
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

    /// Get the maximum grid area length constant
    /// 
    /// # Returns
    /// The maximum grid area string length
    pub fn max_grid_area_length() -> usize {
        MAX_GRID_AREA_LENGTH
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

    /// Validate grid area string
    /// 
    /// # Arguments
    /// * `grid_area` - The grid area string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting grid area string length
    fn validate_grid_area(&self, grid_area: &str) -> Result<(), String> {
        if grid_area.is_empty() {
            return Err("Grid area cannot be empty".to_string());
        }
        if grid_area.len() > MAX_GRID_AREA_LENGTH {
            return Err(format!("Grid area string exceeds maximum length of {} characters", MAX_GRID_AREA_LENGTH));
        }
        // Basic validation for grid area format (e.g., "1 / 1 / 2 / 3" or "header")
        if grid_area.contains('/') {
            let parts: Vec<&str> = grid_area.split('/').collect();
            if parts.len() != 4 {
                return Err("Grid area with slashes must have exactly 4 parts".to_string());
            }
        }
        Ok(())
    }

    /// Apply grid area to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply grid area to
    /// * `grid_area` - The grid area to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates grid area string
    pub fn apply_grid_area(&mut self, node: &mut TipTapNode, grid_area: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate grid area
        self.validate_grid_area(grid_area)?;

        // Apply grid area to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("gridArea".to_string(), serde_json::Value::String(grid_area.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "gridArea": grid_area }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Grid area application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Grid area application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove grid area from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove grid area from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_grid_area(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("gridArea");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Grid area removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Grid area removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get grid area from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get grid area from
    /// 
    /// # Returns
    /// Option containing the grid area string or None
    pub fn get_grid_area(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(grid_area) = obj.get("gridArea") {
                    if let Some(s) = grid_area.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has grid area
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has grid area, false otherwise
    pub fn has_grid_area(&self, node: &TipTapNode) -> bool {
        self.get_grid_area(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_grid_area_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridAreaManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(GridAreaManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(GridAreaManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(GridAreaManager::max_grid_area_length(), MAX_GRID_AREA_LENGTH);
    }

    #[test]
    fn test_apply_grid_area() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridAreaManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_area(&mut node, "header");
        assert!(result.is_ok());
        assert!(manager.has_grid_area(&node));
    }

    #[test]
    fn test_apply_grid_area_with_slashes() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridAreaManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_area(&mut node, "1 / 1 / 2 / 3");
        assert!(result.is_ok());
        assert!(manager.has_grid_area(&node));
    }

    #[test]
    fn test_apply_grid_area_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridAreaManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_area(&mut node, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_grid_area_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridAreaManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_grid_area = "a".repeat(MAX_GRID_AREA_LENGTH + 1);
        let result = manager.apply_grid_area(&mut node, &long_grid_area);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_grid_area_invalid_slashes() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridAreaManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_area(&mut node, "1 / 2 / 3");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_grid_area() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridAreaManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridArea": "header" })),
            marks: None,
        };
        
        assert!(manager.has_grid_area(&node));
        let result = manager.remove_grid_area(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_grid_area(&node));
    }

    #[test]
    fn test_get_grid_area() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridAreaManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridArea": "footer" })),
            marks: None,
        };
        
        let grid_area = manager.get_grid_area(&node);
        assert_eq!(grid_area, Some("footer".to_string()));
    }

    #[test]
    fn test_get_grid_area_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridAreaManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let grid_area = manager.get_grid_area(&node);
        assert!(grid_area.is_none());
    }

    #[test]
    fn test_has_grid_area() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridAreaManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridArea": "sidebar" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_grid_area(&node_with));
        assert!(!manager.has_grid_area(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridAreaManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_grid_area(&mut node, "header").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridAreaManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_grid_area(&mut node, "header").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridAreaManager::new(config_service);
        
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
        let mut manager = GridAreaManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
