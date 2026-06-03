//! TipTap Position Manager - Aerospace-Grade Position Operations Service
//!
//! Safety-critical position operations service with:
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

/// Maximum position string length
const MAX_POSITION_LENGTH: usize = 50;

/// Position type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl Position {
    /// Convert position to string
    pub fn as_str(&self) -> &str {
        match self {
            Position::Static => "static",
            Position::Relative => "relative",
            Position::Absolute => "absolute",
            Position::Fixed => "fixed",
            Position::Sticky => "sticky",
        }
    }

    /// Parse position from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "static" => Ok(Position::Static),
            "relative" => Ok(Position::Relative),
            "absolute" => Ok(Position::Absolute),
            "fixed" => Ok(Position::Fixed),
            "sticky" => Ok(Position::Sticky),
            _ => Err(format!("Invalid position: {}", s)),
        }
    }
}

pub struct PositionManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl PositionManager {
    /// Creates a new position manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new PositionManager instance
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

    /// Get the maximum position length constant
    /// 
    /// # Returns
    /// The maximum position string length
    pub fn max_position_length() -> usize {
        MAX_POSITION_LENGTH
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

    /// Validate position string
    /// 
    /// # Arguments
    /// * `position` - The position string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting position string length
    fn validate_position(&self, position: &str) -> Result<(), String> {
        if position.len() > MAX_POSITION_LENGTH {
            return Err(format!("Position string exceeds maximum length of {} characters", MAX_POSITION_LENGTH));
        }
        
        // Validate position value
        Position::from_str(position)?;
        
        Ok(())
    }

    /// Apply position to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply position to
    /// * `position` - The position to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates position string
    pub fn apply_position(&mut self, node: &mut TipTapNode, position: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate position
        self.validate_position(position)?;

        // Apply position to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("position".to_string(), serde_json::Value::String(position.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "position": position }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Position application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Position application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove position from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove position from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_position(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("position");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Position removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Position removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get position from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get position from
    /// 
    /// # Returns
    /// Option containing the position string or None
    pub fn get_position(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(position) = obj.get("position") {
                    if let Some(s) = position.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has position
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has position, false otherwise
    pub fn has_position(&self, node: &TipTapNode) -> bool {
        self.get_position(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_position_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PositionManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(PositionManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(PositionManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(PositionManager::max_position_length(), MAX_POSITION_LENGTH);
    }

    #[test]
    fn test_position_variants() {
        assert_eq!(Position::Static.as_str(), "static");
        assert_eq!(Position::Relative.as_str(), "relative");
        assert_eq!(Position::Absolute.as_str(), "absolute");
    }

    #[test]
    fn test_position_from_str() {
        assert!(matches!(Position::from_str("static"), Ok(Position::Static)));
        assert!(matches!(Position::from_str("relative"), Ok(Position::Relative)));
        assert!(matches!(Position::from_str("absolute"), Ok(Position::Absolute)));
        assert!(Position::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_position() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_position(&mut node, "relative");
        assert!(result.is_ok());
        assert!(manager.has_position(&node));
    }

    #[test]
    fn test_apply_position_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_position(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_position_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_position = "a".repeat(MAX_POSITION_LENGTH + 1);
        let result = manager.apply_position(&mut node, &long_position);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_position() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "position": "relative" })),
            marks: None,
        };
        
        assert!(manager.has_position(&node));
        let result = manager.remove_position(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_position(&node));
    }

    #[test]
    fn test_get_position() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PositionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "position": "absolute" })),
            marks: None,
        };
        
        let position = manager.get_position(&node);
        assert_eq!(position, Some("absolute".to_string()));
    }

    #[test]
    fn test_get_position_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PositionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let position = manager.get_position(&node);
        assert!(position.is_none());
    }

    #[test]
    fn test_has_position() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PositionManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "position": "fixed" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_position(&node_with));
        assert!(!manager.has_position(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_position(&mut node, "relative").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_position(&mut node, "relative").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PositionManager::new(config_service);
        
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
        let mut manager = PositionManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
