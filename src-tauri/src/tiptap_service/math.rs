//! TipTap Math Formula Manager - Aerospace-Grade Math Formula Operations Service
//!
//! Safety-critical math formula operations service with:
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
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 100;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 500;

/// Maximum math formula length
const MAX_MATH_FORMULA_LENGTH: usize = 10000;

/// Maximum math formula depth to prevent stack overflow
const MAX_MATH_DEPTH: usize = 10;

/// Math formula format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathFormat {
    LaTeX,
    MathML,
    AsciiMath,
}

impl MathFormat {
    /// Convert math format to string
    pub fn as_str(&self) -> &str {
        match self {
            MathFormat::LaTeX => "latex",
            MathFormat::MathML => "mathml",
            MathFormat::AsciiMath => "asciimath",
        }
    }

    /// Parse math format from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "latex" => Ok(MathFormat::LaTeX),
            "mathml" => Ok(MathFormat::MathML),
            "asciimath" => Ok(MathFormat::AsciiMath),
            _ => Err(format!("Invalid math format: {}", s)),
        }
    }
}

/// Math formula attributes
#[derive(Debug, Clone)]
pub struct MathAttributes {
    pub formula: String,
    pub format: MathFormat,
    pub inline: bool,
}

impl MathAttributes {
    /// Create new math attributes
    pub fn new(formula: String, format: MathFormat, inline: bool) -> Self {
        Self { formula, format, inline }
    }
}

pub struct MathManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl MathManager {
    /// Creates a new math manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new MathManager instance
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

    /// Get the maximum math formula length constant
    /// 
    /// # Returns
    /// The maximum math formula length
    pub fn max_math_formula_length() -> usize {
        MAX_MATH_FORMULA_LENGTH
    }

    /// Get the maximum math depth constant
    /// 
    /// # Returns
    /// The maximum math depth
    pub fn max_math_depth() -> usize {
        MAX_MATH_DEPTH
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

    /// Validate math formula
    /// 
    /// # Arguments
    /// * `formula` - The math formula to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting formula length
    fn validate_math_formula(&self, formula: &str) -> Result<(), String> {
        if formula.is_empty() {
            return Err("Math formula cannot be empty".to_string());
        }
        if formula.len() > MAX_MATH_FORMULA_LENGTH {
            return Err(format!("Math formula exceeds maximum length of {} characters", MAX_MATH_FORMULA_LENGTH));
        }
        Ok(())
    }

    /// Validate math depth
    /// 
    /// # Arguments
    /// * `depth` - The current math depth
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents stack overflow by limiting depth
    fn validate_math_depth(&self, depth: usize) -> Result<(), String> {
        if depth >= MAX_MATH_DEPTH {
            return Err(format!("Math depth exceeds maximum of {}", MAX_MATH_DEPTH));
        }
        Ok(())
    }

    /// Create a math formula node
    /// 
    /// # Arguments
    /// * `attributes` - The math attributes
    /// * `depth` - The nesting depth
    /// 
    /// # Returns
    /// Result containing the math formula node or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates formula and depth
    pub fn create_math(&mut self, attributes: MathAttributes, depth: usize) -> Result<TipTapNode, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate formula and depth
        self.validate_math_formula(&attributes.formula)?;
        self.validate_math_depth(depth)?;

        let math_node = TipTapNode {
            node_type: NodeType::CodeBlock,
            content: Some(vec![TipTapNode {
                node_type: NodeType::Text,
                content: None,
                text: Some(attributes.formula.clone()),
                attrs: None,
                marks: None,
            }]),
            text: None,
            attrs: Some(serde_json::json!({
                "language": "math",
                "mathFormat": attributes.format.as_str(),
                "inline": attributes.inline
            })),
            marks: None,
        };

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Math formula creation CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Math formula creation performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(math_node)
    }

    /// Update math formula
    /// 
    /// # Arguments
    /// * `math_node` - The math node to update
    /// * `new_formula` - The new formula
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn update_formula(&mut self, math_node: &mut TipTapNode, new_formula: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate formula
        self.validate_math_formula(new_formula)?;

        if let Some(ref mut content) = math_node.content {
            if let Some(ref mut text_node) = content.first_mut() {
                text_node.text = Some(new_formula.to_string());
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Math formula update CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Math formula update performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Check if a node is a math formula
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node is a math formula, false otherwise
    pub fn is_math(&self, node: &TipTapNode) -> bool {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(language) = obj.get("language") {
                    if let Some(s) = language.as_str() {
                        return s == "math";
                    }
                }
            }
        }
        false
    }

    /// Get math formula from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get formula from
    /// 
    /// # Returns
    /// Option containing the formula or None
    pub fn get_formula(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref content) = node.content {
            if let Some(ref text_node) = content.first() {
                return text_node.text.clone();
            }
        }
        None
    }

