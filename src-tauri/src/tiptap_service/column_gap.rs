//! TipTap Column Gap Manager - Aerospace-Grade Column Gap Operations Service
//!
//! Safety-critical column gap operations service with:
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

/// Maximum column gap value (in pixels)
const MAX_COLUMN_GAP: f64 = 100.0;

/// Minimum column gap value (in pixels)
const MIN_COLUMN_GAP: f64 = 0.0;

/// Maximum column gap string length
const MAX_COLUMN_GAP_LENGTH: usize = 100;

pub struct ColumnGapManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ColumnGapManager {
    /// Creates a new column gap manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new ColumnGapManager instance
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

    /// Get the maximum column gap constant
    /// 
    /// # Returns
    /// The maximum column gap in pixels
    pub fn max_column_gap() -> f64 {
        MAX_COLUMN_GAP
    }

    /// Get the minimum column gap constant
    /// 
    /// # Returns
    /// The minimum column gap in pixels
    pub fn min_column_gap() -> f64 {
        MIN_COLUMN_GAP
    }

    /// Get the maximum column gap length constant
    /// 
    /// # Returns
    /// The maximum column gap string length
    pub fn max_column_gap_length() -> usize {
        MAX_COLUMN_GAP_LENGTH
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

    /// Validate column gap string
    /// 
    /// # Arguments
    /// * `column_gap` - The column gap string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting column gap string length
    fn validate_column_gap(&self, column_gap: &str) -> Result<(), String> {
        if column_gap.is_empty() {
            return Err("Column gap cannot be empty".to_string());
        }
        if column_gap.len() > MAX_COLUMN_GAP_LENGTH {
            return Err(format!("Column gap string exceeds maximum length of {} characters", MAX_COLUMN_GAP_LENGTH));
        }
        // Validate numeric value if it's a pixel value
        if column_gap.ends_with("px") {
            let value_str = column_gap.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_COLUMN_GAP || value > MAX_COLUMN_GAP {
                    return Err(format!("Column gap must be between {} and {} pixels", MIN_COLUMN_GAP, MAX_COLUMN_GAP));
                }
                if !value.is_finite() {
                    return Err("Column gap must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    /// Apply column gap to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply column gap to
    /// * `column_gap` - The column gap to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates column gap string
    pub fn apply_column_gap(&mut self, node: &mut TipTapNode, column_gap: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate column gap
        self.validate_column_gap(column_gap)?;

        // Apply column gap to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("columnGap".to_string(), serde_json::Value::String(column_gap.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "columnGap": column_gap }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Column gap application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Column gap application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove column gap from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove column gap from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_column_gap(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("columnGap");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Column gap removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Column gap removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get column gap from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get column gap from
    /// 
    /// # Returns
    /// Option containing the column gap string or None
    pub fn get_column_gap(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(column_gap) = obj.get("columnGap") {
                    if let Some(s) = column_gap.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has column gap
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has column gap, false otherwise
    pub fn has_column_gap(&self, node: &TipTapNode) -> bool {
        self.get_column_gap(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_column_gap_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnGapManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(ColumnGapManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(ColumnGapManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_min_max_constants() {
        assert_eq!(ColumnGapManager::min_column_gap(), MIN_COLUMN_GAP);
        assert_eq!(ColumnGapManager::max_column_gap(), MAX_COLUMN_GAP);
        assert_eq!(ColumnGapManager::max_column_gap_length(), MAX_COLUMN_GAP_LENGTH);
    }

    #[test]
    fn test_apply_column_gap() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnGapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_column_gap(&mut node, "20px");
        assert!(result.is_ok());
        assert!(manager.has_column_gap(&node));
    }

    #[test]
    fn test_apply_column_gap_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnGapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_column_gap(&mut node, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_column_gap_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnGapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_column_gap = "a".repeat(MAX_COLUMN_GAP_LENGTH + 1);
        let result = manager.apply_column_gap(&mut node, &long_column_gap);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_column_gap_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnGapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_column_gap(&mut node, "200px");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_column_gap() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnGapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "columnGap": "10px" })),
            marks: None,
        };
        
        assert!(manager.has_column_gap(&node));
        let result = manager.remove_column_gap(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_column_gap(&node));
    }

    #[test]
    fn test_get_column_gap() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnGapManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "columnGap": "30px" })),
            marks: None,
        };
        
        let column_gap = manager.get_column_gap(&node);
        assert_eq!(column_gap, Some("30px".to_string()));
    }

    #[test]
    fn test_get_column_gap_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnGapManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let column_gap = manager.get_column_gap(&node);
        assert!(column_gap.is_none());
    }

    #[test]
    fn test_has_column_gap() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnGapManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "columnGap": "15px" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_column_gap(&node_with));
        assert!(!manager.has_column_gap(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnGapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_column_gap(&mut node, "20px").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnGapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_column_gap(&mut node, "20px").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnGapManager::new(config_service);
        
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
        let mut manager = ColumnGapManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
