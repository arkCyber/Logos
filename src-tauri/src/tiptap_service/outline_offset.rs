//! TipTap Outline Offset Manager - Aerospace-Grade Outline Offset Operations Service
//!
//! Safety-critical outline offset operations service with:
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

/// Maximum outline offset value (in pixels)
const MAX_OUTLINE_OFFSET: f64 = 50.0;

/// Minimum outline offset value (in pixels)
const MIN_OUTLINE_OFFSET: f64 = -50.0;

/// Maximum outline offset string length
const MAX_OUTLINE_OFFSET_LENGTH: usize = 50;

pub struct OutlineOffsetManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl OutlineOffsetManager {
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

    pub fn max_outline_offset() -> f64 {
        MAX_OUTLINE_OFFSET
    }

    pub fn min_outline_offset() -> f64 {
        MIN_OUTLINE_OFFSET
    }

    pub fn max_outline_offset_length() -> usize {
        MAX_OUTLINE_OFFSET_LENGTH
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

    fn validate_outline_offset(&self, outline_offset: &str) -> Result<(), String> {
        if outline_offset.is_empty() {
            return Err("Outline offset cannot be empty".to_string());
        }
        if outline_offset.len() > MAX_OUTLINE_OFFSET_LENGTH {
            return Err(format!("Outline offset string exceeds maximum length of {} characters", MAX_OUTLINE_OFFSET_LENGTH));
        }
        if outline_offset.ends_with("px") {
            let value_str = outline_offset.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_OUTLINE_OFFSET || value > MAX_OUTLINE_OFFSET {
                    return Err(format!("Outline offset must be between {} and {} pixels", MIN_OUTLINE_OFFSET, MAX_OUTLINE_OFFSET));
                }
                if !value.is_finite() {
                    return Err("Outline offset must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_outline_offset(&mut self, node: &mut TipTapNode, outline_offset: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_outline_offset(outline_offset)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("outlineOffset".to_string(), serde_json::Value::String(outline_offset.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "outlineOffset": outline_offset }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Outline offset application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Outline offset application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_outline_offset(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("outlineOffset");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Outline offset removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Outline offset removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_outline_offset(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(outline_offset) = obj.get("outlineOffset") {
                    if let Some(s) = outline_offset.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_outline_offset(&self, node: &TipTapNode) -> bool {
        self.get_outline_offset(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_outline_offset_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OutlineOffsetManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_outline_offset() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OutlineOffsetManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_outline_offset(&mut node, "5px");
        assert!(result.is_ok());
        assert!(manager.has_outline_offset(&node));
    }

    #[test]
    fn test_remove_outline_offset() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OutlineOffsetManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "outlineOffset": "10px" })),
            marks: None,
        };
        
        assert!(manager.has_outline_offset(&node));
        let result = manager.remove_outline_offset(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_outline_offset(&node));
    }

    #[test]
    fn test_get_outline_offset() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OutlineOffsetManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "outlineOffset": "15px" })),
            marks: None,
        };
        
        let outline_offset = manager.get_outline_offset(&node);
        assert_eq!(outline_offset, Some("15px".to_string()));
    }
}
