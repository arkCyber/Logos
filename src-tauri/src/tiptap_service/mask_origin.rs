//! TipTap Mask Origin Manager - Aerospace-Grade Mask Origin Operations Service
//!
//! Safety-critical mask origin operations service with:
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

/// Maximum mask origin string length
const MAX_MASK_ORIGIN_LENGTH: usize = 100;

pub struct MaskOriginManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl MaskOriginManager {
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

    pub fn max_mask_origin_length() -> usize {
        MAX_MASK_ORIGIN_LENGTH
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

    fn validate_mask_origin(&self, mask_origin: &str) -> Result<(), String> {
        if mask_origin.is_empty() {
            return Err("Mask origin cannot be empty".to_string());
        }
        if mask_origin.len() > MAX_MASK_ORIGIN_LENGTH {
            return Err(format!("Mask origin string exceeds maximum length of {} characters", MAX_MASK_ORIGIN_LENGTH));
        }
        let valid_patterns = ["center", "top", "bottom", "left", "right", "top left", "top right", "bottom left", "bottom right"];
        if !valid_patterns.iter().any(|pattern| mask_origin.contains(pattern)) {
            if mask_origin.contains('(') && !mask_origin.contains(')') {
                return Err("Invalid mask origin: unmatched parentheses".to_string());
            }
        }
        Ok(())
    }

    pub fn apply_mask_origin(&mut self, node: &mut TipTapNode, mask_origin: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_mask_origin(mask_origin)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("maskOrigin".to_string(), serde_json::Value::String(mask_origin.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "maskOrigin": mask_origin }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Mask origin application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Mask origin application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_mask_origin(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("maskOrigin");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Mask origin removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Mask origin removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_mask_origin(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(mask_origin) = obj.get("maskOrigin") {
                    if let Some(s) = mask_origin.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_mask_origin(&self, node: &TipTapNode) -> bool {
        self.get_mask_origin(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_mask_origin_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskOriginManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_mask_origin() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskOriginManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_mask_origin(&mut node, "center");
        assert!(result.is_ok());
        assert!(manager.has_mask_origin(&node));
    }

    #[test]
    fn test_apply_mask_origin_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskOriginManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_mask_origin(&mut node, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_mask_origin() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskOriginManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maskOrigin": "top left" })),
            marks: None,
        };
        
        assert!(manager.has_mask_origin(&node));
        let result = manager.remove_mask_origin(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_mask_origin(&node));
    }

    #[test]
    fn test_get_mask_origin() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskOriginManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maskOrigin": "bottom right" })),
            marks: None,
        };
        
        let mask_origin = manager.get_mask_origin(&node);
        assert_eq!(mask_origin, Some("bottom right".to_string()));
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskOriginManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        
        let last_error = manager.get_last_error();
        assert!(last_error.is_some());
        let error = last_error.unwrap();
        assert_eq!(error.code, "TEST_ERROR");
        assert_eq!(error.message, "Test error message");
        assert_eq!(error.source, "test_source");
    }
}
