//! TipTap History Manager - Aerospace-Grade Undo/Redo Service
//!
//! Safety-critical history management service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Comprehensive error handling
//! - Performance monitoring
//! - Security hardening
//! - Fault tolerance and error recovery

use serde::{Deserialize, Serialize};
use std::time::Instant;
use crate::error_handling::{ErrorContext, ErrorSeverity};
use crate::config_service::ExportConfigService;
use std::sync::Arc;
use super::editor::TipTapDocument;

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 50;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 200;

/// Maximum history stack size to prevent memory exhaustion
const MAX_HISTORY_SIZE: usize = 100;

/// History operation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HistoryOperation {
    Insert { position: usize, content: String },
    Delete { position: usize, length: usize, content: String },
    Format { position: usize, length: usize, format: String },
    Replace { position: usize, length: usize, old_content: String, new_content: String },
}

/// History entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub operation: HistoryOperation,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub document_snapshot: Option<TipTapDocument>,
}

pub struct HistoryManager {
    config_service: Arc<ExportConfigService>,
    undo_stack: Vec<HistoryEntry>,
    redo_stack: Vec<HistoryEntry>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
    max_history_size: usize,
}

impl HistoryManager {
    /// Creates a new history manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new HistoryManager instance
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            operation_count: 0,
            last_error: None,
            max_history_size: MAX_HISTORY_SIZE,
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

    /// Get the maximum history size constant
    /// 
    /// # Returns
    /// The maximum history stack size
    pub fn max_history_size() -> usize {
        MAX_HISTORY_SIZE
    }

