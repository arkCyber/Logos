//! TipTap Grid Gap Manager - Aerospace-Grade Grid Gap Operations Service
//!
//! Safety-critical grid gap operations service with:
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

/// Maximum grid gap value (in pixels)
const MAX_GRID_GAP: f64 = 100.0;

/// Maximum grid gap string length
const MAX_GRID_GAP_LENGTH: usize = 50;

pub struct GridGapManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl GridGapManager {
    /// Creates a new grid gap manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new GridGapManager instance
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

    /// Get the maximum grid gap constant
    /// 
    /// # Returns
    /// The maximum grid gap in pixels
    pub fn max_grid_gap() -> f64 {
        MAX_GRID_GAP
    }

    /// Get the maximum grid gap length constant
    /// 
    /// # Returns
    /// The maximum grid gap string length
    pub fn max_grid_gap_length() -> usize {
        MAX_GRID_GAP_LENGTH
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

    /// Validate grid gap string
    /// 
    /// # Arguments
    /// * `grid_gap` - The grid gap string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting grid gap string length
    fn validate_grid_gap(&self, grid_gap: &str) -> Result<(), String> {
        if grid_gap.is_empty() {
            return Err("Grid gap cannot be empty".to_string());
        }
        if grid_gap.len() > MAX_GRID_GAP_LENGTH {
            return Err(format!("Grid gap string exceeds maximum length of {} characters", MAX_GRID_GAP_LENGTH));
        }
        // Validate numeric value if it's a pixel value
        if grid_gap.ends_with("px") {
            let value_str = grid_gap.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < 0.0 || value > MAX_GRID_GAP {
                    return Err(format!("Grid gap must be between 0 and {} pixels", MAX_GRID_GAP));
                }
                if !value.is_finite() {
                    return Err("Grid gap must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    /// Apply grid gap to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply grid gap to
    /// * `grid_gap` - The grid gap to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates grid gap string
    pub fn apply_grid_gap(&mut self, node: &mut TipTapNode, grid_gap: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate grid gap
        self.validate_grid_gap(grid_gap)?;

        // Apply grid gap to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("gridGap".to_string(), serde_json::Value::String(grid_gap.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "gridGap": grid_gap }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Grid gap application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Grid gap application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove grid gap from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove grid gap from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_grid_gap(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("gridGap");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Grid gap removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Grid gap removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get grid gap from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get grid gap from
    /// 
    /// # Returns
    /// Option containing the grid gap string or None
    pub fn get_grid_gap(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(grid_gap) = obj.get("gridGap") {
                    if let Some(s) = grid_gap.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has grid gap
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has grid gap, false otherwise
    pub fn has_grid_gap(&self, node: &TipTapNode) -> bool {
        self.get_grid_gap(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_grid_gap_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridGapManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(GridGapManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(GridGapManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(GridGapManager::max_grid_gap(), MAX_GRID_GAP);
        assert_eq!(GridGapManager::max_grid_gap_length(), MAX_GRID_GAP_LENGTH);
    }

    #[test]
    fn test_apply_grid_gap() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridGapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_gap(&mut node, "10px");
        assert!(result.is_ok());
        assert!(manager.has_grid_gap(&node));
    }

    #[test]
    fn test_apply_grid_gap_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridGapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_gap(&mut node, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_grid_gap_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridGapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_grid_gap = "a".repeat(MAX_GRID_GAP_LENGTH + 1);
        let result = manager.apply_grid_gap(&mut node, &long_grid_gap);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_grid_gap_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridGapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_gap(&mut node, "200px");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_grid_gap() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridGapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridGap": "10px" })),
            marks: None,
        };
        
        assert!(manager.has_grid_gap(&node));
        let result = manager.remove_grid_gap(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_grid_gap(&node));
    }

    #[test]
    fn test_get_grid_gap() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridGapManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridGap": "20px" })),
            marks: None,
        };
        
        let grid_gap = manager.get_grid_gap(&node);
        assert_eq!(grid_gap, Some("20px".to_string()));
    }

    #[test]
    fn test_get_grid_gap_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridGapManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let grid_gap = manager.get_grid_gap(&node);
        assert!(grid_gap.is_none());
    }

    #[test]
    fn test_has_grid_gap() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridGapManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridGap": "15px" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_grid_gap(&node_with));
        assert!(!manager.has_grid_gap(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridGapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_grid_gap(&mut node, "10px").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridGapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_grid_gap(&mut node, "10px").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridGapManager::new(config_service);
        
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
        let mut manager = GridGapManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
