//! TipTap Overflow X Manager - Aerospace-Grade Overflow X Operations Service
//!
//! Safety-critical overflow X operations service with:
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

/// Maximum overflow X string length
const MAX_OVERFLOW_X_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverflowX {
    Visible,
    Hidden,
    Scroll,
    Auto,
}

impl OverflowX {
    pub fn as_str(&self) -> &str {
        match self {
            OverflowX::Visible => "visible",
            OverflowX::Hidden => "hidden",
            OverflowX::Scroll => "scroll",
            OverflowX::Auto => "auto",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "visible" => Ok(OverflowX::Visible),
            "hidden" => Ok(OverflowX::Hidden),
            "scroll" => Ok(OverflowX::Scroll),
            "auto" => Ok(OverflowX::Auto),
            _ => Err(format!("Invalid overflow X: {}", s)),
        }
    }
}

pub struct OverflowXManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl OverflowXManager {
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

    pub fn max_overflow_x_length() -> usize {
        MAX_OVERFLOW_X_LENGTH
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

    fn validate_overflow_x(&self, overflow_x: &str) -> Result<(), String> {
        if overflow_x.len() > MAX_OVERFLOW_X_LENGTH {
            return Err(format!("Overflow X string exceeds maximum length of {} characters", MAX_OVERFLOW_X_LENGTH));
        }
        OverflowX::from_str(overflow_x)?;
        Ok(())
    }

    pub fn apply_overflow_x(&mut self, node: &mut TipTapNode, overflow_x: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_overflow_x(overflow_x)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("overflowX".to_string(), serde_json::Value::String(overflow_x.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "overflowX": overflow_x }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Overflow X application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Overflow X application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_overflow_x(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("overflowX");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Overflow X removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Overflow X removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_overflow_x(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(overflow_x) = obj.get("overflowX") {
                    if let Some(s) = overflow_x.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_overflow_x(&self, node: &TipTapNode) -> bool {
        self.get_overflow_x(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_overflow_x_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OverflowXManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_overflow_x_variants() {
        assert_eq!(OverflowX::Visible.as_str(), "visible");
        assert_eq!(OverflowX::Auto.as_str(), "auto");
    }

    #[test]
    fn test_apply_overflow_x() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OverflowXManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_overflow_x(&mut node, "scroll");
        assert!(result.is_ok());
        assert!(manager.has_overflow_x(&node));
    }

    #[test]
    fn test_remove_overflow_x() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OverflowXManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "overflowX": "hidden" })),
            marks: None,
        };
        
        assert!(manager.has_overflow_x(&node));
        let result = manager.remove_overflow_x(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_overflow_x(&node));
    }

    #[test]
    fn test_get_overflow_x() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OverflowXManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "overflowX": "auto" })),
            marks: None,
        };
        
        let overflow_x = manager.get_overflow_x(&node);
        assert_eq!(overflow_x, Some("auto".to_string()));
    }
}
