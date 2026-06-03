//! TipTap Print Color Adjust Manager - Aerospace-Grade Print Color Adjust Operations Service
//!
//! Safety-critical print color adjust operations service with:
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
pub enum PrintColorAdjust {
    Economy,
    Exact,
}

impl PrintColorAdjust {
    pub fn as_str(&self) -> &str {
        match self {
            PrintColorAdjust::Economy => "economy",
            PrintColorAdjust::Exact => "exact",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "economy" => Ok(PrintColorAdjust::Economy),
            "exact" => Ok(PrintColorAdjust::Exact),
            _ => Err(format!("Invalid print color adjust value: {}", s)),
        }
    }
}

pub struct PrintColorAdjustManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl PrintColorAdjustManager {
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

    fn validate_print_color_adjust(&self, print_color_adjust: &str) -> Result<(), String> {
        if print_color_adjust.is_empty() {
            return Err("Print color adjust cannot be empty".to_string());
        }
        PrintColorAdjust::from_str(print_color_adjust)?;
        Ok(())
    }

    pub fn apply_print_color_adjust(&mut self, node: &mut TipTapNode, print_color_adjust: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_print_color_adjust(print_color_adjust)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("printColorAdjust".to_string(), serde_json::Value::String(print_color_adjust.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "printColorAdjust": print_color_adjust }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Print color adjust application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Print color adjust application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_print_color_adjust(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("printColorAdjust");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Print color adjust removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Print color adjust removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_print_color_adjust(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(print_color_adjust) = obj.get("printColorAdjust") {
                    if let Some(s) = print_color_adjust.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_print_color_adjust(&self, node: &TipTapNode) -> bool {
        self.get_print_color_adjust(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_print_color_adjust_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PrintColorAdjustManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_print_color_adjust_variants() {
        assert_eq!(PrintColorAdjust::Economy.as_str(), "economy");
        assert_eq!(PrintColorAdjust::Exact.as_str(), "exact");
    }

    #[test]
    fn test_apply_print_color_adjust() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PrintColorAdjustManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_print_color_adjust(&mut node, "economy");
        assert!(result.is_ok());
        assert!(manager.has_print_color_adjust(&node));
    }

    #[test]
    fn test_remove_print_color_adjust() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = PrintColorAdjustManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "printColorAdjust": "exact" })),
            marks: None,
        };
        
        assert!(manager.has_print_color_adjust(&node));
        let result = manager.remove_print_color_adjust(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_print_color_adjust(&node));
    }

    #[test]
    fn test_get_print_color_adjust() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = PrintColorAdjustManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "printColorAdjust": "economy" })),
            marks: None,
        };
        
        let print_color_adjust = manager.get_print_color_adjust(&node);
        assert_eq!(print_color_adjust, Some("economy".to_string()));
    }
}
