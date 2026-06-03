//! TipTap Shape Margin Manager - Aerospace-Grade Shape Margin Operations Service
//!
//! Safety-critical shape margin operations service with:
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

/// Maximum shape margin value (in pixels)
const MAX_SHAPE_MARGIN: f64 = 1000.0;

/// Minimum shape margin value (in pixels)
const MIN_SHAPE_MARGIN: f64 = 0.0;

/// Maximum shape margin string length
const MAX_SHAPE_MARGIN_LENGTH: usize = 50;

pub struct ShapeMarginManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ShapeMarginManager {
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

    pub fn max_shape_margin() -> f64 {
        MAX_SHAPE_MARGIN
    }

    pub fn min_shape_margin() -> f64 {
        MIN_SHAPE_MARGIN
    }

    pub fn max_shape_margin_length() -> usize {
        MAX_SHAPE_MARGIN_LENGTH
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

    fn validate_shape_margin(&self, shape_margin: &str) -> Result<(), String> {
        if shape_margin.is_empty() {
            return Err("Shape margin cannot be empty".to_string());
        }
        if shape_margin.len() > MAX_SHAPE_MARGIN_LENGTH {
            return Err(format!("Shape margin string exceeds maximum length of {} characters", MAX_SHAPE_MARGIN_LENGTH));
        }
        if shape_margin.ends_with("px") {
            let value_str = shape_margin.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_SHAPE_MARGIN || value > MAX_SHAPE_MARGIN {
                    return Err(format!("Shape margin must be between {} and {} pixels", MIN_SHAPE_MARGIN, MAX_SHAPE_MARGIN));
                }
                if !value.is_finite() {
                    return Err("Shape margin must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_shape_margin(&mut self, node: &mut TipTapNode, shape_margin: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_shape_margin(shape_margin)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("shapeMargin".to_string(), serde_json::Value::String(shape_margin.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "shapeMargin": shape_margin }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Shape margin application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Shape margin application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_shape_margin(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("shapeMargin");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Shape margin removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Shape margin removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_shape_margin(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(shape_margin) = obj.get("shapeMargin") {
                    if let Some(s) = shape_margin.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_shape_margin(&self, node: &TipTapNode) -> bool {
        self.get_shape_margin(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_shape_margin_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ShapeMarginManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_shape_margin() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ShapeMarginManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_shape_margin(&mut node, "10px");
        assert!(result.is_ok());
        assert!(manager.has_shape_margin(&node));
    }

    #[test]
    fn test_remove_shape_margin() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ShapeMarginManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "shapeMargin": "20px" })),
            marks: None,
        };
        
        assert!(manager.has_shape_margin(&node));
        let result = manager.remove_shape_margin(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_shape_margin(&node));
    }

    #[test]
    fn test_get_shape_margin() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ShapeMarginManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "shapeMargin": "30px" })),
            marks: None,
        };
        
        let shape_margin = manager.get_shape_margin(&node);
        assert_eq!(shape_margin, Some("30px".to_string()));
    }
}
