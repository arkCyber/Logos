//! TipTap Object Fit Manager - Aerospace-Grade Object Fit Operations Service
//!
//! Safety-critical object fit operations service with:
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

/// Maximum object fit string length
const MAX_OBJECT_FIT_LENGTH: usize = 50;

/// Object fit type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectFit {
    Fill,
    Contain,
    Cover,
    None,
    ScaleDown,
}

impl ObjectFit {
    /// Convert object fit to string
    pub fn as_str(&self) -> &str {
        match self {
            ObjectFit::Fill => "fill",
            ObjectFit::Contain => "contain",
            ObjectFit::Cover => "cover",
            ObjectFit::None => "none",
            ObjectFit::ScaleDown => "scale-down",
        }
    }

    /// Parse object fit from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "fill" => Ok(ObjectFit::Fill),
            "contain" => Ok(ObjectFit::Contain),
            "cover" => Ok(ObjectFit::Cover),
            "none" => Ok(ObjectFit::None),
            "scale-down" => Ok(ObjectFit::ScaleDown),
            _ => Err(format!("Invalid object fit: {}", s)),
        }
    }
}

pub struct ObjectFitManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ObjectFitManager {
    /// Creates a new object fit manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new ObjectFitManager instance
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

    /// Get the maximum object fit length constant
    /// 
    /// # Returns
    /// The maximum object fit string length
    pub fn max_object_fit_length() -> usize {
        MAX_OBJECT_FIT_LENGTH
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

    /// Validate object fit string
    /// 
    /// # Arguments
    /// * `object_fit` - The object fit string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting object fit string length
    fn validate_object_fit(&self, object_fit: &str) -> Result<(), String> {
        if object_fit.len() > MAX_OBJECT_FIT_LENGTH {
            return Err(format!("Object fit string exceeds maximum length of {} characters", MAX_OBJECT_FIT_LENGTH));
        }
        
        // Validate object fit value
        ObjectFit::from_str(object_fit)?;
        
        Ok(())
    }

    /// Apply object fit to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply object fit to
    /// * `object_fit` - The object fit to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates object fit string
    pub fn apply_object_fit(&mut self, node: &mut TipTapNode, object_fit: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate object fit
        self.validate_object_fit(object_fit)?;

        // Apply object fit to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("objectFit".to_string(), serde_json::Value::String(object_fit.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "objectFit": object_fit }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Object fit application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Object fit application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove object fit from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove object fit from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_object_fit(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("objectFit");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Object fit removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Object fit removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get object fit from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get object fit from
    /// 
    /// # Returns
    /// Option containing the object fit string or None
    pub fn get_object_fit(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(object_fit) = obj.get("objectFit") {
                    if let Some(s) = object_fit.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has object fit
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has object fit, false otherwise
    pub fn has_object_fit(&self, node: &TipTapNode) -> bool {
        self.get_object_fit(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_object_fit_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ObjectFitManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(ObjectFitManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(ObjectFitManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(ObjectFitManager::max_object_fit_length(), MAX_OBJECT_FIT_LENGTH);
    }

    #[test]
    fn test_object_fit_variants() {
        assert_eq!(ObjectFit::Fill.as_str(), "fill");
        assert_eq!(ObjectFit::Contain.as_str(), "contain");
        assert_eq!(ObjectFit::Cover.as_str(), "cover");
    }

    #[test]
    fn test_object_fit_from_str() {
        assert!(matches!(ObjectFit::from_str("fill"), Ok(ObjectFit::Fill)));
        assert!(matches!(ObjectFit::from_str("contain"), Ok(ObjectFit::Contain)));
        assert!(matches!(ObjectFit::from_str("cover"), Ok(ObjectFit::Cover)));
        assert!(ObjectFit::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_object_fit() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ObjectFitManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_object_fit(&mut node, "cover");
        assert!(result.is_ok());
        assert!(manager.has_object_fit(&node));
    }

    #[test]
    fn test_apply_object_fit_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ObjectFitManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_object_fit(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_object_fit_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ObjectFitManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_object_fit = "a".repeat(MAX_OBJECT_FIT_LENGTH + 1);
        let result = manager.apply_object_fit(&mut node, &long_object_fit);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_object_fit() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ObjectFitManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "objectFit": "contain" })),
            marks: None,
        };
        
        assert!(manager.has_object_fit(&node));
        let result = manager.remove_object_fit(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_object_fit(&node));
    }

    #[test]
    fn test_get_object_fit() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ObjectFitManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "objectFit": "scale-down" })),
            marks: None,
        };
        
        let object_fit = manager.get_object_fit(&node);
        assert_eq!(object_fit, Some("scale-down".to_string()));
    }

    #[test]
    fn test_get_object_fit_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ObjectFitManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let object_fit = manager.get_object_fit(&node);
        assert!(object_fit.is_none());
    }

    #[test]
    fn test_has_object_fit() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ObjectFitManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "objectFit": "none" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_object_fit(&node_with));
        assert!(!manager.has_object_fit(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ObjectFitManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_object_fit(&mut node, "cover").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ObjectFitManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_object_fit(&mut node, "cover").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ObjectFitManager::new(config_service);
        
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
        let mut manager = ObjectFitManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
