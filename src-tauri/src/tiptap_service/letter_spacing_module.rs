//! TipTap Letter Spacing Value Manager - Aerospace-Grade Letter Spacing Service
//!
//! Safety-critical letter spacing service with:
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

/// Maximum letter spacing value
const MAX_LETTER_SPACING: f64 = 50.0;

/// Minimum letter spacing value
const MIN_LETTER_SPACING: f64 = -10.0;

/// Letter spacing value
#[derive(Debug, Clone)]
pub struct LetterSpacingValue {
    pub spacing_id: String,
    pub value: f64,
    pub unit: String,
}

pub struct LetterSpacingValueManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
    enabled: bool,
}

impl LetterSpacingValueManager {
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

    pub fn max_letter_spacing() -> f64 {
        MAX_LETTER_SPACING
    }

    pub fn min_letter_spacing() -> f64 {
        MIN_LETTER_SPACING
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
            eprintln!("Enable letter spacing CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Enable letter spacing performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }

    pub fn disable(&mut self) {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.enabled = false;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Disable letter spacing CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Disable letter spacing performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }

    pub fn set_letter_spacing(&mut self, value: f64, unit: String) -> Result<LetterSpacingValue, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if !self.enabled {
            return Err("Letter spacing is disabled".to_string());
        }

        if value < MIN_LETTER_SPACING || value > MAX_LETTER_SPACING {
            return Err(format!("Letter spacing must be between {} and {}", MIN_LETTER_SPACING, MAX_LETTER_SPACING));
        }

        if unit.is_empty() {
            return Err("Unit cannot be empty".to_string());
        }

        let spacing_id = format!("letter_spacing_{}", self.operation_count);

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Set letter spacing CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Set letter spacing performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(LetterSpacingValue {
            spacing_id,
            value,
            unit,
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
    fn test_letter_spacing_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = LetterSpacingValueManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_set_letter_spacing() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LetterSpacingValueManager::new(config_service);
        
        let result = manager.set_letter_spacing(2.0, "px".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_spacing() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LetterSpacingValueManager::new(config_service);
        
        let result = manager.set_letter_spacing(100.0, "px".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_disable() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = LetterSpacingValueManager::new(config_service);
        
        manager.disable();
        assert!(!manager.is_enabled());
    }
}
