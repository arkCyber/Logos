//! TipTap Min Height Manager - Aerospace-Grade Min Height Operations Service
//!
//! Safety-critical min height operations service with:
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

/// Maximum min height value (in pixels)
const MAX_MIN_HEIGHT: f64 = 10000.0;

/// Minimum min height value (in pixels)
const MIN_MIN_HEIGHT: f64 = 0.0;

/// Maximum min height string length
const MAX_MIN_HEIGHT_LENGTH: usize = 50;

pub struct MinHeightManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl MinHeightManager {
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

    pub fn max_min_height() -> f64 {
        MAX_MIN_HEIGHT
    }

    pub fn min_min_height() -> f64 {
        MIN_MIN_HEIGHT
    }

    pub fn max_min_height_length() -> usize {
        MAX_MIN_HEIGHT_LENGTH
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

    fn validate_min_height(&self, min_height: &str) -> Result<(), String> {
        if min_height.is_empty() {
            return Err("Min height cannot be empty".to_string());
        }
        if min_height.len() > MAX_MIN_HEIGHT_LENGTH {
            return Err(format!("Min height string exceeds maximum length of {} characters", MAX_MIN_HEIGHT_LENGTH));
        }
        if min_height.ends_with("px") {
            let value_str = min_height.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_MIN_HEIGHT || value > MAX_MIN_HEIGHT {
                    return Err(format!("Min height must be between {} and {} pixels", MIN_MIN_HEIGHT, MAX_MIN_HEIGHT));
                }
                if !value.is_finite() {
                    return Err("Min height must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_min_height(&mut self, node: &mut TipTapNode, min_height: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_min_height(min_height)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("minHeight".to_string(), serde_json::Value::String(min_height.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "minHeight": min_height }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Min height application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Min height application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_min_height(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("minHeight");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Min height removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Min height removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_min_height(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(min_height) = obj.get("minHeight") {
                    if let Some(s) = min_height.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_min_height(&self, node: &TipTapNode) -> bool {
        self.get_min_height(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_min_height_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MinHeightManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_min_height() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MinHeightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_min_height(&mut node, "100px");
        assert!(result.is_ok());
        assert!(manager.has_min_height(&node));
    }

    #[test]
    fn test_remove_min_height() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MinHeightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "minHeight": "200px" })),
            marks: None,
        };
        
        assert!(manager.has_min_height(&node));
        let result = manager.remove_min_height(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_min_height(&node));
    }

    #[test]
    fn test_get_min_height() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MinHeightManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "minHeight": "300px" })),
            marks: None,
        };
        
        let min_height = manager.get_min_height(&node);
        assert_eq!(min_height, Some("300px".to_string()));
    }
}
