//! TipTap Word Break Manager - Aerospace-Grade Word Break Operations Service
//!
//! Safety-critical word break operations service with:
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
use super::editor::TipTapNode;

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 50;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 200;

/// Maximum word break string length
const MAX_WORD_BREAK_LENGTH: usize = 50;

/// Word break type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordBreak {
    Normal,
    BreakAll,
    KeepAll,
    BreakWord,
}

impl WordBreak {
    /// Convert word break to string
    pub fn as_str(&self) -> &str {
        match self {
            WordBreak::Normal => "normal",
            WordBreak::BreakAll => "break-all",
            WordBreak::KeepAll => "keep-all",
            WordBreak::BreakWord => "break-word",
        }
    }

    /// Parse word break from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(WordBreak::Normal),
            "break-all" => Ok(WordBreak::BreakAll),
            "keep-all" => Ok(WordBreak::KeepAll),
            "break-word" => Ok(WordBreak::BreakWord),
            _ => Err(format!("Invalid word break: {}", s)),
        }
    }
}

pub struct WordBreakManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl WordBreakManager {
    /// Creates a new word break manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new WordBreakManager instance
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
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

    /// Get the maximum word break length constant
    /// 
    /// # Returns
    /// The maximum word break string length
    pub fn max_word_break_length() -> usize {
        MAX_WORD_BREAK_LENGTH
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

    /// Validate word break string
    /// 
    /// # Arguments
    /// * `word_break` - The word break string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting word break string length
    fn validate_word_break(&self, word_break: &str) -> Result<(), String> {
        if word_break.len() > MAX_WORD_BREAK_LENGTH {
            return Err(format!("Word break string exceeds maximum length of {} characters", MAX_WORD_BREAK_LENGTH));
        }
        
        // Validate word break value
        WordBreak::from_str(word_break)?;
        
        Ok(())
    }

    /// Apply word break to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply word break to
    /// * `word_break` - The word break to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates word break string
    pub fn apply_word_break(&mut self, node: &mut TipTapNode, word_break: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate word break
        self.validate_word_break(word_break)?;

        // Apply word break to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("wordBreak".to_string(), serde_json::Value::String(word_break.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "wordBreak": word_break }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Word break application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Word break application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove word break from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove word break from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_word_break(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("wordBreak");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Word break removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Word break removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get word break from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get word break from
    /// 
    /// # Returns
    /// Option containing the word break string or None
    pub fn get_word_break(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(word_break) = obj.get("wordBreak") {
                    if let Some(s) = word_break.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has word break
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has word break, false otherwise
    pub fn has_word_break(&self, node: &TipTapNode) -> bool {
        self.get_word_break(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_word_break_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = WordBreakManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(WordBreakManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(WordBreakManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(WordBreakManager::max_word_break_length(), MAX_WORD_BREAK_LENGTH);
    }

    #[test]
    fn test_word_break_variants() {
        assert_eq!(WordBreak::Normal.as_str(), "normal");
        assert_eq!(WordBreak::BreakAll.as_str(), "break-all");
        assert_eq!(WordBreak::KeepAll.as_str(), "keep-all");
    }

    #[test]
    fn test_word_break_from_str() {
        assert!(matches!(WordBreak::from_str("normal"), Ok(WordBreak::Normal)));
        assert!(matches!(WordBreak::from_str("break-all"), Ok(WordBreak::BreakAll)));
        assert!(matches!(WordBreak::from_str("keep-all"), Ok(WordBreak::KeepAll)));
        assert!(WordBreak::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_word_break() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WordBreakManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_word_break(&mut node, "break-all");
        assert!(result.is_ok());
        assert!(manager.has_word_break(&node));
    }

    #[test]
    fn test_apply_word_break_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WordBreakManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_word_break(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_word_break_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WordBreakManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_word_break = "a".repeat(MAX_WORD_BREAK_LENGTH + 1);
        let result = manager.apply_word_break(&mut node, &long_word_break);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_word_break() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WordBreakManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "wordBreak": "break-all" })),
            marks: None,
        };
        
        assert!(manager.has_word_break(&node));
        let result = manager.remove_word_break(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_word_break(&node));
    }

    #[test]
    fn test_get_word_break() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = WordBreakManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "wordBreak": "keep-all" })),
            marks: None,
        };
        
        let word_break = manager.get_word_break(&node);
        assert_eq!(word_break, Some("keep-all".to_string()));
    }

    #[test]
    fn test_get_word_break_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = WordBreakManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let word_break = manager.get_word_break(&node);
        assert!(word_break.is_none());
    }

    #[test]
    fn test_has_word_break() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = WordBreakManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "wordBreak": "break-word" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_word_break(&node_with));
        assert!(!manager.has_word_break(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WordBreakManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_word_break(&mut node, "break-all").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WordBreakManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_word_break(&mut node, "break-all").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WordBreakManager::new(config_service);
        
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
        let mut manager = WordBreakManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
