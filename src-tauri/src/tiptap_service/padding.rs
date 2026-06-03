//! TipTap Padding Manager - Aerospace-Grade Padding Operations Service
//!
//! Safety-critical padding operations service with:
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

/// Minimum padding value (in pixels)
const MIN_PADDING: f64 = 0.0;

/// Maximum padding value (in pixels)
const MAX_PADDING: f64 = 1000.0;

/// Padding attributes
#[derive(Debug, Clone, Copy, Default)]
pub struct PaddingAttributes {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl PaddingAttributes {
    /// Create new padding attributes
    pub fn new(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Self { top, right, bottom, left }
    }

    /// Create uniform padding
    pub fn uniform(value: f64) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }
}

pub struct PaddingManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl PaddingManager {
    /// Creates a new padding manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new PaddingManager instance
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

    /// Get the minimum padding constant
    /// 
    /// # Returns
    /// The minimum padding in pixels
    pub fn min_padding() -> f64 {
        MIN_PADDING
    }

    /// Get the maximum padding constant
    /// 
    /// # Returns
    /// The maximum padding in pixels
    pub fn max_padding() -> f64 {
        MAX_PADDING
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

    /// Validate padding value
    /// 
    /// # Arguments
    /// * `padding` - The padding to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Ensures padding is within valid range to prevent rendering issues
    fn validate_padding(&self, padding: f64) -> Result<(), String> {
        if padding < MIN_PADDING {
            return Err(format!("Padding must be at least {}", MIN_PADDING));
        }
        if padding > MAX_PADDING {
            return Err(format!("Padding cannot exceed {}", MAX_PADDING));
        }
        if !padding.is_finite() {
            return Err("Padding must be a finite number".to_string());
        }
        Ok(())
    }

    /// Apply padding to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply padding to
    /// * `attributes` - The padding attributes
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates all padding values
    pub fn apply_padding(&mut self, node: &mut TipTapNode, attributes: PaddingAttributes) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate padding values
        self.validate_padding(attributes.top)?;
        self.validate_padding(attributes.right)?;
        self.validate_padding(attributes.bottom)?;
        self.validate_padding(attributes.left)?;

        // Apply padding to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("paddingTop".to_string(), serde_json::json!(attributes.top));
                obj.insert("paddingRight".to_string(), serde_json::json!(attributes.right));
                obj.insert("paddingBottom".to_string(), serde_json::json!(attributes.bottom));
                obj.insert("paddingLeft".to_string(), serde_json::json!(attributes.left));
            }
        } else {
            node.attrs = Some(serde_json::json!({
                "paddingTop": attributes.top,
                "paddingRight": attributes.right,
                "paddingBottom": attributes.bottom,
                "paddingLeft": attributes.left
            }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Padding application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Padding application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove padding from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove padding from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_padding(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("paddingTop");
                obj.remove("paddingRight");
                obj.remove("paddingBottom");
                obj.remove("paddingLeft");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Padding removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Padding removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get padding from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get padding from
    /// 
    /// # Returns
    /// Option containing the padding attributes or None
    pub fn get_padding(&self, node: &TipTapNode) -> Option<PaddingAttributes> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                let top = obj.get("paddingTop").and_then(|v| v.as_f64())?;
                let right = obj.get("paddingRight").and_then(|v| v.as_f64())?;
                let bottom = obj.get("paddingBottom").and_then(|v| v.as_f64())?;
                let left = obj.get("paddingLeft").and_then(|v| v.as_f64())?;
                return Some(PaddingAttributes::new(top, right, bottom, left));
            }
        }
        None
    }

    /// Check if node has padding
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has padding, false otherwise
    pub fn has_padding(&self, node: &TipTapNode) -> bool {
        self.get_padding(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_padding_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PaddingManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(PaddingManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(PaddingManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_min_max_constants() {
        assert_eq!(PaddingManager::min_padding(), MIN_PADDING);
        assert_eq!(PaddingManager::max_padding(), MAX_PADDING);
    }

    #[test]
    fn test_padding_attributes_default() {
        let attrs = PaddingAttributes::default();
        assert_eq!(attrs.top, 0.0);
        assert_eq!(attrs.right, 0.0);
        assert_eq!(attrs.bottom, 0.0);
        assert_eq!(attrs.left, 0.0);
    }

    #[test]
    fn test_padding_attributes_new() {
        let attrs = PaddingAttributes::new(10.0, 20.0, 30.0, 40.0);
        assert_eq!(attrs.top, 10.0);
        assert_eq!(attrs.right, 20.0);
        assert_eq!(attrs.bottom, 30.0);
        assert_eq!(attrs.left, 40.0);
    }

    #[test]
    fn test_padding_attributes_uniform() {
        let attrs = PaddingAttributes::uniform(15.0);
        assert_eq!(attrs.top, 15.0);
        assert_eq!(attrs.right, 15.0);
        assert_eq!(attrs.bottom, 15.0);
        assert_eq!(attrs.left, 15.0);
    }

    #[test]
    fn test_apply_padding() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PaddingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = PaddingAttributes::uniform(10.0);
        let result = manager.apply_padding(&mut node, attributes);
        assert!(result.is_ok());
        assert!(manager.has_padding(&node));
    }

    #[test]
    fn test_apply_padding_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PaddingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = PaddingAttributes::uniform(MAX_PADDING + 1.0);
        let result = manager.apply_padding(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_padding_negative() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PaddingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = PaddingAttributes::uniform(-1.0);
        let result = manager.apply_padding(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_padding_infinite() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PaddingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = PaddingAttributes::uniform(f64::INFINITY);
        let result = manager.apply_padding(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_padding() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PaddingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({
                "paddingTop": 10.0,
                "paddingRight": 10.0,
                "paddingBottom": 10.0,
                "paddingLeft": 10.0
            })),
            marks: None,
        };
        
        assert!(manager.has_padding(&node));
        let result = manager.remove_padding(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_padding(&node));
    }

    #[test]
    fn test_get_padding() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PaddingManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({
                "paddingTop": 5.0,
                "paddingRight": 10.0,
                "paddingBottom": 15.0,
                "paddingLeft": 20.0
            })),
            marks: None,
        };
        
        let padding = manager.get_padding(&node);
        assert!(padding.is_some());
        let attrs = padding.unwrap();
        assert_eq!(attrs.top, 5.0);
        assert_eq!(attrs.right, 10.0);
        assert_eq!(attrs.bottom, 15.0);
        assert_eq!(attrs.left, 20.0);
    }

    #[test]
    fn test_get_padding_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PaddingManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let padding = manager.get_padding(&node);
        assert!(padding.is_none());
    }

    #[test]
    fn test_has_padding() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PaddingManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({
                "paddingTop": 10.0,
                "paddingRight": 10.0,
                "paddingBottom": 10.0,
                "paddingLeft": 10.0
            })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_padding(&node_with));
        assert!(!manager.has_padding(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PaddingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = PaddingAttributes::uniform(10.0);
        manager.apply_padding(&mut node, attributes).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PaddingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = PaddingAttributes::uniform(10.0);
        manager.apply_padding(&mut node, attributes).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PaddingManager::new(config_service);
        
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
        let mut manager = PaddingManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
