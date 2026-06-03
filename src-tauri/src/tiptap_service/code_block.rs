//! TipTap Code Block Manager - Aerospace-Grade Code Block Operations Service
//!
//! Safety-critical code block operations service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Comprehensive error handling
//! - Performance monitoring
//! - Security hardening
//! - Fault tolerance and error recovery

use serde::{Deserialize, Serialize};
use std::time::Instant;
use crate::error_handling::{ErrorContext, ErrorSeverity};
use crate::config_service::ExportConfigService;
use std::sync::Arc;
use super::editor::{TipTapNode, NodeType};

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 50;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 200;

/// Maximum code block depth to prevent stack overflow
const MAX_CODE_BLOCK_DEPTH: usize = 10;

/// Maximum code block text length
const MAX_CODE_BLOCK_TEXT_LENGTH: usize = 50000;

/// Maximum language name length
const MAX_LANGUAGE_LENGTH: usize = 50;

/// Code block attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlockAttributes {
    pub language: Option<String>,
    pub line_numbers: Option<bool>,
}

impl Default for CodeBlockAttributes {
    fn default() -> Self {
        Self {
            language: None,
            line_numbers: None,
        }
    }
}

pub struct CodeBlockManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl CodeBlockManager {
    /// Creates a new code block manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new CodeBlockManager instance
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

    /// Get the maximum code block depth constant
    /// 
    /// # Returns
    /// The maximum code block depth
    pub fn max_code_block_depth() -> usize {
        MAX_CODE_BLOCK_DEPTH
    }

    /// Get the maximum code block text length constant
    /// 
    /// # Returns
    /// The maximum text length for code blocks
    pub fn max_code_block_text_length() -> usize {
        MAX_CODE_BLOCK_TEXT_LENGTH
    }

    /// Get the maximum language length constant
    /// 
    /// # Returns
    /// The maximum language name length
    pub fn max_language_length() -> usize {
        MAX_LANGUAGE_LENGTH
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

    /// Validate code block depth
    /// 
    /// # Arguments
    /// * `depth` - The current code block depth
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents stack overflow by limiting depth
    fn validate_code_block_depth(&self, depth: usize) -> Result<(), String> {
        if depth >= MAX_CODE_BLOCK_DEPTH {
            return Err(format!("Code block depth exceeds maximum of {}", MAX_CODE_BLOCK_DEPTH));
        }
        Ok(())
    }

    /// Validate code block text length
    /// 
    /// # Arguments
    /// * `text` - The text to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting text length
    fn validate_code_block_text(&self, text: &str) -> Result<(), String> {
        if text.len() > MAX_CODE_BLOCK_TEXT_LENGTH {
            return Err(format!("Code block text exceeds maximum length of {} characters", MAX_CODE_BLOCK_TEXT_LENGTH));
        }
        Ok(())
    }

    /// Validate language name
    /// 
    /// # Arguments
    /// * `language` - The language name to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting language name length
    fn validate_language(&self, language: &str) -> Result<(), String> {
        if language.len() > MAX_LANGUAGE_LENGTH {
            return Err(format!("Language name exceeds maximum length of {} characters", MAX_LANGUAGE_LENGTH));
        }
        Ok(())
    }

    /// Create a code block node
    /// 
    /// # Arguments
    /// * `code` - The code content
    /// * `attributes` - Optional code block attributes
    /// * `depth` - The nesting depth
    /// 
    /// # Returns
    /// Result containing the code block node or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates depth, text length, and language name
    pub fn create_code_block(&mut self, code: &str, attributes: Option<CodeBlockAttributes>, depth: usize) -> Result<TipTapNode, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate depth
        self.validate_code_block_depth(depth)?;

        // Validate text length
        self.validate_code_block_text(code)?;

        // Validate language
        if let Some(ref attrs) = attributes {
            if let Some(ref language) = attrs.language {
                self.validate_language(language)?;
            }
        }

        let attrs_json = if let Some(attrs) = attributes {
            Some(serde_json::to_value(attrs).map_err(|e| {
                let error = format!("Failed to serialize code block attributes: {}", e);
                self.record_error("SERIALIZE_ERROR", &error, "create_code_block");
                error
            })?)
        } else {
            None
        };

        let code_node = TipTapNode {
            node_type: NodeType::CodeBlock,
            content: Some(vec![TipTapNode {
                node_type: NodeType::Text,
                content: None,
                text: Some(code.to_string()),
                attrs: None,
                marks: None,
            }]),
            text: None,
            attrs: attrs_json,
            marks: None,
        };

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Code block creation CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Code block creation performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(code_node)
    }

