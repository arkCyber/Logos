//! TipTap Font Feature Settings Manager - Aerospace-Grade Font Feature Settings Operations Service
//!
//! Safety-critical font feature settings operations service with:
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

/// Maximum font feature settings string length
const MAX_FONT_FEATURE_SETTINGS_LENGTH: usize = 200;

pub struct FontFeatureSettingsManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FontFeatureSettingsManager {
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

    pub fn max_font_feature_settings_length() -> usize {
        MAX_FONT_FEATURE_SETTINGS_LENGTH
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

    fn validate_font_feature_settings(&self, font_feature_settings: &str) -> Result<(), String> {
        if font_feature_settings.is_empty() {
            return Err("Font feature settings cannot be empty".to_string());
        }
        if font_feature_settings.len() > MAX_FONT_FEATURE_SETTINGS_LENGTH {
            return Err(format!("Font feature settings string exceeds maximum length of {} characters", MAX_FONT_FEATURE_SETTINGS_LENGTH));
        }
        if font_feature_settings == "normal" {
            return Ok(());
        }
        if font_feature_settings.contains('<') || font_feature_settings.contains('>') || font_feature_settings.contains('"') {
            return Err("Font feature settings contains invalid characters".to_string());
        }
        Ok(())
    }

    pub fn apply_font_feature_settings(&mut self, node: &mut TipTapNode, font_feature_settings: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_font_feature_settings(font_feature_settings)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("fontFeatureSettings".to_string(), serde_json::Value::String(font_feature_settings.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "fontFeatureSettings": font_feature_settings }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font feature settings application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font feature settings application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_font_feature_settings(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("fontFeatureSettings");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font feature settings removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font feature settings removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_font_feature_settings(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(font_feature_settings) = obj.get("fontFeatureSettings") {
                    if let Some(s) = font_feature_settings.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_font_feature_settings(&self, node: &TipTapNode) -> bool {
        self.get_font_feature_settings(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_font_feature_settings_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontFeatureSettingsManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_font_feature_settings() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontFeatureSettingsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_font_feature_settings(&mut node, "normal");
        assert!(result.is_ok());
        assert!(manager.has_font_feature_settings(&node));
    }

    #[test]
    fn test_remove_font_feature_settings() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontFeatureSettingsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontFeatureSettings": "liga 1" })),
            marks: None,
        };
        
        assert!(manager.has_font_feature_settings(&node));
        let result = manager.remove_font_feature_settings(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_font_feature_settings(&node));
    }

    #[test]
    fn test_get_font_feature_settings() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontFeatureSettingsManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontFeatureSettings": "ss01" })),
            marks: None,
        };
        
        let font_feature_settings = manager.get_font_feature_settings(&node);
        assert_eq!(font_feature_settings, Some("ss01".to_string()));
    }
}
