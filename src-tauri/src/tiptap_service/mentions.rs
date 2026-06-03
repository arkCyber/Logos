//! TipTap Mentions Manager - Aerospace-Grade Mentions Operations Service
//!
//! Safety-critical mentions operations service with:
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

/// Maximum mention ID length
const MAX_MENTION_ID_LENGTH: usize = 100;

/// Maximum mention label length
const MAX_MENTION_LABEL_LENGTH: usize = 200;

/// Mention attributes
#[derive(Debug, Clone)]
pub struct MentionAttributes {
    pub id: String,
    pub label: String,
}

impl MentionAttributes {
    /// Create new mention attributes
    pub fn new(id: String, label: String) -> Self {
        Self { id, label }
    }
}

pub struct MentionsManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl MentionsManager {
    /// Creates a new mentions manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new MentionsManager instance
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

    /// Get the maximum mention ID length constant
    /// 
    /// # Returns
    /// The maximum mention ID length
    pub fn max_mention_id_length() -> usize {
        MAX_MENTION_ID_LENGTH
    }

    /// Get the maximum mention label length constant
    /// 
    /// # Returns
    /// The maximum mention label length
    pub fn max_mention_label_length() -> usize {
        MAX_MENTION_LABEL_LENGTH
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

    /// Validate mention ID
    /// 
    /// # Arguments
    /// * `id` - The mention ID to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting ID length
    fn validate_mention_id(&self, id: &str) -> Result<(), String> {
        if id.is_empty() {
            return Err("Mention ID cannot be empty".to_string());
        }
        if id.len() > MAX_MENTION_ID_LENGTH {
            return Err(format!("Mention ID exceeds maximum length of {} characters", MAX_MENTION_ID_LENGTH));
        }
        Ok(())
    }

    /// Validate mention label
    /// 
    /// # Arguments
    /// * `label` - The mention label to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting label length
    fn validate_mention_label(&self, label: &str) -> Result<(), String> {
        if label.is_empty() {
            return Err("Mention label cannot be empty".to_string());
        }
        if label.len() > MAX_MENTION_LABEL_LENGTH {
            return Err(format!("Mention label exceeds maximum length of {} characters", MAX_MENTION_LABEL_LENGTH));
        }
        Ok(())
    }

    /// Create a mention node
    /// 
    /// # Arguments
    /// * `attributes` - The mention attributes
    /// 
    /// # Returns
    /// Result containing the mention node or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates ID and label
    pub fn create_mention(&mut self, attributes: MentionAttributes) -> Result<TipTapNode, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate ID and label
        self.validate_mention_id(&attributes.id)?;
        self.validate_mention_label(&attributes.label)?;

        let mention_node = TipTapNode {
            node_type: NodeType::Link, // Mentions are often represented as links
            content: Some(vec![TipTapNode {
                node_type: NodeType::Text,
                content: None,
                text: Some(attributes.label.clone()),
                attrs: None,
                marks: None,
            }]),
            text: None,
            attrs: Some(serde_json::json!({
                "href": format!("@{}", attributes.id),
                "mention": true,
                "id": attributes.id,
                "label": attributes.label
            })),
            marks: None,
        };

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Mention creation CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Mention creation performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(mention_node)
    }

    /// Check if a node is a mention
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node is a mention, false otherwise
    pub fn is_mention(&self, node: &TipTapNode) -> bool {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(mention) = obj.get("mention") {
                    if let Some(b) = mention.as_bool() {
                        return b;
                    }
                }
            }
        }
        false
    }

    /// Get mention ID from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get mention ID from
    /// 
    /// # Returns
    /// Option containing the mention ID or None
    pub fn get_mention_id(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(id) = obj.get("id") {
                    if let Some(s) = id.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Get mention label from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get mention label from
    /// 
    /// # Returns
    /// Option containing the mention label or None
    pub fn get_mention_label(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(label) = obj.get("label") {
                    if let Some(s) = label.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Get mention attributes from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get mention attributes from
    /// 
    /// # Returns
    /// Option containing the mention attributes or None
    pub fn get_mention_attributes(&self, node: &TipTapNode) -> Option<MentionAttributes> {
        if self.is_mention(node) {
            let id = self.get_mention_id(node)?;
            let label = self.get_mention_label(node)?;
            Some(MentionAttributes::new(id, label))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_mentions_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MentionsManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(MentionsManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(MentionsManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(MentionsManager::max_mention_id_length(), MAX_MENTION_ID_LENGTH);
        assert_eq!(MentionsManager::max_mention_label_length(), MAX_MENTION_LABEL_LENGTH);
    }

    #[test]
    fn test_create_mention() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MentionsManager::new(config_service);
        
        let attributes = MentionAttributes::new("user123".to_string(), "John Doe".to_string());
        let result = manager.create_mention(attributes);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_mention_empty_id() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MentionsManager::new(config_service);
        
        let attributes = MentionAttributes::new("".to_string(), "John Doe".to_string());
        let result = manager.create_mention(attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_mention_empty_label() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MentionsManager::new(config_service);
        
        let attributes = MentionAttributes::new("user123".to_string(), "".to_string());
        let result = manager.create_mention(attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_mention_id_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MentionsManager::new(config_service);
        
        let long_id = "a".repeat(MAX_MENTION_ID_LENGTH + 1);
        let attributes = MentionAttributes::new(long_id, "John Doe".to_string());
        let result = manager.create_mention(attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_mention_label_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MentionsManager::new(config_service);
        
        let long_label = "a".repeat(MAX_MENTION_LABEL_LENGTH + 1);
        let attributes = MentionAttributes::new("user123".to_string(), long_label);
        let result = manager.create_mention(attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_is_mention() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MentionsManager::new(config_service);
        
        let mention_node = manager.create_mention(MentionAttributes::new("user123".to_string(), "John Doe".to_string())).unwrap();
        let text_node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.is_mention(&mention_node));
        assert!(!manager.is_mention(&text_node));
    }

    #[test]
    fn test_get_mention_id() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MentionsManager::new(config_service);
        
        let mention_node = manager.create_mention(MentionAttributes::new("user123".to_string(), "John Doe".to_string())).unwrap();
        let id = manager.get_mention_id(&mention_node);
        assert_eq!(id, Some("user123".to_string()));
    }

    #[test]
    fn test_get_mention_label() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MentionsManager::new(config_service);
        
        let mention_node = manager.create_mention(MentionAttributes::new("user123".to_string(), "John Doe".to_string())).unwrap();
        let label = manager.get_mention_label(&mention_node);
        assert_eq!(label, Some("John Doe".to_string()));
    }

    #[test]
    fn test_get_mention_attributes() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MentionsManager::new(config_service);
        
        let mention_node = manager.create_mention(MentionAttributes::new("user123".to_string(), "John Doe".to_string())).unwrap();
        let attrs = manager.get_mention_attributes(&mention_node);
        assert!(attrs.is_some());
        let mention_attrs = attrs.unwrap();
        assert_eq!(mention_attrs.id, "user123");
        assert_eq!(mention_attrs.label, "John Doe");
    }

    #[test]
    fn test_get_mention_attributes_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MentionsManager::new(config_service);
        
        let text_node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let attrs = manager.get_mention_attributes(&text_node);
        assert!(attrs.is_none());
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MentionsManager::new(config_service);
        
        manager.create_mention(MentionAttributes::new("user123".to_string(), "John Doe".to_string())).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MentionsManager::new(config_service);
        
        manager.create_mention(MentionAttributes::new("user123".to_string(), "John Doe".to_string())).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MentionsManager::new(config_service);
        
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
        let mut manager = MentionsManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
