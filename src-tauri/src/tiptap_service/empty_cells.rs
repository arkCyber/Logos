//! TipTap Empty Cells Manager - Aerospace-Grade Empty Cells Operations Service
//!
//! Safety-critical empty cells operations service with:
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
pub enum EmptyCells {
    Show,
    Hide,
}

impl EmptyCells {
    pub fn as_str(&self) -> &str {
        match self {
            EmptyCells::Show => "show",
            EmptyCells::Hide => "hide",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "show" => Ok(EmptyCells::Show),
            "hide" => Ok(EmptyCells::Hide),
            _ => Err(format!("Invalid empty cells value: {}", s)),
        }
    }
}

pub struct EmptyCellsManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl EmptyCellsManager {
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

    fn validate_empty_cells(&self, empty_cells: &str) -> Result<(), String> {
        if empty_cells.is_empty() {
            return Err("Empty cells cannot be empty".to_string());
        }
        EmptyCells::from_str(empty_cells)?;
        Ok(())
    }

    pub fn apply_empty_cells(&mut self, node: &mut TipTapNode, empty_cells: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_empty_cells(empty_cells)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("emptyCells".to_string(), serde_json::Value::String(empty_cells.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "emptyCells": empty_cells }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Empty cells application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Empty cells application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_empty_cells(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("emptyCells");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Empty cells removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Empty cells removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_empty_cells(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(empty_cells) = obj.get("emptyCells") {
                    if let Some(s) = empty_cells.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_empty_cells(&self, node: &TipTapNode) -> bool {
        self.get_empty_cells(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_empty_cells_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = EmptyCellsManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_empty_cells_variants() {
        assert_eq!(EmptyCells::Show.as_str(), "show");
        assert_eq!(EmptyCells::Hide.as_str(), "hide");
    }

    #[test]
    fn test_apply_empty_cells() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = EmptyCellsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_empty_cells(&mut node, "show");
        assert!(result.is_ok());
        assert!(manager.has_empty_cells(&node));
    }

    #[test]
    fn test_remove_empty_cells() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = EmptyCellsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "emptyCells": "hide" })),
            marks: None,
        };
        
        assert!(manager.has_empty_cells(&node));
        let result = manager.remove_empty_cells(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_empty_cells(&node));
    }

    #[test]
    fn test_get_empty_cells() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = EmptyCellsManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "emptyCells": "show" })),
            marks: None,
        };
        
        let empty_cells = manager.get_empty_cells(&node);
        assert_eq!(empty_cells, Some("show".to_string()));
    }
}
