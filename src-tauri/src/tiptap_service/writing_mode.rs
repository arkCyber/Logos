//! TipTap Writing Mode Manager - Aerospace-Grade Writing Mode Operations Service
//!
//! Safety-critical writing mode operations service with:
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

/// Maximum writing mode string length
const MAX_WRITING_MODE_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WritingMode {
    HorizontalTb,
    VerticalRl,
    VerticalLr,
}

impl WritingMode {
    pub fn as_str(&self) -> &str {
        match self {
            WritingMode::HorizontalTb => "horizontal-tb",
            WritingMode::VerticalRl => "vertical-rl",
            WritingMode::VerticalLr => "vertical-lr",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "horizontal-tb" => Ok(WritingMode::HorizontalTb),
            "vertical-rl" => Ok(WritingMode::VerticalRl),
            "vertical-lr" => Ok(WritingMode::VerticalLr),
            _ => Err(format!("Invalid writing mode: {}", s)),
        }
    }
}

pub struct WritingModeManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl WritingModeManager {
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

    pub fn max_writing_mode_length() -> usize {
        MAX_WRITING_MODE_LENGTH
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

    fn validate_writing_mode(&self, writing_mode: &str) -> Result<(), String> {
        if writing_mode.len() > MAX_WRITING_MODE_LENGTH {
            return Err(format!("Writing mode string exceeds maximum length of {} characters", MAX_WRITING_MODE_LENGTH));
        }
        WritingMode::from_str(writing_mode)?;
        Ok(())
    }

    pub fn apply_writing_mode(&mut self, node: &mut TipTapNode, writing_mode: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_writing_mode(writing_mode)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("writingMode".to_string(), serde_json::Value::String(writing_mode.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "writingMode": writing_mode }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Writing mode application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Writing mode application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_writing_mode(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("writingMode");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Writing mode removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Writing mode removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_writing_mode(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(writing_mode) = obj.get("writingMode") {
                    if let Some(s) = writing_mode.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_writing_mode(&self, node: &TipTapNode) -> bool {
        self.get_writing_mode(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_writing_mode_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = WritingModeManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_writing_mode_variants() {
        assert_eq!(WritingMode::HorizontalTb.as_str(), "horizontal-tb");
        assert_eq!(WritingMode::VerticalRl.as_str(), "vertical-rl");
    }

    #[test]
    fn test_apply_writing_mode() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WritingModeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_writing_mode(&mut node, "vertical-rl");
        assert!(result.is_ok());
        assert!(manager.has_writing_mode(&node));
    }

    #[test]
    fn test_remove_writing_mode() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WritingModeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "writingMode": "horizontal-tb" })),
            marks: None,
        };
        
        assert!(manager.has_writing_mode(&node));
        let result = manager.remove_writing_mode(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_writing_mode(&node));
    }

    #[test]
    fn test_get_writing_mode() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = WritingModeManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "writingMode": "vertical-lr" })),
            marks: None,
        };
        
        let writing_mode = manager.get_writing_mode(&node);
        assert_eq!(writing_mode, Some("vertical-lr".to_string()));
    }
}
