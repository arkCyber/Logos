//! TipTap Horizontal Rule Manager - Aerospace-Grade Horizontal Rule Operations Service
//!
//! Safety-critical horizontal rule operations service with:
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

/// Maximum horizontal rule depth to prevent stack overflow
const MAX_HR_DEPTH: usize = 10;

/// Horizontal rule attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorizontalRuleAttributes {
    pub style: Option<String>,
    pub color: Option<String>,
    pub thickness: Option<usize>,
}

impl Default for HorizontalRuleAttributes {
    fn default() -> Self {
        Self {
            style: None,
            color: None,
            thickness: None,
        }
    }
}

pub struct HorizontalRuleManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl HorizontalRuleManager {
    /// Creates a new horizontal rule manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new HorizontalRuleManager instance
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

    /// Get the maximum horizontal rule depth constant
    /// 
    /// # Returns
    /// The maximum horizontal rule depth
    pub fn max_hr_depth() -> usize {
        MAX_HR_DEPTH
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

    /// Validate horizontal rule depth
    /// 
    /// # Arguments
    /// * `depth` - The current horizontal rule depth
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents stack overflow by limiting depth
    fn validate_hr_depth(&self, depth: usize) -> Result<(), String> {
        if depth >= MAX_HR_DEPTH {
            return Err(format!("Horizontal rule depth exceeds maximum of {}", MAX_HR_DEPTH));
        }
        Ok(())
    }

    /// Validate thickness
    /// 
    /// # Arguments
    /// * `thickness` - The thickness to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    fn validate_thickness(&self, thickness: usize) -> Result<(), String> {
        if thickness == 0 {
            return Err("Thickness cannot be zero".to_string());
        }
        if thickness > 100 {
            return Err("Thickness cannot exceed 100 pixels".to_string());
        }
        Ok(())
    }

    /// Create a horizontal rule node
    /// 
    /// # Arguments
    /// * `attributes` - Optional horizontal rule attributes
    /// * `depth` - The nesting depth
    /// 
    /// # Returns
    /// Result containing the horizontal rule node or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates depth and thickness
    pub fn create_horizontal_rule(&mut self, attributes: Option<HorizontalRuleAttributes>, depth: usize) -> Result<TipTapNode, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate depth
        self.validate_hr_depth(depth)?;

        // Validate thickness if provided
        if let Some(ref attrs) = attributes {
            if let Some(thickness) = attrs.thickness {
                self.validate_thickness(thickness)?;
            }
        }

        let attrs_json = if let Some(attrs) = attributes {
            Some(serde_json::to_value(attrs).map_err(|e| {
                let error = format!("Failed to serialize horizontal rule attributes: {}", e);
                self.record_error("SERIALIZE_ERROR", &error, "create_horizontal_rule");
                error
            })?)
        } else {
            None
        };

        let hr_node = TipTapNode {
            node_type: NodeType::HorizontalRule,
            content: None,
            text: None,
            attrs: attrs_json,
            marks: None,
        };

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Horizontal rule creation CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Horizontal rule creation performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(hr_node)
    }

    /// Update horizontal rule attributes
    /// 
    /// # Arguments
    /// * `hr_node` - The horizontal rule node to update
    /// * `attributes` - The new attributes
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn update_attributes(&mut self, hr_node: &mut TipTapNode, attributes: HorizontalRuleAttributes) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate thickness if provided
        if let Some(thickness) = attributes.thickness {
            self.validate_thickness(thickness)?;
        }

        let attrs_json = serde_json::to_value(&attributes).map_err(|e| {
            let error = format!("Failed to serialize horizontal rule attributes: {}", e);
            self.record_error("SERIALIZE_ERROR", &error, "update_attributes");
            error
        })?;

        hr_node.attrs = Some(attrs_json);

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Horizontal rule attributes update CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Horizontal rule attributes update performance warning: took {}ms", elapsed.as_millis());
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
    fn test_horizontal_rule_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = HorizontalRuleManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(HorizontalRuleManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(HorizontalRuleManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(HorizontalRuleManager::max_hr_depth(), MAX_HR_DEPTH);
    }

    #[test]
    fn test_create_horizontal_rule() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HorizontalRuleManager::new(config_service);
        
        let result = manager.create_horizontal_rule(None, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_horizontal_rule_with_attributes() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HorizontalRuleManager::new(config_service);
        
        let attributes = HorizontalRuleAttributes {
            style: Some("solid".to_string()),
            color: Some("#000000".to_string()),
            thickness: Some(2),
        };
        let result = manager.create_horizontal_rule(Some(attributes), 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_horizontal_rule_too_deep() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HorizontalRuleManager::new(config_service);
        
        let result = manager.create_horizontal_rule(None, MAX_HR_DEPTH);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_horizontal_rule_thickness_zero() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HorizontalRuleManager::new(config_service);
        
        let attributes = HorizontalRuleAttributes {
            style: None,
            color: None,
            thickness: Some(0),
        };
        let result = manager.create_horizontal_rule(Some(attributes), 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_horizontal_rule_thickness_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HorizontalRuleManager::new(config_service);
        
        let attributes = HorizontalRuleAttributes {
            style: None,
            color: None,
            thickness: Some(101),
        };
        let result = manager.create_horizontal_rule(Some(attributes), 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_attributes() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HorizontalRuleManager::new(config_service);
        
        let mut hr_node = manager.create_horizontal_rule(None, 0).unwrap();
        let attributes = HorizontalRuleAttributes {
            style: Some("dashed".to_string()),
            color: Some("#ff0000".to_string()),
            thickness: Some(3),
        };
        let result = manager.update_attributes(&mut hr_node, attributes);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_attributes_thickness_zero() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HorizontalRuleManager::new(config_service);
        
        let mut hr_node = manager.create_horizontal_rule(None, 0).unwrap();
        let attributes = HorizontalRuleAttributes {
            style: None,
            color: None,
            thickness: Some(0),
        };
        let result = manager.update_attributes(&mut hr_node, attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HorizontalRuleManager::new(config_service);
        
        manager.create_horizontal_rule(None, 0).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HorizontalRuleManager::new(config_service);
        
        manager.create_horizontal_rule(None, 0).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HorizontalRuleManager::new(config_service);
        
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
        let mut manager = HorizontalRuleManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }

    #[test]
    fn test_horizontal_rule_attributes_default() {
        let attrs = HorizontalRuleAttributes::default();
        assert!(attrs.style.is_none());
        assert!(attrs.color.is_none());
        assert!(attrs.thickness.is_none());
    }
}
