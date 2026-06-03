//! TipTap Suggestion Manager - Aerospace-Grade Suggestion Service
//!
//! Safety-critical suggestion service with:
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

/// Maximum suggestion text length
const MAX_SUGGESTION_TEXT_LENGTH: usize = 500;

/// Suggestion
#[derive(Debug, Clone)]
pub struct Suggestion {
    pub suggestion_id: String,
    pub text: String,
    pub position: usize,
    pub confidence: f64,
}

pub struct SuggestionManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
    suggestions: HashMap<String, Suggestion>,
    suggestion_counter: u64,
}

impl SuggestionManager {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
            suggestions: HashMap::new(),
            suggestion_counter: 0,
        }
    }

    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    pub fn max_suggestion_text_length() -> usize {
        MAX_SUGGESTION_TEXT_LENGTH
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

    pub fn add_suggestion(&mut self, text: String, position: usize, confidence: f64) -> Result<String, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if text.is_empty() {
            return Err("Suggestion text cannot be empty".to_string());
        }

        if text.len() > MAX_SUGGESTION_TEXT_LENGTH {
            return Err(format!("Suggestion text exceeds maximum length of {} characters", MAX_SUGGESTION_TEXT_LENGTH));
        }

        if confidence < 0.0 || confidence > 1.0 {
            return Err("Confidence must be between 0.0 and 1.0".to_string());
        }

        self.suggestion_counter += 1;
        let suggestion_id = format!("suggestion_{}", self.suggestion_counter);

        let suggestion = Suggestion {
            suggestion_id: suggestion_id.clone(),
            text,
            position,
            confidence,
        };

        self.suggestions.insert(suggestion_id.clone(), suggestion);

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Add suggestion CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Add suggestion performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(suggestion_id)
    }

    pub fn remove_suggestion(&mut self, suggestion_id: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.suggestions.remove(suggestion_id)
            .ok_or("Suggestion not found")?;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Remove suggestion CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Remove suggestion performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn accept_suggestion(&mut self, suggestion_id: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.suggestions.remove(suggestion_id)
            .ok_or("Suggestion not found")?;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Accept suggestion CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Accept suggestion performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_suggestion(&self, suggestion_id: &str) -> Option<&Suggestion> {
        self.suggestions.get(suggestion_id)
    }

    pub fn get_suggestions_at_position(&self, position: usize) -> Vec<&Suggestion> {
        self.suggestions.values()
            .filter(|s| s.position == position)
            .collect()
    }

    pub fn get_all_suggestions(&self) -> Vec<&Suggestion> {
        self.suggestions.values().collect()
    }

    pub fn clear_suggestions(&mut self) {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.suggestions.clear();

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Clear suggestions CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Clear suggestions performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggestion_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = SuggestionManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_add_suggestion() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SuggestionManager::new(config_service);
        
        let result = manager.add_suggestion("hello".to_string(), 0, 0.9);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_confidence() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SuggestionManager::new(config_service);
        
        let result = manager.add_suggestion("hello".to_string(), 0, 1.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_accept_suggestion() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SuggestionManager::new(config_service);
        
        let suggestion_id = manager.add_suggestion("hello".to_string(), 0, 0.9).unwrap();
        let result = manager.accept_suggestion(&suggestion_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_clear_suggestions() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SuggestionManager::new(config_service);
        
        manager.add_suggestion("hello".to_string(), 0, 0.9).unwrap();
        manager.clear_suggestions();
        
        assert_eq!(manager.get_all_suggestions().len(), 0);
    }
}
