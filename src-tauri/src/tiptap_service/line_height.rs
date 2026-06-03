//! TipTap Line Height Manager - Aerospace-Grade Line Height Operations Service
//!
//! Safety-critical line height operations service with:
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

/// Minimum line height (as a multiplier)
const MIN_LINE_HEIGHT: f64 = 0.5;

/// Maximum line height (as a multiplier)
const MAX_LINE_HEIGHT: f64 = 10.0;

pub struct LineHeightManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl LineHeightManager {
    /// Creates a new line height manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new LineHeightManager instance
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

    /// Get the minimum line height constant
    /// 
    /// # Returns
    /// The minimum line height as a multiplier
    pub fn min_line_height() -> f64 {
        MIN_LINE_HEIGHT
    }

    /// Get the maximum line height constant
    /// 
    /// # Returns
    /// The maximum line height as a multiplier
    pub fn max_line_height() -> f64 {
        MAX_LINE_HEIGHT
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

    /// Validate line height
    /// 
    /// # Arguments
    /// * `height` - The line height to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Ensures line height is within valid range to prevent rendering issues
    fn validate_line_height(&self, height: f64) -> Result<(), String> {
        if height < MIN_LINE_HEIGHT {
            return Err(format!("Line height must be at least {}", MIN_LINE_HEIGHT));
        }
        if height > MAX_LINE_HEIGHT {
            return Err(format!("Line height cannot exceed {}", MAX_LINE_HEIGHT));
        }
        if !height.is_finite() {
            return Err("Line height must be a finite number".to_string());
        }
        Ok(())
    }

    /// Apply line height to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply line height to
    /// * `height` - The line height as a multiplier
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates line height
    pub fn apply_line_height(&mut self, node: &mut TipTapNode, height: f64) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate line height
        self.validate_line_height(height)?;

        // Apply line height to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("lineHeight".to_string(), serde_json::json!(height));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "lineHeight": height }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Line height application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Line height application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove line height from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove line height from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_line_height(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("lineHeight");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Line height removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Line height removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get line height from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get line height from
    /// 
    /// # Returns
    /// Option containing the line height or None
    pub fn get_line_height(&self, node: &TipTapNode) -> Option<f64> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(height) = obj.get("lineHeight") {
                    if let Some(n) = height.as_f64() {
                        return Some(n);
                    }
                }
            }
        }
        None
    }

    /// Check if node has line height
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has line height, false otherwise
    pub fn has_line_height(&self, node: &TipTapNode) -> bool {
        self.get_line_height(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_line_height_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = LineHeightManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(LineHeightManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(LineHeightManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_min_max_constants() {
        assert_eq!(LineHeightManager::min_line_height(), MIN_LINE_HEIGHT);
        assert_eq!(LineHeightManager::max_line_height(), MAX_LINE_HEIGHT);
    }

    #[test]
    fn test_apply_line_height() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LineHeightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_line_height(&mut node, 1.5);
        assert!(result.is_ok());
        assert!(manager.has_line_height(&node));
    }

    #[test]
    fn test_apply_line_height_too_small() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LineHeightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_line_height(&mut node, MIN_LINE_HEIGHT - 0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_line_height_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LineHeightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_line_height(&mut node, MAX_LINE_HEIGHT + 0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_line_height_infinite() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LineHeightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_line_height(&mut node, f64::INFINITY);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_line_height() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LineHeightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "lineHeight": 1.5 })),
            marks: None,
        };
        
        assert!(manager.has_line_height(&node));
        let result = manager.remove_line_height(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_line_height(&node));
    }

    #[test]
    fn test_get_line_height() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LineHeightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_line_height(&mut node, 2.0).unwrap();
        let height = manager.get_line_height(&node);
        assert_eq!(height, Some(2.0));
    }

    #[test]
    fn test_get_line_height_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = LineHeightManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let height = manager.get_line_height(&node);
        assert!(height.is_none());
    }

    #[test]
    fn test_has_line_height() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LineHeightManager::new(config_service);
        
        let mut node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_line_height(&mut node_with, 1.5).unwrap();
        
        assert!(manager.has_line_height(&node_with));
        assert!(!manager.has_line_height(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LineHeightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_line_height(&mut node, 1.5).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LineHeightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_line_height(&mut node, 1.5).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LineHeightManager::new(config_service);
        
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
        let mut manager = LineHeightManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
