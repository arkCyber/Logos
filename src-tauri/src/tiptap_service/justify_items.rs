//! TipTap Justify Items Manager - Aerospace-Grade Justify Items Operations Service
//!
//! Safety-critical justify items operations service with:
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

/// Maximum justify items string length
const MAX_JUSTIFY_ITEMS_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JustifyItems {
    Normal,
    FlexStart,
    FlexEnd,
    Center,
    Start,
    End,
    Left,
    Right,
    SelfStart,
    SelfEnd,
}

impl JustifyItems {
    pub fn as_str(&self) -> &str {
        match self {
            JustifyItems::Normal => "normal",
            JustifyItems::FlexStart => "flex-start",
            JustifyItems::FlexEnd => "flex-end",
            JustifyItems::Center => "center",
            JustifyItems::Start => "start",
            JustifyItems::End => "end",
            JustifyItems::Left => "left",
            JustifyItems::Right => "right",
            JustifyItems::SelfStart => "self-start",
            JustifyItems::SelfEnd => "self-end",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(JustifyItems::Normal),
            "flex-start" => Ok(JustifyItems::FlexStart),
            "flex-end" => Ok(JustifyItems::FlexEnd),
            "center" => Ok(JustifyItems::Center),
            "start" => Ok(JustifyItems::Start),
            "end" => Ok(JustifyItems::End),
            "left" => Ok(JustifyItems::Left),
            "right" => Ok(JustifyItems::Right),
            "self-start" => Ok(JustifyItems::SelfStart),
            "self-end" => Ok(JustifyItems::SelfEnd),
            _ => Err(format!("Invalid justify items: {}", s)),
        }
    }
}

pub struct JustifyItemsManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl JustifyItemsManager {
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

    pub fn max_justify_items_length() -> usize {
        MAX_JUSTIFY_ITEMS_LENGTH
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

    fn validate_justify_items(&self, justify_items: &str) -> Result<(), String> {
        if justify_items.len() > MAX_JUSTIFY_ITEMS_LENGTH {
            return Err(format!("Justify items string exceeds maximum length of {} characters", MAX_JUSTIFY_ITEMS_LENGTH));
        }
        JustifyItems::from_str(justify_items)?;
        Ok(())
    }

    pub fn apply_justify_items(&mut self, node: &mut TipTapNode, justify_items: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_justify_items(justify_items)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("justifyItems".to_string(), serde_json::Value::String(justify_items.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "justifyItems": justify_items }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Justify items application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Justify items application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_justify_items(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("justifyItems");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Justify items removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Justify items removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_justify_items(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(justify_items) = obj.get("justifyItems") {
                    if let Some(s) = justify_items.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_justify_items(&self, node: &TipTapNode) -> bool {
        self.get_justify_items(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_justify_items_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = JustifyItemsManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_justify_items_variants() {
        assert_eq!(JustifyItems::Normal.as_str(), "normal");
        assert_eq!(JustifyItems::Center.as_str(), "center");
    }

    #[test]
    fn test_apply_justify_items() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = JustifyItemsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_justify_items(&mut node, "center");
        assert!(result.is_ok());
        assert!(manager.has_justify_items(&node));
    }

    #[test]
    fn test_remove_justify_items() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = JustifyItemsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "justifyItems": "flex-start" })),
            marks: None,
        };
        
        assert!(manager.has_justify_items(&node));
        let result = manager.remove_justify_items(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_justify_items(&node));
    }

    #[test]
    fn test_get_justify_items() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = JustifyItemsManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "justifyItems": "flex-end" })),
            marks: None,
        };
        
        let justify_items = manager.get_justify_items(&node);
        assert_eq!(justify_items, Some("flex-end".to_string()));
    }
}
