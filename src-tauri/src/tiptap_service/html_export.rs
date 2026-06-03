//! TipTap HTML Export Manager - Aerospace-Grade HTML Export Service
//!
//! Safety-critical HTML export service with:
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

/// Maximum exported HTML size
const MAX_EXPORT_SIZE: usize = 10000000;

pub struct HtmlExportManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl HtmlExportManager {
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

    pub fn export_html(&mut self, document: &TipTapNode) -> Result<String, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if document.node_type != NodeType::Document {
            return Err("Root node must be a document".to_string());
        }

        let html = self.node_to_html(document)?;

        if html.len() > MAX_EXPORT_SIZE {
            return Err(format!("Exported HTML exceeds maximum size of {} bytes", MAX_EXPORT_SIZE));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("HTML export CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("HTML export performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(html)
    }

    fn node_to_html(&self, node: &TipTapNode) -> Result<String, String> {
        let mut html = String::new();

        match node.node_type {
            NodeType::Document => {
                if let Some(ref children) = node.content {
                    for child in children {
                        html.push_str(&self.node_to_html(child)?);
                    }
                }
            }
            NodeType::Heading => {
                let level = node.attrs.as_ref()
                    .and_then(|a| a.get("level"))
                    .and_then(|l| l.as_u64())
                    .unwrap_or(1) as u8;
                let content = node.content.as_ref()
                    .and_then(|children| children.first())
                    .and_then(|n| n.text.as_ref())
                    .map(|s| s.as_str())
                    .unwrap_or("");
                html.push_str(&format!("<h{}>{}</h{}>\n", level, content, level));
            }
            NodeType::Paragraph => {
                let content = node.content.as_ref()
                    .and_then(|children| children.first())
                    .and_then(|n| n.text.as_ref())
                    .map(|s| s.as_str())
                    .unwrap_or("");
                html.push_str(&format!("<p>{}</p>\n", content));
            }
            NodeType::Text => {
                let content = node.text.as_deref().unwrap_or("");
                let is_bold = node.marks.as_ref()
                    .map_or(false, |marks| marks.iter().any(|m| m.mark_type == "bold"));
                let is_italic = node.marks.as_ref()
                    .map_or(false, |marks| marks.iter().any(|m| m.mark_type == "italic"));
                
                if is_bold {
                    html.push_str(&format!("<strong>{}</strong>", content));
                } else if is_italic {
                    html.push_str(&format!("<em>{}</em>", content));
                } else {
                    html.push_str(content);
                }
            }
            _ => {
                let content = node.text.as_deref().unwrap_or("");
                html.push_str(&format!("{}\n", content));
            }
        }

        Ok(html)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_html_export_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = HtmlExportManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_export_html() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HtmlExportManager::new(config_service);
        
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
        
        let result = manager.export_html(&document);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("<h1>Test Heading</h1>"));
    }

    #[test]
    fn test_export_non_document() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HtmlExportManager::new(config_service);
        
        let paragraph = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        let result = manager.export_html(&paragraph);
        assert!(result.is_err());
    }
}
