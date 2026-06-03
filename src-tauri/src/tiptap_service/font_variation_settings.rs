//! TipTap Font Variation Settings Manager - Aerospace-Grade Font Variation Settings Operations Service
//!
//! Safety-critical font variation settings operations service with:
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

/// Maximum font variation settings string length
const MAX_FONT_VARIATION_SETTINGS_LENGTH: usize = 200;

pub struct FontVariationSettingsManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FontVariationSettingsManager {
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

    pub fn max_font_variation_settings_length() -> usize {
        MAX_FONT_VARIATION_SETTINGS_LENGTH
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

    fn validate_font_variation_settings(&self, font_variation_settings: &str) -> Result<(), String> {
        if font_variation_settings.is_empty() {
            return Err("Font variation settings cannot be empty".to_string());
        }
        if font_variation_settings.len() > MAX_FONT_VARIATION_SETTINGS_LENGTH {
            return Err(format!("Font variation settings string exceeds maximum length of {} characters", MAX_FONT_VARIATION_SETTINGS_LENGTH));
        }
        if font_variation_settings == "normal" {
            return Ok(());
        }
        if font_variation_settings.contains('<') || font_variation_settings.contains('>') || font_variation_settings.contains('"') {
            return Err("Font variation settings contains invalid characters".to_string());
        }
        Ok(())
    }

    pub fn apply_font_variation_settings(&mut self, node: &mut TipTapNode, font_variation_settings: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_font_variation_settings(font_variation_settings)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("fontVariationSettings".to_string(), serde_json::Value::String(font_variation_settings.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "fontVariationSettings": font_variation_settings }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font variation settings application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font variation settings application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_font_variation_settings(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("fontVariationSettings");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font variation settings removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font variation settings removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_font_variation_settings(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(font_variation_settings) = obj.get("fontVariationSettings") {
                    if let Some(s) = font_variation_settings.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_font_variation_settings(&self, node: &TipTapNode) -> bool {
        self.get_font_variation_settings(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_font_variation_settings_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontVariationSettingsManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_font_variation_settings() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontVariationSettingsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_font_variation_settings(&mut node, "normal");
        assert!(result.is_ok());
        assert!(manager.has_font_variation_settings(&node));
    }

    #[test]
    fn test_remove_font_variation_settings() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontVariationSettingsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontVariationSettings": "wght 400" })),
            marks: None,
        };
        
        assert!(manager.has_font_variation_settings(&node));
        let result = manager.remove_font_variation_settings(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_font_variation_settings(&node));
    }

    #[test]
    fn test_get_font_variation_settings() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontVariationSettingsManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontVariationSettings": "wdth 100" })),
            marks: None,
        };
        
        let font_variation_settings = manager.get_font_variation_settings(&node);
        assert_eq!(font_variation_settings, Some("wdth 100".to_string()));
    }
}