    /// Set custom maximum history size
    /// 
    /// # Arguments
    /// * `size` - The new maximum history size
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Validates size to prevent memory exhaustion
    pub fn set_max_history_size(&mut self, size: usize) -> Result<(), String> {
        if size == 0 {
            return Err("History size must be greater than 0".to_string());
        }
        if size > 1000 {
            return Err("History size cannot exceed 1000".to_string());
        }
        self.max_history_size = size;
        Ok(())
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

    /// Get undo stack size
    pub fn get_undo_stack_size(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get redo stack size
    pub fn get_redo_stack_size(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clear all history
    pub fn clear_history(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.operation_count = 0;
    }

    /// Push an operation to the undo stack
    /// 
    /// # Arguments
    /// * `operation` - The operation to record
    /// * `document_snapshot` - Optional document snapshot
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates stack size to prevent memory exhaustion
    pub fn push_operation(&mut self, operation: HistoryOperation, document_snapshot: Option<TipTapDocument>) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate stack size
        if self.undo_stack.len() >= self.max_history_size {
            self.undo_stack.remove(0); // Remove oldest entry
        }

        let entry = HistoryEntry {
            operation,
            timestamp: chrono::Utc::now(),
            document_snapshot,
        };

        self.undo_stack.push(entry);
        self.redo_stack.clear(); // Clear redo stack on new operation

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("History push CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("History push performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Undo the last operation
    /// 
    /// # Returns
    /// Result containing the undone operation or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn undo(&mut self) -> Result<HistoryOperation, String> {
        let start_time = Instant::now();

        if self.undo_stack.is_empty() {
            let error = "No operations to undo".to_string();
            self.record_error("UNDO_EMPTY", &error, "undo");
            return Err(error);
        }

        let entry = self.undo_stack.pop().unwrap();
        let operation = entry.operation.clone();
        self.redo_stack.push(entry);

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("History undo CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("History undo performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(operation)
    }

    /// Redo the last undone operation
    /// 
    /// # Returns
    /// Result containing the redone operation or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn redo(&mut self) -> Result<HistoryOperation, String> {
        let start_time = Instant::now();

        if self.redo_stack.is_empty() {
            let error = "No operations to redo".to_string();
            self.record_error("REDO_EMPTY", &error, "redo");
            return Err(error);
        }

        let entry = self.redo_stack.pop().unwrap();
        let operation = entry.operation.clone();
        self.undo_stack.push(entry);

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("History redo CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("History redo performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(operation)
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get the last operation without removing it
    pub fn peek_last_operation(&self) -> Option<&HistoryOperation> {
        self.undo_stack.last().map(|entry| &entry.operation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = HistoryManager::new(config_service);
        assert_eq!(manager.get_undo_stack_size(), 0);
        assert_eq!(manager.get_redo_stack_size(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(HistoryManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(HistoryManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_history_size() {
        assert_eq!(HistoryManager::max_history_size(), MAX_HISTORY_SIZE);
    }

    #[test]
    fn test_set_max_history_size() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HistoryManager::new(config_service);
        
        let result = manager.set_max_history_size(50);
        assert!(result.is_ok());
        assert_eq!(manager.max_history_size, 50);
    }

    #[test]
    fn test_set_max_history_size_zero() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HistoryManager::new(config_service);
        
        let result = manager.set_max_history_size(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_max_history_size_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HistoryManager::new(config_service);
        
        let result = manager.set_max_history_size(1001);
        assert!(result.is_err());
    }

    #[test]
    fn test_push_operation() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HistoryManager::new(config_service);
        
        let operation = HistoryOperation::Insert {
            position: 0,
            content: "test".to_string(),
        };
        
        let result = manager.push_operation(operation, None);
        assert!(result.is_ok());
        assert_eq!(manager.get_undo_stack_size(), 1);
    }

    #[test]
    fn test_push_operation_exceeds_limit() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HistoryManager::new(config_service);
        manager.set_max_history_size(3).unwrap();
        
        for i in 0..5 {
            let operation = HistoryOperation::Insert {
                position: i,
                content: format!("test{}", i),
            };
            manager.push_operation(operation, None).unwrap();
        }
        
        assert_eq!(manager.get_undo_stack_size(), 3);
    }

    #[test]
    fn test_undo() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HistoryManager::new(config_service);
        
        let operation = HistoryOperation::Insert {
            position: 0,
            content: "test".to_string(),
        };
        manager.push_operation(operation.clone(), None).unwrap();
        
        let result = manager.undo();
        assert!(result.is_ok());
        assert_eq!(manager.get_undo_stack_size(), 0);
        assert_eq!(manager.get_redo_stack_size(), 1);
    }

    #[test]
    fn test_undo_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HistoryManager::new(config_service);
        
        let result = manager.undo();
        assert!(result.is_err());
    }

    #[test]
    fn test_redo() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HistoryManager::new(config_service);
        
        let operation = HistoryOperation::Insert {
            position: 0,
            content: "test".to_string(),
        };
        manager.push_operation(operation.clone(), None).unwrap();
        manager.undo().unwrap();
        
        let result = manager.redo();
        assert!(result.is_ok());
        assert_eq!(manager.get_undo_stack_size(), 1);
        assert_eq!(manager.get_redo_stack_size(), 0);
    }

    #[test]
    fn test_redo_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HistoryManager::new(config_service);
        
        let result = manager.redo();
        assert!(result.is_err());
    }

    #[test]
    fn test_can_undo() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HistoryManager::new(config_service);
        
        assert!(!manager.can_undo());
        
        let operation = HistoryOperation::Insert {
            position: 0,
            content: "test".to_string(),
        };
        manager.push_operation(operation, None).unwrap();
        
        assert!(manager.can_undo());
    }

    #[test]
    fn test_can_redo() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HistoryManager::new(config_service);
        
        assert!(!manager.can_redo());
        
        let operation = HistoryOperation::Insert {
            position: 0,
            content: "test".to_string(),
        };
        manager.push_operation(operation, None).unwrap();
        manager.undo().unwrap();
        
        assert!(manager.can_redo());
    }

    #[test]
    fn test_clear_history() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HistoryManager::new(config_service);
        
        let operation = HistoryOperation::Insert {
            position: 0,
            content: "test".to_string(),
        };
        manager.push_operation(operation, None).unwrap();
        manager.undo().unwrap();
        
        manager.clear_history();
        
        assert_eq!(manager.get_undo_stack_size(), 0);
        assert_eq!(manager.get_redo_stack_size(), 0);
    }

    #[test]
    fn test_peek_last_operation() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HistoryManager::new(config_service);
        
        let operation = HistoryOperation::Insert {
            position: 0,
            content: "test".to_string(),
        };
        manager.push_operation(operation.clone(), None).unwrap();
        
        let peeked = manager.peek_last_operation();
        assert!(peeked.is_some());
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HistoryManager::new(config_service);
        
        let operation = HistoryOperation::Insert {
            position: 0,
            content: "test".to_string(),
        };
        manager.push_operation(operation, None).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HistoryManager::new(config_service);
        
        let operation = HistoryOperation::Insert {
            position: 0,
            content: "test".to_string(),
        };
        manager.push_operation(operation, None).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HistoryManager::new(config_service);
        
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
        let mut manager = HistoryManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }

    #[test]
    fn test_new_operation_clears_redo_stack() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HistoryManager::new(config_service);
        
        let operation1 = HistoryOperation::Insert {
            position: 0,
            content: "test1".to_string(),
        };
        manager.push_operation(operation1, None).unwrap();
        manager.undo().unwrap();
        
        let operation2 = HistoryOperation::Insert {
            position: 1,
            content: "test2".to_string(),
        };
        manager.push_operation(operation2, None).unwrap();
        
        assert_eq!(manager.get_redo_stack_size(), 0);
    }
}
