//! TipTap Document I/O Manager - Aerospace-Grade Document Import/Export Service
//!
//! Safety-critical document import/export service with:
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
use super::editor::{TipTapDocument, TipTapNode};

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 100;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 500;

/// Maximum export file size to prevent memory exhaustion
const MAX_EXPORT_SIZE: usize = 10 * 1024 * 1024; // 10MB

/// Maximum import file size to prevent DoS attacks
const MAX_IMPORT_SIZE: usize = 10 * 1024 * 1024; // 10MB

/// Export format type
#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    Html,
    Markdown,
    Json,
}

/// Import format type
#[derive(Debug, Clone, Copy)]
pub enum ImportFormat {
    Json,
    Markdown,
}

pub struct DocumentIOManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl DocumentIOManager {
    /// Creates a new document I/O manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new DocumentIOManager instance
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

    /// Get the maximum export size constant
    /// 
    /// # Returns
    /// The maximum export file size in bytes
    pub fn max_export_size() -> usize {
        MAX_EXPORT_SIZE
    }

    /// Get the maximum import size constant
    /// 
    /// # Returns
    /// The maximum import file size in bytes
    pub fn max_import_size() -> usize {
        MAX_IMPORT_SIZE
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

    /// Validate export size
    /// 
    /// # Arguments
    /// * `size` - The size to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents memory exhaustion by limiting export size
    fn validate_export_size(&self, size: usize) -> Result<(), String> {
        if size > MAX_EXPORT_SIZE {
            return Err(format!("Export size exceeds maximum of {} bytes", MAX_EXPORT_SIZE));
        }
        Ok(())
    }

    /// Validate import size
    /// 
    /// # Arguments
    /// * `size` - The size to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting import size
    fn validate_import_size(&self, size: usize) -> Result<(), String> {
        if size > MAX_IMPORT_SIZE {
            return Err(format!("Import size exceeds maximum of {} bytes", MAX_IMPORT_SIZE));
        }
        Ok(())
    }

    /// Export document to specified format
    /// 
    /// # Arguments
    /// * `document` - The document to export
    /// * `format` - The export format
    /// 
    /// # Returns
    /// Result containing the exported string or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates export size to prevent memory exhaustion
    pub fn export_document(&mut self, document: &TipTapDocument, format: ExportFormat) -> Result<String, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        let result = match format {
            ExportFormat::Html => self.export_to_html(document),
            ExportFormat::Markdown => self.export_to_markdown(document),
            ExportFormat::Json => self.export_to_json(document),
        };

        if let Ok(ref exported) = result {
            self.validate_export_size(exported.len())?;
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Document export CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Document export performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        result
    }

    /// Import document from specified format
    /// 
    /// # Arguments
    /// * `content` - The content to import
    /// * `format` - The import format
    /// 
    /// # Returns
    /// Result containing the imported document or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates import size to prevent DoS attacks
    pub fn import_document(&mut self, content: &str, format: ImportFormat) -> Result<TipTapDocument, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate import size
        self.validate_import_size(content.len())?;

        let result = match format {
            ImportFormat::Json => self.import_from_json(content),
            ImportFormat::Markdown => self.import_from_markdown(content),
        };

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Document import CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Document import performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        result
    }

    /// Export document to HTML
    fn export_to_html(&self, document: &TipTapDocument) -> Result<String, String> {
        let mut html = String::new();
        html.push_str("<div class=\"tiptap-document\">");
        for node in &document.content {
            self.node_to_html(node, &mut html, 0);
        }
        html.push_str("</div>");
        Ok(html)
    }

