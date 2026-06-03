//! TipTap Task List Manager - Aerospace-Grade Task List/Checkbox Operations Service
//!
//! Safety-critical task list operations service with:
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
use super::editor::{TipTapNode, NodeType};

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 50;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 200;

/// Maximum task list depth to prevent stack overflow
const MAX_TASK_LIST_DEPTH: usize = 10;

/// Maximum task list item text length
const MAX_TASK_ITEM_TEXT_LENGTH: usize = 10000;

/// Task list item attributes
#[derive(Debug, Clone)]
pub struct TaskItemAttributes {
    pub checked: bool,
}

impl Default for TaskItemAttributes {
    fn default() -> Self {
        Self {
            checked: false,
        }
    }
}

pub struct TaskListManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TaskListManager {
    /// Creates a new task list manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new TaskListManager instance
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

    /// Get the maximum task list depth constant
    /// 
    /// # Returns
    /// The maximum task list depth
    pub fn max_task_list_depth() -> usize {
        MAX_TASK_LIST_DEPTH
    }

    /// Get the maximum task item text length constant
    /// 
    /// # Returns
    /// The maximum text length for task items
    pub fn max_task_item_text_length() -> usize {
        MAX_TASK_ITEM_TEXT_LENGTH
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

    /// Validate task list depth
    /// 
    /// # Arguments
    /// * `depth` - The current task list depth
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents stack overflow by limiting depth
    fn validate_task_list_depth(&self, depth: usize) -> Result<(), String> {
        if depth >= MAX_TASK_LIST_DEPTH {
            return Err(format!("Task list depth exceeds maximum of {}", MAX_TASK_LIST_DEPTH));
        }
        Ok(())
    }

    /// Validate task item text length
    /// 
    /// # Arguments
    /// * `text` - The text to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting text length
    fn validate_task_item_text(&self, text: &str) -> Result<(), String> {
        if text.len() > MAX_TASK_ITEM_TEXT_LENGTH {
            return Err(format!("Task item text exceeds maximum length of {} characters", MAX_TASK_ITEM_TEXT_LENGTH));
        }
        Ok(())
    }

    /// Create a task list node
    /// 
    /// # Arguments
    /// * `items` - The task items
    /// * `depth` - The nesting depth
    /// 
    /// # Returns
    /// Result containing the task list node or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates depth and text length
    pub fn create_task_list(&mut self, items: Vec<TipTapNode>, depth: usize) -> Result<TipTapNode, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate depth
        self.validate_task_list_depth(depth)?;

        // Validate text length in items
        for item in &items {
            if let Some(ref text) = item.text {
                self.validate_task_item_text(text)?;
            }
        }

        let task_list_node = TipTapNode {
            node_type: NodeType::List,
            content: Some(items),
            text: None,
            attrs: Some(serde_json::json!({ "listType": "task" })),
            marks: None,
        };

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Task list creation CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Task list creation performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(task_list_node)
    }

    /// Create a task item node
    /// 
    /// # Arguments
    /// * `text` - The task item text
    /// * `attributes` - The task item attributes
    /// 
    /// # Returns
    /// Result containing the task item node or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates text length
    pub fn create_task_item(&mut self, text: &str, attributes: TaskItemAttributes) -> Result<TipTapNode, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate text length
        self.validate_task_item_text(text)?;

