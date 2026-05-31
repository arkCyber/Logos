//! DOCX 服务模块
//! 提供完整的 Word/DOCX 文档生成、样式、表格、图片等功能

pub mod config;
pub mod export;
pub mod header_footer;
pub mod image;
pub mod paragraph;
pub mod style;
pub mod table;

pub use config::DocxConfig;
pub use export::DocxExporter;
pub use header_footer::{Footer, Header};
pub use image::Image;
pub use paragraph::Paragraph;
pub use style::{ParagraphStyle, TextStyle};
pub use table::Table;
