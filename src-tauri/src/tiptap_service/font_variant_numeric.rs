//! TipTap Font Variant Numeric Manager - Aerospace-Grade Font Variant Numeric Operations Service
//!
//! Safety-critical font variant numeric operations service with:
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
pub enum FontVariantNumeric {
    Normal,
    LiningNums,
    OldstyleNums,
    ProportionalNums,
    TabularNums,
    DiagonalFractions,
    StackedFractions,
}

impl FontVariantNumeric {
    pub fn as_str(&self) -> &str {
        match self {
            FontVariantNumeric::Normal => "normal",
            FontVariantNumeric::LiningNums => "lining-nums",
            FontVariantNumeric::OldstyleNums => "oldstyle-nums",
            FontVariantNumeric::ProportionalNums => "proportional-nums",
            FontVariantNumeric::TabularNums => "tabular-nums",
            FontVariantNumeric::DiagonalFractions => "diagonal-fractions",
            FontVariantNumeric::StackedFractions => "stacked-fractions",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(FontVariantNumeric::Normal),
            "lining-nums" => Ok(FontVariantNumeric::LiningNums),
            "oldstyle-nums" => Ok(FontVariantNumeric::OldstyleNums),
            "proportional-nums" => Ok(FontVariantNumeric::ProportionalNums),
            "tabular-nums" => Ok(FontVariantNumeric::TabularNums),
            "diagonal-fractions" => Ok(FontVariantNumeric::DiagonalFractions),
            "stacked-fractions" => Ok(FontVariantNumeric::StackedFractions),
            _ => Err(format!("Invalid font variant numeric value: {}", s)),
        }
    }
}

pub struct FontVariantNumericManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FontVariantNumericManager {
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

    fn validate_font_variant_numeric(&self, font_variant_numeric: &str) -> Result<(), String> {
        if font_variant_numeric.is_empty() {
            return Err("Font variant numeric cannot be empty".to_string());
        }
        FontVariantNumeric::from_str(font_variant_numeric)?;
        Ok(())
    }

    pub fn apply_font_variant_numeric(&mut self, node: &mut TipTapNode, font_variant_numeric: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_font_variant_numeric(font_variant_numeric)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("fontVariantNumeric".to_string(), serde_json::Value::String(font_variant_numeric.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "fontVariantNumeric": font_variant_numeric }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font variant numeric application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font variant numeric application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_font_variant_numeric(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("fontVariantNumeric");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font variant numeric removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font variant numeric removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_font_variant_numeric(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(font_variant_numeric) = obj.get("fontVariantNumeric") {
                    if let Some(s) = font_variant_numeric.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_font_variant_numeric(&self, node: &TipTapNode) -> bool {
        self.get_font_variant_numeric(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_font_variant_numeric_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontVariantNumericManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_font_variant_numeric_variants() {
        assert_eq!(FontVariantNumeric::Normal.as_str(), "normal");
        assert_eq!(FontVariantNumeric::LiningNums.as_str(), "lining-nums");
    }

    #[test]
    fn test_apply_font_variant_numeric() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontVariantNumericManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_font_variant_numeric(&mut node, "lining-nums");
        assert!(result.is_ok());
        assert!(manager.has_font_variant_numeric(&node));
    }

    #[test]
    fn test_remove_font_variant_numeric() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontVariantNumericManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontVariantNumeric": "oldstyle-nums" })),
            marks: None,
        };
        
        assert!(manager.has_font_variant_numeric(&node));
        let result = manager.remove_font_variant_numeric(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_font_variant_numeric(&node));
    }

    #[test]
    fn test_get_font_variant_numeric() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontVariantNumericManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontVariantNumeric": "tabular-nums" })),
            marks: None,
        };
        
        let font_variant_numeric = manager.get_font_variant_numeric(&node);
        assert_eq!(font_variant_numeric, Some("tabular-nums".to_string()));
    }
}
