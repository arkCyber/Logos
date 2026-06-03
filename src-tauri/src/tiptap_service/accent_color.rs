//! TipTap Accent Color Manager - Aerospace-Grade Accent Color Operations Service
//!
//! Safety-critical accent color operations service with:
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

/// Maximum accent color string length
const MAX_ACCENT_COLOR_LENGTH: usize = 100;

pub struct AccentColorManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl AccentColorManager {
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

    pub fn max_accent_color_length() -> usize {
        MAX_ACCENT_COLOR_LENGTH
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

    fn validate_accent_color(&self, accent_color: &str) -> Result<(), String> {
        if accent_color.is_empty() {
            return Err("Accent color cannot be empty".to_string());
        }
        if accent_color.len() > MAX_ACCENT_COLOR_LENGTH {
            return Err(format!("Accent color string exceeds maximum length of {} characters", MAX_ACCENT_COLOR_LENGTH));
        }
        if accent_color.contains('<') || accent_color.contains('>') || accent_color.contains('"') {
            return Err("Accent color contains invalid characters".to_string());
        }
        Ok(())
    }

    pub fn apply_accent_color(&mut self, node: &mut TipTapNode, accent_color: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_accent_color(accent_color)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("accentColor".to_string(), serde_json::Value::String(accent_color.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "accentColor": accent_color }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Accent color application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Accent color application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_accent_color(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("accentColor");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Accent color removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Accent color removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_accent_color(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(accent_color) = obj.get("accentColor") {
                    if let Some(s) = accent_color.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_accent_color(&self, node: &TipTapNode) -> bool {
        self.get_accent_color(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_accent_color_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AccentColorManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_accent_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AccentColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_accent_color(&mut node, "#ff0000");
        assert!(result.is_ok());
        assert!(manager.has_accent_color(&node));
    }

    #[test]
    fn test_remove_accent_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AccentColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "accentColor": "blue" })),
            marks: None,
        };
        
        assert!(manager.has_accent_color(&node));
        let result = manager.remove_accent_color(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_accent_color(&node));
    }

    #[test]
    fn test_get_accent_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AccentColorManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "accentColor": "#00ff00" })),
            marks: None,
        };
        
        let accent_color = manager.get_accent_color(&node);
        assert_eq!(accent_color, Some("#00ff00".to_string()));
    }
}
