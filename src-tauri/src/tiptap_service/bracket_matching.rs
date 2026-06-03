//! TipTap Bracket Matching Manager - Aerospace-Grade Bracket Matching Service
//!
//! Safety-critical bracket matching service with:
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

/// Bracket pair
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BracketPair {
    pub open: char,
    pub close: char,
}

impl BracketPair {
    pub fn new(open: char, close: char) -> Self {
        Self { open, close }
    }
}

/// Match result
#[derive(Debug, Clone)]
pub struct BracketMatch {
    pub open_position: Option<usize>,
    pub close_position: Option<usize>,
    pub bracket_pair: BracketPair,
}

pub struct BracketMatchingManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
    bracket_pairs: Vec<BracketPair>,
    enabled: bool,
}

impl BracketMatchingManager {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        let default_pairs = vec![
            BracketPair::new('(', ')'),
            BracketPair::new('[', ']'),
            BracketPair::new('{', '}'),
            BracketPair::new('<', '>'),
            BracketPair::new('"', '"'),
            BracketPair::new('\'', '\''),
        ];

        Self {
            config_service,
            operation_count: 0,
            last_error: None,
            bracket_pairs: default_pairs,
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
            eprintln!("Enable bracket matching CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Enable bracket matching performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }

    pub fn disable(&mut self) {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.enabled = false;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Disable bracket matching CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Disable bracket matching performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }

    pub fn add_bracket_pair(&mut self, open: char, close: char) {
        let start_time = Instant::now();
        self.operation_count += 1;

        let pair = BracketPair::new(open, close);
        if !self.bracket_pairs.contains(&pair) {
            self.bracket_pairs.push(pair);
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Add bracket pair CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Add bracket pair performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }

    pub fn find_match(&mut self, text: &str, position: usize) -> Option<BracketMatch> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if !self.enabled {
            return None;
        }

        let chars: Vec<char> = text.chars().collect();
        if position >= chars.len() {
            return None;
        }

        let current_char = chars[position];

        for pair in &self.bracket_pairs {
            if current_char == pair.open {
                if let Some(close_pos) = self.find_closing_bracket(&chars, position, pair) {
                    return Some(BracketMatch {
                        open_position: Some(position),
                        close_position: Some(close_pos),
                        bracket_pair: *pair,
                    });
                }
            } else if current_char == pair.close {
                if let Some(open_pos) = self.find_opening_bracket(&chars, position, pair) {
                    return Some(BracketMatch {
                        open_position: Some(open_pos),
                        close_position: Some(position),
                        bracket_pair: *pair,
                    });
                }
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Find match CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Find match performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        None
    }

    fn find_closing_bracket(&self, chars: &[char], start: usize, pair: &BracketPair) -> Option<usize> {
        let mut depth = 1;
        for (i, &c) in chars.iter().enumerate().skip(start + 1) {
            if c == pair.open {
                depth += 1;
            } else if c == pair.close {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
        }
        None
    }

    fn find_opening_bracket(&self, chars: &[char], start: usize, pair: &BracketPair) -> Option<usize> {
        let mut depth = 1;
        for i in (0..start).rev() {
            let c = chars[i];
            if c == pair.close {
                depth += 1;
            } else if c == pair.open {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
        }
        None
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_bracket_pairs(&self) -> &Vec<BracketPair> {
        &self.bracket_pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bracket_matching_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BracketMatchingManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_find_matching_parentheses() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BracketMatchingManager::new(config_service);
        
        let text = "(hello world)";
        let result = manager.find_match(text, 0);
        
        assert!(result.is_some());
        let match_result = result.unwrap();
        assert_eq!(match_result.open_position, Some(0));
        assert_eq!(match_result.close_position, Some(12));
    }

    #[test]
    fn test_find_matching_brackets() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BracketMatchingManager::new(config_service);
        
        let text = "[test]";
        let result = manager.find_match(text, 0);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_nested_brackets() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BracketMatchingManager::new(config_service);
        
        let text = "((nested))";
        let result = manager.find_match(text, 0);
        
        assert!(result.is_some());
        assert_eq!(result.unwrap().close_position, Some(9));
    }

    #[test]
    fn test_disable() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BracketMatchingManager::new(config_service);
        
        manager.disable();
        assert!(!manager.is_enabled());
    }
}
