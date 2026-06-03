//! TipTap Flex Align Manager - Aerospace-Grade Flex Align Operations Service
//!
//! Safety-critical flex align operations service with:
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

/// Maximum flex align string length
const MAX_FLEX_ALIGN_LENGTH: usize = 50;

/// Flex align type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexAlign {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl FlexAlign {
    /// Convert flex align to string
    pub fn as_str(&self) -> &str {
        match self {
            FlexAlign::Start => "flex-start",
            FlexAlign::End => "flex-end",
            FlexAlign::Center => "center",
            FlexAlign::Baseline => "baseline",
            FlexAlign::Stretch => "stretch",
        }
    }

    /// Parse flex align from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "flex-start" | "start" => Ok(FlexAlign::Start),
            "flex-end" | "end" => Ok(FlexAlign::End),
            "center" => Ok(FlexAlign::Center),
            "baseline" => Ok(FlexAlign::Baseline),
            "stretch" => Ok(FlexAlign::Stretch),
            _ => Err(format!("Invalid flex align: {}", s)),
        }
    }
}

pub struct FlexAlignManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FlexAlignManager {
    /// Creates a new flex align manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new FlexAlignManager instance
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

    /// Get the maximum flex align length constant
    /// 
    /// # Returns
    /// The maximum flex align string length
    pub fn max_flex_align_length() -> usize {
        MAX_FLEX_ALIGN_LENGTH
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

    /// Validate flex align string
    /// 
    /// # Arguments
    /// * `flex_align` - The flex align string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting flex align string length
    fn validate_flex_align(&self, flex_align: &str) -> Result<(), String> {
        if flex_align.len() > MAX_FLEX_ALIGN_LENGTH {
            return Err(format!("Flex align string exceeds maximum length of {} characters", MAX_FLEX_ALIGN_LENGTH));
        }
        
        // Validate flex align value
        FlexAlign::from_str(flex_align)?;
        
        Ok(())
    }

    /// Apply flex align to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply flex align to
    /// * `flex_align` - The flex align to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates flex align string
    pub fn apply_flex_align(&mut self, node: &mut TipTapNode, flex_align: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate flex align
        self.validate_flex_align(flex_align)?;

        // Apply flex align to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("alignItems".to_string(), serde_json::Value::String(flex_align.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "alignItems": flex_align }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Flex align application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Flex align application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove flex align from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove flex align from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_flex_align(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("alignItems");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Flex align removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Flex align removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get flex align from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get flex align from
    /// 
    /// # Returns
    /// Option containing the flex align string or None
    pub fn get_flex_align(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(flex_align) = obj.get("alignItems") {
                    if let Some(s) = flex_align.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has flex align
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has flex align, false otherwise
    pub fn has_flex_align(&self, node: &TipTapNode) -> bool {
        self.get_flex_align(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_flex_align_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexAlignManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(FlexAlignManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(FlexAlignManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(FlexAlignManager::max_flex_align_length(), MAX_FLEX_ALIGN_LENGTH);
    }

    #[test]
    fn test_flex_align_variants() {
        assert_eq!(FlexAlign::Start.as_str(), "flex-start");
        assert_eq!(FlexAlign::Center.as_str(), "center");
        assert_eq!(FlexAlign::Stretch.as_str(), "stretch");
    }

    #[test]
    fn test_flex_align_from_str() {
        assert!(matches!(FlexAlign::from_str("flex-start"), Ok(FlexAlign::Start)));
        assert!(matches!(FlexAlign::from_str("center"), Ok(FlexAlign::Center)));
        assert!(matches!(FlexAlign::from_str("stretch"), Ok(FlexAlign::Stretch)));
        assert!(FlexAlign::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_flex_align() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexAlignManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_flex_align(&mut node, "center");
        assert!(result.is_ok());
        assert!(manager.has_flex_align(&node));
    }

    #[test]
    fn test_apply_flex_align_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexAlignManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_flex_align(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_flex_align_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexAlignManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_flex_align = "a".repeat(MAX_FLEX_ALIGN_LENGTH + 1);
        let result = manager.apply_flex_align(&mut node, &long_flex_align);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_flex_align() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexAlignManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "alignItems": "flex-start" })),
            marks: None,
        };
        
        assert!(manager.has_flex_align(&node));
        let result = manager.remove_flex_align(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_flex_align(&node));
    }

    #[test]
    fn test_get_flex_align() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexAlignManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "alignItems": "baseline" })),
            marks: None,
        };
        
        let flex_align = manager.get_flex_align(&node);
        assert_eq!(flex_align, Some("baseline".to_string()));
    }

    #[test]
    fn test_get_flex_align_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexAlignManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let flex_align = manager.get_flex_align(&node);
        assert!(flex_align.is_none());
    }

    #[test]
    fn test_has_flex_align() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexAlignManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "alignItems": "center" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_flex_align(&node_with));
        assert!(!manager.has_flex_align(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexAlignManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_flex_align(&mut node, "center").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexAlignManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_flex_align(&mut node, "center").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexAlignManager::new(config_service);
        
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
        let mut manager = FlexAlignManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
