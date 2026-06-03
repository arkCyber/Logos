//! TipTap Text Color Manager - Aerospace-Grade Text Color Operations Service
//!
//! Safety-critical text color operations service with:
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

/// Maximum color value length
const MAX_COLOR_LENGTH: usize = 50;

/// Text color attributes
#[derive(Debug, Clone)]
pub struct TextColorAttributes {
    pub color: Option<String>,
    pub background: Option<String>,
}

impl Default for TextColorAttributes {
    fn default() -> Self {
        Self {
            color: None,
            background: None,
        }
    }
}

pub struct TextColorManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextColorManager {
    /// Creates a new text color manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new TextColorManager instance
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

    /// Get the maximum color length constant
    /// 
    /// # Returns
    /// The maximum color value length
    pub fn max_color_length() -> usize {
        MAX_COLOR_LENGTH
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

    /// Validate color value
    /// 
    /// # Arguments
    /// * `color` - The color value to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting color string length
    fn validate_color(&self, color: &str) -> Result<(), String> {
        if color.len() > MAX_COLOR_LENGTH {
            return Err(format!("Color value exceeds maximum length of {} characters", MAX_COLOR_LENGTH));
        }
        
        // Basic validation for hex color format
        if color.starts_with('#') {
            if color.len() != 4 && color.len() != 7 {
                return Err("Invalid hex color format. Expected #RGB or #RRGGBB".to_string());
            }
        }
        
        Ok(())
    }

    /// Apply text color to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply color to
    /// * `color` - The color to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates color value
    pub fn apply_color(&mut self, node: &mut TipTapNode, color: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate color
        self.validate_color(color)?;

        // Apply color to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textColor".to_string(), serde_json::Value::String(color.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textColor": color }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text color application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text color application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Apply background color (highlight) to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply background color to
    /// * `background` - The background color to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates color value
    pub fn apply_background(&mut self, node: &mut TipTapNode, background: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate color
        self.validate_color(background)?;

        // Apply background color to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("backgroundColor".to_string(), serde_json::Value::String(background.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "backgroundColor": background }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Background color application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Background color application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Apply both text color and background color
    /// 
    /// # Arguments
    /// * `node` - The node to apply colors to
    /// * `attributes` - The color attributes
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn apply_colors(&mut self, node: &mut TipTapNode, attributes: TextColorAttributes) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate colors if provided
        if let Some(ref color) = attributes.color {
            self.validate_color(color)?;
        }
        if let Some(ref background) = attributes.background {
            self.validate_color(background)?;
        }

        // Apply colors to node attributes
        let mut attrs_obj = if let Some(ref attrs) = node.attrs {
            attrs.as_object().cloned().unwrap_or_default()
        } else {
            serde_json::Map::new()
        };

        if let Some(color) = attributes.color {
            attrs_obj.insert("textColor".to_string(), serde_json::Value::String(color));
        }
        if let Some(background) = attributes.background {
            attrs_obj.insert("backgroundColor".to_string(), serde_json::Value::String(background));
        }

        node.attrs = Some(serde_json::Value::Object(attrs_obj));

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Colors application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Colors application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove text color from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove color from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_color(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textColor");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text color removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text color removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove background color from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove background color from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_background(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("backgroundColor");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Background color removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Background color removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get text color from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get color from
    /// 
    /// # Returns
    /// Option containing the color string or None
    pub fn get_color(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(color) = obj.get("textColor") {
                    if let Some(s) = color.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Get background color from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get background color from
    /// 
    /// # Returns
    /// Option containing the background color string or None
    pub fn get_background(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(bg) = obj.get("backgroundColor") {
                    if let Some(s) = bg.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Get all color attributes from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get color attributes from
    /// 
    /// # Returns
    /// TextColorAttributes containing the color and background
    pub fn get_color_attributes(&self, node: &TipTapNode) -> TextColorAttributes {
        TextColorAttributes {
            color: self.get_color(node),
            background: self.get_background(node),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_color_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextColorManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(TextColorManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(TextColorManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(TextColorManager::max_color_length(), MAX_COLOR_LENGTH);
    }

    #[test]
    fn test_apply_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_color(&mut node, "#ff0000");
        assert!(result.is_ok());
        assert_eq!(manager.get_color(&node), Some("#ff0000".to_string()));
    }

    #[test]
    fn test_apply_color_invalid_hex() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_color(&mut node, "#ff");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_color_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_color = "a".repeat(MAX_COLOR_LENGTH + 1);
        let result = manager.apply_color(&mut node, &long_color);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_background() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_background(&mut node, "#ffff00");
        assert!(result.is_ok());
        assert_eq!(manager.get_background(&node), Some("#ffff00".to_string()));
    }

    #[test]
    fn test_apply_colors() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = TextColorAttributes {
            color: Some("#ff0000".to_string()),
            background: Some("#ffff00".to_string()),
        };
        let result = manager.apply_colors(&mut node, attributes);
        assert!(result.is_ok());
        assert_eq!(manager.get_color(&node), Some("#ff0000".to_string()));
        assert_eq!(manager.get_background(&node), Some("#ffff00".to_string()));
    }

    #[test]
    fn test_remove_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textColor": "#ff0000" })),
            marks: None,
        };
        
        assert!(manager.get_color(&node).is_some());
        let result = manager.remove_color(&mut node);
        assert!(result.is_ok());
        assert!(manager.get_color(&node).is_none());
    }

    #[test]
    fn test_remove_background() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "backgroundColor": "#ffff00" })),
            marks: None,
        };
        
        assert!(manager.get_background(&node).is_some());
        let result = manager.remove_background(&mut node);
        assert!(result.is_ok());
        assert!(manager.get_background(&node).is_none());
    }

    #[test]
    fn test_get_color_attributes() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextColorManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ 
                "textColor": "#ff0000",
                "backgroundColor": "#ffff00"
            })),
            marks: None,
        };
        
        let attrs = manager.get_color_attributes(&node);
        assert_eq!(attrs.color, Some("#ff0000".to_string()));
        assert_eq!(attrs.background, Some("#ffff00".to_string()));
    }

    #[test]
    fn test_color_attributes_default() {
        let attrs = TextColorAttributes::default();
        assert!(attrs.color.is_none());
        assert!(attrs.background.is_none());
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_color(&mut node, "#ff0000").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_color(&mut node, "#ff0000").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextColorManager::new(config_service);
        
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
        let mut manager = TextColorManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
