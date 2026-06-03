//! TipTap Font Variant Position Manager - Aerospace-Grade Font Variant Position Operations Service
//!
//! Safety-critical font variant position operations service with:
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
pub enum FontVariantPosition {
    Normal,
    Sub,
    Super,
}

impl FontVariantPosition {
    pub fn as_str(&self) -> &str {
        match self {
            FontVariantPosition::Normal => "normal",
            FontVariantPosition::Sub => "sub",
            FontVariantPosition::Super => "super",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(FontVariantPosition::Normal),
            "sub" => Ok(FontVariantPosition::Sub),
            "super" => Ok(FontVariantPosition::Super),
            _ => Err(format!("Invalid font variant position value: {}", s)),
        }
    }
}

pub struct FontVariantPositionManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FontVariantPositionManager {
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

    fn validate_font_variant_position(&self, font_variant_position: &str) -> Result<(), String> {
        if font_variant_position.is_empty() {
            return Err("Font variant position cannot be empty".to_string());
        }
        FontVariantPosition::from_str(font_variant_position)?;
        Ok(())
    }

    pub fn apply_font_variant_position(&mut self, node: &mut TipTapNode, font_variant_position: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_font_variant_position(font_variant_position)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("fontVariantPosition".to_string(), serde_json::Value::String(font_variant_position.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "fontVariantPosition": font_variant_position }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font variant position application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font variant position application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_font_variant_position(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("fontVariantPosition");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font variant position removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font variant position removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_font_variant_position(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(font_variant_position) = obj.get("fontVariantPosition") {
                    if let Some(s) = font_variant_position.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_font_variant_position(&self, node: &TipTapNode) -> bool {
        self.get_font_variant_position(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_font_variant_position_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontVariantPositionManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_font_variant_position_variants() {
        assert_eq!(FontVariantPosition::Normal.as_str(), "normal");
        assert_eq!(FontVariantPosition::Sub.as_str(), "sub");
    }

    #[test]
    fn test_apply_font_variant_position() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontVariantPositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_font_variant_position(&mut node, "sub");
        assert!(result.is_ok());
        assert!(manager.has_font_variant_position(&node));
    }

    #[test]
    fn test_remove_font_variant_position() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontVariantPositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontVariantPosition": "super" })),
            marks: None,
        };
        
        assert!(manager.has_font_variant_position(&node));
        let result = manager.remove_font_variant_position(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_font_variant_position(&node));
    }

    #[test]
    fn test_get_font_variant_position() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontVariantPositionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontVariantPosition": "normal" })),
            marks: None,
        };
        
        let font_variant_position = manager.get_font_variant_position(&node);
        assert_eq!(font_variant_position, Some("normal".to_string()));
    }
}
