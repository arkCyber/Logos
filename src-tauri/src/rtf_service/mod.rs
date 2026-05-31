//! RTF 服务模块
//! 提供完整的 Rich Text Format (RTF) 文档生成、样式、格式化等功能

pub mod config;
pub mod export;
pub mod paragraph;
pub mod style;

pub use config::RtfConfig;
pub use export::RtfExporter;
pub use paragraph::RtfParagraph;
pub use style::{RtfParagraphStyle, RtfTextStyle};
