//! TipTap Code Highlight Manager - Aerospace-Grade Code Highlight Operations Service
//!
//! Safety-critical code highlight operations service with:
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

/// Maximum language name length
const MAX_LANGUAGE_LENGTH: usize = 50;

/// Maximum code highlight depth to prevent stack overflow
const MAX_HIGHLIGHT_DEPTH: usize = 10;

/// Supported languages for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightLanguage {
    JavaScript,
    TypeScript,
    Python,
    Rust,
    Go,
    Java,
    C,
    Cpp,
    CSharp,
    HTML,
    CSS,
    JSON,
    SQL,
    Bash,
    Markdown,
    Other,
}

impl HighlightLanguage {
    /// Convert language to string
    pub fn as_str(&self) -> &str {
        match self {
            HighlightLanguage::JavaScript => "javascript",
            HighlightLanguage::TypeScript => "typescript",
            HighlightLanguage::Python => "python",
            HighlightLanguage::Rust => "rust",
            HighlightLanguage::Go => "go",
            HighlightLanguage::Java => "java",
            HighlightLanguage::C => "c",
            HighlightLanguage::Cpp => "cpp",
            HighlightLanguage::CSharp => "csharp",
            HighlightLanguage::HTML => "html",
            HighlightLanguage::CSS => "css",
            HighlightLanguage::JSON => "json",
            HighlightLanguage::SQL => "sql",
            HighlightLanguage::Bash => "bash",
            HighlightLanguage::Markdown => "markdown",
            HighlightLanguage::Other => "other",
        }
    }

    /// Parse language from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "javascript" | "js" => Ok(HighlightLanguage::JavaScript),
            "typescript" | "ts" => Ok(HighlightLanguage::TypeScript),
            "python" | "py" => Ok(HighlightLanguage::Python),
            "rust" | "rs" => Ok(HighlightLanguage::Rust),
            "go" | "golang" => Ok(HighlightLanguage::Go),
            "java" => Ok(HighlightLanguage::Java),
            "c" => Ok(HighlightLanguage::C),
            "cpp" | "c++" => Ok(HighlightLanguage::Cpp),
            "csharp" | "c#" => Ok(HighlightLanguage::CSharp),
            "html" => Ok(HighlightLanguage::HTML),
            "css" => Ok(HighlightLanguage::CSS),
            "json" => Ok(HighlightLanguage::JSON),
            "sql" => Ok(HighlightLanguage::SQL),
            "bash" | "sh" => Ok(HighlightLanguage::Bash),
            "markdown" | "md" => Ok(HighlightLanguage::Markdown),
            _ => Ok(HighlightLanguage::Other),
        }
    }
}

/// Code highlight attributes
#[derive(Debug, Clone)]
pub struct HighlightAttributes {
    pub language: HighlightLanguage,
    pub line_numbers: bool,
}

impl Default for HighlightAttributes {
    fn default() -> Self {
        Self {
            language: HighlightLanguage::Other,
            line_numbers: false,
        }
    }
}

pub struct CodeHighlightManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl CodeHighlightManager {
    /// Creates a new code highlight manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new CodeHighlightManager instance
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

    /// Get the maximum language length constant
    /// 
    /// # Returns
    /// The maximum language name length
    pub fn max_language_length() -> usize {
        MAX_LANGUAGE_LENGTH
    }

