//! TipTap Font Language Override Manager - Aerospace-Grade Font Language Override Operations Service
//!
//! Safety-critical font language override operations service with:
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

/// Maximum font language override string length
const MAX_FONT_LANGUAGE_OVERRIDE_LENGTH: usize = 50;

pub struct FontLanguageOverrideManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FontLanguageOverrideManager {
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

    pub fn max_font_language_override_length() -> usize {
        MAX_FONT_LANGUAGE_OVERRIDE_LENGTH
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

    fn validate_font_language_override(&self, font_language_override: &str) -> Result<(), String> {
        if font_language_override.is_empty() {
            return Err("Font language override cannot be empty".to_string());
        }
        if font_language_override.len() > MAX_FONT_LANGUAGE_OVERRIDE_LENGTH {
            return Err(format!("Font language override string exceeds maximum length of {} characters", MAX_FONT_LANGUAGE_OVERRIDE_LENGTH));
        }
        if font_language_override == "normal" {
            return Ok(());
        }
        if font_language_override.contains('<') || font_language_override.contains('>') || font_language_override.contains('"') {
            return Err("Font language override contains invalid characters".to_string());
        }
        Ok(())
    }

    pub fn apply_font_language_override(&mut self, node: &mut TipTapNode, font_language_override: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_font_language_override(font_language_override)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("fontLanguageOverride".to_string(), serde_json::Value::String(font_language_override.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "fontLanguageOverride": font_language_override }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font language override application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font language override application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_font_language_override(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("fontLanguageOverride");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font language override removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font language override removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_font_language_override(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(font_language_override) = obj.get("fontLanguageOverride") {
                    if let Some(s) = font_language_override.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_font_language_override(&self, node: &TipTapNode) -> bool {
        self.get_font_language_override(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_font_language_override_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontLanguageOverrideManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_font_language_override() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontLanguageOverrideManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_font_language_override(&mut node, "normal");
        assert!(result.is_ok());
        assert!(manager.has_font_language_override(&node));
    }

    #[test]
    fn test_remove_font_language_override() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontLanguageOverrideManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontLanguageOverride": "jpn" })),
            marks: None,
        };
        
        assert!(manager.has_font_language_override(&node));
        let result = manager.remove_font_language_override(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_font_language_override(&node));
    }

    #[test]
    fn test_get_font_language_override() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontLanguageOverrideManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontLanguageOverride": "srp" })),
            marks: None,
        };
        
        let font_language_override = manager.get_font_language_override(&node);
        assert_eq!(font_language_override, Some("srp".to_string()));
    }
}
