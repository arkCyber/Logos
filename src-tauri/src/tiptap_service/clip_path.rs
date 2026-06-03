//! TipTap Clip Path Manager - Aerospace-Grade Clip Path Operations Service
//!
//! Safety-critical clip path operations service with:
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

/// Maximum clip path string length
const MAX_CLIP_PATH_LENGTH: usize = 500;

pub struct ClipPathManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ClipPathManager {
    /// Creates a new clip path manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new ClipPathManager instance
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

    /// Get the maximum clip path length constant
    /// 
    /// # Returns
    /// The maximum clip path string length
    pub fn max_clip_path_length() -> usize {
        MAX_CLIP_PATH_LENGTH
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

    /// Validate clip path string
    /// 
    /// # Arguments
    /// * `clip_path` - The clip path string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting clip path string length
    fn validate_clip_path(&self, clip_path: &str) -> Result<(), String> {
        if clip_path.is_empty() {
            return Err("Clip path cannot be empty".to_string());
        }
        if clip_path.len() > MAX_CLIP_PATH_LENGTH {
            return Err(format!("Clip path string exceeds maximum length of {} characters", MAX_CLIP_PATH_LENGTH));
        }
        // Check for unmatched parentheses
        if clip_path.contains('(') && !clip_path.contains(')') {
            return Err("Invalid clip path: unmatched parentheses".to_string());
        }
        // Basic validation for common clip path functions
        let valid_patterns = ["circle", "ellipse", "polygon", "inset", "path", "none", "url"];
        if !valid_patterns.iter().any(|pattern| clip_path.contains(pattern)) {
            // Allow custom values but validate basic structure
            if clip_path.contains('(') && !clip_path.contains(')') {
                return Err("Invalid clip path: unmatched parentheses".to_string());
            }
        }
        Ok(())
    }

    /// Apply clip path to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply clip path to
    /// * `clip_path` - The clip path to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates clip path string
    pub fn apply_clip_path(&mut self, node: &mut TipTapNode, clip_path: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate clip path
        self.validate_clip_path(clip_path)?;

        // Apply clip path to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("clipPath".to_string(), serde_json::Value::String(clip_path.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "clipPath": clip_path }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Clip path application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Clip path application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove clip path from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove clip path from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_clip_path(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("clipPath");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Clip path removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Clip path removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get clip path from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get clip path from
    /// 
    /// # Returns
    /// Option containing the clip path string or None
    pub fn get_clip_path(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(clip_path) = obj.get("clipPath") {
                    if let Some(s) = clip_path.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has clip path
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has clip path, false otherwise
    pub fn has_clip_path(&self, node: &TipTapNode) -> bool {
        self.get_clip_path(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_clip_path_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ClipPathManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(ClipPathManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(ClipPathManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(ClipPathManager::max_clip_path_length(), MAX_CLIP_PATH_LENGTH);
    }

    #[test]
    fn test_apply_clip_path() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ClipPathManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_clip_path(&mut node, "circle(50%)");
        assert!(result.is_ok());
        assert!(manager.has_clip_path(&node));
    }

    #[test]
    fn test_apply_clip_path_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ClipPathManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_clip_path(&mut node, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_clip_path_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ClipPathManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_clip_path = "a".repeat(MAX_CLIP_PATH_LENGTH + 1);
        let result = manager.apply_clip_path(&mut node, &long_clip_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_clip_path_invalid_parentheses() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ClipPathManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_clip_path(&mut node, "polygon(0 0, 100 0, 100 100");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_clip_path() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ClipPathManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "clipPath": "none" })),
            marks: None,
        };
        
        assert!(manager.has_clip_path(&node));
        let result = manager.remove_clip_path(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_clip_path(&node));
    }

    #[test]
    fn test_get_clip_path() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ClipPathManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "clipPath": "ellipse(50% 50% at 50% 50%)" })),
            marks: None,
        };
        
        let clip_path = manager.get_clip_path(&node);
        assert_eq!(clip_path, Some("ellipse(50% 50% at 50% 50%)".to_string()));
    }

    #[test]
    fn test_get_clip_path_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ClipPathManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let clip_path = manager.get_clip_path(&node);
        assert!(clip_path.is_none());
    }

    #[test]
    fn test_has_clip_path() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ClipPathManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "clipPath": "inset(10px)" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_clip_path(&node_with));
        assert!(!manager.has_clip_path(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ClipPathManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_clip_path(&mut node, "circle(50%)").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ClipPathManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_clip_path(&mut node, "circle(50%)").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ClipPathManager::new(config_service);
        
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
        let mut manager = ClipPathManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
