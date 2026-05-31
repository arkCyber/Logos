//! Markdown 服务模块
//! 提供 HTML 到 Markdown 的转换功能

pub mod config;
pub mod converter;

pub use config::MarkdownConfig;
pub use converter::MarkdownConverter;
