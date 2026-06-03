//! TipTap Size Manager - Aerospace-Grade Size Operations Service
//!
//! Safety-critical size operations service with:
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

/// Minimum size value (in pixels)
const MIN_SIZE: f64 = 0.0;

/// Maximum size value (in pixels)
const MAX_SIZE: f64 = 10000.0;

/// Size attributes
#[derive(Debug, Clone, Copy, Default)]
pub struct SizeAttributes {
    pub width: f64,
    pub height: f64,
}

impl SizeAttributes {
    /// Create new size attributes
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    /// Create square size
    pub fn square(value: f64) -> Self {
        Self {
            width: value,
            height: value,
        }
    }
}

pub struct SizeManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl SizeManager {
    /// Creates a new size manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new SizeManager instance
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

    /// Get the minimum size constant
    /// 
    /// # Returns
    /// The minimum size in pixels
    pub fn min_size() -> f64 {
        MIN_SIZE
    }

    /// Get the maximum size constant
    /// 
    /// # Returns
    /// The maximum size in pixels
    pub fn max_size() -> f64 {
        MAX_SIZE
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

    /// Validate size value
    /// 
    /// # Arguments
    /// * `size` - The size to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Ensures size is within valid range to prevent rendering issues
    fn validate_size(&self, size: f64) -> Result<(), String> {
        if size < MIN_SIZE {
            return Err(format!("Size must be at least {}", MIN_SIZE));
        }
        if size > MAX_SIZE {
            return Err(format!("Size cannot exceed {}", MAX_SIZE));
        }
        if !size.is_finite() {
            return Err("Size must be a finite number".to_string());
        }
        Ok(())
    }

    /// Apply size to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply size to
    /// * `attributes` - The size attributes
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates all size values
    pub fn apply_size(&mut self, node: &mut TipTapNode, attributes: SizeAttributes) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate size values
        self.validate_size(attributes.width)?;
        self.validate_size(attributes.height)?;

        // Apply size to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("width".to_string(), serde_json::json!(attributes.width));
                obj.insert("height".to_string(), serde_json::json!(attributes.height));
            }
        } else {
            node.attrs = Some(serde_json::json!({
                "width": attributes.width,
                "height": attributes.height
            }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Size application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Size application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove size from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove size from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_size(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("width");
                obj.remove("height");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Size removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Size removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get size from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get size from
    /// 
    /// # Returns
    /// Option containing the size attributes or None
    pub fn get_size(&self, node: &TipTapNode) -> Option<SizeAttributes> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                let width = obj.get("width").and_then(|v| v.as_f64())?;
                let height = obj.get("height").and_then(|v| v.as_f64())?;
                return Some(SizeAttributes::new(width, height));
            }
        }
        None
    }

    /// Check if node has size
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has size, false otherwise
    pub fn has_size(&self, node: &TipTapNode) -> bool {
        self.get_size(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_size_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = SizeManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(SizeManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(SizeManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_min_max_constants() {
        assert_eq!(SizeManager::min_size(), MIN_SIZE);
        assert_eq!(SizeManager::max_size(), MAX_SIZE);
    }

    #[test]
    fn test_size_attributes_default() {
        let attrs = SizeAttributes::default();
        assert_eq!(attrs.width, 0.0);
        assert_eq!(attrs.height, 0.0);
    }

    #[test]
    fn test_size_attributes_new() {
        let attrs = SizeAttributes::new(100.0, 200.0);
        assert_eq!(attrs.width, 100.0);
        assert_eq!(attrs.height, 200.0);
    }

    #[test]
    fn test_size_attributes_square() {
        let attrs = SizeAttributes::square(150.0);
        assert_eq!(attrs.width, 150.0);
        assert_eq!(attrs.height, 150.0);
    }

    #[test]
    fn test_apply_size() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Image,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        let attributes = SizeAttributes::new(300.0, 200.0);
        let result = manager.apply_size(&mut node, attributes);
        assert!(result.is_ok());
        assert!(manager.has_size(&node));
    }

    #[test]
    fn test_apply_size_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Image,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        let attributes = SizeAttributes::new(MAX_SIZE + 1.0, 200.0);
        let result = manager.apply_size(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_size_negative() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Image,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        let attributes = SizeAttributes::new(-1.0, 200.0);
        let result = manager.apply_size(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_size_infinite() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Image,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        let attributes = SizeAttributes::new(f64::INFINITY, 200.0);
        let result = manager.apply_size(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_size() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Image,
            content: None,
            text: None,
            attrs: Some(serde_json::json!({
                "width": 300.0,
                "height": 200.0
            })),
            marks: None,
        };
        
        assert!(manager.has_size(&node));
        let result = manager.remove_size(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_size(&node));
    }

    #[test]
    fn test_get_size() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = SizeManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Image,
            content: None,
            text: None,
            attrs: Some(serde_json::json!({
                "width": 400.0,
                "height": 300.0
            })),
            marks: None,
        };
        
        let size = manager.get_size(&node);
        assert!(size.is_some());
        let attrs = size.unwrap();
        assert_eq!(attrs.width, 400.0);
        assert_eq!(attrs.height, 300.0);
    }

    #[test]
    fn test_get_size_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = SizeManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Image,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        let size = manager.get_size(&node);
        assert!(size.is_none());
    }

    #[test]
    fn test_has_size() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = SizeManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Image,
            content: None,
            text: None,
            attrs: Some(serde_json::json!({
                "width": 300.0,
                "height": 200.0
            })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Image,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_size(&node_with));
        assert!(!manager.has_size(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Image,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        let attributes = SizeAttributes::new(300.0, 200.0);
        manager.apply_size(&mut node, attributes).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Image,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        let attributes = SizeAttributes::new(300.0, 200.0);
        manager.apply_size(&mut node, attributes).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SizeManager::new(config_service);
        
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
        let mut manager = SizeManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
