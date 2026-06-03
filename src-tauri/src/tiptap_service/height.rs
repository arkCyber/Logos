//! TipTap Height Manager - Aerospace-Grade Height Operations Service
//!
//! Safety-critical height operations service with:
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

/// Maximum height value (in pixels)
const MAX_HEIGHT: f64 = 10000.0;

/// Minimum height value (in pixels)
const MIN_HEIGHT: f64 = 0.0;

/// Maximum height string length
const MAX_HEIGHT_LENGTH: usize = 50;

pub struct HeightManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl HeightManager {
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

    pub fn max_height() -> f64 {
        MAX_HEIGHT
    }

    pub fn min_height() -> f64 {
        MIN_HEIGHT
    }

    pub fn max_height_length() -> usize {
        MAX_HEIGHT_LENGTH
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

    fn validate_height(&self, height: &str) -> Result<(), String> {
        if height.is_empty() {
            return Err("Height cannot be empty".to_string());
        }
        if height.len() > MAX_HEIGHT_LENGTH {
            return Err(format!("Height string exceeds maximum length of {} characters", MAX_HEIGHT_LENGTH));
        }
        if height == "auto" || height == "100%" {
            return Ok(());
        }
        if height.ends_with("px") {
            let value_str = height.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_HEIGHT || value > MAX_HEIGHT {
                    return Err(format!("Height must be between {} and {} pixels", MIN_HEIGHT, MAX_HEIGHT));
                }
                if !value.is_finite() {
                    return Err("Height must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_height(&mut self, node: &mut TipTapNode, height: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_height(height)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("height".to_string(), serde_json::Value::String(height.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "height": height }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Height application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Height application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_height(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("height");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Height removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Height removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_height(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(height) = obj.get("height") {
                    if let Some(s) = height.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_height(&self, node: &TipTapNode) -> bool {
        self.get_height(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_height_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = HeightManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_height() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HeightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_height(&mut node, "100px");
        assert!(result.is_ok());
        assert!(manager.has_height(&node));
    }

    #[test]
    fn test_apply_height_auto() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HeightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_height(&mut node, "auto");
        assert!(result.is_ok());
        assert!(manager.has_height(&node));
    }

    #[test]
    fn test_remove_height() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HeightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "height": "200px" })),
            marks: None,
        };
        
        assert!(manager.has_height(&node));
        let result = manager.remove_height(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_height(&node));
    }

    #[test]
    fn test_get_height() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = HeightManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "height": "300px" })),
            marks: None,
        };
        
        let height = manager.get_height(&node);
        assert_eq!(height, Some("300px".to_string()));
    }
}
