use serde::{Deserialize, Serialize};

/// 水印位置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WatermarkPosition {
    /// 居中
    Center,
    /// 左上角
    TopLeft,
    /// 右上角
    TopRight,
    /// 左下角
    BottomLeft,
    /// 右下角
    BottomRight,
    /// 平铺（重复覆盖整个页面）
    Tiled,
    /// 自定义位置（X, Y 坐标，单位：点）
    Custom { x: f64, y: f64 },
}

/// 水印不透明度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatermarkOpacity {
    /// 不透明度值（0.0 - 1.0）
    pub value: f64,
}

impl WatermarkOpacity {
    /// 创建新的不透明度
    #[allow(dead_code)]
    pub fn new(value: f64) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
        }
    }

    /// 创建半透明水印
    #[allow(dead_code)]
    pub fn semi_transparent() -> Self {
        Self::new(0.3)
    }

    /// 创建较淡的水印
    #[allow(dead_code)]
    pub fn light() -> Self {
        Self::new(0.15)
    }

    /// 创建较深的水印
    #[allow(dead_code)]
    pub fn medium() -> Self {
        Self::new(0.5)
    }
}

/// PDF 水印
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfWatermark {
    /// 水印文本
    pub text: String,
    /// 水印位置
    pub position: WatermarkPosition,
    /// 不透明度
    pub opacity: WatermarkOpacity,
    /// 字体大小（点）
    pub font_size: f64,
    /// 字体颜色（RGB）
    pub color: (u8, u8, u8),
    /// 旋转角度（度）
    pub rotation: f64,
    /// 是否只应用于第一页
    pub first_page_only: bool,
    /// 是否只应用于奇数页
    pub odd_pages_only: bool,
    /// 是否只应用于偶数页
    pub even_pages_only: bool,
}

impl PdfWatermark {
    /// 创建新的文本水印
    #[allow(dead_code)]
    pub fn new(text: String) -> Self {
        Self {
            text,
            position: WatermarkPosition::Center,
            opacity: WatermarkOpacity::light(),
            font_size: 48.0,
            color: (128, 128, 128),
            rotation: 45.0,
            first_page_only: false,
            odd_pages_only: false,
            even_pages_only: false,
        }
    }

    /// 设置位置
    #[allow(dead_code)]
    pub fn with_position(mut self, position: WatermarkPosition) -> Self {
        self.position = position;
        self
    }

    /// 设置不透明度
    #[allow(dead_code)]
    pub fn with_opacity(mut self, opacity: WatermarkOpacity) -> Self {
        self.opacity = opacity;
        self
    }

    /// 设置字体大小
    #[allow(dead_code)]
    pub fn with_font_size(mut self, size: f64) -> Self {
        self.font_size = size;
        self
    }

    /// 设置颜色
    #[allow(dead_code)]
    pub fn with_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = (r, g, b);
        self
    }

    /// 设置旋转角度
    #[allow(dead_code)]
    pub fn with_rotation(mut self, degrees: f64) -> Self {
        self.rotation = degrees;
        self
    }

    /// 设置只应用于第一页
    #[allow(dead_code)]
    pub fn first_page_only(mut self) -> Self {
        self.first_page_only = true;
        self
    }

    /// 设置只应用于奇数页
    #[allow(dead_code)]
    pub fn odd_pages_only(mut self) -> Self {
        self.odd_pages_only = true;
        self
    }

    /// 设置只应用于偶数页
    #[allow(dead_code)]
    pub fn even_pages_only(mut self) -> Self {
        self.even_pages_only = true;
        self
    }

    /// 创建"草稿"水印
    #[allow(dead_code)]
    pub fn draft() -> Self {
        Self::new("DRAFT".to_string())
            .with_opacity(WatermarkOpacity::light())
            .with_rotation(45.0)
    }

    /// 创建"机密"水印
    #[allow(dead_code)]
    pub fn confidential() -> Self {
        Self::new("CONFIDENTIAL".to_string())
            .with_opacity(WatermarkOpacity::medium())
            .with_color(255, 0, 0)
            .with_rotation(45.0)
    }

    /// 创建版权水印
    #[allow(dead_code)]
    pub fn copyright(text: String) -> Self {
        Self::new(text)
            .with_opacity(WatermarkOpacity::semi_transparent())
            .with_position(WatermarkPosition::BottomLeft)
            .with_font_size(12.0)
            .with_rotation(0.0)
    }

    /// 检查水印是否应该应用于指定页码
    #[allow(dead_code)]
    pub fn should_apply_to_page(&self, page_index: usize) -> bool {
        if self.first_page_only && page_index != 0 {
            return false;
        }
        if self.odd_pages_only && page_index % 2 != 0 {
            return false;
        }
        if self.even_pages_only && page_index % 2 != 1 {
            return false;
        }
        true
    }
}

