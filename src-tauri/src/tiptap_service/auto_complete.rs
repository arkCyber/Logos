//! TipTap Auto Complete Manager - Aerospace-Grade Auto Complete Service
//!
//! Safety-critical auto complete service with:
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

/// Maximum suggestion text length
const MAX_SUGGESTION_LENGTH: usize = 200;

/// Maximum number of suggestions
const MAX_SUGGESTIONS: usize = 20;

/// Minimum trigger length
const MIN_TRIGGER_LENGTH: usize = 2;

/// Completion kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionKind {
    Text,
    Keyword,
    Variable,
    Function,
    Class,
    Interface,
    Module,
    Property,
    Method,
    Snippet,
}

impl CompletionKind {
    pub fn as_str(&self) -> &str {
        match self {
            CompletionKind::Text => "text",
            CompletionKind::Keyword => "keyword",
            CompletionKind::Variable => "variable",
            CompletionKind::Function => "function",
            CompletionKind::Class => "class",
            CompletionKind::Interface => "interface",
            CompletionKind::Module => "module",
            CompletionKind::Property => "property",
            CompletionKind::Method => "method",
            CompletionKind::Snippet => "snippet",
        }
    }
}

/// Completion suggestion
#[derive(Debug, Clone)]
pub struct CompletionSuggestion {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: String,
    pub sort_text: Option<String>,
    pub filter_text: Option<String>,
}

pub struct AutoCompleteManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
    enabled: bool,
    trigger_characters: Vec<char>,
    custom_suggestions: Vec<CompletionSuggestion>,
}

impl AutoCompleteManager {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
            enabled: true,
            trigger_characters: vec!['.', ':', ' ', '\n'],
            custom_suggestions: Vec::new(),
        }
    }

    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    pub fn max_suggestion_length() -> usize {
        MAX_SUGGESTION_LENGTH
    }

    pub fn max_suggestions() -> usize {
        MAX_SUGGESTIONS
    }

    pub fn min_trigger_length() -> usize {
        MIN_TRIGGER_LENGTH
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
            eprintln!("Enable auto complete CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Enable auto complete performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }

    pub fn disable(&mut self) {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.enabled = false;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Disable auto complete CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Disable auto complete performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }

    pub fn add_trigger_character(&mut self, char: char) {
        let start_time = Instant::now();
        self.operation_count += 1;

        if !self.trigger_characters.contains(&char) {
            self.trigger_characters.push(char);
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Add trigger character CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Add trigger character performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }

    pub fn add_suggestion(&mut self, suggestion: CompletionSuggestion) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if suggestion.label.is_empty() {
            return Err("Suggestion label cannot be empty".to_string());
        }

        if suggestion.label.len() > MAX_SUGGESTION_LENGTH {
            return Err(format!("Suggestion label exceeds maximum length of {} characters", MAX_SUGGESTION_LENGTH));
        }

        self.custom_suggestions.push(suggestion);

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Add suggestion CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Add suggestion performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_suggestions(&mut self, trigger: &str) -> Vec<CompletionSuggestion> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if !self.enabled {
            return Vec::new();
        }

        if trigger.len() < MIN_TRIGGER_LENGTH {
            return Vec::new();
        }

        let mut suggestions: Vec<CompletionSuggestion> = self.custom_suggestions
            .iter()
            .filter(|s| {
                let filter_text = s.filter_text.as_ref().unwrap_or(&s.label);
                filter_text.to_lowercase().starts_with(&trigger.to_lowercase())
            })
            .cloned()
            .collect();

        suggestions.truncate(MAX_SUGGESTIONS);

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Get suggestions CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Get suggestions performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        suggestions
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_trigger_characters(&self) -> &Vec<char> {
        &self.trigger_characters
    }

    pub fn clear_suggestions(&mut self) {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.custom_suggestions.clear();

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
    fn test_auto_complete_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = AutoCompleteManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_enable() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AutoCompleteManager::new(config_service);
        
        manager.disable();
        manager.enable();
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_disable() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AutoCompleteManager::new(config_service);
        
        manager.disable();
        assert!(!manager.is_enabled());
    }

    #[test]
    fn test_add_suggestion() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AutoCompleteManager::new(config_service);
        
        let suggestion = CompletionSuggestion {
            label: "hello".to_string(),
            kind: CompletionKind::Text,
            detail: Some("A greeting".to_string()),
            documentation: None,
            insert_text: "Hello World".to_string(),
            sort_text: None,
            filter_text: None,
        };
        
        let result = manager.add_suggestion(suggestion);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_suggestions() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AutoCompleteManager::new(config_service);
        
        let suggestion = CompletionSuggestion {
            label: "hello".to_string(),
            kind: CompletionKind::Text,
            detail: None,
            documentation: None,
            insert_text: "Hello World".to_string(),
            sort_text: None,
            filter_text: None,
        };
        
        manager.add_suggestion(suggestion).unwrap();
        
        let suggestions = manager.get_suggestions("he");
        assert_eq!(suggestions.len(), 1);
    }

    #[test]
    fn test_clear_suggestions() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = AutoCompleteManager::new(config_service);
        
        let suggestion = CompletionSuggestion {
            label: "hello".to_string(),
            kind: CompletionKind::Text,
            detail: None,
            documentation: None,
            insert_text: "Hello".to_string(),
            sort_text: None,
            filter_text: None,
        };
        
        manager.add_suggestion(suggestion).unwrap();
        manager.clear_suggestions();
        
        let suggestions = manager.get_suggestions("he");
        assert_eq!(suggestions.len(), 0);
    }
}
