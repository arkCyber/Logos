//! TipTap Perspective Manager - Aerospace-Grade Perspective Operations Service
//!
//! Safety-critical perspective operations service with:
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

/// Maximum perspective value (in pixels)
const MAX_PERSPECTIVE: f64 = 1000.0;

/// Minimum perspective value (in pixels)
const MIN_PERSPECTIVE: f64 = 0.0;

pub struct PerspectiveManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl PerspectiveManager {
    /// Creates a new perspective manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new PerspectiveManager instance
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

    /// Get the maximum perspective constant
    /// 
    /// # Returns
    /// The maximum perspective in pixels
    pub fn max_perspective() -> f64 {
        MAX_PERSPECTIVE
    }

    /// Get the minimum perspective constant
    /// 
    /// # Returns
    /// The minimum perspective in pixels
    pub fn min_perspective() -> f64 {
        MIN_PERSPECTIVE
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

    /// Validate perspective
    /// 
    /// # Arguments
    /// * `perspective` - The perspective to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Ensures perspective is within valid range to prevent rendering issues
    fn validate_perspective(&self, perspective: f64) -> Result<(), String> {
        if perspective < MIN_PERSPECTIVE {
            return Err(format!("Perspective must be at least {}", MIN_PERSPECTIVE));
        }
        if perspective > MAX_PERSPECTIVE {
            return Err(format!("Perspective cannot exceed {}", MAX_PERSPECTIVE));
        }
        if !perspective.is_finite() {
            return Err("Perspective must be a finite number".to_string());
        }
        Ok(())
    }

    /// Apply perspective to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply perspective to
    /// * `perspective` - The perspective value
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates perspective
    pub fn apply_perspective(&mut self, node: &mut TipTapNode, perspective: f64) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate perspective
        self.validate_perspective(perspective)?;

        // Apply perspective to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("perspective".to_string(), serde_json::json!(perspective));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "perspective": perspective }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Perspective application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Perspective application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove perspective from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove perspective from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_perspective(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("perspective");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Perspective removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Perspective removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get perspective from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get perspective from
    /// 
    /// # Returns
    /// Option containing the perspective or None
    pub fn get_perspective(&self, node: &TipTapNode) -> Option<f64> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(perspective) = obj.get("perspective") {
                    if let Some(n) = perspective.as_f64() {
                        return Some(n);
                    }
                }
            }
        }
        None
    }

    /// Check if node has perspective
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has perspective, false otherwise
    pub fn has_perspective(&self, node: &TipTapNode) -> bool {
        self.get_perspective(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_perspective_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PerspectiveManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(PerspectiveManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(PerspectiveManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_min_max_constants() {
        assert_eq!(PerspectiveManager::min_perspective(), MIN_PERSPECTIVE);
        assert_eq!(PerspectiveManager::max_perspective(), MAX_PERSPECTIVE);
    }

    #[test]
    fn test_apply_perspective() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PerspectiveManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_perspective(&mut node, 500.0);
        assert!(result.is_ok());
        assert!(manager.has_perspective(&node));
    }

    #[test]
    fn test_apply_perspective_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PerspectiveManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_perspective(&mut node, MAX_PERSPECTIVE + 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_perspective_too_small() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PerspectiveManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_perspective(&mut node, MIN_PERSPECTIVE - 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_perspective_infinite() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PerspectiveManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_perspective(&mut node, f64::INFINITY);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_perspective() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PerspectiveManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "perspective": 300.0 })),
            marks: None,
        };
        
        assert!(manager.has_perspective(&node));
        let result = manager.remove_perspective(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_perspective(&node));
    }

    #[test]
    fn test_get_perspective() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PerspectiveManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "perspective": 200.0 })),
            marks: None,
        };
        
        let perspective = manager.get_perspective(&node);
        assert_eq!(perspective, Some(200.0));
    }

    #[test]
    fn test_get_perspective_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PerspectiveManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let perspective = manager.get_perspective(&node);
        assert!(perspective.is_none());
    }

    #[test]
    fn test_has_perspective() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PerspectiveManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "perspective": 100.0 })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_perspective(&node_with));
        assert!(!manager.has_perspective(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PerspectiveManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_perspective(&mut node, 500.0).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PerspectiveManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_perspective(&mut node, 500.0).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PerspectiveManager::new(config_service);
        
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
        let mut manager = PerspectiveManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
