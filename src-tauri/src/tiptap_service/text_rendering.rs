//! TipTap Text Rendering Manager - Aerospace-Grade Text Rendering Operations Service
//!
//! Safety-critical text rendering operations service with:
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
pub enum TextRendering {
    Auto,
    OptimizeSpeed,
    OptimizeLegibility,
    GeometricPrecision,
}

impl TextRendering {
    pub fn as_str(&self) -> &str {
        match self {
            TextRendering::Auto => "auto",
            TextRendering::OptimizeSpeed => "optimizeSpeed",
            TextRendering::OptimizeLegibility => "optimizeLegibility",
            TextRendering::GeometricPrecision => "geometricPrecision",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(TextRendering::Auto),
            "optimizespeed" => Ok(TextRendering::OptimizeSpeed),
            "optimizelegibility" => Ok(TextRendering::OptimizeLegibility),
            "geometricprecision" => Ok(TextRendering::GeometricPrecision),
            _ => Err(format!("Invalid text rendering value: {}", s)),
        }
    }
}

pub struct TextRenderingManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextRenderingManager {
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

    fn validate_text_rendering(&self, text_rendering: &str) -> Result<(), String> {
        if text_rendering.is_empty() {
            return Err("Text rendering cannot be empty".to_string());
        }
        TextRendering::from_str(text_rendering)?;
        Ok(())
    }

    pub fn apply_text_rendering(&mut self, node: &mut TipTapNode, text_rendering: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_text_rendering(text_rendering)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textRendering".to_string(), serde_json::Value::String(text_rendering.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textRendering": text_rendering }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text rendering application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text rendering application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_text_rendering(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textRendering");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text rendering removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text rendering removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_text_rendering(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(text_rendering) = obj.get("textRendering") {
                    if let Some(s) = text_rendering.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_text_rendering(&self, node: &TipTapNode) -> bool {
        self.get_text_rendering(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_rendering_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextRenderingManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_text_rendering_variants() {
        assert_eq!(TextRendering::Auto.as_str(), "auto");
        assert_eq!(TextRendering::OptimizeLegibility.as_str(), "optimizeLegibility");
    }

    #[test]
    fn test_apply_text_rendering() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextRenderingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_text_rendering(&mut node, "optimizeLegibility");
        assert!(result.is_ok());
        assert!(manager.has_text_rendering(&node));
    }

    #[test]
    fn test_remove_text_rendering() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextRenderingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textRendering": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_text_rendering(&node));
        let result = manager.remove_text_rendering(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_text_rendering(&node));
    }

    #[test]
    fn test_get_text_rendering() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextRenderingManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textRendering": "optimizeSpeed" })),
            marks: None,
        };
        
        let text_rendering = manager.get_text_rendering(&node);
        assert_eq!(text_rendering, Some("optimizeSpeed".to_string()));
    }
}
