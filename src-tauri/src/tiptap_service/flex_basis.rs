//! TipTap Flex Basis Manager - Aerospace-Grade Flex Basis Operations Service
//!
//! Safety-critical flex basis operations service with:
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

/// Maximum flex basis string length
const MAX_FLEX_BASIS_LENGTH: usize = 50;

/// Flex basis type
#[derive(Debug, Clone, PartialEq)]
pub enum FlexBasis {
    Auto,
    Content,
    Pixel(f64),
    Percent(f64),
}

impl FlexBasis {
    /// Convert flex basis to string
    pub fn as_str(&self) -> String {
        match self {
            FlexBasis::Auto => "auto".to_string(),
            FlexBasis::Content => "content".to_string(),
            FlexBasis::Pixel(value) => format!("{}px", value),
            FlexBasis::Percent(value) => format!("{}%", value),
        }
    }

    /// Parse flex basis from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        let s = s.trim().to_lowercase();
        if s == "auto" {
            return Ok(FlexBasis::Auto);
        }
        if s == "content" {
            return Ok(FlexBasis::Content);
        }
        if s.ends_with("px") {
            let value_str = s.trim_end_matches("px");
            let value: f64 = value_str.parse().map_err(|_| "Invalid pixel value".to_string())?;
            if value < 0.0 || !value.is_finite() {
                return Err("Invalid pixel value".to_string());
            }
            return Ok(FlexBasis::Pixel(value));
        }
        if s.ends_with("%") {
            let value_str = s.trim_end_matches("%");
            let value: f64 = value_str.parse().map_err(|_| "Invalid percent value".to_string())?;
            if value < 0.0 || value > 100.0 || !value.is_finite() {
                return Err("Invalid percent value".to_string());
            }
            return Ok(FlexBasis::Percent(value));
        }
        Err(format!("Invalid flex basis: {}", s))
    }
}

pub struct FlexBasisManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FlexBasisManager {
    /// Creates a new flex basis manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new FlexBasisManager instance
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

    /// Get the maximum flex basis length constant
    /// 
    /// # Returns
    /// The maximum flex basis string length
    pub fn max_flex_basis_length() -> usize {
        MAX_FLEX_BASIS_LENGTH
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

    /// Validate flex basis string
    /// 
    /// # Arguments
    /// * `flex_basis` - The flex basis string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting flex basis string length
    fn validate_flex_basis(&self, flex_basis: &str) -> Result<(), String> {
        if flex_basis.len() > MAX_FLEX_BASIS_LENGTH {
            return Err(format!("Flex basis string exceeds maximum length of {} characters", MAX_FLEX_BASIS_LENGTH));
        }
        
        // Validate flex basis value
        FlexBasis::from_str(flex_basis)?;
        
        Ok(())
    }

    /// Apply flex basis to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply flex basis to
    /// * `flex_basis` - The flex basis to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates flex basis string
    pub fn apply_flex_basis(&mut self, node: &mut TipTapNode, flex_basis: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate flex basis
        self.validate_flex_basis(flex_basis)?;

        // Apply flex basis to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("flexBasis".to_string(), serde_json::Value::String(flex_basis.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "flexBasis": flex_basis }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Flex basis application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Flex basis application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove flex basis from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove flex basis from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_flex_basis(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("flexBasis");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Flex basis removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Flex basis removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get flex basis from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get flex basis from
    /// 
    /// # Returns
    /// Option containing the flex basis string or None
    pub fn get_flex_basis(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(flex_basis) = obj.get("flexBasis") {
                    if let Some(s) = flex_basis.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has flex basis
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has flex basis, false otherwise
    pub fn has_flex_basis(&self, node: &TipTapNode) -> bool {
        self.get_flex_basis(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_flex_basis_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexBasisManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(FlexBasisManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(FlexBasisManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(FlexBasisManager::max_flex_basis_length(), MAX_FLEX_BASIS_LENGTH);
    }

    #[test]
    fn test_flex_basis_variants() {
        assert_eq!(FlexBasis::Auto.as_str(), "auto");
        assert_eq!(FlexBasis::Content.as_str(), "content");
        assert_eq!(FlexBasis::Pixel(100.0).as_str(), "100px");
        assert_eq!(FlexBasis::Percent(50.0).as_str(), "50%");
    }

    #[test]
    fn test_flex_basis_from_str() {
        assert!(matches!(FlexBasis::from_str("auto"), Ok(FlexBasis::Auto)));
        assert!(matches!(FlexBasis::from_str("content"), Ok(FlexBasis::Content)));
        assert!(matches!(FlexBasis::from_str("100px"), Ok(FlexBasis::Pixel(100.0))));
        assert!(matches!(FlexBasis::from_str("50%"), Ok(FlexBasis::Percent(50.0))));
        assert!(FlexBasis::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_flex_basis() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexBasisManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_flex_basis(&mut node, "auto");
        assert!(result.is_ok());
        assert!(manager.has_flex_basis(&node));
    }

    #[test]
    fn test_apply_flex_basis_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexBasisManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_flex_basis(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_flex_basis_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexBasisManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_flex_basis = "a".repeat(MAX_FLEX_BASIS_LENGTH + 1);
        let result = manager.apply_flex_basis(&mut node, &long_flex_basis);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_flex_basis() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexBasisManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "flexBasis": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_flex_basis(&node));
        let result = manager.remove_flex_basis(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_flex_basis(&node));
    }

    #[test]
    fn test_get_flex_basis() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexBasisManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "flexBasis": "200px" })),
            marks: None,
        };
        
        let flex_basis = manager.get_flex_basis(&node);
        assert_eq!(flex_basis, Some("200px".to_string()));
    }

    #[test]
    fn test_get_flex_basis_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexBasisManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let flex_basis = manager.get_flex_basis(&node);
        assert!(flex_basis.is_none());
    }

    #[test]
    fn test_has_flex_basis() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexBasisManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "flexBasis": "75%" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_flex_basis(&node_with));
        assert!(!manager.has_flex_basis(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexBasisManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_flex_basis(&mut node, "auto").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexBasisManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_flex_basis(&mut node, "auto").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexBasisManager::new(config_service);
        
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
        let mut manager = FlexBasisManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
