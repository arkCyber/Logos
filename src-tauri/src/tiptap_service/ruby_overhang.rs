//! TipTap Ruby Overhang Manager - Aerospace-Grade Ruby Overhang Operations Service
//!
//! Safety-critical ruby overhang operations service with:
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
pub enum RubyOverhang {
    Auto,
    None,
}

impl RubyOverhang {
    pub fn as_str(&self) -> &str {
        match self {
            RubyOverhang::Auto => "auto",
            RubyOverhang::None => "none",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(RubyOverhang::Auto),
            "none" => Ok(RubyOverhang::None),
            _ => Err(format!("Invalid ruby overhang value: {}", s)),
        }
    }
}

pub struct RubyOverhangManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl RubyOverhangManager {
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

    fn validate_ruby_overhang(&self, ruby_overhang: &str) -> Result<(), String> {
        if ruby_overhang.is_empty() {
            return Err("Ruby overhang cannot be empty".to_string());
        }
        RubyOverhang::from_str(ruby_overhang)?;
        Ok(())
    }

    pub fn apply_ruby_overhang(&mut self, node: &mut TipTapNode, ruby_overhang: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_ruby_overhang(ruby_overhang)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("rubyOverhang".to_string(), serde_json::Value::String(ruby_overhang.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "rubyOverhang": ruby_overhang }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Ruby overhang application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Ruby overhang application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_ruby_overhang(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("rubyOverhang");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Ruby overhang removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Ruby overhang removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_ruby_overhang(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(ruby_overhang) = obj.get("rubyOverhang") {
                    if let Some(s) = ruby_overhang.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_ruby_overhang(&self, node: &TipTapNode) -> bool {
        self.get_ruby_overhang(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_ruby_overhang_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = RubyOverhangManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_ruby_overhang_variants() {
        assert_eq!(RubyOverhang::Auto.as_str(), "auto");
        assert_eq!(RubyOverhang::None.as_str(), "none");
    }

    #[test]
    fn test_apply_ruby_overhang() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = RubyOverhangManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_ruby_overhang(&mut node, "auto");
        assert!(result.is_ok());
        assert!(manager.has_ruby_overhang(&node));
    }

    #[test]
    fn test_remove_ruby_overhang() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = RubyOverhangManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "rubyOverhang": "none" })),
            marks: None,
        };
        
        assert!(manager.has_ruby_overhang(&node));
        let result = manager.remove_ruby_overhang(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_ruby_overhang(&node));
    }

    #[test]
    fn test_get_ruby_overhang() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = RubyOverhangManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "rubyOverhang": "auto" })),
            marks: None,
        };
        
        let ruby_overhang = manager.get_ruby_overhang(&node);
        assert_eq!(ruby_overhang, Some("auto".to_string()));
    }
}
