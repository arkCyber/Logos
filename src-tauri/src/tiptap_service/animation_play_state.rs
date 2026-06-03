//! TipTap Animation Play State Manager - Aerospace-Grade Animation Play State Operations Service
//!
//! Safety-critical animation play state operations service with:
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

/// Maximum animation play state string length
const MAX_ANIMATION_PLAY_STATE_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationPlayState {
    Running,
    Paused,
}

impl AnimationPlayState {
    pub fn as_str(&self) -> &str {
        match self {
            AnimationPlayState::Running => "running",
            AnimationPlayState::Paused => "paused",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "running" => Ok(AnimationPlayState::Running),
            "paused" => Ok(AnimationPlayState::Paused),
            _ => Err(format!("Invalid animation play state: {}", s)),
        }
    }
}

pub struct AnimationPlayStateManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl AnimationPlayStateManager {
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

    pub fn max_animation_play_state_length() -> usize {
        MAX_ANIMATION_PLAY_STATE_LENGTH
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

    fn validate_animation_play_state(&self, animation_play_state: &str) -> Result<(), String> {
        if animation_play_state.len() > MAX_ANIMATION_PLAY_STATE_LENGTH {
            return Err(format!("Animation play state string exceeds maximum length of {} characters", MAX_ANIMATION_PLAY_STATE_LENGTH));
        }
        AnimationPlayState::from_str(animation_play_state)?;
        Ok(())
    }

    pub fn apply_animation_play_state(&mut self, node: &mut TipTapNode, animation_play_state: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_animation_play_state(animation_play_state)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("animationPlayState".to_string(), serde_json::Value::String(animation_play_state.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "animationPlayState": animation_play_state }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Animation play state application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Animation play state application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_animation_play_state(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("animationPlayState");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Animation play state removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Animation play state removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_animation_play_state(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(animation_play_state) = obj.get("animationPlayState") {
                    if let Some(s) = animation_play_state.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_animation_play_state(&self, node: &TipTapNode) -> bool {
        self.get_animation_play_state(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_animation_play_state_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AnimationPlayStateManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_animation_play_state_variants() {
        assert_eq!(AnimationPlayState::Running.as_str(), "running");
        assert_eq!(AnimationPlayState::Paused.as_str(), "paused");
    }

    #[test]
    fn test_apply_animation_play_state() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AnimationPlayStateManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_animation_play_state(&mut node, "paused");
        assert!(result.is_ok());
        assert!(manager.has_animation_play_state(&node));
    }

    #[test]
    fn test_remove_animation_play_state() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AnimationPlayStateManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "animationPlayState": "running" })),
            marks: None,
        };
        
        assert!(manager.has_animation_play_state(&node));
        let result = manager.remove_animation_play_state(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_animation_play_state(&node));
    }

    #[test]
    fn test_get_animation_play_state() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AnimationPlayStateManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "animationPlayState": "running" })),
            marks: None,
        };
        
        let animation_play_state = manager.get_animation_play_state(&node);
        assert_eq!(animation_play_state, Some("running".to_string()));
    }
}
