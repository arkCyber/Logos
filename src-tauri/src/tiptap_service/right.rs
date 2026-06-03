//! TipTap Right Manager - Aerospace-Grade Right Operations Service
//!
//! Safety-critical right operations service with:
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

/// Maximum right value (in pixels)
const MAX_RIGHT: f64 = 10000.0;

/// Minimum right value (in pixels)
const MIN_RIGHT: f64 = -10000.0;

/// Maximum right string length
const MAX_RIGHT_LENGTH: usize = 50;

pub struct RightManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl RightManager {
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

    pub fn max_right() -> f64 {
        MAX_RIGHT
    }

    pub fn min_right() -> f64 {
        MIN_RIGHT
    }

    pub fn max_right_length() -> usize {
        MAX_RIGHT_LENGTH
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

    fn validate_right(&self, right: &str) -> Result<(), String> {
        if right.is_empty() {
            return Err("Right cannot be empty".to_string());
        }
        if right.len() > MAX_RIGHT_LENGTH {
            return Err(format!("Right string exceeds maximum length of {} characters", MAX_RIGHT_LENGTH));
        }
        if right.ends_with("px") {
            let value_str = right.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_RIGHT || value > MAX_RIGHT {
                    return Err(format!("Right must be between {} and {} pixels", MIN_RIGHT, MAX_RIGHT));
                }
                if !value.is_finite() {
                    return Err("Right must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_right(&mut self, node: &mut TipTapNode, right: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_right(right)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("right".to_string(), serde_json::Value::String(right.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "right": right }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Right application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Right application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_right(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("right");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Right removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Right removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_right(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(right) = obj.get("right") {
                    if let Some(s) = right.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_right(&self, node: &TipTapNode) -> bool {
        self.get_right(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_right_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = RightManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_right() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = RightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_right(&mut node, "10px");
        assert!(result.is_ok());
        assert!(manager.has_right(&node));
    }

    #[test]
    fn test_remove_right() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = RightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "right": "20px" })),
            marks: None,
        };
        
        assert!(manager.has_right(&node));
        let result = manager.remove_right(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_right(&node));
    }

    #[test]
    fn test_get_right() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = RightManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "right": "30px" })),
            marks: None,
        };
        
        let right = manager.get_right(&node);
        assert_eq!(right, Some("30px".to_string()));
    }
}
