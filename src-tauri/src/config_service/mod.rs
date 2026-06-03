//! Configuration Service - Aerospace-Grade Configuration Management
//!
//! Safety-critical configuration service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Comprehensive error handling
//! - Performance monitoring
//! - Security hardening
//! - Fault tolerance and error recovery

pub mod export_config;
pub mod error;

pub use export_config::ExportConfigService;
