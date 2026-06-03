//! TipTap Word Spacing Manager - Aerospace-Grade Word Spacing Operations Service
//!
//! Safety-critical word spacing operations service with:
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

/// Minimum word spacing (in pixels)
const MIN_WORD_SPACING: f64 = -10.0;

/// Maximum word spacing (in pixels)
const MAX_WORD_SPACING: f64 = 50.0;

pub struct WordSpacingManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl WordSpacingManager {
    /// Creates a new word spacing manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new WordSpacingManager instance
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

    /// Get the minimum word spacing constant
    /// 
    /// # Returns
    /// The minimum word spacing in pixels
    pub fn min_word_spacing() -> f64 {
        MIN_WORD_SPACING
    }

    /// Get the maximum word spacing constant
    /// 
    /// # Returns
    /// The maximum word spacing in pixels
    pub fn max_word_spacing() -> f64 {
        MAX_WORD_SPACING
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

    /// Validate word spacing
    /// 
    /// # Arguments
    /// * `spacing` - The word spacing to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Ensures word spacing is within valid range to prevent rendering issues
    fn validate_word_spacing(&self, spacing: f64) -> Result<(), String> {
        if spacing < MIN_WORD_SPACING {
            return Err(format!("Word spacing must be at least {}", MIN_WORD_SPACING));
        }
        if spacing > MAX_WORD_SPACING {
            return Err(format!("Word spacing cannot exceed {}", MAX_WORD_SPACING));
        }
        if !spacing.is_finite() {
            return Err("Word spacing must be a finite number".to_string());
        }
        Ok(())
    }

    /// Apply word spacing to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply word spacing to
    /// * `spacing` - The word spacing in pixels
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates word spacing
    pub fn apply_word_spacing(&mut self, node: &mut TipTapNode, spacing: f64) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate word spacing
        self.validate_word_spacing(spacing)?;

        // Apply word spacing to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("wordSpacing".to_string(), serde_json::json!(spacing));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "wordSpacing": spacing }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Word spacing application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Word spacing application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove word spacing from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove word spacing from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_word_spacing(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("wordSpacing");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Word spacing removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Word spacing removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get word spacing from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get word spacing from
    /// 
    /// # Returns
    /// Option containing the word spacing or None
    pub fn get_word_spacing(&self, node: &TipTapNode) -> Option<f64> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(spacing) = obj.get("wordSpacing") {
                    if let Some(n) = spacing.as_f64() {
                        return Some(n);
                    }
                }
            }
        }
        None
    }

    /// Check if node has word spacing
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has word spacing, false otherwise
    pub fn has_word_spacing(&self, node: &TipTapNode) -> bool {
        self.get_word_spacing(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_word_spacing_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = WordSpacingManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(WordSpacingManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(WordSpacingManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_min_max_constants() {
        assert_eq!(WordSpacingManager::min_word_spacing(), MIN_WORD_SPACING);
        assert_eq!(WordSpacingManager::max_word_spacing(), MAX_WORD_SPACING);
    }

    #[test]
    fn test_apply_word_spacing() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WordSpacingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_word_spacing(&mut node, 2.0);
        assert!(result.is_ok());
        assert!(manager.has_word_spacing(&node));
    }

    #[test]
    fn test_apply_word_spacing_too_small() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WordSpacingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_word_spacing(&mut node, MIN_WORD_SPACING - 0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_word_spacing_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WordSpacingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_word_spacing(&mut node, MAX_WORD_SPACING + 0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_word_spacing_infinite() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WordSpacingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_word_spacing(&mut node, f64::INFINITY);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_word_spacing() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WordSpacingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "wordSpacing": 2.0 })),
            marks: None,
        };
        
        assert!(manager.has_word_spacing(&node));
        let result = manager.remove_word_spacing(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_word_spacing(&node));
    }

    #[test]
    fn test_get_word_spacing() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WordSpacingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_word_spacing(&mut node, 3.0).unwrap();
        let spacing = manager.get_word_spacing(&node);
        assert_eq!(spacing, Some(3.0));
    }

    #[test]
    fn test_get_word_spacing_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = WordSpacingManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let spacing = manager.get_word_spacing(&node);
        assert!(spacing.is_none());
    }

    #[test]
    fn test_has_word_spacing() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WordSpacingManager::new(config_service);
        
        let mut node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_word_spacing(&mut node_with, 1.5).unwrap();
        
        assert!(manager.has_word_spacing(&node_with));
        assert!(!manager.has_word_spacing(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WordSpacingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_word_spacing(&mut node, 2.0).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WordSpacingManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_word_spacing(&mut node, 2.0).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = WordSpacingManager::new(config_service);
        
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
        let mut manager = WordSpacingManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
