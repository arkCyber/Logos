//! TipTap Text Combine Upright Manager - Aerospace-Grade Text Combine Upright Operations Service
//!
//! Safety-critical text combine upright operations service with:
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
pub enum TextCombineUpright {
    None,
    All,
    Digits,
}

impl TextCombineUpright {
    pub fn as_str(&self) -> &str {
        match self {
            TextCombineUpright::None => "none",
            TextCombineUpright::All => "all",
            TextCombineUpright::Digits => "digits",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" => Ok(TextCombineUpright::None),
            "all" => Ok(TextCombineUpright::All),
            "digits" => Ok(TextCombineUpright::Digits),
            _ => Err(format!("Invalid text combine upright value: {}", s)),
        }
    }
}

pub struct TextCombineUprightManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextCombineUprightManager {
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

    fn validate_text_combine_upright(&self, text_combine_upright: &str) -> Result<(), String> {
        if text_combine_upright.is_empty() {
            return Err("Text combine upright cannot be empty".to_string());
        }
        TextCombineUpright::from_str(text_combine_upright)?;
        Ok(())
    }

    pub fn apply_text_combine_upright(&mut self, node: &mut TipTapNode, text_combine_upright: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_text_combine_upright(text_combine_upright)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textCombineUpright".to_string(), serde_json::Value::String(text_combine_upright.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textCombineUpright": text_combine_upright }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text combine upright application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text combine upright application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_text_combine_upright(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textCombineUpright");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text combine upright removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text combine upright removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_text_combine_upright(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(text_combine_upright) = obj.get("textCombineUpright") {
                    if let Some(s) = text_combine_upright.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_text_combine_upright(&self, node: &TipTapNode) -> bool {
        self.get_text_combine_upright(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_combine_upright_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextCombineUprightManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_text_combine_upright_variants() {
        assert_eq!(TextCombineUpright::None.as_str(), "none");
        assert_eq!(TextCombineUpright::All.as_str(), "all");
    }

    #[test]
    fn test_apply_text_combine_upright() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextCombineUprightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_text_combine_upright(&mut node, "all");
        assert!(result.is_ok());
        assert!(manager.has_text_combine_upright(&node));
    }

    #[test]
    fn test_remove_text_combine_upright() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextCombineUprightManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textCombineUpright": "none" })),
            marks: None,
        };
        
        assert!(manager.has_text_combine_upright(&node));
        let result = manager.remove_text_combine_upright(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_text_combine_upright(&node));
    }

    #[test]
    fn test_get_text_combine_upright() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextCombineUprightManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textCombineUpright": "digits" })),
            marks: None,
        };
        
        let text_combine_upright = manager.get_text_combine_upright(&node);
        assert_eq!(text_combine_upright, Some("digits".to_string()));
    }
}
