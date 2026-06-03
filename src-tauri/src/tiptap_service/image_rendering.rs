//! TipTap Image Rendering Manager - Aerospace-Grade Image Rendering Operations Service
//!
//! Safety-critical image rendering operations service with:
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

/// Maximum image rendering string length
const MAX_IMAGE_RENDERING_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageRendering {
    Auto,
    CrispEdges,
    Pixelated,
}

impl ImageRendering {
    pub fn as_str(&self) -> &str {
        match self {
            ImageRendering::Auto => "auto",
            ImageRendering::CrispEdges => "crisp-edges",
            ImageRendering::Pixelated => "pixelated",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(ImageRendering::Auto),
            "crisp-edges" => Ok(ImageRendering::CrispEdges),
            "pixelated" => Ok(ImageRendering::Pixelated),
            _ => Err(format!("Invalid image rendering: {}", s)),
        }
    }
}

pub struct ImageRenderingManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ImageRenderingManager {
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

    pub fn max_image_rendering_length() -> usize {
        MAX_IMAGE_RENDERING_LENGTH
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

    fn validate_image_rendering(&self, image_rendering: &str) -> Result<(), String> {
        if image_rendering.len() > MAX_IMAGE_RENDERING_LENGTH {
            return Err(format!("Image rendering string exceeds maximum length of {} characters", MAX_IMAGE_RENDERING_LENGTH));
        }
        ImageRendering::from_str(image_rendering)?;
        Ok(())
    }

    pub fn apply_image_rendering(&mut self, node: &mut TipTapNode, image_rendering: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_image_rendering(image_rendering)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("imageRendering".to_string(), serde_json::Value::String(image_rendering.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "imageRendering": image_rendering }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Image rendering application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Image rendering application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_image_rendering(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("imageRendering");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Image rendering removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Image rendering removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_image_rendering(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(image_rendering) = obj.get("imageRendering") {
                    if let Some(s) = image_rendering.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_image_rendering(&self, node: &TipTapNode) -> bool {
        self.get_image_rendering(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_image_rendering_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ImageRenderingManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_image_rendering_variants() {
        assert_eq!(ImageRendering::Auto.as_str(), "auto");
        assert_eq!(ImageRendering::Pixelated.as_str(), "pixelated");
    }

    #[test]
    fn test_apply_image_rendering() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ImageRenderingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_image_rendering(&mut node, "pixelated");
        assert!(result.is_ok());
        assert!(manager.has_image_rendering(&node));
    }

    #[test]
    fn test_remove_image_rendering() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ImageRenderingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "imageRendering": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_image_rendering(&node));
        let result = manager.remove_image_rendering(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_image_rendering(&node));
    }

    #[test]
    fn test_get_image_rendering() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ImageRenderingManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "imageRendering": "crisp-edges" })),
            marks: None,
        };
        
        let image_rendering = manager.get_image_rendering(&node);
        assert_eq!(image_rendering, Some("crisp-edges".to_string()));
    }
}
