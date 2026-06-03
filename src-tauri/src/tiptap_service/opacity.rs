//! TipTap Opacity Manager - Aerospace-Grade Opacity Operations Service
//!
//! Safety-critical opacity operations service with:
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

/// Minimum opacity value
const MIN_OPACITY: f64 = 0.0;

/// Maximum opacity value
const MAX_OPACITY: f64 = 1.0;

pub struct OpacityManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl OpacityManager {
    /// Creates a new opacity manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new OpacityManager instance
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

    /// Get the minimum opacity constant
    /// 
    /// # Returns
    /// The minimum opacity value
    pub fn min_opacity() -> f64 {
        MIN_OPACITY
    }

    /// Get the maximum opacity constant
    /// 
    /// # Returns
    /// The maximum opacity value
    pub fn max_opacity() -> f64 {
        MAX_OPACITY
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

    /// Validate opacity
    /// 
    /// # Arguments
    /// * `opacity` - The opacity to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Ensures opacity is within valid range to prevent rendering issues
    fn validate_opacity(&self, opacity: f64) -> Result<(), String> {
        if opacity < MIN_OPACITY {
            return Err(format!("Opacity must be at least {}", MIN_OPACITY));
        }
        if opacity > MAX_OPACITY {
            return Err(format!("Opacity cannot exceed {}", MAX_OPACITY));
        }
        if !opacity.is_finite() {
            return Err("Opacity must be a finite number".to_string());
        }
        Ok(())
    }

    /// Apply opacity to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply opacity to
    /// * `opacity` - The opacity value
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates opacity
    pub fn apply_opacity(&mut self, node: &mut TipTapNode, opacity: f64) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate opacity
        self.validate_opacity(opacity)?;

        // Apply opacity to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("opacity".to_string(), serde_json::json!(opacity));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "opacity": opacity }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Opacity application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Opacity application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove opacity from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove opacity from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_opacity(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("opacity");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Opacity removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Opacity removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get opacity from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get opacity from
    /// 
    /// # Returns
    /// Option containing the opacity or None
    pub fn get_opacity(&self, node: &TipTapNode) -> Option<f64> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(opacity) = obj.get("opacity") {
                    if let Some(n) = opacity.as_f64() {
                        return Some(n);
                    }
                }
            }
        }
        None
    }

    /// Check if node has opacity
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has opacity, false otherwise
    pub fn has_opacity(&self, node: &TipTapNode) -> bool {
        self.get_opacity(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_opacity_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OpacityManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(OpacityManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(OpacityManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_min_max_constants() {
        assert_eq!(OpacityManager::min_opacity(), MIN_OPACITY);
        assert_eq!(OpacityManager::max_opacity(), MAX_OPACITY);
    }

    #[test]
    fn test_apply_opacity() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OpacityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_opacity(&mut node, 0.5);
        assert!(result.is_ok());
        assert!(manager.has_opacity(&node));
    }

    #[test]
    fn test_apply_opacity_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OpacityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_opacity(&mut node, MAX_OPACITY + 0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_opacity_too_small() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OpacityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_opacity(&mut node, MIN_OPACITY - 0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_opacity_infinite() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OpacityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_opacity(&mut node, f64::INFINITY);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_opacity() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OpacityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "opacity": 0.5 })),
            marks: None,
        };
        
        assert!(manager.has_opacity(&node));
        let result = manager.remove_opacity(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_opacity(&node));
    }

    #[test]
    fn test_get_opacity() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OpacityManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "opacity": 0.75 })),
            marks: None,
        };
        
        let opacity = manager.get_opacity(&node);
        assert_eq!(opacity, Some(0.75));
    }

    #[test]
    fn test_get_opacity_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OpacityManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let opacity = manager.get_opacity(&node);
        assert!(opacity.is_none());
    }

    #[test]
    fn test_has_opacity() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OpacityManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "opacity": 0.3 })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_opacity(&node_with));
        assert!(!manager.has_opacity(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OpacityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_opacity(&mut node, 0.5).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OpacityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_opacity(&mut node, 0.5).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OpacityManager::new(config_service);
        
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
        let mut manager = OpacityManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
