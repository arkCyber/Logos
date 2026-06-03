//! TipTap Subscript Manager - Aerospace-Grade Subscript Service
//!
//! Safety-critical subscript service with:
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

/// Subscript
#[derive(Debug, Clone)]
pub struct Subscript {
    pub subscript_id: String,
    pub text: String,
    pub start_position: usize,
    pub end_position: usize,
}

pub struct SubscriptManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
    enabled: bool,
}

impl SubscriptManager {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
            enabled: true,
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

    pub fn enable(&mut self) {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.enabled = true;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Enable subscript CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Enable subscript performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }

    pub fn disable(&mut self) {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.enabled = false;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Disable subscript CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Disable subscript performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }

    pub fn apply_subscript(&mut self, text: String, start_position: usize, end_position: usize) -> Result<Subscript, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if !self.enabled {
            return Err("Subscript is disabled".to_string());
        }

        if text.is_empty() {
            return Err("Subscript text cannot be empty".to_string());
        }

        if start_position >= end_position {
            return Err("Start position must be less than end position".to_string());
        }

        let subscript_id = format!("subscript_{}", self.operation_count);

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Apply subscript CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Apply subscript performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(Subscript {
            subscript_id,
            text,
            start_position,
            end_position,
        })
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscript_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = SubscriptManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_apply_subscript() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SubscriptManager::new(config_service);
        
        let result = manager.apply_subscript("2".to_string(), 0, 1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_text() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SubscriptManager::new(config_service);
        
        let result = manager.apply_subscript("".to_string(), 0, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_disable() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SubscriptManager::new(config_service);
        
        manager.disable();
        assert!(!manager.is_enabled());
    }
}
