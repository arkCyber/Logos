use serde::{Deserialize, Serialize};

/// PDF 页面大小
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PageSize {
    /// A4 (210mm x 297mm)
    A4,
    /// A3 (297mm x 420mm)
    A3,
    /// A5 (148mm x 210mm)
    A5,
    /// Letter (8.5in x 11in)
    Letter,
    /// Legal (8.5in x 14in)
    Legal,
    /// Tabloid (11in x 17in)
    Tabloid,
    /// 自定义尺寸（宽度 x 高度，单位：点）
    Custom { width: f64, height: f64 },
}

impl PageSize {
    /// 获取页面尺寸（单位：点）
    #[allow(dead_code)]
    pub fn to_points(&self) -> (f64, f64) {
        match self {
            PageSize::A4 => (595.28, 841.89),
            PageSize::A3 => (841.89, 1190.55),
            PageSize::A5 => (420.94, 595.28),
            PageSize::Letter => (612.0, 792.0),
            PageSize::Legal => (612.0, 1008.0),
            PageSize::Tabloid => (792.0, 1224.0),
            PageSize::Custom { width, height } => (*width, *height),
        }
    }

    /// 获取页面尺寸（单位：毫米）
    #[allow(dead_code)]
    pub fn to_mm(&self) -> (f64, f64) {
        let (width_pt, height_pt) = self.to_points();
        (width_pt * 0.3528, height_pt * 0.3528)
    }

    /// 获取页面尺寸（单位：英寸）
    #[allow(dead_code)]
    pub fn to_inches(&self) -> (f64, f64) {
        let (width_pt, height_pt) = self.to_points();
        (width_pt / 72.0, height_pt / 72.0)
    }
}

/// PDF 页面方向
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PageOrientation {
    /// 纵向
    Portrait,
    /// 横向
    Landscape,
}

impl PageOrientation {
    /// 切换方向
    #[allow(dead_code)]
    pub fn toggle(&self) -> Self {
        match self {
            PageOrientation::Portrait => PageOrientation::Landscape,
            PageOrientation::Landscape => PageOrientation::Portrait,
        }
    }
}

/// PDF 页面边距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMargins {
    /// 上边距（点）
    pub top: f64,
    /// 下边距（点）
    pub bottom: f64,
    /// 左边距（点）
    pub left: f64,
    /// 右边距（点）
    pub right: f64,
}

impl PageMargins {
    /// 创建新的边距
    #[allow(dead_code)]
    pub fn new(top: f64, bottom: f64, left: f64, right: f64) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    /// 创建均匀边距
    pub fn uniform(margin: f64) -> Self {
        Self {
            top: margin,
            bottom: margin,
            left: margin,
            right: margin,
        }
    }

    /// 创建标准 A4 边距（25mm）
    pub fn a4_standard() -> Self {
        Self::uniform(70.87) // 25mm in points
    }

    /// 创建窄边距（12.7mm）
    #[allow(dead_code)]
    pub fn narrow() -> Self {
        Self::uniform(36.0) // 12.7mm in points
    }

    /// 创建宽边距（38mm）
    #[allow(dead_code)]
    pub fn wide() -> Self {
        Self::uniform(107.72) // 38mm in points
    }

    /// 获取水平边距总和
    #[allow(dead_code)]
    pub fn horizontal(&self) -> f64 {
        self.left + self.right
    }

    /// 获取垂直边距总和
    #[allow(dead_code)]
    pub fn vertical(&self) -> f64 {
        self.top + self.bottom
    }
}

impl Default for PageMargins {
    fn default() -> Self {
        Self::a4_standard()
    }
}

/// PDF 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfConfig {
    /// 页面大小
    pub page_size: PageSize,
    /// 页面方向
    pub orientation: PageOrientation,
    /// 页面边距
    pub margins: PageMargins,
    /// 是否包含页码
    pub include_page_numbers: bool,
    /// 页码起始位置
    pub page_number_start: usize,
    /// 是否包含目录
    pub include_toc: bool,
    /// 是否嵌入字体
    pub embed_fonts: bool,
    /// 是否压缩图像
    pub compress_images: bool,
    /// 是否启用超链接
    pub enable_hyperlinks: bool,
    /// 是否启用书签
    pub enable_bookmarks: bool,
    /// PDF 版本
    pub version: String,
}

