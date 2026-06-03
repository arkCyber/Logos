//! TipTap Border Radius Manager - Aerospace-Grade Border Radius Operations Service
//!
//! Safety-critical border radius operations service with:
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

/// Maximum border radius value (in pixels)
const MAX_BORDER_RADIUS: f64 = 1000.0;

/// Minimum border radius value (in pixels)
const MIN_BORDER_RADIUS: f64 = 0.0;

/// Maximum border radius string length
const MAX_BORDER_RADIUS_LENGTH: usize = 100;

pub struct BorderRadiusManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl BorderRadiusManager {
    /// Creates a new border radius manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new BorderRadiusManager instance
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

    /// Get the maximum border radius constant
    /// 
    /// # Returns
    /// The maximum border radius in pixels
    pub fn max_border_radius() -> f64 {
        MAX_BORDER_RADIUS
    }

    /// Get the minimum border radius constant
    /// 
    /// # Returns
    /// The minimum border radius in pixels
    pub fn min_border_radius() -> f64 {
        MIN_BORDER_RADIUS
    }

    /// Get the maximum border radius length constant
    /// 
    /// # Returns
    /// The maximum border radius string length
    pub fn max_border_radius_length() -> usize {
        MAX_BORDER_RADIUS_LENGTH
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

    /// Validate border radius string
    /// 
    /// # Arguments
    /// * `border_radius` - The border radius string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting border radius string length
    fn validate_border_radius(&self, border_radius: &str) -> Result<(), String> {
        if border_radius.is_empty() {
            return Err("Border radius cannot be empty".to_string());
        }
        if border_radius.len() > MAX_BORDER_RADIUS_LENGTH {
            return Err(format!("Border radius string exceeds maximum length of {} characters", MAX_BORDER_RADIUS_LENGTH));
        }
        // Validate numeric value if it's a pixel value
        if border_radius.ends_with("px") {
            let value_str = border_radius.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_BORDER_RADIUS || value > MAX_BORDER_RADIUS {
                    return Err(format!("Border radius must be between {} and {} pixels", MIN_BORDER_RADIUS, MAX_BORDER_RADIUS));
                }
                if !value.is_finite() {
                    return Err("Border radius must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    /// Apply border radius to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply border radius to
    /// * `border_radius` - The border radius to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates border radius string
    pub fn apply_border_radius(&mut self, node: &mut TipTapNode, border_radius: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate border radius
        self.validate_border_radius(border_radius)?;

        // Apply border radius to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("borderRadius".to_string(), serde_json::Value::String(border_radius.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "borderRadius": border_radius }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Border radius application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Border radius application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove border radius from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove border radius from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_border_radius(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("borderRadius");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Border radius removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Border radius removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get border radius from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get border radius from
    /// 
    /// # Returns
    /// Option containing the border radius string or None
    pub fn get_border_radius(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(border_radius) = obj.get("borderRadius") {
                    if let Some(s) = border_radius.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has border radius
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has border radius, false otherwise
    pub fn has_border_radius(&self, node: &TipTapNode) -> bool {
        self.get_border_radius(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_border_radius_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BorderRadiusManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(BorderRadiusManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(BorderRadiusManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_min_max_constants() {
        assert_eq!(BorderRadiusManager::min_border_radius(), MIN_BORDER_RADIUS);
        assert_eq!(BorderRadiusManager::max_border_radius(), MAX_BORDER_RADIUS);
        assert_eq!(BorderRadiusManager::max_border_radius_length(), MAX_BORDER_RADIUS_LENGTH);
    }

    #[test]
    fn test_apply_border_radius() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderRadiusManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_border_radius(&mut node, "10px");
        assert!(result.is_ok());
        assert!(manager.has_border_radius(&node));
    }

    #[test]
    fn test_apply_border_radius_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderRadiusManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_border_radius(&mut node, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_border_radius_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderRadiusManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_border_radius = "a".repeat(MAX_BORDER_RADIUS_LENGTH + 1);
        let result = manager.apply_border_radius(&mut node, &long_border_radius);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_border_radius_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderRadiusManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_border_radius(&mut node, "2000px");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_border_radius() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderRadiusManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "borderRadius": "5px" })),
            marks: None,
        };
        
        assert!(manager.has_border_radius(&node));
        let result = manager.remove_border_radius(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_border_radius(&node));
    }

    #[test]
    fn test_get_border_radius() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BorderRadiusManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "borderRadius": "15px" })),
            marks: None,
        };
        
        let border_radius = manager.get_border_radius(&node);
        assert_eq!(border_radius, Some("15px".to_string()));
    }

    #[test]
    fn test_get_border_radius_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BorderRadiusManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let border_radius = manager.get_border_radius(&node);
        assert!(border_radius.is_none());
    }

    #[test]
    fn test_has_border_radius() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BorderRadiusManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "borderRadius": "20px" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_border_radius(&node_with));
        assert!(!manager.has_border_radius(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderRadiusManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_border_radius(&mut node, "10px").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderRadiusManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_border_radius(&mut node, "10px").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BorderRadiusManager::new(config_service);
        
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
        let mut manager = BorderRadiusManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
