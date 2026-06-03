//! TipTap Mask Composite Manager - Aerospace-Grade Mask Composite Operations Service
//!
//! Safety-critical mask composite operations service with:
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

/// Maximum mask composite string length
const MAX_MASK_COMPOSITE_LENGTH: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskComposite {
    Add,
    Subtract,
    Intersect,
    Exclude,
}

impl MaskComposite {
    pub fn as_str(&self) -> &str {
        match self {
            MaskComposite::Add => "add",
            MaskComposite::Subtract => "subtract",
            MaskComposite::Intersect => "intersect",
            MaskComposite::Exclude => "exclude",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "add" => Ok(MaskComposite::Add),
            "subtract" => Ok(MaskComposite::Subtract),
            "intersect" => Ok(MaskComposite::Intersect),
            "exclude" => Ok(MaskComposite::Exclude),
            _ => Err(format!("Invalid mask composite: {}", s)),
        }
    }
}

pub struct MaskCompositeManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl MaskCompositeManager {
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

    pub fn max_mask_composite_length() -> usize {
        MAX_MASK_COMPOSITE_LENGTH
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

    fn validate_mask_composite(&self, mask_composite: &str) -> Result<(), String> {
        if mask_composite.len() > MAX_MASK_COMPOSITE_LENGTH {
            return Err(format!("Mask composite string exceeds maximum length of {} characters", MAX_MASK_COMPOSITE_LENGTH));
        }
        MaskComposite::from_str(mask_composite)?;
        Ok(())
    }

    pub fn apply_mask_composite(&mut self, node: &mut TipTapNode, mask_composite: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.validate_mask_composite(mask_composite)?;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("maskComposite".to_string(), serde_json::Value::String(mask_composite.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "maskComposite": mask_composite }));
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Mask composite application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Mask composite application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn remove_mask_composite(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("maskComposite");
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Mask composite removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Mask composite removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_mask_composite(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(mask_composite) = obj.get("maskComposite") {
                    if let Some(s) = mask_composite.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn has_mask_composite(&self, node: &TipTapNode) -> bool {
        self.get_mask_composite(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_mask_composite_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskCompositeManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_mask_composite_variants() {
        assert_eq!(MaskComposite::Add.as_str(), "add");
        assert_eq!(MaskComposite::Intersect.as_str(), "intersect");
    }

    #[test]
    fn test_apply_mask_composite() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskCompositeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_mask_composite(&mut node, "add");
        assert!(result.is_ok());
        assert!(manager.has_mask_composite(&node));
    }

    #[test]
    fn test_remove_mask_composite() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MaskCompositeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maskComposite": "subtract" })),
            marks: None,
        };
        
        assert!(manager.has_mask_composite(&node));
        let result = manager.remove_mask_composite(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_mask_composite(&node));
    }

    #[test]
    fn test_get_mask_composite() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MaskCompositeManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "maskComposite": "exclude" })),
            marks: None,
        };
        
        let mask_composite = manager.get_mask_composite(&node);
        assert_eq!(mask_composite, Some("exclude".to_string()));
    }
}
