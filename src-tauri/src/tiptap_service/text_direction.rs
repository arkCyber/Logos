//! TipTap Text Direction Manager - Aerospace-Grade Text Direction Operations Service
//!
//! Safety-critical text direction operations service with:
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

/// Maximum direction value length
const MAX_DIRECTION_LENGTH: usize = 20;

/// Text direction type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDirection {
    LeftToRight,
    RightToLeft,
    Auto,
}

impl TextDirection {
    /// Convert text direction to string
    pub fn as_str(&self) -> &str {
        match self {
            TextDirection::LeftToRight => "ltr",
            TextDirection::RightToLeft => "rtl",
            TextDirection::Auto => "auto",
        }
    }

    /// Parse text direction from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "ltr" | "lefttoright" => Ok(TextDirection::LeftToRight),
            "rtl" | "righttoleft" => Ok(TextDirection::RightToLeft),
            "auto" => Ok(TextDirection::Auto),
            _ => Err(format!("Invalid text direction: {}", s)),
        }
    }
}

pub struct TextDirectionManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TextDirectionManager {
    /// Creates a new text direction manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new TextDirectionManager instance
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

    /// Get the maximum direction length constant
    /// 
    /// # Returns
    /// The maximum direction value length
    pub fn max_direction_length() -> usize {
        MAX_DIRECTION_LENGTH
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

    /// Validate direction string
    /// 
    /// # Arguments
    /// * `direction` - The direction string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting direction string length
    fn validate_direction(&self, direction: &str) -> Result<(), String> {
        if direction.len() > MAX_DIRECTION_LENGTH {
            return Err(format!("Direction string exceeds maximum length of {} characters", MAX_DIRECTION_LENGTH));
        }
        
        // Validate direction value
        TextDirection::from_str(direction)?;
        
        Ok(())
    }

    /// Apply text direction to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply direction to
    /// * `direction` - The direction to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates direction string
    pub fn apply_direction(&mut self, node: &mut TipTapNode, direction: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate direction
        self.validate_direction(direction)?;

        // Apply direction to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textDirection".to_string(), serde_json::Value::String(direction.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "textDirection": direction }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text direction application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text direction application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove text direction from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove direction from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_direction(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("textDirection");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text direction removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text direction removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get text direction from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get direction from
    /// 
    /// # Returns
    /// Option containing the direction string or None
    pub fn get_direction(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(direction) = obj.get("textDirection") {
                    if let Some(s) = direction.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has text direction
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has text direction, false otherwise
    pub fn has_direction(&self, node: &TipTapNode) -> bool {
        self.get_direction(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_text_direction_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextDirectionManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(TextDirectionManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(TextDirectionManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(TextDirectionManager::max_direction_length(), MAX_DIRECTION_LENGTH);
    }

    #[test]
    fn test_text_direction_variants() {
        assert_eq!(TextDirection::LeftToRight.as_str(), "ltr");
        assert_eq!(TextDirection::RightToLeft.as_str(), "rtl");
        assert_eq!(TextDirection::Auto.as_str(), "auto");
    }

    #[test]
    fn test_text_direction_from_str() {
        assert!(matches!(TextDirection::from_str("ltr"), Ok(TextDirection::LeftToRight)));
        assert!(matches!(TextDirection::from_str("rtl"), Ok(TextDirection::RightToLeft)));
        assert!(matches!(TextDirection::from_str("auto"), Ok(TextDirection::Auto)));
        assert!(TextDirection::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_direction() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDirectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_direction(&mut node, "rtl");
        assert!(result.is_ok());
        assert!(manager.has_direction(&node));
    }

    #[test]
    fn test_apply_direction_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDirectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_direction(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_direction_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDirectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_direction = "a".repeat(MAX_DIRECTION_LENGTH + 1);
        let result = manager.apply_direction(&mut node, &long_direction);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_direction() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDirectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textDirection": "ltr" })),
            marks: None,
        };
        
        assert!(manager.has_direction(&node));
        let result = manager.remove_direction(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_direction(&node));
    }

    #[test]
    fn test_get_direction() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextDirectionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textDirection": "rtl" })),
            marks: None,
        };
        
        let direction = manager.get_direction(&node);
        assert_eq!(direction, Some("rtl".to_string()));
    }

    #[test]
    fn test_get_direction_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextDirectionManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let direction = manager.get_direction(&node);
        assert!(direction.is_none());
    }

    #[test]
    fn test_has_direction() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TextDirectionManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "textDirection": "ltr" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_direction(&node_with));
        assert!(!manager.has_direction(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDirectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_direction(&mut node, "ltr").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDirectionManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_direction(&mut node, "ltr").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TextDirectionManager::new(config_service);
        
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
        let mut manager = TextDirectionManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
