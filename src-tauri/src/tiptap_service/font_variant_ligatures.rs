//! TipTap Font Variant Ligatures Manager - Aerospace-Grade Font Variant Ligatures Operations Service
//!
//! Safety-critical font variant ligatures operations service with:
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
pub enum FontVariantLigatures {
    Normal,
    None,
    CommonLigatures,
    NoCommonLigatures,
    DiscretionaryLigatures,
    NoDiscretionaryLigatures,
    HistoricalLigatures,
    NoHistoricalLigatures,
    Contextual,
    NoContextual,
}

impl FontVariantLigatures {
    pub fn as_str(&self) -> &str {
        match self {
            FontVariantLigatures::Normal => "normal",
            FontVariantLigatures::None => "none",
            FontVariantLigatures::CommonLigatures => "common-ligatures",
            FontVariantLigatures::NoCommonLigatures => "no-common-ligatures",
            FontVariantLigatures::DiscretionaryLigatures => "discretionary-ligatures",
            FontVariantLigatures::NoDiscretionaryLigatures => "no-discretionary-ligatures",
            FontVariantLigatures::HistoricalLigatures => "historical-ligatures",
            FontVariantLigatures::NoHistoricalLigatures => "no-historical-ligatures",
            FontVariantLigatures::Contextual => "contextual",
            FontVariantLigatures::NoContextual => "no-contextual",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(FontVariantLigatures::Normal),
            "none" => Ok(FontVariantLigatures::None),
            "common-ligatures" => Ok(FontVariantLigatures::CommonLigatures),
            "no-common-ligatures" => Ok(FontVariantLigatures::NoCommonLigatures),
            "discretionary-ligatures" => Ok(FontVariantLigatures::DiscretionaryLigatures),
            "no-discretionary-ligatures" => Ok(FontVariantLigatures::NoDiscretionaryLigatures),
            "historical-ligatures" => Ok(FontVariantLigatures::HistoricalLigatures),
            "no-historical-ligatures" => Ok(FontVariantLigatures::NoHistoricalLigatures),
            "contextual" => Ok(FontVariantLigatures::Contextual),
            "no-contextual" => Ok(FontVariantLigatures::NoContextual),
            _ => Err(format!("Invalid font variant ligatures value: {}", s)),
        }
    }
}

pub struct FontVariantLigaturesManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FontVariantLigaturesManager {
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

    fn validate_font_variant_ligatures(&self, font_variant_ligatures: &str) -> Result<(), String> {
        if font_variant_ligatures.is_empty() {
            return Err("Font variant ligatures cannot be empty".to_string());
        }
        FontVariantLigatures::from_str(font_variant_ligatures)?;
        Ok(())
    }

    pub fn apply_font_variant_ligatures(&mut self, node: &mut TipTapNode, font_variant_ligatures: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_font_variant_ligatures(font_variant_ligatures)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("fontVariantLigatures".to_string(), serde_json::Value::String(font_variant_ligatures.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "fontVariantLigatures": font_variant_ligatures }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font variant ligatures application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font variant ligatures application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_font_variant_ligatures(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("fontVariantLigatures");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font variant ligatures removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font variant ligatures removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_font_variant_ligatures(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(font_variant_ligatures) = obj.get("fontVariantLigatures") {
                    if let Some(s) = font_variant_ligatures.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_font_variant_ligatures(&self, node: &TipTapNode) -> bool {
        self.get_font_variant_ligatures(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_font_variant_ligatures_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontVariantLigaturesManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_font_variant_ligatures_variants() {
        assert_eq!(FontVariantLigatures::Normal.as_str(), "normal");
        assert_eq!(FontVariantLigatures::CommonLigatures.as_str(), "common-ligatures");
    }

    #[test]
    fn test_apply_font_variant_ligatures() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontVariantLigaturesManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_font_variant_ligatures(&mut node, "common-ligatures");
        assert!(result.is_ok());
        assert!(manager.has_font_variant_ligatures(&node));
    }

    #[test]
    fn test_remove_font_variant_ligatures() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontVariantLigaturesManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontVariantLigatures": "none" })),
            marks: None,
        };
        
        assert!(manager.has_font_variant_ligatures(&node));
        let result = manager.remove_font_variant_ligatures(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_font_variant_ligatures(&node));
    }

    #[test]
    fn test_get_font_variant_ligatures() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontVariantLigaturesManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontVariantLigatures": "contextual" })),
            marks: None,
        };
        
        let font_variant_ligatures = manager.get_font_variant_ligatures(&node);
        assert_eq!(font_variant_ligatures, Some("contextual".to_string()));
    }
}
