//! TipTap Pointer Events Manager - Aerospace-Grade Pointer Events Operations Service
//!
//! Safety-critical pointer events operations service with:
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

/// Maximum pointer events string length
const MAX_POINTER_EVENTS_LENGTH: usize = 50;

/// Pointer events type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerEvents {
    Auto,
    None,
}

impl PointerEvents {
    /// Convert pointer events to string
    pub fn as_str(&self) -> &str {
        match self {
            PointerEvents::Auto => "auto",
            PointerEvents::None => "none",
        }
    }

    /// Parse pointer events from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(PointerEvents::Auto),
            "none" => Ok(PointerEvents::None),
            _ => Err(format!("Invalid pointer events: {}", s)),
        }
    }
}

pub struct PointerEventsManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl PointerEventsManager {
    /// Creates a new pointer events manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new PointerEventsManager instance
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

    /// Get the maximum pointer events length constant
    /// 
    /// # Returns
    /// The maximum pointer events string length
    pub fn max_pointer_events_length() -> usize {
        MAX_POINTER_EVENTS_LENGTH
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

    /// Validate pointer events string
    /// 
    /// # Arguments
    /// * `pointer_events` - The pointer events string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting pointer events string length
    fn validate_pointer_events(&self, pointer_events: &str) -> Result<(), String> {
        if pointer_events.len() > MAX_POINTER_EVENTS_LENGTH {
            return Err(format!("Pointer events string exceeds maximum length of {} characters", MAX_POINTER_EVENTS_LENGTH));
        }
        
        // Validate pointer events value
        PointerEvents::from_str(pointer_events)?;
        
        Ok(())
    }

    /// Apply pointer events to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply pointer events to
    /// * `pointer_events` - The pointer events to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates pointer events string
    pub fn apply_pointer_events(&mut self, node: &mut TipTapNode, pointer_events: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate pointer events
        self.validate_pointer_events(pointer_events)?;

        // Apply pointer events to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("pointerEvents".to_string(), serde_json::Value::String(pointer_events.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "pointerEvents": pointer_events }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Pointer events application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Pointer events application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove pointer events from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove pointer events from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_pointer_events(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("pointerEvents");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Pointer events removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Pointer events removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get pointer events from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get pointer events from
    /// 
    /// # Returns
    /// Option containing the pointer events string or None
    pub fn get_pointer_events(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(pointer_events) = obj.get("pointerEvents") {
                    if let Some(s) = pointer_events.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has pointer events
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has pointer events, false otherwise
    pub fn has_pointer_events(&self, node: &TipTapNode) -> bool {
        self.get_pointer_events(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_pointer_events_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PointerEventsManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(PointerEventsManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(PointerEventsManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(PointerEventsManager::max_pointer_events_length(), MAX_POINTER_EVENTS_LENGTH);
    }

    #[test]
    fn test_pointer_events_variants() {
        assert_eq!(PointerEvents::Auto.as_str(), "auto");
        assert_eq!(PointerEvents::None.as_str(), "none");
    }

    #[test]
    fn test_pointer_events_from_str() {
        assert!(matches!(PointerEvents::from_str("auto"), Ok(PointerEvents::Auto)));
        assert!(matches!(PointerEvents::from_str("none"), Ok(PointerEvents::None)));
        assert!(PointerEvents::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_pointer_events() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PointerEventsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_pointer_events(&mut node, "none");
        assert!(result.is_ok());
        assert!(manager.has_pointer_events(&node));
    }

    #[test]
    fn test_apply_pointer_events_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PointerEventsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_pointer_events(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_pointer_events_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PointerEventsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_pointer_events = "a".repeat(MAX_POINTER_EVENTS_LENGTH + 1);
        let result = manager.apply_pointer_events(&mut node, &long_pointer_events);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_pointer_events() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PointerEventsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "pointerEvents": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_pointer_events(&node));
        let result = manager.remove_pointer_events(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_pointer_events(&node));
    }

    #[test]
    fn test_get_pointer_events() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PointerEventsManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "pointerEvents": "none" })),
            marks: None,
        };
        
        let pointer_events = manager.get_pointer_events(&node);
        assert_eq!(pointer_events, Some("none".to_string()));
    }

    #[test]
    fn test_get_pointer_events_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PointerEventsManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let pointer_events = manager.get_pointer_events(&node);
        assert!(pointer_events.is_none());
    }

    #[test]
    fn test_has_pointer_events() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PointerEventsManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "pointerEvents": "auto" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_pointer_events(&node_with));
        assert!(!manager.has_pointer_events(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PointerEventsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_pointer_events(&mut node, "none").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PointerEventsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_pointer_events(&mut node, "none").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PointerEventsManager::new(config_service);
        
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
        let mut manager = PointerEventsManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
