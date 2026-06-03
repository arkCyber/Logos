//! TipTap Mix Blend Mode Manager - Aerospace-Grade Mix Blend Mode Operations Service
//!
//! Safety-critical mix blend mode operations service with:
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

/// Maximum mix blend mode string length
const MAX_MIX_BLEND_MODE_LENGTH: usize = 50;

/// Mix blend mode type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MixBlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl MixBlendMode {
    /// Convert mix blend mode to string
    pub fn as_str(&self) -> &str {
        match self {
            MixBlendMode::Normal => "normal",
            MixBlendMode::Multiply => "multiply",
            MixBlendMode::Screen => "screen",
            MixBlendMode::Overlay => "overlay",
            MixBlendMode::Darken => "darken",
            MixBlendMode::Lighten => "lighten",
            MixBlendMode::ColorDodge => "color-dodge",
            MixBlendMode::ColorBurn => "color-burn",
            MixBlendMode::HardLight => "hard-light",
            MixBlendMode::SoftLight => "soft-light",
            MixBlendMode::Difference => "difference",
            MixBlendMode::Exclusion => "exclusion",
            MixBlendMode::Hue => "hue",
            MixBlendMode::Saturation => "saturation",
            MixBlendMode::Color => "color",
            MixBlendMode::Luminosity => "luminosity",
        }
    }

    /// Parse mix blend mode from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(MixBlendMode::Normal),
            "multiply" => Ok(MixBlendMode::Multiply),
            "screen" => Ok(MixBlendMode::Screen),
            "overlay" => Ok(MixBlendMode::Overlay),
            "darken" => Ok(MixBlendMode::Darken),
            "lighten" => Ok(MixBlendMode::Lighten),
            "color-dodge" => Ok(MixBlendMode::ColorDodge),
            "color-burn" => Ok(MixBlendMode::ColorBurn),
            "hard-light" => Ok(MixBlendMode::HardLight),
            "soft-light" => Ok(MixBlendMode::SoftLight),
            "difference" => Ok(MixBlendMode::Difference),
            "exclusion" => Ok(MixBlendMode::Exclusion),
            "hue" => Ok(MixBlendMode::Hue),
            "saturation" => Ok(MixBlendMode::Saturation),
            "color" => Ok(MixBlendMode::Color),
            "luminosity" => Ok(MixBlendMode::Luminosity),
            _ => Err(format!("Invalid mix blend mode: {}", s)),
        }
    }
}

