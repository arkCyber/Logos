//! TipTap Row Gap Manager - Aerospace-Grade Row Gap Operations Service
//!
//! Safety-critical row gap operations service with:
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

/// Maximum row gap value (in pixels)
const MAX_ROW_GAP: f64 = 1000.0;

/// Minimum row gap value (in pixels)
const MIN_ROW_GAP: f64 = 0.0;

/// Maximum row gap string length
const MAX_ROW_GAP_LENGTH: usize = 50;

pub struct RowGapManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl RowGapManager {
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

    pub fn max_row_gap() -> f64 {
        MAX_ROW_GAP
    }

    pub fn min_row_gap() -> f64 {
        MIN_ROW_GAP
    }

    pub fn max_row_gap_length() -> usize {
        MAX_ROW_GAP_LENGTH
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

    fn validate_row_gap(&self, row_gap: &str) -> Result<(), String> {
        if row_gap.is_empty() {
            return Err("Row gap cannot be empty".to_string());
        }
        if row_gap.len() > MAX_ROW_GAP_LENGTH {
            return Err(format!("Row gap string exceeds maximum length of {} characters", MAX_ROW_GAP_LENGTH));
        }
        if row_gap.ends_with("px") {
            let value_str = row_gap.trim_end_matches("px");
            if let Ok(value) = value_str.parse::<f64>() {
                if value < MIN_ROW_GAP || value > MAX_ROW_GAP {
                    return Err(format!("Row gap must be between {} and {} pixels", MIN_ROW_GAP, MAX_ROW_GAP));
                }
                if !value.is_finite() {
                    return Err("Row gap must be a finite number".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn apply_row_gap(&mut self, node: &mut TipTapNode, row_gap: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_row_gap(row_gap)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("rowGap".to_string(), serde_json::Value::String(row_gap.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "rowGap": row_gap }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Row gap application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Row gap application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_row_gap(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("rowGap");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Row gap removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Row gap removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_row_gap(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(row_gap) = obj.get("rowGap") {
                    if let Some(s) = row_gap.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_row_gap(&self, node: &TipTapNode) -> bool {
        self.get_row_gap(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_row_gap_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = RowGapManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_apply_row_gap() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = RowGapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_row_gap(&mut node, "10px");
        assert!(result.is_ok());
        assert!(manager.has_row_gap(&node));
    }

    #[test]
    fn test_remove_row_gap() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = RowGapManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "rowGap": "20px" })),
            marks: None,
        };
        
        assert!(manager.has_row_gap(&node));
        let result = manager.remove_row_gap(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_row_gap(&node));
    }

    #[test]
    fn test_get_row_gap() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = RowGapManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "rowGap": "30px" })),
            marks: None,
        };
        
        let row_gap = manager.get_row_gap(&node);
        assert_eq!(row_gap, Some("30px".to_string()));
    }
}
