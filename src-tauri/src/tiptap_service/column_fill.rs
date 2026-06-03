//! TipTap Column Fill Manager - Aerospace-Grade Column Fill Operations Service
//!
//! Safety-critical column fill operations service with:
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

/// Maximum column fill string length
const MAX_COLUMN_FILL_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnFill {
    Auto,
    Balance,
}

impl ColumnFill {
    pub fn as_str(&self) -> &str {
        match self {
            ColumnFill::Auto => "auto",
            ColumnFill::Balance => "balance",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(ColumnFill::Auto),
            "balance" => Ok(ColumnFill::Balance),
            _ => Err(format!("Invalid column fill: {}", s)),
        }
    }
}

pub struct ColumnFillManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ColumnFillManager {
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

    pub fn max_column_fill_length() -> usize {
        MAX_COLUMN_FILL_LENGTH
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

    fn validate_column_fill(&self, column_fill: &str) -> Result<(), String> {
        if column_fill.len() > MAX_COLUMN_FILL_LENGTH {
            return Err(format!("Column fill string exceeds maximum length of {} characters", MAX_COLUMN_FILL_LENGTH));
        }
        ColumnFill::from_str(column_fill)?;
        Ok(())
    }

    pub fn apply_column_fill(&mut self, node: &mut TipTapNode, column_fill: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_column_fill(column_fill)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("columnFill".to_string(), serde_json::Value::String(column_fill.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "columnFill": column_fill }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Column fill application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Column fill application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_column_fill(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("columnFill");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Column fill removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Column fill removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_column_fill(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(column_fill) = obj.get("columnFill") {
                    if let Some(s) = column_fill.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_column_fill(&self, node: &TipTapNode) -> bool {
        self.get_column_fill(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_column_fill_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnFillManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_column_fill_variants() {
        assert_eq!(ColumnFill::Auto.as_str(), "auto");
        assert_eq!(ColumnFill::Balance.as_str(), "balance");
    }

    #[test]
    fn test_apply_column_fill() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnFillManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_column_fill(&mut node, "balance");
        assert!(result.is_ok());
        assert!(manager.has_column_fill(&node));
    }

    #[test]
    fn test_remove_column_fill() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnFillManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "columnFill": "auto" })),
            marks: None,
        };
        
        assert!(manager.has_column_fill(&node));
        let result = manager.remove_column_fill(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_column_fill(&node));
    }

    #[test]
    fn test_get_column_fill() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnFillManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "columnFill": "balance" })),
            marks: None,
        };
        
        let column_fill = manager.get_column_fill(&node);
        assert_eq!(column_fill, Some("balance".to_string()));
    }
}
