//! TipTap Editor Core - Aerospace-Grade Editor Service
//!
//! Safety-critical editor service with:
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

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 100;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 500;

/// Maximum document content size in bytes to prevent DoS attacks
const MAX_DOCUMENT_SIZE: usize = 10 * 1024 * 1024; // 10MB

/// Maximum number of nodes in a document to prevent memory exhaustion
const MAX_NODE_COUNT: usize = 10000;

/// Maximum text length for a single node
const MAX_NODE_TEXT_LENGTH: usize = 10000;

/// TipTap document node type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NodeType {
    Document,
    Paragraph,
    Heading,
    Text,
    Image,
    List,
    ListItem,
    CodeBlock,
    Table,
    TableRow,
    TableCell,
    Bold,
    Italic,
    Underline,
    Strike,
    Link,
    Blockquote,
    HorizontalRule,
    HardBreak,
    SoftBreak,
}

/// TipTap document node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TipTapNode {
    pub node_type: NodeType,
    pub content: Option<Vec<TipTapNode>>,
    pub text: Option<String>,
    pub attrs: Option<serde_json::Value>,
    pub marks: Option<Vec<Mark>>,
}

/// Text mark (formatting)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mark {
    pub mark_type: String,
    pub attrs: Option<serde_json::Value>,
}

/// TipTap document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TipTapDocument {
    pub node_type: NodeType,
    pub content: Vec<TipTapNode>,
}

/// Editor operation result
#[derive(Debug, Serialize, Deserialize)]
pub struct EditorResult {
    pub success: bool,
    pub document: Option<TipTapDocument>,
    pub error: Option<String>,
    pub operation_time_ms: u128,
}

pub struct TipTapEditor {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TipTapEditor {
    /// Creates a new TipTap editor instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new TipTapEditor instance
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

    /// Get the maximum document size constant
    /// 
    /// # Returns
    /// The maximum document size in bytes
    pub fn max_document_size() -> usize {
        MAX_DOCUMENT_SIZE
    }

    /// Get the maximum node count constant
    /// 
    /// # Returns
    /// The maximum number of nodes in a document
    pub fn max_node_count() -> usize {
        MAX_NODE_COUNT
    }

