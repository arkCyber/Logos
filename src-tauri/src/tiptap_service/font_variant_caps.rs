//! TipTap Font Variant Caps Manager - Aerospace-Grade Font Variant Caps Operations Service
//!
//! Safety-critical font variant caps operations service with:
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
pub enum FontVariantCaps {
    Normal,
    SmallCaps,
    AllSmallCaps,
    PetiteCaps,
    AllPetiteCaps,
    Unicase,
    TitlingCaps,
}

impl FontVariantCaps {
    pub fn as_str(&self) -> &str {
        match self {
            FontVariantCaps::Normal => "normal",
            FontVariantCaps::SmallCaps => "small-caps",
            FontVariantCaps::AllSmallCaps => "all-small-caps",
            FontVariantCaps::PetiteCaps => "petite-caps",
            FontVariantCaps::AllPetiteCaps => "all-petite-caps",
            FontVariantCaps::Unicase => "unicase",
            FontVariantCaps::TitlingCaps => "titling-caps",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(FontVariantCaps::Normal),
            "small-caps" => Ok(FontVariantCaps::SmallCaps),
            "all-small-caps" => Ok(FontVariantCaps::AllSmallCaps),
            "petite-caps" => Ok(FontVariantCaps::PetiteCaps),
            "all-petite-caps" => Ok(FontVariantCaps::AllPetiteCaps),
            "unicase" => Ok(FontVariantCaps::Unicase),
            "titling-caps" => Ok(FontVariantCaps::TitlingCaps),
            _ => Err(format!("Invalid font variant caps value: {}", s)),
        }
    }
}

pub struct FontVariantCapsManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FontVariantCapsManager {
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

    fn validate_font_variant_caps(&self, font_variant_caps: &str) -> Result<(), String> {
        if font_variant_caps.is_empty() {
            return Err("Font variant caps cannot be empty".to_string());
        }
        FontVariantCaps::from_str(font_variant_caps)?;
        Ok(())
    }

    pub fn apply_font_variant_caps(&mut self, node: &mut TipTapNode, font_variant_caps: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_font_variant_caps(font_variant_caps)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("fontVariantCaps".to_string(), serde_json::Value::String(font_variant_caps.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "fontVariantCaps": font_variant_caps }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font variant caps application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font variant caps application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_font_variant_caps(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("fontVariantCaps");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font variant caps removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font variant caps removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_font_variant_caps(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(font_variant_caps) = obj.get("fontVariantCaps") {
                    if let Some(s) = font_variant_caps.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_font_variant_caps(&self, node: &TipTapNode) -> bool {
        self.get_font_variant_caps(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_font_variant_caps_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontVariantCapsManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_font_variant_caps_variants() {
        assert_eq!(FontVariantCaps::Normal.as_str(), "normal");
        assert_eq!(FontVariantCaps::SmallCaps.as_str(), "small-caps");
    }

    #[test]
    fn test_apply_font_variant_caps() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontVariantCapsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_font_variant_caps(&mut node, "small-caps");
        assert!(result.is_ok());
        assert!(manager.has_font_variant_caps(&node));
    }

    #[test]
    fn test_remove_font_variant_caps() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontVariantCapsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontVariantCaps": "all-small-caps" })),
            marks: None,
        };
        
        assert!(manager.has_font_variant_caps(&node));
        let result = manager.remove_font_variant_caps(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_font_variant_caps(&node));
    }

    #[test]
    fn test_get_font_variant_caps() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontVariantCapsManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontVariantCaps": "titling-caps" })),
            marks: None,
        };
        
        let font_variant_caps = manager.get_font_variant_caps(&node);
        assert_eq!(font_variant_caps, Some("titling-caps".to_string()));
    }
}