pub struct MixBlendModeManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl MixBlendModeManager {
    /// Creates a new mix blend mode manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new MixBlendModeManager instance
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
        }
    }

    /// Get the performance warning threshold
    /// 
    /// # Returns
    /// The performance warning threshold in milliseconds
    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    /// Get the performance critical threshold
    /// 
    /// # Returns
    /// The performance critical threshold in milliseconds
    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    /// Get the maximum mix blend mode length constant
    /// 
    /// # Returns
    /// The maximum mix blend mode string length
    pub fn max_mix_blend_mode_length() -> usize {
        MAX_MIX_BLEND_MODE_LENGTH
    }

    /// Record error context
    fn record_error(&mut self, code: &str, message: &str, source: &str) {
        self.last_error = Some(ErrorContext::new(
            ErrorSeverity::Error,
            code,
            message,
            source,
        ));
    }

    /// Get last error
    pub fn get_last_error(&self) -> Option<&ErrorContext> {
        self.last_error.as_ref()
    }

    /// Get operation count
    pub fn get_operation_count(&self) -> u64 {
        self.operation_count
    }

    /// Reset error state
    pub fn reset_error_state(&mut self) {
        self.last_error = None;
    }

    /// Reset operation count
    pub fn reset_operation_count(&mut self) {
        self.operation_count = 0;
    }

    /// Validate mix blend mode string
    /// 
    /// # Arguments
    /// * `mix_blend_mode` - The mix blend mode string to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting mix blend mode string length
    fn validate_mix_blend_mode(&self, mix_blend_mode: &str) -> Result<(), String> {
        if mix_blend_mode.len() > MAX_MIX_BLEND_MODE_LENGTH {
            return Err(format!("Mix blend mode string exceeds maximum length of {} characters", MAX_MIX_BLEND_MODE_LENGTH));
        }
        
        // Validate mix blend mode value
        MixBlendMode::from_str(mix_blend_mode)?;
        
        Ok(())
    }

    /// Apply mix blend mode to a node
    /// 
    /// # Arguments
    /// * `node` - The node to apply mix blend mode to
    /// * `mix_blend_mode` - The mix blend mode to apply
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates mix blend mode string
    pub fn apply_mix_blend_mode(&mut self, node: &mut TipTapNode, mix_blend_mode: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate mix blend mode
        self.validate_mix_blend_mode(mix_blend_mode)?;

        // Apply mix blend mode to node attributes
        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("mixBlendMode".to_string(), serde_json::Value::String(mix_blend_mode.to_string()));
            }
        } else {
            node.attrs = Some(serde_json::json!({ "mixBlendMode": mix_blend_mode }));
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Mix blend mode application CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Mix blend mode application performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Remove mix blend mode from a node
    /// 
    /// # Arguments
    /// * `node` - The node to remove mix blend mode from
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn remove_mix_blend_mode(&mut self, node: &mut TipTapNode) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(ref mut attrs) = node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.remove("mixBlendMode");
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Mix blend mode removal CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Mix blend mode removal performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Get mix blend mode from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get mix blend mode from
    /// 
    /// # Returns
    /// Option containing the mix blend mode string or None
    pub fn get_mix_blend_mode(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(mix_blend_mode) = obj.get("mixBlendMode") {
                    if let Some(s) = mix_blend_mode.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Check if node has mix blend mode
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node has mix blend mode, false otherwise
    pub fn has_mix_blend_mode(&self, node: &TipTapNode) -> bool {
        self.get_mix_blend_mode(node).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_mix_blend_mode_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MixBlendModeManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(MixBlendModeManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(MixBlendModeManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(MixBlendModeManager::max_mix_blend_mode_length(), MAX_MIX_BLEND_MODE_LENGTH);
    }

    #[test]
    fn test_mix_blend_mode_variants() {
        assert_eq!(MixBlendMode::Normal.as_str(), "normal");
        assert_eq!(MixBlendMode::Multiply.as_str(), "multiply");
        assert_eq!(MixBlendMode::Screen.as_str(), "screen");
        assert_eq!(MixBlendMode::Overlay.as_str(), "overlay");
        assert_eq!(MixBlendMode::Difference.as_str(), "difference");
    }

    #[test]
    fn test_mix_blend_mode_from_str() {
        assert!(matches!(MixBlendMode::from_str("normal"), Ok(MixBlendMode::Normal)));
        assert!(matches!(MixBlendMode::from_str("multiply"), Ok(MixBlendMode::Multiply)));
        assert!(matches!(MixBlendMode::from_str("screen"), Ok(MixBlendMode::Screen)));
        assert!(matches!(MixBlendMode::from_str("overlay"), Ok(MixBlendMode::Overlay)));
        assert!(MixBlendMode::from_str("invalid").is_err());
    }

    #[test]
    fn test_apply_mix_blend_mode() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MixBlendModeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_mix_blend_mode(&mut node, "multiply");
        assert!(result.is_ok());
        assert!(manager.has_mix_blend_mode(&node));
    }

    #[test]
    fn test_apply_mix_blend_mode_invalid() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MixBlendModeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let result = manager.apply_mix_blend_mode(&mut node, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_mix_blend_mode_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MixBlendModeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let long_mix_blend_mode = "a".repeat(MAX_MIX_BLEND_MODE_LENGTH + 1);
        let result = manager.apply_mix_blend_mode(&mut node, &long_mix_blend_mode);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_mix_blend_mode() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MixBlendModeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "mixBlendMode": "screen" })),
            marks: None,
        };
        
        assert!(manager.has_mix_blend_mode(&node));
        let result = manager.remove_mix_blend_mode(&mut node);
        assert!(result.is_ok());
        assert!(!manager.has_mix_blend_mode(&node));
    }

    #[test]
    fn test_get_mix_blend_mode() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MixBlendModeManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "mixBlendMode": "overlay" })),
            marks: None,
        };
        
        let mix_blend_mode = manager.get_mix_blend_mode(&node);
        assert_eq!(mix_blend_mode, Some("overlay".to_string()));
    }

    #[test]
    fn test_get_mix_blend_mode_none() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MixBlendModeManager::new(config_service);
        
        let node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        let mix_blend_mode = manager.get_mix_blend_mode(&node);
        assert!(mix_blend_mode.is_none());
    }

    #[test]
    fn test_has_mix_blend_mode() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MixBlendModeManager::new(config_service);
        
        let node_with = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: Some(serde_json::json!({ "mixBlendMode": "difference" })),
            marks: None,
        };
        
        let node_without = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.has_mix_blend_mode(&node_with));
        assert!(!manager.has_mix_blend_mode(&node_without));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MixBlendModeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_mix_blend_mode(&mut node, "multiply").unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MixBlendModeManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        manager.apply_mix_blend_mode(&mut node, "multiply").unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MixBlendModeManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        
        let last_error = manager.get_last_error();
        assert!(last_error.is_some());
        let error = last_error.unwrap();
        assert_eq!(error.code, "TEST_ERROR");
        assert_eq!(error.message, "Test error message");
        assert_eq!(error.source, "test_source");
    }

    #[test]
    fn test_error_state_reset() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MixBlendModeManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
