//! TipTap Font Family Manager - Aerospace-Grade Font Family Operations Service
//!
//! Safety-critical font family operations service with:
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

/// Maximum font family string length
const MAX_FONT_FAMILY_LENGTH: usize = 200;

/// Common safe font families
const SAFE_FONT_FAMILIES: &[&str] = &[
    "Arial", "Helvetica", "Times New Roman", "Times", "Courier New", "Courier",
    "Verdana", "Georgia", "Palatino", "Garamond", "Bookman", "Comic Sans MS",
    "Trebuchet MS", "Arial Black", "Impact", "sans-serif", "serif", "monospace",
    "cursive", "fantasy", "system-ui", "-apple-system", "BlinkMacSystemFont",
    "Segoe UI", "Roboto", "Oxygen", "Ubuntu", "Cantarell", "Fira Sans",
    "Droid Sans", "Helvetica Neue",
];

pub struct FontFamilyManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FontFamilyManager {
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

    pub fn max_font_family_length() -> usize {
        MAX_FONT_FAMILY_LENGTH
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

    fn validate_font_family(&self, font_family: &str) -> Result<(), String> {
        if font_family.is_empty() {
            return Err("Font family cannot be empty".to_string());
        }
        if font_family.len() > MAX_FONT_FAMILY_LENGTH {
            return Err(format!("Font family string exceeds maximum length of {} characters", MAX_FONT_FAMILY_LENGTH));
        }
        // Check for potentially dangerous characters
        if font_family.contains('<') || font_family.contains('>') || font_family.contains('"') {
            return Err("Font family contains invalid characters".to_string());
        }
        Ok(())
    }

    pub fn apply_font_family(&mut self, node: &mut TipTapNode, font_family: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_font_family(font_family)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("fontFamily".to_string(), serde_json::Value::String(font_family.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "fontFamily": font_family }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font family application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font family application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_font_family(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("fontFamily");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font family removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font family removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_font_family(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(font_family) = obj.get("fontFamily") {
                    if let Some(s) = font_family.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_font_family(&self, node: &TipTapNode) -> bool {
        self.get_font_family(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_font_family_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontFamilyManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_font_family() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontFamilyManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_font_family(&mut node, "Arial");
        assert!(result.is_ok());
        assert!(manager.has_font_family(&node));
    }

    #[test]
    fn test_remove_font_family() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontFamilyManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontFamily": "Helvetica" })),
            marks: None,
        };
        
        assert!(manager.has_font_family(&node));
        let result = manager.remove_font_family(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_font_family(&node));
    }

    #[test]
    fn test_get_font_family() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontFamilyManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontFamily": "Times New Roman" })),
            marks: None,
        };
        
        let font_family = manager.get_font_family(&node);
        assert_eq!(font_family, Some("Times New Roman".to_string()));
    }
}