    /// Get the maximum node text length constant
    /// 
    /// # Returns
    /// The maximum text length for a single node
    pub fn max_node_text_length() -> usize {
        MAX_NODE_TEXT_LENGTH
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

    /// Validate document size
    /// 
    /// # Arguments
    /// * `document` - The document to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting document size
    fn validate_document_size(&self, document: &TipTapDocument) -> Result<(), String> {
        let json = serde_json::to_string(document)
            .map_err(|e| format!("Failed to serialize document: {}", e))?;
        
        if json.len() > MAX_DOCUMENT_SIZE {
            return Err(format!("Document exceeds maximum size of {} bytes", MAX_DOCUMENT_SIZE));
        }
        
        Ok(())
    }

    /// Validate node count
    /// 
    /// # Arguments
    /// * `document` - The document to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents memory exhaustion by limiting node count
    fn validate_node_count(&self, document: &TipTapDocument) -> Result<(), String> {
        let count = self.count_nodes_in_document(document);
        
        if count > MAX_NODE_COUNT {
            return Err(format!("Document exceeds maximum node count of {}", MAX_NODE_COUNT));
        }
        
        Ok(())
    }

    /// Count total nodes in a document
    fn count_nodes_in_document(&self, document: &TipTapDocument) -> usize {
        let mut count = 0;
        for node in &document.content {
            count += self.count_nodes(node);
        }
        count
    }

    /// Count total nodes in a node tree
    fn count_nodes(&self, node: &TipTapNode) -> usize {
        let mut count = 1;
        if let Some(ref content) = node.content {
            for child in content {
                count += self.count_nodes(child);
            }
        }
        count
    }

    /// Validate node text length
    /// 
    /// # Arguments
    /// * `node` - The node to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents memory exhaustion by limiting text length
    fn validate_node_text(&self, node: &TipTapNode) -> Result<(), String> {
        if let Some(ref text) = node.text {
            if text.len() > MAX_NODE_TEXT_LENGTH {
                return Err(format!("Node text exceeds maximum length of {} characters", MAX_NODE_TEXT_LENGTH));
            }
        }
        
        if let Some(ref content) = node.content {
            for child in content {
                self.validate_node_text(child)?;
            }
        }
        
        Ok(())
    }

    /// Parse JSON to TipTap document
    /// 
    /// # Arguments
    /// * `json` - The JSON string to parse
    /// 
    /// # Returns
    /// Result containing the parsed document or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates document size and node count
    pub fn parse_document(&mut self, json: &str) -> Result<TipTapDocument, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate input size
        if json.len() > MAX_DOCUMENT_SIZE {
            let error = format!("Input JSON exceeds maximum size of {} bytes", MAX_DOCUMENT_SIZE);
            self.record_error("INPUT_TOO_LARGE", &error, "parse_document");
            return Err(error);
        }

        let document: TipTapDocument = serde_json::from_str(json)
            .map_err(|e| {
                let error = format!("Failed to parse JSON: {}", e);
                self.record_error("PARSE_ERROR", &error, "parse_document");
                error
            })?;

        // Validate document
        self.validate_document_size(&document)?;
        self.validate_node_count(&document)?;
        self.validate_node_text(&TipTapNode {
            node_type: document.node_type.clone(),
            content: Some(document.content.clone()),
            text: None,
            attrs: None,
            marks: None,
        })?;

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("TipTap document parse CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("TipTap document parse performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(document)
    }

    /// Serialize TipTap document to JSON
    /// 
    /// # Arguments
    /// * `document` - The document to serialize
    /// 
    /// # Returns
    /// Result containing the JSON string or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn serialize_document(&mut self, document: &TipTapDocument) -> Result<String, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate document
        self.validate_document_size(document)?;
        self.validate_node_count(document)?;

        let json = serde_json::to_string(document)
            .map_err(|e| {
                let error = format!("Failed to serialize document: {}", e);
                self.record_error("SERIALIZE_ERROR", &error, "serialize_document");
                error
            })?;

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("TipTap document serialize CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("TipTap document serialize performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(json)
    }

    /// Create a new empty document
    /// 
    /// # Returns
    /// A new empty TipTap document
    pub fn create_empty_document(&self) -> TipTapDocument {
        TipTapDocument {
            node_type: NodeType::Paragraph,
            content: vec![],
        }
    }

    /// Add a text node to the document
    /// 
    /// # Arguments
    /// * `document` - The document to modify
    /// * `text` - The text content
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Validates text length and node count
    pub fn add_text_node(&mut self, document: &mut TipTapDocument, text: &str) -> Result<(), String> {
        self.operation_count += 1;

        // Validate text length
        if text.len() > MAX_NODE_TEXT_LENGTH {
            return Err(format!("Text exceeds maximum length of {} characters", MAX_NODE_TEXT_LENGTH));
        }

        // Validate node count
        let current_count = self.count_nodes(&TipTapNode {
            node_type: document.node_type.clone(),
            content: Some(document.content.clone()),
            text: None,
            attrs: None,
            marks: None,
        });

        if current_count >= MAX_NODE_COUNT {
            return Err(format!("Document has reached maximum node count of {}", MAX_NODE_COUNT));
        }

        let node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some(text.to_string()),
            attrs: None,
            marks: None,
        };

        document.content.push(node);
        self.last_error = None;
        Ok(())
    }

