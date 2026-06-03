//! TipTap Whitespace Manager - Aerospace-Grade Whitespace Operations Service
//!
//! Safety-critical whitespace operations service with:
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

/// Maximum whitespace string length
const MAX_WHITESPACE_LENGTH: usize = 50;

/// Whitespace type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Whitespace {
    Normal,
    Pre,
    Nowrap,
    PreWrap,
    PreLine,
}

impl Whitespace {
    /// Convert whitespace to string
    pub fn as_str(&self) -> &str {
        match self {
            Whitespace::Normal => "normal",
            Whitespace::Pre => "pre",
            Whitespace::Nowrap => "nowrap",
            Whitespace::PreWrap => "pre-wrap",
            Whitespace::PreLine => "pre-line",
        }
    }

    /// Parse whitespace from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(Whitespace::Normal),
            "pre" => Ok(Whitespace::Pre),
            "nowrap" => Ok(Whitespace::Nowrap),
            "pre-wrap" => Ok(Whitespace::PreWrap),
            "pre-line" => Ok(Whitespace::PreLine),
            _ => Err(format!("Invalid whitespace: {}", s)),
        }
    }
}

pub struct WhitespaceManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl WhitespaceManager {
    /// Creates a new whitespace manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new WhitespaceManager instance
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

    /// Get the maximum whitespace length constant
    /// 
    /// # Returns
    /// The maximum whitespace string length
    pub fn max_whitespace_length() -> usize {
        MAX_WHITESPACE_LENGTH
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

    /// Validate whitespace string
    /// 
    /// # Arguments
    /// * `whitespace` - The whitespace string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting whitespace string length
    fn validate_whitespace(&self, whitespace: &str) -> Result<(), String> {
        if whitespace.len() > MAX_WHITESPACE_LENGTH {
            return Err(format!("Whitespace string exceeds maximum length of {} characters", MAX_WHITESPACE_LENGTH));
        }
        
        // Validate whitespace value
        Whitespace::from_str(whitespace)?;
        
        Ok(())
    }

    /// Apply whitespace to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply whitespace to
    /// * `whitespace` - The whitespace to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates whitespace string
    pub fn apply_whitespace(&mut self, node: &mut TipTapNode, whitespace: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate whitespace
        self.validate_whitespace(whitespace)?;

        // Apply whitespace to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("whiteSpace".to_string(), serde_json::Value::String(whitespace.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "whiteSpace": whitespace }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Whitespace application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Whitespace application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove whitespace from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove whitespace from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_whitespace(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("whiteSpace");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Whitespace removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Whitespace removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get whitespace from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get whitespace from
    /// 
    /// # Returns
    /// Option containing the whitespace string or None
    pub fn get_whitespace(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(whitespace) = obj.get("whiteSpace") {
                    if let Some(s) = whitespace.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has whitespace
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has whitespace, false otherwise
    pub fn has_whitespace(&self, node: &TipTapNode) -> bool {
        self.get_whitespace(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_whitespace_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = WhitespaceManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(WhitespaceManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(WhitespaceManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(WhitespaceManager::max_whitespace_length(), MAX_WHITESPACE_LENGTH);
    }

    #[test]
    fn test_whitespace_variants() {
        assert_eq!(Whitespace::Normal.as_str(), "normal");
        assert_eq!(Whitespace::Pre.as_str(), "pre");
        assert_eq!(Whitespace::Nowrap.as_str(), "nowrap");
    }

    #[test]
    fn test_whitespace_from_str() {
        assert!(matches!(Whitespace::from_str("normal"), Ok(Whitespace::Normal)));
        assert!(matches!(Whitespace::from_str("pre"), Ok(Whitespace::Pre)));
        assert!(matches!(Whitespace::from_str("nowrap"), Ok(Whitespace::Nowrap)));
        assert!(Whitespace::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_whitespace() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WhitespaceManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_whitespace(&mut node, "pre");
        assert!(result.is_ok());
        assert!(manager.has_whitespace(&node));
    }

    #[test]
    fn test_apply_whitespace_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WhitespaceManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_whitespace(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_whitespace_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WhitespaceManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_whitespace = "a".repeat(MAX_WHITESPACE_LENGTH + 1);
        let result = manager.apply_whitespace(&mut node, &long_whitespace);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_whitespace() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WhitespaceManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "whiteSpace": "pre" })),
            marks: None,
        };
        
        assert!(manager.has_whitespace(&node));
        let result = manager.remove_whitespace(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_whitespace(&node));
    }

    #[test]
    fn test_get_whitespace() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = WhitespaceManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "whiteSpace": "nowrap" })),
            marks: None,
        };
        
        let whitespace = manager.get_whitespace(&node);
        assert_eq!(whitespace, Some("nowrap".to_string()));
    }

    #[test]
    fn test_get_whitespace_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = WhitespaceManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let whitespace = manager.get_whitespace(&node);
        assert!(whitespace.is_none());
    }

    #[test]
    fn test_has_whitespace() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = WhitespaceManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "whiteSpace": "pre-wrap" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_whitespace(&node_with));
        assert!(!manager.has_whitespace(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WhitespaceManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_whitespace(&mut node, "pre").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WhitespaceManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_whitespace(&mut node, "pre").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WhitespaceManager::new(config_service);
        
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
        let mut manager = WhitespaceManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
