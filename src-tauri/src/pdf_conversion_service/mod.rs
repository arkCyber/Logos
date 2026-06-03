//! PDF Conversion Service - Aerospace-Grade PDF to Office Format Conversion
//!
//! Safety-critical PDF conversion service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Comprehensive error handling
//! - Performance monitoring
//! - Security hardening
//! - Fault tolerance and error recovery
//!
//! Supports conversion from PDF to:
//! - DOCX (Word documents)
//! - PPTX (PowerPoint presentations)
//! - XLSX (Excel spreadsheets)

pub mod converter;
pub mod error;

pub use converter::PdfConverter;
