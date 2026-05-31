//! Aerospace-grade input validation and resource limits for PPT service
//! Ensures all inputs are validated and resource usage is controlled

use super::error::{PptError, PptResult};
use serde::{Deserialize, Serialize};

/// Resource limits configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum image data size in bytes (default: 10MB)
    pub max_image_size: usize,
    /// Maximum number of slides (default: 1000)
    pub max_slides: usize,
    /// Maximum table rows (default: 100)
    pub max_table_rows: usize,
    /// Maximum table columns (default: 100)
    pub max_table_columns: usize,
    /// Maximum file size in bytes (default: 100MB)
    pub max_file_size: usize,
    /// Minimum font size in points (default: 6)
    pub min_font_size: f64,
    /// Maximum font size in points (default: 72)
    pub max_font_size: f64,
    /// Maximum animation count per slide (default: 50)
    pub max_animations_per_slide: usize,
    /// Maximum text length per element (default: 10000)
    pub max_text_length: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_image_size: 10 * 1024 * 1024, // 10MB
            max_slides: 1000,
            max_table_rows: 100,
            max_table_columns: 100,
            max_file_size: 100 * 1024 * 1024, // 100MB
            min_font_size: 6.0,
            max_font_size: 72.0,
            max_animations_per_slide: 50,
            max_text_length: 10000,
        }
    }
}

impl ResourceLimits {
    /// Create resource limits with custom values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum image size
    pub fn with_max_image_size(mut self, size: usize) -> Self {
        self.max_image_size = size;
        self
    }

    /// Set maximum slides
    pub fn with_max_slides(mut self, count: usize) -> Self {
        self.max_slides = count;
        self
    }

    /// Set maximum table dimensions
    pub fn with_max_table_dimensions(mut self, rows: usize, cols: usize) -> Self {
        self.max_table_rows = rows;
        self.max_table_columns = cols;
        self
    }

    /// Set maximum file size
    pub fn with_max_file_size(mut self, size: usize) -> Self {
        self.max_file_size = size;
        self
    }

    /// Set font size range
    pub fn with_font_size_range(mut self, min: f64, max: f64) -> Self {
        self.min_font_size = min;
        self.max_font_size = max;
        self
    }
}

/// Validator for PPT inputs
pub struct Validator {
    limits: ResourceLimits,
}

impl Validator {
    /// Create a new validator with default limits
    pub fn new() -> Self {
        Self {
            limits: ResourceLimits::default(),
        }
    }

    /// Create a new validator with custom limits
    pub fn with_limits(limits: ResourceLimits) -> Self {
        Self { limits }
    }

    /// Get the resource limits
    pub fn limits(&self) -> &ResourceLimits {
        &self.limits
    }

    /// Validate image data size
    pub fn validate_image_size(&self, size: usize) -> PptResult<()> {
        if size > self.limits.max_image_size {
            return Err(PptError::image_data_too_large(size, self.limits.max_image_size));
        }
        Ok(())
    }

    /// Validate slide count
    pub fn validate_slide_count(&self, count: usize) -> PptResult<()> {
        if count > self.limits.max_slides {
            return Err(PptError::slide_count_limit_exceeded(self.limits.max_slides));
        }
        Ok(())
    }

    /// Validate table dimensions
    pub fn validate_table_dimensions(&self, rows: usize, cols: usize) -> PptResult<()> {
        if rows == 0 || cols == 0 {
            return Err(PptError::invalid_table_dimensions(rows, cols));
        }
        if rows > self.limits.max_table_rows {
            return Err(PptError::invalid_table_dimensions(rows, cols)
                .with_context(format!("rows exceed limit of {}", self.limits.max_table_rows)));
        }
        if cols > self.limits.max_table_columns {
            return Err(PptError::invalid_table_dimensions(rows, cols)
                .with_context(format!("columns exceed limit of {}", self.limits.max_table_columns)));
        }
        Ok(())
    }

    /// Validate file size
    pub fn validate_file_size(&self, size: usize) -> PptResult<()> {
        if size > self.limits.max_file_size {
            return Err(PptError::file_size_limit_exceeded(self.limits.max_file_size));
        }
        Ok(())
    }

    /// Validate font size
    pub fn validate_font_size(&self, size: f64) -> PptResult<()> {
        if size < self.limits.min_font_size || size > self.limits.max_font_size {
            return Err(PptError::invalid_font_size(size)
                .with_context(format!("valid range: {}-{}", self.limits.min_font_size, self.limits.max_font_size)));
        }
        Ok(())
    }

    /// Validate coordinate (must be non-negative)
    pub fn validate_coordinate(&self, value: f64, name: &str) -> PptResult<()> {
        if value < 0.0 {
            return Err(PptError::invalid_dimension(name, value)
                .with_context("coordinate must be non-negative"));
        }
        if !value.is_finite() {
            return Err(PptError::invalid_dimension(name, value)
                .with_context("coordinate must be finite"));
        }
        Ok(())
    }

    /// Validate dimension (must be positive)
    pub fn validate_dimension(&self, value: f64, name: &str) -> PptResult<()> {
        if value <= 0.0 {
            return Err(PptError::invalid_dimension(name, value)
                .with_context("dimension must be positive"));
        }
        if !value.is_finite() {
            return Err(PptError::invalid_dimension(name, value)
                .with_context("dimension must be finite"));
        }
        Ok(())
    }

    /// Validate RGB color values (0-255)
    pub fn validate_rgb_color(&self, _r: u8, _g: u8, _b: u8) -> PptResult<()> {
        // u8 is already 0-255, so this is always valid
        // But we keep the validation for future extensibility
        Ok(())
    }

