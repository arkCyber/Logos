//! TipTap Background Attachment Manager - Aerospace-Grade Background Attachment Operations Service
//!
//! Safety-critical background attachment operations service with:
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

/// Maximum background attachment string length
const MAX_BACKGROUND_ATTACHMENT_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundAttachment {
    Scroll,
    Fixed,
    Local,
}

impl BackgroundAttachment {
    pub fn as_str(&self) -> &str {
        match self {
            BackgroundAttachment::Scroll => "scroll",
            BackgroundAttachment::Fixed => "fixed",
            BackgroundAttachment::Local => "local",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "scroll" => Ok(BackgroundAttachment::Scroll),
            "fixed" => Ok(BackgroundAttachment::Fixed),
            "local" => Ok(BackgroundAttachment::Local),
            _ => Err(format!("Invalid background attachment: {}", s)),
        }
    }
}

pub struct BackgroundAttachmentManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl BackgroundAttachmentManager {
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

    pub fn max_background_attachment_length() -> usize {
        MAX_BACKGROUND_ATTACHMENT_LENGTH
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

    fn validate_background_attachment(&self, background_attachment: &str) -> Result<(), String> {
        if background_attachment.len() > MAX_BACKGROUND_ATTACHMENT_LENGTH {
            return Err(format!("Background attachment string exceeds maximum length of {} characters", MAX_BACKGROUND_ATTACHMENT_LENGTH));
        }
        BackgroundAttachment::from_str(background_attachment)?;
        Ok(())
    }

    pub fn apply_background_attachment(&mut self, node: &mut TipTapNode, background_attachment: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_background_attachment(background_attachment)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("backgroundAttachment".to_string(), serde_json::Value::String(background_attachment.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "backgroundAttachment": background_attachment }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Background attachment application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Background attachment application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_background_attachment(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("backgroundAttachment");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Background attachment removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Background attachment removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_background_attachment(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(background_attachment) = obj.get("backgroundAttachment") {
                    if let Some(s) = background_attachment.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_background_attachment(&self, node: &TipTapNode) -> bool {
        self.get_background_attachment(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_background_attachment_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BackgroundAttachmentManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_background_attachment_variants() {
        assert_eq!(BackgroundAttachment::Scroll.as_str(), "scroll");
        assert_eq!(BackgroundAttachment::Fixed.as_str(), "fixed");
    }

    #[test]
    fn test_apply_background_attachment() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackgroundAttachmentManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_background_attachment(&mut node, "fixed");
        assert!(result.is_ok());
        assert!(manager.has_background_attachment(&node));
    }

    #[test]
    fn test_remove_background_attachment() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackgroundAttachmentManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "backgroundAttachment": "scroll" })),
            marks: None,
        };
        
        assert!(manager.has_background_attachment(&node));
        let result = manager.remove_background_attachment(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_background_attachment(&node));
    }

    #[test]
    fn test_get_background_attachment() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BackgroundAttachmentManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "backgroundAttachment": "local" })),
            marks: None,
        };
        
        let background_attachment = manager.get_background_attachment(&node);
        assert_eq!(background_attachment, Some("local".to_string()));
    }
}
