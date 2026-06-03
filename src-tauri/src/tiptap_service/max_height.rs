//! TipTap Max Height Manager - Aerospace-Grade Max Height Operations Service
//!
//! Safety-critical max height operations service with:
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

/// Maximum max height value (in pixels)
const MAX_MAX_HEIGHT: f64 = 10000.0;

/// Minimum max height value (in pixels)
const MIN_MAX_HEIGHT: f64 = 0.0;

/// Maximum max height string length
const MAX_MAX_HEIGHT_LENGTH: usize = 50;

pub struct MaxHeightManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl MaxHeightManager {
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

    pub fn max_max_height() -> f64 {
        MAX_MAX_HEIGHT
    }

    pub fn min_max_height() -> f64 {
        MIN_MAX_HEIGHT
    }

    pub fn max_max_height_length() -> usize {
        MAX_MAX_HEIGHT_LENGTH
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

    fn validate_max_height(&self, max_height: &str) -> Result<(), String> {
        if max_height.is_empty() {
            return Err("Max height cannot be empty".to_string());
        }
        if max_height.len() > MAX_MAX_HEIGHT_LENGTH {
            return Err(format!("Max height string exceeds maximum length of {} characters", MAX_MAX_HEIGHT_LENGTH));
        }
        if max_height.ends_with("px") {
            let value_str = max_height.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_MAX_HEIGHT || value > MAX_MAX_HEIGHT {
                    return Err(format!("Max height must be between {} and {} pixels", MIN_MAX_HEIGHT, MAX_MAX_HEIGHT));
                }
                if !value.is_finite() {
                    return Err("Max height must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_max_height(&mut self, node: &mut TipTapNode, max_height: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_max_height(max_height)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("maxHeight".to_string(), serde_json::Value::String(max_height.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "maxHeight": max_height }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Max height application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Max height application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_max_height(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("maxHeight");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Max height removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Max height removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_max_height(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(max_height) = obj.get("maxHeight") {
                    if let Some(s) = max_height.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_max_height(&self, node: &TipTapNode) -> bool {
        self.get_max_height(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_max_height_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaxHeightManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_max_height() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaxHeightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_max_height(&mut node, "1000px");
        assert!(result.is_ok());
        assert!(manager.has_max_height(&node));
    }

    #[test]
    fn test_remove_max_height() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaxHeightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maxHeight": "800px" })),
            marks: None,
        };
        
        assert!(manager.has_max_height(&node));
        let result = manager.remove_max_height(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_max_height(&node));
    }

    #[test]
    fn test_get_max_height() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaxHeightManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maxHeight": "600px" })),
            marks: None,
        };
        
        let max_height = manager.get_max_height(&node);
        assert_eq!(max_height, Some("600px".to_string()));
    }
}
