//! TipTap Embedded Content Manager - Aerospace-Grade Embedded Content Operations Service
//!
//! Safety-critical embedded content operations service with:
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

/// Maximum URL length for embedded content
const MAX_EMBED_URL_LENGTH: usize = 2048;

/// Maximum embed depth to prevent stack overflow
const MAX_EMBED_DEPTH: usize = 10;

/// Maximum embed width
const MAX_EMBED_WIDTH: usize = 4000;

/// Maximum embed height
const MAX_EMBED_HEIGHT: usize = 4000;

/// Embedded content type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmbedType {
    YouTube,
    Vimeo,
    Twitter,
    Iframe,
    Video,
    Audio,
    Other,
}

impl EmbedType {
    /// Convert embed type to string
    pub fn as_str(&self) -> &str {
        match self {
            EmbedType::YouTube => "youtube",
            EmbedType::Vimeo => "vimeo",
            EmbedType::Twitter => "twitter",
            EmbedType::Iframe => "iframe",
            EmbedType::Video => "video",
            EmbedType::Audio => "audio",
            EmbedType::Other => "other",
        }
    }

    /// Parse embed type from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "youtube" => Ok(EmbedType::YouTube),
            "vimeo" => Ok(EmbedType::Vimeo),
            "twitter" => Ok(EmbedType::Twitter),
            "iframe" => Ok(EmbedType::Iframe),
            "video" => Ok(EmbedType::Video),
            "audio" => Ok(EmbedType::Audio),
            _ => Ok(EmbedType::Other),
        }
    }
}

/// Embedded content attributes
#[derive(Debug, Clone)]
pub struct EmbedAttributes {
    pub url: String,
    pub embed_type: EmbedType,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub autoplay: bool,
}

impl EmbedAttributes {
    /// Create new embed attributes
    pub fn new(url: String, embed_type: EmbedType) -> Self {
        Self {
            url,
            embed_type,
            width: None,
            height: None,
            autoplay: false,
        }
    }
}

pub struct EmbeddedManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl EmbeddedManager {
    /// Creates a new embedded manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new EmbeddedManager instance
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

    /// Get the maximum embed URL length constant
    /// 
    /// # Returns
    /// The maximum embed URL length
    pub fn max_embed_url_length() -> usize {
        MAX_EMBED_URL_LENGTH
    }

    /// Get the maximum embed depth constant
    /// 
    /// # Returns
    /// The maximum embed depth
    pub fn max_embed_depth() -> usize {
        MAX_EMBED_DEPTH
    }

    /// Get the maximum embed width constant
    /// 
    /// # Returns
    /// The maximum embed width
    pub fn max_embed_width() -> usize {
        MAX_EMBED_WIDTH
    }

    /// Get the maximum embed height constant
    /// 
    /// # Returns
    /// The maximum embed height
    pub fn max_embed_height() -> usize {
        MAX_EMBED_HEIGHT
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

    /// Validate embed URL
    /// 
    /// # Arguments
    /// * `url` - The embed URL to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting URL length
    fn validate_embed_url(&self, url: &str) -> Result<(), String> {
        if url.is_empty() {
            return Err("Embed URL cannot be empty".to_string());
        }
        if url.len() > MAX_EMBED_URL_LENGTH {
            return Err(format!("Embed URL exceeds maximum length of {} characters", MAX_EMBED_URL_LENGTH));
        }
        // Basic URL validation
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err("Embed URL must start with http:// or https://".to_string());
        }
        Ok(())
    }

    /// Validate embed depth
    /// 
    /// # Arguments
    /// * `depth` - The current embed depth
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents stack overflow by limiting depth
    fn validate_embed_depth(&self, depth: usize) -> Result<(), String> {
        if depth >= MAX_EMBED_DEPTH {
            return Err(format!("Embed depth exceeds maximum of {}", MAX_EMBED_DEPTH));
        }
        Ok(())
    }