impl PdfConfig {
    /// 创建新的 PDF 配置
    pub fn new() -> Self {
        Self {
            page_size: PageSize::A4,
            orientation: PageOrientation::Portrait,
            margins: PageMargins::default(),
            include_page_numbers: true,
            page_number_start: 1,
            include_toc: false,
            embed_fonts: true,
            compress_images: true,
            enable_hyperlinks: true,
            enable_bookmarks: true,
            version: "1.7".to_string(),
        }
    }

    /// 设置页面大小
    #[allow(dead_code)]
    pub fn with_page_size(mut self, size: PageSize) -> Self {
        self.page_size = size;
        self
    }

    /// 设置页面方向
    #[allow(dead_code)]
    pub fn with_orientation(mut self, orientation: PageOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// 设置边距
    #[allow(dead_code)]
    pub fn with_margins(mut self, margins: PageMargins) -> Self {
        self.margins = margins;
        self
    }

    /// 设置是否包含页码
    #[allow(dead_code)]
    pub fn with_page_numbers(mut self, include: bool) -> Self {
        self.include_page_numbers = include;
        self
    }

    /// 设置页码起始位置
    #[allow(dead_code)]
    pub fn with_page_number_start(mut self, start: usize) -> Self {
        self.page_number_start = start;
        self
    }

    /// 设置是否包含目录
    #[allow(dead_code)]
    pub fn with_toc(mut self, include: bool) -> Self {
        self.include_toc = include;
        self
    }

    /// 设置是否嵌入字体
    #[allow(dead_code)]
    pub fn with_embed_fonts(mut self, embed: bool) -> Self {
        self.embed_fonts = embed;
        self
    }

    /// 设置是否压缩图像
    #[allow(dead_code)]
    pub fn with_compress_images(mut self, compress: bool) -> Self {
        self.compress_images = compress;
        self
    }

    /// 设置是否启用超链接
    #[allow(dead_code)]
    pub fn with_hyperlinks(mut self, enable: bool) -> Self {
        self.enable_hyperlinks = enable;
        self
    }

    /// 设置是否启用书签
    #[allow(dead_code)]
    pub fn with_bookmarks(mut self, enable: bool) -> Self {
        self.enable_bookmarks = enable;
        self
    }

    /// 设置 PDF 版本
    #[allow(dead_code)]
    pub fn with_version(mut self, version: String) -> Self {
        self.version = version;
        self
    }

    /// 获取实际页面尺寸（考虑方向）
    #[allow(dead_code)]
    pub fn get_actual_page_size(&self) -> (f64, f64) {
        let (width, height) = self.page_size.to_points();
        match self.orientation {
            PageOrientation::Portrait => (width, height),
            PageOrientation::Landscape => (height, width),
        }
    }

    /// 获取可打印区域尺寸
    #[allow(dead_code)]
    pub fn get_printable_area(&self) -> (f64, f64) {
        let (width, height) = self.get_actual_page_size();
        let printable_width = width - self.margins.horizontal();
        let printable_height = height - self.margins.vertical();
        (printable_width, printable_height)
    }
}

impl Default for PdfConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_size_a4() {
        let size = PageSize::A4;
        let (width, height) = size.to_points();
        assert!((width - 595.28).abs() < 0.01);
        assert!((height - 841.89).abs() < 0.01);
    }

    #[test]
    fn test_page_size_letter() {
        let size = PageSize::Letter;
        let (width, height) = size.to_points();
        assert_eq!(width, 612.0);
        assert_eq!(height, 792.0);
    }

    #[test]
    fn test_page_size_custom() {
        let size = PageSize::Custom {
            width: 100.0,
            height: 200.0,
        };
        let (width, height) = size.to_points();
        assert_eq!(width, 100.0);
        assert_eq!(height, 200.0);
    }

