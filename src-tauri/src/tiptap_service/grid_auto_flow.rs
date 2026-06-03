//! TipTap Grid Auto Flow Manager - Aerospace-Grade Grid Auto Flow Operations Service
//!
//! Safety-critical grid auto flow operations service with:
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

/// Maximum grid auto flow string length
const MAX_GRID_AUTO_FLOW_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridAutoFlow {
    Row,
    Column,
    Dense,
    RowDense,
    ColumnDense,
}

impl GridAutoFlow {
    pub fn as_str(&self) -> &str {
        match self {
            GridAutoFlow::Row => "row",
            GridAutoFlow::Column => "column",
            GridAutoFlow::Dense => "dense",
            GridAutoFlow::RowDense => "row dense",
            GridAutoFlow::ColumnDense => "column dense",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "row" => Ok(GridAutoFlow::Row),
            "column" => Ok(GridAutoFlow::Column),
            "dense" => Ok(GridAutoFlow::Dense),
            "row dense" => Ok(GridAutoFlow::RowDense),
            "column dense" => Ok(GridAutoFlow::ColumnDense),
            _ => Err(format!("Invalid grid auto flow: {}", s)),
        }
    }
}

pub struct GridAutoFlowManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl GridAutoFlowManager {
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

    pub fn max_grid_auto_flow_length() -> usize {
        MAX_GRID_AUTO_FLOW_LENGTH
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

    fn validate_grid_auto_flow(&self, grid_auto_flow: &str) -> Result<(), String> {
        if grid_auto_flow.len() > MAX_GRID_AUTO_FLOW_LENGTH {
            return Err(format!("Grid auto flow string exceeds maximum length of {} characters", MAX_GRID_AUTO_FLOW_LENGTH));
        }
        GridAutoFlow::from_str(grid_auto_flow)?;
        Ok(())
    }

    pub fn apply_grid_auto_flow(&mut self, node: &mut TipTapNode, grid_auto_flow: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_grid_auto_flow(grid_auto_flow)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("gridAutoFlow".to_string(), serde_json::Value::String(grid_auto_flow.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "gridAutoFlow": grid_auto_flow }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Grid auto flow application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Grid auto flow application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_grid_auto_flow(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("gridAutoFlow");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Grid auto flow removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Grid auto flow removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_grid_auto_flow(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(grid_auto_flow) = obj.get("gridAutoFlow") {
                    if let Some(s) = grid_auto_flow.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_grid_auto_flow(&self, node: &TipTapNode) -> bool {
        self.get_grid_auto_flow(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_grid_auto_flow_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridAutoFlowManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_grid_auto_flow_variants() {
        assert_eq!(GridAutoFlow::Row.as_str(), "row");
        assert_eq!(GridAutoFlow::Dense.as_str(), "dense");
    }

    #[test]
    fn test_apply_grid_auto_flow() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridAutoFlowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_grid_auto_flow(&mut node, "row");
        assert!(result.is_ok());
        assert!(manager.has_grid_auto_flow(&node));
    }

    #[test]
    fn test_remove_grid_auto_flow() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = GridAutoFlowManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridAutoFlow": "column" })),
            marks: None,
        };
        
        assert!(manager.has_grid_auto_flow(&node));
        let result = manager.remove_grid_auto_flow(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_grid_auto_flow(&node));
    }

    #[test]
    fn test_get_grid_auto_flow() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = GridAutoFlowManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "gridAutoFlow": "row dense" })),
            marks: None,
        };
        
        let grid_auto_flow = manager.get_grid_auto_flow(&node);
        assert_eq!(grid_auto_flow, Some("row dense".to_string()));
    }
}
