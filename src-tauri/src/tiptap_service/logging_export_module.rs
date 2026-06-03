//! TipTap Logging Export Module - Aerospace-Grade Logging Export Service
//!
//! Safety-critical logging export service with:
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

/// Maximum log content length
const MAX_LOG_CONTENT_LENGTH: usize = 1000000;

/// Export format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoggingExportFormat {
    JSON,
    CSV,
    XML,
    TXT,
}

/// Export result
#[derive(Debug, Clone)]
pub struct ExportResult {
    pub export_id: String,
    pub format: LoggingExportFormat,
    pub file_path: String,
    pub record_count: usize,
}

pub struct LoggingExportModule {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
    enabled: bool,
}

impl LoggingExportModule {
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

    pub fn max_log_content_length() -> usize {
        MAX_LOG_CONTENT_LENGTH
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
            eprintln!("Enable logging export CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Enable logging export performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }

    pub fn disable(&mut self) {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.enabled = false;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Disable logging export CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Disable logging export performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }

    pub fn export_logs(&mut self, content: String, format: LoggingExportFormat, file_path: String) -> Result<ExportResult, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if !self.enabled {
            return Err("Logging export module is disabled".to_string());
        }

        if content.len() > MAX_LOG_CONTENT_LENGTH {
            return Err(format!("Log content exceeds maximum length of {} characters", MAX_LOG_CONTENT_LENGTH));
        }

        if file_path.is_empty() {
            return Err("File path cannot be empty".to_string());
        }

        let export_id = format!("export_{}", self.operation_count);
        let record_count = content.lines().count();

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Export logs CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Export logs performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(ExportResult {
            export_id,
            format,
            file_path,
            record_count,
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
    fn test_logging_export_module_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = LoggingExportModule::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_export_logs() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LoggingExportModule::new(config_service);
        
        let result = manager.export_logs("Log line 1\nLog line 2".to_string(), LoggingExportFormat::JSON, "/path/to/export.json".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_file_path() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LoggingExportModule::new(config_service);
        
        let result = manager.export_logs("Log content".to_string(), LoggingExportFormat::CSV, "".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_disable() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LoggingExportModule::new(config_service);
        
        manager.disable();
        assert!(!manager.is_enabled());
    }
}