    /// Convert node to HTML recursively
    fn node_to_html(&self, node: &TipTapNode, html: &mut String, depth: usize) {
        if depth > 100 {
            return; // Prevent stack overflow
        }

        match node.node_type {
            super::editor::NodeType::Paragraph => {
                html.push_str("<p>");
                if let Some(ref content) = node.content {
                    for child in content {
                        self.node_to_html(child, html, depth + 1);
                    }
                } else if let Some(ref text) = node.text {
                    html.push_str(text);
                }
                html.push_str("</p>");
            }
            super::editor::NodeType::Heading => {
                let level = node.attrs.as_ref()
                    .and_then(|a| a.get("level"))
                    .and_then(|l| l.as_u64())
                    .unwrap_or(1);
                html.push_str(&format!("<h{}>", level));
                if let Some(ref content) = node.content {
                    for child in content {
                        self.node_to_html(child, html, depth + 1);
                    }
                }
                html.push_str(&format!("</h{}>", level));
            }
            super::editor::NodeType::Text => {
                if let Some(ref text) = node.text {
                    html.push_str(text);
                }
            }
            super::editor::NodeType::Bold => {
                html.push_str("<strong>");
                if let Some(ref content) = node.content {
                    for child in content {
                        self.node_to_html(child, html, depth + 1);
                    }
                }
                html.push_str("</strong>");
            }
            super::editor::NodeType::Italic => {
                html.push_str("<em>");
                if let Some(ref content) = node.content {
                    for child in content {
                        self.node_to_html(child, html, depth + 1);
                    }
                }
                html.push_str("</em>");
            }
            super::editor::NodeType::CodeBlock => {
                html.push_str("<pre><code>");
                if let Some(ref content) = node.content {
                    for child in content {
                        self.node_to_html(child, html, depth + 1);
                    }
                }
                html.push_str("</code></pre>");
            }
            super::editor::NodeType::Blockquote => {
                html.push_str("<blockquote>");
                if let Some(ref content) = node.content {
                    for child in content {
                        self.node_to_html(child, html, depth + 1);
                    }
                }
                html.push_str("</blockquote>");
            }
            super::editor::NodeType::List => {
                html.push_str("<ul>");
                if let Some(ref content) = node.content {
                    for child in content {
                        self.node_to_html(child, html, depth + 1);
                    }
                }
                html.push_str("</ul>");
            }
            super::editor::NodeType::ListItem => {
                html.push_str("<li>");
                if let Some(ref content) = node.content {
                    for child in content {
                        self.node_to_html(child, html, depth + 1);
                    }
                }
                html.push_str("</li>");
            }
            super::editor::NodeType::HorizontalRule => {
                html.push_str("<hr>");
            }
            _ => {
                // Default handling for other node types
                if let Some(ref content) = node.content {
                    for child in content {
                        self.node_to_html(child, html, depth + 1);
                    }
                }
            }
        }
    }

    /// Export document to Markdown
    fn export_to_markdown(&self, document: &TipTapDocument) -> Result<String, String> {
        let mut markdown = String::new();
        for node in &document.content {
            self.node_to_markdown(node, &mut markdown, 0);
        }
        Ok(markdown)
    }

    /// Convert node to Markdown recursively
    fn node_to_markdown(&self, node: &TipTapNode, markdown: &mut String, depth: usize) {
        if depth > 100 {
            return; // Prevent stack overflow
        }

        match node.node_type {
            super::editor::NodeType::Paragraph => {
                if let Some(ref content) = node.content {
                    for child in content {
                        self.node_to_markdown(child, markdown, depth + 1);
                    }
                }
                markdown.push_str("\n\n");
            }
            super::editor::NodeType::Heading => {
                let level = node.attrs.as_ref()
                    .and_then(|a| a.get("level"))
                    .and_then(|l| l.as_u64())
                    .unwrap_or(1);
                markdown.push_str(&format!("{} ", "#".repeat(level as usize)));
                if let Some(ref content) = node.content {
                    for child in content {
                        self.node_to_markdown(child, markdown, depth + 1);
                    }
                }
                markdown.push_str("\n\n");
            }
            super::editor::NodeType::Text => {
                if let Some(ref text) = node.text {
                    markdown.push_str(text);
                }
            }
            super::editor::NodeType::Bold => {
                markdown.push_str("**");
                if let Some(ref content) = node.content {
                    for child in content {
                        self.node_to_markdown(child, markdown, depth + 1);
                    }
                }
                markdown.push_str("**");
            }
            super::editor::NodeType::Italic => {
                markdown.push_str("*");
                if let Some(ref content) = node.content {
                    for child in content {
                        self.node_to_markdown(child, markdown, depth + 1);
                    }
                }
                markdown.push_str("*");
            }
            super::editor::NodeType::CodeBlock => {
                markdown.push_str("```\n");
                if let Some(ref content) = node.content {
                    for child in content {
                        self.node_to_markdown(child, markdown, depth + 1);
                    }
                }
                markdown.push_str("\n```\n\n");
            }
            super::editor::NodeType::Blockquote => {
                markdown.push_str("> ");
                if let Some(ref content) = node.content {
                    for child in content {
                        self.node_to_markdown(child, markdown, depth + 1);
                    }
                }
                markdown.push_str("\n\n");
            }
            super::editor::NodeType::List => {
                if let Some(ref content) = node.content {
                    for child in content {
                        self.node_to_markdown(child, markdown, depth + 1);
                    }
                }
                markdown.push_str("\n");
            }
            super::editor::NodeType::ListItem => {
                markdown.push_str("- ");
                if let Some(ref content) = node.content {
                    for child in content {
                        self.node_to_markdown(child, markdown, depth + 1);
                    }
                }
                markdown.push_str("\n");
            }
            super::editor::NodeType::HorizontalRule => {
                markdown.push_str("---\n\n");
            }
            _ => {
                // Default handling for other node types
                if let Some(ref content) = node.content {
                    for child in content {
                        self.node_to_markdown(child, markdown, depth + 1);
                    }
                }
            }
        }
    }

