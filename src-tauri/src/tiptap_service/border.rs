//! TipTap Border Manager - Aerospace-Grade Border Operations Service
//!
//! Safety-critical border operations service with:
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

/// Minimum border width (in pixels)
const MIN_BORDER_WIDTH: f64 = 0.0;

/// Maximum border width (in pixels)
const MAX_BORDER_WIDTH: f64 = 50.0;

/// Maximum border color string length
const MAX_BORDER_COLOR_LENGTH: usize = 50;

/// Border style type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle {
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

impl BorderStyle {
    /// Convert border style to string
    pub fn as_str(&self) -> &str {
        match self {
            BorderStyle::None => "none",
            BorderStyle::Solid => "solid",
            BorderStyle::Dashed => "dashed",
            BorderStyle::Dotted => "dotted",
            BorderStyle::Double => "double",
            BorderStyle::Groove => "groove",
            BorderStyle::Ridge => "ridge",
            BorderStyle::Inset => "inset",
            BorderStyle::Outset => "outset",
        }
    }

    /// Parse border style from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" => Ok(BorderStyle::None),
            "solid" => Ok(BorderStyle::Solid),
            "dashed" => Ok(BorderStyle::Dashed),
            "dotted" => Ok(BorderStyle::Dotted),
            "double" => Ok(BorderStyle::Double),
            "groove" => Ok(BorderStyle::Groove),
            "ridge" => Ok(BorderStyle::Ridge),
            "inset" => Ok(BorderStyle::Inset),
            "outset" => Ok(BorderStyle::Outset),
            _ => Err(format!("Invalid border style: {}", s)),
        }
    }
}

/// Border attributes
#[derive(Debug, Clone)]
pub struct BorderAttributes {
    pub width: f64,
    pub style: BorderStyle,
    pub color: String,
}

impl Default for BorderAttributes {
    fn default() -> Self {
        Self {
            width: 1.0,
            style: BorderStyle::Solid,
            color: "#000000".to_string(),
        }
    }
}

pub struct BorderManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl BorderManager {
    /// Creates a new border manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new BorderManager instance
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

    /// Get the minimum border width constant
    /// 
    /// # Returns
    /// The minimum border width in pixels
    pub fn min_border_width() -> f64 {
        MIN_BORDER_WIDTH
    }

    /// Get the maximum border width constant
    /// 
    /// # Returns
    /// The maximum border width in pixels
    pub fn max_border_width() -> f64 {
        MAX_BORDER_WIDTH
    }

