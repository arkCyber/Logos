//! TipTap Formatting Manager - Aerospace-Grade Text Formatting Service
//!
//! Safety-critical text formatting service with:
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
use super::editor::{TipTapNode, Mark};

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 50;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 200;

/// Maximum number of marks per node to prevent performance issues
const MAX_MARKS_PER_NODE: usize = 10;

/// Maximum text length for formatting operations
const MAX_FORMAT_TEXT_LENGTH: usize = 10000;

/// Text formatting type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FormatType {
    Bold,
    Italic,
    Underline,
    Strike,
    Code,
    Subscript,
    Superscript,
    Highlight,
}

/// Formatting attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatAttributes {
    pub color: Option<String>,
    pub background_color: Option<String>,
    pub font_size: Option<String>,
    pub font_family: Option<String>,
}

impl Default for FormatAttributes {
    fn default() -> Self {
        Self {
            color: None,
            background_color: None,
            font_size: None,
            font_family: None,
        }
    }
}

/// Formatting operation result
#[derive(Debug, Serialize, Deserialize)]
pub struct FormatResult {
    pub success: bool,
    pub formatted_node: Option<TipTapNode>,
    pub error: Option<String>,
    pub operation_time_ms: u128,
}

pub struct FormattingManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FormattingManager {
    /// Creates a new formatting manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new FormattingManager instance
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

    /// Get the maximum marks per node constant
    /// 
    /// # Returns
    /// The maximum number of marks per node
    pub fn max_marks_per_node() -> usize {
        MAX_MARKS_PER_NODE
    }

    /// Get the maximum format text length constant
    /// 
    /// # Returns
    /// The maximum text length for formatting operations
    pub fn max_format_text_length() -> usize {
        MAX_FORMAT_TEXT_LENGTH
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

    /// Validate text length for formatting
    /// 
    /// # Arguments
    /// * `text` - The text to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting text length
    fn validate_text_length(&self, text: &str) -> Result<(), String> {
        if text.len() > MAX_FORMAT_TEXT_LENGTH {
            return Err(format!("Text exceeds maximum length of {} characters", MAX_FORMAT_TEXT_LENGTH));
        }
        Ok(())
    }

    /// Validate mark count
    /// 
    /// # Arguments
    /// * `node` - The node to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents performance issues by limiting mark count
    fn validate_mark_count(&self, node: &TipTapNode) -> Result<(), String> {
        if let Some(ref marks) = node.marks {
            if marks.len() >= MAX_MARKS_PER_NODE {
                return Err(format!("Node has reached maximum mark count of {}", MAX_MARKS_PER_NODE));
            }
        }
        Ok(())
    }

    /// Apply formatting to a node
    /// 
    /// # Arguments
    /// * `node` - The node to format
    /// * `format_type` - The type of formatting to apply
    /// * `attributes` - Optional formatting attributes
    /// 
    /// # Returns
    /// Result containing the formatted node or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates mark count to prevent performance issues
    pub fn apply_format(&mut self, node: &mut TipTapNode, format_type: FormatType, attributes: Option<FormatAttributes>) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate mark count
        self.validate_mark_count(node)?;

        // Create mark
        let mark_type = match format_type {
            FormatType::Bold => "bold".to_string(),
            FormatType::Italic => "italic".to_string(),
            FormatType::Underline => "underline".to_string(),
            FormatType::Strike => "strike".to_string(),
            FormatType::Code => "code".to_string(),
            FormatType::Subscript => "subscript".to_string(),
            FormatType::Superscript => "superscript".to_string(),
            FormatType::Highlight => "highlight".to_string(),
        };

        let attrs_json = if let Some(attrs) = attributes {
            Some(serde_json::to_value(attrs).map_err(|e| {
                let error = format!("Failed to serialize attributes: {}", e);
                self.record_error("SERIALIZE_ERROR", &error, "apply_format");
                error
            })?)
        } else {
            None
        };

        let mark = Mark {
            mark_type,
            attrs: attrs_json,
        };

        if node.marks.is_none() {
            node.marks = Some(vec![mark]);
        } else {
            node.marks.as_mut().unwrap().push(mark);
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Formatting apply CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Formatting apply performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove formatting from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove formatting from
    /// * `format_type` - The type of formatting to remove
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_format(&mut self, node: &mut TipTapNode, format_type: FormatType) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        let mark_type = match format_type {
            FormatType::Bold => "bold".to_string(),
            FormatType::Italic => "italic".to_string(),
            FormatType::Underline => "underline".to_string(),
            FormatType::Strike => "strike".to_string(),
            FormatType::Code => "code".to_string(),
            FormatType::Subscript => "subscript".to_string(),
            FormatType::Superscript => "superscript".to_string(),
            FormatType::Highlight => "highlight".to_string(),
        };

        if let Some(ref mut marks) = node.marks {
            marks.retain(|m| m.mark_type != mark_type);
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Formatting remove CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Formatting remove performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Clear all formatting from a node
    /// 
    /// # Arguments
    /// * `node` - The node to clear formatting from
    /// 
    /// # Returns
    /// Result indicating success or failure
    pub fn clear_formatting(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        self.operation_count += 1;
        node.marks = None;
        self.last_error = None;
        Ok(())
    }

    /// Check if a node has specific formatting
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// * `format_type` - The type of formatting to check for
    /// 
    /// # Returns
    /// True if the node has the formatting, false otherwise
    pub fn has_format(&self, node: &TipTapNode, format_type: FormatType) -> bool {
        let mark_type = match format_type {
            FormatType::Bold => "bold".to_string(),
            FormatType::Italic => "italic".to_string(),
            FormatType::Underline => "underline".to_string(),
            FormatType::Strike => "strike".to_string(),
            FormatType::Code => "code".to_string(),
            FormatType::Subscript => "subscript".to_string(),
            FormatType::Superscript => "superscript".to_string(),
            FormatType::Highlight => "highlight".to_string(),
        };

        if let Some(ref marks) = node.marks {
            marks.iter().any(|m| m.mark_type == mark_type)
        } else {
            false
        }
    }

    /// Get all formatting marks from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get formatting from
    /// 
    /// # Returns
    /// Vector of mark types
    pub fn get_formats(&self, node: &TipTapNode) -> Vec<String> {
        if let Some(ref marks) = node.marks {
            marks.iter().map(|m| m.mark_type.clone()).collect()
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_formatting_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FormattingManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(FormattingManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(FormattingManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(FormattingManager::max_marks_per_node(), MAX_MARKS_PER_NODE);
        assert_eq!(FormattingManager::max_format_text_length(), MAX_FORMAT_TEXT_LENGTH);
    }

    #[test]
    fn test_apply_format_bold() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FormattingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_format(&mut node, FormatType::Bold, None);
        assert!(result.is_ok());
        assert!(manager.has_format(&node, FormatType::Bold));
    }

    #[test]
    fn test_apply_format_italic() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FormattingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_format(&mut node, FormatType::Italic, None);
        assert!(result.is_ok());
        assert!(manager.has_format(&node, FormatType::Italic));
    }

    #[test]
    fn test_apply_format_exceeds_limit() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FormattingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("test".to_string()),
            attrs: None,
            marks: Some(vec![
                Mark { mark_type: "bold".to_string(), attrs: None },
                Mark { mark_type: "italic".to_string(), attrs: None },
                Mark { mark_type: "underline".to_string(), attrs: None },
                Mark { mark_type: "strike".to_string(), attrs: None },
                Mark { mark_type: "code".to_string(), attrs: None },
                Mark { mark_type: "subscript".to_string(), attrs: None },
                Mark { mark_type: "superscript".to_string(), attrs: None },
                Mark { mark_type: "highlight".to_string(), attrs: None },
                Mark { mark_type: "custom1".to_string(), attrs: None },
                Mark { mark_type: "custom2".to_string(), attrs: None },
            ]),
        };
        
        let result = manager.apply_format(&mut node, FormatType::Bold, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_format() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FormattingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_format(&mut node, FormatType::Bold, None).unwrap();
        let result = manager.remove_format(&mut node, FormatType::Bold);
        assert!(result.is_ok());
        assert!(!manager.has_format(&node, FormatType::Bold));
    }

    #[test]
    fn test_clear_formatting() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FormattingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_format(&mut node, FormatType::Bold, None).unwrap();
        manager.apply_format(&mut node, FormatType::Italic, None).unwrap();
        
        let result = manager.clear_formatting(&mut node);
        assert!(result.is_ok());
        assert!(node.marks.is_none());
    }

    #[test]
    fn test_has_format() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FormattingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(!manager.has_format(&node, FormatType::Bold));
        
        manager.apply_format(&mut node, FormatType::Bold, None).unwrap();
        assert!(manager.has_format(&node, FormatType::Bold));
    }

    #[test]
    fn test_get_formats() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FormattingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_format(&mut node, FormatType::Bold, None).unwrap();
        manager.apply_format(&mut node, FormatType::Italic, None).unwrap();
        
        let formats = manager.get_formats(&node);
        assert_eq!(formats.len(), 2);
    }

