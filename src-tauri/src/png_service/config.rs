use serde::{Deserialize, Serialize};

/// PNG 格式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PngFormat {
    /// 标准 PNG
    Standard,
    /// PNG-8（8位颜色）
    Png8,
    /// PNG-24（24位颜色）
    Png24,
    /// PNG-32（32位颜色，带透明度）
    Png32,
}

/// PNG 颜色空间
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PngColorSpace {
    /// RGB
    Rgb,
    /// RGBA（带透明度）
    Rgba,
    /// 灰度
    Grayscale,
    /// 灰度带透明度
    GrayscaleAlpha,
}

/// PNG 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PngConfig {
    /// PNG 格式
    pub format: PngFormat,
    /// 颜色空间
    pub color_space: PngColorSpace,
    /// 宽度（像素）
    pub width: u32,
    /// 高度（像素）
    pub height: u32,
    /// DPI（每英寸点数）
    pub dpi: u32,
}

impl PngConfig {
    /// 创建新的 PNG 配置
    pub fn new() -> Self {
        Self {
            format: PngFormat::Png32,
            color_space: PngColorSpace::Rgba,
            width: 800,
            height: 600,
            dpi: 96,
        }
    }

    /// 设置格式
    #[allow(dead_code)]
    pub fn with_format(mut self, format: PngFormat) -> Self {
        self.format = format;
        self
    }

    /// 设置颜色空间
    #[allow(dead_code)]
    pub fn with_color_space(mut self, color_space: PngColorSpace) -> Self {
        self.color_space = color_space;
        self
    }

    /// 设置尺寸
    #[allow(dead_code)]
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// 设置 DPI
    #[allow(dead_code)]
    pub fn with_dpi(mut self, dpi: u32) -> Self {
        self.dpi = dpi;
        self
    }
}

impl Default for PngConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_png_config_new() {
        let config = PngConfig::new();
        assert_eq!(config.format, PngFormat::Png32);
        assert_eq!(config.width, 800);
    }

    #[test]
    fn test_png_config_with_size() {
        let config = PngConfig::new().with_size(1024, 768);
        assert_eq!(config.width, 1024);
        assert_eq!(config.height, 768);
    }

    #[test]
    fn test_png_config_serialization() {
        let config = PngConfig::new();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_png_config_deserialization() {
        let json = r#"{"format":"Png32","color_space":"Rgba","width":800,"height":600,"dpi":96}"#;
        let config: PngConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.width, 800);
    }

    #[test]
    fn test_png_config_default() {
        let config = PngConfig::default();
        assert_eq!(config.format, PngFormat::Png32);
        assert_eq!(config.width, 800);
    }

    #[test]
    fn test_png_config_with_format() {
        let config = PngConfig::new().with_format(PngFormat::Png24);
        assert_eq!(config.format, PngFormat::Png24);
    }

    #[test]
    fn test_png_config_with_dpi() {
        let config = PngConfig::new().with_dpi(150);
        assert_eq!(config.dpi, 150);
    }

    #[test]
    fn test_png_config_chaining() {
        let config = PngConfig::new()
            .with_size(1024, 768)
            .with_format(PngFormat::Png24)
            .with_dpi(150);
        assert_eq!(config.width, 1024);
        assert_eq!(config.height, 768);
        assert_eq!(config.format, PngFormat::Png24);
        assert_eq!(config.dpi, 150);
    }

    #[test]
    fn test_png_format_png24() {
        assert_eq!(PngFormat::Png24, PngFormat::Png24);
    }

    #[test]
    fn test_png_format_png32() {
        assert_eq!(PngFormat::Png32, PngFormat::Png32);
    }

    #[test]
    fn test_png_format_png8() {
        assert_eq!(PngFormat::Png8, PngFormat::Png8);
    }

    #[test]
    fn test_png_format_serialization() {
        let format = PngFormat::Png32;
        let json = serde_json::to_string(&format);
        assert!(json.is_ok());
    }

    #[test]
    fn test_png_format_deserialization() {
        let json = r#""Png32""#;
        let format: PngFormat = serde_json::from_str(json).unwrap();
        assert_eq!(format, PngFormat::Png32);
    }

    #[test]
    fn test_png_config_width_bounds() {
        let config = PngConfig::new().with_size(10000, 600);
        assert_eq!(config.width, 10000);
    }

    #[test]
    fn test_png_config_height_bounds() {
        let config = PngConfig::new().with_size(800, 10000);
        assert_eq!(config.height, 10000);
    }

    #[test]
    fn test_png_config_dpi_bounds() {
        let config = PngConfig::new().with_dpi(300);
        assert_eq!(config.dpi, 300);
    }

    #[test]
    fn test_png_config_zero_size() {
        let config = PngConfig::new().with_size(0, 0);
        assert_eq!(config.width, 0);
        assert_eq!(config.height, 0);
    }
}
