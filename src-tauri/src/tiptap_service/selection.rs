//! TipTap Selection Manager - Aerospace-Grade Selection Operations Service
//!
//! Safety-critical selection operations service with:
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

/// Maximum selection range to prevent performance issues
const MAX_SELECTION_RANGE: usize = 100000;

/// Selection range
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionRange {
    pub from: usize,
    pub to: usize,
}

impl SelectionRange {
    /// Create a new selection range
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }

    /// Get the length of the selection
    pub fn length(&self) -> usize {
        if self.to > self.from {
            self.to - self.from
        } else {
            self.from - self.to
        }
    }

    /// Check if the selection is valid
    pub fn is_valid(&self) -> bool {
        self.from != self.to
    }
}

pub struct SelectionManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl SelectionManager {
    /// Creates a new selection manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new SelectionManager instance
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

    /// Get the maximum selection range constant
    /// 
    /// # Returns
    /// The maximum selection range
    pub fn max_selection_range() -> usize {
        MAX_SELECTION_RANGE
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

    /// Validate selection range
    /// 
    /// # Arguments
    /// * `range` - The selection range to validate
    /// * `text_length` - The length of the text being selected
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents performance issues by limiting selection range
    fn validate_selection_range(&self, range: &SelectionRange, text_length: usize) -> Result<(), String> {
        if range.from > text_length {
            return Err(format!("Selection start position {} exceeds text length {}", range.from, text_length));
        }
        if range.to > text_length {
            return Err(format!("Selection end position {} exceeds text length {}", range.to, text_length));
        }
        if range.length() > MAX_SELECTION_RANGE {
            return Err(format!("Selection range {} exceeds maximum of {}", range.length(), MAX_SELECTION_RANGE));
        }
        Ok(())
    }

    /// Set selection on a node
    /// 
    /// # Arguments
    /// * `node` - The node to set selection on
    /// * `range` - The selection range
    /// * `text_length` - The length of the text
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates selection range
    pub fn set_selection(&mut self, node: &mut TipTapNode, range: SelectionRange, text_length: usize) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate selection range
        self.validate_selection_range(&range, text_length)?;

        // Set selection on node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("selectionFrom".to_string(), serde_json::json!(range.from));
                obj.insert("selectionTo".to_string(), serde_json::json!(range.to));
            }
        } else {
            node.attrs = Some(serde_json::json!({
                "selectionFrom": range.from,
                "selectionTo": range.to
            }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Selection setting CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Selection setting performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Clear selection from a node
    /// 
    /// # Arguments
    /// * `node` - The node to clear selection from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn clear_selection(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("selectionFrom");
                obj.remove("selectionTo");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Selection clearing CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Selection clearing performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get selection from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get selection from
    /// 
    /// # Returns
    /// Option containing the selection range or None
    pub fn get_selection(&self, node: &TipTapNode) -> Option<SelectionRange> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                let from = obj.get("selectionFrom").and_then(|v| v.as_u64())? as usize;
                let to = obj.get("selectionTo").and_then(|v| v.as_u64())? as usize;
                return Some(SelectionRange::new(from, to));
            }
        }
        None
    }

    /// Check if node has selection
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has selection, false otherwise
    pub fn has_selection(&self, node: &TipTapNode) -> bool {
        self.get_selection(node).is_some()
    }

    /// Get selected text from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get selected text from
    /// 
    /// # Returns
    /// Option containing the selected text or None
    pub fn get_selected_text(&self, node: &TipTapNode) -> Option<String> {
        let selection = self.get_selection(node)?;
        let text = node.text.as_ref()?;
        
        let start = selection.from.min(selection.to);
        let end = selection.from.max(selection.to);
        
        if start <= text.len() && end <= text.len() {
            Some(text[start..end].to_string())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_selection_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = SelectionManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(SelectionManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(SelectionManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(SelectionManager::max_selection_range(), MAX_SELECTION_RANGE);
    }

    #[test]
    fn test_selection_range_creation() {
        let range = SelectionRange::new(0, 10);
        assert_eq!(range.from, 0);
        assert_eq!(range.to, 10);
    }

    #[test]
    fn test_selection_range_length() {
        let range = SelectionRange::new(0, 10);
        assert_eq!(range.length(), 10);
        
        let range_reverse = SelectionRange::new(10, 0);
        assert_eq!(range_reverse.length(), 10);
    }

    #[test]
    fn test_selection_range_is_valid() {
        let valid_range = SelectionRange::new(0, 10);
        assert!(valid_range.is_valid());
        
        let invalid_range = SelectionRange::new(5, 5);
        assert!(!invalid_range.is_valid());
    }

    #[test]
    fn test_set_selection() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SelectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Hello World".to_string()),
            attrs: None,
            marks: None,
        };
        
        let range = SelectionRange::new(0, 5);
        let result = manager.set_selection(&mut node, range, 11);
        assert!(result.is_ok());
        assert!(manager.has_selection(&node));
    }

    #[test]
    fn test_set_selection_invalid_range() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SelectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Hello".to_string()),
            attrs: None,
            marks: None,
        };
        
        let range = SelectionRange::new(0, 100);
        let result = manager.set_selection(&mut node, range, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_selection_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SelectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Hello".to_string()),
            attrs: None,
            marks: None,
        };
        
        let range = SelectionRange::new(0, MAX_SELECTION_RANGE + 1);
        let result = manager.set_selection(&mut node, range, MAX_SELECTION_RANGE + 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_clear_selection() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SelectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Hello".to_string()),
            attrs: Some(serde_json::json!({
                "selectionFrom": 0,
                "selectionTo": 5
            })),
            marks: None,
        };
        
        assert!(manager.has_selection(&node));
        let result = manager.clear_selection(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_selection(&node));
    }

    #[test]
    fn test_get_selection() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = SelectionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Hello".to_string()),
            attrs: Some(serde_json::json!({
                "selectionFrom": 0,
                "selectionTo": 5
            })),
            marks: None,
        };
        
        let selection = manager.get_selection(&node);
        assert!(selection.is_some());
        let range = selection.unwrap();
        assert_eq!(range.from, 0);
        assert_eq!(range.to, 5);
    }

    #[test]
    fn test_get_selection_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = SelectionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Hello".to_string()),
            attrs: None,
            marks: None,
        };
        
        let selection = manager.get_selection(&node);
        assert!(selection.is_none());
    }

    #[test]
    fn test_has_selection() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = SelectionManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Hello".to_string()),
            attrs: Some(serde_json::json!({
                "selectionFrom": 0,
                "selectionTo": 5
            })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Hello".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_selection(&node_with));
        assert!(!manager.has_selection(&node_without));
    }

    #[test]
    fn test_get_selected_text() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = SelectionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Hello World".to_string()),
            attrs: Some(serde_json::json!({
                "selectionFrom": 0,
                "selectionTo": 5
            })),
            marks: None,
        };
        
        let selected_text = manager.get_selected_text(&node);
        assert_eq!(selected_text, Some("Hello".to_string()));
    }

    #[test]
    fn test_get_selected_text_reverse() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = SelectionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Hello World".to_string()),
            attrs: Some(serde_json::json!({
                "selectionFrom": 5,
                "selectionTo": 0
            })),
            marks: None,
        };
        
        let selected_text = manager.get_selected_text(&node);
        assert_eq!(selected_text, Some("Hello".to_string()));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SelectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Hello".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.set_selection(&mut node, SelectionRange::new(0, 5), 5).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SelectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Hello".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.set_selection(&mut node, SelectionRange::new(0, 5), 5).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SelectionManager::new(config_service);
        
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
        let mut manager = SelectionManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
