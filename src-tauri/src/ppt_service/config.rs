use serde::{Deserialize, Serialize};

/// 幻灯片尺寸
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SlideSize {
    /// 标准 4:3 (10 x 7.5 英寸)
    Standard4_3,
    /// 宽屏 16:9 (10 x 5.625 英寸)
    Widescreen16_9,
    /// 宽屏 16:10 (10 x 6.25 英寸)
    Widescreen16_10,
    /// A4 纸张 (11.69 x 8.27 英寸)
    A4,
    /// 自定义尺寸（宽度 x 高度，单位：英寸）
    Custom { width: f64, height: f64 },
}

impl SlideSize {
    /// 获取尺寸（英寸）
    pub fn to_inches(&self) -> (f64, f64) {
        match self {
            SlideSize::Standard4_3 => (10.0, 7.5),
            SlideSize::Widescreen16_9 => (10.0, 5.625),
            SlideSize::Widescreen16_10 => (10.0, 6.25),
            SlideSize::A4 => (11.69, 8.27),
            SlideSize::Custom { width, height } => (*width, *height),
        }
    }

    /// 获取尺寸（点，1英寸 = 72点）
    pub fn to_points(&self) -> (f64, f64) {
        let (width, height) = self.to_inches();
        (width * 72.0, height * 72.0)
    }

    /// 获取尺寸（像素，96 DPI）
    pub fn to_pixels(&self) -> (u32, u32) {
        let (width, height) = self.to_inches();
        ((width * 96.0) as u32, (height * 96.0) as u32)
    }

    /// 获取宽高比
    pub fn aspect_ratio(&self) -> f64 {
        let (width, height) = self.to_inches();
        width / height
    }
}

/// 幻灯片方向
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SlideOrientation {
    /// 横向
    Landscape,
    /// 纵向
    Portrait,
}

/// 幻灯片布局
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SlideLayout {
    /// 空白
    Blank,
    /// 标题
    Title,
    /// 标题和内容
    TitleAndContent,
    /// 两栏内容
    TwoContent,
    /// 两栏对比
    Comparison,
    /// 标题和两栏内容
    TitleAndTwoContent,
    /// 仅标题
    TitleOnly,
    /// 节标题
    SectionHeader,
    /// 内容和标题
    ContentWithCaption,
    /// 图片和标题
    PictureWithCaption,
    /// 垂直标题和文本
    VerticalTitleAndText,
    /// 自定义
    Custom(String),
}

/// PPT 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PptConfig {
    /// 幻灯片尺寸
    pub slide_size: SlideSize,
    /// 幻灯片方向
    pub orientation: SlideOrientation,
    /// 默认布局
    pub default_layout: SlideLayout,
    /// 是否显示页码
    pub show_page_numbers: bool,
    /// 是否显示日期
    pub show_date: bool,
    /// 是否显示页脚
    pub show_footer: bool,
    /// 页脚文本
    pub footer_text: String,
    /// 幻灯片编号起始
    pub slide_number_start: usize,
}

impl PptConfig {
    /// 创建新的 PPT 配置
    pub fn new() -> Self {
        Self {
            slide_size: SlideSize::Widescreen16_9,
            orientation: SlideOrientation::Landscape,
            default_layout: SlideLayout::TitleAndContent,
            show_page_numbers: true,
            show_date: true,
            show_footer: false,
            footer_text: String::new(),
            slide_number_start: 1,
        }
    }

    /// 设置幻灯片尺寸
    pub fn with_slide_size(mut self, size: SlideSize) -> Self {
        self.slide_size = size;
        self
    }

    /// 设置方向
    pub fn with_orientation(mut self, orientation: SlideOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// 设置默认布局
    pub fn with_default_layout(mut self, layout: SlideLayout) -> Self {
        self.default_layout = layout;
        self
    }

    /// 设置是否显示页码
    pub fn with_page_numbers(mut self, show: bool) -> Self {
        self.show_page_numbers = show;
        self
    }

    /// 设置是否显示日期
    pub fn with_date(mut self, show: bool) -> Self {
        self.show_date = show;
        self
    }

    /// 设置是否显示页脚
    pub fn with_footer(mut self, show: bool) -> Self {
        self.show_footer = show;
        self
    }

    /// 设置页脚文本
    pub fn with_footer_text(mut self, text: String) -> Self {
        self.footer_text = text;
        self.show_footer = true;
        self
    }

    /// 设置幻灯片编号起始
    pub fn with_slide_number_start(mut self, start: usize) -> Self {
        self.slide_number_start = start;
        self
    }

    /// 获取实际幻灯片尺寸（考虑方向）
    pub fn get_actual_size(&self) -> (f64, f64) {
        let (width, height) = self.slide_size.to_inches();
        match self.orientation {
            SlideOrientation::Landscape => (width, height),
            SlideOrientation::Portrait => (height, width),
        }
    }
}

impl Default for PptConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_size_standard_4_3() {
        let size = SlideSize::Standard4_3;
        let (width, height) = size.to_inches();
        assert_eq!(width, 10.0);
        assert_eq!(height, 7.5);
    }

    #[test]
    fn test_slide_size_widescreen_16_9() {
        let size = SlideSize::Widescreen16_9;
        let (width, height) = size.to_inches();
        assert_eq!(width, 10.0);
        assert!((height - 5.625).abs() < 0.01);
    }

    #[test]
    fn test_slide_size_to_points() {
        let size = SlideSize::Standard4_3;
        let (width, height) = size.to_points();
        assert_eq!(width, 720.0);
        assert_eq!(height, 540.0);
    }

    #[test]
    fn test_slide_size_aspect_ratio() {
        let size = SlideSize::Widescreen16_9;
        let ratio = size.aspect_ratio();
        assert!((ratio - 1.777).abs() < 0.01);
    }

    #[test]
    fn test_ppt_config_new() {
        let config = PptConfig::new();
        assert_eq!(config.slide_size, SlideSize::Widescreen16_9);
        assert_eq!(config.orientation, SlideOrientation::Landscape);
        assert!(config.show_page_numbers);
    }

    #[test]
    fn test_ppt_config_with_slide_size() {
        let config = PptConfig::new().with_slide_size(SlideSize::A4);
        assert_eq!(config.slide_size, SlideSize::A4);
    }

    #[test]
    fn test_ppt_config_with_orientation() {
        let config = PptConfig::new().with_orientation(SlideOrientation::Portrait);
        assert_eq!(config.orientation, SlideOrientation::Portrait);
    }

    #[test]
    fn test_ppt_config_chaining() {
        let config = PptConfig::new()
            .with_slide_size(SlideSize::Standard4_3)
            .with_page_numbers(false)
            .with_footer_text("Confidential".to_string());
        assert_eq!(config.slide_size, SlideSize::Standard4_3);
        assert!(!config.show_page_numbers);
        assert_eq!(config.footer_text, "Confidential");
    }

    #[test]
    fn test_ppt_config_get_actual_size() {
        let config = PptConfig::new();
        let (width, height) = config.get_actual_size();
        assert!(width > height);
    }

    #[test]
    fn test_ppt_config_get_actual_size_portrait() {
        let config = PptConfig::new().with_orientation(SlideOrientation::Portrait);
        let (width, height) = config.get_actual_size();
        assert!(height > width);
    }

    #[test]
    fn test_ppt_config_default() {
        let config = PptConfig::default();
        assert_eq!(config.slide_size, SlideSize::Widescreen16_9);
    }

    #[test]
    fn test_ppt_config_serialization() {
        let config = PptConfig::new();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }
}
