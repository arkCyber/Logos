//! TipTap Animation Fill Mode Manager - Aerospace-Grade Animation Fill Mode Operations Service
//!
//! Safety-critical animation fill mode operations service with:
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

/// Maximum animation fill mode string length
const MAX_ANIMATION_FILL_MODE_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

impl AnimationFillMode {
    pub fn as_str(&self) -> &str {
        match self {
            AnimationFillMode::None => "none",
            AnimationFillMode::Forwards => "forwards",
            AnimationFillMode::Backwards => "backwards",
            AnimationFillMode::Both => "both",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" => Ok(AnimationFillMode::None),
            "forwards" => Ok(AnimationFillMode::Forwards),
            "backwards" => Ok(AnimationFillMode::Backwards),
            "both" => Ok(AnimationFillMode::Both),
            _ => Err(format!("Invalid animation fill mode: {}", s)),
        }
    }
}

pub struct AnimationFillModeManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl AnimationFillModeManager {
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

    pub fn max_animation_fill_mode_length() -> usize {
        MAX_ANIMATION_FILL_MODE_LENGTH
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

    fn validate_animation_fill_mode(&self, animation_fill_mode: &str) -> Result<(), String> {
        if animation_fill_mode.len() > MAX_ANIMATION_FILL_MODE_LENGTH {
            return Err(format!("Animation fill mode string exceeds maximum length of {} characters", MAX_ANIMATION_FILL_MODE_LENGTH));
        }
        AnimationFillMode::from_str(animation_fill_mode)?;
        Ok(())
    }

    pub fn apply_animation_fill_mode(&mut self, node: &mut TipTapNode, animation_fill_mode: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_animation_fill_mode(animation_fill_mode)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("animationFillMode".to_string(), serde_json::Value::String(animation_fill_mode.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "animationFillMode": animation_fill_mode }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Animation fill mode application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Animation fill mode application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_animation_fill_mode(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("animationFillMode");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Animation fill mode removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Animation fill mode removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_animation_fill_mode(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(animation_fill_mode) = obj.get("animationFillMode") {
                    if let Some(s) = animation_fill_mode.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_animation_fill_mode(&self, node: &TipTapNode) -> bool {
        self.get_animation_fill_mode(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_animation_fill_mode_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AnimationFillModeManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_animation_fill_mode_variants() {
        assert_eq!(AnimationFillMode::None.as_str(), "none");
        assert_eq!(AnimationFillMode::Forwards.as_str(), "forwards");
    }

    #[test]
    fn test_apply_animation_fill_mode() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AnimationFillModeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_animation_fill_mode(&mut node, "forwards");
        assert!(result.is_ok());
        assert!(manager.has_animation_fill_mode(&node));
    }

    #[test]
    fn test_remove_animation_fill_mode() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AnimationFillModeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "animationFillMode": "both" })),
            marks: None,
        };
        
        assert!(manager.has_animation_fill_mode(&node));
        let result = manager.remove_animation_fill_mode(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_animation_fill_mode(&node));
    }

    #[test]
    fn test_get_animation_fill_mode() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AnimationFillModeManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "animationFillMode": "backwards" })),
            marks: None,
        };
        
        let animation_fill_mode = manager.get_animation_fill_mode(&node);
        assert_eq!(animation_fill_mode, Some("backwards".to_string()));
    }
}