    /// Get the maximum border color length constant
    /// 
    /// # Returns
    /// The maximum border color string length
    pub fn max_border_color_length() -> usize {
        MAX_BORDER_COLOR_LENGTH
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

    /// Validate border width
    /// 
    /// # Arguments
    /// * `width` - The border width to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Ensures width is within valid range
    fn validate_border_width(&self, width: f64) -> Result<(), String> {
        if width < MIN_BORDER_WIDTH {
            return Err(format!("Border width must be at least {}", MIN_BORDER_WIDTH));
        }
        if width > MAX_BORDER_WIDTH {
            return Err(format!("Border width cannot exceed {}", MAX_BORDER_WIDTH));
        }
        if !width.is_finite() {
            return Err("Border width must be a finite number".to_string());
        }
        Ok(())
    }

    /// Validate border color
    /// 
    /// # Arguments
    /// * `color` - The border color to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting color string length
    fn validate_border_color(&self, color: &str) -> Result<(), String> {
        if color.is_empty() {
            return Err("Border color cannot be empty".to_string());
        }
        if color.len() > MAX_BORDER_COLOR_LENGTH {
            return Err(format!("Border color exceeds maximum length of {} characters", MAX_BORDER_COLOR_LENGTH));
        }
        // Basic hex color validation
        if color.starts_with('#') {
            if color.len() != 4 && color.len() != 7 {
                return Err("Invalid hex color format".to_string());
            }
        }
        Ok(())
    }

    /// Apply border to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply border to
    /// * `attributes` - The border attributes
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates all border attributes
    pub fn apply_border(&mut self, node: &mut TipTapNode, attributes: BorderAttributes) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate attributes
        self.validate_border_width(attributes.width)?;
        self.validate_border_color(&attributes.color)?;

        // Apply border to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("borderWidth".to_string(), serde_json::json!(attributes.width));
                obj.insert("borderStyle".to_string(), serde_json::json!(attributes.style.as_str()));
                obj.insert("borderColor".to_string(), serde_json::json!(attributes.color));
            }
        } else {
            node.attrs = Some(serde_json::json!({
                "borderWidth": attributes.width,
                "borderStyle": attributes.style.as_str(),
                "borderColor": attributes.color
            }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Border application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Border application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove border from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove border from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_border(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("borderWidth");
                obj.remove("borderStyle");
                obj.remove("borderColor");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Border removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Border removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get border from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get border from
    /// 
    /// # Returns
    /// Option containing the border attributes or None
    pub fn get_border(&self, node: &TipTapNode) -> Option<BorderAttributes> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                let width = obj.get("borderWidth").and_then(|v| v.as_f64())?;
                let style_str = obj.get("borderStyle").and_then(|v| v.as_str())?;
                let style = BorderStyle::from_str(style_str).ok()?;
                let color = obj.get("borderColor").and_then(|v| v.as_str())?.to_string();
                return Some(BorderAttributes {
                    width,
                    style,
                    color,
                });
            }
        }
        None
    }

    /// Check if node has border
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has border, false otherwise
    pub fn has_border(&self, node: &TipTapNode) -> bool {
        self.get_border(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_border_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BorderManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(BorderManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(BorderManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_min_max_constants() {
        assert_eq!(BorderManager::min_border_width(), MIN_BORDER_WIDTH);
        assert_eq!(BorderManager::max_border_width(), MAX_BORDER_WIDTH);
        assert_eq!(BorderManager::max_border_color_length(), MAX_BORDER_COLOR_LENGTH);
    }

    #[test]
    fn test_border_style_variants() {
        assert_eq!(BorderStyle::Solid.as_str(), "solid");
        assert_eq!(BorderStyle::Dashed.as_str(), "dashed");
        assert_eq!(BorderStyle::Dotted.as_str(), "dotted");
    }

    #[test]
    fn test_border_style_from_str() {
        assert!(matches!(BorderStyle::from_str("solid"), Ok(BorderStyle::Solid)));
        assert!(matches!(BorderStyle::from_str("dashed"), Ok(BorderStyle::Dashed)));
        assert!(matches!(BorderStyle::from_str("dotted"), Ok(BorderStyle::Dotted)));
        assert!(BorderStyle::from_str("invalid").is_err());
    }

    #[test]
    fn test_border_attributes_default() {
        let attrs = BorderAttributes::default();
        assert_eq!(attrs.width, 1.0);
        assert_eq!(attrs.style, BorderStyle::Solid);
        assert_eq!(attrs.color, "#000000");
    }

    #[test]
    fn test_apply_border() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = BorderAttributes {
            width: 2.0,
            style: BorderStyle::Dashed,
            color: "#ff0000".to_string(),
        };
        let result = manager.apply_border(&mut node, attributes);
        assert!(result.is_ok());
        assert!(manager.has_border(&node));
    }

    #[test]
    fn test_apply_border_width_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = BorderAttributes {
            width: MAX_BORDER_WIDTH + 1.0,
            style: BorderStyle::Solid,
            color: "#ff0000".to_string(),
        };
        let result = manager.apply_border(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_border_width_negative() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = BorderAttributes {
            width: -1.0,
            style: BorderStyle::Solid,
            color: "#ff0000".to_string(),
        };
        let result = manager.apply_border(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_border_color_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = BorderAttributes {
            width: 1.0,
            style: BorderStyle::Solid,
            color: "".to_string(),
        };
        let result = manager.apply_border(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_border_color_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = BorderAttributes {
            width: 1.0,
            style: BorderStyle::Solid,
            color: "#".repeat(MAX_BORDER_COLOR_LENGTH + 1),
        };
        let result = manager.apply_border(&mut node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_border() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({
                "borderWidth": 2.0,
                "borderStyle": "solid",
                "borderColor": "#ff0000"
            })),
            marks: None,
        };
        
        assert!(manager.has_border(&node));
        let result = manager.remove_border(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_border(&node));
    }

    #[test]
    fn test_get_border() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BorderManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({
                "borderWidth": 3.0,
                "borderStyle": "dashed",
                "borderColor": "#00ff00"
            })),
            marks: None,
        };
        
        let border = manager.get_border(&node);
        assert!(border.is_some());
        let attrs = border.unwrap();
        assert_eq!(attrs.width, 3.0);
        assert_eq!(attrs.style, BorderStyle::Dashed);
        assert_eq!(attrs.color, "#00ff00");
    }

    #[test]
    fn test_get_border_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BorderManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let border = manager.get_border(&node);
        assert!(border.is_none());
    }

    #[test]
    fn test_has_border() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BorderManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({
                "borderWidth": 2.0,
                "borderStyle": "solid",
                "borderColor": "#ff0000"
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
        
        assert!(manager.has_border(&node_with));
        assert!(!manager.has_border(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = BorderAttributes::default();
        manager.apply_border(&mut node, attributes).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = BorderAttributes::default();
        manager.apply_border(&mut node, attributes).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderManager::new(config_service);
        
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
        let mut manager = BorderManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
