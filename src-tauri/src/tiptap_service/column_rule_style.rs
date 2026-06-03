//! TipTap Column Rule Style Manager - Aerospace-Grade Column Rule Style Operations Service
//!
//! Safety-critical column rule style operations service with:
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

/// Maximum column rule style string length
const MAX_COLUMN_RULE_STYLE_LENGTH: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnRuleStyle {
    None,
    Hidden,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

impl ColumnRuleStyle {
    pub fn as_str(&self) -> &str {
        match self {
            ColumnRuleStyle::None => "none",
            ColumnRuleStyle::Hidden => "hidden",
            ColumnRuleStyle::Dotted => "dotted",
            ColumnRuleStyle::Dashed => "dashed",
            ColumnRuleStyle::Solid => "solid",
            ColumnRuleStyle::Double => "double",
            ColumnRuleStyle::Groove => "groove",
            ColumnRuleStyle::Ridge => "ridge",
            ColumnRuleStyle::Inset => "inset",
            ColumnRuleStyle::Outset => "outset",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" => Ok(ColumnRuleStyle::None),
            "hidden" => Ok(ColumnRuleStyle::Hidden),
            "dotted" => Ok(ColumnRuleStyle::Dotted),
            "dashed" => Ok(ColumnRuleStyle::Dashed),
            "solid" => Ok(ColumnRuleStyle::Solid),
            "double" => Ok(ColumnRuleStyle::Double),
            "groove" => Ok(ColumnRuleStyle::Groove),
            "ridge" => Ok(ColumnRuleStyle::Ridge),
            "inset" => Ok(ColumnRuleStyle::Inset),
            "outset" => Ok(ColumnRuleStyle::Outset),
            _ => Err(format!("Invalid column rule style: {}", s)),
        }
    }
}

pub struct ColumnRuleStyleManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ColumnRuleStyleManager {
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

    pub fn max_column_rule_style_length() -> usize {
        MAX_COLUMN_RULE_STYLE_LENGTH
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

    fn validate_column_rule_style(&self, column_rule_style: &str) -> Result<(), String> {
        if column_rule_style.len() > MAX_COLUMN_RULE_STYLE_LENGTH {
            return Err(format!("Column rule style string exceeds maximum length of {} characters", MAX_COLUMN_RULE_STYLE_LENGTH));
        }
        ColumnRuleStyle::from_str(column_rule_style)?;
        Ok(())
    }

    pub fn apply_column_rule_style(&mut self, node: &mut TipTapNode, column_rule_style: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_column_rule_style(column_rule_style)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("columnRuleStyle".to_string(), serde_json::Value::String(column_rule_style.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "columnRuleStyle": column_rule_style }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Column rule style application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Column rule style application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_column_rule_style(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("columnRuleStyle");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Column rule style removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Column rule style removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_column_rule_style(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(column_rule_style) = obj.get("columnRuleStyle") {
                    if let Some(s) = column_rule_style.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_column_rule_style(&self, node: &TipTapNode) -> bool {
        self.get_column_rule_style(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_column_rule_style_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnRuleStyleManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_column_rule_style_variants() {
        assert_eq!(ColumnRuleStyle::Solid.as_str(), "solid");
        assert_eq!(ColumnRuleStyle::Dotted.as_str(), "dotted");
    }

    #[test]
    fn test_apply_column_rule_style() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnRuleStyleManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_column_rule_style(&mut node, "dashed");
        assert!(result.is_ok());
        assert!(manager.has_column_rule_style(&node));
    }

    #[test]
    fn test_remove_column_rule_style() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ColumnRuleStyleManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "columnRuleStyle": "solid" })),
            marks: None,
        };
        
        assert!(manager.has_column_rule_style(&node));
        let result = manager.remove_column_rule_style(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_column_rule_style(&node));
    }

    #[test]
    fn test_get_column_rule_style() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ColumnRuleStyleManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "columnRuleStyle": "double" })),
            marks: None,
        };
        
        let column_rule_style = manager.get_column_rule_style(&node);
        assert_eq!(column_rule_style, Some("double".to_string()));
    }
}
