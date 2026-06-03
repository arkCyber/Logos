//! TipTap Transform Manager - Aerospace-Grade Transform Operations Service
//!
//! Safety-critical transform operations service with:
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

/// Maximum transform value (in pixels or degrees)
const MAX_TRANSFORM_VALUE: f64 = 1000.0;

/// Maximum transform string length
const MAX_TRANSFORM_LENGTH: usize = 100;

/// Transform attributes
#[derive(Debug, Clone)]
pub struct TransformAttributes {
    pub translate_x: f64,
    pub translate_y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub rotate: f64,
    pub skew_x: f64,
    pub skew_y: f64,
}

impl Default for TransformAttributes {
    fn default() -> Self {
        Self {
            translate_x: 0.0,
            translate_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotate: 0.0,
            skew_x: 0.0,
            skew_y: 0.0,
        }
    }
}

pub struct TransformManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TransformManager {
    /// Creates a new transform manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new TransformManager instance
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

    /// Get the maximum transform value constant
    /// 
    /// # Returns
    /// The maximum transform value
    pub fn max_transform_value() -> f64 {
        MAX_TRANSFORM_VALUE
    }

    /// Get the maximum transform length constant
    /// 
    /// # Returns
    /// The maximum transform string length
    pub fn max_transform_length() -> usize {
        MAX_TRANSFORM_LENGTH
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

    /// Validate translate value
    /// 
    /// # Arguments
    /// * `value` - The translate value to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    fn validate_translate(&self, value: f64) -> Result<(), String> {
        if value.abs() > MAX_TRANSFORM_VALUE {
            return Err(format!("Translate value cannot exceed {}", MAX_TRANSFORM_VALUE));
        }
        if !value.is_finite() {
            return Err("Translate value must be a finite number".to_string());
        }
        Ok(())
    }

