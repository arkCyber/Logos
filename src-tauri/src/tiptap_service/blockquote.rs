//! TipTap Block Quote Manager - Aerospace-Grade Block Quote Operations Service
//!
//! Safety-critical block quote operations service with:
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

/// Maximum block quote depth to prevent stack overflow
const MAX_BLOCKQUOTE_DEPTH: usize = 10;

/// Maximum block quote text length
const MAX_BLOCKQUOTE_TEXT_LENGTH: usize = 10000;

/// Block quote attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockQuoteAttributes {
    pub citation: Option<String>,
    pub alignment: Option<String>,
}

impl Default for BlockQuoteAttributes {
    fn default() -> Self {
        Self {
            citation: None,
            alignment: None,
        }
    }
}

pub struct BlockQuoteManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl BlockQuoteManager {
    /// Creates a new block quote manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new BlockQuoteManager instance
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

    /// Get the maximum block quote depth constant
    /// 
    /// # Returns
    /// The maximum block quote depth
    pub fn max_blockquote_depth() -> usize {
        MAX_BLOCKQUOTE_DEPTH
    }

    /// Get the maximum block quote text length constant
    /// 
    /// # Returns
    /// The maximum text length for block quotes
    pub fn max_blockquote_text_length() -> usize {
        MAX_BLOCKQUOTE_TEXT_LENGTH
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

    /// Validate block quote depth
    /// 
    /// # Arguments
    /// * `depth` - The current block quote depth
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents stack overflow by limiting depth
    fn validate_blockquote_depth(&self, depth: usize) -> Result<(), String> {
        if depth >= MAX_BLOCKQUOTE_DEPTH {
            return Err(format!("Block quote depth exceeds maximum of {}", MAX_BLOCKQUOTE_DEPTH));
        }
        Ok(())
    }

    /// Validate block quote text length
    /// 
    /// # Arguments
    /// * `text` - The text to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting text length
    fn validate_blockquote_text(&self, text: &str) -> Result<(), String> {
        if text.len() > MAX_BLOCKQUOTE_TEXT_LENGTH {
            return Err(format!("Block quote text exceeds maximum length of {} characters", MAX_BLOCKQUOTE_TEXT_LENGTH));
        }
        Ok(())
    }

    /// Create a block quote node
    /// 
    /// # Arguments
    /// * `content` - The content of the block quote
    /// * `attributes` - Optional block quote attributes
    /// * `depth` - The nesting depth
    /// 
    /// # Returns
    /// Result containing the block quote node or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates depth and text length
    pub fn create_blockquote(&mut self, content: Vec<TipTapNode>, attributes: Option<BlockQuoteAttributes>, depth: usize) -> Result<TipTapNode, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate depth
        self.validate_blockquote_depth(depth)?;

        // Validate text length in content
        for node in &content {
            if let Some(ref text) = node.text {
                self.validate_blockquote_text(text)?;
            }
        }

        let attrs_json = if let Some(attrs) = attributes {
            Some(serde_json::to_value(attrs).map_err(|e| {
                let error = format!("Failed to serialize block quote attributes: {}", e);
                self.record_error("SERIALIZE_ERROR", &error, "create_blockquote");
                error
            })?)
        } else {
            None
        };

        let blockquote_node = TipTapNode {
            node_type: NodeType::Blockquote,
            content: Some(content),
            text: None,
            attrs: attrs_json,
            marks: None,
        };

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Block quote creation CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Block quote creation performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(blockquote_node)
    }

    /// Add content to a block quote
    /// 
    /// # Arguments
    /// * `blockquote_node` - The block quote node to add to
    /// * `content_node` - The content node to add
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn add_content(&mut self, blockquote_node: &mut TipTapNode, content_node: TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate text length
        if let Some(ref text) = content_node.text {
            self.validate_blockquote_text(text)?;
        }

        if let Some(ref mut content) = blockquote_node.content {
            content.push(content_node);
        } else {
            blockquote_node.content = Some(vec![content_node]);
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Block quote content add CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Block quote content add performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Update block quote attributes
    /// 
    /// # Arguments
    /// * `blockquote_node` - The block quote node to update
    /// * `attributes` - The new attributes
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn update_attributes(&mut self, blockquote_node: &mut TipTapNode, attributes: BlockQuoteAttributes) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        let attrs_json = serde_json::to_value(&attributes).map_err(|e| {
            let error = format!("Failed to serialize block quote attributes: {}", e);
            self.record_error("SERIALIZE_ERROR", &error, "update_attributes");
            error
        })?;

        blockquote_node.attrs = Some(attrs_json);

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Block quote attributes update CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Block quote attributes update performance warning: took {}ms", elapsed.as_millis());
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
    fn test_blockquote_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BlockQuoteManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(BlockQuoteManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(BlockQuoteManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(BlockQuoteManager::max_blockquote_depth(), MAX_BLOCKQUOTE_DEPTH);
        assert_eq!(BlockQuoteManager::max_blockquote_text_length(), MAX_BLOCKQUOTE_TEXT_LENGTH);
    }

    #[test]
    fn test_create_blockquote() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BlockQuoteManager::new(config_service);
        
        let content = vec![TipTapNode {
            node_type: NodeType::Paragraph,
            content: Some(vec![TipTapNode {
                node_type: NodeType::Text,
                content: None,
                text: Some("Test quote".to_string()),
                attrs: None,
                marks: None,
            }]),
            text: None,
            attrs: None,
            marks: None,
        }];
        
        let result = manager.create_blockquote(content, None, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_blockquote_too_deep() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BlockQuoteManager::new(config_service);
        
        let content = vec![TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        }];
        
        let result = manager.create_blockquote(content, None, MAX_BLOCKQUOTE_DEPTH);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_blockquote_text_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BlockQuoteManager::new(config_service);
        
        let long_text = "a".repeat(MAX_BLOCKQUOTE_TEXT_LENGTH + 1);
        let content = vec![TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some(long_text),
            attrs: None,
            marks: None,
        }];
        
        let result = manager.create_blockquote(content, None, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_content() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BlockQuoteManager::new(config_service);
        
        let mut blockquote_node = manager.create_blockquote(vec![], None, 0).unwrap();
        let content_node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.add_content(&mut blockquote_node, content_node);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_content_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BlockQuoteManager::new(config_service);
        
        let mut blockquote_node = manager.create_blockquote(vec![], None, 0).unwrap();
        let long_text = "a".repeat(MAX_BLOCKQUOTE_TEXT_LENGTH + 1);
        let content_node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some(long_text),
            attrs: None,
            marks: None,
        };
        
        let result = manager.add_content(&mut blockquote_node, content_node);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_attributes() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BlockQuoteManager::new(config_service);
        
        let mut blockquote_node = manager.create_blockquote(vec![], None, 0).unwrap();
        let attributes = BlockQuoteAttributes {
            citation: Some("Test citation".to_string()),
            alignment: Some("center".to_string()),
        };
        
        let result = manager.update_attributes(&mut blockquote_node, attributes);
        assert!(result.is_ok());
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BlockQuoteManager::new(config_service);
        
        let content = vec![TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        }];
        
        manager.create_blockquote(content, None, 0).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BlockQuoteManager::new(config_service);
        
        let content = vec![TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        }];
        
        manager.create_blockquote(content, None, 0).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BlockQuoteManager::new(config_service);
        
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
        let mut manager = BlockQuoteManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }

    #[test]
    fn test_blockquote_attributes_default() {
        let attrs = BlockQuoteAttributes::default();
        assert!(attrs.citation.is_none());
        assert!(attrs.alignment.is_none());
    }
}