// 为 WatermarkPosition 添加 BottomCenter 变体
impl WatermarkPosition {
    /// 获取位置坐标（相对于页面尺寸）
    #[allow(dead_code)]
    pub fn get_coordinates(&self, page_width: f64, page_height: f64) -> (f64, f64) {
        match self {
            WatermarkPosition::Center => (page_width / 2.0, page_height / 2.0),
            WatermarkPosition::TopLeft => (50.0, page_height - 50.0),
            WatermarkPosition::TopRight => (page_width - 50.0, page_height - 50.0),
            WatermarkPosition::BottomLeft => (50.0, 50.0),
            WatermarkPosition::BottomRight => (page_width - 50.0, 50.0),
            WatermarkPosition::Custom { x, y } => (*x, *y),
            WatermarkPosition::Tiled => (0.0, 0.0), // 平铺模式需要特殊处理
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watermark_opacity_new() {
        let opacity = WatermarkOpacity::new(0.5);
        assert_eq!(opacity.value, 0.5);
    }

    #[test]
    fn test_watermark_opacity_clamp() {
        let opacity = WatermarkOpacity::new(1.5);
        assert_eq!(opacity.value, 1.0);
    }

    #[test]
    fn test_watermark_opacity_semi_transparent() {
        let opacity = WatermarkOpacity::semi_transparent();
        assert_eq!(opacity.value, 0.3);
    }

    #[test]
    fn test_pdf_watermark_new() {
        let watermark = PdfWatermark::new("Test".to_string());
        assert_eq!(watermark.text, "Test");
        assert_eq!(watermark.position, WatermarkPosition::Center);
    }

    #[test]
    fn test_pdf_watermark_with_position() {
        let watermark =
            PdfWatermark::new("Test".to_string()).with_position(WatermarkPosition::TopLeft);
        assert_eq!(watermark.position, WatermarkPosition::TopLeft);
    }

    #[test]
    fn test_pdf_watermark_with_opacity() {
        let watermark =
            PdfWatermark::new("Test".to_string()).with_opacity(WatermarkOpacity::medium());
        assert_eq!(watermark.opacity.value, 0.5);
    }

    #[test]
    fn test_pdf_watermark_with_font_size() {
        let watermark = PdfWatermark::new("Test".to_string()).with_font_size(72.0);
        assert_eq!(watermark.font_size, 72.0);
    }

    #[test]
    fn test_pdf_watermark_with_color() {
        let watermark = PdfWatermark::new("Test".to_string()).with_color(255, 0, 0);
        assert_eq!(watermark.color, (255, 0, 0));
    }

    #[test]
    fn test_pdf_watermark_with_rotation() {
        let watermark = PdfWatermark::new("Test".to_string()).with_rotation(90.0);
        assert_eq!(watermark.rotation, 90.0);
    }

    #[test]
    fn test_pdf_watermark_draft() {
        let watermark = PdfWatermark::draft();
        assert_eq!(watermark.text, "DRAFT");
        assert_eq!(watermark.rotation, 45.0);
    }

    #[test]
    fn test_pdf_watermark_confidential() {
        let watermark = PdfWatermark::confidential();
        assert_eq!(watermark.text, "CONFIDENTIAL");
        assert_eq!(watermark.color, (255, 0, 0));
    }

    #[test]
    fn test_pdf_watermark_copyright() {
        let watermark = PdfWatermark::copyright("© 2024".to_string());
        assert_eq!(watermark.text, "© 2024");
        assert_eq!(watermark.rotation, 0.0);
    }

    #[test]
    fn test_pdf_watermark_should_apply_to_page() {
        let watermark = PdfWatermark::new("Test".to_string());
        assert!(watermark.should_apply_to_page(0));
        assert!(watermark.should_apply_to_page(1));
    }

    #[test]
    fn test_pdf_watermark_first_page_only() {
        let watermark = PdfWatermark::new("Test".to_string()).first_page_only();
        assert!(watermark.should_apply_to_page(0));
        assert!(!watermark.should_apply_to_page(1));
    }

    #[test]
    fn test_pdf_watermark_odd_pages_only() {
        let watermark = PdfWatermark::new("Test".to_string()).odd_pages_only();
        assert!(watermark.should_apply_to_page(0));
        assert!(!watermark.should_apply_to_page(1));
        assert!(watermark.should_apply_to_page(2));
    }

    #[test]
    fn test_pdf_watermark_even_pages_only() {
        let watermark = PdfWatermark::new("Test".to_string()).even_pages_only();
        assert!(!watermark.should_apply_to_page(0));
        assert!(watermark.should_apply_to_page(1));
        assert!(!watermark.should_apply_to_page(2));
    }

    #[test]
    fn test_watermark_position_get_coordinates() {
        let pos = WatermarkPosition::Center;
        let (x, y) = pos.get_coordinates(100.0, 200.0);
        assert_eq!(x, 50.0);
        assert_eq!(y, 100.0);
    }

    #[test]
    fn test_pdf_watermark_chaining() {
        let watermark = PdfWatermark::new("Test".to_string())
            .with_position(WatermarkPosition::TopRight)
            .with_font_size(72.0)
            .with_rotation(30.0);
        assert_eq!(watermark.position, WatermarkPosition::TopRight);
        assert_eq!(watermark.font_size, 72.0);
        assert_eq!(watermark.rotation, 30.0);
    }

    #[test]
    fn test_pdf_watermark_serialization() {
        let watermark = PdfWatermark::new("Test".to_string());
        let json = serde_json::to_string(&watermark);
        assert!(json.is_ok());
    }
}
