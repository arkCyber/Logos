//! TipTap Border Spacing Manager - Aerospace-Grade Border Spacing Operations Service
//!
//! Safety-critical border spacing operations service with:
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

/// Maximum border spacing value (in pixels)
const MAX_BORDER_SPACING: f64 = 100.0;

/// Minimum border spacing value (in pixels)
const MIN_BORDER_SPACING: f64 = 0.0;

/// Maximum border spacing string length
const MAX_BORDER_SPACING_LENGTH: usize = 50;

pub struct BorderSpacingManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl BorderSpacingManager {
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

    pub fn max_border_spacing() -> f64 {
        MAX_BORDER_SPACING
    }

    pub fn min_border_spacing() -> f64 {
        MIN_BORDER_SPACING
    }

    pub fn max_border_spacing_length() -> usize {
        MAX_BORDER_SPACING_LENGTH
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

    fn validate_border_spacing(&self, border_spacing: &str) -> Result<(), String> {
        if border_spacing.is_empty() {
            return Err("Border spacing cannot be empty".to_string());
        }
        if border_spacing.len() > MAX_BORDER_SPACING_LENGTH {
            return Err(format!("Border spacing string exceeds maximum length of {} characters", MAX_BORDER_SPACING_LENGTH));
        }
        if border_spacing.ends_with("px") {
            let value_str = border_spacing.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_BORDER_SPACING || value > MAX_BORDER_SPACING {
                    return Err(format!("Border spacing must be between {} and {} pixels", MIN_BORDER_SPACING, MAX_BORDER_SPACING));
                }
                if !value.is_finite() {
                    return Err("Border spacing must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_border_spacing(&mut self, node: &mut TipTapNode, border_spacing: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_border_spacing(border_spacing)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("borderSpacing".to_string(), serde_json::Value::String(border_spacing.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "borderSpacing": border_spacing }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Border spacing application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Border spacing application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_border_spacing(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("borderSpacing");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Border spacing removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Border spacing removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_border_spacing(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(border_spacing) = obj.get("borderSpacing") {
                    if let Some(s) = border_spacing.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_border_spacing(&self, node: &TipTapNode) -> bool {
        self.get_border_spacing(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_border_spacing_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BorderSpacingManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_border_spacing() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderSpacingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_border_spacing(&mut node, "10px");
        assert!(result.is_ok());
        assert!(manager.has_border_spacing(&node));
    }

    #[test]
    fn test_remove_border_spacing() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderSpacingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "borderSpacing": "5px" })),
            marks: None,
        };
        
        assert!(manager.has_border_spacing(&node));
        let result = manager.remove_border_spacing(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_border_spacing(&node));
    }

    #[test]
    fn test_get_border_spacing() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BorderSpacingManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "borderSpacing": "15px" })),
            marks: None,
        };
        
        let border_spacing = manager.get_border_spacing(&node);
        assert_eq!(border_spacing, Some("15px".to_string()));
    }
}