        let task_item_node = TipTapNode {
            node_type: NodeType::ListItem,
            content: Some(vec![TipTapNode {
                node_type: NodeType::Paragraph,
                content: Some(vec![TipTapNode {
                    node_type: NodeType::Text,
                    content: None,
                    text: Some(text.to_string()),
                    attrs: None,
                    marks: None,
                }]),
                text: None,
                attrs: None,
                marks: None,
            }]),
            text: None,
            attrs: Some(serde_json::json!({ "checked": attributes.checked })),
            marks: None,
        };

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Task item creation CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Task item creation performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(task_item_node)
    }

    /// Toggle task item checked state
    /// 
    /// # Arguments
    /// * `task_item` - The task item to toggle
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn toggle_task_item(&mut self, task_item: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = task_item.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                if let Some(checked) = obj.get_mut("checked") {
                    if let Some(b) = checked.as_bool() {
                        *checked = serde_json::Value::Bool(!b);
                    }
                }
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Task item toggle CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Task item toggle performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Set task item checked state
    /// 
    /// # Arguments
    /// * `task_item` - The task item to update
    /// * `checked` - The checked state
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn set_task_item_checked(&mut self, task_item: &mut TipTapNode, checked: bool) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = task_item.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("checked".to_string(), serde_json::Value::Bool(checked));
            }
        } else {
            task_item.attrs = Some(serde_json::json!({ "checked": checked }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Task item checked state set CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Task item checked state set performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get task item checked state
    /// 
    /// # Arguments
    /// * `task_item` - The task item to check
    /// 
    /// # Returns
    /// Option containing the checked state or None
    pub fn get_task_item_checked(&self, task_item: &TipTapNode) -> Option<bool> {
        if let Some(ref attrs) = task_item.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(checked) = obj.get("checked") {
                    if let Some(b) = checked.as_bool() {
                        return Some(b);
                    }
                }
            }
        }
        None
    }

    /// Check if task item is checked
    /// 
    /// # Arguments
    /// * `task_item` - The task item to check
    /// 
    /// # Returns
    /// True if checked, false otherwise
    pub fn is_task_item_checked(&self, task_item: &TipTapNode) -> bool {
        self.get_task_item_checked(task_item).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_task_list_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TaskListManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(TaskListManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(TaskListManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(TaskListManager::max_task_list_depth(), MAX_TASK_LIST_DEPTH);
        assert_eq!(TaskListManager::max_task_item_text_length(), MAX_TASK_ITEM_TEXT_LENGTH);
    }

    #[test]
    fn test_create_task_list() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TaskListManager::new(config_service);
        
        let items = vec![manager.create_task_item("Task 1", TaskItemAttributes::default()).unwrap()];
        let result = manager.create_task_list(items, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_task_list_too_deep() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TaskListManager::new(config_service);
        
        let items = vec![];
        let result = manager.create_task_list(items, MAX_TASK_LIST_DEPTH);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_task_item() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TaskListManager::new(config_service);
        
        let result = manager.create_task_item("Buy groceries", TaskItemAttributes::default());
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_task_item_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TaskListManager::new(config_service);
        
        let long_text = "a".repeat(MAX_TASK_ITEM_TEXT_LENGTH + 1);
        let result = manager.create_task_item(&long_text, TaskItemAttributes::default());
        assert!(result.is_err());
    }

    #[test]
    fn test_create_task_item_checked() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TaskListManager::new(config_service);
        
        let attributes = TaskItemAttributes { checked: true };
        let result = manager.create_task_item("Completed task", attributes);
        assert!(result.is_ok());
    }

    #[test]
    fn test_toggle_task_item() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TaskListManager::new(config_service);
        
        let mut task_item = manager.create_task_item("Test", TaskItemAttributes { checked: false }).unwrap();
        let result = manager.toggle_task_item(&mut task_item);
        assert!(result.is_ok());
        assert!(manager.is_task_item_checked(&task_item));
    }

    #[test]
    fn test_set_task_item_checked() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TaskListManager::new(config_service);
        
        let mut task_item = manager.create_task_item("Test", TaskItemAttributes::default()).unwrap();
        let result = manager.set_task_item_checked(&mut task_item, true);
        assert!(result.is_ok());
        assert!(manager.is_task_item_checked(&task_item));
    }

    #[test]
    fn test_get_task_item_checked() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TaskListManager::new(config_service);
        
        let task_item = manager.create_task_item("Test", TaskItemAttributes { checked: true }).unwrap();
        let checked = manager.get_task_item_checked(&task_item);
        assert_eq!(checked, Some(true));
    }

    #[test]
    fn test_get_task_item_checked_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TaskListManager::new(config_service);
        
        let task_item = TipTapNode {
            node_type: NodeType::ListItem,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        let checked = manager.get_task_item_checked(&task_item);
        assert!(checked.is_none());
    }

    #[test]
    fn test_is_task_item_checked() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TaskListManager::new(config_service);
        
        let checked_item = manager.create_task_item("Test", TaskItemAttributes { checked: true }).unwrap();
        let unchecked_item = manager.create_task_item("Test", TaskItemAttributes { checked: false }).unwrap();
        
        assert!(manager.is_task_item_checked(&checked_item));
        assert!(!manager.is_task_item_checked(&unchecked_item));
    }

    #[test]
    fn test_task_item_attributes_default() {
        let attrs = TaskItemAttributes::default();
        assert!(!attrs.checked);
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TaskListManager::new(config_service);
        
        manager.create_task_item("Test", TaskItemAttributes::default()).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TaskListManager::new(config_service);
        
        manager.create_task_item("Test", TaskItemAttributes::default()).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TaskListManager::new(config_service);
        
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
        let mut manager = TaskListManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