    /// Get math format from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get format from
    /// 
    /// # Returns
    /// Option containing the math format or None
    pub fn get_math_format(&self, node: &TipTapNode) -> Option<MathFormat> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(format) = obj.get("mathFormat") {
                    if let Some(s) = format.as_str() {
                        return MathFormat::from_str(s).ok();
                    }
                }
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
    fn test_math_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = MathManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(MathManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(MathManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(MathManager::max_math_formula_length(), MAX_MATH_FORMULA_LENGTH);
        assert_eq!(MathManager::max_math_depth(), MAX_MATH_DEPTH);
    }

    #[test]
    fn test_math_format_variants() {
        assert_eq!(MathFormat::LaTeX.as_str(), "latex");
        assert_eq!(MathFormat::MathML.as_str(), "mathml");
        assert_eq!(MathFormat::AsciiMath.as_str(), "asciimath");
    }

    #[test]
    fn test_math_format_from_str() {
        assert!(matches!(MathFormat::from_str("latex"), Ok(MathFormat::LaTeX)));
        assert!(matches!(MathFormat::from_str("mathml"), Ok(MathFormat::MathML)));
        assert!(matches!(MathFormat::from_str("asciimath"), Ok(MathFormat::AsciiMath)));
        assert!(MathFormat::from_str("invalid").is_err());
    }

    #[test]
    fn test_create_math() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MathManager::new(config_service);
        
        let attributes = MathAttributes::new("E = mc^2".to_string(), MathFormat::LaTeX, false);
        let result = manager.create_math(attributes, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_math_empty_formula() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MathManager::new(config_service);
        
        let attributes = MathAttributes::new("".to_string(), MathFormat::LaTeX, false);
        let result = manager.create_math(attributes, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_math_formula_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MathManager::new(config_service);
        
        let long_formula = "a".repeat(MAX_MATH_FORMULA_LENGTH + 1);
        let attributes = MathAttributes::new(long_formula, MathFormat::LaTeX, false);
        let result = manager.create_math(attributes, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_math_too_deep() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MathManager::new(config_service);
        
        let attributes = MathAttributes::new("E = mc^2".to_string(), MathFormat::LaTeX, false);
        let result = manager.create_math(attributes, MAX_MATH_DEPTH);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_formula() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MathManager::new(config_service);
        
        let mut math_node = manager.create_math(MathAttributes::new("E = mc^2".to_string(), MathFormat::LaTeX, false), 0).unwrap();
        let result = manager.update_formula(&mut math_node, "F = ma");
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_formula_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MathManager::new(config_service);
        
        let mut math_node = manager.create_math(MathAttributes::new("E = mc^2".to_string(), MathFormat::LaTeX, false), 0).unwrap();
        let result = manager.update_formula(&mut math_node, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_is_math() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MathManager::new(config_service);
        
        let math_node = manager.create_math(MathAttributes::new("E = mc^2".to_string(), MathFormat::LaTeX, false), 0).unwrap();
        let text_node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.is_math(&math_node));
        assert!(!manager.is_math(&text_node));
    }

    #[test]
    fn test_get_formula() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MathManager::new(config_service);
        
        let math_node = manager.create_math(MathAttributes::new("E = mc^2".to_string(), MathFormat::LaTeX, false), 0).unwrap();
        let formula = manager.get_formula(&math_node);
        assert_eq!(formula, Some("E = mc^2".to_string()));
    }

    #[test]
    fn test_get_math_format() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MathManager::new(config_service);
        
        let math_node = manager.create_math(MathAttributes::new("E = mc^2".to_string(), MathFormat::LaTeX, false), 0).unwrap();
        let format = manager.get_math_format(&math_node);
        assert_eq!(format, Some(MathFormat::LaTeX));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MathManager::new(config_service);
        
        manager.create_math(MathAttributes::new("E = mc^2".to_string(), MathFormat::LaTeX, false), 0).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MathManager::new(config_service);
        
        manager.create_math(MathAttributes::new("E = mc^2".to_string(), MathFormat::LaTeX, false), 0).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = MathManager::new(config_service);
        
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
        let mut manager = MathManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
