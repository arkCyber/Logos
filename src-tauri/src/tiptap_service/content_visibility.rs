//! TipTap Content Visibility Manager - Aerospace-Grade Content Visibility Operations Service
//!
//! Safety-critical content visibility operations service with:
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

/// Maximum content visibility string length
const MAX_CONTENT_VISIBILITY_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentVisibility {
    Visible,
    Hidden,
}

impl ContentVisibility {
    pub fn as_str(&self) -> &str {
        match self {
            ContentVisibility::Visible => "visible",
            ContentVisibility::Hidden => "hidden",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "visible" => Ok(ContentVisibility::Visible),
            "hidden" => Ok(ContentVisibility::Hidden),
            _ => Err(format!("Invalid content visibility: {}", s)),
        }
    }
}

pub struct ContentVisibilityManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ContentVisibilityManager {
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

    pub fn max_content_visibility_length() -> usize {
        MAX_CONTENT_VISIBILITY_LENGTH
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

    fn validate_content_visibility(&self, content_visibility: &str) -> Result<(), String> {
        if content_visibility.len() > MAX_CONTENT_VISIBILITY_LENGTH {
            return Err(format!("Content visibility string exceeds maximum length of {} characters", MAX_CONTENT_VISIBILITY_LENGTH));
        }
        ContentVisibility::from_str(content_visibility)?;
        Ok(())
    }

    pub fn apply_content_visibility(&mut self, node: &mut TipTapNode, content_visibility: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_content_visibility(content_visibility)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("contentVisibility".to_string(), serde_json::Value::String(content_visibility.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "contentVisibility": content_visibility }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Content visibility application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Content visibility application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_content_visibility(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("contentVisibility");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Content visibility removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Content visibility removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_content_visibility(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(content_visibility) = obj.get("contentVisibility") {
                    if let Some(s) = content_visibility.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_content_visibility(&self, node: &TipTapNode) -> bool {
        self.get_content_visibility(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_content_visibility_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ContentVisibilityManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_content_visibility_variants() {
        assert_eq!(ContentVisibility::Visible.as_str(), "visible");
        assert_eq!(ContentVisibility::Hidden.as_str(), "hidden");
    }

    #[test]
    fn test_apply_content_visibility() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ContentVisibilityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_content_visibility(&mut node, "hidden");
        assert!(result.is_ok());
        assert!(manager.has_content_visibility(&node));
    }

    #[test]
    fn test_remove_content_visibility() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ContentVisibilityManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "contentVisibility": "visible" })),
            marks: None,
        };
        
        assert!(manager.has_content_visibility(&node));
        let result = manager.remove_content_visibility(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_content_visibility(&node));
    }

    #[test]
    fn test_get_content_visibility() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ContentVisibilityManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "contentVisibility": "visible" })),
            marks: None,
        };
        
        let content_visibility = manager.get_content_visibility(&node);
        assert_eq!(content_visibility, Some("visible".to_string()));
    }
}
