//! TipTap Box Shadow Manager - Aerospace-Grade Box Shadow Operations Service
//!
//! Safety-critical box shadow operations service with:
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

/// Maximum shadow blur (in pixels)
const MAX_SHADOW_BLUR: f64 = 50.0;

/// Maximum shadow spread (in pixels)
const MAX_SHADOW_SPREAD: f64 = 50.0;

/// Maximum shadow color string length
const MAX_SHADOW_COLOR_LENGTH: usize = 50;

/// Box shadow attributes
#[derive(Debug, Clone)]
pub struct BoxShadowAttributes {
    pub x_offset: f64,
    pub y_offset: f64,
    pub blur: f64,
    pub spread: f64,
    pub color: String,
    pub inset: bool,
}

impl Default for BoxShadowAttributes {
    fn default() -> Self {
        Self {
            x_offset: 0.0,
            y_offset: 2.0,
            blur: 4.0,
            spread: 0.0,
            color: "#000000".to_string(),
            inset: false,
        }
    }
}

pub struct BoxShadowManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl BoxShadowManager {
    /// Creates a new box shadow manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new BoxShadowManager instance
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
    /// The maximum shadow blur in pixels
    pub fn max_shadow_blur() -> f64 {
        MAX_SHADOW_BLUR
    }

    /// Get the maximum shadow spread constant
    /// 
    /// # Returns
    /// The maximum shadow spread in pixels
    pub fn max_shadow_spread() -> f64 {
        MAX_SHADOW_SPREAD
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

    /// Validate shadow offset
    /// 
    /// # Arguments
    /// * `offset` - The offset to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    fn validate_offset(&self, offset: f64) -> Result<(), String> {
        if offset.abs() > MAX_SHADOW_OFFSET {
            return Err(format!("Shadow offset cannot exceed {}", MAX_SHADOW_OFFSET));
        }
        if !offset.is_finite() {
            return Err("Shadow offset must be a finite number".to_string());
        }
        Ok(())
    }

