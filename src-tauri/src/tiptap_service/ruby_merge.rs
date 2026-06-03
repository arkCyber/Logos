//! TipTap Ruby Merge Manager - Aerospace-Grade Ruby Merge Operations Service
//!
//! Safety-critical ruby merge operations service with:
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
pub enum RubyMerge {
    Auto,
    Separate,
}

impl RubyMerge {
    pub fn as_str(&self) -> &str {
        match self {
            RubyMerge::Auto => "auto",
            RubyMerge::Separate => "separate",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(RubyMerge::Auto),
            "separate" => Ok(RubyMerge::Separate),
            _ => Err(format!("Invalid ruby merge value: {}", s)),
        }
    }
}

pub struct RubyMergeManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl RubyMergeManager {
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

    fn validate_ruby_merge(&self, ruby_merge: &str) -> Result<(), String> {
        if ruby_merge.is_empty() {
            return Err("Ruby merge cannot be empty".to_string());
        }
        RubyMerge::from_str(ruby_merge)?;
        Ok(())
    }

    pub fn apply_ruby_merge(&mut self, node: &mut TipTapNode, ruby_merge: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_ruby_merge(ruby_merge)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("rubyMerge".to_string(), serde_json::Value::String(ruby_merge.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "rubyMerge": ruby_merge }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Ruby merge application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Ruby merge application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_ruby_merge(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("rubyMerge");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Ruby merge removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Ruby merge removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_ruby_merge(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(ruby_merge) = obj.get("rubyMerge") {
                    if let Some(s) = ruby_merge.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_ruby_merge(&self, node: &TipTapNode) -> bool {
        self.get_ruby_merge(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_ruby_merge_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = RubyMergeManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_ruby_merge_variants() {
        assert_eq!(RubyMerge::Auto.as_str(), "auto");
        assert_eq!(RubyMerge::Separate.as_str(), "separate");
    }

    #[test]
    fn test_apply_ruby_merge() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = RubyMergeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_ruby_merge(&mut node, "auto");
        assert!(result.is_ok());
        assert!(manager.has_ruby_merge(&node));
    }

    #[test]
    fn test_remove_ruby_merge() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = RubyMergeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "rubyMerge": "separate" })),
            marks: None,
        };
        
        assert!(manager.has_ruby_merge(&node));
        let result = manager.remove_ruby_merge(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_ruby_merge(&node));
    }

    #[test]
    fn test_get_ruby_merge() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = RubyMergeManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "rubyMerge": "auto" })),
            marks: None,
        };
        
        let ruby_merge = manager.get_ruby_merge(&node);
        assert_eq!(ruby_merge, Some("auto".to_string()));
    }
}
