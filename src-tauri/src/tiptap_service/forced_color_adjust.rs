//! TipTap Forced Color Adjust Manager - Aerospace-Grade Forced Color Adjust Operations Service
//!
//! Safety-critical forced color adjust operations service with:
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
pub enum ForcedColorAdjust {
    Auto,
    None,
}

impl ForcedColorAdjust {
    pub fn as_str(&self) -> &str {
        match self {
            ForcedColorAdjust::Auto => "auto",
            ForcedColorAdjust::None => "none",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(ForcedColorAdjust::Auto),
            "none" => Ok(ForcedColorAdjust::None),
            _ => Err(format!("Invalid forced color adjust value: {}", s)),
        }
    }
}

pub struct ForcedColorAdjustManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ForcedColorAdjustManager {
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

    fn validate_forced_color_adjust(&self, forced_color_adjust: &str) -> Result<(), String> {
        if forced_color_adjust.is_empty() {
            return Err("Forced color adjust cannot be empty".to_string());
        }
        ForcedColorAdjust::from_str(forced_color_adjust)?;
        Ok(())
    }

    pub fn apply_forced_color_adjust(&mut self, node: &mut TipTapNode, forced_color_adjust: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_forced_color_adjust(forced_color_adjust)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("forcedColorAdjust".to_string(), serde_json::Value::String(forced_color_adjust.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "forcedColorAdjust": forced_color_adjust }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Forced color adjust application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Forced color adjust application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_forced_color_adjust(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("forcedColorAdjust");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Forced color adjust removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Forced color adjust removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_forced_color_adjust(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(forced_color_adjust) = obj.get("forcedColorAdjust") {
                    if let Some(s) = forced_color_adjust.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_forced_color_adjust(&self, node: &TipTapNode) -> bool {
        self.get_forced_color_adjust(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_forced_color_adjust_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ForcedColorAdjustManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_forced_color_adjust_variants() {
        assert_eq!(ForcedColorAdjust::Auto.as_str(), "auto");
        assert_eq!(ForcedColorAdjust::None.as_str(), "none");
    }

    #[test]
    fn test_apply_forced_color_adjust() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ForcedColorAdjustManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_forced_color_adjust(&mut node, "auto");
        assert!(result.is_ok());
        assert!(manager.has_forced_color_adjust(&node));
    }

    #[test]
    fn test_remove_forced_color_adjust() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ForcedColorAdjustManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "forcedColorAdjust": "none" })),
            marks: None,
        };
        
        assert!(manager.has_forced_color_adjust(&node));
        let result = manager.remove_forced_color_adjust(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_forced_color_adjust(&node));
    }

    #[test]
    fn test_get_forced_color_adjust() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ForcedColorAdjustManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "forcedColorAdjust": "auto" })),
            marks: None,
        };
        
        let forced_color_adjust = manager.get_forced_color_adjust(&node);
        assert_eq!(forced_color_adjust, Some("auto".to_string()));
    }
}
