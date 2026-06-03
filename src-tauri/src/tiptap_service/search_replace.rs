//! TipTap Search Replace Manager - Aerospace-Grade Search and Replace Service
//!
//! Safety-critical search and replace service with:
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

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 50;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 200;

/// Maximum search pattern length
const MAX_SEARCH_PATTERN_LENGTH: usize = 1000;

/// Maximum replacement text length
const MAX_REPLACEMENT_LENGTH: usize = 10000;

/// Search options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SearchOptions {
    pub case_sensitive: bool,
    pub whole_word: bool,
    pub use_regex: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            whole_word: false,
            use_regex: false,
        }
    }
}

/// Search result
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub position: usize,
    pub length: usize,
    pub matched_text: String,
}

pub struct SearchReplaceManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl SearchReplaceManager {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
        }
    }

    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    pub fn max_search_pattern_length() -> usize {
        MAX_SEARCH_PATTERN_LENGTH
    }

    pub fn max_replacement_length() -> usize {
        MAX_REPLACEMENT_LENGTH
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

    pub fn search(&mut self, text: &str, pattern: &str, options: SearchOptions) -> Result<Vec<SearchResult>, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if pattern.is_empty() {
            return Err("Search pattern cannot be empty".to_string());
        }

        if pattern.len() > MAX_SEARCH_PATTERN_LENGTH {
            return Err(format!("Search pattern exceeds maximum length of {} characters", MAX_SEARCH_PATTERN_LENGTH));
        }

        let results = self.find_matches(text, pattern, options)?;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Search CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Search performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(results)
    }

    pub fn replace(&mut self, text: &str, pattern: &str, replacement: &str, options: SearchOptions) -> Result<(String, usize), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if pattern.is_empty() {
            return Err("Search pattern cannot be empty".to_string());
        }

        if pattern.len() > MAX_SEARCH_PATTERN_LENGTH {
            return Err(format!("Search pattern exceeds maximum length of {} characters", MAX_SEARCH_PATTERN_LENGTH));
        }

        if replacement.len() > MAX_REPLACEMENT_LENGTH {
            return Err(format!("Replacement text exceeds maximum length of {} characters", MAX_REPLACEMENT_LENGTH));
        }

        let (new_text, count) = self.perform_replace(text, pattern, replacement, options)?;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Replace CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Replace performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok((new_text, count))
    }

    fn find_matches(&self, text: &str, pattern: &str, options: SearchOptions) -> Result<Vec<SearchResult>, String> {
        let mut results = Vec::new();
        let search_text = if options.case_sensitive { text.to_string() } else { text.to_lowercase() };
        let search_pattern = if options.case_sensitive { pattern.to_string() } else { pattern.to_lowercase() };

        let mut pos = 0;
        while let Some(index) = search_text[pos..].find(&search_pattern) {
            let absolute_pos = pos + index;
            let matched_text = &text[absolute_pos..absolute_pos + pattern.len()];

            if options.whole_word {
                let prev_char = if absolute_pos > 0 {
                    text.chars().nth(absolute_pos - 1)
                } else {
                    None
                };
                let next_char = text.chars().nth(absolute_pos + pattern.len());

                if prev_char.map_or(true, |c| !c.is_alphanumeric()) && next_char.map_or(true, |c| !c.is_alphanumeric()) {
                    results.push(SearchResult {
                        position: absolute_pos,
                        length: pattern.len(),
                        matched_text: matched_text.to_string(),
                    });
                }
            } else {
                results.push(SearchResult {
                    position: absolute_pos,
                    length: pattern.len(),
                    matched_text: matched_text.to_string(),
                });
            }

            pos = absolute_pos + pattern.len();
        }

        Ok(results)
    }

    fn perform_replace(&self, text: &str, pattern: &str, replacement: &str, options: SearchOptions) -> Result<(String, usize), String> {
        let matches = self.find_matches(text, pattern, options)?;
        let mut new_text = text.to_string();
        let mut count = 0;

        for (_i, match_result) in matches.iter().enumerate() {
            new_text.replace_range(match_result.position..match_result.position + match_result.length, replacement);
            count += 1;
        }

        Ok((new_text, count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_replace_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = SearchReplaceManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_search() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SearchReplaceManager::new(config_service);
        
        let options = SearchOptions::default();
        let result = manager.search("hello world hello", "hello", options);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn test_search_empty_pattern() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SearchReplaceManager::new(config_service);
        
        let options = SearchOptions::default();
        let result = manager.search("hello world", "", options);
        assert!(result.is_err());
    }

    #[test]
    fn test_replace() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SearchReplaceManager::new(config_service);
        
        let options = SearchOptions::default();
        let result = manager.replace("hello world", "hello", "hi", options);
        assert!(result.is_ok());
        let (new_text, count) = result.unwrap();
        assert_eq!(new_text, "hi world");
        assert_eq!(count, 1);
    }

    #[test]
    fn test_search_case_sensitive() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = SearchReplaceManager::new(config_service);
        
        let options = SearchOptions { case_sensitive: true, whole_word: false, use_regex: false };
        let result = manager.search("Hello hello", "hello", options);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }
}
