//! TipTap Text Transform Manager - Aerospace-Grade Text Transform Operations Service
//!
//! Safety-critical text transform operations service with:
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

/// Maximum transform string length
const MAX_TRANSFORM_LENGTH: usize = 50;

/// Text transform type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextTransform {
    None,
    Uppercase,
    Lowercase,
    Capitalize,
}

impl TextTransform {
    /// Convert text transform to string
    pub fn as_str(&self) -> &str {
        match self {
            TextTransform::None => "none",
            TextTransform::Uppercase => "uppercase",
            TextTransform::Lowercase => "lowercase",
            TextTransform::Capitalize => "capitalize",
        }
    }

    /// Parse text transform from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" => Ok(TextTransform::None),
            "uppercase" => Ok(TextTransform::Uppercase),
            "lowercase" => Ok(TextTransform::Lowercase),
            "capitalize" => Ok(TextTransform::Capitalize),
            _ => Err(format!("Invalid text transform: {}", s)),
        }
    }
}

pub struct TextTransformManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextTransformManager {
    /// Creates a new text transform manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new TextTransformManager instance
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

    /// Get the maximum transform length constant
    /// 
    /// # Returns
    /// The maximum transform string length
    pub fn max_transform_length() -> usize {
        MAX_TRANSFORM_LENGTH
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

    /// Validate transform string
    /// 
    /// # Arguments
    /// * `transform` - The transform string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting transform string length
    fn validate_transform(&self, transform: &str) -> Result<(), String> {
        if transform.len() > MAX_TRANSFORM_LENGTH {
            return Err(format!("Transform string exceeds maximum length of {} characters", MAX_TRANSFORM_LENGTH));
        }
        
        // Validate transform value
        TextTransform::from_str(transform)?;
        
        Ok(())
    }

    /// Apply text transform to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply transform to
    /// * `transform` - The transform to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates transform string
    pub fn apply_transform(&mut self, node: &mut TipTapNode, transform: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate transform
        self.validate_transform(transform)?;

        // Apply transform to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textTransform".to_string(), serde_json::Value::String(transform.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textTransform": transform }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text transform application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text transform application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove text transform from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove transform from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_transform(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textTransform");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text transform removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text transform removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get text transform from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get transform from
    /// 
    /// # Returns
    /// Option containing the transform string or None
    pub fn get_transform(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(transform) = obj.get("textTransform") {
                    if let Some(s) = transform.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has text transform
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has text transform, false otherwise
    pub fn has_transform(&self, node: &TipTapNode) -> bool {
        self.get_transform(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_transform_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextTransformManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(TextTransformManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(TextTransformManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(TextTransformManager::max_transform_length(), MAX_TRANSFORM_LENGTH);
    }

    #[test]
    fn test_text_transform_variants() {
        assert_eq!(TextTransform::Uppercase.as_str(), "uppercase");
        assert_eq!(TextTransform::Lowercase.as_str(), "lowercase");
        assert_eq!(TextTransform::Capitalize.as_str(), "capitalize");
    }

    #[test]
    fn test_text_transform_from_str() {
        assert!(matches!(TextTransform::from_str("uppercase"), Ok(TextTransform::Uppercase)));
        assert!(matches!(TextTransform::from_str("lowercase"), Ok(TextTransform::Lowercase)));
        assert!(matches!(TextTransform::from_str("capitalize"), Ok(TextTransform::Capitalize)));
        assert!(TextTransform::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_transform() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextTransformManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_transform(&mut node, "uppercase");
        assert!(result.is_ok());
        assert!(manager.has_transform(&node));
    }

    #[test]
    fn test_apply_transform_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextTransformManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_transform(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_transform_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextTransformManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_transform = "a".repeat(MAX_TRANSFORM_LENGTH + 1);
        let result = manager.apply_transform(&mut node, &long_transform);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_transform() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextTransformManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textTransform": "uppercase" })),
            marks: None,
        };
        
        assert!(manager.has_transform(&node));
        let result = manager.remove_transform(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_transform(&node));
    }

    #[test]
    fn test_get_transform() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextTransformManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textTransform": "lowercase" })),
            marks: None,
        };
        
        let transform = manager.get_transform(&node);
        assert_eq!(transform, Some("lowercase".to_string()));
    }

    #[test]
    fn test_get_transform_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextTransformManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let transform = manager.get_transform(&node);
        assert!(transform.is_none());
    }

    #[test]
    fn test_has_transform() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextTransformManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textTransform": "capitalize" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_transform(&node_with));
        assert!(!manager.has_transform(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextTransformManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_transform(&mut node, "uppercase").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextTransformManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_transform(&mut node, "uppercase").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextTransformManager::new(config_service);
        
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
        let mut manager = TextTransformManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
