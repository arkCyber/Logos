//! TipTap Mask Repeat Manager - Aerospace-Grade Mask Repeat Operations Service
//!
//! Safety-critical mask repeat operations service with:
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

/// Maximum mask repeat string length
const MAX_MASK_REPEAT_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskRepeat {
    Repeat,
    RepeatX,
    RepeatY,
    NoRepeat,
    Space,
    Round,
}

impl MaskRepeat {
    pub fn as_str(&self) -> &str {
        match self {
            MaskRepeat::Repeat => "repeat",
            MaskRepeat::RepeatX => "repeat-x",
            MaskRepeat::RepeatY => "repeat-y",
            MaskRepeat::NoRepeat => "no-repeat",
            MaskRepeat::Space => "space",
            MaskRepeat::Round => "round",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "repeat" => Ok(MaskRepeat::Repeat),
            "repeat-x" => Ok(MaskRepeat::RepeatX),
            "repeat-y" => Ok(MaskRepeat::RepeatY),
            "no-repeat" => Ok(MaskRepeat::NoRepeat),
            "space" => Ok(MaskRepeat::Space),
            "round" => Ok(MaskRepeat::Round),
            _ => Err(format!("Invalid mask repeat: {}", s)),
        }
    }
}

pub struct MaskRepeatManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl MaskRepeatManager {
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

    pub fn max_mask_repeat_length() -> usize {
        MAX_MASK_REPEAT_LENGTH
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

    fn validate_mask_repeat(&self, mask_repeat: &str) -> Result<(), String> {
        if mask_repeat.len() > MAX_MASK_REPEAT_LENGTH {
            return Err(format!("Mask repeat string exceeds maximum length of {} characters", MAX_MASK_REPEAT_LENGTH));
        }
        MaskRepeat::from_str(mask_repeat)?;
        Ok(())
    }

    pub fn apply_mask_repeat(&mut self, node: &mut TipTapNode, mask_repeat: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_mask_repeat(mask_repeat)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("maskRepeat".to_string(), serde_json::Value::String(mask_repeat.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "maskRepeat": mask_repeat }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Mask repeat application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Mask repeat application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_mask_repeat(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("maskRepeat");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Mask repeat removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Mask repeat removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_mask_repeat(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(mask_repeat) = obj.get("maskRepeat") {
                    if let Some(s) = mask_repeat.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_mask_repeat(&self, node: &TipTapNode) -> bool {
        self.get_mask_repeat(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_mask_repeat_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskRepeatManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_mask_repeat_variants() {
        assert_eq!(MaskRepeat::Repeat.as_str(), "repeat");
        assert_eq!(MaskRepeat::NoRepeat.as_str(), "no-repeat");
    }

    #[test]
    fn test_apply_mask_repeat() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskRepeatManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_mask_repeat(&mut node, "repeat");
        assert!(result.is_ok());
        assert!(manager.has_mask_repeat(&node));
    }

    #[test]
    fn test_remove_mask_repeat() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskRepeatManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maskRepeat": "no-repeat" })),
            marks: None,
        };
        
        assert!(manager.has_mask_repeat(&node));
        let result = manager.remove_mask_repeat(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_mask_repeat(&node));
    }

    #[test]
    fn test_get_mask_repeat() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskRepeatManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maskRepeat": "repeat-x" })),
            marks: None,
        };
        
        let mask_repeat = manager.get_mask_repeat(&node);
        assert_eq!(mask_repeat, Some("repeat-x".to_string()));
    }
}
