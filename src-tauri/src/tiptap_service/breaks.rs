//! TipTap Breaks Manager - Aerospace-Grade Hard/Soft Break Operations Service
//!
//! Safety-critical break operations service with:
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

/// Maximum break depth to prevent stack overflow
const MAX_BREAK_DEPTH: usize = 10;

/// Break type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakType {
    HardBreak,
    SoftBreak,
}

impl BreakType {
    /// Convert break type to string
    pub fn as_str(&self) -> &str {
        match self {
            BreakType::HardBreak => "hardBreak",
            BreakType::SoftBreak => "softBreak",
        }
    }

    /// Parse break type from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "hardbreak" | "hard_break" | "hard" => Ok(BreakType::HardBreak),
            "softbreak" | "soft_break" | "soft" => Ok(BreakType::SoftBreak),
            _ => Err(format!("Invalid break type: {}", s)),
        }
    }
}

pub struct BreaksManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl BreaksManager {
    /// Creates a new breaks manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new BreaksManager instance
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

    /// Get the maximum break depth constant
    /// 
    /// # Returns
    /// The maximum break depth
    pub fn max_break_depth() -> usize {
        MAX_BREAK_DEPTH
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

    /// Validate break depth
    /// 
    /// # Arguments
    /// * `depth` - The current break depth
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents stack overflow by limiting depth
    fn validate_break_depth(&self, depth: usize) -> Result<(), String> {
        if depth >= MAX_BREAK_DEPTH {
            return Err(format!("Break depth exceeds maximum of {}", MAX_BREAK_DEPTH));
        }
        Ok(())
    }

    /// Create a hard break node
    /// 
    /// # Arguments
    /// * `depth` - The nesting depth
    /// 
    /// # Returns
    /// Result containing the hard break node or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates break depth
    pub fn create_hard_break(&mut self, depth: usize) -> Result<TipTapNode, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate depth
        self.validate_break_depth(depth)?;

        let hard_break_node = TipTapNode {
            node_type: NodeType::HardBreak,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Hard break creation CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Hard break creation performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(hard_break_node)
    }

    /// Create a soft break node
    /// 
    /// # Arguments
    /// * `depth` - The nesting depth
    /// 
    /// # Returns
    /// Result containing the soft break node or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates break depth
    pub fn create_soft_break(&mut self, depth: usize) -> Result<TipTapNode, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate depth
        self.validate_break_depth(depth)?;

        let soft_break_node = TipTapNode {
            node_type: NodeType::SoftBreak,
            content: None,
            text: None,
            attrs: None,
            marks: None,
        };

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Soft break creation CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Soft break creation performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(soft_break_node)
    }

    /// Insert a hard break into a node's content
    /// 
    /// # Arguments
    /// * `node` - The node to insert the break into
    /// * `position` - The position to insert at
    /// * `depth` - The nesting depth
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn insert_hard_break(&mut self, node: &mut TipTapNode, position: usize, depth: usize) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate depth
        self.validate_break_depth(depth)?;

        let hard_break = self.create_hard_break(depth)?;

