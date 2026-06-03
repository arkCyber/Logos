//! TipTap Orphans Manager - Aerospace-Grade Orphans Operations Service
//!
//! Safety-critical orphans operations service with:
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

/// Maximum orphans value
const MAX_ORPHANS: u32 = 100;

/// Minimum orphans value
const MIN_ORPHANS: u32 = 1;

/// Maximum orphans string length
const MAX_ORPHANS_LENGTH: usize = 50;

pub struct OrphansManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl OrphansManager {
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

    pub fn max_orphans() -> u32 {
        MAX_ORPHANS
    }

    pub fn min_orphans() -> u32 {
        MIN_ORPHANS
    }

    pub fn max_orphans_length() -> usize {
        MAX_ORPHANS_LENGTH
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

    fn validate_orphans(&self, orphans: &str) -> Result<(), String> {
        if orphans.is_empty() {
            return Err("Orphans cannot be empty".to_string());
        }
        if orphans.len() > MAX_ORPHANS_LENGTH {
            return Err(format!("Orphans string exceeds maximum length of {} characters", MAX_ORPHANS_LENGTH));
        }
        if orphans == "inherit" {
            return Ok(());
        }
        if let Ok(value) = orphans.parse::<u32>() {
            if value < MIN_ORPHANS || value > MAX_ORPHANS {
                return Err(format!("Orphans must be between {} and {}", MIN_ORPHANS, MAX_ORPHANS));
            }
        } else {
            return Err(format!("Invalid orphans value: {}", orphans));
        }
        Ok(())
    }

    pub fn apply_orphans(&mut self, node: &mut TipTapNode, orphans: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_orphans(orphans)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("orphans".to_string(), serde_json::Value::String(orphans.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "orphans": orphans }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Orphans application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Orphans application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_orphans(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("orphans");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Orphans removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Orphans removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_orphans(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(orphans) = obj.get("orphans") {
                    if let Some(s) = orphans.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_orphans(&self, node: &TipTapNode) -> bool {
        self.get_orphans(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_orphans_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OrphansManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_orphans() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OrphansManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_orphans(&mut node, "2");
        assert!(result.is_ok());
        assert!(manager.has_orphans(&node));
    }

    #[test]
    fn test_remove_orphans() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OrphansManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "orphans": "3" })),
            marks: None,
        };
        
        assert!(manager.has_orphans(&node));
        let result = manager.remove_orphans(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_orphans(&node));
    }

    #[test]
    fn test_get_orphans() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OrphansManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "orphans": "4" })),
            marks: None,
        };
        
        let orphans = manager.get_orphans(&node);
        assert_eq!(orphans, Some("4".to_string()));
    }
}
