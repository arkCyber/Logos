//! TipTap Mask Position Manager - Aerospace-Grade Mask Position Operations Service
//!
//! Safety-critical mask position operations service with:
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

/// Maximum mask position string length
const MAX_MASK_POSITION_LENGTH: usize = 100;

pub struct MaskPositionManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl MaskPositionManager {
    /// Creates a new mask position manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new MaskPositionManager instance
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

    /// Get the maximum mask position length constant
    /// 
    /// # Returns
    /// The maximum mask position string length
    pub fn max_mask_position_length() -> usize {
        MAX_MASK_POSITION_LENGTH
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

    /// Validate mask position string
    /// 
    /// # Arguments
    /// * `mask_position` - The mask position string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting mask position string length
    fn validate_mask_position(&self, mask_position: &str) -> Result<(), String> {
        if mask_position.is_empty() {
            return Err("Mask position cannot be empty".to_string());
        }
        if mask_position.len() > MAX_MASK_POSITION_LENGTH {
            return Err(format!("Mask position string exceeds maximum length of {} characters", MAX_MASK_POSITION_LENGTH));
        }
        // Basic validation for common mask position values
        let valid_patterns = ["center", "top", "bottom", "left", "right", "top left", "top right", "bottom left", "bottom right"];
        if !valid_patterns.iter().any(|pattern| mask_position.contains(pattern)) {
            // Allow custom values but validate basic structure
            if mask_position.contains('(') && !mask_position.contains(')') {
                return Err("Invalid mask position: unmatched parentheses".to_string());
            }
        }
        Ok(())
    }

    /// Apply mask position to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply mask position to
    /// * `mask_position` - The mask position to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates mask position string
    pub fn apply_mask_position(&mut self, node: &mut TipTapNode, mask_position: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate mask position
        self.validate_mask_position(mask_position)?;

        // Apply mask position to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("maskPosition".to_string(), serde_json::Value::String(mask_position.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "maskPosition": mask_position }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Mask position application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Mask position application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove mask position from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove mask position from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_mask_position(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("maskPosition");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Mask position removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Mask position removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get mask position from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get mask position from
    /// 
    /// # Returns
    /// Option containing the mask position string or None
    pub fn get_mask_position(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(mask_position) = obj.get("maskPosition") {
                    if let Some(s) = mask_position.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has mask position
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has mask position, false otherwise
    pub fn has_mask_position(&self, node: &TipTapNode) -> bool {
        self.get_mask_position(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_mask_position_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskPositionManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(MaskPositionManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(MaskPositionManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(MaskPositionManager::max_mask_position_length(), MAX_MASK_POSITION_LENGTH);
    }

    #[test]
    fn test_apply_mask_position() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskPositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_mask_position(&mut node, "center");
        assert!(result.is_ok());
        assert!(manager.has_mask_position(&node));
    }

    #[test]
    fn test_apply_mask_position_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskPositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_mask_position(&mut node, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_mask_position_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskPositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_mask_position = "a".repeat(MAX_MASK_POSITION_LENGTH + 1);
        let result = manager.apply_mask_position(&mut node, &long_mask_position);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_mask_position_invalid_parentheses() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskPositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_mask_position(&mut node, "calc(50% - 10px");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_mask_position() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskPositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maskPosition": "top left" })),
            marks: None,
        };
        
        assert!(manager.has_mask_position(&node));
        let result = manager.remove_mask_position(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_mask_position(&node));
    }

    #[test]
    fn test_get_mask_position() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskPositionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maskPosition": "bottom right" })),
            marks: None,
        };
        
        let mask_position = manager.get_mask_position(&node);
        assert_eq!(mask_position, Some("bottom right".to_string()));
    }

    #[test]
    fn test_get_mask_position_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskPositionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let mask_position = manager.get_mask_position(&node);
        assert!(mask_position.is_none());
    }

    #[test]
    fn test_has_mask_position() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskPositionManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maskPosition": "top" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_mask_position(&node_with));
        assert!(!manager.has_mask_position(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskPositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_mask_position(&mut node, "center").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskPositionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_mask_position(&mut node, "center").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskPositionManager::new(config_service);
        
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
        let mut manager = MaskPositionManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