        if let Some(ref mut content) = node.content {
            if position <= content.len() {
                content.insert(position, hard_break);
            } else {
                return Err(format!("Position {} exceeds content length {}", position, content.len()));
            }
        } else {
            node.content = Some(vec![hard_break]);
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Hard break insertion CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Hard break insertion performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Insert a soft break into a node's content
    /// 
    /// # Arguments
    /// * `node` - The node to insert the break into
    /// * `position` - The position to insert at
    /// * `depth` - The nesting depth
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn insert_soft_break(&mut self, node: &mut TipTapNode, position: usize, depth: usize) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate depth
        self.validate_break_depth(depth)?;

        let soft_break = self.create_soft_break(depth)?;

        if let Some(ref mut content) = node.content {
            if position <= content.len() {
                content.insert(position, soft_break);
            } else {
                return Err(format!("Position {} exceeds content length {}", position, content.len()));
            }
        } else {
            node.content = Some(vec![soft_break]);
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Soft break insertion CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Soft break insertion performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Check if a node is a hard break
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node is a hard break, false otherwise
    pub fn is_hard_break(&self, node: &TipTapNode) -> bool {
        node.node_type == NodeType::HardBreak
    }

    /// Check if a node is a soft break
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node is a soft break, false otherwise
    pub fn is_soft_break(&self, node: &TipTapNode) -> bool {
        node.node_type == NodeType::SoftBreak
    }

    /// Check if a node is any type of break
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node is a break, false otherwise
    pub fn is_break(&self, node: &TipTapNode) -> bool {
        self.is_hard_break(node) || self.is_soft_break(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_breaks_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BreaksManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(BreaksManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(BreaksManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(BreaksManager::max_break_depth(), MAX_BREAK_DEPTH);
    }

    #[test]
    fn test_break_type_variants() {
        assert_eq!(BreakType::HardBreak.as_str(), "hardBreak");
        assert_eq!(BreakType::SoftBreak.as_str(), "softBreak");
    }

    #[test]
    fn test_break_type_from_str() {
        assert!(matches!(BreakType::from_str("hardBreak"), Ok(BreakType::HardBreak)));
        assert!(matches!(BreakType::from_str("softBreak"), Ok(BreakType::SoftBreak)));
        assert!(matches!(BreakType::from_str("hard"), Ok(BreakType::HardBreak)));
        assert!(matches!(BreakType::from_str("soft"), Ok(BreakType::SoftBreak)));
        assert!(BreakType::from_str("invalid").is_err());
    }

    #[test]
    fn test_create_hard_break() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BreaksManager::new(config_service);
        
        let result = manager.create_hard_break(0);
        assert!(result.is_ok());
        let node = result.unwrap();
        assert!(manager.is_hard_break(&node));
    }

    #[test]
    fn test_create_hard_break_too_deep() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BreaksManager::new(config_service);
        
        let result = manager.create_hard_break(MAX_BREAK_DEPTH);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_soft_break() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BreaksManager::new(config_service);
        
        let result = manager.create_soft_break(0);
        assert!(result.is_ok());
        let node = result.unwrap();
        assert!(manager.is_soft_break(&node));
    }

    #[test]
    fn test_create_soft_break_too_deep() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BreaksManager::new(config_service);
        
        let result = manager.create_soft_break(MAX_BREAK_DEPTH);
        assert!(result.is_err());
    }

    #[test]
    fn test_insert_hard_break() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BreaksManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: Some(vec![TipTapNode {
                node_type: NodeType::Text,
                content: None,
                text: Some("Test".to_string()),
                attrs: None,
                marks: None,
            }]),
            text: None,
            attrs: None,
            marks: None,
        };
        
        let result = manager.insert_hard_break(&mut node, 1, 0);
        assert!(result.is_ok());
        assert_eq!(node.content.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_insert_hard_break_invalid_position() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BreaksManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: Some(vec![TipTapNode {
                node_type: NodeType::Text,
                content: None,
                text: Some("Test".to_string()),
                attrs: None,
                marks: None,
            }]),
            text: None,
            attrs: None,
            marks: None,
        };
        
        let result = manager.insert_hard_break(&mut node, 10, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_insert_soft_break() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BreaksManager::new(config_service);
        
        let mut node = TipTapNode {
            node_type: NodeType::Paragraph,
            content: Some(vec![TipTapNode {
                node_type: NodeType::Text,
                content: None,
                text: Some("Test".to_string()),
                attrs: None,
                marks: None,
            }]),
            text: None,
            attrs: None,
            marks: None,
        };
        
        let result = manager.insert_soft_break(&mut node, 1, 0);
        assert!(result.is_ok());
        assert_eq!(node.content.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_is_hard_break() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BreaksManager::new(config_service);
        
        let hard_break = manager.create_hard_break(0).unwrap();
        let soft_break = manager.create_soft_break(0).unwrap();
        let text_node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.is_hard_break(&hard_break));
        assert!(!manager.is_hard_break(&soft_break));
        assert!(!manager.is_hard_break(&text_node));
    }

    #[test]
    fn test_is_soft_break() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BreaksManager::new(config_service);
        
        let hard_break = manager.create_hard_break(0).unwrap();
        let soft_break = manager.create_soft_break(0).unwrap();
        let text_node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(!manager.is_soft_break(&hard_break));
        assert!(manager.is_soft_break(&soft_break));
        assert!(!manager.is_soft_break(&text_node));
    }

    #[test]
    fn test_is_break() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BreaksManager::new(config_service);
        
        let hard_break = manager.create_hard_break(0).unwrap();
        let soft_break = manager.create_soft_break(0).unwrap();
        let text_node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.is_break(&hard_break));
        assert!(manager.is_break(&soft_break));
        assert!(!manager.is_break(&text_node));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BreaksManager::new(config_service);
        
        manager.create_hard_break(0).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BreaksManager::new(config_service);
        
        manager.create_hard_break(0).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BreaksManager::new(config_service);
        
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
        let mut manager = BreaksManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
