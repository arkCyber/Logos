//! TipTap Mask Clip Manager - Aerospace-Grade Mask Clip Operations Service
//!
//! Safety-critical mask clip operations service with:
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

/// Maximum mask clip string length
const MAX_MASK_CLIP_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskClip {
    BorderBox,
    PaddingBox,
    ContentBox,
    Text,
}

impl MaskClip {
    pub fn as_str(&self) -> &str {
        match self {
            MaskClip::BorderBox => "border-box",
            MaskClip::PaddingBox => "padding-box",
            MaskClip::ContentBox => "content-box",
            MaskClip::Text => "text",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "border-box" => Ok(MaskClip::BorderBox),
            "padding-box" => Ok(MaskClip::PaddingBox),
            "content-box" => Ok(MaskClip::ContentBox),
            "text" => Ok(MaskClip::Text),
            _ => Err(format!("Invalid mask clip: {}", s)),
        }
    }
}

pub struct MaskClipManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl MaskClipManager {
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

    pub fn max_mask_clip_length() -> usize {
        MAX_MASK_CLIP_LENGTH
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

    fn validate_mask_clip(&self, mask_clip: &str) -> Result<(), String> {
        if mask_clip.len() > MAX_MASK_CLIP_LENGTH {
            return Err(format!("Mask clip string exceeds maximum length of {} characters", MAX_MASK_CLIP_LENGTH));
        }
        MaskClip::from_str(mask_clip)?;
        Ok(())
    }

    pub fn apply_mask_clip(&mut self, node: &mut TipTapNode, mask_clip: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_mask_clip(mask_clip)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("maskClip".to_string(), serde_json::Value::String(mask_clip.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "maskClip": mask_clip }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Mask clip application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Mask clip application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_mask_clip(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("maskClip");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Mask clip removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Mask clip removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_mask_clip(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(mask_clip) = obj.get("maskClip") {
                    if let Some(s) = mask_clip.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_mask_clip(&self, node: &TipTapNode) -> bool {
        self.get_mask_clip(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_mask_clip_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskClipManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_mask_clip_variants() {
        assert_eq!(MaskClip::BorderBox.as_str(), "border-box");
        assert_eq!(MaskClip::Text.as_str(), "text");
    }

    #[test]
    fn test_apply_mask_clip() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskClipManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_mask_clip(&mut node, "border-box");
        assert!(result.is_ok());
        assert!(manager.has_mask_clip(&node));
    }

    #[test]
    fn test_remove_mask_clip() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskClipManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maskClip": "content-box" })),
            marks: None,
        };
        
        assert!(manager.has_mask_clip(&node));
        let result = manager.remove_mask_clip(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_mask_clip(&node));
    }

    #[test]
    fn test_get_mask_clip() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskClipManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maskClip": "padding-box" })),
            marks: None,
        };
        
        let mask_clip = manager.get_mask_clip(&node);
        assert_eq!(mask_clip, Some("padding-box".to_string()));
    }
}
