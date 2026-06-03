//! TipTap Scripts Manager - Aerospace-Grade Subscript/Superscript Operations Service
//!
//! Safety-critical subscript/superscript operations service with:
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

/// Maximum script depth to prevent stack overflow
const MAX_SCRIPT_DEPTH: usize = 10;

/// Script type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptType {
    Subscript,
    Superscript,
}

impl ScriptType {
    /// Convert script type to string
    pub fn as_str(&self) -> &str {
        match self {
            ScriptType::Subscript => "sub",
            ScriptType::Superscript => "sup",
        }
    }

    /// Parse script type from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "sub" | "subscript" => Ok(ScriptType::Subscript),
            "sup" | "superscript" => Ok(ScriptType::Superscript),
            _ => Err(format!("Invalid script type: {}", s)),
        }
    }
}

pub struct ScriptsManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ScriptsManager {
    /// Creates a new scripts manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new ScriptsManager instance
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

    /// Get the maximum script depth constant
    /// 
    /// # Returns
    /// The maximum script depth
    pub fn max_script_depth() -> usize {
        MAX_SCRIPT_DEPTH
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

    /// Validate script depth
    /// 
    /// # Arguments
    /// * `depth` - The current script depth
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents stack overflow by limiting depth
    fn validate_script_depth(&self, depth: usize) -> Result<(), String> {
        if depth >= MAX_SCRIPT_DEPTH {
            return Err(format!("Script depth exceeds maximum of {}", MAX_SCRIPT_DEPTH));
        }
        Ok(())
    }

    /// Apply subscript to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply subscript to
    /// * `depth` - The nesting depth
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates script depth
    pub fn apply_subscript(&mut self, node: &mut TipTapNode, depth: usize) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate depth
        self.validate_script_depth(depth)?;

        // Apply subscript to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("script".to_string(), serde_json::Value::String("sub".to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "script": "sub" }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Subscript application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Subscript application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Apply superscript to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply superscript to
    /// * `depth` - The nesting depth
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates script depth
    pub fn apply_superscript(&mut self, node: &mut TipTapNode, depth: usize) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate depth
        self.validate_script_depth(depth)?;

        // Apply superscript to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("script".to_string(), serde_json::Value::String("sup".to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "script": "sup" }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Superscript application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Superscript application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove script from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove script from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_script(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("script");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Script removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Script removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get script type from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get script type from
    /// 
    /// # Returns
    /// Option containing the script type or None
    pub fn get_script_type(&self, node: &TipTapNode) -> Option<ScriptType> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(script) = obj.get("script") {
                    if let Some(s) = script.as_str() {
                        return ScriptType::from_str(s).ok();
                    }
                }
            }
        }
        None
    }

    /// Check if node has subscript
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has subscript, false otherwise
    pub fn has_subscript(&self, node: &TipTapNode) -> bool {
        self.get_script_type(node) == Some(ScriptType::Subscript)
    }

    /// Check if node has superscript
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has superscript, false otherwise
    pub fn has_superscript(&self, node: &TipTapNode) -> bool {
        self.get_script_type(node) == Some(ScriptType::Superscript)
    }

    /// Check if node has any script
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has any script, false otherwise
    pub fn has_script(&self, node: &TipTapNode) -> bool {
        self.get_script_type(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_scripts_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ScriptsManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(ScriptsManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(ScriptsManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(ScriptsManager::max_script_depth(), MAX_SCRIPT_DEPTH);
    }

    #[test]
    fn test_script_type_variants() {
        assert_eq!(ScriptType::Subscript.as_str(), "sub");
        assert_eq!(ScriptType::Superscript.as_str(), "sup");
    }

    #[test]
    fn test_script_type_from_str() {
        assert!(matches!(ScriptType::from_str("sub"), Ok(ScriptType::Subscript)));
        assert!(matches!(ScriptType::from_str("sup"), Ok(ScriptType::Superscript)));
        assert!(matches!(ScriptType::from_str("subscript"), Ok(ScriptType::Subscript)));
        assert!(matches!(ScriptType::from_str("superscript"), Ok(ScriptType::Superscript)));
        assert!(ScriptType::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_subscript() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScriptsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_subscript(&mut node, 0);
        assert!(result.is_ok());
        assert!(manager.has_subscript(&node));
    }

    #[test]
    fn test_apply_subscript_too_deep() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScriptsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_subscript(&mut node, MAX_SCRIPT_DEPTH);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_superscript() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScriptsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_superscript(&mut node, 0);
        assert!(result.is_ok());
        assert!(manager.has_superscript(&node));
    }

    #[test]
    fn test_apply_superscript_too_deep() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScriptsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_superscript(&mut node, MAX_SCRIPT_DEPTH);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_script() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScriptsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "script": "sub" })),
            marks: None,
        };
        
        assert!(manager.has_script(&node));
        let result = manager.remove_script(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_script(&node));
    }

    #[test]
    fn test_get_script_type() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ScriptsManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "script": "sup" })),
            marks: None,
        };
        
        let script_type = manager.get_script_type(&node);
        assert_eq!(script_type, Some(ScriptType::Superscript));
    }

    #[test]
    fn test_get_script_type_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ScriptsManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let script_type = manager.get_script_type(&node);
        assert!(script_type.is_none());
    }

    #[test]
    fn test_has_subscript() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ScriptsManager::new(config_service);
        
        let node_sub = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "script": "sub" })),
            marks: None,
        };
        
        let node_sup = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "script": "sup" })),
            marks: None,
        };
        
        let node_none = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_subscript(&node_sub));
        assert!(!manager.has_subscript(&node_sup));
        assert!(!manager.has_subscript(&node_none));
    }

    #[test]
    fn test_has_superscript() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ScriptsManager::new(config_service);
        
        let node_sub = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "script": "sub" })),
            marks: None,
        };
        
        let node_sup = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "script": "sup" })),
            marks: None,
        };
        
        let node_none = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(!manager.has_superscript(&node_sub));
        assert!(manager.has_superscript(&node_sup));
        assert!(!manager.has_superscript(&node_none));
    }

    #[test]
    fn test_has_script() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ScriptsManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "script": "sub" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_script(&node_with));
        assert!(!manager.has_script(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScriptsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_subscript(&mut node, 0).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScriptsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_subscript(&mut node, 0).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ScriptsManager::new(config_service);
        
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
        let mut manager = ScriptsManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
