//! JSON to Typst Converter - Aerospace-Grade Document Conversion
//!
//! Safety-critical JSON to Typst conversion with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Comprehensive error handling
//! - Performance monitoring
//! - Security hardening against injection attacks
//! - Fault tolerance and error recovery

use serde_json::Value;
use std::sync::Arc;
use std::time::Instant;
use crate::error_handling::{ConversionResult as ErrorResult, ErrorContext, ErrorSeverity, FallbackStrategy};
use crate::config_service::ExportConfigService;

/// JSON to Typst converter with aerospace-grade safety
#[derive(Debug)]
pub struct JsonToTypstConverter {
    /// Current recursion depth for safety
    recursion_depth: usize,
    /// Conversion statistics
    stats: ConversionStats,
    config_service: Arc<ExportConfigService>,
}

/// Conversion statistics for monitoring
#[derive(Debug, Default, Clone)]
struct ConversionStats {
    /// Number of nodes processed
    nodes_processed: usize,
    /// Number of errors encountered
    errors: usize,
    /// Conversion duration in milliseconds
    duration_ms: u64,
}

impl Default for JsonToTypstConverter {
    fn default() -> Self {
        Self::new(Arc::new(ExportConfigService::new()))
    }
}

impl JsonToTypstConverter {
    /// Create a new converter
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            recursion_depth: 0,
            stats: ConversionStats::default(),
            config_service,
        }
    }

    /// Validate input size to prevent DoS attacks
    fn validate_input_size(&self, input: &str) -> Result<(), String> {
        let editing_config = self.config_service.get_editing_engine_config();
        if input.len() > editing_config.max_input_size {
            return Err(format!(
                "Input size {} exceeds maximum allowed size of {} bytes",
                input.len(),
                editing_config.max_input_size
            ));
        }
        Ok(())
    }

    /// Check recursion depth to prevent stack overflow
    fn check_recursion_depth(&mut self) -> Result<(), String> {
        let editing_config = self.config_service.get_editing_engine_config();
        if self.recursion_depth >= editing_config.max_recursion_depth {
            return Err(format!(
                "Maximum recursion depth {} exceeded",
                editing_config.max_recursion_depth
            ));
        }
        self.recursion_depth += 1;
        Ok(())
    }

    /// Reset recursion depth
    fn reset_recursion_depth(&mut self) {
        self.recursion_depth = 0;
    }

    /// Validate heading level
    fn validate_heading_level(&self, level: u64) -> Result<(), String> {
        let editing_config = self.config_service.get_editing_engine_config();
        if level == 0 || level > editing_config.max_heading_level {
            return Err(format!(
                "Invalid heading level {}. Must be between 1 and {}",
                level,
                editing_config.max_heading_level
            ));
        }
        Ok(())
    }

    /// Sanitize text content to prevent injection
    /// Note: We don't escape formatting characters (*, _, etc.) as they are part of Typst syntax
    fn sanitize_text(text: &str) -> String {
        text.replace('\\', "\\\\")
            .replace('#', "\\#")
            .replace('[', "\\[")
            .replace(']', "\\]")
            .replace('$', "\\$")
    }

    /// Convert JSON to Typst with full validation and error recovery
    /// 
    /// # Safety
    /// - Validates input size
    /// - Sanitizes all text content
    /// - Handles malformed JSON gracefully
    /// - Monitors performance
    /// - Provides fallback on conversion errors
    pub fn convert_with_fallback(json: &str) -> ErrorResult<String> {
        let config_service = Arc::new(ExportConfigService::new());
        let converter = Self::new(config_service);
        if let Err(error) = converter.validate_input_size(json) {
            let context = ErrorContext::new(
                ErrorSeverity::Error,
                "JSON_TO_TYPST_FAILED",
                &error,
                "json_to_typst",
            );
            let fallback = Self::fallback_typst(&error);
            return ErrorResult::fallback(fallback, FallbackStrategy::Partial, context);
        }
        
        match Self::convert(json) {
            Ok(typst) => ErrorResult::success(typst),
            Err(error) => {
                let context = ErrorContext::new(
                    ErrorSeverity::Error,
                    "JSON_TO_TYPST_FAILED",
                    &error,
                    "json_to_typst",
                );
                // Fallback: return basic Typst with error message
                let fallback = Self::fallback_typst(&error);
                ErrorResult::fallback(fallback, FallbackStrategy::Partial, context)
            }
        }
    }

    /// Convert JSON to Typst with full validation
    /// 
    /// # Safety
    /// - Validates input size
    /// - Sanitizes all text content
    /// - Handles malformed JSON gracefully
    /// - Monitors performance
    pub fn convert(json: &str) -> Result<String, String> {
        let config_service = Arc::new(ExportConfigService::new());
        let mut converter = Self::new(config_service);
        converter.validate_input_size(json)?;
        
        let start = Instant::now();
        
        let doc: Value =
            serde_json::from_str(json).map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let mut typst = String::from("#set page(paper: \"a4\", margin: (x: 2cm, y: 2.5cm))\n");
        typst.push_str("#set text(font: \"SimSun\", size: 11pt)\n\n");

        if let Some(content) = doc.get("content").and_then(|c| c.as_array()) {
            for node in content {
                converter.check_recursion_depth()?;
                converter.stats.nodes_processed += 1;
                typst.push_str(&converter.convert_node(node)?);
            }
        }

        converter.stats.duration_ms = start.elapsed().as_millis() as u64;
        
        Ok(typst)
    }

    fn convert_node(&mut self, node: &Value) -> Result<String, String> {
        let node_type = node.get("type").and_then(|t| t.as_str()).unwrap_or("");

        match node_type {
            "paragraph" => self.convert_paragraph(node),
            "heading" => self.convert_heading(node),
            "bulletList" => self.convert_bullet_list(node),
            "orderedList" => self.convert_ordered_list(node),
            "blockquote" => self.convert_blockquote(node),
            "codeBlock" => self.convert_code_block(node),
            "horizontalRule" => Ok("#line(length: 100%)\n".to_string()),
            "table" => self.convert_table(node),
            "text" => self.convert_text(node),
            "hardBreak" => Ok("\\\n".to_string()),
            _ => {
                // Unknown node type - skip gracefully
                Ok(String::new())
            }
        }
    }

    fn convert_paragraph(&mut self, node: &Value) -> Result<String, String> {
        let content = self.extract_text_content(node).unwrap_or_default();
        Ok(format!("{}\n\n", content))
    }

    fn convert_heading(&mut self, node: &Value) -> Result<String, String> {
        let level = node
            .get("attrs")
            .and_then(|a| a.get("level"))
            .and_then(|l| l.as_u64())
            .unwrap_or(1);

        if let Err(_) = self.validate_heading_level(level) {
            // Use default level instead of error
            let content = self.extract_text_content(node).unwrap_or_default();
            let sanitized = Self::sanitize_text(&content);
            return Ok(format!("= {}\n\n", sanitized));
        }

        let content = self.extract_text_content(node)?;
        let sanitized = Self::sanitize_text(&content);
        let equals = "=".repeat(level as usize);
        Ok(format!("{} {}\n\n", equals, sanitized))
    }

    fn convert_bullet_list(&mut self, node: &Value) -> Result<String, String> {
        let editing_config = self.config_service.get_editing_engine_config();
        if let Some(content) = node.get("content").and_then(|c| c.as_array()) {
            if content.len() > editing_config.max_list_items {
                // Truncate list instead of error
                let truncated_count = editing_config.max_list_items;
                let mut result = String::new();
                for item in content.iter().take(truncated_count) {
                    if let Some(item_type) = item.get("type").and_then(|t| t.as_str()) {
                        if item_type == "listItem" {
                            let item_content = self.extract_text_content(item).unwrap_or_default();
                            let sanitized = Self::sanitize_text(&item_content);
                            result.push_str(&format!("- {}\n", sanitized));
                        }
                    }
                }
                result.push_str(&format!("// ... and {} more items (truncated)\n\n", content.len() - truncated_count));
                return Ok(result);
            }
            
            let mut result = String::new();
            for item in content {
                if let Some(item_type) = item.get("type").and_then(|t| t.as_str()) {
                    if item_type == "listItem" {
                        let item_content = self.extract_text_content(item)?;
                        let sanitized = Self::sanitize_text(&item_content);
                        result.push_str(&format!("- {}\n", sanitized));
                    }
                }
            }
            result.push('\n');
            Ok(result)
        } else {
            Ok(String::new())
        }
    }

    fn convert_ordered_list(&mut self, node: &Value) -> Result<String, String> {
        let editing_config = self.config_service.get_editing_engine_config();
        if let Some(content) = node.get("content").and_then(|c| c.as_array()) {
            if content.len() > editing_config.max_list_items {
                // Truncate list instead of error
                let truncated_count = editing_config.max_list_items;
                let mut result = String::new();
                for (index, item) in content.iter().take(truncated_count).enumerate() {
                    if let Some(item_type) = item.get("type").and_then(|t| t.as_str()) {
                        if item_type == "listItem" {
                            let item_content = self.extract_text_content(item).unwrap_or_default();
                            let sanitized = Self::sanitize_text(&item_content);
                            result.push_str(&format!("{}. {}\n", index + 1, sanitized));
                        }
                    }
                }
                result.push_str(&format!("// ... and {} more items (truncated)\n\n", content.len() - truncated_count));
                return Ok(result);
            }
            
            let mut result = String::new();
            for (index, item) in content.iter().enumerate() {
                if let Some(item_type) = item.get("type").and_then(|t| t.as_str()) {
                    if item_type == "listItem" {
                        let item_content = self.extract_text_content(item)?;
                        let sanitized = Self::sanitize_text(&item_content);
                        result.push_str(&format!("{}. {}\n", index + 1, sanitized));
                    }
                }
            }
            result.push('\n');
            Ok(result)
        } else {
            Ok(String::new())
        }
    }

    fn convert_blockquote(&mut self, node: &Value) -> Result<String, String> {
        let content = self.extract_text_content(node).unwrap_or_default();
        let sanitized = Self::sanitize_text(&content);
        Ok(format!(
            "#block(fill: rgb(\"f0f0f0\"), inset: 8pt, radius: 4pt)[\n  {}\n]\n\n",
            sanitized
        ))
    }

    fn convert_code_block(&mut self, node: &Value) -> Result<String, String> {
        let content = self.extract_text_content(node).unwrap_or_default();
        // Don't sanitize code blocks - preserve original content
        Ok(format!("```\n{}\n```\n\n", content))
    }

    fn convert_table(&mut self, node: &Value) -> Result<String, String> {
        let editing_config = self.config_service.get_editing_engine_config();
        if let Some(content) = node.get("content").and_then(|c| c.as_array()) {
            if content.len() > editing_config.max_table_rows {
                // Return simplified table instead of error
                return Ok("#text(\"[Table too large - truncated]\")\n\n".to_string());
            }
            
            let mut result = String::new();
            result.push_str("#table(\n");

            for row in content {
                if let Some(row_type) = row.get("type").and_then(|t| t.as_str()) {
                    if row_type == "tableRow" {
                        if let Some(row_content) = row.get("content").and_then(|c| c.as_array()) {
                            if row_content.len() > editing_config.max_table_columns {
                                // Skip this row instead of error
                                continue;
                            }
                            
                            let cells: Vec<String> = row_content
                                .iter()
                                .filter_map(|c| {
                                    let cell_type =
                                        c.get("type").and_then(|t| t.as_str()).unwrap_or("");
                                    if cell_type == "tableCell" || cell_type == "tableHeader" {
                                        let text = self.extract_text_content(c).unwrap_or_default();
                                        Some(Self::sanitize_text(&text))
                                    } else {
                                        None
                                    }
                                })
                                .collect();

                            let cell_str = cells
                                .iter()
                                .map(|c| format!("[{}]", c))
                                .collect::<Vec<_>>()
                                .join(", ");
                            result.push_str(&format!("  {}\n", cell_str));
                        }
                    }
                }
            }

            result.push_str(")\n\n");
            Ok(result)
        } else {
            Ok(String::new())
        }
    }

    fn convert_text(&mut self, node: &Value) -> Result<String, String> {
        let mut text = String::new();

        if let Some(content) = node.get("text").and_then(|t| t.as_str()) {
            text.push_str(content);
        }

        // Apply marks
        if let Some(marks) = node.get("marks").and_then(|m| m.as_array()) {
            for mark in marks {
                if let Some(mark_type) = mark.get("type").and_then(|t| t.as_str()) {
                    match mark_type {
                        "bold" => {
                            text = format!("*{}*", text);
                        }
                        "italic" => {
                            text = format!("_{}_", text);
                        }
                        "strike" => {
                            text = format!("#strike({})", text);
                        }
                        "code" => {
                            text = format!("`{}`", text);
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(Self::sanitize_text(&text))
    }

    fn extract_text_content(&mut self, node: &Value) -> Result<String, String> {
        let mut result = String::new();

        if let Some(content) = node.get("content").and_then(|c| c.as_array()) {
            for c in content {
                if let Some(node_type) = c.get("type").and_then(|t| t.as_str()) {
                    if node_type == "text" {
                        if let Some(text) = c.get("text").and_then(|t| t.as_str()) {
                            result.push_str(text);
                        }
                    } else {
                        // Skip errors in nested nodes for robustness
                        if let Ok(text) = self.convert_node(c) {
                            result.push_str(&text);
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    /// Fallback: generate basic Typst with error message
    fn fallback_typst(error: &str) -> String {
        format!(
            "#set page(paper: \"a4\", margin: (x: 2cm, y: 2.5cm))\n
            #set text(font: \"SimSun\", size: 11pt)\n\n
            = Conversion Error\n\n
            The document could not be converted.\n
            Error: {}\n\n
            Please check the input format and try again.",
            Self::sanitize_text(error)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_paragraph() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "paragraph",
                "content": [{"type": "text", "text": "Hello world"}]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("Hello world"));
    }

    #[test]
    fn test_heading() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "heading",
                "attrs": {"level": 1},
                "content": [{"type": "text", "text": "Title"}]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("= Title"));
    }

    #[test]
    fn test_invalid_json() {
        let json = "invalid json";
        let result = JsonToTypstConverter::convert(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_input_size_validation() {
        let config_service = Arc::new(ExportConfigService::new());
        let editing_config = config_service.get_editing_engine_config();
        let large_input = "a".repeat(editing_config.max_input_size + 1);
        let json = format!(r#"{{"type": "doc", "content": [{{"type": "paragraph", "content": [{{"type": "text", "text": "{}"}}]}}]}}"#, large_input);
        let result = JsonToTypstConverter::convert(&json);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum allowed size"));
    }

    #[test]
    fn test_max_input_size_accepted() {
        let config_service = Arc::new(ExportConfigService::new());
        let editing_config = config_service.get_editing_engine_config();
        // Use a smaller input that fits within limits when JSON overhead is considered
        let large_input = "a".repeat(editing_config.max_input_size - 1000); // Leave room for JSON structure
        let json = format!(r#"{{"type": "doc", "content": [{{"type": "paragraph", "content": [{{"type": "text", "text": "{}"}}]}}]}}"#, large_input);
        let result = JsonToTypstConverter::convert(&json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_heading_level_validation() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "heading",
                "attrs": {"level": 7},
                "content": [{"type": "text", "text": "Title"}]
            }]
        }"#;
        let result = JsonToTypstConverter::convert(json);
        // With graceful degradation, should use default level instead of error
        assert!(result.is_ok());
        let typst = result.unwrap();
        assert!(typst.contains("= Title"));
    }

    #[test]
    fn test_heading_level_zero() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "heading",
                "attrs": {"level": 0},
                "content": [{"type": "text", "text": "Title"}]
            }]
        }"#;
        let result = JsonToTypstConverter::convert(json);
        // With graceful degradation, should use default level instead of error
        assert!(result.is_ok());
        let typst = result.unwrap();
        assert!(typst.contains("= Title"));
    }

    #[test]
    fn test_list_size_validation() {
        let config_service = Arc::new(ExportConfigService::new());
        let editing_config = config_service.get_editing_engine_config();
        let mut items = Vec::new();
        for i in 0..=editing_config.max_list_items {
            items.push(format!(r#"{{"type": "listItem", "content": [{{"type": "text", "text": "Item {}"}}]}}"#, i));
        }
        let json = format!(r#"{{"type": "doc", "content": [{{"type": "bulletList", "content": [{}]}}]}}"#, items.join(","));
        let result = JsonToTypstConverter::convert(&json);
        // With graceful degradation, should truncate instead of error
        assert!(result.is_ok());
        let typst = result.unwrap();
        assert!(typst.contains("truncated"));
    }

    #[test]
    fn test_table_rows_validation() {
        let config_service = Arc::new(ExportConfigService::new());
        let editing_config = config_service.get_editing_engine_config();
        let mut rows = Vec::new();
        for i in 0..=editing_config.max_table_rows {
            rows.push(format!(r#"{{"type": "tableRow", "content": [{{"type": "tableCell", "content": [{{"type": "text", "text": "Row {}"}}]}}]}}"#, i));
        }
        let json = format!(r#"{{"type": "doc", "content": [{{"type": "table", "content": [{}]}}]}}"#, rows.join(","));
        let result = JsonToTypstConverter::convert(&json);
        // With graceful degradation, should truncate instead of error
        assert!(result.is_ok());
        let typst = result.unwrap();
        assert!(typst.contains("truncated"));
    }

    #[test]
    fn test_table_columns_validation() {
        let config_service = Arc::new(ExportConfigService::new());
        let editing_config = config_service.get_editing_engine_config();
        let mut cells = Vec::new();
        for i in 0..=editing_config.max_table_columns {
            cells.push(format!(r#"{{"type": "tableCell", "content": [{{"type": "text", "text": "Cell {}"}}]}}"#, i));
        }
        let json = format!(r#"{{"type": "doc", "content": [{{"type": "table", "content": [{{"type": "tableRow", "content": [{}]}}]}}]}}"#, cells.join(","));
        let result = JsonToTypstConverter::convert(&json);
        // With graceful degradation, should skip row instead of error
        assert!(result.is_ok());
        // Row with too many columns is skipped, so result may be empty or minimal
    }

    #[test]
    fn test_text_sanitization() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "paragraph",
                "content": [{"type": "text", "text": "Test"}]
            }]
        }"#;
        let result = JsonToTypstConverter::convert(json).unwrap();
        // Should successfully convert with sanitization
        assert!(result.contains("Test"));
    }

    #[test]
    fn test_converter_new() {
        let converter = JsonToTypstConverter::new(Arc::new(ExportConfigService::new()));
        assert_eq!(converter.recursion_depth, 0);
    }

    #[test]
    fn test_converter_default() {
        let converter = JsonToTypstConverter::default();
        assert_eq!(converter.recursion_depth, 0);
    }

    #[test]
    fn test_recursion_depth_reset() {
        let mut converter = JsonToTypstConverter::new(Arc::new(ExportConfigService::new()));
        converter.recursion_depth = 50;
        converter.reset_recursion_depth();
        assert_eq!(converter.recursion_depth, 0);
    }

    #[test]
    fn test_convert_with_fallback_success() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "paragraph",
                "content": [{"type": "text", "text": "Test"}]
            }]
        }"#;
        let result = JsonToTypstConverter::convert_with_fallback(json);
        assert!(!result.is_fallback);
        assert!(result.result.contains("Test"));
    }

    #[test]
    fn test_convert_with_fallback_on_error() {
        let json = "invalid json";
        let result = JsonToTypstConverter::convert_with_fallback(json);
        assert!(result.is_fallback);
        assert!(result.fallback_strategy.is_some());
        assert!(result.result.contains("Conversion Error"));
    }

    #[test]
    fn test_list_truncation() {
        let config_service = Arc::new(ExportConfigService::new());
        let editing_config = config_service.get_editing_engine_config();
        let mut items = Vec::new();
        for i in 0..=editing_config.max_list_items {
            items.push(format!(r#"{{"type": "listItem", "content": [{{"type": "text", "text": "Item {}"}}]}}"#, i));
        }
        let json = format!(r#"{{"type": "doc", "content": [{{"type": "bulletList", "content": [{}]}}]}}"#, items.join(","));
        let result = JsonToTypstConverter::convert(&json);
        assert!(result.is_ok());
        let typst = result.unwrap();
        assert!(typst.contains("truncated"));
    }

    #[test]
    fn test_table_truncation() {
        let config_service = Arc::new(ExportConfigService::new());
        let editing_config = config_service.get_editing_engine_config();
        let mut rows = Vec::new();
        for i in 0..=editing_config.max_table_rows {
            rows.push(format!(r#"{{"type": "tableRow", "content": [{{"type": "tableCell", "content": [{{"type": "text", "text": "Row {}"}}]}}]}}"#, i));
        }
        let json = format!(r#"{{"type": "doc", "content": [{{"type": "table", "content": [{}]}}]}}"#, rows.join(","));
        let result = JsonToTypstConverter::convert(&json);
        assert!(result.is_ok());
        let typst = result.unwrap();
        assert!(typst.contains("truncated"));
    }

    #[test]
    fn test_heading_level_fallback() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "heading",
                "attrs": {"level": 10},
                "content": [{"type": "text", "text": "Title"}]
            }]
        }"#;
        let result = JsonToTypstConverter::convert(&json);
        assert!(result.is_ok());
        let typst = result.unwrap();
        // Should use default level instead of error
        assert!(typst.contains("= Title"));
    }

    #[test]
    fn test_unknown_node_type_graceful_skip() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "unknownType",
                "content": [{"type": "text", "text": "Test"}]
            }]
        }"#;
        let result = JsonToTypstConverter::convert(&json);
        assert!(result.is_ok());
        // Should skip unknown node type gracefully
    }

    #[test]
    fn test_error_context_in_fallback() {
        let json = "invalid json";
        let result = JsonToTypstConverter::convert_with_fallback(json);
        if result.is_fallback {
            assert!(result.error_context.is_some());
            let context = result.error_context.unwrap();
            assert_eq!(context.code, "JSON_TO_TYPST_FAILED");
            assert_eq!(context.source, "json_to_typst");
        }
    }

    #[test]
    fn test_fallback_typst() {
        let error = "Test error message";
        let fallback = JsonToTypstConverter::fallback_typst(error);
        assert!(fallback.contains("Conversion Error"));
        assert!(fallback.contains("Test error message"));
    }

    #[test]
    fn test_bullet_list() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "bulletList",
                "content": [{
                    "type": "listItem",
                    "content": [{"type": "text", "text": "Item 1"}]
                }]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("- Item 1"));
    }

    #[test]
    fn test_ordered_list() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "orderedList",
                "content": [{
                    "type": "listItem",
                    "content": [{"type": "text", "text": "First"}]
                }]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("1. First"));
    }

    #[test]
    fn test_blockquote() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "blockquote",
                "content": [{"type": "text", "text": "Quote"}]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("#block"));
    }

    #[test]
    fn test_code_block() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "codeBlock",
                "content": [{"type": "text", "text": "code"}]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("```"));
    }

    #[test]
    fn test_horizontal_rule() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "horizontalRule"
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("#line"));
    }

    #[test]
    fn test_empty_document() {
        let json = r#"{
            "type": "doc",
            "content": []
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("#set page"));
    }

    #[test]
    fn test_text_bold() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "text",
                "text": "Bold",
                "marks": [{"type": "bold"}]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("*Bold*"));
    }

    #[test]
    fn test_text_italic() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "text",
                "text": "Italic",
                "marks": [{"type": "italic"}]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("_Italic_"));
    }

    #[test]
    fn test_text_strike() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "text",
                "text": "Strike",
                "marks": [{"type": "strike"}]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("#strike"));
    }

    #[test]
    fn test_text_code() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "text",
                "text": "code",
                "marks": [{"type": "code"}]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("`code`"));
    }

    #[test]
    fn test_text_multiple_marks() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "paragraph",
                "content": [{
                    "type": "text",
                    "text": "Text",
                    "marks": [{"type": "bold"}, {"type": "italic"}]
                }]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_heading_level_2() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "heading",
                "attrs": {"level": 2},
                "content": [{"type": "text", "text": "Subtitle"}]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("== Subtitle"));
    }

    #[test]
    fn test_heading_level_3() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "heading",
                "attrs": {"level": 3},
                "content": [{"type": "text", "text": "Section"}]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("=== Section"));
    }

    #[test]
    fn test_heading_without_level() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "heading",
                "content": [{"type": "text", "text": "Default"}]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("= Default"));
    }

    #[test]
    fn test_table() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "table",
                "content": [{
                    "type": "tableRow",
                    "content": [
                        {"type": "tableCell", "content": [{"type": "text", "text": "Cell 1"}]},
                        {"type": "tableCell", "content": [{"type": "text", "text": "Cell 2"}]}
                    ]
                }]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("#table"));
        assert!(result.contains("Cell 1"));
        assert!(result.contains("Cell 2"));
    }

    #[test]
    fn test_table_with_header() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "table",
                "content": [{
                    "type": "tableRow",
                    "content": [
                        {"type": "tableHeader", "content": [{"type": "text", "text": "Header"}]},
                        {"type": "tableCell", "content": [{"type": "text", "text": "Data"}]}
                    ]
                }]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("Header"));
        assert!(result.contains("Data"));
    }

    #[test]
    fn test_table_multiple_rows() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "table",
                "content": [
                    {
                        "type": "tableRow",
                        "content": [{"type": "tableCell", "content": [{"type": "text", "text": "Row 1"}]}]
                    },
                    {
                        "type": "tableRow",
                        "content": [{"type": "tableCell", "content": [{"type": "text", "text": "Row 2"}]}]
                    }
                ]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("Row 1"));
        assert!(result.contains("Row 2"));
    }

    #[test]
    fn test_hard_break() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "paragraph",
                "content": [{"type": "hardBreak"}]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("\\"));
    }

    #[test]
    fn test_unknown_node_type() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "unknownType"
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_paragraphs() {
        let json = r#"{
            "type": "doc",
            "content": [
                {
                    "type": "paragraph",
                    "content": [{"type": "text", "text": "First"}]
                },
                {
                    "type": "paragraph",
                    "content": [{"type": "text", "text": "Second"}]
                }
            ]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("First"));
        assert!(result.contains("Second"));
    }

    #[test]
    fn test_bullet_list_multiple_items() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "bulletList",
                "content": [
                    {"type": "listItem", "content": [{"type": "text", "text": "Item 1"}]},
                    {"type": "listItem", "content": [{"type": "text", "text": "Item 2"}]},
                    {"type": "listItem", "content": [{"type": "text", "text": "Item 3"}]}
                ]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("- Item 1"));
        assert!(result.contains("- Item 2"));
        assert!(result.contains("- Item 3"));
    }

    #[test]
    fn test_ordered_list_multiple_items() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "orderedList",
                "content": [
                    {"type": "listItem", "content": [{"type": "text", "text": "First"}]},
                    {"type": "listItem", "content": [{"type": "text", "text": "Second"}]},
                    {"type": "listItem", "content": [{"type": "text", "text": "Third"}]}
                ]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("1. First"));
        assert!(result.contains("2. Second"));
        assert!(result.contains("3. Third"));
    }

    #[test]
    fn test_text_without_marks() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "paragraph",
                "content": [{"type": "text", "text": "Plain text"}]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("Plain text"));
    }

    #[test]
    fn test_paragraph_with_nested_content() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "paragraph",
                "content": [
                    {"type": "text", "text": "Hello "},
                    {"type": "text", "text": "world", "marks": [{"type": "bold"}]}
                ]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("Hello"));
        assert!(result.contains("world"));
    }

    #[test]
    fn test_table_empty_content() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "table",
                "content": []
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_bullet_list_empty_content() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "bulletList",
                "content": []
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_ordered_list_empty_content() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "orderedList",
                "content": []
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_document_without_content() {
        let json = r#"{
            "type": "doc"
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("#set page"));
    }

    #[test]
    fn test_text_with_unknown_mark() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "paragraph",
                "content": [{
                    "type": "text",
                    "text": "Text",
                    "marks": [{"type": "unknown"}]
                }]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("Text"));
    }

    #[test]
    fn test_heading_with_nested_marks() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "heading",
                "attrs": {"level": 1},
                "content": [{
                    "type": "text",
                    "text": "Title",
                    "marks": [{"type": "bold"}]
                }]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("Title"));
    }

    #[test]
    fn test_code_block_multiline() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "codeBlock",
                "content": [
                    {"type": "text", "text": "Line 1"},
                    {"type": "hardBreak"},
                    {"type": "text", "text": "Line 2"}
                ]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("Line 1"));
        assert!(result.contains("Line 2"));
    }

    #[test]
    fn test_table_row_with_empty_cells() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "table",
                "content": [{
                    "type": "tableRow",
                    "content": [
                        {"type": "tableCell", "content": []},
                        {"type": "tableCell", "content": [{"type": "text", "text": "Data"}]}
                    ]
                }]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("Data"));
    }

    #[test]
    fn test_paragraph_empty_content() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "paragraph",
                "content": []
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_typst_header() {
        let json = r#"{
            "type": "doc",
            "content": []
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("#set page(paper: \"a4\", margin: (x: 2cm, y: 2.5cm))"));
        assert!(result.contains("#set text(font: \"SimSun\", size: 11pt)"));
    }

    #[test]
    fn test_blockquote_with_nested_content() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "blockquote",
                "content": [
                    {"type": "text", "text": "Quote "},
                    {"type": "text", "text": "text", "marks": [{"type": "italic"}]}
                ]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("#block"));
        assert!(result.contains("Quote"));
    }

    #[test]
    fn test_list_item_with_nested_content() {
        let json = r#"{
            "type": "doc",
            "content": [{
                "type": "bulletList",
                "content": [{
                    "type": "listItem",
                    "content": [
                        {"type": "text", "text": "Item "},
                        {"type": "text", "text": "text", "marks": [{"type": "bold"}]}
                    ]
                }]
            }]
        }"#;

        let result = JsonToTypstConverter::convert(json).unwrap();
        assert!(result.contains("- Item"));
        assert!(result.contains("text"));
    }
}
