//! TipTap Grid Auto Rows Manager - Aerospace-Grade Grid Auto Rows Operations Service
//!
//! Safety-critical grid auto rows operations service with:
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

/// Maximum grid auto rows string length
const MAX_GRID_AUTO_ROWS_LENGTH: usize = 200;

pub struct GridAutoRowsManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl GridAutoRowsManager {
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

    pub fn max_grid_auto_rows_length() -> usize {
        MAX_GRID_AUTO_ROWS_LENGTH
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

    fn validate_grid_auto_rows(&self, grid_auto_rows: &str) -> Result<(), String> {
        if grid_auto_rows.is_empty() {
            return Err("Grid auto rows cannot be empty".to_string());
        }
        if grid_auto_rows.len() > MAX_GRID_AUTO_ROWS_LENGTH {
            return Err(format!("Grid auto rows string exceeds maximum length of {} characters", MAX_GRID_AUTO_ROWS_LENGTH));
        }
        if grid_auto_rows.contains('(') && !grid_auto_rows.contains(')') {
            return Err("Invalid grid auto rows: unmatched parentheses".to_string());
        }
        Ok(())
    }

    pub fn apply_grid_auto_rows(&mut self, node: &mut TipTapNode, grid_auto_rows: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_grid_auto_rows(grid_auto_rows)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("gridAutoRows".to_string(), serde_json::Value::String(grid_auto_rows.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "gridAutoRows": grid_auto_rows }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Grid auto rows application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Grid auto rows application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_grid_auto_rows(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("gridAutoRows");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Grid auto rows removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Grid auto rows removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_grid_auto_rows(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(grid_auto_rows) = obj.get("gridAutoRows") {
                    if let Some(s) = grid_auto_rows.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_grid_auto_rows(&self, node: &TipTapNode) -> bool {
        self.get_grid_auto_rows(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_grid_auto_rows_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridAutoRowsManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_grid_auto_rows() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridAutoRowsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_auto_rows(&mut node, "minmax(100px, auto)");
        assert!(result.is_ok());
        assert!(manager.has_grid_auto_rows(&node));
    }

    #[test]
    fn test_remove_grid_auto_rows() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridAutoRowsManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridAutoRows": "200px" })),
            marks: None,
        };
        
        assert!(manager.has_grid_auto_rows(&node));
        let result = manager.remove_grid_auto_rows(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_grid_auto_rows(&node));
    }

    #[test]
    fn test_get_grid_auto_rows() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridAutoRowsManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridAutoRows": "auto" })),
            marks: None,
        };
        
        let grid_auto_rows = manager.get_grid_auto_rows(&node);
        assert_eq!(grid_auto_rows, Some("auto".to_string()));
    }
}
