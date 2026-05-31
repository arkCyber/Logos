//! PPT 服务模块
//! 提供完整的 PowerPoint/PPTX 幻灯片生成、配置、样式、动画等功能

pub mod animation;
pub mod artword;
pub mod audio;
pub mod chart;
pub mod config;
pub mod error;
pub mod export;
pub mod hyperlink;
pub mod image;
pub mod playback;
pub mod presenter;
pub mod rehearsal;
pub mod shape;
pub mod slide;
pub mod smartart;
pub mod table;
pub mod text;
pub mod theme;
pub mod transition;
pub mod validation;
pub mod video;

#[cfg(test)]
mod integration_test;

pub use artword::ArtWordElement;
pub use audio::AudioElement;
pub use chart::ChartElement;
pub use config::PptConfig;
pub use export::PptxExporter;
pub use hyperlink::HyperlinkElement;
pub use image::ImageElement;
pub use shape::Shape;
pub use slide::Slide;
pub use smartart::{SmartArtElement, SmartArtNode, SmartArtType};
pub use table::TableElement;
pub use text::TextElement;
pub use theme::PptTheme;
pub use transition::SlideTransition;
pub use video::VideoElement;
