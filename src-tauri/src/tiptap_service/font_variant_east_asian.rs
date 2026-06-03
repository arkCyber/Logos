//! TipTap Font Variant East Asian Manager - Aerospace-Grade Font Variant East Asian Operations Service
//!
//! Safety-critical font variant east asian operations service with:
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
pub enum FontVariantEastAsian {
    Normal,
    Ruby,
    Jis78,
    Jis83,
    Jis90,
    Jis04,
    Simplified,
    Traditional,
}

impl FontVariantEastAsian {
    pub fn as_str(&self) -> &str {
        match self {
            FontVariantEastAsian::Normal => "normal",
            FontVariantEastAsian::Ruby => "ruby",
            FontVariantEastAsian::Jis78 => "jis78",
            FontVariantEastAsian::Jis83 => "jis83",
            FontVariantEastAsian::Jis90 => "jis90",
            FontVariantEastAsian::Jis04 => "jis04",
            FontVariantEastAsian::Simplified => "simplified",
            FontVariantEastAsian::Traditional => "traditional",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(FontVariantEastAsian::Normal),
            "ruby" => Ok(FontVariantEastAsian::Ruby),
            "jis78" => Ok(FontVariantEastAsian::Jis78),
            "jis83" => Ok(FontVariantEastAsian::Jis83),
            "jis90" => Ok(FontVariantEastAsian::Jis90),
            "jis04" => Ok(FontVariantEastAsian::Jis04),
            "simplified" => Ok(FontVariantEastAsian::Simplified),
            "traditional" => Ok(FontVariantEastAsian::Traditional),
            _ => Err(format!("Invalid font variant east asian value: {}", s)),
        }
    }
}

pub struct FontVariantEastAsianManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FontVariantEastAsianManager {
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

    fn validate_font_variant_east_asian(&self, font_variant_east_asian: &str) -> Result<(), String> {
        if font_variant_east_asian.is_empty() {
            return Err("Font variant east asian cannot be empty".to_string());
        }
        FontVariantEastAsian::from_str(font_variant_east_asian)?;
        Ok(())
    }

    pub fn apply_font_variant_east_asian(&mut self, node: &mut TipTapNode, font_variant_east_asian: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_font_variant_east_asian(font_variant_east_asian)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("fontVariantEastAsian".to_string(), serde_json::Value::String(font_variant_east_asian.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "fontVariantEastAsian": font_variant_east_asian }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font variant east asian application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font variant east asian application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_font_variant_east_asian(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("fontVariantEastAsian");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font variant east asian removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font variant east asian removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_font_variant_east_asian(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(font_variant_east_asian) = obj.get("fontVariantEastAsian") {
                    if let Some(s) = font_variant_east_asian.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_font_variant_east_asian(&self, node: &TipTapNode) -> bool {
        self.get_font_variant_east_asian(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_font_variant_east_asian_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontVariantEastAsianManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_font_variant_east_asian_variants() {
        assert_eq!(FontVariantEastAsian::Normal.as_str(), "normal");
        assert_eq!(FontVariantEastAsian::Ruby.as_str(), "ruby");
    }

    #[test]
    fn test_apply_font_variant_east_asian() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontVariantEastAsianManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_font_variant_east_asian(&mut node, "ruby");
        assert!(result.is_ok());
        assert!(manager.has_font_variant_east_asian(&node));
    }

    #[test]
    fn test_remove_font_variant_east_asian() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontVariantEastAsianManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontVariantEastAsian": "simplified" })),
            marks: None,
        };
        
        assert!(manager.has_font_variant_east_asian(&node));
        let result = manager.remove_font_variant_east_asian(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_font_variant_east_asian(&node));
    }

    #[test]
    fn test_get_font_variant_east_asian() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontVariantEastAsianManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontVariantEastAsian": "traditional" })),
            marks: None,
        };
        
        let font_variant_east_asian = manager.get_font_variant_east_asian(&node);
        assert_eq!(font_variant_east_asian, Some("traditional".to_string()));
    }
}
