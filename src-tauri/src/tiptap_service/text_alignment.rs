//! TipTap Text Alignment Manager - Aerospace-Grade Text Alignment Operations Service
//!
//! Safety-critical text alignment operations service with:
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
use super::editor::TipTapNode;

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 50;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 200;

/// Maximum alignment value length
const MAX_ALIGNMENT_LENGTH: usize = 20;

/// Text alignment type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    Left,
    Center,
    Right,
    Justify,
}

impl Alignment {
    /// Convert alignment to string
    pub fn as_str(&self) -> &str {
        match self {
            Alignment::Left => "left",
            Alignment::Center => "center",
            Alignment::Right => "right",
            Alignment::Justify => "justify",
        }
    }

    /// Parse alignment from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "left" => Ok(Alignment::Left),
            "center" => Ok(Alignment::Center),
            "right" => Ok(Alignment::Right),
            "justify" => Ok(Alignment::Justify),
            _ => Err(format!("Invalid alignment: {}", s)),
        }
    }
}

pub struct TextAlignmentManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextAlignmentManager {
    /// Creates a new text alignment manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new TextAlignmentManager instance
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

    /// Get the maximum alignment length constant
    /// 
    /// # Returns
    /// The maximum alignment value length
    pub fn max_alignment_length() -> usize {
        MAX_ALIGNMENT_LENGTH
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

    /// Validate alignment string
    /// 
    /// # Arguments
    /// * `alignment` - The alignment string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting alignment string length
    fn validate_alignment(&self, alignment: &str) -> Result<(), String> {
        if alignment.len() > MAX_ALIGNMENT_LENGTH {
            return Err(format!("Alignment string exceeds maximum length of {} characters", MAX_ALIGNMENT_LENGTH));
        }
        
        // Validate alignment value
        Alignment::from_str(alignment)?;
        
        Ok(())
    }

    /// Apply alignment to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply alignment to
    /// * `alignment` - The alignment to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates alignment string
    pub fn apply_alignment(&mut self, node: &mut TipTapNode, alignment: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate alignment
        self.validate_alignment(alignment)?;

        // Apply alignment to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textAlign".to_string(), serde_json::Value::String(alignment.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textAlign": alignment }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text alignment application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text alignment application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove alignment from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove alignment from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_alignment(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textAlign");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text alignment removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text alignment removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get alignment from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get alignment from
    /// 
    /// # Returns
    /// Option containing the alignment string or None
    pub fn get_alignment(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(align) = obj.get("textAlign") {
                    if let Some(s) = align.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has alignment
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has alignment, false otherwise
    pub fn has_alignment(&self, node: &TipTapNode) -> bool {
        self.get_alignment(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_alignment_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextAlignmentManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(TextAlignmentManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(TextAlignmentManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(TextAlignmentManager::max_alignment_length(), MAX_ALIGNMENT_LENGTH);
    }

    #[test]
    fn test_alignment_variants() {
        assert_eq!(Alignment::Left.as_str(), "left");
        assert_eq!(Alignment::Center.as_str(), "center");
        assert_eq!(Alignment::Right.as_str(), "right");
        assert_eq!(Alignment::Justify.as_str(), "justify");
    }

    #[test]
    fn test_alignment_from_str() {
        assert!(matches!(Alignment::from_str("left"), Ok(Alignment::Left)));
        assert!(matches!(Alignment::from_str("center"), Ok(Alignment::Center)));
        assert!(matches!(Alignment::from_str("right"), Ok(Alignment::Right)));
        assert!(matches!(Alignment::from_str("justify"), Ok(Alignment::Justify)));
        assert!(Alignment::from_str("invalid").is_err());
    }

    #[test]
    fn test_alignment_from_str_case_insensitive() {
        assert!(matches!(Alignment::from_str("LEFT"), Ok(Alignment::Left)));
        assert!(matches!(Alignment::from_str("Center"), Ok(Alignment::Center)));
        assert!(matches!(Alignment::from_str("RIGHT"), Ok(Alignment::Right)));
    }

    #[test]
    fn test_apply_alignment() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextAlignmentManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_alignment(&mut node, "center");
        assert!(result.is_ok());
        assert!(manager.has_alignment(&node));
    }

    #[test]
    fn test_apply_alignment_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextAlignmentManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_alignment(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_alignment_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextAlignmentManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_alignment = "a".repeat(MAX_ALIGNMENT_LENGTH + 1);
        let result = manager.apply_alignment(&mut node, &long_alignment);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_alignment() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextAlignmentManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textAlign": "center" })),
            marks: None,
        };
        
        assert!(manager.has_alignment(&node));
        let result = manager.remove_alignment(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_alignment(&node));
    }

    #[test]
    fn test_get_alignment() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextAlignmentManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textAlign": "right" })),
            marks: None,
        };
        
        let alignment = manager.get_alignment(&node);
        assert_eq!(alignment, Some("right".to_string()));
    }

    #[test]
    fn test_get_alignment_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextAlignmentManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let alignment = manager.get_alignment(&node);
        assert!(alignment.is_none());
    }

    #[test]
    fn test_has_alignment() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextAlignmentManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textAlign": "left" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_alignment(&node_with));
        assert!(!manager.has_alignment(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextAlignmentManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_alignment(&mut node, "left").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextAlignmentManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_alignment(&mut node, "left").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextAlignmentManager::new(config_service);
        
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
        let mut manager = TextAlignmentManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
