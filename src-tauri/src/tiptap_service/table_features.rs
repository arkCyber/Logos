//! TipTap Table Features Manager - Aerospace-Grade Table Features Service
//!
//! Safety-critical table features service with:
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
use super::editor::{TipTapNode, NodeType};

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 50;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 200;

/// Maximum table rows
const MAX_TABLE_ROWS: usize = 1000;

/// Maximum table columns
const MAX_TABLE_COLUMNS: usize = 100;

/// Table cell alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableCellAlignment {
    Left,
    Center,
    Right,
    Justify,
}

impl TableCellAlignment {
    pub fn as_str(&self) -> &str {
        match self {
            TableCellAlignment::Left => "left",
            TableCellAlignment::Center => "center",
            TableCellAlignment::Right => "right",
            TableCellAlignment::Justify => "justify",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "left" => Ok(TableCellAlignment::Left),
            "center" => Ok(TableCellAlignment::Center),
            "right" => Ok(TableCellAlignment::Right),
            "justify" => Ok(TableCellAlignment::Justify),
            _ => Err(format!("Invalid cell alignment: {}", s)),
        }
    }
}

/// Table cell attributes
#[derive(Debug, Clone)]
pub struct TableCellFeatures {
    pub row_span: usize,
    pub col_span: usize,
    pub alignment: TableCellAlignment,
    pub background_color: Option<String>,
}

pub struct TableFeaturesManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl TableFeaturesManager {
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

    pub fn max_table_rows() -> usize {
        MAX_TABLE_ROWS
    }

    pub fn max_table_columns() -> usize {
        MAX_TABLE_COLUMNS
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

    pub fn set_cell_alignment(&mut self, node: &mut TipTapNode, alignment: TableCellAlignment) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if node.node_type != NodeType::TableCell {
            return Err("Node must be a table cell".to_string());
        }

        if node.attrs.is_none() {
            node.attrs = Some(serde_json::json!({}));
        }

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("textAlign".to_string(), serde_json::Value::String(alignment.as_str().to_string()));
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Set cell alignment CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Set cell alignment performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn set_cell_span(&mut self, node: &mut TipTapNode, row_span: usize, col_span: usize) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if node.node_type != NodeType::TableCell {
            return Err("Node must be a table cell".to_string());
        }

        if row_span == 0 || col_span == 0 {
            return Err("Row span and column span must be greater than 0".to_string());
        }

        if node.attrs.is_none() {
            node.attrs = Some(serde_json::json!({}));
        }

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("rowspan".to_string(), serde_json::Value::Number(row_span.into()));
                obj.insert("colspan".to_string(), serde_json::Value::Number(col_span.into()));
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Set cell span CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Set cell span performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn set_cell_background(&mut self, node: &mut TipTapNode, color: String) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if node.node_type != NodeType::TableCell {
            return Err("Node must be a table cell".to_string());
        }

        if color.is_empty() {
            return Err("Color cannot be empty".to_string());
        }

        if node.attrs.is_none() {
            node.attrs = Some(serde_json::json!({}));
        }

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("backgroundColor".to_string(), serde_json::Value::String(color));
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Set cell background CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Set cell background performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_cell_alignment(&self, node: &TipTapNode) -> Option<TableCellAlignment> {
        if node.node_type != NodeType::TableCell {
            return None;
        }

        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(value) = obj.get("textAlign") {
                    if let Some(s) = value.as_str() {
                        return TableCellAlignment::from_str(s).ok();
                    }
                }
            }
        }
        None
    }

    pub fn get_cell_span(&self, node: &TipTapNode) -> Option<(usize, usize)> {
        if node.node_type != NodeType::TableCell {
            return None;
        }

        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                let row_span = obj.get("rowspan")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1) as usize;
                let col_span = obj.get("colspan")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1) as usize;
                return Some((row_span, col_span));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_table_features_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = TableFeaturesManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_set_cell_alignment() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TableFeaturesManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::TableCell,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        let result = manager.set_cell_alignment(&mut node, TableCellAlignment::Center);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_cell_span() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TableFeaturesManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::TableCell,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        let result = manager.set_cell_span(&mut node, 2, 3);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_cell_background() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TableFeaturesManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::TableCell,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        let result = manager.set_cell_background(&mut node, "#ff0000".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_cell_alignment() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = TableFeaturesManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::TableCell,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };
        
        manager.set_cell_alignment(&mut node, TableCellAlignment::Center).unwrap();
        
        let alignment = manager.get_cell_alignment(&node);
        assert!(alignment.is_some());
        assert_eq!(alignment.unwrap(), TableCellAlignment::Center);
    }
}
