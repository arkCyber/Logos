//! TipTap Background Repeat Manager - Aerospace-Grade Background Repeat Operations Service
//!
//! Safety-critical background repeat operations service with:
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

/// Maximum background repeat string length
const MAX_BACKGROUND_REPEAT_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundRepeat {
    Repeat,
    RepeatX,
    RepeatY,
    NoRepeat,
    Space,
    Round,
}

impl BackgroundRepeat {
    pub fn as_str(&self) -> &str {
        match self {
            BackgroundRepeat::Repeat => "repeat",
            BackgroundRepeat::RepeatX => "repeat-x",
            BackgroundRepeat::RepeatY => "repeat-y",
            BackgroundRepeat::NoRepeat => "no-repeat",
            BackgroundRepeat::Space => "space",
            BackgroundRepeat::Round => "round",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "repeat" => Ok(BackgroundRepeat::Repeat),
            "repeat-x" => Ok(BackgroundRepeat::RepeatX),
            "repeat-y" => Ok(BackgroundRepeat::RepeatY),
            "no-repeat" => Ok(BackgroundRepeat::NoRepeat),
            "space" => Ok(BackgroundRepeat::Space),
            "round" => Ok(BackgroundRepeat::Round),
            _ => Err(format!("Invalid background repeat: {}", s)),
        }
    }
}

pub struct BackgroundRepeatManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl BackgroundRepeatManager {
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

    pub fn max_background_repeat_length() -> usize {
        MAX_BACKGROUND_REPEAT_LENGTH
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

    fn validate_background_repeat(&self, background_repeat: &str) -> Result<(), String> {
        if background_repeat.len() > MAX_BACKGROUND_REPEAT_LENGTH {
            return Err(format!("Background repeat string exceeds maximum length of {} characters", MAX_BACKGROUND_REPEAT_LENGTH));
        }
        BackgroundRepeat::from_str(background_repeat)?;
        Ok(())
    }

    pub fn apply_background_repeat(&mut self, node: &mut TipTapNode, background_repeat: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_background_repeat(background_repeat)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("backgroundRepeat".to_string(), serde_json::Value::String(background_repeat.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "backgroundRepeat": background_repeat }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Background repeat application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Background repeat application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_background_repeat(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("backgroundRepeat");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Background repeat removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Background repeat removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_background_repeat(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(background_repeat) = obj.get("backgroundRepeat") {
                    if let Some(s) = background_repeat.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_background_repeat(&self, node: &TipTapNode) -> bool {
        self.get_background_repeat(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_background_repeat_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BackgroundRepeatManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_background_repeat_variants() {
        assert_eq!(BackgroundRepeat::Repeat.as_str(), "repeat");
        assert_eq!(BackgroundRepeat::NoRepeat.as_str(), "no-repeat");
    }

    #[test]
    fn test_apply_background_repeat() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackgroundRepeatManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_background_repeat(&mut node, "no-repeat");
        assert!(result.is_ok());
        assert!(manager.has_background_repeat(&node));
    }

    #[test]
    fn test_remove_background_repeat() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackgroundRepeatManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "backgroundRepeat": "repeat" })),
            marks: None,
        };
        
        assert!(manager.has_background_repeat(&node));
        let result = manager.remove_background_repeat(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_background_repeat(&node));
    }

    #[test]
    fn test_get_background_repeat() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BackgroundRepeatManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "backgroundRepeat": "repeat-x" })),
            marks: None,
        };
        
        let background_repeat = manager.get_background_repeat(&node);
        assert_eq!(background_repeat, Some("repeat-x".to_string()));
    }
}