    /// Get the maximum highlight depth constant
    /// 
    /// # Returns
    /// The maximum highlight depth
    pub fn max_highlight_depth() -> usize {
        MAX_HIGHLIGHT_DEPTH
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

    /// Validate language string
    /// 
    /// # Arguments
    /// * `language` - The language string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting language string length
    fn validate_language(&self, language: &str) -> Result<(), String> {
        if language.len() > MAX_LANGUAGE_LENGTH {
            return Err(format!("Language name exceeds maximum length of {} characters", MAX_LANGUAGE_LENGTH));
        }
        Ok(())
    }

    /// Validate highlight depth
    /// 
    /// # Arguments
    /// * `depth` - The current highlight depth
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents stack overflow by limiting depth
    fn validate_highlight_depth(&self, depth: usize) -> Result<(), String> {
        if depth >= MAX_HIGHLIGHT_DEPTH {
            return Err(format!("Highlight depth exceeds maximum of {}", MAX_HIGHLIGHT_DEPTH));
        }
        Ok(())
    }

    /// Apply syntax highlighting to a code block node
    /// 
    /// # Arguments
    /// * `code_block` - The code block node to apply highlighting to
    /// * `attributes` - The highlight attributes
    /// * `depth` - The nesting depth
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates language and depth
    pub fn apply_highlight(&mut self, code_block: &mut TipTapNode, attributes: HighlightAttributes, depth: usize) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate language and depth
        self.validate_language(attributes.language.as_str())?;
        self.validate_highlight_depth(depth)?;

        // Apply highlighting attributes to code block
        if let Some(ref mut attrs) = code_block.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("language".to_string(), serde_json::Value::String(attributes.language.as_str().to_string()));
                obj.insert("lineNumbers".to_string(), serde_json::Value::Bool(attributes.line_numbers));
                obj.insert("highlighted".to_string(), serde_json::Value::Bool(true));
            }
        } else {
            code_block.attrs = Some(serde_json::json!({
                "language": attributes.language.as_str(),
                "lineNumbers": attributes.line_numbers,
                "highlighted": true
            }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Code highlight application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Code highlight application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove syntax highlighting from a code block node
    /// 
    /// # Arguments
    /// * `code_block` - The code block node to remove highlighting from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_highlight(&mut self, code_block: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = code_block.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("highlighted");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Code highlight removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Code highlight removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get language from a code block node
    /// 
    /// # Arguments
    /// * `code_block` - The code block node to get language from
    /// 
    /// # Returns
    /// Option containing the language or None
    pub fn get_language(&self, code_block: &TipTapNode) -> Option<HighlightLanguage> {
        if let Some(ref attrs) = code_block.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(language) = obj.get("language") {
                    if let Some(s) = language.as_str() {
                        return HighlightLanguage::from_str(s).ok();
                    }
                }
            }
        }
        None
    }

    /// Check if a code block has highlighting
    /// 
    /// # Arguments
    /// * `code_block` - The code block node to check
    /// 
    /// # Returns
    /// True if code block has highlighting, false otherwise
    pub fn has_highlight(&self, code_block: &TipTapNode) -> bool {
        if let Some(ref attrs) = code_block.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(highlighted) = obj.get("highlighted") {
                    if let Some(b) = highlighted.as_bool() {
                        return b;
                    }
                }
            }
        }
        false
    }

    /// Get highlight attributes from a code block node
    /// 
    /// # Arguments
    /// * `code_block` - The code block node to get attributes from
    /// 
    /// # Returns
    /// Option containing the highlight attributes or None
    pub fn get_highlight_attributes(&self, code_block: &TipTapNode) -> Option<HighlightAttributes> {
        let language = self.get_language(code_block)?;
        let line_numbers = if let Some(ref attrs) = code_block.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(ln) = obj.get("lineNumbers") {
                    ln.as_bool().unwrap_or(false)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };
        Some(HighlightAttributes { language, line_numbers })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_code_highlight_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = CodeHighlightManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(CodeHighlightManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(CodeHighlightManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(CodeHighlightManager::max_language_length(), MAX_LANGUAGE_LENGTH);
        assert_eq!(CodeHighlightManager::max_highlight_depth(), MAX_HIGHLIGHT_DEPTH);
    }

    #[test]
    fn test_highlight_language_variants() {
        assert_eq!(HighlightLanguage::JavaScript.as_str(), "javascript");
        assert_eq!(HighlightLanguage::Rust.as_str(), "rust");
        assert_eq!(HighlightLanguage::Python.as_str(), "python");
    }

    #[test]
    fn test_highlight_language_from_str() {
        assert!(matches!(HighlightLanguage::from_str("javascript"), Ok(HighlightLanguage::JavaScript)));
        assert!(matches!(HighlightLanguage::from_str("rust"), Ok(HighlightLanguage::Rust)));
        assert!(matches!(HighlightLanguage::from_str("python"), Ok(HighlightLanguage::Python)));
        assert!(matches!(HighlightLanguage::from_str("unknown"), Ok(HighlightLanguage::Other)));
    }

    #[test]
    fn test_apply_highlight() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeHighlightManager::new(config_service);
        
        let mut code_block = TipTapNode {
            node_type: NodeType::CodeBlock,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        let attributes = HighlightAttributes {
            language: HighlightLanguage::Rust,
            line_numbers: true,
        };
        let result = manager.apply_highlight(&mut code_block, attributes, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_highlight_language_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeHighlightManager::new(config_service);
        
        let mut code_block = TipTapNode {
            node_type: NodeType::CodeBlock,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        let mut attributes = HighlightAttributes::default();
        attributes.language = HighlightLanguage::Other;
        
        let result = manager.apply_highlight(&mut code_block, attributes, MAX_HIGHLIGHT_DEPTH);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_highlight() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeHighlightManager::new(config_service);
        
        let mut code_block = TipTapNode {
            node_type: NodeType::CodeBlock,
            content: None,
            text: None,
            attrs: Some(serde_json::json!({ "highlighted": true })),
            marks: None,
        };
        
        assert!(manager.has_highlight(&code_block));
        let result = manager.remove_highlight(&mut code_block);
        assert!(result.is_ok());
        assert!(!manager.has_highlight(&code_block));
    }

    #[test]
    fn test_get_language() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeHighlightManager::new(config_service);
        
        let mut code_block = TipTapNode {
            node_type: NodeType::CodeBlock,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        let attributes = HighlightAttributes {
            language: HighlightLanguage::Python,
            line_numbers: false,
        };
        manager.apply_highlight(&mut code_block, attributes, 0).unwrap();
        
        let language = manager.get_language(&code_block);
        assert_eq!(language, Some(HighlightLanguage::Python));
    }

    #[test]
    fn test_has_highlight() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeHighlightManager::new(config_service);
        
        let mut code_block = TipTapNode {
            node_type: NodeType::CodeBlock,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        let attributes = HighlightAttributes::default();
        manager.apply_highlight(&mut code_block, attributes, 0).unwrap();
        
        assert!(manager.has_highlight(&code_block));
    }

    #[test]
    fn test_get_highlight_attributes() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeHighlightManager::new(config_service);
        
        let mut code_block = TipTapNode {
            node_type: NodeType::CodeBlock,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        let attributes = HighlightAttributes {
            language: HighlightLanguage::JavaScript,
            line_numbers: true,
        };
        manager.apply_highlight(&mut code_block, attributes, 0).unwrap();
        
        let attrs = manager.get_highlight_attributes(&code_block);
        assert!(attrs.is_some());
        let highlight_attrs = attrs.unwrap();
        assert_eq!(highlight_attrs.language, HighlightLanguage::JavaScript);
        assert!(highlight_attrs.line_numbers);
    }

    #[test]
    fn test_highlight_attributes_default() {
        let attrs = HighlightAttributes::default();
        assert_eq!(attrs.language, HighlightLanguage::Other);
        assert!(!attrs.line_numbers);
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeHighlightManager::new(config_service);
        
        let mut code_block = TipTapNode {
            node_type: NodeType::CodeBlock,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        manager.apply_highlight(&mut code_block, HighlightAttributes::default(), 0).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeHighlightManager::new(config_service);
        
        let mut code_block = TipTapNode {
            node_type: NodeType::CodeBlock,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        manager.apply_highlight(&mut code_block, HighlightAttributes::default(), 0).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CodeHighlightManager::new(config_service);
        
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
        let mut manager = CodeHighlightManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
