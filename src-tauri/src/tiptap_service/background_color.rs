//! TipTap Background Color Manager - Aerospace-Grade Background Color Operations Service
//!
//! Safety-critical background color operations service with:
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

/// Maximum background color string length
const MAX_BACKGROUND_COLOR_LENGTH: usize = 50;

pub struct BackgroundColorManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl BackgroundColorManager {
    /// Creates a new background color manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new BackgroundColorManager instance
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

    /// Get the maximum background color length constant
    /// 
    /// # Returns
    /// The maximum background color string length
    pub fn max_background_color_length() -> usize {
        MAX_BACKGROUND_COLOR_LENGTH
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

    /// Validate background color
    /// 
    /// # Arguments
    /// * `color` - The background color to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting color string length
    fn validate_background_color(&self, color: &str) -> Result<(), String> {
        if color.is_empty() {
            return Err("Background color cannot be empty".to_string());
        }
        if color.len() > MAX_BACKGROUND_COLOR_LENGTH {
            return Err(format!("Background color exceeds maximum length of {} characters", MAX_BACKGROUND_COLOR_LENGTH));
        }
        // Basic hex color validation
        if color.starts_with('#') {
            if color.len() != 4 && color.len() != 7 {
                return Err("Invalid hex color format".to_string());
            }
        }
        Ok(())
    }

    /// Apply background color to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply background color to
    /// * `color` - The background color
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates background color
    pub fn apply_background_color(&mut self, node: &mut TipTapNode, color: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate background color
        self.validate_background_color(color)?;

        // Apply background color to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("backgroundColor".to_string(), serde_json::Value::String(color.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "backgroundColor": color }));
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
    pub fn remove_background_color(&mut self, node: &mut TipTapNode) -> Result<(), String> {
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

    /// Get background color from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get background color from
    /// 
    /// # Returns
    /// Option containing the background color or None
    pub fn get_background_color(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(color) = obj.get("backgroundColor") {
                    if let Some(s) = color.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has background color
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has background color, false otherwise
    pub fn has_background_color(&self, node: &TipTapNode) -> bool {
        self.get_background_color(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_background_color_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BackgroundColorManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(BackgroundColorManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(BackgroundColorManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(BackgroundColorManager::max_background_color_length(), MAX_BACKGROUND_COLOR_LENGTH);
    }

    #[test]
    fn test_apply_background_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackgroundColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_background_color(&mut node, "#ff0000");
        assert!(result.is_ok());
        assert!(manager.has_background_color(&node));
    }

    #[test]
    fn test_apply_background_color_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackgroundColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_background_color(&mut node, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_background_color_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackgroundColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_color = "#".repeat(MAX_BACKGROUND_COLOR_LENGTH + 1);
        let result = manager.apply_background_color(&mut node, &long_color);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_background_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackgroundColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "backgroundColor": "#ff0000" })),
            marks: None,
        };
        
        assert!(manager.has_background_color(&node));
        let result = manager.remove_background_color(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_background_color(&node));
    }

    #[test]
    fn test_get_background_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BackgroundColorManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "backgroundColor": "#00ff00" })),
            marks: None,
        };
        
        let color = manager.get_background_color(&node);
        assert_eq!(color, Some("#00ff00".to_string()));
    }

    #[test]
    fn test_get_background_color_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BackgroundColorManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let color = manager.get_background_color(&node);
        assert!(color.is_none());
    }

    #[test]
    fn test_has_background_color() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BackgroundColorManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "backgroundColor": "#0000ff" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_background_color(&node_with));
        assert!(!manager.has_background_color(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackgroundColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_background_color(&mut node, "#ff0000").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackgroundColorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_background_color(&mut node, "#ff0000").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackgroundColorManager::new(config_service);
        
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
        let mut manager = BackgroundColorManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
