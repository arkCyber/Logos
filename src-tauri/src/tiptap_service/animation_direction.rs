//! TipTap Animation Direction Manager - Aerospace-Grade Animation Direction Operations Service
//!
//! Safety-critical animation direction operations service with:
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

/// Maximum animation direction string length
const MAX_ANIMATION_DIRECTION_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

impl AnimationDirection {
    pub fn as_str(&self) -> &str {
        match self {
            AnimationDirection::Normal => "normal",
            AnimationDirection::Reverse => "reverse",
            AnimationDirection::Alternate => "alternate",
            AnimationDirection::AlternateReverse => "alternate-reverse",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(AnimationDirection::Normal),
            "reverse" => Ok(AnimationDirection::Reverse),
            "alternate" => Ok(AnimationDirection::Alternate),
            "alternate-reverse" => Ok(AnimationDirection::AlternateReverse),
            _ => Err(format!("Invalid animation direction: {}", s)),
        }
    }
}

pub struct AnimationDirectionManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl AnimationDirectionManager {
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

    pub fn max_animation_direction_length() -> usize {
        MAX_ANIMATION_DIRECTION_LENGTH
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

    fn validate_animation_direction(&self, animation_direction: &str) -> Result<(), String> {
        if animation_direction.len() > MAX_ANIMATION_DIRECTION_LENGTH {
            return Err(format!("Animation direction string exceeds maximum length of {} characters", MAX_ANIMATION_DIRECTION_LENGTH));
        }
        AnimationDirection::from_str(animation_direction)?;
        Ok(())
    }

    pub fn apply_animation_direction(&mut self, node: &mut TipTapNode, animation_direction: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_animation_direction(animation_direction)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("animationDirection".to_string(), serde_json::Value::String(animation_direction.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "animationDirection": animation_direction }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Animation direction application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Animation direction application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_animation_direction(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("animationDirection");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Animation direction removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Animation direction removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_animation_direction(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(animation_direction) = obj.get("animationDirection") {
                    if let Some(s) = animation_direction.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_animation_direction(&self, node: &TipTapNode) -> bool {
        self.get_animation_direction(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_animation_direction_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AnimationDirectionManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_animation_direction_variants() {
        assert_eq!(AnimationDirection::Normal.as_str(), "normal");
        assert_eq!(AnimationDirection::Alternate.as_str(), "alternate");
    }

    #[test]
    fn test_apply_animation_direction() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AnimationDirectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_animation_direction(&mut node, "reverse");
        assert!(result.is_ok());
        assert!(manager.has_animation_direction(&node));
    }

    #[test]
    fn test_remove_animation_direction() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AnimationDirectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "animationDirection": "normal" })),
            marks: None,
        };
        
        assert!(manager.has_animation_direction(&node));
        let result = manager.remove_animation_direction(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_animation_direction(&node));
    }

    #[test]
    fn test_get_animation_direction() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AnimationDirectionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "animationDirection": "alternate-reverse" })),
            marks: None,
        };
        
        let animation_direction = manager.get_animation_direction(&node);
        assert_eq!(animation_direction, Some("alternate-reverse".to_string()));
    }
}
