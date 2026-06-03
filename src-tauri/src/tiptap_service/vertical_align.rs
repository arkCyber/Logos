//! TipTap Vertical Align Manager - Aerospace-Grade Vertical Align Operations Service
//!
//! Safety-critical vertical align operations service with:
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

/// Maximum vertical align string length
const MAX_VERTICAL_ALIGN_LENGTH: usize = 50;

/// Vertical align type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlign {
    Baseline,
    Top,
    Middle,
    Bottom,
    TextTop,
    TextBottom,
    Sub,
    Super,
}

impl VerticalAlign {
    /// Convert vertical align to string
    pub fn as_str(&self) -> &str {
        match self {
            VerticalAlign::Baseline => "baseline",
            VerticalAlign::Top => "top",
            VerticalAlign::Middle => "middle",
            VerticalAlign::Bottom => "bottom",
            VerticalAlign::TextTop => "text-top",
            VerticalAlign::TextBottom => "text-bottom",
            VerticalAlign::Sub => "sub",
            VerticalAlign::Super => "super",
        }
    }

    /// Parse vertical align from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "baseline" => Ok(VerticalAlign::Baseline),
            "top" => Ok(VerticalAlign::Top),
            "middle" => Ok(VerticalAlign::Middle),
            "bottom" => Ok(VerticalAlign::Bottom),
            "text-top" => Ok(VerticalAlign::TextTop),
            "text-bottom" => Ok(VerticalAlign::TextBottom),
            "sub" => Ok(VerticalAlign::Sub),
            "super" => Ok(VerticalAlign::Super),
            _ => Err(format!("Invalid vertical align: {}", s)),
        }
    }
}

pub struct VerticalAlignManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl VerticalAlignManager {
    /// Creates a new vertical align manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new VerticalAlignManager instance
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

    /// Get the maximum vertical align length constant
    /// 
    /// # Returns
    /// The maximum vertical align string length
    pub fn max_vertical_align_length() -> usize {
        MAX_VERTICAL_ALIGN_LENGTH
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

    /// Validate vertical align string
    /// 
    /// # Arguments
    /// * `vertical_align` - The vertical align string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting vertical align string length
    fn validate_vertical_align(&self, vertical_align: &str) -> Result<(), String> {
        if vertical_align.len() > MAX_VERTICAL_ALIGN_LENGTH {
            return Err(format!("Vertical align string exceeds maximum length of {} characters", MAX_VERTICAL_ALIGN_LENGTH));
        }
        
        // Validate vertical align value
        VerticalAlign::from_str(vertical_align)?;
        
        Ok(())
    }

    /// Apply vertical align to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply vertical align to
    /// * `vertical_align` - The vertical align to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates vertical align string
    pub fn apply_vertical_align(&mut self, node: &mut TipTapNode, vertical_align: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate vertical align
        self.validate_vertical_align(vertical_align)?;

        // Apply vertical align to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("verticalAlign".to_string(), serde_json::Value::String(vertical_align.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "verticalAlign": vertical_align }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Vertical align application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Vertical align application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove vertical align from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove vertical align from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_vertical_align(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("verticalAlign");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Vertical align removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Vertical align removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get vertical align from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get vertical align from
    /// 
    /// # Returns
    /// Option containing the vertical align string or None
    pub fn get_vertical_align(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(vertical_align) = obj.get("verticalAlign") {
                    if let Some(s) = vertical_align.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has vertical align
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has vertical align, false otherwise
    pub fn has_vertical_align(&self, node: &TipTapNode) -> bool {
        self.get_vertical_align(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_vertical_align_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = VerticalAlignManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(VerticalAlignManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(VerticalAlignManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(VerticalAlignManager::max_vertical_align_length(), MAX_VERTICAL_ALIGN_LENGTH);
    }

    #[test]
    fn test_vertical_align_variants() {
        assert_eq!(VerticalAlign::Baseline.as_str(), "baseline");
        assert_eq!(VerticalAlign::Top.as_str(), "top");
        assert_eq!(VerticalAlign::Middle.as_str(), "middle");
    }

    #[test]
    fn test_vertical_align_from_str() {
        assert!(matches!(VerticalAlign::from_str("baseline"), Ok(VerticalAlign::Baseline)));
        assert!(matches!(VerticalAlign::from_str("top"), Ok(VerticalAlign::Top)));
        assert!(matches!(VerticalAlign::from_str("middle"), Ok(VerticalAlign::Middle)));
        assert!(VerticalAlign::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_vertical_align() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = VerticalAlignManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_vertical_align(&mut node, "middle");
        assert!(result.is_ok());
        assert!(manager.has_vertical_align(&node));
    }

    #[test]
    fn test_apply_vertical_align_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = VerticalAlignManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_vertical_align(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_vertical_align_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = VerticalAlignManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_vertical_align = "a".repeat(MAX_VERTICAL_ALIGN_LENGTH + 1);
        let result = manager.apply_vertical_align(&mut node, &long_vertical_align);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_vertical_align() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = VerticalAlignManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "verticalAlign": "middle" })),
            marks: None,
        };
        
        assert!(manager.has_vertical_align(&node));
        let result = manager.remove_vertical_align(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_vertical_align(&node));
    }

    #[test]
    fn test_get_vertical_align() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = VerticalAlignManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "verticalAlign": "top" })),
            marks: None,
        };
        
        let vertical_align = manager.get_vertical_align(&node);
        assert_eq!(vertical_align, Some("top".to_string()));
    }

    #[test]
    fn test_get_vertical_align_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = VerticalAlignManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let vertical_align = manager.get_vertical_align(&node);
        assert!(vertical_align.is_none());
    }

    #[test]
    fn test_has_vertical_align() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = VerticalAlignManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "verticalAlign": "bottom" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_vertical_align(&node_with));
        assert!(!manager.has_vertical_align(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = VerticalAlignManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_vertical_align(&mut node, "middle").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = VerticalAlignManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_vertical_align(&mut node, "middle").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = VerticalAlignManager::new(config_service);
        
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
        let mut manager = VerticalAlignManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