    /// Validate embed dimensions
    /// 
    /// # Arguments
    /// * `width` - The embed width
    /// * `height` - The embed height
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents rendering issues by limiting dimensions
    fn validate_embed_dimensions(&self, width: Option<usize>, height: Option<usize>) -> Result<(), String> {
        if let Some(w) = width {
            if w > MAX_EMBED_WIDTH {
                return Err(format!("Embed width exceeds maximum of {} pixels", MAX_EMBED_WIDTH));
            }
        }
        if let Some(h) = height {
            if h > MAX_EMBED_HEIGHT {
                return Err(format!("Embed height exceeds maximum of {} pixels", MAX_EMBED_HEIGHT));
            }
        }
        Ok(())
    }

    /// Create an embedded content node
    /// 
    /// # Arguments
    /// * `attributes` - The embed attributes
    /// * `depth` - The nesting depth
    /// 
    /// # Returns
    /// Result containing the embedded content node or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates URL, depth, and dimensions
    pub fn create_embed(&mut self, attributes: EmbedAttributes, depth: usize) -> Result<TipTapNode, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate URL, depth, and dimensions
        self.validate_embed_url(&attributes.url)?;
        self.validate_embed_depth(depth)?;
        self.validate_embed_dimensions(attributes.width, attributes.height)?;

        let mut attrs_obj = serde_json::Map::new();
        attrs_obj.insert("src".to_string(), serde_json::Value::String(attributes.url.clone()));
        attrs_obj.insert("embedType".to_string(), serde_json::Value::String(attributes.embed_type.as_str().to_string()));
        attrs_obj.insert("autoplay".to_string(), serde_json::Value::Bool(attributes.autoplay));
        
        if let Some(width) = attributes.width {
            attrs_obj.insert("width".to_string(), serde_json::Value::Number(width.into()));
        }
        if let Some(height) = attributes.height {
            attrs_obj.insert("height".to_string(), serde_json::Value::Number(height.into()));
        }

