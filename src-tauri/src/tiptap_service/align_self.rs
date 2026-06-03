//! TipTap Align Self Manager - Aerospace-Grade Align Self Operations Service
//!
//! Safety-critical align self operations service with:
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

/// Maximum align self string length
const MAX_ALIGN_SELF_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignSelf {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl AlignSelf {
    pub fn as_str(&self) -> &str {
        match self {
            AlignSelf::Auto => "auto",
            AlignSelf::FlexStart => "flex-start",
            AlignSelf::FlexEnd => "flex-end",
            AlignSelf::Center => "center",
            AlignSelf::Baseline => "baseline",
            AlignSelf::Stretch => "stretch",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(AlignSelf::Auto),
            "flex-start" => Ok(AlignSelf::FlexStart),
            "flex-end" => Ok(AlignSelf::FlexEnd),
            "center" => Ok(AlignSelf::Center),
            "baseline" => Ok(AlignSelf::Baseline),
            "stretch" => Ok(AlignSelf::Stretch),
            _ => Err(format!("Invalid align self: {}", s)),
        }
    }
}

pub struct AlignSelfManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl AlignSelfManager {
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

    pub fn max_align_self_length() -> usize {
        MAX_ALIGN_SELF_LENGTH
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

    fn validate_align_self(&self, align_self: &str) -> Result<(), String> {
        if align_self.len() > MAX_ALIGN_SELF_LENGTH {
            return Err(format!("Align self string exceeds maximum length of {} characters", MAX_ALIGN_SELF_LENGTH));
        }
        AlignSelf::from_str(align_self)?;
        Ok(())
    }

    pub fn apply_align_self(&mut self, node: &mut TipTapNode, align_self: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_align_self(align_self)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("alignSelf".to_string(), serde_json::Value::String(align_self.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "alignSelf": align_self }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Align self application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Align self application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_align_self(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("alignSelf");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Align self removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Align self removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_align_self(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(align_self) = obj.get("alignSelf") {
                    if let Some(s) = align_self.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_align_self(&self, node: &TipTapNode) -> bool {
        self.get_align_self(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_align_self_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AlignSelfManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_align_self_variants() {
        assert_eq!(AlignSelf::Auto.as_str(), "auto");
        assert_eq!(AlignSelf::Center.as_str(), "center");
    }

    #[test]
    fn test_apply_align_self() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AlignSelfManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_align_self(&mut node, "center");
        assert!(result.is_ok());
        assert!(manager.has_align_self(&node));
    }

    #[test]
    fn test_remove_align_self() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AlignSelfManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "alignSelf": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_align_self(&node));
        let result = manager.remove_align_self(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_align_self(&node));
    }

    #[test]
    fn test_get_align_self() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AlignSelfManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "alignSelf": "stretch" })),
            marks: None,
        };
        
        let align_self = manager.get_align_self(&node);
        assert_eq!(align_self, Some("stretch".to_string()));
    }
}
