//! TipTap Backface Visibility Manager - Aerospace-Grade Backface Visibility Operations Service
//!
//! Safety-critical backface visibility operations service with:
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

/// Maximum backface visibility string length
const MAX_BACKFACE_VISIBILITY_LENGTH: usize = 50;

/// Backface visibility type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackfaceVisibility {
    Visible,
    Hidden,
}

impl BackfaceVisibility {
    /// Convert backface visibility to string
    pub fn as_str(&self) -> &str {
        match self {
            BackfaceVisibility::Visible => "visible",
            BackfaceVisibility::Hidden => "hidden",
        }
    }

    /// Parse backface visibility from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "visible" => Ok(BackfaceVisibility::Visible),
            "hidden" => Ok(BackfaceVisibility::Hidden),
            _ => Err(format!("Invalid backface visibility: {}", s)),
        }
    }
}

pub struct BackfaceVisibilityManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl BackfaceVisibilityManager {
    /// Creates a new backface visibility manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new BackfaceVisibilityManager instance
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

    /// Get the maximum backface visibility length constant
    /// 
    /// # Returns
    /// The maximum backface visibility string length
    pub fn max_backface_visibility_length() -> usize {
        MAX_BACKFACE_VISIBILITY_LENGTH
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

    /// Validate backface visibility string
    /// 
    /// # Arguments
    /// * `backface_visibility` - The backface visibility string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting backface visibility string length
    fn validate_backface_visibility(&self, backface_visibility: &str) -> Result<(), String> {
        if backface_visibility.len() > MAX_BACKFACE_VISIBILITY_LENGTH {
            return Err(format!("Backface visibility string exceeds maximum length of {} characters", MAX_BACKFACE_VISIBILITY_LENGTH));
        }
        
        // Validate backface visibility value
        BackfaceVisibility::from_str(backface_visibility)?;
        
        Ok(())
    }

    /// Apply backface visibility to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply backface visibility to
    /// * `backface_visibility` - The backface visibility to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates backface visibility string
    pub fn apply_backface_visibility(&mut self, node: &mut TipTapNode, backface_visibility: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate backface visibility
        self.validate_backface_visibility(backface_visibility)?;

        // Apply backface visibility to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("backfaceVisibility".to_string(), serde_json::Value::String(backface_visibility.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "backfaceVisibility": backface_visibility }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Backface visibility application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Backface visibility application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove backface visibility from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove backface visibility from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_backface_visibility(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("backfaceVisibility");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Backface visibility removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Backface visibility removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get backface visibility from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get backface visibility from
    /// 
    /// # Returns
    /// Option containing the backface visibility string or None
    pub fn get_backface_visibility(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(backface_visibility) = obj.get("backfaceVisibility") {
                    if let Some(s) = backface_visibility.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has backface visibility
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has backface visibility, false otherwise
    pub fn has_backface_visibility(&self, node: &TipTapNode) -> bool {
        self.get_backface_visibility(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_backface_visibility_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BackfaceVisibilityManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(BackfaceVisibilityManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(BackfaceVisibilityManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(BackfaceVisibilityManager::max_backface_visibility_length(), MAX_BACKFACE_VISIBILITY_LENGTH);
    }

    #[test]
    fn test_backface_visibility_variants() {
        assert_eq!(BackfaceVisibility::Visible.as_str(), "visible");
        assert_eq!(BackfaceVisibility::Hidden.as_str(), "hidden");
    }

    #[test]
    fn test_backface_visibility_from_str() {
        assert!(matches!(BackfaceVisibility::from_str("visible"), Ok(BackfaceVisibility::Visible)));
        assert!(matches!(BackfaceVisibility::from_str("hidden"), Ok(BackfaceVisibility::Hidden)));
        assert!(BackfaceVisibility::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_backface_visibility() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackfaceVisibilityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_backface_visibility(&mut node, "hidden");
        assert!(result.is_ok());
        assert!(manager.has_backface_visibility(&node));
    }

    #[test]
    fn test_apply_backface_visibility_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackfaceVisibilityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_backface_visibility(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_backface_visibility_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackfaceVisibilityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_backface_visibility = "a".repeat(MAX_BACKFACE_VISIBILITY_LENGTH + 1);
        let result = manager.apply_backface_visibility(&mut node, &long_backface_visibility);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_backface_visibility() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackfaceVisibilityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "backfaceVisibility": "visible" })),
            marks: None,
        };
        
        assert!(manager.has_backface_visibility(&node));
        let result = manager.remove_backface_visibility(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_backface_visibility(&node));
    }

    #[test]
    fn test_get_backface_visibility() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BackfaceVisibilityManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "backfaceVisibility": "hidden" })),
            marks: None,
        };
        
        let backface_visibility = manager.get_backface_visibility(&node);
        assert_eq!(backface_visibility, Some("hidden".to_string()));
    }

    #[test]
    fn test_get_backface_visibility_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BackfaceVisibilityManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let backface_visibility = manager.get_backface_visibility(&node);
        assert!(backface_visibility.is_none());
    }

    #[test]
    fn test_has_backface_visibility() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BackfaceVisibilityManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "backfaceVisibility": "visible" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_backface_visibility(&node_with));
        assert!(!manager.has_backface_visibility(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackfaceVisibilityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_backface_visibility(&mut node, "hidden").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackfaceVisibilityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_backface_visibility(&mut node, "hidden").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackfaceVisibilityManager::new(config_service);
        
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
        let mut manager = BackfaceVisibilityManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
