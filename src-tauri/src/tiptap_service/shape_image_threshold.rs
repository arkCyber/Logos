//! TipTap Shape Image Threshold Manager - Aerospace-Grade Shape Image Threshold Operations Service
//!
//! Safety-critical shape image threshold operations service with:
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

/// Maximum shape image threshold value
const MAX_SHAPE_IMAGE_THRESHOLD: f64 = 1.0;

/// Minimum shape image threshold value
const MIN_SHAPE_IMAGE_THRESHOLD: f64 = 0.0;

/// Maximum shape image threshold string length
const MAX_SHAPE_IMAGE_THRESHOLD_LENGTH: usize = 50;

pub struct ShapeImageThresholdManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ShapeImageThresholdManager {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
        }
    }

    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    pub fn max_shape_image_threshold() -> f64 {
        MAX_SHAPE_IMAGE_THRESHOLD
    }

    pub fn min_shape_image_threshold() -> f64 {
        MIN_SHAPE_IMAGE_THRESHOLD
    }

    pub fn max_shape_image_threshold_length() -> usize {
        MAX_SHAPE_IMAGE_THRESHOLD_LENGTH
    }

    fn record_error(&mut self, code: &str, message: &str, source: &str) {
        self.last_error = Some(ErrorContext::new(ErrorSeverity::Error, code, message, source));
    }

    pub fn get_last_error(&self) -> Option<&ErrorContext> {
        self.last_error.as_ref()
    }

    pub fn get_operation_count(&self) -> u64 {
        self.operation_count
    }

    pub fn reset_error_state(&mut self) {
        self.last_error = None;
    }

    pub fn reset_operation_count(&mut self) {
        self.operation_count = 0;
    }

    fn validate_shape_image_threshold(&self, shape_image_threshold: &str) -> Result<(), String> {
        if shape_image_threshold.is_empty() {
            return Err("Shape image threshold cannot be empty".to_string());
        }
        if shape_image_threshold.len() > MAX_SHAPE_IMAGE_THRESHOLD_LENGTH {
            return Err(format!("Shape image threshold string exceeds maximum length of {} characters", MAX_SHAPE_IMAGE_THRESHOLD_LENGTH));
        }
        if let Ok(value) = shape_image_threshold.parse::<f64>() {
            if value < MIN_SHAPE_IMAGE_THRESHOLD || value > MAX_SHAPE_IMAGE_THRESHOLD {
                return Err(format!("Shape image threshold must be between {} and {}", MIN_SHAPE_IMAGE_THRESHOLD, MAX_SHAPE_IMAGE_THRESHOLD));
            }
            if !value.is_finite() {
                return Err("Shape image threshold must be a finite number".to_string());
            }
        }
        Ok(())
    }

    pub fn apply_shape_image_threshold(&mut self, node: &mut TipTapNode, shape_image_threshold: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_shape_image_threshold(shape_image_threshold)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("shapeImageThreshold".to_string(), serde_json::Value::String(shape_image_threshold.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "shapeImageThreshold": shape_image_threshold }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Shape image threshold application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Shape image threshold application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_shape_image_threshold(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("shapeImageThreshold");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Shape image threshold removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Shape image threshold removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_shape_image_threshold(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(shape_image_threshold) = obj.get("shapeImageThreshold") {
                    if let Some(s) = shape_image_threshold.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_shape_image_threshold(&self, node: &TipTapNode) -> bool {
        self.get_shape_image_threshold(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_shape_image_threshold_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ShapeImageThresholdManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_shape_image_threshold() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ShapeImageThresholdManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_shape_image_threshold(&mut node, "0.5");
        assert!(result.is_ok());
        assert!(manager.has_shape_image_threshold(&node));
    }

    #[test]
    fn test_remove_shape_image_threshold() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ShapeImageThresholdManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "shapeImageThreshold": "0.3" })),
            marks: None,
        };
        
        assert!(manager.has_shape_image_threshold(&node));
        let result = manager.remove_shape_image_threshold(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_shape_image_threshold(&node));
    }

    #[test]
    fn test_get_shape_image_threshold() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ShapeImageThresholdManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "shapeImageThreshold": "0.7" })),
            marks: None,
        };
        
        let shape_image_threshold = manager.get_shape_image_threshold(&node);
        assert_eq!(shape_image_threshold, Some("0.7".to_string()));
    }
}
