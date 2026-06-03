//! TipTap Cursor Manager - Aerospace-Grade Cursor Operations Service
//!
//! Safety-critical cursor operations service with:
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

/// Maximum cursor string length
const MAX_CURSOR_LENGTH: usize = 50;

/// Cursor type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cursor {
    Auto,
    Default,
    Pointer,
    Move,
    Text,
    Wait,
    Help,
    Crosshair,
    NotAllowed,
    Grab,
    Grabbing,
}

impl Cursor {
    /// Convert cursor to string
    pub fn as_str(&self) -> &str {
        match self {
            Cursor::Auto => "auto",
            Cursor::Default => "default",
            Cursor::Pointer => "pointer",
            Cursor::Move => "move",
            Cursor::Text => "text",
            Cursor::Wait => "wait",
            Cursor::Help => "help",
            Cursor::Crosshair => "crosshair",
            Cursor::NotAllowed => "not-allowed",
            Cursor::Grab => "grab",
            Cursor::Grabbing => "grabbing",
        }
    }

    /// Parse cursor from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(Cursor::Auto),
            "default" => Ok(Cursor::Default),
            "pointer" => Ok(Cursor::Pointer),
            "move" => Ok(Cursor::Move),
            "text" => Ok(Cursor::Text),
            "wait" => Ok(Cursor::Wait),
            "help" => Ok(Cursor::Help),
            "crosshair" => Ok(Cursor::Crosshair),
            "not-allowed" => Ok(Cursor::NotAllowed),
            "grab" => Ok(Cursor::Grab),
            "grabbing" => Ok(Cursor::Grabbing),
            _ => Err(format!("Invalid cursor: {}", s)),
        }
    }
}

pub struct CursorManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl CursorManager {
    /// Creates a new cursor manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new CursorManager instance
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

    /// Get the maximum cursor length constant
    /// 
    /// # Returns
    /// The maximum cursor string length
    pub fn max_cursor_length() -> usize {
        MAX_CURSOR_LENGTH
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

    /// Validate cursor string
    /// 
    /// # Arguments
    /// * `cursor` - The cursor string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting cursor string length
    fn validate_cursor(&self, cursor: &str) -> Result<(), String> {
        if cursor.len() > MAX_CURSOR_LENGTH {
            return Err(format!("Cursor string exceeds maximum length of {} characters", MAX_CURSOR_LENGTH));
        }
        
        // Validate cursor value
        Cursor::from_str(cursor)?;
        
        Ok(())
    }

    /// Apply cursor to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply cursor to
    /// * `cursor` - The cursor to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates cursor string
    pub fn apply_cursor(&mut self, node: &mut TipTapNode, cursor: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate cursor
        self.validate_cursor(cursor)?;

        // Apply cursor to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("cursor".to_string(), serde_json::Value::String(cursor.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "cursor": cursor }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Cursor application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Cursor application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove cursor from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove cursor from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_cursor(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("cursor");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Cursor removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Cursor removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get cursor from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get cursor from
    /// 
    /// # Returns
    /// Option containing the cursor string or None
    pub fn get_cursor(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(cursor) = obj.get("cursor") {
                    if let Some(s) = cursor.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has cursor
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has cursor, false otherwise
    pub fn has_cursor(&self, node: &TipTapNode) -> bool {
        self.get_cursor(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_cursor_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = CursorManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(CursorManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(CursorManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(CursorManager::max_cursor_length(), MAX_CURSOR_LENGTH);
    }

    #[test]
    fn test_cursor_variants() {
        assert_eq!(Cursor::Auto.as_str(), "auto");
        assert_eq!(Cursor::Pointer.as_str(), "pointer");
        assert_eq!(Cursor::Text.as_str(), "text");
    }

    #[test]
    fn test_cursor_from_str() {
        assert!(matches!(Cursor::from_str("auto"), Ok(Cursor::Auto)));
        assert!(matches!(Cursor::from_str("pointer"), Ok(Cursor::Pointer)));
        assert!(matches!(Cursor::from_str("text"), Ok(Cursor::Text)));
        assert!(Cursor::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_cursor() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CursorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_cursor(&mut node, "pointer");
        assert!(result.is_ok());
        assert!(manager.has_cursor(&node));
    }

    #[test]
    fn test_apply_cursor_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CursorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_cursor(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_cursor_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CursorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_cursor = "a".repeat(MAX_CURSOR_LENGTH + 1);
        let result = manager.apply_cursor(&mut node, &long_cursor);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_cursor() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CursorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "cursor": "pointer" })),
            marks: None,
        };
        
        assert!(manager.has_cursor(&node));
        let result = manager.remove_cursor(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_cursor(&node));
    }

    #[test]
    fn test_get_cursor() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = CursorManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "cursor": "text" })),
            marks: None,
        };
        
        let cursor = manager.get_cursor(&node);
        assert_eq!(cursor, Some("text".to_string()));
    }

    #[test]
    fn test_get_cursor_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = CursorManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let cursor = manager.get_cursor(&node);
        assert!(cursor.is_none());
    }

    #[test]
    fn test_has_cursor() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = CursorManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "cursor": "grab" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_cursor(&node_with));
        assert!(!manager.has_cursor(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CursorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_cursor(&mut node, "pointer").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CursorManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_cursor(&mut node, "pointer").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CursorManager::new(config_service);
        
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
        let mut manager = CursorManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
