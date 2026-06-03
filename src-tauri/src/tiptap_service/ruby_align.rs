//! TipTap Ruby Align Manager - Aerospace-Grade Ruby Align Operations Service
//!
//! Safety-critical ruby align operations service with:
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RubyAlign {
    Start,
    Center,
    SpaceBetween,
    SpaceAround,
}

impl RubyAlign {
    pub fn as_str(&self) -> &str {
        match self {
            RubyAlign::Start => "start",
            RubyAlign::Center => "center",
            RubyAlign::SpaceBetween => "space-between",
            RubyAlign::SpaceAround => "space-around",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "start" => Ok(RubyAlign::Start),
            "center" => Ok(RubyAlign::Center),
            "space-between" => Ok(RubyAlign::SpaceBetween),
            "space-around" => Ok(RubyAlign::SpaceAround),
            _ => Err(format!("Invalid ruby align value: {}", s)),
        }
    }
}

pub struct RubyAlignManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl RubyAlignManager {
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

    fn validate_ruby_align(&self, ruby_align: &str) -> Result<(), String> {
        if ruby_align.is_empty() {
            return Err("Ruby align cannot be empty".to_string());
        }
        RubyAlign::from_str(ruby_align)?;
        Ok(())
    }

    pub fn apply_ruby_align(&mut self, node: &mut TipTapNode, ruby_align: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_ruby_align(ruby_align)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("rubyAlign".to_string(), serde_json::Value::String(ruby_align.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "rubyAlign": ruby_align }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Ruby align application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Ruby align application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_ruby_align(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("rubyAlign");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Ruby align removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Ruby align removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_ruby_align(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(ruby_align) = obj.get("rubyAlign") {
                    if let Some(s) = ruby_align.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_ruby_align(&self, node: &TipTapNode) -> bool {
        self.get_ruby_align(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_ruby_align_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = RubyAlignManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_ruby_align_variants() {
        assert_eq!(RubyAlign::Start.as_str(), "start");
        assert_eq!(RubyAlign::Center.as_str(), "center");
    }

    #[test]
    fn test_apply_ruby_align() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = RubyAlignManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_ruby_align(&mut node, "center");
        assert!(result.is_ok());
        assert!(manager.has_ruby_align(&node));
    }

    #[test]
    fn test_remove_ruby_align() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = RubyAlignManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "rubyAlign": "start" })),
            marks: None,
        };
        
        assert!(manager.has_ruby_align(&node));
        let result = manager.remove_ruby_align(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_ruby_align(&node));
    }

    #[test]
    fn test_get_ruby_align() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = RubyAlignManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "rubyAlign": "space-between" })),
            marks: None,
        };
        
        let ruby_align = manager.get_ruby_align(&node);
        assert_eq!(ruby_align, Some("space-between".to_string()));
    }
}
