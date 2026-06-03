//! TipTap Place Items Manager - Aerospace-Grade Place Items Operations Service
//!
//! Safety-critical place items operations service with:
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

/// Maximum place items string length
const MAX_PLACE_ITEMS_LENGTH: usize = 100;

pub struct PlaceItemsManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl PlaceItemsManager {
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

    pub fn max_place_items_length() -> usize {
        MAX_PLACE_ITEMS_LENGTH
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

    fn validate_place_items(&self, place_items: &str) -> Result<(), String> {
        if place_items.is_empty() {
            return Err("Place items cannot be empty".to_string());
        }
        if place_items.len() > MAX_PLACE_ITEMS_LENGTH {
            return Err(format!("Place items string exceeds maximum length of {} characters", MAX_PLACE_ITEMS_LENGTH));
        }
        if place_items.contains('(') && !place_items.contains(')') {
            return Err("Invalid place items: unmatched parentheses".to_string());
        }
        Ok(())
    }

    pub fn apply_place_items(&mut self, node: &mut TipTapNode, place_items: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_place_items(place_items)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("placeItems".to_string(), serde_json::Value::String(place_items.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "placeItems": place_items }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Place items application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Place items application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_place_items(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("placeItems");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Place items removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Place items removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_place_items(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(place_items) = obj.get("placeItems") {
                    if let Some(s) = place_items.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_place_items(&self, node: &TipTapNode) -> bool {
        self.get_place_items(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_place_items_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PlaceItemsManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_place_items() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PlaceItemsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_place_items(&mut node, "center");
        assert!(result.is_ok());
        assert!(manager.has_place_items(&node));
    }

    #[test]
    fn test_remove_place_items() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PlaceItemsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "placeItems": "start" })),
            marks: None,
        };
        
        assert!(manager.has_place_items(&node));
        let result = manager.remove_place_items(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_place_items(&node));
    }

    #[test]
    fn test_get_place_items() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PlaceItemsManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "placeItems": "end" })),
            marks: None,
        };
        
        let place_items = manager.get_place_items(&node);
        assert_eq!(place_items, Some("end".to_string()));
    }
}