    /// Update code block content
    /// 
    /// # Arguments
    /// * `code_block_node` - The code block node to update
    /// * `new_code` - The new code content
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn update_code(&mut self, code_block_node: &mut TipTapNode, new_code: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate text length
        self.validate_code_block_text(new_code)?;

        if let Some(ref mut content) = code_block_node.content {
            if let Some(ref mut text_node) = content.first_mut() {
                text_node.text = Some(new_code.to_string());
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Code block update CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Code block update performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Update code block attributes
    /// 
    /// # Arguments
    /// * `code_block_node` - The code block node to update
    /// * `attributes` - The new attributes
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn update_attributes(&mut self, code_block_node: &mut TipTapNode, attributes: CodeBlockAttributes) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate language
        if let Some(ref language) = attributes.language {
            self.validate_language(language)?;
        }

        let attrs_json = serde_json::to_value(&attributes).map_err(|e| {
            let error = format!("Failed to serialize code block attributes: {}", e);
            self.record_error("SERIALIZE_ERROR", &error, "update_attributes");
            error
        })?;

        code_block_node.attrs = Some(attrs_json);

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Code block attributes update CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Code block attributes update performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_code_block_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = CodeBlockManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(CodeBlockManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(CodeBlockManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(CodeBlockManager::max_code_block_depth(), MAX_CODE_BLOCK_DEPTH);
        assert_eq!(CodeBlockManager::max_code_block_text_length(), MAX_CODE_BLOCK_TEXT_LENGTH);
        assert_eq!(CodeBlockManager::max_language_length(), MAX_LANGUAGE_LENGTH);
    }

    #[test]
    fn test_create_code_block() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeBlockManager::new(config_service);
        
        let code = "fn main() { println!(\"Hello\"); }";
        let result = manager.create_code_block(code, None, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_code_block_with_language() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeBlockManager::new(config_service);
        
        let code = "fn main() { println!(\"Hello\"); }";
        let attributes = CodeBlockAttributes {
            language: Some("rust".to_string()),
            line_numbers: Some(true),
        };
        let result = manager.create_code_block(code, Some(attributes), 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_code_block_too_deep() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeBlockManager::new(config_service);
        
        let code = "test";
        let result = manager.create_code_block(code, None, MAX_CODE_BLOCK_DEPTH);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_code_block_text_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeBlockManager::new(config_service);
        
        let long_code = "a".repeat(MAX_CODE_BLOCK_TEXT_LENGTH + 1);
        let result = manager.create_code_block(&long_code, None, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_code_block_language_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeBlockManager::new(config_service);
        
        let long_language = "a".repeat(MAX_LANGUAGE_LENGTH + 1);
        let attributes = CodeBlockAttributes {
            language: Some(long_language),
            line_numbers: None,
        };
        let result = manager.create_code_block("test", Some(attributes), 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_code() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeBlockManager::new(config_service);
        
        let mut code_block_node = manager.create_code_block("old code", None, 0).unwrap();
        let result = manager.update_code(&mut code_block_node, "new code");
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_code_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeBlockManager::new(config_service);
        
        let mut code_block_node = manager.create_code_block("old code", None, 0).unwrap();
        let long_code = "a".repeat(MAX_CODE_BLOCK_TEXT_LENGTH + 1);
        let result = manager.update_code(&mut code_block_node, &long_code);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_attributes() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeBlockManager::new(config_service);
        
        let mut code_block_node = manager.create_code_block("test", None, 0).unwrap();
        let attributes = CodeBlockAttributes {
            language: Some("python".to_string()),
            line_numbers: Some(false),
        };
        let result = manager.update_attributes(&mut code_block_node, attributes);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_attributes_language_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeBlockManager::new(config_service);
        
        let mut code_block_node = manager.create_code_block("test", None, 0).unwrap();
        let long_language = "a".repeat(MAX_LANGUAGE_LENGTH + 1);
        let attributes = CodeBlockAttributes {
            language: Some(long_language),
            line_numbers: None,
        };
        let result = manager.update_attributes(&mut code_block_node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeBlockManager::new(config_service);
        
        manager.create_code_block("test", None, 0).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeBlockManager::new(config_service);
        
        manager.create_code_block("test", None, 0).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeBlockManager::new(config_service);
        
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
        let mut manager = CodeBlockManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }

    #[test]
    fn test_code_block_attributes_default() {
        let attrs = CodeBlockAttributes::default();
        assert!(attrs.language.is_none());
        assert!(attrs.line_numbers.is_none());
    }
}
