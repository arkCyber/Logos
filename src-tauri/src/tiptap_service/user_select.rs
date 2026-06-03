//! TipTap User Select Manager - Aerospace-Grade User Select Operations Service
//!
//! Safety-critical user select operations service with:
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

/// Maximum user select string length
const MAX_USER_SELECT_LENGTH: usize = 50;

/// User select type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserSelect {
    Auto,
    None,
    Text,
    Contain,
    All,
}

impl UserSelect {
    /// Convert user select to string
    pub fn as_str(&self) -> &str {
        match self {
            UserSelect::Auto => "auto",
            UserSelect::None => "none",
            UserSelect::Text => "text",
            UserSelect::Contain => "contain",
            UserSelect::All => "all",
        }
    }

    /// Parse user select from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(UserSelect::Auto),
            "none" => Ok(UserSelect::None),
            "text" => Ok(UserSelect::Text),
            "contain" => Ok(UserSelect::Contain),
            "all" => Ok(UserSelect::All),
            _ => Err(format!("Invalid user select: {}", s)),
        }
    }
}

pub struct UserSelectManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl UserSelectManager {
    /// Creates a new user select manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new UserSelectManager instance
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

    /// Get the maximum user select length constant
    /// 
    /// # Returns
    /// The maximum user select string length
    pub fn max_user_select_length() -> usize {
        MAX_USER_SELECT_LENGTH
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

    /// Validate user select string
    /// 
    /// # Arguments
    /// * `user_select` - The user select string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting user select string length
    fn validate_user_select(&self, user_select: &str) -> Result<(), String> {
        if user_select.len() > MAX_USER_SELECT_LENGTH {
            return Err(format!("User select string exceeds maximum length of {} characters", MAX_USER_SELECT_LENGTH));
        }
        
        // Validate user select value
        UserSelect::from_str(user_select)?;
        
        Ok(())
    }

    /// Apply user select to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply user select to
    /// * `user_select` - The user select to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates user select string
    pub fn apply_user_select(&mut self, node: &mut TipTapNode, user_select: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate user select
        self.validate_user_select(user_select)?;

        // Apply user select to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("userSelect".to_string(), serde_json::Value::String(user_select.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "userSelect": user_select }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("User select application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("User select application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove user select from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove user select from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_user_select(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("userSelect");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("User select removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("User select removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get user select from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get user select from
    /// 
    /// # Returns
    /// Option containing the user select string or None
    pub fn get_user_select(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(user_select) = obj.get("userSelect") {
                    if let Some(s) = user_select.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has user select
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has user select, false otherwise
    pub fn has_user_select(&self, node: &TipTapNode) -> bool {
        self.get_user_select(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_user_select_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = UserSelectManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(UserSelectManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(UserSelectManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(UserSelectManager::max_user_select_length(), MAX_USER_SELECT_LENGTH);
    }

    #[test]
    fn test_user_select_variants() {
        assert_eq!(UserSelect::Auto.as_str(), "auto");
        assert_eq!(UserSelect::None.as_str(), "none");
        assert_eq!(UserSelect::Text.as_str(), "text");
        assert_eq!(UserSelect::Contain.as_str(), "contain");
        assert_eq!(UserSelect::All.as_str(), "all");
    }

    #[test]
    fn test_user_select_from_str() {
        assert!(matches!(UserSelect::from_str("auto"), Ok(UserSelect::Auto)));
        assert!(matches!(UserSelect::from_str("none"), Ok(UserSelect::None)));
        assert!(matches!(UserSelect::from_str("text"), Ok(UserSelect::Text)));
        assert!(matches!(UserSelect::from_str("contain"), Ok(UserSelect::Contain)));
        assert!(matches!(UserSelect::from_str("all"), Ok(UserSelect::All)));
        assert!(UserSelect::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_user_select() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = UserSelectManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_user_select(&mut node, "none");
        assert!(result.is_ok());
        assert!(manager.has_user_select(&node));
    }

    #[test]
    fn test_apply_user_select_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = UserSelectManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_user_select(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_user_select_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = UserSelectManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_user_select = "a".repeat(MAX_USER_SELECT_LENGTH + 1);
        let result = manager.apply_user_select(&mut node, &long_user_select);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_user_select() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = UserSelectManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "userSelect": "text" })),
            marks: None,
        };
        
        assert!(manager.has_user_select(&node));
        let result = manager.remove_user_select(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_user_select(&node));
    }

    #[test]
    fn test_get_user_select() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = UserSelectManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "userSelect": "all" })),
            marks: None,
        };
        
        let user_select = manager.get_user_select(&node);
        assert_eq!(user_select, Some("all".to_string()));
    }

    #[test]
    fn test_get_user_select_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = UserSelectManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let user_select = manager.get_user_select(&node);
        assert!(user_select.is_none());
    }

    #[test]
    fn test_has_user_select() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = UserSelectManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "userSelect": "contain" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_user_select(&node_with));
        assert!(!manager.has_user_select(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = UserSelectManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_user_select(&mut node, "none").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = UserSelectManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_user_select(&mut node, "none").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = UserSelectManager::new(config_service);
        
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
        let mut manager = UserSelectManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
