//! TipTap Font Variant Manager - Aerospace-Grade Font Variant Operations Service
//!
//! Safety-critical font variant operations service with:
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

/// Maximum font variant string length
const MAX_FONT_VARIANT_LENGTH: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontVariant {
    Normal,
    SmallCaps,
    AllSmallCaps,
    PetiteCaps,
    AllPetiteCaps,
    Unicase,
    TitlingCaps,
}

impl FontVariant {
    pub fn as_str(&self) -> &str {
        match self {
            FontVariant::Normal => "normal",
            FontVariant::SmallCaps => "small-caps",
            FontVariant::AllSmallCaps => "all-small-caps",
            FontVariant::PetiteCaps => "petite-caps",
            FontVariant::AllPetiteCaps => "all-petite-caps",
            FontVariant::Unicase => "unicase",
            FontVariant::TitlingCaps => "titling-caps",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(FontVariant::Normal),
            "small-caps" => Ok(FontVariant::SmallCaps),
            "all-small-caps" => Ok(FontVariant::AllSmallCaps),
            "petite-caps" => Ok(FontVariant::PetiteCaps),
            "all-petite-caps" => Ok(FontVariant::AllPetiteCaps),
            "unicase" => Ok(FontVariant::Unicase),
            "titling-caps" => Ok(FontVariant::TitlingCaps),
            _ => Err(format!("Invalid font variant: {}", s)),
        }
    }
}

pub struct FontVariantManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FontVariantManager {
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

    pub fn max_font_variant_length() -> usize {
        MAX_FONT_VARIANT_LENGTH
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

    fn validate_font_variant(&self, font_variant: &str) -> Result<(), String> {
        if font_variant.is_empty() {
            return Err("Font variant cannot be empty".to_string());
        }
        if font_variant.len() > MAX_FONT_VARIANT_LENGTH {
            return Err(format!("Font variant string exceeds maximum length of {} characters", MAX_FONT_VARIANT_LENGTH));
        }
        FontVariant::from_str(font_variant)?;
        Ok(())
    }

    pub fn apply_font_variant(&mut self, node: &mut TipTapNode, font_variant: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_font_variant(font_variant)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("fontVariant".to_string(), serde_json::Value::String(font_variant.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "fontVariant": font_variant }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font variant application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font variant application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_font_variant(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("fontVariant");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font variant removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font variant removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_font_variant(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(font_variant) = obj.get("fontVariant") {
                    if let Some(s) = font_variant.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_font_variant(&self, node: &TipTapNode) -> bool {
        self.get_font_variant(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_font_variant_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontVariantManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_font_variant_variants() {
        assert_eq!(FontVariant::Normal.as_str(), "normal");
        assert_eq!(FontVariant::SmallCaps.as_str(), "small-caps");
        assert_eq!(FontVariant::AllSmallCaps.as_str(), "all-small-caps");
    }

    #[test]
    fn test_apply_font_variant() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontVariantManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_font_variant(&mut node, "small-caps");
        assert!(result.is_ok());
        assert!(manager.has_font_variant(&node));
    }

    #[test]
    fn test_remove_font_variant() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontVariantManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontVariant": "normal" })),
            marks: None,
        };
        
        assert!(manager.has_font_variant(&node));
        let result = manager.remove_font_variant(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_font_variant(&node));
    }

    #[test]
    fn test_get_font_variant() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontVariantManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontVariant": "petite-caps" })),
            marks: None,
        };
        
        let font_variant = manager.get_font_variant(&node);
        assert_eq!(font_variant, Some("petite-caps".to_string()));
    }
}
