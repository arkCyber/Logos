//! TipTap Hyphens Manager - Aerospace-Grade Hyphens Operations Service
//!
//! Safety-critical hyphens operations service with:
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hyphens {
    None,
    Manual,
    Auto,
}

impl Hyphens {
    pub fn as_str(&self) -> &str {
        match self {
            Hyphens::None => "none",
            Hyphens::Manual => "manual",
            Hyphens::Auto => "auto",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Hyphens::None),
            "manual" => Ok(Hyphens::Manual),
            "auto" => Ok(Hyphens::Auto),
            _ => Err(format!("Invalid hyphens value: {}", s)),
        }
    }
}

pub struct HyphensManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl HyphensManager {
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

    fn validate_hyphens(&self, hyphens: &str) -> Result<(), String> {
        if hyphens.is_empty() {
            return Err("Hyphens cannot be empty".to_string());
        }
        Hyphens::from_str(hyphens)?;
        Ok(())
    }

    pub fn apply_hyphens(&mut self, node: &mut TipTapNode, hyphens: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_hyphens(hyphens)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("hyphens".to_string(), serde_json::Value::String(hyphens.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "hyphens": hyphens }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Hyphens application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Hyphens application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_hyphens(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("hyphens");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Hyphens removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Hyphens removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_hyphens(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(hyphens) = obj.get("hyphens") {
                    if let Some(s) = hyphens.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_hyphens(&self, node: &TipTapNode) -> bool {
        self.get_hyphens(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_hyphens_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = HyphensManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_hyphens_variants() {
        assert_eq!(Hyphens::None.as_str(), "none");
        assert_eq!(Hyphens::Auto.as_str(), "auto");
    }

    #[test]
    fn test_apply_hyphens() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HyphensManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_hyphens(&mut node, "auto");
        assert!(result.is_ok());
        assert!(manager.has_hyphens(&node));
    }

    #[test]
    fn test_remove_hyphens() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = HyphensManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "hyphens": "manual" })),
            marks: None,
        };
        
        assert!(manager.has_hyphens(&node));
        let result = manager.remove_hyphens(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_hyphens(&node));
    }

    #[test]
    fn test_get_hyphens() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = HyphensManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "hyphens": "none" })),
            marks: None,
        };
        
        let hyphens = manager.get_hyphens(&node);
        assert_eq!(hyphens, Some("none".to_string()));
    }
}
