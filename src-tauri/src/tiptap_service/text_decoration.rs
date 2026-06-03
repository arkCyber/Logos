//! TipTap Text Decoration Manager - Aerospace-Grade Text Decoration Operations Service
//!
//! Safety-critical text decoration operations service with:
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

/// Maximum decoration string length
const MAX_DECORATION_LENGTH: usize = 50;

/// Text decoration type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDecoration {
    None,
    Underline,
    Overline,
    LineThrough,
    Blink,
}

impl TextDecoration {
    /// Convert text decoration to string
    pub fn as_str(&self) -> &str {
        match self {
            TextDecoration::None => "none",
            TextDecoration::Underline => "underline",
            TextDecoration::Overline => "overline",
            TextDecoration::LineThrough => "line-through",
            TextDecoration::Blink => "blink",
        }
    }

    /// Parse text decoration from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" => Ok(TextDecoration::None),
            "underline" => Ok(TextDecoration::Underline),
            "overline" => Ok(TextDecoration::Overline),
            "line-through" | "linethrough" => Ok(TextDecoration::LineThrough),
            "blink" => Ok(TextDecoration::Blink),
            _ => Err(format!("Invalid text decoration: {}", s)),
        }
    }
}

pub struct TextDecorationManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextDecorationManager {
    /// Creates a new text decoration manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new TextDecorationManager instance
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

    /// Get the maximum decoration length constant
    /// 
    /// # Returns
    /// The maximum decoration string length
    pub fn max_decoration_length() -> usize {
        MAX_DECORATION_LENGTH
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

    /// Validate decoration string
    /// 
    /// # Arguments
    /// * `decoration` - The decoration string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting decoration string length
    fn validate_decoration(&self, decoration: &str) -> Result<(), String> {
        if decoration.len() > MAX_DECORATION_LENGTH {
            return Err(format!("Decoration string exceeds maximum length of {} characters", MAX_DECORATION_LENGTH));
        }
        
        // Validate decoration value
        TextDecoration::from_str(decoration)?;
        
        Ok(())
    }

    /// Apply text decoration to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply decoration to
    /// * `decoration` - The decoration to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates decoration string
    pub fn apply_decoration(&mut self, node: &mut TipTapNode, decoration: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate decoration
        self.validate_decoration(decoration)?;

        // Apply decoration to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textDecoration".to_string(), serde_json::Value::String(decoration.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textDecoration": decoration }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text decoration application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text decoration application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove text decoration from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove decoration from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_decoration(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textDecoration");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text decoration removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text decoration removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get text decoration from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get decoration from
    /// 
    /// # Returns
    /// Option containing the decoration string or None
    pub fn get_decoration(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(decoration) = obj.get("textDecoration") {
                    if let Some(s) = decoration.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has text decoration
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has text decoration, false otherwise
    pub fn has_decoration(&self, node: &TipTapNode) -> bool {
        self.get_decoration(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_decoration_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextDecorationManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(TextDecorationManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(TextDecorationManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(TextDecorationManager::max_decoration_length(), MAX_DECORATION_LENGTH);
    }

    #[test]
    fn test_text_decoration_variants() {
        assert_eq!(TextDecoration::Underline.as_str(), "underline");
        assert_eq!(TextDecoration::Overline.as_str(), "overline");
        assert_eq!(TextDecoration::LineThrough.as_str(), "line-through");
    }

    #[test]
    fn test_text_decoration_from_str() {
        assert!(matches!(TextDecoration::from_str("underline"), Ok(TextDecoration::Underline)));
        assert!(matches!(TextDecoration::from_str("overline"), Ok(TextDecoration::Overline)));
        assert!(matches!(TextDecoration::from_str("line-through"), Ok(TextDecoration::LineThrough)));
        assert!(matches!(TextDecoration::from_str("blink"), Ok(TextDecoration::Blink)));
        assert!(TextDecoration::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_decoration() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDecorationManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_decoration(&mut node, "underline");
        assert!(result.is_ok());
        assert!(manager.has_decoration(&node));
    }

    #[test]
    fn test_apply_decoration_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDecorationManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_decoration(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_decoration_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDecorationManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_decoration = "a".repeat(MAX_DECORATION_LENGTH + 1);
        let result = manager.apply_decoration(&mut node, &long_decoration);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_decoration() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDecorationManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textDecoration": "underline" })),
            marks: None,
        };
        
        assert!(manager.has_decoration(&node));
        let result = manager.remove_decoration(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_decoration(&node));
    }

    #[test]
    fn test_get_decoration() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextDecorationManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textDecoration": "line-through" })),
            marks: None,
        };
        
        let decoration = manager.get_decoration(&node);
        assert_eq!(decoration, Some("line-through".to_string()));
    }

    #[test]
    fn test_get_decoration_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextDecorationManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let decoration = manager.get_decoration(&node);
        assert!(decoration.is_none());
    }

    #[test]
    fn test_has_decoration() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextDecorationManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textDecoration": "underline" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_decoration(&node_with));
        assert!(!manager.has_decoration(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDecorationManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_decoration(&mut node, "underline").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDecorationManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_decoration(&mut node, "underline").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDecorationManager::new(config_service);
        
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
        let mut manager = TextDecorationManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
