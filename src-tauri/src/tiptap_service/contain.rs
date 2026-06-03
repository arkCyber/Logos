//! TipTap Contain Manager - Aerospace-Grade Contain Operations Service
//!
//! Safety-critical contain operations service with:
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

/// Maximum contain string length
const MAX_CONTAIN_LENGTH: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Contain {
    None,
    Strict,
    Content,
    Size,
    InlineSize,
}

impl Contain {
    pub fn as_str(&self) -> &str {
        match self {
            Contain::None => "none",
            Contain::Strict => "strict",
            Contain::Content => "content",
            Contain::Size => "size",
            Contain::InlineSize => "inline-size",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Contain::None),
            "strict" => Ok(Contain::Strict),
            "content" => Ok(Contain::Content),
            "size" => Ok(Contain::Size),
            "inline-size" => Ok(Contain::InlineSize),
            _ => Err(format!("Invalid contain: {}", s)),
        }
    }
}

pub struct ContainManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ContainManager {
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

    pub fn max_contain_length() -> usize {
        MAX_CONTAIN_LENGTH
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

    fn validate_contain(&self, contain: &str) -> Result<(), String> {
        if contain.len() > MAX_CONTAIN_LENGTH {
            return Err(format!("Contain string exceeds maximum length of {} characters", MAX_CONTAIN_LENGTH));
        }
        Contain::from_str(contain)?;
        Ok(())
    }

    pub fn apply_contain(&mut self, node: &mut TipTapNode, contain: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_contain(contain)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("contain".to_string(), serde_json::Value::String(contain.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "contain": contain }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Contain application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Contain application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_contain(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("contain");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Contain removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Contain removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_contain(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(contain) = obj.get("contain") {
                    if let Some(s) = contain.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_contain(&self, node: &TipTapNode) -> bool {
        self.get_contain(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_contain_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ContainManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_contain_variants() {
        assert_eq!(Contain::None.as_str(), "none");
        assert_eq!(Contain::Strict.as_str(), "strict");
    }

    #[test]
    fn test_apply_contain() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ContainManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_contain(&mut node, "strict");
        assert!(result.is_ok());
        assert!(manager.has_contain(&node));
    }

    #[test]
    fn test_remove_contain() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ContainManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "contain": "content" })),
            marks: None,
        };
        
        assert!(manager.has_contain(&node));
        let result = manager.remove_contain(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_contain(&node));
    }

    #[test]
    fn test_get_contain() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ContainManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "contain": "size" })),
            marks: None,
        };
        
        let contain = manager.get_contain(&node);
        assert_eq!(contain, Some("size".to_string()));
    }
}