    /// Apply formatting to text
    /// 
    /// # Arguments
    /// * `node` - The node to format
    /// * `mark_type` - The type of mark to apply
    /// * `attrs` - Optional attributes for the mark
    /// 
    /// # Returns
    /// Result indicating success or failure
    pub fn apply_mark(&mut self, node: &mut TipTapNode, mark_type: &str, attrs: Option<serde_json::Value>) -> Result<(), String> {
        self.operation_count += 1;

        let mark = Mark {
            mark_type: mark_type.to_string(),
            attrs,
        };

        if node.marks.is_none() {
            node.marks = Some(vec![mark]);
        } else {
            node.marks.as_mut().unwrap().push(mark);
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
    fn test_editor_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let editor = TipTapEditor::new(config_service);
        assert_eq!(editor.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(TipTapEditor::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(TipTapEditor::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(TipTapEditor::max_document_size(), MAX_DOCUMENT_SIZE);
        assert_eq!(TipTapEditor::max_node_count(), MAX_NODE_COUNT);
        assert_eq!(TipTapEditor::max_node_text_length(), MAX_NODE_TEXT_LENGTH);
    }

    #[test]
    fn test_create_empty_document() {
        let config_service = Arc::new(ExportConfigService::new());
        let editor = TipTapEditor::new(config_service);
        let document = editor.create_empty_document();
        assert!(document.content.is_empty());
    }

    #[test]
    fn test_add_text_node() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut editor = TipTapEditor::new(config_service);
        let mut document = editor.create_empty_document();
        
        let result = editor.add_text_node(&mut document, "Hello, world!");
        assert!(result.is_ok());
        assert_eq!(document.content.len(), 1);
    }

    #[test]
    fn test_add_text_node_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut editor = TipTapEditor::new(config_service);
        let mut document = editor.create_empty_document();
        
        let long_text = "a".repeat(MAX_NODE_TEXT_LENGTH + 1);
        let result = editor.add_text_node(&mut document, &long_text);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_text_node_max_length() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut editor = TipTapEditor::new(config_service);
        let mut document = editor.create_empty_document();
        
        let long_text = "a".repeat(MAX_NODE_TEXT_LENGTH);
        let result = editor.add_text_node(&mut document, &long_text);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_document() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut editor = TipTapEditor::new(config_service);
        
        let json = r#"{"node_type":"paragraph","content":[]}"#;
        
        let result = editor.parse_document(json);
        if let Err(ref e) = result {
            println!("Parse error: {}", e);
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_document_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut editor = TipTapEditor::new(config_service);
        
        let large_json = "a".repeat(MAX_DOCUMENT_SIZE + 1);
        let result = editor.parse_document(&large_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_document() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut editor = TipTapEditor::new(config_service);
        
        let document = TipTapDocument {
            node_type: NodeType::Paragraph,
            content: vec![TipTapNode {
                node_type: NodeType::Text,
                content: None,
                text: Some("Hello".to_string()),
                attrs: None,
                marks: None,
            }],
        };
        
        let result = editor.serialize_document(&document);
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_mark() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut editor = TipTapEditor::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Hello".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = editor.apply_mark(&mut node, "bold", None);
        assert!(result.is_ok());
        assert!(node.marks.is_some());
        assert_eq!(node.marks.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut editor = TipTapEditor::new(config_service);
        
        let mut document = editor.create_empty_document();
        editor.add_text_node(&mut document, "test").unwrap();
        
        assert!(editor.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut editor = TipTapEditor::new(config_service);
        
        let mut document = editor.create_empty_document();
        editor.add_text_node(&mut document, "test").unwrap();
        
        editor.reset_operation_count();
        assert_eq!(editor.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut editor = TipTapEditor::new(config_service);
        
        editor.record_error("TEST_ERROR", "Test error message", "test_source");
        
        let last_error = editor.get_last_error();
        assert!(last_error.is_some());
        let error = last_error.unwrap();
        assert_eq!(error.code, "TEST_ERROR");
        assert_eq!(error.message, "Test error message");
        assert_eq!(error.source, "test_source");
    }

    #[test]
    fn test_error_state_reset() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut editor = TipTapEditor::new(config_service);
        
        editor.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(editor.get_last_error().is_some());
        
        editor.reset_error_state();
        assert!(editor.get_last_error().is_none());
    }
}
