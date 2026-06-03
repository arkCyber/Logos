//! Typst Conversion Service
//! 提供HTML到Typst的高性能转换功能
//! 将前端计算密集型的Typst转换迁移到Rust后台

pub mod converter;
pub mod slide_converter;
pub mod typst_to_html;

pub use converter::HtmlToTypstConverter;
pub use slide_converter::HtmlToTypstSlideConverter;
pub use typst_to_html::TypstToHtmlConverter;

use serde::{Deserialize, Serialize};

/// Typst转换配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypstConversionConfig {
    /// 页面设置
    pub page: PageConfig,
    /// 字体设置
    pub font: FontConfig,
    /// 表格样式
    pub table: TableConfig,
}

impl Default for TypstConversionConfig {
    fn default() -> Self {
        Self {
            page: PageConfig::default(),
            font: FontConfig::default(),
            table: TableConfig::default(),
        }
    }
}

/// 页面配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageConfig {
    /// 纸张大小
    pub paper: String,
    /// 边距（厘米）
    pub margin: MarginConfig,
}

impl Default for PageConfig {
    fn default() -> Self {
        Self {
            paper: "a4".to_string(),
            margin: MarginConfig::default(),
        }
    }
}

/// 边距配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginConfig {
    pub x: f64,
    pub y: f64,
}

impl Default for MarginConfig {
    fn default() -> Self {
        Self { x: 2.0, y: 2.5 }
    }
}

/// 字体配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    /// 字体名称
    pub family: String,
    /// 字体大小（点）
    pub size: u32,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            family: "SimSun".to_string(),
            size: 11,
        }
    }
}

/// 表格配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableConfig {
    /// 是否使用三线表样式
    pub use_three_line: bool,
    /// 表头背景色
    pub header_bg: String,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            use_three_line: true,
            header_bg: "e0e7ff".to_string(),
        }
    }
}

/// 幻灯片配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlideConfig {
    /// 主题
    pub theme: String,
    /// 宽高比
    pub aspect_ratio: String,
    /// 幻灯片尺寸
    pub size: SlideSize,
}

impl Default for SlideConfig {
    fn default() -> Self {
        Self {
            theme: "metropolis".to_string(),
            aspect_ratio: "16-9".to_string(),
            size: SlideSize::default(),
        }
    }
}

/// 幻灯片尺寸
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlideSize {
    pub width: f64,
    pub height: f64,
}

impl Default for SlideSize {
    fn default() -> Self {
        Self {
            width: 1920.0,
            height: 1080.0,
        }
    }
}

/// 转换结果
#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionResult {
    /// Typst代码
    pub typst_code: String,
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error: Option<String>,
    /// 转换耗时（毫秒）
    pub duration_ms: u64,
}
