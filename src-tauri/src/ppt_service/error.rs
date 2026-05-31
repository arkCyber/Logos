//! Aerospace-grade error handling for PPT service
//! Provides detailed, categorized error types with error codes

use serde::{Deserialize, Serialize};
use std::fmt;

/// PPT error codes for programmatic handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PptErrorCode {
    /// Invalid input parameter
    InvalidInput = 1001,
    /// Memory limit exceeded
    MemoryLimitExceeded = 1002,
    /// File size limit exceeded
    FileSizeLimitExceeded = 1003,
    /// Slide count limit exceeded
    SlideCountLimitExceeded = 1004,
    /// Invalid coordinate or dimension
    InvalidDimension = 1005,
    /// Invalid color value
    InvalidColor = 1006,
    /// Invalid font size
    InvalidFontSize = 1007,
    /// Image data too large
    ImageDataTooLarge = 1008,
    /// Table dimensions invalid
    InvalidTableDimensions = 1009,
    /// Generation failed
    GenerationFailed = 2001,
    /// Export failed
    ExportFailed = 2002,
    /// Invalid format
    InvalidFormat = 2003,
    /// Unsupported feature
    UnsupportedFeature = 2004,
    /// IO error
    IoError = 3001,
    /// Parse error
    ParseError = 3002,
    /// Serialization error
    SerializationError = 3003,
    /// Internal error
    InternalError = 5000,
}

impl fmt::Display for PptErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PptErrorCode::InvalidInput => write!(f, "Invalid input parameter"),
            PptErrorCode::MemoryLimitExceeded => write!(f, "Memory limit exceeded"),
            PptErrorCode::FileSizeLimitExceeded => write!(f, "File size limit exceeded"),
            PptErrorCode::SlideCountLimitExceeded => write!(f, "Slide count limit exceeded"),
            PptErrorCode::InvalidDimension => write!(f, "Invalid coordinate or dimension"),
            PptErrorCode::InvalidColor => write!(f, "Invalid color value"),
            PptErrorCode::InvalidFontSize => write!(f, "Invalid font size"),
            PptErrorCode::ImageDataTooLarge => write!(f, "Image data too large"),
            PptErrorCode::InvalidTableDimensions => write!(f, "Invalid table dimensions"),
            PptErrorCode::GenerationFailed => write!(f, "Generation failed"),
            PptErrorCode::ExportFailed => write!(f, "Export failed"),
            PptErrorCode::InvalidFormat => write!(f, "Invalid format"),
            PptErrorCode::UnsupportedFeature => write!(f, "Unsupported feature"),
            PptErrorCode::IoError => write!(f, "IO error"),
            PptErrorCode::ParseError => write!(f, "Parse error"),
            PptErrorCode::SerializationError => write!(f, "Serialization error"),
            PptErrorCode::InternalError => write!(f, "Internal error"),
        }
    }
}

/// Detailed PPT error with context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PptError {
    /// Error code
    pub code: PptErrorCode,
    /// Human-readable error message
    pub message: String,
    /// Additional context (e.g., parameter name, value)
    pub context: Option<String>,
    /// Source location (file, line) for debugging
    pub location: Option<String>,
    /// Inner error cause
    pub cause: Option<String>,
}

impl PptError {
    /// Create a new PPT error
    pub fn new(code: PptErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            context: None,
            location: None,
            cause: None,
        }
    }

    /// Add context to the error
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Add location to the error
    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    /// Add cause to the error
    pub fn with_cause(mut self, cause: impl Into<String>) -> Self {
        self.cause = Some(cause.into());
        self
    }
}

impl fmt::Display for PptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)?;
        if let Some(context) = &self.context {
            write!(f, " (context: {})", context)?;
        }
        if let Some(cause) = &self.cause {
            write!(f, " (cause: {})", cause)?;
        }
        Ok(())
    }
}

impl std::error::Error for PptError {}

/// Result type for PPT operations
pub type PptResult<T> = Result<T, PptError>;

/// Convenience constructors for common errors
impl PptError {
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::new(PptErrorCode::InvalidInput, message)
    }

    pub fn memory_limit_exceeded(limit: usize) -> Self {
        Self::new(PptErrorCode::MemoryLimitExceeded, format!("Memory limit exceeded: {} bytes", limit))
    }

    pub fn file_size_limit_exceeded(limit: usize) -> Self {
        Self::new(PptErrorCode::FileSizeLimitExceeded, format!("File size limit exceeded: {} bytes", limit))
    }

    pub fn slide_count_limit_exceeded(limit: usize) -> Self {
        Self::new(PptErrorCode::SlideCountLimitExceeded, format!("Slide count limit exceeded: {}", limit))
    }

    pub fn invalid_dimension(name: impl Into<String>, value: f64) -> Self {
        Self::new(PptErrorCode::InvalidDimension, format!("Invalid dimension: {} = {}", name.into(), value))
    }

    pub fn invalid_color(value: impl Into<String>) -> Self {
        Self::new(PptErrorCode::InvalidColor, format!("Invalid color value: {}", value.into()))
    }

    pub fn invalid_font_size(size: f64) -> Self {
        Self::new(PptErrorCode::InvalidFontSize, format!("Invalid font size: {}", size))
    }

    pub fn image_data_too_large(size: usize, limit: usize) -> Self {
        Self::new(PptErrorCode::ImageDataTooLarge, format!("Image data too large: {} bytes (limit: {})", size, limit))
    }

    pub fn invalid_table_dimensions(rows: usize, cols: usize) -> Self {
        Self::new(PptErrorCode::InvalidTableDimensions, format!("Invalid table dimensions: {}x{}", rows, cols))
    }

    pub fn generation_failed(message: impl Into<String>) -> Self {
        Self::new(PptErrorCode::GenerationFailed, message)
    }

    pub fn export_failed(message: impl Into<String>) -> Self {
        Self::new(PptErrorCode::ExportFailed, message)
    }

    pub fn invalid_format(format: impl Into<String>) -> Self {
        Self::new(PptErrorCode::InvalidFormat, format!("Invalid format: {}", format.into()))
    }

    pub fn unsupported_feature(feature: impl Into<String>) -> Self {
        Self::new(PptErrorCode::UnsupportedFeature, format!("Unsupported feature: {}", feature.into()))
    }

    pub fn io_error(message: impl Into<String>) -> Self {
        Self::new(PptErrorCode::IoError, message)
    }

    pub fn parse_error(message: impl Into<String>) -> Self {
        Self::new(PptErrorCode::ParseError, message)
    }

    pub fn serialization_error(message: impl Into<String>) -> Self {
        Self::new(PptErrorCode::SerializationError, message)
    }

    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::new(PptErrorCode::InternalError, message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_display() {
        assert_eq!(format!("{}", PptErrorCode::InvalidInput), "Invalid input parameter");
    }

    #[test]
    fn test_ppt_error_display() {
        let error = PptError::invalid_input("test error");
        assert!(format!("{}", error).contains("Invalid input parameter"));
    }

    #[test]
    fn test_ppt_error_with_context() {
        let error = PptError::invalid_input("test error").with_context("parameter: x");
        assert!(format!("{}", error).contains("context: parameter: x"));
    }

    #[test]
    fn test_convenience_constructors() {
        let error = PptError::memory_limit_exceeded(1024);
        assert_eq!(error.code, PptErrorCode::MemoryLimitExceeded);
        assert!(error.message.contains("1024"));
    }

    #[test]
    fn test_error_serialization() {
        let error = PptError::invalid_input("test");
        let json = serde_json::to_string(&error);
        assert!(json.is_ok());
    }
}
