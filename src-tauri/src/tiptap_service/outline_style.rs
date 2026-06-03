//! TipTap Outline Style Manager - Aerospace-Grade Outline Style Operations Service
//!
//! Safety-critical outline style operations service with:
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

/// Maximum outline style string length
const MAX_OUTLINE_STYLE_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutlineStyle {
    None,
    Solid,
    Dotted,
    Dashed,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

impl OutlineStyle {
    pub fn as_str(&self) -> &str {
        match self {
            OutlineStyle::None => "none",
            OutlineStyle::Solid => "solid",
            OutlineStyle::Dotted => "dotted",
            OutlineStyle::Dashed => "dashed",
            OutlineStyle::Double => "double",
            OutlineStyle::Groove => "groove",
            OutlineStyle::Ridge => "ridge",
            OutlineStyle::Inset => "inset",
            OutlineStyle::Outset => "outset",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" => Ok(OutlineStyle::None),
            "solid" => Ok(OutlineStyle::Solid),
            "dotted" => Ok(OutlineStyle::Dotted),
            "dashed" => Ok(OutlineStyle::Dashed),
            "double" => Ok(OutlineStyle::Double),
            "groove" => Ok(OutlineStyle::Groove),
            "ridge" => Ok(OutlineStyle::Ridge),
            "inset" => Ok(OutlineStyle::Inset),
            "outset" => Ok(OutlineStyle::Outset),
            _ => Err(format!("Invalid outline style: {}", s)),
        }
    }
}

pub struct OutlineStyleManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl OutlineStyleManager {
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

    pub fn max_outline_style_length() -> usize {
        MAX_OUTLINE_STYLE_LENGTH
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

    fn validate_outline_style(&self, outline_style: &str) -> Result<(), String> {
        if outline_style.len() > MAX_OUTLINE_STYLE_LENGTH {
            return Err(format!("Outline style string exceeds maximum length of {} characters", MAX_OUTLINE_STYLE_LENGTH));
        }
        OutlineStyle::from_str(outline_style)?;
        Ok(())
    }

    pub fn apply_outline_style(&mut self, node: &mut TipTapNode, outline_style: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_outline_style(outline_style)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("outlineStyle".to_string(), serde_json::Value::String(outline_style.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "outlineStyle": outline_style }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Outline style application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Outline style application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_outline_style(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("outlineStyle");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Outline style removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Outline style removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_outline_style(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(outline_style) = obj.get("outlineStyle") {
                    if let Some(s) = outline_style.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_outline_style(&self, node: &TipTapNode) -> bool {
        self.get_outline_style(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_outline_style_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OutlineStyleManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_outline_style_variants() {
        assert_eq!(OutlineStyle::Solid.as_str(), "solid");
        assert_eq!(OutlineStyle::Dashed.as_str(), "dashed");
    }

    #[test]
    fn test_apply_outline_style() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OutlineStyleManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_outline_style(&mut node, "solid");
        assert!(result.is_ok());
        assert!(manager.has_outline_style(&node));
    }

    #[test]
    fn test_remove_outline_style() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OutlineStyleManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "outlineStyle": "dotted" })),
            marks: None,
        };
        
        assert!(manager.has_outline_style(&node));
        let result = manager.remove_outline_style(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_outline_style(&node));
    }

    #[test]
    fn test_get_outline_style() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OutlineStyleManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "outlineStyle": "dashed" })),
            marks: None,
        };
        
        let outline_style = manager.get_outline_style(&node);
        assert_eq!(outline_style, Some("dashed".to_string()));
    }
}
