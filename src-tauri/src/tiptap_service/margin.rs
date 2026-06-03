//! TipTap Margin Manager - Aerospace-Grade Margin Operations Service
//!
//! Safety-critical margin operations service with:
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

/// Minimum margin value (in pixels)
const MIN_MARGIN: f64 = 0.0;

/// Maximum margin value (in pixels)
const MAX_MARGIN: f64 = 1000.0;

/// Margin attributes
#[derive(Debug, Clone, Copy, Default)]
pub struct MarginAttributes {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl MarginAttributes {
    /// Create new margin attributes
    pub fn new(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Self { top, right, bottom, left }
    }

    /// Create uniform margin
    pub fn uniform(value: f64) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }
}

pub struct MarginManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl MarginManager {
    /// Creates a new margin manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new MarginManager instance
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

    /// Get the minimum margin constant
    /// 
    /// # Returns
    /// The minimum margin in pixels
    pub fn min_margin() -> f64 {
        MIN_MARGIN
    }

    /// Get the maximum margin constant
    /// 
    /// # Returns
    /// The maximum margin in pixels
    pub fn max_margin() -> f64 {
        MAX_MARGIN
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

    /// Validate margin value
    /// 
    /// # Arguments
    /// * `margin` - The margin to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Ensures margin is within valid range to prevent rendering issues
    fn validate_margin(&self, margin: f64) -> Result<(), String> {
        if margin < MIN_MARGIN {
            return Err(format!("Margin must be at least {}", MIN_MARGIN));
        }
        if margin > MAX_MARGIN {
            return Err(format!("Margin cannot exceed {}", MAX_MARGIN));
        }
        if !margin.is_finite() {
            return Err("Margin must be a finite number".to_string());
        }
        Ok(())
    }

    /// Apply margin to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply margin to
    /// * `attributes` - The margin attributes
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates all margin values
    pub fn apply_margin(&mut self, node: &mut TipTapNode, attributes: MarginAttributes) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate margin values
        self.validate_margin(attributes.top)?;
        self.validate_margin(attributes.right)?;
        self.validate_margin(attributes.bottom)?;
        self.validate_margin(attributes.left)?;

        // Apply margin to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("marginTop".to_string(), serde_json::json!(attributes.top));
                obj.insert("marginRight".to_string(), serde_json::json!(attributes.right));
                obj.insert("marginBottom".to_string(), serde_json::json!(attributes.bottom));
                obj.insert("marginLeft".to_string(), serde_json::json!(attributes.left));
            }
        } else {
            node.attrs = Some(serde_json::json!({
                "marginTop": attributes.top,
                "marginRight": attributes.right,
                "marginBottom": attributes.bottom,
                "marginLeft": attributes.left
            }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Margin application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Margin application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove margin from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove margin from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_margin(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("marginTop");
                obj.remove("marginRight");
                obj.remove("marginBottom");
                obj.remove("marginLeft");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Margin removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Margin removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get margin from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get margin from
    /// 
    /// # Returns
    /// Option containing the margin attributes or None
    pub fn get_margin(&self, node: &TipTapNode) -> Option<MarginAttributes> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                let top = obj.get("marginTop").and_then(|v| v.as_f64())?;
                let right = obj.get("marginRight").and_then(|v| v.as_f64())?;
                let bottom = obj.get("marginBottom").and_then(|v| v.as_f64())?;
                let left = obj.get("marginLeft").and_then(|v| v.as_f64())?;
                return Some(MarginAttributes::new(top, right, bottom, left));
            }
        }
        None
    }

    /// Check if node has margin
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has margin, false otherwise
    pub fn has_margin(&self, node: &TipTapNode) -> bool {
        self.get_margin(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_margin_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MarginManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(MarginManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(MarginManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_min_max_constants() {
        assert_eq!(MarginManager::min_margin(), MIN_MARGIN);
        assert_eq!(MarginManager::max_margin(), MAX_MARGIN);
    }

    #[test]
    fn test_margin_attributes_default() {
        let attrs = MarginAttributes::default();
        assert_eq!(attrs.top, 0.0);
        assert_eq!(attrs.right, 0.0);
        assert_eq!(attrs.bottom, 0.0);
        assert_eq!(attrs.left, 0.0);
    }

    #[test]
    fn test_margin_attributes_new() {
        let attrs = MarginAttributes::new(10.0, 20.0, 30.0, 40.0);
        assert_eq!(attrs.top, 10.0);
        assert_eq!(attrs.right, 20.0);
        assert_eq!(attrs.bottom, 30.0);
        assert_eq!(attrs.left, 40.0);
    }

    #[test]
    fn test_margin_attributes_uniform() {
        let attrs = MarginAttributes::uniform(15.0);
        assert_eq!(attrs.top, 15.0);
        assert_eq!(attrs.right, 15.0);
        assert_eq!(attrs.bottom, 15.0);
        assert_eq!(attrs.left, 15.0);
    }

    #[test]
    fn test_apply_margin() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MarginManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = MarginAttributes::uniform(10.0);
        let result = manager.apply_margin(&mut node, attributes);
        assert!(result.is_ok());
        assert!(manager.has_margin(&node));
    }

    #[test]
    fn test_apply_margin_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MarginManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = MarginAttributes::uniform(MAX_MARGIN + 1.0);
        let result = manager.apply_margin(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_margin_negative() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MarginManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = MarginAttributes::uniform(-1.0);
        let result = manager.apply_margin(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_margin_infinite() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MarginManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = MarginAttributes::uniform(f64::INFINITY);
        let result = manager.apply_margin(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_margin() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MarginManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({
                "marginTop": 10.0,
                "marginRight": 10.0,
                "marginBottom": 10.0,
                "marginLeft": 10.0
            })),
            marks: None,
        };
        
        assert!(manager.has_margin(&node));
        let result = manager.remove_margin(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_margin(&node));
    }

    #[test]
    fn test_get_margin() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MarginManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({
                "marginTop": 5.0,
                "marginRight": 10.0,
                "marginBottom": 15.0,
                "marginLeft": 20.0
            })),
            marks: None,
        };
        
        let margin = manager.get_margin(&node);
        assert!(margin.is_some());
        let attrs = margin.unwrap();
        assert_eq!(attrs.top, 5.0);
        assert_eq!(attrs.right, 10.0);
        assert_eq!(attrs.bottom, 15.0);
        assert_eq!(attrs.left, 20.0);
    }

    #[test]
    fn test_get_margin_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MarginManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let margin = manager.get_margin(&node);
        assert!(margin.is_none());
    }

    #[test]
    fn test_has_margin() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MarginManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({
                "marginTop": 10.0,
                "marginRight": 10.0,
                "marginBottom": 10.0,
                "marginLeft": 10.0
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
        
        assert!(manager.has_margin(&node_with));
        assert!(!manager.has_margin(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MarginManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = MarginAttributes::uniform(10.0);
        manager.apply_margin(&mut node, attributes).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MarginManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = MarginAttributes::uniform(10.0);
        manager.apply_margin(&mut node, attributes).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MarginManager::new(config_service);
        
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
        let mut manager = MarginManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
