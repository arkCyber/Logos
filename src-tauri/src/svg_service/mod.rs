//! SVG 服务模块
//! 提供完整的 SVG 矢量图形生成、样式、格式化等功能

pub mod config;
pub mod element;
pub mod export;
pub mod style;

pub use config::{SvgConfig, SvgVersion};
pub use element::{SvgElement, SvgElementType};
pub use export::SvgExporter;
pub use style::SvgStyle;
