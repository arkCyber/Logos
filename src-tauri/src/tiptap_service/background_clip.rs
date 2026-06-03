//! TipTap Background Clip Manager - Aerospace-Grade Background Clip Operations Service
//!
//! Safety-critical background clip operations service with:
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

/// Maximum background clip string length
const MAX_BACKGROUND_CLIP_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundClip {
    BorderBox,
    PaddingBox,
    ContentBox,
    Text,
}

impl BackgroundClip {
    pub fn as_str(&self) -> &str {
        match self {
            BackgroundClip::BorderBox => "border-box",
            BackgroundClip::PaddingBox => "padding-box",
            BackgroundClip::ContentBox => "content-box",
            BackgroundClip::Text => "text",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "border-box" => Ok(BackgroundClip::BorderBox),
            "padding-box" => Ok(BackgroundClip::PaddingBox),
            "content-box" => Ok(BackgroundClip::ContentBox),
            "text" => Ok(BackgroundClip::Text),
            _ => Err(format!("Invalid background clip: {}", s)),
        }
    }
}

pub struct BackgroundClipManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl BackgroundClipManager {
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

    pub fn max_background_clip_length() -> usize {
        MAX_BACKGROUND_CLIP_LENGTH
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

    fn validate_background_clip(&self, background_clip: &str) -> Result<(), String> {
        if background_clip.len() > MAX_BACKGROUND_CLIP_LENGTH {
            return Err(format!("Background clip string exceeds maximum length of {} characters", MAX_BACKGROUND_CLIP_LENGTH));
        }
        BackgroundClip::from_str(background_clip)?;
        Ok(())
    }

    pub fn apply_background_clip(&mut self, node: &mut TipTapNode, background_clip: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_background_clip(background_clip)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("backgroundClip".to_string(), serde_json::Value::String(background_clip.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "backgroundClip": background_clip }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Background clip application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Background clip application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_background_clip(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("backgroundClip");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Background clip removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Background clip removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_background_clip(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(background_clip) = obj.get("backgroundClip") {
                    if let Some(s) = background_clip.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_background_clip(&self, node: &TipTapNode) -> bool {
        self.get_background_clip(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_background_clip_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BackgroundClipManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_background_clip_variants() {
        assert_eq!(BackgroundClip::BorderBox.as_str(), "border-box");
        assert_eq!(BackgroundClip::Text.as_str(), "text");
    }

    #[test]
    fn test_apply_background_clip() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackgroundClipManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_background_clip(&mut node, "content-box");
        assert!(result.is_ok());
        assert!(manager.has_background_clip(&node));
    }

    #[test]
    fn test_remove_background_clip() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackgroundClipManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "backgroundClip": "border-box" })),
            marks: None,
        };
        
        assert!(manager.has_background_clip(&node));
        let result = manager.remove_background_clip(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_background_clip(&node));
    }

    #[test]
    fn test_get_background_clip() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BackgroundClipManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "backgroundClip": "padding-box" })),
            marks: None,
        };
        
        let background_clip = manager.get_background_clip(&node);
        assert_eq!(background_clip, Some("padding-box".to_string()));
    }
}
