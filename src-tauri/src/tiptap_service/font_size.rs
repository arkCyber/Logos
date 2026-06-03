//! TipTap Font Size Manager - Aerospace-Grade Font Size Operations Service
//!
//! Safety-critical font size operations service with:
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

/// Minimum font size in pixels
const MIN_FONT_SIZE: usize = 8;

/// Maximum font size in pixels
const MAX_FONT_SIZE: usize = 72;

pub struct FontSizeManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FontSizeManager {
    /// Creates a new font size manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new FontSizeManager instance
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

    /// Get the minimum font size constant
    /// 
    /// # Returns
    /// The minimum font size in pixels
    pub fn min_font_size() -> usize {
        MIN_FONT_SIZE
    }

    /// Get the maximum font size constant
    /// 
    /// # Returns
    /// The maximum font size in pixels
    pub fn max_font_size() -> usize {
        MAX_FONT_SIZE
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

    /// Validate font size
    /// 
    /// # Arguments
    /// * `size` - The font size to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Ensures font size is within valid range to prevent rendering issues
    fn validate_font_size(&self, size: usize) -> Result<(), String> {
        if size < MIN_FONT_SIZE {
            return Err(format!("Font size must be at least {} pixels", MIN_FONT_SIZE));
        }
        if size > MAX_FONT_SIZE {
            return Err(format!("Font size cannot exceed {} pixels", MAX_FONT_SIZE));
        }
        Ok(())
    }

    /// Apply font size to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply font size to
    /// * `size` - The font size in pixels
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates font size
    pub fn apply_font_size(&mut self, node: &mut TipTapNode, size: usize) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate font size
        self.validate_font_size(size)?;

        // Apply font size to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("fontSize".to_string(), serde_json::Value::Number(size.into()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "fontSize": size }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font size application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font size application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove font size from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove font size from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_font_size(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("fontSize");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Font size removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Font size removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get font size from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get font size from
    /// 
    /// # Returns
    /// Option containing the font size or None
    pub fn get_font_size(&self, node: &TipTapNode) -> Option<usize> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(size) = obj.get("fontSize") {
                    if let Some(n) = size.as_u64() {
                        return Some(n as usize);
                    }
                }
            }
        }
        None
    }

    /// Check if node has font size
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has font size, false otherwise
    pub fn has_font_size(&self, node: &TipTapNode) -> bool {
        self.get_font_size(node).is_some()
    }

    /// Increase font size by a step
    /// 
    /// # Arguments
    /// * `node` - The node to increase font size for
    /// * `step` - The step size to increase by
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn increase_font_size(&mut self, node: &mut TipTapNode, step: usize) -> Result<(), String> {
        let current_size = self.get_font_size(node).unwrap_or(16); // Default to 16px
        let new_size = current_size.saturating_add(step);
        self.apply_font_size(node, new_size)
    }

    /// Decrease font size by a step
    /// 
    /// # Arguments
    /// * `node` - The node to decrease font size for
    /// * `step` - The step size to decrease by
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn decrease_font_size(&mut self, node: &mut TipTapNode, step: usize) -> Result<(), String> {
        let current_size = self.get_font_size(node).unwrap_or(16); // Default to 16px
        let new_size = current_size.saturating_sub(step);
        self.apply_font_size(node, new_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_font_size_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontSizeManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(FontSizeManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(FontSizeManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_min_max_constants() {
        assert_eq!(FontSizeManager::min_font_size(), MIN_FONT_SIZE);
        assert_eq!(FontSizeManager::max_font_size(), MAX_FONT_SIZE);
    }

    #[test]
    fn test_apply_font_size() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontSizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_font_size(&mut node, 16);
        assert!(result.is_ok());
        assert_eq!(manager.get_font_size(&node), Some(16));
    }

    #[test]
    fn test_apply_font_size_too_small() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontSizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_font_size(&mut node, MIN_FONT_SIZE - 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_font_size_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontSizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_font_size(&mut node, MAX_FONT_SIZE + 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_font_size() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontSizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontSize": 16 })),
            marks: None,
        };
        
        assert!(manager.has_font_size(&node));
        let result = manager.remove_font_size(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_font_size(&node));
    }

    #[test]
    fn test_get_font_size() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontSizeManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontSize": 24 })),
            marks: None,
        };
        
        let size = manager.get_font_size(&node);
        assert_eq!(size, Some(24));
    }

    #[test]
    fn test_get_font_size_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontSizeManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let size = manager.get_font_size(&node);
        assert!(size.is_none());
    }

    #[test]
    fn test_has_font_size() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FontSizeManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontSize": 18 })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_font_size(&node_with));
        assert!(!manager.has_font_size(&node_without));
    }

    #[test]
    fn test_increase_font_size() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontSizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontSize": 16 })),
            marks: None,
        };
        
        let result = manager.increase_font_size(&mut node, 4);
        assert!(result.is_ok());
        assert_eq!(manager.get_font_size(&node), Some(20));
    }

    #[test]
    fn test_decrease_font_size() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontSizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "fontSize": 20 })),
            marks: None,
        };
        
        let result = manager.decrease_font_size(&mut node, 4);
        assert!(result.is_ok());
        assert_eq!(manager.get_font_size(&node), Some(16));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontSizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_font_size(&mut node, 16).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontSizeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_font_size(&mut node, 16).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FontSizeManager::new(config_service);
        
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
        let mut manager = FontSizeManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
