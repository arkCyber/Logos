//! TipTap Text Indent Manager - Aerospace-Grade Text Indent Operations Service
//!
//! Safety-critical text indent operations service with:
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

/// Minimum indent value (in pixels)
const MIN_INDENT: i64 = 0;

/// Maximum indent value (in pixels)
const MAX_INDENT: i64 = 1000;

/// Maximum indent depth to prevent stack overflow
const MAX_INDENT_DEPTH: usize = 10;

pub struct TextIndentManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextIndentManager {
    /// Creates a new text indent manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new TextIndentManager instance
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

    /// Get the minimum indent constant
    /// 
    /// # Returns
    /// The minimum indent value in pixels
    pub fn min_indent() -> i64 {
        MIN_INDENT
    }

    /// Get the maximum indent constant
    /// 
    /// # Returns
    /// The maximum indent value in pixels
    pub fn max_indent() -> i64 {
        MAX_INDENT
    }

    /// Get the maximum indent depth constant
    /// 
    /// # Returns
    /// The maximum indent depth
    pub fn max_indent_depth() -> usize {
        MAX_INDENT_DEPTH
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

    /// Validate indent value
    /// 
    /// # Arguments
    /// * `indent` - The indent value to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Ensures indent is within valid range to prevent rendering issues
    fn validate_indent(&self, indent: i64) -> Result<(), String> {
        if indent < MIN_INDENT {
            return Err(format!("Indent must be at least {} pixels", MIN_INDENT));
        }
        if indent > MAX_INDENT {
            return Err(format!("Indent cannot exceed {} pixels", MAX_INDENT));
        }
        Ok(())
    }

    /// Validate indent depth
    /// 
    /// # Arguments
    /// * `depth` - The current indent depth
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents stack overflow by limiting depth
    fn validate_indent_depth(&self, depth: usize) -> Result<(), String> {
        if depth >= MAX_INDENT_DEPTH {
            return Err(format!("Indent depth exceeds maximum of {}", MAX_INDENT_DEPTH));
        }
        Ok(())
    }

    /// Apply indent to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply indent to
    /// * `indent` - The indent value in pixels
    /// * `depth` - The nesting depth
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates indent and depth
    pub fn apply_indent(&mut self, node: &mut TipTapNode, indent: i64, depth: usize) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate indent and depth
        self.validate_indent(indent)?;
        self.validate_indent_depth(depth)?;

        // Apply indent to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textIndent".to_string(), serde_json::Value::Number(indent.into()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textIndent": indent }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text indent application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text indent application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove indent from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove indent from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_indent(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textIndent");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text indent removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text indent removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get indent from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get indent from
    /// 
    /// # Returns
    /// Option containing the indent value or None
    pub fn get_indent(&self, node: &TipTapNode) -> Option<i64> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(indent) = obj.get("textIndent") {
                    if let Some(n) = indent.as_i64() {
                        return Some(n);
                    }
                }
            }
        }
        None
    }

    /// Check if node has indent
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has indent, false otherwise
    pub fn has_indent(&self, node: &TipTapNode) -> bool {
        self.get_indent(node).is_some()
    }

    /// Increase indent by a step
    /// 
    /// # Arguments
    /// * `node` - The node to increase indent for
    /// * `step` - The step size in pixels
    /// * `depth` - The nesting depth
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn increase_indent(&mut self, node: &mut TipTapNode, step: i64, depth: usize) -> Result<(), String> {
        let current_indent = self.get_indent(node).unwrap_or(0);
        let new_indent = current_indent.saturating_add(step);
        self.apply_indent(node, new_indent, depth)
    }

    /// Decrease indent by a step
    /// 
    /// # Arguments
    /// * `node` - The node to decrease indent for
    /// * `step` - The step size in pixels
    /// * `depth` - The nesting depth
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn decrease_indent(&mut self, node: &mut TipTapNode, step: i64, depth: usize) -> Result<(), String> {
        let current_indent = self.get_indent(node).unwrap_or(0);
        let new_indent = current_indent.saturating_sub(step);
        self.apply_indent(node, new_indent, depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_indent_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextIndentManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(TextIndentManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(TextIndentManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_min_max_constants() {
        assert_eq!(TextIndentManager::min_indent(), MIN_INDENT);
        assert_eq!(TextIndentManager::max_indent(), MAX_INDENT);
        assert_eq!(TextIndentManager::max_indent_depth(), MAX_INDENT_DEPTH);
    }

    #[test]
    fn test_apply_indent() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextIndentManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_indent(&mut node, 40, 0);
        assert!(result.is_ok());
        assert!(manager.has_indent(&node));
    }

    #[test]
    fn test_apply_indent_negative() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextIndentManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_indent(&mut node, -1, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_indent_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextIndentManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_indent(&mut node, MAX_INDENT + 1, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_indent_too_deep() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextIndentManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_indent(&mut node, 40, MAX_INDENT_DEPTH);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_indent() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextIndentManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textIndent": 40 })),
            marks: None,
        };
        
        assert!(manager.has_indent(&node));
        let result = manager.remove_indent(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_indent(&node));
    }

    #[test]
    fn test_get_indent() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextIndentManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_indent(&mut node, 80, 0).unwrap();
        let indent = manager.get_indent(&node);
        assert_eq!(indent, Some(80));
    }

    #[test]
    fn test_get_indent_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextIndentManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let indent = manager.get_indent(&node);
        assert!(indent.is_none());
    }

    #[test]
    fn test_has_indent() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextIndentManager::new(config_service);
        
        let mut node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_indent(&mut node_with, 40, 0).unwrap();
        
        assert!(manager.has_indent(&node_with));
        assert!(!manager.has_indent(&node_without));
    }

    #[test]
    fn test_increase_indent() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextIndentManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textIndent": 40 })),
            marks: None,
        };
        
        let result = manager.increase_indent(&mut node, 20, 0);
        assert!(result.is_ok());
        assert_eq!(manager.get_indent(&node), Some(60));
    }

    #[test]
    fn test_decrease_indent() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextIndentManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textIndent": 60 })),
            marks: None,
        };
        
        let result = manager.decrease_indent(&mut node, 20, 0);
        assert!(result.is_ok());
        assert_eq!(manager.get_indent(&node), Some(40));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextIndentManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_indent(&mut node, 40, 0).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextIndentManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_indent(&mut node, 40, 0).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextIndentManager::new(config_service);
        
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
        let mut manager = TextIndentManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
