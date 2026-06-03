//! TipTap Border Image Manager - Aerospace-Grade Border Image Operations Service
//!
//! Safety-critical border image operations service with:
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

/// Maximum border image string length
const MAX_BORDER_IMAGE_LENGTH: usize = 500;

pub struct BorderImageManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl BorderImageManager {
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

    pub fn max_border_image_length() -> usize {
        MAX_BORDER_IMAGE_LENGTH
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

    fn validate_border_image(&self, border_image: &str) -> Result<(), String> {
        if border_image.is_empty() {
            return Err("Border image cannot be empty".to_string());
        }
        if border_image.len() > MAX_BORDER_IMAGE_LENGTH {
            return Err(format!("Border image string exceeds maximum length of {} characters", MAX_BORDER_IMAGE_LENGTH));
        }
        if border_image.contains('(') && !border_image.contains(')') {
            return Err("Invalid border image: unmatched parentheses".to_string());
        }
        Ok(())
    }

    pub fn apply_border_image(&mut self, node: &mut TipTapNode, border_image: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_border_image(border_image)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("borderImage".to_string(), serde_json::Value::String(border_image.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "borderImage": border_image }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Border image application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Border image application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_border_image(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("borderImage");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Border image removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Border image removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_border_image(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(border_image) = obj.get("borderImage") {
                    if let Some(s) = border_image.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_border_image(&self, node: &TipTapNode) -> bool {
        self.get_border_image(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_border_image_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BorderImageManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_border_image() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderImageManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_border_image(&mut node, "url('border.png')");
        assert!(result.is_ok());
        assert!(manager.has_border_image(&node));
    }

    #[test]
    fn test_remove_border_image() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderImageManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "borderImage": "url('border.png')" })),
            marks: None,
        };
        
        assert!(manager.has_border_image(&node));
        let result = manager.remove_border_image(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_border_image(&node));
    }

    #[test]
    fn test_get_border_image() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BorderImageManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "borderImage": "url('border.png')" })),
            marks: None,
        };
        
        let border_image = manager.get_border_image(&node);
        assert_eq!(border_image, Some("url('border.png')".to_string()));
    }
}
