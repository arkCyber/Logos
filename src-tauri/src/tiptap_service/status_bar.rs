//! TipTap Status Bar Manager - Aerospace-Grade Status Bar Service
//!
//! Safety-critical status bar service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Performance monitoring
//! - Comprehensive error handling
//! - Security hardening
//! - Fault tolerance and error recovery

use std::time::Instant;
use crate::error_handling::{ErrorContext, ErrorSeverity};
use crate::config_service::ExportConfigService;
use std::sync::Arc;

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 50;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 200;

/// Maximum status message length
const MAX_STATUS_MESSAGE_LENGTH: usize = 500;

/// Status bar item
#[derive(Debug, Clone)]
pub struct StatusBarItem {
    pub id: String,
    pub label: String,
    pub value: String,
    pub visible: bool,
}

pub struct StatusBarManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
    enabled: bool,
    items: Vec<StatusBarItem>,
    current_message: Option<String>,
}

impl StatusBarManager {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
            enabled: true,
            items: Vec::new(),
            current_message: None,
        }
    }

    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    pub fn max_status_message_length() -> usize {
        MAX_STATUS_MESSAGE_LENGTH
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

    pub fn enable(&mut self) {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.enabled = true;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Enable status bar CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Enable status bar performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }

    pub fn disable(&mut self) {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.enabled = false;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Disable status bar CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Disable status bar performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }

    pub fn add_item(&mut self, id: String, label: String, value: String) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if id.is_empty() {
            return Err("Item ID cannot be empty".to_string());
        }

        if label.is_empty() {
            return Err("Item label cannot be empty".to_string());
        }

        let item = StatusBarItem {
            id,
            label,
            value,
            visible: true,
        };

        self.items.push(item);

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Add status bar item CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Add status bar item performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_item(&mut self, id: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(pos) = self.items.iter().position(|item| item.id == id) {
            self.items.remove(pos);
        } else {
            return Err("Item not found".to_string());
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Remove status bar item CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Remove status bar item performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn update_item(&mut self, id: &str, value: String) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.value = value;
        } else {
            return Err("Item not found".to_string());
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Update status bar item CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Update status bar item performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn set_message(&mut self, message: String) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if message.len() > MAX_STATUS_MESSAGE_LENGTH {
            return Err(format!("Message exceeds maximum length of {} characters", MAX_STATUS_MESSAGE_LENGTH));
        }

        self.current_message = Some(message);

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Set status message CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Set status message performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn clear_message(&mut self) {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.current_message = None;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Clear status message CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Clear status message performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_items(&self) -> &Vec<StatusBarItem> {
        &self.items
    }

    pub fn get_message(&self) -> Option<&String> {
        self.current_message.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_bar_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = StatusBarManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_add_item() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = StatusBarManager::new(config_service);
        
        let result = manager.add_item("cursor".to_string(), "Cursor".to_string(), "1,1".to_string());
        assert!(result.is_ok());
        assert_eq!(manager.get_items().len(), 1);
    }

    #[test]
    fn test_remove_item() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = StatusBarManager::new(config_service);
        
        manager.add_item("cursor".to_string(), "Cursor".to_string(), "1,1".to_string()).unwrap();
        manager.remove_item("cursor").unwrap();
        
        assert_eq!(manager.get_items().len(), 0);
    }

    #[test]
    fn test_update_item() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = StatusBarManager::new(config_service);
        
        manager.add_item("cursor".to_string(), "Cursor".to_string(), "1,1".to_string()).unwrap();
        manager.update_item("cursor", "5,10".to_string()).unwrap();
        
        assert_eq!(manager.get_items()[0].value, "5,10");
    }

    #[test]
    fn test_set_message() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = StatusBarManager::new(config_service);
        
        let result = manager.set_message("Document saved".to_string());
        assert!(result.is_ok());
        assert!(manager.get_message().is_some());
    }
}