        let embed_node = TipTapNode {
            node_type: NodeType::Image, // Using Image as a placeholder for embed content
            content: None,
            text: None,
            attrs: Some(serde_json::Value::Object(attrs_obj)),
            marks: None,
        };

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Embed creation CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Embed creation performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(embed_node)
    }

    /// Update embed URL
    /// 
    /// # Arguments
    /// * `embed_node` - The embed node to update
    /// * `new_url` - The new URL
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn update_embed_url(&mut self, embed_node: &mut TipTapNode, new_url: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate URL
        self.validate_embed_url(new_url)?;

        if let Some(ref mut attrs) = embed_node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("src".to_string(), serde_json::Value::String(new_url.to_string()));
            }
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Embed URL update CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Embed URL update performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Check if a node is an embedded content
    /// 
    /// # Arguments
    /// * `node` - The node to check
    /// 
    /// # Returns
    /// True if node is an embedded content, false otherwise
    pub fn is_embed(&self, node: &TipTapNode) -> bool {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(embed_type) = obj.get("embedType") {
                    return embed_type.is_string();
                }
            }
        }
        false
    }

    /// Get embed URL from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get URL from
    /// 
    /// # Returns
    /// Option containing the URL or None
    pub fn get_embed_url(&self, node: &TipTapNode) -> Option<String> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(src) = obj.get("src") {
                    if let Some(s) = src.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
        None
    }

    /// Get embed type from a node
    /// 
    /// # Arguments
    /// * `node` - The node to get type from
    /// 
    /// # Returns
    /// Option containing the embed type or None
    pub fn get_embed_type(&self, node: &TipTapNode) -> Option<EmbedType> {
        if let Some(ref attrs) = node.attrs {
            if let Some(obj) = attrs.as_object() {
                if let Some(embed_type) = obj.get("embedType") {
                    if let Some(s) = embed_type.as_str() {
                        return EmbedType::from_str(s).ok();
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
    fn test_embedded_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = EmbeddedManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(EmbeddedManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(EmbeddedManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(EmbeddedManager::max_embed_url_length(), MAX_EMBED_URL_LENGTH);
        assert_eq!(EmbeddedManager::max_embed_depth(), MAX_EMBED_DEPTH);
        assert_eq!(EmbeddedManager::max_embed_width(), MAX_EMBED_WIDTH);
        assert_eq!(EmbeddedManager::max_embed_height(), MAX_EMBED_HEIGHT);
    }

    #[test]
    fn test_embed_type_variants() {
        assert_eq!(EmbedType::YouTube.as_str(), "youtube");
        assert_eq!(EmbedType::Vimeo.as_str(), "vimeo");
        assert_eq!(EmbedType::Twitter.as_str(), "twitter");
    }

    #[test]
    fn test_embed_type_from_str() {
        assert!(matches!(EmbedType::from_str("youtube"), Ok(EmbedType::YouTube)));
        assert!(matches!(EmbedType::from_str("vimeo"), Ok(EmbedType::Vimeo)));
        assert!(matches!(EmbedType::from_str("twitter"), Ok(EmbedType::Twitter)));
        assert!(matches!(EmbedType::from_str("unknown"), Ok(EmbedType::Other)));
    }

    #[test]
    fn test_create_embed() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = EmbeddedManager::new(config_service);
        
        let attributes = EmbedAttributes::new("https://youtube.com/watch?v=123".to_string(), EmbedType::YouTube);
        let result = manager.create_embed(attributes, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_embed_empty_url() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = EmbeddedManager::new(config_service);
        
        let attributes = EmbedAttributes::new("".to_string(), EmbedType::YouTube);
        let result = manager.create_embed(attributes, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_embed_invalid_url() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = EmbeddedManager::new(config_service);
        
        let attributes = EmbedAttributes::new("invalid-url".to_string(), EmbedType::YouTube);
        let result = manager.create_embed(attributes, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_embed_url_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = EmbeddedManager::new(config_service);
        
        let long_url = format!("https://example.com/{}", "a".repeat(MAX_EMBED_URL_LENGTH));
        let attributes = EmbedAttributes::new(long_url, EmbedType::YouTube);
        let result = manager.create_embed(attributes, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_embed_too_deep() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = EmbeddedManager::new(config_service);
        
        let attributes = EmbedAttributes::new("https://youtube.com/watch?v=123".to_string(), EmbedType::YouTube);
        let result = manager.create_embed(attributes, MAX_EMBED_DEPTH);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_embed_dimensions_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = EmbeddedManager::new(config_service);
        
        let mut attributes = EmbedAttributes::new("https://youtube.com/watch?v=123".to_string(), EmbedType::YouTube);
        attributes.width = Some(MAX_EMBED_WIDTH + 1);
        let result = manager.create_embed(attributes, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_embed_url() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = EmbeddedManager::new(config_service);
        
        let mut embed_node = manager.create_embed(EmbedAttributes::new("https://youtube.com/watch?v=123".to_string(), EmbedType::YouTube), 0).unwrap();
        let result = manager.update_embed_url(&mut embed_node, "https://youtube.com/watch?v=456");
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_embed() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = EmbeddedManager::new(config_service);
        
        let embed_node = manager.create_embed(EmbedAttributes::new("https://youtube.com/watch?v=123".to_string(), EmbedType::YouTube), 0).unwrap();
        let text_node = TipTapNode {
            node_type: NodeType::Text,
            content: None,
            text: Some("Test".to_string()),
            attrs: None,
            marks: None,
        };
        
        assert!(manager.is_embed(&embed_node));
        assert!(!manager.is_embed(&text_node));
    }

    #[test]
    fn test_get_embed_url() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = EmbeddedManager::new(config_service);
        
        let embed_node = manager.create_embed(EmbedAttributes::new("https://youtube.com/watch?v=123".to_string(), EmbedType::YouTube), 0).unwrap();
        let url = manager.get_embed_url(&embed_node);
        assert_eq!(url, Some("https://youtube.com/watch?v=123".to_string()));
    }

    #[test]
    fn test_get_embed_type() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = EmbeddedManager::new(config_service);
        
        let embed_node = manager.create_embed(EmbedAttributes::new("https://youtube.com/watch?v=123".to_string(), EmbedType::YouTube), 0).unwrap();
        let embed_type = manager.get_embed_type(&embed_node);
        assert_eq!(embed_type, Some(EmbedType::YouTube));
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = EmbeddedManager::new(config_service);
        
        manager.create_embed(EmbedAttributes::new("https://youtube.com/watch?v=123".to_string(), EmbedType::YouTube), 0).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = EmbeddedManager::new(config_service);
        
        manager.create_embed(EmbedAttributes::new("https://youtube.com/watch?v=123".to_string(), EmbedType::YouTube), 0).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = EmbeddedManager::new(config_service);
        
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
        let mut manager = EmbeddedManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
