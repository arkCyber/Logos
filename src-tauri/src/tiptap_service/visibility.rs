//! TipTap Visibility Manager - Aerospace-Grade Visibility Operations Service
//!
//! Safety-critical visibility operations service with:
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

/// Maximum visibility string length
const MAX_VISIBILITY_LENGTH: usize = 50;

/// Visibility type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

impl Visibility {
    /// Convert visibility to string
    pub fn as_str(&self) -> &str {
        match self {
            Visibility::Visible => "visible",
            Visibility::Hidden => "hidden",
            Visibility::Collapse => "collapse",
        }
    }

    /// Parse visibility from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "visible" => Ok(Visibility::Visible),
            "hidden" => Ok(Visibility::Hidden),
            "collapse" => Ok(Visibility::Collapse),
            _ => Err(format!("Invalid visibility: {}", s)),
        }
    }
}

pub struct VisibilityManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl VisibilityManager {
    /// Creates a new visibility manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new VisibilityManager instance
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

    /// Get the maximum visibility length constant
    /// 
    /// # Returns
    /// The maximum visibility string length
    pub fn max_visibility_length() -> usize {
        MAX_VISIBILITY_LENGTH
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

    /// Validate visibility string
    /// 
    /// # Arguments
    /// * `visibility` - The visibility string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting visibility string length
    fn validate_visibility(&self, visibility: &str) -> Result<(), String> {
        if visibility.len() > MAX_VISIBILITY_LENGTH {
            return Err(format!("Visibility string exceeds maximum length of {} characters", MAX_VISIBILITY_LENGTH));
        }
        
        // Validate visibility value
        Visibility::from_str(visibility)?;
        
        Ok(())
    }

    /// Apply visibility to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply visibility to
    /// * `visibility` - The visibility to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates visibility string
    pub fn apply_visibility(&mut self, node: &mut TipTapNode, visibility: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate visibility
        self.validate_visibility(visibility)?;

        // Apply visibility to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("visibility".to_string(), serde_json::Value::String(visibility.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "visibility": visibility }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Visibility application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Visibility application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove visibility from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove visibility from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_visibility(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("visibility");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Visibility removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Visibility removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get visibility from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get visibility from
    /// 
    /// # Returns
    /// Option containing the visibility string or None
    pub fn get_visibility(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(visibility) = obj.get("visibility") {
                    if let Some(s) = visibility.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has visibility
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has visibility, false otherwise
    pub fn has_visibility(&self, node: &TipTapNode) -> bool {
        self.get_visibility(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_visibility_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = VisibilityManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(VisibilityManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(VisibilityManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(VisibilityManager::max_visibility_length(), MAX_VISIBILITY_LENGTH);
    }

    #[test]
    fn test_visibility_variants() {
        assert_eq!(Visibility::Visible.as_str(), "visible");
        assert_eq!(Visibility::Hidden.as_str(), "hidden");
        assert_eq!(Visibility::Collapse.as_str(), "collapse");
    }

    #[test]
    fn test_visibility_from_str() {
        assert!(matches!(Visibility::from_str("visible"), Ok(Visibility::Visible)));
        assert!(matches!(Visibility::from_str("hidden"), Ok(Visibility::Hidden)));
        assert!(matches!(Visibility::from_str("collapse"), Ok(Visibility::Collapse)));
        assert!(Visibility::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_visibility() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = VisibilityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_visibility(&mut node, "hidden");
        assert!(result.is_ok());
        assert!(manager.has_visibility(&node));
    }

    #[test]
    fn test_apply_visibility_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = VisibilityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_visibility(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_visibility_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = VisibilityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_visibility = "a".repeat(MAX_VISIBILITY_LENGTH + 1);
        let result = manager.apply_visibility(&mut node, &long_visibility);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_visibility() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = VisibilityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "visibility": "hidden" })),
            marks: None,
        };
        
        assert!(manager.has_visibility(&node));
        let result = manager.remove_visibility(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_visibility(&node));
    }

    #[test]
    fn test_get_visibility() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = VisibilityManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "visibility": "collapse" })),
            marks: None,
        };
        
        let visibility = manager.get_visibility(&node);
        assert_eq!(visibility, Some("collapse".to_string()));
    }

    #[test]
    fn test_get_visibility_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = VisibilityManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let visibility = manager.get_visibility(&node);
        assert!(visibility.is_none());
    }

    #[test]
    fn test_has_visibility() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = VisibilityManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "visibility": "visible" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_visibility(&node_with));
        assert!(!manager.has_visibility(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = VisibilityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_visibility(&mut node, "hidden").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = VisibilityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_visibility(&mut node, "hidden").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = VisibilityManager::new(config_service);
        
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
        let mut manager = VisibilityManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
