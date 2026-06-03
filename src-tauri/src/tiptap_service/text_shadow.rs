//! TipTap Text Shadow Manager - Aerospace-Grade Text Shadow Operations Service
//!
//! Safety-critical text shadow operations service with:
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

/// Maximum shadow offset (in pixels)
const MAX_SHADOW_OFFSET: f64 = 100.0;

/// Maximum shadow blur radius (in pixels)
const MAX_SHADOW_BLUR: f64 = 50.0;

/// Maximum shadow color string length
const MAX_SHADOW_COLOR_LENGTH: usize = 50;

/// Text shadow attributes
#[derive(Debug, Clone)]
pub struct TextShadowAttributes {
    pub color: String,
    pub offset_x: f64,
    pub offset_y: f64,
    pub blur_radius: f64,
}

impl Default for TextShadowAttributes {
    fn default() -> Self {
        Self {
            color: "#000000".to_string(),
            offset_x: 0.0,
            offset_y: 0.0,
            blur_radius: 0.0,
        }
    }
}

pub struct TextShadowManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextShadowManager {
    /// Creates a new text shadow manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new TextShadowManager instance
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
        }
    }

    /// Get the performance warning threshold
    /// 
    /// # Returns
    /// The performance warning threshold in milliseconds
    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    /// Get the performance critical threshold
    /// 
    /// # Returns
    /// The performance critical threshold in milliseconds
    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    /// Get the maximum shadow offset constant
    /// 
    /// # Returns
    /// The maximum shadow offset in pixels
    pub fn max_shadow_offset() -> f64 {
        MAX_SHADOW_OFFSET
    }

    /// Get the maximum shadow blur constant
    /// 
    /// # Returns
    /// The maximum shadow blur radius in pixels
    pub fn max_shadow_blur() -> f64 {
        MAX_SHADOW_BLUR
    }

    /// Get the maximum shadow color length constant
    /// 
    /// # Returns
    /// The maximum shadow color string length
    pub fn max_shadow_color_length() -> usize {
        MAX_SHADOW_COLOR_LENGTH
    }

    /// Record error context
    fn record_error(&mut self, code: &str, message: &str, source: &str) {
        self.last_error = Some(ErrorContext::new(
            ErrorSeverity::Error,
            code,
            message,
            source,
        ));
    }

    /// Get last error
    pub fn get_last_error(&self) -> Option<&ErrorContext> {
        self.last_error.as_ref()
    }

    /// Get operation count
    pub fn get_operation_count(&self) -> u64 {
        self.operation_count
    }

    /// Reset error state
    pub fn reset_error_state(&mut self) {
        self.last_error = None;
    }

    /// Reset operation count
    pub fn reset_operation_count(&mut self) {
        self.operation_count = 0;
    }

    /// Validate shadow color
    /// 
    /// # Arguments
    /// * `color` - The shadow color to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting color string length
    fn validate_shadow_color(&self, color: &str) -> Result<(), String> {
        if color.is_empty() {
            return Err("Shadow color cannot be empty".to_string());
        }
        if color.len() > MAX_SHADOW_COLOR_LENGTH {
            return Err(format!("Shadow color exceeds maximum length of {} characters", MAX_SHADOW_COLOR_LENGTH));
        }
        // Basic hex color validation
        if color.starts_with('#') {
            if color.len() != 4 && color.len() != 7 {
                return Err("Invalid hex color format".to_string());
            }
        }
        Ok(())
    }

    /// Validate shadow offset
    /// 
    /// # Arguments
    /// * `offset` - The shadow offset to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Ensures offset is within valid range
    fn validate_shadow_offset(&self, offset: f64) -> Result<(), String> {
        if offset.abs() > MAX_SHADOW_OFFSET {
            return Err(format!("Shadow offset cannot exceed {} pixels", MAX_SHADOW_OFFSET));
        }
        if !offset.is_finite() {
            return Err("Shadow offset must be a finite number".to_string());
        }
        Ok(())
    }

    /// Validate shadow blur
    /// 
    /// # Arguments
    /// * `blur` - The shadow blur to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Ensures blur is within valid range
    fn validate_shadow_blur(&self, blur: f64) -> Result<(), String> {
        if blur < 0.0 {
            return Err("Shadow blur cannot be negative".to_string());
        }
        if blur > MAX_SHADOW_BLUR {
            return Err(format!("Shadow blur cannot exceed {} pixels", MAX_SHADOW_BLUR));
        }
        if !blur.is_finite() {
            return Err("Shadow blur must be a finite number".to_string());
        }
        Ok(())
    }

    /// Apply text shadow to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply shadow to
    /// * `attributes` - The shadow attributes
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates all shadow attributes
    pub fn apply_text_shadow(&mut self, node: &mut TipTapNode, attributes: TextShadowAttributes) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate attributes
        self.validate_shadow_color(&attributes.color)?;
        self.validate_shadow_offset(attributes.offset_x)?;
        self.validate_shadow_offset(attributes.offset_y)?;
        self.validate_shadow_blur(attributes.blur_radius)?;

        // Apply shadow to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textShadowColor".to_string(), serde_json::json!(attributes.color));
                obj.insert("textShadowOffsetX".to_string(), serde_json::json!(attributes.offset_x));
                obj.insert("textShadowOffsetY".to_string(), serde_json::json!(attributes.offset_y));
                obj.insert("textShadowBlur".to_string(), serde_json::json!(attributes.blur_radius));
            }
        } else {
            node.attrs = Some(serde_json::json!({
                "textShadowColor": attributes.color,
                "textShadowOffsetX": attributes.offset_x,
                "textShadowOffsetY": attributes.offset_y,
                "textShadowBlur": attributes.blur_radius
            }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text shadow application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text shadow application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove text shadow from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove shadow from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_text_shadow(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textShadowColor");
                obj.remove("textShadowOffsetX");
                obj.remove("textShadowOffsetY");
                obj.remove("textShadowBlur");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text shadow removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text shadow removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get text shadow attributes from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get shadow attributes from
    /// 
    /// # Returns
    /// Option containing the shadow attributes or None
    pub fn get_text_shadow(&self, node: &TipTapNode) -> Option<TextShadowAttributes> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                let color = obj.get("textShadowColor").and_then(|v| v.as_str())?.to_string();
                let offset_x = obj.get("textShadowOffsetX").and_then(|v| v.as_f64())?;
                let offset_y = obj.get("textShadowOffsetY").and_then(|v| v.as_f64())?;
                let blur_radius = obj.get("textShadowBlur").and_then(|v| v.as_f64())?;
                return Some(TextShadowAttributes {
                    color,
                    offset_x,
                    offset_y,
                    blur_radius,
                });
            }
        }
        None
    }

    /// Check if node has text shadow
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has text shadow, false otherwise
    pub fn has_text_shadow(&self, node: &TipTapNode) -> bool {
        self.get_text_shadow(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_shadow_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextShadowManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(TextShadowManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(TextShadowManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(TextShadowManager::max_shadow_offset(), MAX_SHADOW_OFFSET);
        assert_eq!(TextShadowManager::max_shadow_blur(), MAX_SHADOW_BLUR);
        assert_eq!(TextShadowManager::max_shadow_color_length(), MAX_SHADOW_COLOR_LENGTH);
    }

    #[test]
    fn test_text_shadow_attributes_default() {
        let attrs = TextShadowAttributes::default();
        assert_eq!(attrs.color, "#000000");
        assert_eq!(attrs.offset_x, 0.0);
        assert_eq!(attrs.offset_y, 0.0);
        assert_eq!(attrs.blur_radius, 0.0);
    }

    #[test]
    fn test_apply_text_shadow() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextShadowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = TextShadowAttributes {
            color: "#ff0000".to_string(),
            offset_x: 2.0,
            offset_y: 2.0,
            blur_radius: 4.0,
        };
        let result = manager.apply_text_shadow(&mut node, attributes);
        assert!(result.is_ok());
        assert!(manager.has_text_shadow(&node));
    }

    #[test]
    fn test_apply_text_shadow_empty_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextShadowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = TextShadowAttributes {
            color: "".to_string(),
            offset_x: 2.0,
            offset_y: 2.0,
            blur_radius: 4.0,
        };
        let result = manager.apply_text_shadow(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_text_shadow_color_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextShadowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = TextShadowAttributes {
            color: "#".repeat(MAX_SHADOW_COLOR_LENGTH + 1),
            offset_x: 2.0,
            offset_y: 2.0,
            blur_radius: 4.0,
        };
        let result = manager.apply_text_shadow(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_text_shadow_offset_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextShadowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = TextShadowAttributes {
            color: "#ff0000".to_string(),
            offset_x: MAX_SHADOW_OFFSET + 1.0,
            offset_y: 2.0,
            blur_radius: 4.0,
        };
        let result = manager.apply_text_shadow(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_text_shadow_blur_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextShadowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = TextShadowAttributes {
            color: "#ff0000".to_string(),
            offset_x: 2.0,
            offset_y: 2.0,
            blur_radius: MAX_SHADOW_BLUR + 1.0,
        };
        let result = manager.apply_text_shadow(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_text_shadow() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextShadowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({
                "textShadowColor": "#ff0000",
                "textShadowOffsetX": 2.0,
                "textShadowOffsetY": 2.0,
                "textShadowBlur": 4.0
            })),
            marks: None,
        };
        
        assert!(manager.has_text_shadow(&node));
        let result = manager.remove_text_shadow(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_text_shadow(&node));
    }

    #[test]
    fn test_get_text_shadow() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextShadowManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({
                "textShadowColor": "#00ff00",
                "textShadowOffsetX": 3.0,
                "textShadowOffsetY": 3.0,
                "textShadowBlur": 5.0
            })),
            marks: None,
        };
        
        let shadow = manager.get_text_shadow(&node);
        assert!(shadow.is_some());
        let attrs = shadow.unwrap();
        assert_eq!(attrs.color, "#00ff00");
        assert_eq!(attrs.offset_x, 3.0);
        assert_eq!(attrs.offset_y, 3.0);
        assert_eq!(attrs.blur_radius, 5.0);
    }

    #[test]
    fn test_get_text_shadow_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextShadowManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let shadow = manager.get_text_shadow(&node);
        assert!(shadow.is_none());
    }

    #[test]
    fn test_has_text_shadow() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextShadowManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({
                "textShadowColor": "#ff0000",
                "textShadowOffsetX": 2.0,
                "textShadowOffsetY": 2.0,
                "textShadowBlur": 4.0
            })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_text_shadow(&node_with));
        assert!(!manager.has_text_shadow(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextShadowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = TextShadowAttributes::default();
        manager.apply_text_shadow(&mut node, attributes).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextShadowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = TextShadowAttributes::default();
        manager.apply_text_shadow(&mut node, attributes).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextShadowManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        
        let last_error = manager.get_last_error();
        assert!(last_error.is_some());
        let error = last_error.unwrap();
        assert_eq!(error.code, "TEST_ERROR");
        assert_eq!(error.message, "Test error message");
        assert_eq!(error.source, "test_source");
    }

    #[test]
    fn test_error_state_reset() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextShadowManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
