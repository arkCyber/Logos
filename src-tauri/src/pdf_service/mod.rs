//! PDF 服务模块
//! 提供完整的 PDF 生成、配置、安全性、书签、压缩、水印等功能

pub mod bookmarks;
pub mod compression;
pub mod config;
pub mod forms;
pub mod generator;
pub mod merge;
pub mod metadata;
pub mod security;
pub mod watermark;

pub use bookmarks::PdfOutline;
pub use compression::PdfCompression;
pub use config::PdfConfig;
pub use generator::PdfGenerator;
pub use metadata::PdfMetadata;
pub use security::PdfSecurity;
pub use watermark::PdfWatermark;