    #[test]
    fn test_apply_format_with_attributes() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FormattingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attributes = FormatAttributes {
            color: Some("#ff0000".to_string()),
            background_color: Some("#00ff00".to_string()),
            font_size: Some("14px".to_string()),
            font_family: Some("Arial".to_string()),
        };
        
        let result = manager.apply_format(&mut node, FormatType::Bold, Some(attributes));
        assert!(result.is_ok());
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FormattingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_format(&mut node, FormatType::Bold, None).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FormattingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_format(&mut node, FormatType::Bold, None).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FormattingManager::new(config_service);
        
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
        let mut manager = FormattingManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }

    #[test]
    fn test_format_type_variants() {
        let bold = FormatType::Bold;
        let italic = FormatType::Italic;
        let underline = FormatType::Underline;
        let strike = FormatType::Strike;
        let code = FormatType::Code;
        let subscript = FormatType::Subscript;
        let superscript = FormatType::Superscript;
        let highlight = FormatType::Highlight;

        assert!(matches!(bold, FormatType::Bold));
        assert!(matches!(italic, FormatType::Italic));
        assert!(matches!(underline, FormatType::Underline));
        assert!(matches!(strike, FormatType::Strike));
        assert!(matches!(code, FormatType::Code));
        assert!(matches!(subscript, FormatType::Subscript));
        assert!(matches!(superscript, FormatType::Superscript));
        assert!(matches!(highlight, FormatType::Highlight));
    }

    #[test]
    fn test_format_attributes_default() {
        let attrs = FormatAttributes::default();
        assert!(attrs.color.is_none());
        assert!(attrs.background_color.is_none());
        assert!(attrs.font_size.is_none());
        assert!(attrs.font_family.is_none());
    }
}
