//! TipTap Outline Manager - Aerospace-Grade Outline Operations Service
//!
//! Safety-critical outline operations service with:
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

/// Maximum outline string length
const MAX_OUTLINE_LENGTH: usize = 200;

pub struct OutlineManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl OutlineManager {
    /// Creates a new outline manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new OutlineManager instance
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

    /// Get the maximum outline length constant
    /// 
    /// # Returns
    /// The maximum outline string length
    pub fn max_outline_length() -> usize {
        MAX_OUTLINE_LENGTH
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

    /// Validate outline string
    /// 
    /// # Arguments
    /// * `outline` - The outline string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting outline string length
    fn validate_outline(&self, outline: &str) -> Result<(), String> {
        if outline.is_empty() {
            return Err("Outline cannot be empty".to_string());
        }
        if outline.len() > MAX_OUTLINE_LENGTH {
            return Err(format!("Outline string exceeds maximum length of {} characters", MAX_OUTLINE_LENGTH));
        }
        // Check for unmatched parentheses first
        if outline.contains('(') && !outline.contains(')') {
            return Err("Invalid outline: unmatched parentheses".to_string());
        }
        // Basic validation for common outline values
        let valid_patterns = ["none", "solid", "dotted", "dashed", "double", "groove", "ridge", "inset", "outset"];
        if !valid_patterns.iter().any(|pattern| outline.contains(pattern)) {
            // Allow custom values but validate basic structure
            if outline.contains('(') && !outline.contains(')') {
                return Err("Invalid outline: unmatched parentheses".to_string());
            }
        }
        Ok(())
    }

    /// Apply outline to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply outline to
    /// * `outline` - The outline to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates outline string
    pub fn apply_outline(&mut self, node: &mut TipTapNode, outline: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate outline
        self.validate_outline(outline)?;

        // Apply outline to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("outline".to_string(), serde_json::Value::String(outline.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "outline": outline }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Outline application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Outline application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove outline from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove outline from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_outline(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("outline");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Outline removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Outline removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get outline from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get outline from
    /// 
    /// # Returns
    /// Option containing the outline string or None
    pub fn get_outline(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(outline) = obj.get("outline") {
                    if let Some(s) = outline.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has outline
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has outline, false otherwise
    pub fn has_outline(&self, node: &TipTapNode) -> bool {
        self.get_outline(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_outline_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OutlineManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(OutlineManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(OutlineManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(OutlineManager::max_outline_length(), MAX_OUTLINE_LENGTH);
    }

    #[test]
    fn test_apply_outline() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OutlineManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_outline(&mut node, "2px solid red");
        assert!(result.is_ok());
        assert!(manager.has_outline(&node));
    }

    #[test]
    fn test_apply_outline_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OutlineManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_outline(&mut node, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_outline_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OutlineManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_outline = "a".repeat(MAX_OUTLINE_LENGTH + 1);
        let result = manager.apply_outline(&mut node, &long_outline);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_outline_invalid_parentheses() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OutlineManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_outline(&mut node, "2px solid rgb(255, 0, 0");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_outline() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OutlineManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "outline": "none" })),
            marks: None,
        };
        
        assert!(manager.has_outline(&node));
        let result = manager.remove_outline(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_outline(&node));
    }

    #[test]
    fn test_get_outline() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OutlineManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "outline": "1px dashed blue" })),
            marks: None,
        };
        
        let outline = manager.get_outline(&node);
        assert_eq!(outline, Some("1px dashed blue".to_string()));
    }

    #[test]
    fn test_get_outline_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OutlineManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let outline = manager.get_outline(&node);
        assert!(outline.is_none());
    }

    #[test]
    fn test_has_outline() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OutlineManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "outline": "3px double green" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_outline(&node_with));
        assert!(!manager.has_outline(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OutlineManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_outline(&mut node, "2px solid red").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OutlineManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_outline(&mut node, "2px solid red").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OutlineManager::new(config_service);
        
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
        let mut manager = OutlineManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
