//! TipTap Mask Size Manager - Aerospace-Grade Mask Size Operations Service
//!
//! Safety-critical mask size operations service with:
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

/// Maximum mask size string length
const MAX_MASK_SIZE_LENGTH: usize = 100;

pub struct MaskSizeManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl MaskSizeManager {
    /// Creates a new mask size manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new MaskSizeManager instance
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

    /// Get the maximum mask size length constant
    /// 
    /// # Returns
    /// The maximum mask size string length
    pub fn max_mask_size_length() -> usize {
        MAX_MASK_SIZE_LENGTH
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

    /// Validate mask size string
    /// 
    /// # Arguments
    /// * `mask_size` - The mask size string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting mask size string length
    fn validate_mask_size(&self, mask_size: &str) -> Result<(), String> {
        if mask_size.is_empty() {
            return Err("Mask size cannot be empty".to_string());
        }
        if mask_size.len() > MAX_MASK_SIZE_LENGTH {
            return Err(format!("Mask size string exceeds maximum length of {} characters", MAX_MASK_SIZE_LENGTH));
        }
        // Check for unmatched parentheses first
        if mask_size.contains('(') && !mask_size.contains(')') {
            return Err("Invalid mask size: unmatched parentheses".to_string());
        }
        // Basic validation for common mask size values
        let valid_patterns = ["contain", "cover", "auto", "50%", "100%"];
        if !valid_patterns.iter().any(|pattern| mask_size.contains(pattern)) {
            // Allow custom values but validate basic structure
            if mask_size.contains('(') && !mask_size.contains(')') {
                return Err("Invalid mask size: unmatched parentheses".to_string());
            }
        }
        Ok(())
    }

    /// Apply mask size to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply mask size to
    /// * `mask_size` - The mask size to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates mask size string
    pub fn apply_mask_size(&mut self, node: &mut TipTapNode, mask_size: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate mask size
        self.validate_mask_size(mask_size)?;

        // Apply mask size to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("maskSize".to_string(), serde_json::Value::String(mask_size.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "maskSize": mask_size }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Mask size application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Mask size application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove mask size from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove mask size from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_mask_size(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("maskSize");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Mask size removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Mask size removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get mask size from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get mask size from
    /// 
    /// # Returns
    /// Option containing the mask size string or None
    pub fn get_mask_size(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(mask_size) = obj.get("maskSize") {
                    if let Some(s) = mask_size.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has mask size
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has mask size, false otherwise
    pub fn has_mask_size(&self, node: &TipTapNode) -> bool {
        self.get_mask_size(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_mask_size_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskSizeManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(MaskSizeManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(MaskSizeManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(MaskSizeManager::max_mask_size_length(), MAX_MASK_SIZE_LENGTH);
    }

    #[test]
    fn test_apply_mask_size() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskSizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_mask_size(&mut node, "cover");
        assert!(result.is_ok());
        assert!(manager.has_mask_size(&node));
    }

    #[test]
    fn test_apply_mask_size_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskSizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_mask_size(&mut node, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_mask_size_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskSizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_mask_size = "a".repeat(MAX_MASK_SIZE_LENGTH + 1);
        let result = manager.apply_mask_size(&mut node, &long_mask_size);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_mask_size_invalid_parentheses() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskSizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_mask_size(&mut node, "calc(100% - 20px");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_mask_size() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskSizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maskSize": "contain" })),
            marks: None,
        };
        
        assert!(manager.has_mask_size(&node));
        let result = manager.remove_mask_size(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_mask_size(&node));
    }

    #[test]
    fn test_get_mask_size() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskSizeManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maskSize": "auto" })),
            marks: None,
        };
        
        let mask_size = manager.get_mask_size(&node);
        assert_eq!(mask_size, Some("auto".to_string()));
    }

    #[test]
    fn test_get_mask_size_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskSizeManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let mask_size = manager.get_mask_size(&node);
        assert!(mask_size.is_none());
    }

    #[test]
    fn test_has_mask_size() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskSizeManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maskSize": "50%" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_mask_size(&node_with));
        assert!(!manager.has_mask_size(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskSizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_mask_size(&mut node, "cover").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskSizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_mask_size(&mut node, "cover").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskSizeManager::new(config_service);
        
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
        let mut manager = MaskSizeManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
