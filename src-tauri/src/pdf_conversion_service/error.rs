//! PDF Conversion Error Handling - Aerospace-Grade Error Management

use std::fmt;

/// PDF conversion error types
#[derive(Debug, Clone)]
pub enum PdfConversionError {
    /// Invalid PDF data
    InvalidPdfData(String),
    /// PDF parsing failed
    PdfParseFailed(String),
    /// Conversion to target format failed
    ConversionFailed(String),
    /// Target format not supported
    UnsupportedFormat(String),
    /// File size exceeds maximum limit
    FileSizeExceeded { max_size: usize, actual_size: usize },
    /// Memory allocation failed
    MemoryAllocationFailed(String),
    /// Timeout during conversion
    ConversionTimeout,
    /// Invalid input parameters
    InvalidParameters(String),
    /// External conversion tool not available
    ToolNotAvailable(String),
    /// Conversion process crashed
    ProcessCrashed(String),
}

impl fmt::Display for PdfConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PdfConversionError::InvalidPdfData(msg) => write!(f, "Invalid PDF data: {}", msg),
            PdfConversionError::PdfParseFailed(msg) => write!(f, "PDF parsing failed: {}", msg),
            PdfConversionError::ConversionFailed(msg) => write!(f, "Conversion failed: {}", msg),
            PdfConversionError::UnsupportedFormat(format) => write!(f, "Unsupported format: {}", format),
            PdfConversionError::FileSizeExceeded { max_size, actual_size } => {
                write!(f, "File size {} exceeds maximum {}", actual_size, max_size)
            }
            PdfConversionError::MemoryAllocationFailed(msg) => {
                write!(f, "Memory allocation failed: {}", msg)
            }
            PdfConversionError::ConversionTimeout => write!(f, "Conversion timeout"),
            PdfConversionError::InvalidParameters(msg) => write!(f, "Invalid parameters: {}", msg),
            PdfConversionError::ToolNotAvailable(tool) => write!(f, "Tool not available: {}", tool),
            PdfConversionError::ProcessCrashed(msg) => write!(f, "Process crashed: {}", msg),
        }
    }
}

impl std::error::Error for PdfConversionError {}

impl From<PdfConversionError> for String {
    fn from(error: PdfConversionError) -> Self {
        error.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = PdfConversionError::InvalidPdfData("test".to_string());
        assert_eq!(error.to_string(), "Invalid PDF data: test");
    }

    #[test]
    fn test_file_size_exceeded() {
        let error = PdfConversionError::FileSizeExceeded {
            max_size: 100,
            actual_size: 200,
        };
        assert!(error.to_string().contains("200"));
        assert!(error.to_string().contains("100"));
    }

    #[test]
    fn test_error_to_string() {
        let error = PdfConversionError::ConversionFailed("test error".to_string());
        let s: String = error.into();
        assert_eq!(s, "Conversion failed: test error");
    }
}
