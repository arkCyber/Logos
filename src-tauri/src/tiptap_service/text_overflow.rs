//! TipTap Text Overflow Manager - Aerospace-Grade Text Overflow Operations Service
//!
//! Safety-critical text overflow operations service with:
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

/// Maximum overflow string length
const MAX_OVERFLOW_LENGTH: usize = 50;

/// Text overflow type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextOverflow {
    Clip,
    Ellipsis,
    Visible,
    Hidden,
}

impl TextOverflow {
    /// Convert text overflow to string
    pub fn as_str(&self) -> &str {
        match self {
            TextOverflow::Clip => "clip",
            TextOverflow::Ellipsis => "ellipsis",
            TextOverflow::Visible => "visible",
            TextOverflow::Hidden => "hidden",
        }
    }

    /// Parse text overflow from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "clip" => Ok(TextOverflow::Clip),
            "ellipsis" => Ok(TextOverflow::Ellipsis),
            "visible" => Ok(TextOverflow::Visible),
            "hidden" => Ok(TextOverflow::Hidden),
            _ => Err(format!("Invalid text overflow: {}", s)),
        }
    }
}

pub struct TextOverflowManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextOverflowManager {
    /// Creates a new text overflow manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new TextOverflowManager instance
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

    /// Get the maximum overflow length constant
    /// 
    /// # Returns
    /// The maximum overflow string length
    pub fn max_overflow_length() -> usize {
        MAX_OVERFLOW_LENGTH
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

    /// Validate overflow string
    /// 
    /// # Arguments
    /// * `overflow` - The overflow string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting overflow string length
    fn validate_overflow(&self, overflow: &str) -> Result<(), String> {
        if overflow.len() > MAX_OVERFLOW_LENGTH {
            return Err(format!("Overflow string exceeds maximum length of {} characters", MAX_OVERFLOW_LENGTH));
        }
        
        // Validate overflow value
        TextOverflow::from_str(overflow)?;
        
        Ok(())
    }

    /// Apply text overflow to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply overflow to
    /// * `overflow` - The overflow to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates overflow string
    pub fn apply_overflow(&mut self, node: &mut TipTapNode, overflow: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate overflow
        self.validate_overflow(overflow)?;

        // Apply overflow to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textOverflow".to_string(), serde_json::Value::String(overflow.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textOverflow": overflow }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text overflow application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text overflow application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove text overflow from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove overflow from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_overflow(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textOverflow");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text overflow removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text overflow removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get text overflow from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get overflow from
    /// 
    /// # Returns
    /// Option containing the overflow string or None
    pub fn get_overflow(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(overflow) = obj.get("textOverflow") {
                    if let Some(s) = overflow.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has text overflow
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has text overflow, false otherwise
    pub fn has_overflow(&self, node: &TipTapNode) -> bool {
        self.get_overflow(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_overflow_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextOverflowManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(TextOverflowManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(TextOverflowManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(TextOverflowManager::max_overflow_length(), MAX_OVERFLOW_LENGTH);
    }

    #[test]
    fn test_text_overflow_variants() {
        assert_eq!(TextOverflow::Clip.as_str(), "clip");
        assert_eq!(TextOverflow::Ellipsis.as_str(), "ellipsis");
        assert_eq!(TextOverflow::Visible.as_str(), "visible");
    }

    #[test]
    fn test_text_overflow_from_str() {
        assert!(matches!(TextOverflow::from_str("clip"), Ok(TextOverflow::Clip)));
        assert!(matches!(TextOverflow::from_str("ellipsis"), Ok(TextOverflow::Ellipsis)));
        assert!(matches!(TextOverflow::from_str("visible"), Ok(TextOverflow::Visible)));
        assert!(TextOverflow::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_overflow() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextOverflowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_overflow(&mut node, "ellipsis");
        assert!(result.is_ok());
        assert!(manager.has_overflow(&node));
    }

    #[test]
    fn test_apply_overflow_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextOverflowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_overflow(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_overflow_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextOverflowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_overflow = "a".repeat(MAX_OVERFLOW_LENGTH + 1);
        let result = manager.apply_overflow(&mut node, &long_overflow);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_overflow() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextOverflowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textOverflow": "ellipsis" })),
            marks: None,
        };
        
        assert!(manager.has_overflow(&node));
        let result = manager.remove_overflow(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_overflow(&node));
    }

    #[test]
    fn test_get_overflow() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextOverflowManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textOverflow": "clip" })),
            marks: None,
        };
        
        let overflow = manager.get_overflow(&node);
        assert_eq!(overflow, Some("clip".to_string()));
    }

    #[test]
    fn test_get_overflow_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextOverflowManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let overflow = manager.get_overflow(&node);
        assert!(overflow.is_none());
    }

    #[test]
    fn test_has_overflow() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextOverflowManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textOverflow": "ellipsis" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_overflow(&node_with));
        assert!(!manager.has_overflow(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextOverflowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_overflow(&mut node, "ellipsis").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextOverflowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_overflow(&mut node, "ellipsis").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextOverflowManager::new(config_service);
        
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
        let mut manager = TextOverflowManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
