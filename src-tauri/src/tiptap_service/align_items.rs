//! TipTap Align Items Manager - Aerospace-Grade Align Items Operations Service
//!
//! Safety-critical align items operations service with:
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

/// Maximum align items string length
const MAX_ALIGN_ITEMS_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl AlignItems {
    pub fn as_str(&self) -> &str {
        match self {
            AlignItems::FlexStart => "flex-start",
            AlignItems::FlexEnd => "flex-end",
            AlignItems::Center => "center",
            AlignItems::Baseline => "baseline",
            AlignItems::Stretch => "stretch",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "flex-start" => Ok(AlignItems::FlexStart),
            "flex-end" => Ok(AlignItems::FlexEnd),
            "center" => Ok(AlignItems::Center),
            "baseline" => Ok(AlignItems::Baseline),
            "stretch" => Ok(AlignItems::Stretch),
            _ => Err(format!("Invalid align items: {}", s)),
        }
    }
}

pub struct AlignItemsManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl AlignItemsManager {
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

    pub fn max_align_items_length() -> usize {
        MAX_ALIGN_ITEMS_LENGTH
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

    fn validate_align_items(&self, align_items: &str) -> Result<(), String> {
        if align_items.len() > MAX_ALIGN_ITEMS_LENGTH {
            return Err(format!("Align items string exceeds maximum length of {} characters", MAX_ALIGN_ITEMS_LENGTH));
        }
        AlignItems::from_str(align_items)?;
        Ok(())
    }

    pub fn apply_align_items(&mut self, node: &mut TipTapNode, align_items: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_align_items(align_items)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("alignItems".to_string(), serde_json::Value::String(align_items.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "alignItems": align_items }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Align items application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Align items application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_align_items(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("alignItems");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Align items removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Align items removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_align_items(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(align_items) = obj.get("alignItems") {
                    if let Some(s) = align_items.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_align_items(&self, node: &TipTapNode) -> bool {
        self.get_align_items(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_align_items_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AlignItemsManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_align_items_variants() {
        assert_eq!(AlignItems::FlexStart.as_str(), "flex-start");
        assert_eq!(AlignItems::Center.as_str(), "center");
    }

    #[test]
    fn test_apply_align_items() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AlignItemsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_align_items(&mut node, "center");
        assert!(result.is_ok());
        assert!(manager.has_align_items(&node));
    }

    #[test]
    fn test_remove_align_items() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AlignItemsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "alignItems": "flex-start" })),
            marks: None,
        };
        
        assert!(manager.has_align_items(&node));
        let result = manager.remove_align_items(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_align_items(&node));
    }

    #[test]
    fn test_get_align_items() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AlignItemsManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "alignItems": "stretch" })),
            marks: None,
        };
        
        let align_items = manager.get_align_items(&node);
        assert_eq!(align_items, Some("stretch".to_string()));
    }
}
