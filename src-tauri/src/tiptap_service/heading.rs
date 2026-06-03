//! TipTap Heading Manager - Aerospace-Grade Heading Operations Service
//!
//! Safety-critical heading operations service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Comprehensive error handling
//! - Performance monitoring
//! - Security hardening
//! - Fault tolerance and error recovery

use serde::{Deserialize, Serialize};
use std::time::Instant;
use crate::error_handling::{ErrorContext, ErrorSeverity};
use crate::config_service::ExportConfigService;
use std::sync::Arc;
use super::editor::{TipTapNode, NodeType};

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 50;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 200;

/// Maximum heading level
const MAX_HEADING_LEVEL: usize = 6;

/// Minimum heading level
const MIN_HEADING_LEVEL: usize = 1;

/// Maximum heading text length
const MAX_HEADING_TEXT_LENGTH: usize = 10000;

/// Heading attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingAttributes {
    pub level: usize,
    pub alignment: Option<String>,
}

impl Default for HeadingAttributes {
    fn default() -> Self {
        Self {
            level: 1,
            alignment: None,
        }
    }
}

pub struct HeadingManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl HeadingManager {
    /// Creates a new heading manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new HeadingManager instance
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

    /// Get the maximum heading level constant
    /// 
    /// # Returns
    /// The maximum heading level
    pub fn max_heading_level() -> usize {
        MAX_HEADING_LEVEL
    }

    /// Get the minimum heading level constant
    /// 
    /// # Returns
    /// The minimum heading level
    pub fn min_heading_level() -> usize {
        MIN_HEADING_LEVEL
    }

    /// Get the maximum heading text length constant
    /// 
    /// # Returns
    /// The maximum text length for headings
    pub fn max_heading_text_length() -> usize {
        MAX_HEADING_TEXT_LENGTH
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

    /// Validate heading level
    /// 
    /// # Arguments
    /// * `level` - The heading level to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Ensures heading level is within valid range
    fn validate_heading_level(&self, level: usize) -> Result<(), String> {
        if level < MIN_HEADING_LEVEL {
            return Err(format!("Heading level must be at least {}", MIN_HEADING_LEVEL));
        }
        if level > MAX_HEADING_LEVEL {
            return Err(format!("Heading level cannot exceed {}", MAX_HEADING_LEVEL));
        }
        Ok(())
    }

    /// Validate heading text length
    /// 
    /// # Arguments
    /// * `text` - The text to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting text length
    fn validate_heading_text(&self, text: &str) -> Result<(), String> {
        if text.len() > MAX_HEADING_TEXT_LENGTH {
            return Err(format!("Heading text exceeds maximum length of {} characters", MAX_HEADING_TEXT_LENGTH));
        }
        Ok(())
    }

    /// Create a heading node
    /// 
    /// # Arguments
    /// * `text` - The heading text
    /// * `level` - The heading level (1-6)
    /// * `attributes` - Optional heading attributes
    /// 
    /// # Returns
    /// Result containing the heading node or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates heading level and text length
    pub fn create_heading(&mut self, text: &str, level: usize, attributes: Option<HeadingAttributes>) -> Result<TipTapNode, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate level
        self.validate_heading_level(level)?;

        // Validate text length
        self.validate_heading_text(text)?;

        let attrs_json = if let Some(attrs) = attributes {
            Some(serde_json::to_value(attrs).map_err(|e| {
                let error = format!("Failed to serialize heading attributes: {}", e);
                self.record_error("SERIALIZE_ERROR", &error, "create_heading");
                error
            })?)
        } else {
            Some(serde_json::json!({ "level": level }))
        };

        let heading_node = TipTapNode {
            node_type: NodeType::Heading,
            content: Some(vec![TipTapNode {
                node_type: NodeType::Text,
                content: None,
                text: Some(text.to_string()),
                attrs: None,
                marks: None,
            }]),
            text: None,
            attrs: attrs_json,
            marks: None,
        };

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Heading creation CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Heading creation performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(heading_node)
    }

