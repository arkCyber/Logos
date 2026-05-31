use serde::{Deserialize, Serialize};

/// 页面尺寸
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
    /// 自定义尺寸（宽度 x 高度，单位：点）
    Custom { width: f64, height: f64 },
}

impl PageSize {
    /// 获取尺寸（点，1英寸 = 72点）
    #[allow(dead_code)]
    pub fn to_points(&self) -> (f64, f64) {
        match self {
            PageSize::A4 => (595.28, 841.89),
            PageSize::A3 => (841.89, 1190.55),
            PageSize::A5 => (419.53, 595.28),
            PageSize::Letter => (612.0, 792.0),
            PageSize::Legal => (612.0, 1008.0),
            PageSize::Custom { width, height } => (*width, *height),
        }
    }
}

/// 页面方向
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PageOrientation {
    /// 横向
    Landscape,
    /// 纵向
    Portrait,
}

/// 页面边距
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
    /// 创建标准边距（1英寸）
    pub fn standard() -> Self {
        Self {
            top: 72.0,
            bottom: 72.0,
            left: 72.0,
            right: 72.0,
        }
    }

    /// 创建窄边距（0.5英寸）
    #[allow(dead_code)]
    pub fn narrow() -> Self {
        Self {
            top: 36.0,
            bottom: 36.0,
            left: 36.0,
            right: 36.0,
        }
    }

    /// 创建宽边距（2英寸）
    #[allow(dead_code)]
    pub fn wide() -> Self {
        Self {
            top: 144.0,
            bottom: 144.0,
            left: 144.0,
            right: 144.0,
        }
    }

    /// 创建自定义边距
    #[allow(dead_code)]
    pub fn custom(top: f64, bottom: f64, left: f64, right: f64) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }
}

impl Default for PageMargins {
    fn default() -> Self {
        Self::standard()
    }
}

/// DOCX 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocxConfig {
    /// 页面尺寸
    pub page_size: PageSize,
    /// 页面方向
    pub orientation: PageOrientation,
    /// 页面边距
    pub margins: PageMargins,
    /// 是否显示页码
    pub show_page_numbers: bool,
    /// 页码起始
    pub page_number_start: usize,
}

impl DocxConfig {
    /// 创建新的 DOCX 配置
    pub fn new() -> Self {
        Self {
            page_size: PageSize::A4,
            orientation: PageOrientation::Portrait,
            margins: PageMargins::standard(),
            show_page_numbers: true,
            page_number_start: 1,
        }
    }

    /// 设置页面尺寸
    #[allow(dead_code)]
    pub fn with_page_size(mut self, size: PageSize) -> Self {
        self.page_size = size;
        self
    }

    /// 设置方向
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

    /// 设置是否显示页码
    #[allow(dead_code)]
    pub fn with_page_numbers(mut self, show: bool) -> Self {
        self.show_page_numbers = show;
        self
    }
}

impl Default for DocxConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_size_a4() {
        let (width, height) = PageSize::A4.to_points();
        assert!((width - 595.28).abs() < 0.1);
        assert!((height - 841.89).abs() < 0.1);
    }

    #[test]
    fn test_page_margins_standard() {
        let margins = PageMargins::standard();
        assert_eq!(margins.top, 72.0);
        assert_eq!(margins.bottom, 72.0);
    }

    #[test]
    fn test_page_margins_narrow() {
        let margins = PageMargins::narrow();
        assert_eq!(margins.top, 36.0);
    }

    #[test]
    fn test_page_margins_custom() {
        let margins = PageMargins::custom(50.0, 60.0, 70.0, 80.0);
        assert_eq!(margins.top, 50.0);
        assert_eq!(margins.right, 80.0);
    }

    #[test]
    fn test_docx_config_new() {
        let config = DocxConfig::new();
        assert_eq!(config.page_size, PageSize::A4);
        assert_eq!(config.orientation, PageOrientation::Portrait);
    }

    #[test]
    fn test_docx_config_with_page_size() {
        let config = DocxConfig::new().with_page_size(PageSize::Letter);
        assert_eq!(config.page_size, PageSize::Letter);
    }

    #[test]
    fn test_docx_config_chaining() {
        let config = DocxConfig::new()
            .with_page_size(PageSize::A4)
            .with_orientation(PageOrientation::Landscape)
            .with_page_numbers(false);
        assert_eq!(config.orientation, PageOrientation::Landscape);
        assert!(!config.show_page_numbers);
    }

    #[test]
    fn test_docx_config_serialization() {
        let config = DocxConfig::new();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }
}