    #[test]
    fn test_page_size_to_mm() {
        let size = PageSize::A4;
        let (width, height) = size.to_mm();
        assert!((width - 210.0).abs() < 1.0);
        assert!((height - 297.0).abs() < 1.0);
    }

    #[test]
    fn test_page_orientation_toggle() {
        let orientation = PageOrientation::Portrait;
        assert_eq!(orientation.toggle(), PageOrientation::Landscape);
        assert_eq!(orientation.toggle().toggle(), PageOrientation::Portrait);
    }

    #[test]
    fn test_page_margins_new() {
        let margins = PageMargins::new(10.0, 20.0, 30.0, 40.0);
        assert_eq!(margins.top, 10.0);
        assert_eq!(margins.bottom, 20.0);
        assert_eq!(margins.left, 30.0);
        assert_eq!(margins.right, 40.0);
    }

    #[test]
    fn test_page_margins_uniform() {
        let margins = PageMargins::uniform(50.0);
        assert_eq!(margins.top, 50.0);
        assert_eq!(margins.bottom, 50.0);
        assert_eq!(margins.left, 50.0);
        assert_eq!(margins.right, 50.0);
    }

    #[test]
    fn test_page_margins_horizontal() {
        let margins = PageMargins::new(10.0, 20.0, 30.0, 40.0);
        assert_eq!(margins.horizontal(), 70.0);
    }

    #[test]
    fn test_page_margins_vertical() {
        let margins = PageMargins::new(10.0, 20.0, 30.0, 40.0);
        assert_eq!(margins.vertical(), 30.0);
    }

    #[test]
    fn test_pdf_config_new() {
        let config = PdfConfig::new();
        assert_eq!(config.page_size, PageSize::A4);
        assert_eq!(config.orientation, PageOrientation::Portrait);
        assert!(config.include_page_numbers);
        assert!(config.embed_fonts);
    }

    #[test]
    fn test_pdf_config_with_page_size() {
        let config = PdfConfig::new().with_page_size(PageSize::Letter);
        assert_eq!(config.page_size, PageSize::Letter);
    }

    #[test]
    fn test_pdf_config_with_orientation() {
        let config = PdfConfig::new().with_orientation(PageOrientation::Landscape);
        assert_eq!(config.orientation, PageOrientation::Landscape);
    }

    #[test]
    fn test_pdf_config_chaining() {
        let config = PdfConfig::new()
            .with_page_size(PageSize::A3)
            .with_orientation(PageOrientation::Landscape)
            .with_page_numbers(false);
        assert_eq!(config.page_size, PageSize::A3);
        assert_eq!(config.orientation, PageOrientation::Landscape);
        assert!(!config.include_page_numbers);
    }

    #[test]
    fn test_pdf_config_get_actual_page_size() {
        let config = PdfConfig::new();
        let (width, height) = config.get_actual_page_size();
        assert!((width - 595.28).abs() < 0.01);
        assert!((height - 841.89).abs() < 0.01);
    }

    #[test]
    fn test_pdf_config_get_actual_page_size_landscape() {
        let config = PdfConfig::new().with_orientation(PageOrientation::Landscape);
        let (width, height) = config.get_actual_page_size();
        assert!((width - 841.89).abs() < 0.01);
        assert!((height - 595.28).abs() < 0.01);
    }

    #[test]
    fn test_pdf_config_get_printable_area() {
        let config = PdfConfig::new();
        let (width, height) = config.get_printable_area();
        assert!(width > 0.0);
        assert!(height > 0.0);
        assert!(width < config.get_actual_page_size().0);
        assert!(height < config.get_actual_page_size().1);
    }

    #[test]
    fn test_pdf_config_default() {
        let config = PdfConfig::default();
        assert_eq!(config.page_size, PageSize::A4);
        assert_eq!(config.orientation, PageOrientation::Portrait);
    }

    #[test]
    fn test_pdf_config_serialization() {
        let config = PdfConfig::new();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_pdf_config_deserialization() {
        let config = PdfConfig::new();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: PdfConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.page_size, config.page_size);
        assert_eq!(deserialized.orientation, config.orientation);
    }
}