    /// Update heading text
    /// 
    /// # Arguments
    /// * `heading_node` - The heading node to update
    /// * `new_text` - The new heading text
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn update_text(&mut self, heading_node: &mut TipTapNode, new_text: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate text length
        self.validate_heading_text(new_text)?;

        if let Some(ref mut content) = heading_node.content {
            if let Some(ref mut text_node) = content.first_mut() {
                text_node.text = Some(new_text.to_string());
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Heading text update CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Heading text update performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Update heading level
    /// 
    /// # Arguments
    /// * `heading_node` - The heading node to update
    /// * `new_level` - The new heading level
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn update_level(&mut self, heading_node: &mut TipTapNode, new_level: usize) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate level
        self.validate_heading_level(new_level)?;

        if let Some(ref mut attrs) = heading_node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("level".to_string(), serde_json::Value::Number(new_level.into()));
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Heading level update CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Heading level update performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Update heading attributes
    /// 
    /// # Arguments
    /// * `heading_node` - The heading node to update
    /// * `attributes` - The new attributes
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn update_attributes(&mut self, heading_node: &mut TipTapNode, attributes: HeadingAttributes) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate level
        self.validate_heading_level(attributes.level)?;

        let attrs_json = serde_json::to_value(&attributes).map_err(|e| {
            let error = format!("Failed to serialize heading attributes: {}", e);
            self.record_error("SERIALIZE_ERROR", &error, "update_attributes");
            error
        })?;

        heading_node.attrs = Some(attrs_json);

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Heading attributes update CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Heading attributes update performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_heading_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = HeadingManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(HeadingManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(HeadingManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(HeadingManager::max_heading_level(), MAX_HEADING_LEVEL);
        assert_eq!(HeadingManager::min_heading_level(), MIN_HEADING_LEVEL);
        assert_eq!(HeadingManager::max_heading_text_length(), MAX_HEADING_TEXT_LENGTH);
    }

    #[test]
    fn test_create_heading() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HeadingManager::new(config_service);
        
        let result = manager.create_heading("Test Heading", 1, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_heading_level_6() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HeadingManager::new(config_service);
        
        let result = manager.create_heading("Test Heading", 6, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_heading_level_too_high() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HeadingManager::new(config_service);
        
        let result = manager.create_heading("Test Heading", 7, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_heading_level_too_low() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HeadingManager::new(config_service);
        
        let result = manager.create_heading("Test Heading", 0, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_heading_text_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HeadingManager::new(config_service);
        
        let long_text = "a".repeat(MAX_HEADING_TEXT_LENGTH + 1);
        let result = manager.create_heading(&long_text, 1, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_heading_with_attributes() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HeadingManager::new(config_service);
        
        let attributes = HeadingAttributes {
            level: 2,
            alignment: Some("center".to_string()),
        };
        let result = manager.create_heading("Test Heading", 2, Some(attributes));
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_text() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HeadingManager::new(config_service);
        
        let mut heading_node = manager.create_heading("Old Text", 1, None).unwrap();
        let result = manager.update_text(&mut heading_node, "New Text");
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_text_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HeadingManager::new(config_service);
        
        let mut heading_node = manager.create_heading("Old Text", 1, None).unwrap();
        let long_text = "a".repeat(MAX_HEADING_TEXT_LENGTH + 1);
        let result = manager.update_text(&mut heading_node, &long_text);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_level() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HeadingManager::new(config_service);
        
        let mut heading_node = manager.create_heading("Test", 1, None).unwrap();
        let result = manager.update_level(&mut heading_node, 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_level_too_high() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HeadingManager::new(config_service);
        
        let mut heading_node = manager.create_heading("Test", 1, None).unwrap();
        let result = manager.update_level(&mut heading_node, 7);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_attributes() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HeadingManager::new(config_service);
        
        let mut heading_node = manager.create_heading("Test", 1, None).unwrap();
        let attributes = HeadingAttributes {
            level: 3,
            alignment: Some("right".to_string()),
        };
        let result = manager.update_attributes(&mut heading_node, attributes);
        assert!(result.is_ok());
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HeadingManager::new(config_service);
        
        manager.create_heading("Test", 1, None).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HeadingManager::new(config_service);
        
        manager.create_heading("Test", 1, None).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HeadingManager::new(config_service);
        
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
        let mut manager = HeadingManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }

    #[test]
    fn test_heading_attributes_default() {
        let attrs = HeadingAttributes::default();
        assert_eq!(attrs.level, 1);
        assert!(attrs.alignment.is_none());
    }
}
