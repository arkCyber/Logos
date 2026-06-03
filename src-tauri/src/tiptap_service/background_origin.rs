//! TipTap Background Origin Manager - Aerospace-Grade Background Origin Operations Service
//!
//! Safety-critical background origin operations service with:
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

/// Maximum background origin string length
const MAX_BACKGROUND_ORIGIN_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundOrigin {
    BorderBox,
    PaddingBox,
    ContentBox,
}

impl BackgroundOrigin {
    pub fn as_str(&self) -> &str {
        match self {
            BackgroundOrigin::BorderBox => "border-box",
            BackgroundOrigin::PaddingBox => "padding-box",
            BackgroundOrigin::ContentBox => "content-box",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "border-box" => Ok(BackgroundOrigin::BorderBox),
            "padding-box" => Ok(BackgroundOrigin::PaddingBox),
            "content-box" => Ok(BackgroundOrigin::ContentBox),
            _ => Err(format!("Invalid background origin: {}", s)),
        }
    }
}

pub struct BackgroundOriginManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl BackgroundOriginManager {
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

    pub fn max_background_origin_length() -> usize {
        MAX_BACKGROUND_ORIGIN_LENGTH
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

    fn validate_background_origin(&self, background_origin: &str) -> Result<(), String> {
        if background_origin.len() > MAX_BACKGROUND_ORIGIN_LENGTH {
            return Err(format!("Background origin string exceeds maximum length of {} characters", MAX_BACKGROUND_ORIGIN_LENGTH));
        }
        BackgroundOrigin::from_str(background_origin)?;
        Ok(())
    }

    pub fn apply_background_origin(&mut self, node: &mut TipTapNode, background_origin: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_background_origin(background_origin)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("backgroundOrigin".to_string(), serde_json::Value::String(background_origin.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "backgroundOrigin": background_origin }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Background origin application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Background origin application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_background_origin(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("backgroundOrigin");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Background origin removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Background origin removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_background_origin(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(background_origin) = obj.get("backgroundOrigin") {
                    if let Some(s) = background_origin.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_background_origin(&self, node: &TipTapNode) -> bool {
        self.get_background_origin(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_background_origin_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BackgroundOriginManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_background_origin_variants() {
        assert_eq!(BackgroundOrigin::BorderBox.as_str(), "border-box");
        assert_eq!(BackgroundOrigin::ContentBox.as_str(), "content-box");
    }

    #[test]
    fn test_apply_background_origin() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackgroundOriginManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_background_origin(&mut node, "padding-box");
        assert!(result.is_ok());
        assert!(manager.has_background_origin(&node));
    }

    #[test]
    fn test_remove_background_origin() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BackgroundOriginManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "backgroundOrigin": "border-box" })),
            marks: None,
        };
        
        assert!(manager.has_background_origin(&node));
        let result = manager.remove_background_origin(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_background_origin(&node));
    }

    #[test]
    fn test_get_background_origin() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BackgroundOriginManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "backgroundOrigin": "content-box" })),
            marks: None,
        };
        
        let background_origin = manager.get_background_origin(&node);
        assert_eq!(background_origin, Some("content-box".to_string()));
    }
}
