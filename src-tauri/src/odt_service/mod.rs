//! ODT 服务模块
//! 提供完整的 OpenDocument Text (ODT) 文档生成、样式、格式化等功能

pub mod config;
pub mod export;
pub mod paragraph;
pub mod style;

pub use config::OdtConfig;
pub use export::OdtExporter;
pub use paragraph::OdtParagraph;
pub use style::{OdtParagraphStyle, OdtTextStyle};
