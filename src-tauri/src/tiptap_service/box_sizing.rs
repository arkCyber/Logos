//! TipTap Box Sizing Manager - Aerospace-Grade Box Sizing Operations Service
//!
//! Safety-critical box sizing operations service with:
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

/// Maximum box sizing string length
const MAX_BOX_SIZING_LENGTH: usize = 50;

/// Box sizing type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
}

impl BoxSizing {
    /// Convert box sizing to string
    pub fn as_str(&self) -> &str {
        match self {
            BoxSizing::ContentBox => "content-box",
            BoxSizing::BorderBox => "border-box",
        }
    }

    /// Parse box sizing from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "content-box" => Ok(BoxSizing::ContentBox),
            "border-box" => Ok(BoxSizing::BorderBox),
            _ => Err(format!("Invalid box sizing: {}", s)),
        }
    }
}

pub struct BoxSizingManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl BoxSizingManager {
    /// Creates a new box sizing manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new BoxSizingManager instance
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

    /// Get the maximum box sizing length constant
    /// 
    /// # Returns
    /// The maximum box sizing string length
    pub fn max_box_sizing_length() -> usize {
        MAX_BOX_SIZING_LENGTH
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

    /// Validate box sizing string
    /// 
    /// # Arguments
    /// * `box_sizing` - The box sizing string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting box sizing string length
    fn validate_box_sizing(&self, box_sizing: &str) -> Result<(), String> {
        if box_sizing.len() > MAX_BOX_SIZING_LENGTH {
            return Err(format!("Box sizing string exceeds maximum length of {} characters", MAX_BOX_SIZING_LENGTH));
        }
        
        // Validate box sizing value
        BoxSizing::from_str(box_sizing)?;
        
        Ok(())
    }

    /// Apply box sizing to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply box sizing to
    /// * `box_sizing` - The box sizing to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates box sizing string
    pub fn apply_box_sizing(&mut self, node: &mut TipTapNode, box_sizing: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate box sizing
        self.validate_box_sizing(box_sizing)?;

        // Apply box sizing to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("boxSizing".to_string(), serde_json::Value::String(box_sizing.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "boxSizing": box_sizing }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Box sizing application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Box sizing application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove box sizing from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove box sizing from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_box_sizing(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("boxSizing");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Box sizing removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Box sizing removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get box sizing from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get box sizing from
    /// 
    /// # Returns
    /// Option containing the box sizing string or None
    pub fn get_box_sizing(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(box_sizing) = obj.get("boxSizing") {
                    if let Some(s) = box_sizing.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has box sizing
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has box sizing, false otherwise
    pub fn has_box_sizing(&self, node: &TipTapNode) -> bool {
        self.get_box_sizing(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_box_sizing_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BoxSizingManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(BoxSizingManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(BoxSizingManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(BoxSizingManager::max_box_sizing_length(), MAX_BOX_SIZING_LENGTH);
    }

    #[test]
    fn test_box_sizing_variants() {
        assert_eq!(BoxSizing::ContentBox.as_str(), "content-box");
        assert_eq!(BoxSizing::BorderBox.as_str(), "border-box");
    }

    #[test]
    fn test_box_sizing_from_str() {
        assert!(matches!(BoxSizing::from_str("content-box"), Ok(BoxSizing::ContentBox)));
        assert!(matches!(BoxSizing::from_str("border-box"), Ok(BoxSizing::BorderBox)));
        assert!(BoxSizing::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_box_sizing() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BoxSizingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_box_sizing(&mut node, "border-box");
        assert!(result.is_ok());
        assert!(manager.has_box_sizing(&node));
    }

    #[test]
    fn test_apply_box_sizing_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BoxSizingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_box_sizing(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_box_sizing_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BoxSizingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_box_sizing = "a".repeat(MAX_BOX_SIZING_LENGTH + 1);
        let result = manager.apply_box_sizing(&mut node, &long_box_sizing);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_box_sizing() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BoxSizingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "boxSizing": "content-box" })),
            marks: None,
        };
        
        assert!(manager.has_box_sizing(&node));
        let result = manager.remove_box_sizing(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_box_sizing(&node));
    }

    #[test]
    fn test_get_box_sizing() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BoxSizingManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "boxSizing": "border-box" })),
            marks: None,
        };
        
        let box_sizing = manager.get_box_sizing(&node);
        assert_eq!(box_sizing, Some("border-box".to_string()));
    }

    #[test]
    fn test_get_box_sizing_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BoxSizingManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let box_sizing = manager.get_box_sizing(&node);
        assert!(box_sizing.is_none());
    }

    #[test]
    fn test_has_box_sizing() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BoxSizingManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "boxSizing": "content-box" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_box_sizing(&node_with));
        assert!(!manager.has_box_sizing(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BoxSizingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_box_sizing(&mut node, "border-box").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BoxSizingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_box_sizing(&mut node, "border-box").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BoxSizingManager::new(config_service);
        
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
        let mut manager = BoxSizingManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
