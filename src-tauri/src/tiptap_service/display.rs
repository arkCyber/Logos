//! TipTap Display Manager - Aerospace-Grade Display Operations Service
//!
//! Safety-critical display operations service with:
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

/// Maximum display string length
const MAX_DISPLAY_LENGTH: usize = 50;

/// Display type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    Block,
    Inline,
    InlineBlock,
    Flex,
    Grid,
    None,
    InlineFlex,
    InlineGrid,
}

impl Display {
    /// Convert display to string
    pub fn as_str(&self) -> &str {
        match self {
            Display::Block => "block",
            Display::Inline => "inline",
            Display::InlineBlock => "inline-block",
            Display::Flex => "flex",
            Display::Grid => "grid",
            Display::None => "none",
            Display::InlineFlex => "inline-flex",
            Display::InlineGrid => "inline-grid",
        }
    }

    /// Parse display from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "block" => Ok(Display::Block),
            "inline" => Ok(Display::Inline),
            "inline-block" => Ok(Display::InlineBlock),
            "flex" => Ok(Display::Flex),
            "grid" => Ok(Display::Grid),
            "none" => Ok(Display::None),
            "inline-flex" => Ok(Display::InlineFlex),
            "inline-grid" => Ok(Display::InlineGrid),
            _ => Err(format!("Invalid display: {}", s)),
        }
    }
}

pub struct DisplayManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl DisplayManager {
    /// Creates a new display manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new DisplayManager instance
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

    /// Get the maximum display length constant
    /// 
    /// # Returns
    /// The maximum display string length
    pub fn max_display_length() -> usize {
        MAX_DISPLAY_LENGTH
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

    /// Validate display string
    /// 
    /// # Arguments
    /// * `display` - The display string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting display string length
    fn validate_display(&self, display: &str) -> Result<(), String> {
        if display.len() > MAX_DISPLAY_LENGTH {
            return Err(format!("Display string exceeds maximum length of {} characters", MAX_DISPLAY_LENGTH));
        }
        
        // Validate display value
        Display::from_str(display)?;
        
        Ok(())
    }

    /// Apply display to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply display to
    /// * `display` - The display to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates display string
    pub fn apply_display(&mut self, node: &mut TipTapNode, display: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate display
        self.validate_display(display)?;

        // Apply display to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("display".to_string(), serde_json::Value::String(display.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "display": display }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Display application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Display application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove display from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove display from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_display(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("display");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Display removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Display removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get display from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get display from
    /// 
    /// # Returns
    /// Option containing the display string or None
    pub fn get_display(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(display) = obj.get("display") {
                    if let Some(s) = display.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has display
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has display, false otherwise
    pub fn has_display(&self, node: &TipTapNode) -> bool {
        self.get_display(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_display_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = DisplayManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(DisplayManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(DisplayManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(DisplayManager::max_display_length(), MAX_DISPLAY_LENGTH);
    }

    #[test]
    fn test_display_variants() {
        assert_eq!(Display::Block.as_str(), "block");
        assert_eq!(Display::Inline.as_str(), "inline");
        assert_eq!(Display::Flex.as_str(), "flex");
    }

    #[test]
    fn test_display_from_str() {
        assert!(matches!(Display::from_str("block"), Ok(Display::Block)));
        assert!(matches!(Display::from_str("inline"), Ok(Display::Inline)));
        assert!(matches!(Display::from_str("flex"), Ok(Display::Flex)));
        assert!(Display::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_display() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = DisplayManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_display(&mut node, "flex");
        assert!(result.is_ok());
        assert!(manager.has_display(&node));
    }

    #[test]
    fn test_apply_display_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = DisplayManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_display(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_display_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = DisplayManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_display = "a".repeat(MAX_DISPLAY_LENGTH + 1);
        let result = manager.apply_display(&mut node, &long_display);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_display() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = DisplayManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "display": "flex" })),
            marks: None,
        };
        
        assert!(manager.has_display(&node));
        let result = manager.remove_display(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_display(&node));
    }

    #[test]
    fn test_get_display() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = DisplayManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "display": "grid" })),
            marks: None,
        };
        
        let display = manager.get_display(&node);
        assert_eq!(display, Some("grid".to_string()));
    }

    #[test]
    fn test_get_display_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = DisplayManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let display = manager.get_display(&node);
        assert!(display.is_none());
    }

    #[test]
    fn test_has_display() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = DisplayManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "display": "inline-block" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_display(&node_with));
        assert!(!manager.has_display(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = DisplayManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_display(&mut node, "flex").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = DisplayManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_display(&mut node, "flex").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = DisplayManager::new(config_service);
        
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
        let mut manager = DisplayManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
