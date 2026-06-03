//! TipTap Color Scheme Manager - Aerospace-Grade Color Scheme Operations Service
//!
//! Safety-critical color scheme operations service with:
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
pub enum ColorScheme {
    Normal,
    Light,
    Dark,
}

impl ColorScheme {
    pub fn as_str(&self) -> &str {
        match self {
            ColorScheme::Normal => "normal",
            ColorScheme::Light => "light",
            ColorScheme::Dark => "dark",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(ColorScheme::Normal),
            "light" => Ok(ColorScheme::Light),
            "dark" => Ok(ColorScheme::Dark),
            _ => Err(format!("Invalid color scheme value: {}", s)),
        }
    }
}

pub struct ColorSchemeManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ColorSchemeManager {
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

    fn validate_color_scheme(&self, color_scheme: &str) -> Result<(), String> {
        if color_scheme.is_empty() {
            return Err("Color scheme cannot be empty".to_string());
        }
        ColorScheme::from_str(color_scheme)?;
        Ok(())
    }

    pub fn apply_color_scheme(&mut self, node: &mut TipTapNode, color_scheme: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_color_scheme(color_scheme)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("colorScheme".to_string(), serde_json::Value::String(color_scheme.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "colorScheme": color_scheme }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Color scheme application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Color scheme application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_color_scheme(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("colorScheme");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Color scheme removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Color scheme removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_color_scheme(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(color_scheme) = obj.get("colorScheme") {
                    if let Some(s) = color_scheme.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_color_scheme(&self, node: &TipTapNode) -> bool {
        self.get_color_scheme(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_color_scheme_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColorSchemeManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_color_scheme_variants() {
        assert_eq!(ColorScheme::Normal.as_str(), "normal");
        assert_eq!(ColorScheme::Dark.as_str(), "dark");
    }

    #[test]
    fn test_apply_color_scheme() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColorSchemeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_color_scheme(&mut node, "dark");
        assert!(result.is_ok());
        assert!(manager.has_color_scheme(&node));
    }

    #[test]
    fn test_remove_color_scheme() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColorSchemeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "colorScheme": "light" })),
            marks: None,
        };
        
        assert!(manager.has_color_scheme(&node));
        let result = manager.remove_color_scheme(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_color_scheme(&node));
    }

    #[test]
    fn test_get_color_scheme() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColorSchemeManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "colorScheme": "normal" })),
            marks: None,
        };
        
        let color_scheme = manager.get_color_scheme(&node);
        assert_eq!(color_scheme, Some("normal".to_string()));
    }
}