    /// Validate shadow blur
    /// 
    /// # Arguments
    /// * `blur` - The blur to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    fn validate_blur(&self, blur: f64) -> Result<(), String> {
        if blur < 0.0 || blur > MAX_SHADOW_BLUR {
            return Err(format!("Shadow blur must be between 0 and {}", MAX_SHADOW_BLUR));
        }
        if !blur.is_finite() {
            return Err("Shadow blur must be a finite number".to_string());
        }
        Ok(())
    }

    /// Validate shadow spread
    /// 
    /// # Arguments
    /// * `spread` - The spread to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    fn validate_spread(&self, spread: f64) -> Result<(), String> {
        if spread.abs() > MAX_SHADOW_SPREAD {
            return Err(format!("Shadow spread cannot exceed {}", MAX_SHADOW_SPREAD));
        }
        if !spread.is_finite() {
            return Err("Shadow spread must be a finite number".to_string());
        }
        Ok(())
    }

    /// Validate shadow color
    /// 
    /// # Arguments
    /// * `color` - The color to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    fn validate_color(&self, color: &str) -> Result<(), String> {
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

    /// Apply box shadow to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply box shadow to
    /// * `attributes` - The box shadow attributes
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
    pub fn apply_box_shadow(&mut self, node: &mut TipTapNode, attributes: BoxShadowAttributes) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate attributes
        self.validate_offset(attributes.x_offset)?;
        self.validate_offset(attributes.y_offset)?;
        self.validate_blur(attributes.blur)?;
        self.validate_spread(attributes.spread)?;
        self.validate_color(&attributes.color)?;

        // Apply box shadow to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("boxShadowXOffset".to_string(), serde_json::json!(attributes.x_offset));
                obj.insert("boxShadowYOffset".to_string(), serde_json::json!(attributes.y_offset));
                obj.insert("boxShadowBlur".to_string(), serde_json::json!(attributes.blur));
                obj.insert("boxShadowSpread".to_string(), serde_json::json!(attributes.spread));
                obj.insert("boxShadowColor".to_string(), serde_json::json!(attributes.color));
                obj.insert("boxShadowInset".to_string(), serde_json::json!(attributes.inset));
            }
        } else {
            node.attrs = Some(serde_json::json!({
                "boxShadowXOffset": attributes.x_offset,
                "boxShadowYOffset": attributes.y_offset,
                "boxShadowBlur": attributes.blur,
                "boxShadowSpread": attributes.spread,
                "boxShadowColor": attributes.color,
                "boxShadowInset": attributes.inset
            }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Box shadow application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Box shadow application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove box shadow from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove box shadow from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_box_shadow(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("boxShadowXOffset");
                obj.remove("boxShadowYOffset");
                obj.remove("boxShadowBlur");
                obj.remove("boxShadowSpread");
                obj.remove("boxShadowColor");
                obj.remove("boxShadowInset");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Box shadow removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Box shadow removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get box shadow from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get box shadow from
    /// 
    /// # Returns
    /// Option containing the box shadow attributes or None
    pub fn get_box_shadow(&self, node: &TipTapNode) -> Option<BoxShadowAttributes> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                let x_offset = obj.get("boxShadowXOffset").and_then(|v| v.as_f64())?;
                let y_offset = obj.get("boxShadowYOffset").and_then(|v| v.as_f64())?;
                let blur = obj.get("boxShadowBlur").and_then(|v| v.as_f64())?;
                let spread = obj.get("boxShadowSpread").and_then(|v| v.as_f64())?;
                let color = obj.get("boxShadowColor").and_then(|v| v.as_str())?.to_string();
                let inset = obj.get("boxShadowInset").and_then(|v| v.as_bool()).unwrap_or(false);
                return Some(BoxShadowAttributes {
                    x_offset,
                    y_offset,
                    blur,
                    spread,
                    color,
                    inset,
                });
            }
        }
        None
    }

    /// Check if node has box shadow
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has box shadow, false otherwise
    pub fn has_box_shadow(&self, node: &TipTapNode) -> bool {
        self.get_box_shadow(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_box_shadow_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BoxShadowManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(BoxShadowManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(BoxShadowManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(BoxShadowManager::max_shadow_offset(), MAX_SHADOW_OFFSET);
        assert_eq!(BoxShadowManager::max_shadow_blur(), MAX_SHADOW_BLUR);
        assert_eq!(BoxShadowManager::max_shadow_spread(), MAX_SHADOW_SPREAD);
        assert_eq!(BoxShadowManager::max_shadow_color_length(), MAX_SHADOW_COLOR_LENGTH);
    }

    #[test]
    fn test_box_shadow_attributes_default() {
        let attrs = BoxShadowAttributes::default();
        assert_eq!(attrs.x_offset, 0.0);
        assert_eq!(attrs.y_offset, 2.0);
        assert_eq!(attrs.blur, 4.0);
        assert_eq!(attrs.spread, 0.0);
        assert_eq!(attrs.color, "#000000");
        assert_eq!(attrs.inset, false);
    }

    #[test]
    fn test_apply_box_shadow() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BoxShadowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = BoxShadowAttributes {
            x_offset: 5.0,
            y_offset: 5.0,
            blur: 10.0,
            spread: 0.0,
            color: "#ff0000".to_string(),
            inset: false,
        };
        let result = manager.apply_box_shadow(&mut node, attributes);
        assert!(result.is_ok());
        assert!(manager.has_box_shadow(&node));
    }

    #[test]
    fn test_apply_box_shadow_offset_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BoxShadowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = BoxShadowAttributes {
            x_offset: MAX_SHADOW_OFFSET + 1.0,
            y_offset: 5.0,
            blur: 10.0,
            spread: 0.0,
            color: "#ff0000".to_string(),
            inset: false,
        };
        let result = manager.apply_box_shadow(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_box_shadow_blur_negative() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BoxShadowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = BoxShadowAttributes {
            x_offset: 5.0,
            y_offset: 5.0,
            blur: -1.0,
            spread: 0.0,
            color: "#ff0000".to_string(),
            inset: false,
        };
        let result = manager.apply_box_shadow(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_box_shadow_color_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BoxShadowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = BoxShadowAttributes {
            x_offset: 5.0,
            y_offset: 5.0,
            blur: 10.0,
            spread: 0.0,
            color: "".to_string(),
            inset: false,
        };
        let result = manager.apply_box_shadow(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_box_shadow() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BoxShadowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({
                "boxShadowXOffset": 5.0,
                "boxShadowYOffset": 5.0,
                "boxShadowBlur": 10.0,
                "boxShadowSpread": 0.0,
                "boxShadowColor": "#ff0000",
                "boxShadowInset": false
            })),
            marks: None,
        };
        
        assert!(manager.has_box_shadow(&node));
        let result = manager.remove_box_shadow(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_box_shadow(&node));
    }

    #[test]
    fn test_get_box_shadow() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BoxShadowManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({
                "boxShadowXOffset": 3.0,
                "boxShadowYOffset": 3.0,
                "boxShadowBlur": 6.0,
                "boxShadowSpread": 2.0,
                "boxShadowColor": "#00ff00",
                "boxShadowInset": true
            })),
            marks: None,
        };
        
        let shadow = manager.get_box_shadow(&node);
        assert!(shadow.is_some());
        let attrs = shadow.unwrap();
        assert_eq!(attrs.x_offset, 3.0);
        assert_eq!(attrs.y_offset, 3.0);
        assert_eq!(attrs.blur, 6.0);
        assert_eq!(attrs.spread, 2.0);
        assert_eq!(attrs.color, "#00ff00");
        assert_eq!(attrs.inset, true);
    }

    #[test]
    fn test_get_box_shadow_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BoxShadowManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let shadow = manager.get_box_shadow(&node);
        assert!(shadow.is_none());
    }

    #[test]
    fn test_has_box_shadow() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BoxShadowManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({
                "boxShadowXOffset": 5.0,
                "boxShadowYOffset": 5.0,
                "boxShadowBlur": 10.0,
                "boxShadowSpread": 0.0,
                "boxShadowColor": "#ff0000",
                "boxShadowInset": false
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
        
        assert!(manager.has_box_shadow(&node_with));
        assert!(!manager.has_box_shadow(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BoxShadowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = BoxShadowAttributes::default();
        manager.apply_box_shadow(&mut node, attributes).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BoxShadowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = BoxShadowAttributes::default();
        manager.apply_box_shadow(&mut node, attributes).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BoxShadowManager::new(config_service);
        
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
        let mut manager = BoxShadowManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
