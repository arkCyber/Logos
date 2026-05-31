//! EPUB 服务模块
//! 提供完整的 EPUB 电子书生成、元数据、章节、样式等功能

pub mod chapter;
pub mod config;
pub mod export;
pub mod metadata;
pub mod style;

pub use chapter::{EpubChapter, EpubToc};
pub use config::EpubConfig;
pub use export::EpubExporter;
pub use metadata::EpubMetadata;
pub use style::EpubStyle;
