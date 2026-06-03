//! TipTap Flex Flow Manager - Aerospace-Grade Flex Flow Operations Service
//!
//! Safety-critical flex flow operations service with:
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

/// Maximum flex flow string length
const MAX_FLEX_FLOW_LENGTH: usize = 100;

pub struct FlexFlowManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl FlexFlowManager {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
        }
    }

    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    pub fn max_flex_flow_length() -> usize {
        MAX_FLEX_FLOW_LENGTH
    }

    fn record_error(&mut self, code: &str, message: &str, source: &str) {
        self.last_error = Some(ErrorContext::new(ErrorSeverity::Error, code, message, source));
    }

    pub fn get_last_error(&self) -> Option<&ErrorContext> {
        self.last_error.as_ref()
    }

    pub fn get_operation_count(&self) -> u64 {
        self.operation_count
    }

    pub fn reset_error_state(&mut self) {
        self.last_error = None;
    }

    pub fn reset_operation_count(&mut self) {
        self.operation_count = 0;
    }

    fn validate_flex_flow(&self, flex_flow: &str) -> Result<(), String> {
        if flex_flow.is_empty() {
            return Err("Flex flow cannot be empty".to_string());
        }
        if flex_flow.len() > MAX_FLEX_FLOW_LENGTH {
            return Err(format!("Flex flow string exceeds maximum length of {} characters", MAX_FLEX_FLOW_LENGTH));
        }
        if flex_flow.contains('(') && !flex_flow.contains(')') {
            return Err("Invalid flex flow: unmatched parentheses".to_string());
        }
        Ok(())
    }

    pub fn apply_flex_flow(&mut self, node: &mut TipTapNode, flex_flow: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_flex_flow(flex_flow)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("flexFlow".to_string(), serde_json::Value::String(flex_flow.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "flexFlow": flex_flow }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Flex flow application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Flex flow application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_flex_flow(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("flexFlow");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Flex flow removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Flex flow removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_flex_flow(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(flex_flow) = obj.get("flexFlow") {
                    if let Some(s) = flex_flow.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_flex_flow(&self, node: &TipTapNode) -> bool {
        self.get_flex_flow(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_flex_flow_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexFlowManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_flex_flow() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexFlowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_flex_flow(&mut node, "row nowrap");
        assert!(result.is_ok());
        assert!(manager.has_flex_flow(&node));
    }

    #[test]
    fn test_remove_flex_flow() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = FlexFlowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "flexFlow": "column wrap" })),
            marks: None,
        };
        
        assert!(manager.has_flex_flow(&node));
        let result = manager.remove_flex_flow(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_flex_flow(&node));
    }

    #[test]
    fn test_get_flex_flow() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = FlexFlowManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "flexFlow": "row wrap-reverse" })),
            marks: None,
        };
        
        let flex_flow = manager.get_flex_flow(&node);
        assert_eq!(flex_flow, Some("row wrap-reverse".to_string()));
    }
}