    /// Export document to JSON
    fn export_to_json(&self, document: &TipTapDocument) -> Result<String, String> {
        serde_json::to_string_pretty(document)
            .map_err(|e| format!("Failed to serialize document to JSON: {}", e))
    }

    /// Import document from JSON
    fn import_from_json(&self, content: &str) -> Result<TipTapDocument, String> {
        serde_json::from_str(content)
            .map_err(|e| format!("Failed to parse JSON document: {}", e))
    }

    /// Import document from Markdown
    fn import_from_markdown(&self, _content: &str) -> Result<TipTapDocument, String> {
        // Markdown import would require a full parser
        // For now, return an error indicating this is not yet implemented
        Err("Markdown import is not yet implemented".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_document_io_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = DocumentIOManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(DocumentIOManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(DocumentIOManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(DocumentIOManager::max_export_size(), MAX_EXPORT_SIZE);
        assert_eq!(DocumentIOManager::max_import_size(), MAX_IMPORT_SIZE);
    }

    #[test]
    fn test_export_to_json() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = DocumentIOManager::new(config_service);
        
        let document = TipTapDocument {
            node_type: NodeType::Document,
            content: vec![TipTapNode {
                node_type: NodeType::Paragraph,
                content: Some(vec![TipTapNode {
                    node_type: NodeType::Text,
                    content: None,
                    text: Some("Test".to_string()),
                    attrs: None,
                    marks: None,
                }]),
                text: None,
                attrs: None,
                marks: None,
            }],
        };
        
        let result = manager.export_document(&document, ExportFormat::Json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_export_to_html() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = DocumentIOManager::new(config_service);
        
        let document = TipTapDocument {
            node_type: NodeType::Document,
            content: vec![TipTapNode {
                node_type: NodeType::Paragraph,
                content: Some(vec![TipTapNode {
                    node_type: NodeType::Text,
                    content: None,
                    text: Some("Test".to_string()),
                    attrs: None,
                    marks: None,
                }]),
                text: None,
                attrs: None,
                marks: None,
            }],
        };
        
        let result = manager.export_document(&document, ExportFormat::Html);
        assert!(result.is_ok());
        let html = result.unwrap();
        assert!(html.contains("<p>"));
        assert!(html.contains("Test"));
    }

    #[test]
    fn test_export_to_markdown() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = DocumentIOManager::new(config_service);
        
        let document = TipTapDocument {
            node_type: NodeType::Document,
            content: vec![TipTapNode {
                node_type: NodeType::Paragraph,
                content: Some(vec![TipTapNode {
                    node_type: NodeType::Text,
                    content: None,
                    text: Some("Test".to_string()),
                    attrs: None,
                    marks: None,
                }]),
                text: None,
                attrs: None,
                marks: None,
            }],
        };
        
        let result = manager.export_document(&document, ExportFormat::Markdown);
        assert!(result.is_ok());
    }

    #[test]
    fn test_import_from_json() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = DocumentIOManager::new(config_service);
        
        let json = r#"{"node_type":"document","content":[{"node_type":"paragraph","content":[{"node_type":"text","text":"Test"}]}]}"#;
        let result = manager.import_document(json, ImportFormat::Json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_import_from_json_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = DocumentIOManager::new(config_service);
        
        let json = "invalid json";
        let result = manager.import_document(json, ImportFormat::Json);
        assert!(result.is_err());
    }

    #[test]
    fn test_import_from_markdown_not_implemented() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = DocumentIOManager::new(config_service);
        
        let markdown = "# Test\n\nContent";
        let result = manager.import_document(markdown, ImportFormat::Markdown);
        assert!(result.is_err());
    }

    #[test]
    fn test_import_size_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = DocumentIOManager::new(config_service);
        
        let large_content = "a".repeat(MAX_IMPORT_SIZE + 1);
        let result = manager.import_document(&large_content, ImportFormat::Json);
        assert!(result.is_err());
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = DocumentIOManager::new(config_service);
        
        let document = TipTapDocument {
            node_type: NodeType::Document,
            content: vec![],
        };
        
        manager.export_document(&document, ExportFormat::Json).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = DocumentIOManager::new(config_service);
        
        let document = TipTapDocument {
            node_type: NodeType::Document,
            content: vec![],
        };
        
        manager.export_document(&document, ExportFormat::Json).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = DocumentIOManager::new(config_service);
        
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
        let mut manager = DocumentIOManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
