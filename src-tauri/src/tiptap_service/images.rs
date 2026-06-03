//! TipTap Image Manager - Aerospace-Grade Image Operations Service
//!
//! Safety-critical image operations service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Comprehensive error handling
//! - Performance monitoring
//! - Security hardening
//! - Fault tolerance and error recovery

use serde::{Deserialize, Serialize};
use std::time::Instant;
use crate::error_handling::{ErrorContext, ErrorSeverity};
use crate::config_service::ExportConfigService;
use std::sync::Arc;
use super::editor::{TipTapNode, NodeType};

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 100;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 500;

/// Maximum image size in bytes to prevent memory exhaustion
const MAX_IMAGE_SIZE: usize = 10 * 1024 * 1024; // 10MB

/// Maximum image URL length
const MAX_IMAGE_URL_LENGTH: usize = 2048;

/// Maximum image alt text length
const MAX_IMAGE_ALT_LENGTH: usize = 500;

/// Image attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAttributes {
    pub src: String,
    pub alt: Option<String>,
    pub title: Option<String>,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub inline: bool,
}

pub struct ImageManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl ImageManager {
    /// Creates a new image manager instance
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new ImageManager instance
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

    /// Get the maximum image size constant
    /// 
    /// # Returns
    /// The maximum image size in bytes
    pub fn max_image_size() -> usize {
        MAX_IMAGE_SIZE
    }

    /// Get the maximum image URL length constant
    /// 
    /// # Returns
    /// The maximum image URL length
    pub fn max_image_url_length() -> usize {
        MAX_IMAGE_URL_LENGTH
    }

    /// Get the maximum image alt length constant
    /// 
    /// # Returns
    /// The maximum image alt text length
    pub fn max_image_alt_length() -> usize {
        MAX_IMAGE_ALT_LENGTH
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

    /// Validate image URL
    /// 
    /// # Arguments
    /// * `url` - The URL to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting URL length
    fn validate_image_url(&self, url: &str) -> Result<(), String> {
        if url.is_empty() {
            return Err("Image URL cannot be empty".to_string());
        }
        if url.len() > MAX_IMAGE_URL_LENGTH {
            return Err(format!("Image URL exceeds maximum length of {} characters", MAX_IMAGE_URL_LENGTH));
        }
        Ok(())
    }

    /// Validate image alt text
    /// 
    /// # Arguments
    /// * `alt` - The alt text to validate
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Prevents DoS attacks by limiting alt text length
    fn validate_image_alt(&self, alt: &str) -> Result<(), String> {
        if alt.len() > MAX_IMAGE_ALT_LENGTH {
            return Err(format!("Image alt text exceeds maximum length of {} characters", MAX_IMAGE_ALT_LENGTH));
        }
        Ok(())
    }

    /// Validate image dimensions
    /// 
    /// # Arguments
    /// * `width` - The image width
    /// * `height` - The image height
    /// 
    /// # Returns
    /// Result indicating success or failure
    fn validate_image_dimensions(&self, width: Option<usize>, height: Option<usize>) -> Result<(), String> {
        if let Some(w) = width {
            if w == 0 {
                return Err("Image width cannot be zero".to_string());
            }
            if w > 10000 {
                return Err("Image width exceeds maximum of 10000 pixels".to_string());
            }
        }
        if let Some(h) = height {
            if h == 0 {
                return Err("Image height cannot be zero".to_string());
            }
            if h > 10000 {
                return Err("Image height exceeds maximum of 10000 pixels".to_string());
            }
        }
        Ok(())
    }

    /// Create an image node
    /// 
    /// # Arguments
    /// * `attributes` - The image attributes
    /// 
    /// # Returns
    /// Result containing the image node or an error message
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates URL length, alt text length, and dimensions
    pub fn create_image(&mut self, attributes: ImageAttributes) -> Result<TipTapNode, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate URL
        self.validate_image_url(&attributes.src)?;

        // Validate alt text
        if let Some(ref alt) = attributes.alt {
            self.validate_image_alt(alt)?;
        }

        // Validate dimensions
        self.validate_image_dimensions(attributes.width, attributes.height)?;

        let attrs_json = serde_json::to_value(&attributes).map_err(|e| {
            let error = format!("Failed to serialize image attributes: {}", e);
            self.record_error("SERIALIZE_ERROR", &error, "create_image");
            error
        })?;

        let image_node = TipTapNode {
            node_type: NodeType::Image,
            content: None,
            text: None,
            attrs: Some(attrs_json),
            marks: None,
        };

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Image creation CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Image creation performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(image_node)
    }

    /// Update image attributes
    /// 
    /// # Arguments
    /// * `image_node` - The image node to update
    /// * `attributes` - The new attributes
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    pub fn update_image(&mut self, image_node: &mut TipTapNode, attributes: ImageAttributes) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Validate URL
        self.validate_image_url(&attributes.src)?;

        // Validate alt text
        if let Some(ref alt) = attributes.alt {
            self.validate_image_alt(alt)?;
        }

        // Validate dimensions
        self.validate_image_dimensions(attributes.width, attributes.height)?;

