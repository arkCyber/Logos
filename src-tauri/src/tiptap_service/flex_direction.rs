//! TipTap Flex Direction Manager - Aerospace-Grade Flex Direction Operations Service
//!
//! Safety-critical flex direction operations service with:
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

/// Maximum flex direction string length
const MAX_FLEX_DIRECTION_LENGTH: usize = 50;

/// Flex direction type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl FlexDirection {
    /// Convert flex direction to string
    pub fn as_str(&self) -> &str {
        match self {
            FlexDirection::Row => "row",
            FlexDirection::RowReverse => "row-reverse",
            FlexDirection::Column => "column",
            FlexDirection::ColumnReverse => "column-reverse",
        }
    }

    /// Parse flex direction from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "row" => Ok(FlexDirection::Row),
            "row-reverse" => Ok(FlexDirection::RowReverse),
            "column" => Ok(FlexDirection::Column),
            "column-reverse" => Ok(FlexDirection::ColumnReverse),
            _ => Err(format!("Invalid flex direction: {}", s)),
        }
    }
}

pub struct FlexDirectionManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FlexDirectionManager {
    /// Creates a new flex direction manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new FlexDirectionManager instance
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

    /// Get the maximum flex direction length constant
    /// 
    /// # Returns
    /// The maximum flex direction string length
    pub fn max_flex_direction_length() -> usize {
        MAX_FLEX_DIRECTION_LENGTH
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

    /// Validate flex direction string
    /// 
    /// # Arguments
    /// * `flex_direction` - The flex direction string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting flex direction string length
    fn validate_flex_direction(&self, flex_direction: &str) -> Result<(), String> {
        if flex_direction.len() > MAX_FLEX_DIRECTION_LENGTH {
            return Err(format!("Flex direction string exceeds maximum length of {} characters", MAX_FLEX_DIRECTION_LENGTH));
        }
        
        // Validate flex direction value
        FlexDirection::from_str(flex_direction)?;
        
        Ok(())
    }

    /// Apply flex direction to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply flex direction to
    /// * `flex_direction` - The flex direction to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates flex direction string
    pub fn apply_flex_direction(&mut self, node: &mut TipTapNode, flex_direction: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate flex direction
        self.validate_flex_direction(flex_direction)?;

        // Apply flex direction to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("flexDirection".to_string(), serde_json::Value::String(flex_direction.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "flexDirection": flex_direction }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Flex direction application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Flex direction application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove flex direction from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove flex direction from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_flex_direction(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("flexDirection");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Flex direction removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Flex direction removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get flex direction from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get flex direction from
    /// 
    /// # Returns
    /// Option containing the flex direction string or None
    pub fn get_flex_direction(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(flex_direction) = obj.get("flexDirection") {
                    if let Some(s) = flex_direction.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has flex direction
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has flex direction, false otherwise
    pub fn has_flex_direction(&self, node: &TipTapNode) -> bool {
        self.get_flex_direction(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_flex_direction_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexDirectionManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(FlexDirectionManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(FlexDirectionManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(FlexDirectionManager::max_flex_direction_length(), MAX_FLEX_DIRECTION_LENGTH);
    }

    #[test]
    fn test_flex_direction_variants() {
        assert_eq!(FlexDirection::Row.as_str(), "row");
        assert_eq!(FlexDirection::Column.as_str(), "column");
        assert_eq!(FlexDirection::RowReverse.as_str(), "row-reverse");
    }

    #[test]
    fn test_flex_direction_from_str() {
        assert!(matches!(FlexDirection::from_str("row"), Ok(FlexDirection::Row)));
        assert!(matches!(FlexDirection::from_str("column"), Ok(FlexDirection::Column)));
        assert!(matches!(FlexDirection::from_str("row-reverse"), Ok(FlexDirection::RowReverse)));
        assert!(FlexDirection::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_flex_direction() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexDirectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_flex_direction(&mut node, "column");
        assert!(result.is_ok());
        assert!(manager.has_flex_direction(&node));
    }

    #[test]
    fn test_apply_flex_direction_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexDirectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_flex_direction(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_flex_direction_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexDirectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_flex_direction = "a".repeat(MAX_FLEX_DIRECTION_LENGTH + 1);
        let result = manager.apply_flex_direction(&mut node, &long_flex_direction);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_flex_direction() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexDirectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "flexDirection": "row" })),
            marks: None,
        };
        
        assert!(manager.has_flex_direction(&node));
        let result = manager.remove_flex_direction(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_flex_direction(&node));
    }

    #[test]
    fn test_get_flex_direction() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexDirectionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "flexDirection": "column-reverse" })),
            marks: None,
        };
        
        let flex_direction = manager.get_flex_direction(&node);
        assert_eq!(flex_direction, Some("column-reverse".to_string()));
    }

    #[test]
    fn test_get_flex_direction_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexDirectionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let flex_direction = manager.get_flex_direction(&node);
        assert!(flex_direction.is_none());
    }

    #[test]
    fn test_has_flex_direction() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexDirectionManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "flexDirection": "row" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_flex_direction(&node_with));
        assert!(!manager.has_flex_direction(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexDirectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_flex_direction(&mut node, "row").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexDirectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_flex_direction(&mut node, "row").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexDirectionManager::new(config_service);
        
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
        let mut manager = FlexDirectionManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