    /// Validate hex color string
    pub fn validate_hex_color(&self, hex: &str) -> PptResult<()> {
        if !hex.starts_with('#') {
            return Err(PptError::invalid_color(hex).with_context("must start with #"));
        }
        if hex.len() != 7 {
            return Err(PptError::invalid_color(hex).with_context("must be 7 characters (#RRGGBB)"));
        }
        
        for c in hex[1..].chars() {
            if !c.is_ascii_hexdigit() {
                return Err(PptError::invalid_color(hex).with_context("contains invalid hex digit"));
            }
        }
        
        Ok(())
    }

    /// Validate text length
    pub fn validate_text_length(&self, text: &str) -> PptResult<()> {
        if text.len() > self.limits.max_text_length {
            return Err(PptError::invalid_input(format!("text too long: {} characters (limit: {})", 
                text.len(), self.limits.max_text_length)));
        }
        Ok(())
    }

    /// Validate animation count per slide
    pub fn validate_animation_count(&self, count: usize) -> PptResult<()> {
        if count > self.limits.max_animations_per_slide {
            return Err(PptError::invalid_input(format!("too many animations: {} (limit: {})", 
                count, self.limits.max_animations_per_slide)));
        }
        Ok(())
    }

    /// Validate percentage (0-100)
    pub fn validate_percentage(&self, value: f64, name: &str) -> PptResult<()> {
        if value < 0.0 || value > 100.0 {
            return Err(PptError::invalid_dimension(name, value)
                .with_context("percentage must be between 0 and 100"));
        }
        Ok(())
    }

    /// Validate non-empty string
    pub fn validate_non_empty(&self, value: &str, name: &str) -> PptResult<()> {
        if value.trim().is_empty() {
            return Err(PptError::invalid_input(format!("{} cannot be empty", name)));
        }
        Ok(())
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_limits_default() {
        let limits = ResourceLimits::default();
        assert_eq!(limits.max_image_size, 10 * 1024 * 1024);
        assert_eq!(limits.max_slides, 1000);
        assert_eq!(limits.max_table_rows, 100);
        assert_eq!(limits.max_table_columns, 100);
    }

    #[test]
    fn test_resource_limits_custom() {
        let limits = ResourceLimits::new()
            .with_max_image_size(5 * 1024 * 1024)
            .with_max_slides(500);
        assert_eq!(limits.max_image_size, 5 * 1024 * 1024);
        assert_eq!(limits.max_slides, 500);
    }

    #[test]
    fn test_validator_validate_image_size() {
        let validator = Validator::new();
        assert!(validator.validate_image_size(1024).is_ok());
        assert!(validator.validate_image_size(11 * 1024 * 1024).is_err());
    }

    #[test]
    fn test_validator_validate_slide_count() {
        let validator = Validator::new();
        assert!(validator.validate_slide_count(500).is_ok());
        assert!(validator.validate_slide_count(1001).is_err());
    }

    #[test]
    fn test_validator_validate_table_dimensions() {
        let validator = Validator::new();
        assert!(validator.validate_table_dimensions(10, 10).is_ok());
        assert!(validator.validate_table_dimensions(0, 10).is_err());
        assert!(validator.validate_table_dimensions(10, 0).is_err());
        assert!(validator.validate_table_dimensions(101, 10).is_err());
        assert!(validator.validate_table_dimensions(10, 101).is_err());
    }

    #[test]
    fn test_validator_validate_font_size() {
        let validator = Validator::new();
        assert!(validator.validate_font_size(12.0).is_ok());
        assert!(validator.validate_font_size(5.0).is_err());
        assert!(validator.validate_font_size(80.0).is_err());
    }

    #[test]
    fn test_validator_validate_coordinate() {
        let validator = Validator::new();
        assert!(validator.validate_coordinate(10.0, "x").is_ok());
        assert!(validator.validate_coordinate(-1.0, "x").is_err());
        assert!(validator.validate_coordinate(f64::NAN, "x").is_err());
    }

    #[test]
    fn test_validator_validate_dimension() {
        let validator = Validator::new();
        assert!(validator.validate_dimension(10.0, "width").is_ok());
        assert!(validator.validate_dimension(0.0, "width").is_err());
        assert!(validator.validate_dimension(-1.0, "width").is_err());
    }

    #[test]
    fn test_validator_validate_hex_color() {
        let validator = Validator::new();
        assert!(validator.validate_hex_color("#FF0000").is_ok());
        assert!(validator.validate_hex_color("FF0000").is_err());
        assert!(validator.validate_hex_color("#FF00").is_err());
        assert!(validator.validate_hex_color("#GG0000").is_err());
    }

    #[test]
    fn test_validator_validate_text_length() {
        let validator = Validator::new();
        assert!(validator.validate_text_length("hello").is_ok());
        let long_text = "a".repeat(10001);
        assert!(validator.validate_text_length(&long_text).is_err());
    }

    #[test]
    fn test_validator_validate_percentage() {
        let validator = Validator::new();
        assert!(validator.validate_percentage(50.0, "opacity").is_ok());
        assert!(validator.validate_percentage(-1.0, "opacity").is_err());
        assert!(validator.validate_percentage(101.0, "opacity").is_err());
    }

    #[test]
    fn test_validator_validate_non_empty() {
        let validator = Validator::new();
        assert!(validator.validate_non_empty("hello", "name").is_ok());
        assert!(validator.validate_non_empty("", "name").is_err());
        assert!(validator.validate_non_empty("   ", "name").is_err());
    }
}
