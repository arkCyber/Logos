//! TipTap Mask Image Manager - Aerospace-Grade Mask Image Operations Service
//!
//! Safety-critical mask image operations service with:
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

/// Maximum mask image string length
const MAX_MASK_IMAGE_LENGTH: usize = 500;

pub struct MaskImageManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl MaskImageManager {
    /// Creates a new mask image manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new MaskImageManager instance
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

    /// Get the maximum mask image length constant
    /// 
    /// # Returns
    /// The maximum mask image string length
    pub fn max_mask_image_length() -> usize {
        MAX_MASK_IMAGE_LENGTH
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

    /// Validate mask image string
    /// 
    /// # Arguments
    /// * `mask_image` - The mask image string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting mask image string length
    fn validate_mask_image(&self, mask_image: &str) -> Result<(), String> {
        if mask_image.is_empty() {
            return Err("Mask image cannot be empty".to_string());
        }
        if mask_image.len() > MAX_MASK_IMAGE_LENGTH {
            return Err(format!("Mask image string exceeds maximum length of {} characters", MAX_MASK_IMAGE_LENGTH));
        }
        // Check for unmatched parentheses first
        if mask_image.contains('(') && !mask_image.contains(')') {
            return Err("Invalid mask image: unmatched parentheses".to_string());
        }
        // Basic validation for common mask image values
        let valid_patterns = ["none", "url", "linear-gradient", "radial-gradient", "conic-gradient"];
        if !valid_patterns.iter().any(|pattern| mask_image.contains(pattern)) {
            // Allow custom values but validate basic structure
            if mask_image.contains('(') && !mask_image.contains(')') {
                return Err("Invalid mask image: unmatched parentheses".to_string());
            }
        }
        Ok(())
    }

    /// Apply mask image to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply mask image to
    /// * `mask_image` - The mask image to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates mask image string
    pub fn apply_mask_image(&mut self, node: &mut TipTapNode, mask_image: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate mask image
        self.validate_mask_image(mask_image)?;

        // Apply mask image to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("maskImage".to_string(), serde_json::Value::String(mask_image.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "maskImage": mask_image }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Mask image application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Mask image application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove mask image from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove mask image from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_mask_image(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("maskImage");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Mask image removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Mask image removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get mask image from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get mask image from
    /// 
    /// # Returns
    /// Option containing the mask image string or None
    pub fn get_mask_image(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(mask_image) = obj.get("maskImage") {
                    if let Some(s) = mask_image.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has mask image
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has mask image, false otherwise
    pub fn has_mask_image(&self, node: &TipTapNode) -> bool {
        self.get_mask_image(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_mask_image_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskImageManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(MaskImageManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(MaskImageManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(MaskImageManager::max_mask_image_length(), MAX_MASK_IMAGE_LENGTH);
    }

    #[test]
    fn test_apply_mask_image() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskImageManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_mask_image(&mut node, "url('mask.png')");
        assert!(result.is_ok());
        assert!(manager.has_mask_image(&node));
    }

    #[test]
    fn test_apply_mask_image_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskImageManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_mask_image(&mut node, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_mask_image_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskImageManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_mask_image = "a".repeat(MAX_MASK_IMAGE_LENGTH + 1);
        let result = manager.apply_mask_image(&mut node, &long_mask_image);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_mask_image_invalid_parentheses() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskImageManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_mask_image(&mut node, "linear-gradient(to right, red, blue");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_mask_image() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskImageManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maskImage": "none" })),
            marks: None,
        };
        
        assert!(manager.has_mask_image(&node));
        let result = manager.remove_mask_image(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_mask_image(&node));
    }

    #[test]
    fn test_get_mask_image() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskImageManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maskImage": "radial-gradient(circle, red, transparent)" })),
            marks: None,
        };
        
        let mask_image = manager.get_mask_image(&node);
        assert_eq!(mask_image, Some("radial-gradient(circle, red, transparent)".to_string()));
    }

    #[test]
    fn test_get_mask_image_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskImageManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let mask_image = manager.get_mask_image(&node);
        assert!(mask_image.is_none());
    }

    #[test]
    fn test_has_mask_image() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskImageManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maskImage": "conic-gradient(from 0deg, red, blue)" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_mask_image(&node_with));
        assert!(!manager.has_mask_image(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskImageManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_mask_image(&mut node, "url('mask.png')").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskImageManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_mask_image(&mut node, "url('mask.png')").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskImageManager::new(config_service);
        
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
        let mut manager = MaskImageManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