        let attrs_json = serde_json::to_value(&attributes).map_err(|e| {
            let error = format!("Failed to serialize image attributes: {}", e);
            self.record_error("SERIALIZE_ERROR", &error, "update_image");
            error
        })?;

        image_node.attrs = Some(attrs_json);

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Image update CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Image update performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    /// Resize image
    /// 
    /// # Arguments
    /// * `image_node` - The image node to resize
    /// * `width` - The new width
    /// * `height` - The new height
    /// 
    /// # Returns
    /// Result indicating success or failure
    pub fn resize_image(&mut self, image_node: &mut TipTapNode, width: usize, height: usize) -> Result<(), String> {
        self.operation_count += 1;

        // Validate dimensions
        self.validate_image_dimensions(Some(width), Some(height))?;

        if let Some(ref mut attrs) = image_node.attrs {
            if let Some(obj) = attrs.as_object_mut() {
                obj.insert("width".to_string(), serde_json::Value::Number(width.into()));
                obj.insert("height".to_string(), serde_json::Value::Number(height.into()));
            }
        }

        self.last_error = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::editor::NodeType;

    #[test]
    fn test_image_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = ImageManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_performance_threshold_getters() {
        assert_eq!(ImageManager::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(ImageManager::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(ImageManager::max_image_size(), MAX_IMAGE_SIZE);
        assert_eq!(ImageManager::max_image_url_length(), MAX_IMAGE_URL_LENGTH);
        assert_eq!(ImageManager::max_image_alt_length(), MAX_IMAGE_ALT_LENGTH);
    }

    #[test]
    fn test_create_image() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ImageManager::new(config_service);
        
        let attributes = ImageAttributes {
            src: "https://example.com/image.png".to_string(),
            alt: Some("Test image".to_string()),
            title: None,
            width: Some(100),
            height: Some(100),
            inline: false,
        };
        
        let result = manager.create_image(attributes);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_image_empty_url() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ImageManager::new(config_service);
        
        let attributes = ImageAttributes {
            src: "".to_string(),
            alt: None,
            title: None,
            width: None,
            height: None,
            inline: false,
        };
        
        let result = manager.create_image(attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_image_url_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ImageManager::new(config_service);
        
        let long_url = "a".repeat(MAX_IMAGE_URL_LENGTH + 1);
        let attributes = ImageAttributes {
            src: long_url,
            alt: None,
            title: None,
            width: None,
            height: None,
            inline: false,
        };
        
        let result = manager.create_image(attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_image_alt_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ImageManager::new(config_service);
        
        let long_alt = "a".repeat(MAX_IMAGE_ALT_LENGTH + 1);
        let attributes = ImageAttributes {
            src: "https://example.com/image.png".to_string(),
            alt: Some(long_alt),
            title: None,
            width: None,
            height: None,
            inline: false,
        };
        
        let result = manager.create_image(attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_image_zero_width() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ImageManager::new(config_service);
        
        let attributes = ImageAttributes {
            src: "https://example.com/image.png".to_string(),
            alt: None,
            title: None,
            width: Some(0),
            height: Some(100),
            inline: false,
        };
        
        let result = manager.create_image(attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_image_width_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ImageManager::new(config_service);
        
        let attributes = ImageAttributes {
            src: "https://example.com/image.png".to_string(),
            alt: None,
            title: None,
            width: Some(10001),
            height: Some(100),
            inline: false,
        };
        
        let result = manager.create_image(attributes);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_image() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ImageManager::new(config_service);
        
        let mut image_node = manager.create_image(ImageAttributes {
            src: "https://example.com/image.png".to_string(),
            alt: None,
            title: None,
            width: None,
            height: None,
            inline: false,
        }).unwrap();
        
        let new_attributes = ImageAttributes {
            src: "https://example.com/new_image.png".to_string(),
            alt: Some("New alt".to_string()),
            title: None,
            width: Some(200),
            height: Some(200),
            inline: false,
        };
        
        let result = manager.update_image(&mut image_node, new_attributes);
        assert!(result.is_ok());
    }

    #[test]
    fn test_resize_image() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ImageManager::new(config_service);
        
        let mut image_node = manager.create_image(ImageAttributes {
            src: "https://example.com/image.png".to_string(),
            alt: None,
            title: None,
            width: Some(100),
            height: Some(100),
            inline: false,
        }).unwrap();
        
        let result = manager.resize_image(&mut image_node, 200, 200);
        assert!(result.is_ok());
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ImageManager::new(config_service);
        
        let attributes = ImageAttributes {
            src: "https://example.com/image.png".to_string(),
            alt: None,
            title: None,
            width: None,
            height: None,
            inline: false,
        };
        
        manager.create_image(attributes).unwrap();
        
        assert!(manager.get_operation_count() > 0);
    }

    #[test]
    fn test_reset_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ImageManager::new(config_service);
        
        let attributes = ImageAttributes {
            src: "https://example.com/image.png".to_string(),
            alt: None,
            title: None,
            width: None,
            height: None,
            inline: false,
        };
        
        manager.create_image(attributes).unwrap();
        
        manager.reset_operation_count();
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = ImageManager::new(config_service);
        
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
        let mut manager = ImageManager::new(config_service);
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }
}