    /// Validate scale value
    /// 
    /// # Arguments
    /// * `value` - The scale value to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    fn validate_scale(&self, value: f64) -> Result<(), String> {
        if value.abs() > MAX_TRANSFORM_VALUE {
            return Err(format!("Scale value cannot exceed {}", MAX_TRANSFORM_VALUE));
        }
        if !value.is_finite() {
            return Err("Scale value must be a finite number".to_string());
        }
        Ok(())
    }

    /// Validate rotate value
    /// 
    /// # Arguments
    /// * `value` - The rotate value to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    fn validate_rotate(&self, value: f64) -> Result<(), String> {
        if value.abs() > 360.0 {
            return Err("Rotate value must be between -360 and 360 degrees".to_string());
        }
        if !value.is_finite() {
            return Err("Rotate value must be a finite number".to_string());
        }
        Ok(())
    }

    /// Validate skew value
    /// 
    /// # Arguments
    /// * `value` - The skew value to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    fn validate_skew(&self, value: f64) -> Result<(), String> {
        if value.abs() > 90.0 {
            return Err("Skew value must be between -90 and 90 degrees".to_string());
        }
        if !value.is_finite() {
            return Err("Skew value must be a finite number".to_string());
        }
        Ok(())
    }

    /// Apply transform to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply transform to
    /// * `attributes` - The transform attributes
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates all transform attributes
    pub fn apply_transform(&mut self, node: &mut TipTapNode, attributes: TransformAttributes) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate attributes
        self.validate_translate(attributes.translate_x)?;
        self.validate_translate(attributes.translate_y)?;
        self.validate_scale(attributes.scale_x)?;
        self.validate_scale(attributes.scale_y)?;
        self.validate_rotate(attributes.rotate)?;
        self.validate_skew(attributes.skew_x)?;
        self.validate_skew(attributes.skew_y)?;

        // Apply transform to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("transformTranslateX".to_string(), serde_json::json!(attributes.translate_x));
                obj.insert("transformTranslateY".to_string(), serde_json::json!(attributes.translate_y));
                obj.insert("transformScaleX".to_string(), serde_json::json!(attributes.scale_x));
                obj.insert("transformScaleY".to_string(), serde_json::json!(attributes.scale_y));
                obj.insert("transformRotate".to_string(), serde_json::json!(attributes.rotate));
                obj.insert("transformSkewX".to_string(), serde_json::json!(attributes.skew_x));
                obj.insert("transformSkewY".to_string(), serde_json::json!(attributes.skew_y));
            }
        } else {
            node.attrs = Some(serde_json::json!({
                "transformTranslateX": attributes.translate_x,
                "transformTranslateY": attributes.translate_y,
                "transformScaleX": attributes.scale_x,
                "transformScaleY": attributes.scale_y,
                "transformRotate": attributes.rotate,
                "transformSkewX": attributes.skew_x,
                "transformSkewY": attributes.skew_y
            }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Transform application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Transform application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove transform from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove transform from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_transform(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("transformTranslateX");
                obj.remove("transformTranslateY");
                obj.remove("transformScaleX");
                obj.remove("transformScaleY");
                obj.remove("transformRotate");
                obj.remove("transformSkewX");
                obj.remove("transformSkewY");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Transform removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Transform removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get transform from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get transform from
    /// 
    /// # Returns
    /// Option containing the transform attributes or None
    pub fn get_transform(&self, node: &TipTapNode) -> Option<TransformAttributes> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                let translate_x = obj.get("transformTranslateX").and_then(|v| v.as_f64())?;
                let translate_y = obj.get("transformTranslateY").and_then(|v| v.as_f64())?;
                let scale_x = obj.get("transformScaleX").and_then(|v| v.as_f64())?;
                let scale_y = obj.get("transformScaleY").and_then(|v| v.as_f64())?;
                let rotate = obj.get("transformRotate").and_then(|v| v.as_f64())?;
                let skew_x = obj.get("transformSkewX").and_then(|v| v.as_f64())?;
                let skew_y = obj.get("transformSkewY").and_then(|v| v.as_f64())?;
                return Some(TransformAttributes {
                    translate_x,
                    translate_y,
                    scale_x,
                    scale_y,
                    rotate,
                    skew_x,
                    skew_y,
                });
            }
        }
        None
    }

    /// Check if node has transform
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has transform, false otherwise
    pub fn has_transform(&self, node: &TipTapNode) -> bool {
        self.get_transform(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_transform_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TransformManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(TransformManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(TransformManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(TransformManager::max_transform_value(), MAX_TRANSFORM_VALUE);
        assert_eq!(TransformManager::max_transform_length(), MAX_TRANSFORM_LENGTH);
    }

    #[test]
    fn test_transform_attributes_default() {
        let attrs = TransformAttributes::default();
        assert_eq!(attrs.translate_x, 0.0);
        assert_eq!(attrs.translate_y, 0.0);
        assert_eq!(attrs.scale_x, 1.0);
        assert_eq!(attrs.scale_y, 1.0);
        assert_eq!(attrs.rotate, 0.0);
        assert_eq!(attrs.skew_x, 0.0);
        assert_eq!(attrs.skew_y, 0.0);
    }

    #[test]
    fn test_apply_transform() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransformManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = TransformAttributes {
            translate_x: 10.0,
            translate_y: 20.0,
            scale_x: 1.5,
            scale_y: 1.5,
            rotate: 45.0,
            skew_x: 0.0,
            skew_y: 0.0,
        };
        let result = manager.apply_transform(&mut node, attributes);
        assert!(result.is_ok());
        assert!(manager.has_transform(&node));
    }

    #[test]
    fn test_apply_transform_translate_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransformManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = TransformAttributes {
            translate_x: MAX_TRANSFORM_VALUE + 1.0,
            translate_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotate: 0.0,
            skew_x: 0.0,
            skew_y: 0.0,
        };
        let result = manager.apply_transform(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_transform_rotate_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransformManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = TransformAttributes {
            translate_x: 0.0,
            translate_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotate: 361.0,
            skew_x: 0.0,
            skew_y: 0.0,
        };
        let result = manager.apply_transform(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_transform_skew_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransformManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = TransformAttributes {
            translate_x: 0.0,
            translate_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotate: 0.0,
            skew_x: 91.0,
            skew_y: 0.0,
        };
        let result = manager.apply_transform(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_transform() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransformManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({
                "transformTranslateX": 10.0,
                "transformTranslateY": 20.0,
                "transformScaleX": 1.5,
                "transformScaleY": 1.5,
                "transformRotate": 45.0,
                "transformSkewX": 0.0,
                "transformSkewY": 0.0
            })),
            marks: None,
        };
        
        assert!(manager.has_transform(&node));
        let result = manager.remove_transform(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_transform(&node));
    }

    #[test]
    fn test_get_transform() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TransformManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({
                "transformTranslateX": 15.0,
                "transformTranslateY": 25.0,
                "transformScaleX": 2.0,
                "transformScaleY": 2.0,
                "transformRotate": 90.0,
                "transformSkewX": 10.0,
                "transformSkewY": 5.0
            })),
            marks: None,
        };
        
        let transform = manager.get_transform(&node);
        assert!(transform.is_some());
        let attrs = transform.unwrap();
        assert_eq!(attrs.translate_x, 15.0);
        assert_eq!(attrs.translate_y, 25.0);
        assert_eq!(attrs.scale_x, 2.0);
        assert_eq!(attrs.scale_y, 2.0);
        assert_eq!(attrs.rotate, 90.0);
        assert_eq!(attrs.skew_x, 10.0);
        assert_eq!(attrs.skew_y, 5.0);
    }

    #[test]
    fn test_get_transform_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TransformManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let transform = manager.get_transform(&node);
        assert!(transform.is_none());
    }

    #[test]
    fn test_has_transform() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TransformManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({
                "transformTranslateX": 10.0,
                "transformTranslateY": 20.0,
                "transformScaleX": 1.5,
                "transformScaleY": 1.5,
                "transformRotate": 45.0,
                "transformSkewX": 0.0,
                "transformSkewY": 0.0
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
        
        assert!(manager.has_transform(&node_with));
        assert!(!manager.has_transform(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransformManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = TransformAttributes::default();
        manager.apply_transform(&mut node, attributes).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransformManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = TransformAttributes::default();
        manager.apply_transform(&mut node, attributes).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TransformManager::new(config_service);
        
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
        let mut manager = TransformManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
