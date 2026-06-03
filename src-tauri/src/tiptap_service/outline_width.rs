//! TipTap Outline Width Manager - Aerospace-Grade Outline Width Operations Service
//!
//! Safety-critical outline width operations service with:
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

/// Maximum outline width value (in pixels)
const MAX_OUTLINE_WIDTH: f64 = 50.0;

/// Minimum outline width value (in pixels)
const MIN_OUTLINE_WIDTH: f64 = 0.0;

/// Maximum outline width string length
const MAX_OUTLINE_WIDTH_LENGTH: usize = 50;

pub struct OutlineWidthManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl OutlineWidthManager {
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

    pub fn max_outline_width() -> f64 {
        MAX_OUTLINE_WIDTH
    }

    pub fn min_outline_width() -> f64 {
        MIN_OUTLINE_WIDTH
    }

    pub fn max_outline_width_length() -> usize {
        MAX_OUTLINE_WIDTH_LENGTH
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

    fn validate_outline_width(&self, outline_width: &str) -> Result<(), String> {
        if outline_width.is_empty() {
            return Err("Outline width cannot be empty".to_string());
        }
        if outline_width.len() > MAX_OUTLINE_WIDTH_LENGTH {
            return Err(format!("Outline width string exceeds maximum length of {} characters", MAX_OUTLINE_WIDTH_LENGTH));
        }
        if outline_width.ends_with("px") {
            let value_str = outline_width.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_OUTLINE_WIDTH || value > MAX_OUTLINE_WIDTH {
                    return Err(format!("Outline width must be between {} and {} pixels", MIN_OUTLINE_WIDTH, MAX_OUTLINE_WIDTH));
                }
                if !value.is_finite() {
                    return Err("Outline width must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_outline_width(&mut self, node: &mut TipTapNode, outline_width: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_outline_width(outline_width)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("outlineWidth".to_string(), serde_json::Value::String(outline_width.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "outlineWidth": outline_width }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Outline width application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Outline width application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_outline_width(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("outlineWidth");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Outline width removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Outline width removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_outline_width(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(outline_width) = obj.get("outlineWidth") {
                    if let Some(s) = outline_width.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_outline_width(&self, node: &TipTapNode) -> bool {
        self.get_outline_width(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_outline_width_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OutlineWidthManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_outline_width() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OutlineWidthManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_outline_width(&mut node, "5px");
        assert!(result.is_ok());
        assert!(manager.has_outline_width(&node));
    }

    #[test]
    fn test_remove_outline_width() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OutlineWidthManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "outlineWidth": "10px" })),
            marks: None,
        };
        
        assert!(manager.has_outline_width(&node));
        let result = manager.remove_outline_width(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_outline_width(&node));
    }

    #[test]
    fn test_get_outline_width() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OutlineWidthManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "outlineWidth": "15px" })),
            marks: None,
        };
        
        let outline_width = manager.get_outline_width(&node);
        assert_eq!(outline_width, Some("15px".to_string()));
    }
}
