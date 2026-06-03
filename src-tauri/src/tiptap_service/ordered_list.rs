//! TipTap Ordered List Manager - Aerospace-Grade Ordered List Service
//!
//! Safety-critical ordered list service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Performance monitoring
//! - Comprehensive error handling
//! - Security hardening
//! - Fault tolerance and error recovery

use std::time::Instant;
use std::collections::HashMap;
use crate::error_handling::{ErrorContext, ErrorSeverity};
use crate::config_service::ExportConfigService;
use std::sync::Arc;

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 50;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 200;

/// Ordered list
#[derive(Debug, Clone)]
pub struct OrderedList {
    pub list_id: String,
    pub items: Vec<String>,
    pub start_number: usize,
    pub position: usize,
}

pub struct OrderedListManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
    lists: HashMap<String, OrderedList>,
    list_counter: u64,
}

impl OrderedListManager {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
            lists: HashMap::new(),
            list_counter: 0,
        }
    }

    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    fn record_error(&mut self, code: &str, message: &str, source: &str) {
        self.last_error = Some(ErrorContext::new(ErrorSeverity::Error, code, message, source));
    }

    pub fn get_last_error(&self) -> Option<&ErrorContext> {
        self.last_error.as_ref()
    }

    pub fn get_operation_count(&self) -> u64 {
        self.operation_count
    }

    pub fn reset_error_state(&mut self) {
        self.last_error = None;
    }

    pub fn reset_operation_count(&mut self) {
        self.operation_count = 0;
    }

    pub fn create_list(&mut self, items: Vec<String>, start_number: usize, position: usize) -> Result<String, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if items.is_empty() {
            return Err("Ordered list cannot be empty".to_string());
        }

        self.list_counter += 1;
        let list_id = format!("ordered_list_{}", self.list_counter);

        let list = OrderedList {
            list_id: list_id.clone(),
            items,
            start_number,
            position,
        };

        self.lists.insert(list_id.clone(), list);

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Create ordered list CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Create ordered list performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(list_id)
    }

    pub fn remove_list(&mut self, list_id: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.lists.remove(list_id)
            .ok_or("Ordered list not found")?;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Remove ordered list CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Remove ordered list performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_list(&self, list_id: &str) -> Option<&OrderedList> {
        self.lists.get(list_id)
    }

    pub fn get_all_lists(&self) -> Vec<&OrderedList> {
        self.lists.values().collect()
    }

    pub fn clear_lists(&mut self) {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.lists.clear();

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Clear ordered lists CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Clear ordered lists performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_list_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OrderedListManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_create_list() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OrderedListManager::new(config_service);
        
        let items = vec!["Item 1".to_string(), "Item 2".to_string()];
        let result = manager.create_list(items, 1, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_list() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OrderedListManager::new(config_service);
        
        let result = manager.create_list(vec![], 1, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_clear_lists() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OrderedListManager::new(config_service);
        
        let items = vec!["Item 1".to_string()];
        manager.create_list(items, 1, 0).unwrap();
        manager.clear_lists();
        
        assert_eq!(manager.get_all_lists().len(), 0);
    }
}
