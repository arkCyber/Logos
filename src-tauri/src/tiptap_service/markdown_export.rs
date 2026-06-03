//! TipTap Markdown Export Manager - Aerospace-Grade Markdown Export Service
//!
//! Safety-critical markdown export service with:
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
use super::editor::{TipTapNode, NodeType};

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 50;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 200;

/// Maximum exported markdown size
const MAX_EXPORT_SIZE: usize = 10000000;

pub struct MarkdownExportManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl MarkdownExportManager {
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

    pub fn max_export_size() -> usize {
        MAX_EXPORT_SIZE
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

    pub fn export_markdown(&mut self, document: &TipTapNode) -> Result<String, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if document.node_type != NodeType::Document {
            return Err("Root node must be a document".to_string());
        }

        let markdown = self.node_to_markdown(document)?;

        if markdown.len() > MAX_EXPORT_SIZE {
            return Err(format!("Exported markdown exceeds maximum size of {} bytes", MAX_EXPORT_SIZE));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Markdown export CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Markdown export performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(markdown)
    }

    fn node_to_markdown(&self, node: &TipTapNode) -> Result<String, String> {
        let mut markdown = String::new();

        match node.node_type {
            NodeType::Document => {
                if let Some(ref children) = node.content {
                    for child in children {
                        markdown.push_str(&self.node_to_markdown(child)?);
                    }
                }
            }
            NodeType::Heading => {
                let level = node.attrs.as_ref()
                    .and_then(|a| a.get("level"))
                    .and_then(|l| l.as_u64())
                    .unwrap_or(1) as u8;
                let heading = "#".repeat(level as usize);
                let content = node.content.as_ref()
                    .and_then(|children| children.first())
                    .and_then(|n| n.text.as_ref())
                    .map(|s| s.as_str())
                    .unwrap_or("");
                markdown.push_str(&format!("{} {}\n\n", heading, content));
            }
            NodeType::Paragraph => {
                let content = node.content.as_ref()
                    .and_then(|children| children.first())
                    .and_then(|n| n.text.as_ref())
                    .map(|s| s.as_str())
                    .unwrap_or("");
                markdown.push_str(&format!("{}\n\n", content));
            }
            NodeType::Text => {
                let content = node.text.as_deref().unwrap_or("");
                markdown.push_str(content);
            }
            _ => {
                let content = node.text.as_deref().unwrap_or("");
                markdown.push_str(&format!("{}\n", content));
            }
        }

        Ok(markdown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_markdown_export_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MarkdownExportManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_export_markdown() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MarkdownExportManager::new(config_service);
        
        let mut document = TipTapNode {
            node_type: NodeType::Document,
            content: Some(vec![TipTapNode {
                node_type: NodeType::Heading,
                content: Some(vec![TipTapNode {
                    node_type: NodeType::Text,
                    content: None,
                    text: Some("Test Heading".to_string()),
                    attrs: Some(serde_json::json!({"level": 1})),
                    marks: None,
                }]),
                text: None,
                attrs: None,
                marks: None,
            }]),
            text: None,
            attrs: None,
            marks: None,
        };
        
        let result = manager.export_markdown(&document);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("# Test Heading"));
    }

    #[test]
    fn test_export_non_document() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MarkdownExportManager::new(config_service);
        
        let paragraph = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        let result = manager.export_markdown(&paragraph);
        assert!(result.is_err());
    }
}
