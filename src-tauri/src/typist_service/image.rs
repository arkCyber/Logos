/*!
 * 航空航天级图像系统
 * 实现 Typst 的图像功能（插入、缩放、滤镜、格式转换）
 */

use serde::{Deserialize, Serialize};

/// 图像格式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImageFormat {
    Auto,
    Png,
    Jpeg,
    Jpg,
    Svg,
    Pdf,
    Gif,
    Bmp,
    Tiff,
    WebP,
    Custom(String),
}

use std::str::FromStr;

impl FromStr for ImageFormat {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "png" => Ok(ImageFormat::Png),
            "jpeg" | "jpg" => Ok(ImageFormat::Jpeg),
            "svg" => Ok(ImageFormat::Svg),
            "pdf" => Ok(ImageFormat::Pdf),
            "gif" => Ok(ImageFormat::Gif),
            "bmp" => Ok(ImageFormat::Bmp),
            "tiff" | "tif" => Ok(ImageFormat::Tiff),
            "webp" => Ok(ImageFormat::WebP),
            "auto" => Ok(ImageFormat::Auto),
            _ => Ok(ImageFormat::Custom(s.to_string())),
        }
    }
}

impl ImageFormat {
    pub fn extension(&self) -> &str {
        match self {
            ImageFormat::Auto => "auto",
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpeg",
            ImageFormat::Jpg => "jpg",
            ImageFormat::Svg => "svg",
            ImageFormat::Pdf => "pdf",
            ImageFormat::Gif => "gif",
            ImageFormat::Bmp => "bmp",
            ImageFormat::Tiff => "tiff",
            ImageFormat::WebP => "webp",
            ImageFormat::Custom(ext) => ext,
        }
    }

    pub fn from_str_legacy(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "png" => ImageFormat::Png,
            "jpeg" | "jpg" => ImageFormat::Jpeg,
            "svg" => ImageFormat::Svg,
            "pdf" => ImageFormat::Pdf,
            "gif" => ImageFormat::Gif,
            "bmp" => ImageFormat::Bmp,
            "tiff" | "tif" => ImageFormat::Tiff,
            "webp" => ImageFormat::WebP,
            "auto" => ImageFormat::Auto,
            _ => ImageFormat::Custom(s.to_string()),
        }
    }
}

/// 图像尺寸
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageSize {
    Auto,
    Fixed(f64),
    Relative(f64),
    Fraction(f64),
}

/// 图像适配方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImageFit {
    Contain,
    Cover,
    Fill,
    None,
    ScaleDown,
}

/// 图像缩放方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImageScaling {
    Auto,
    Bilinear,
    Nearest,
    Lanczos,
}

/// 图像滤镜
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageFilter {
    pub blur: Option<f64>,
    pub brightness: Option<f64>,
    pub contrast: Option<f64>,
    pub grayscale: Option<bool>,
    pub invert: Option<bool>,
    pub saturate: Option<f64>,
    pub sepia: Option<f64>,
}

/// 图像配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    pub format: ImageFormat,
    pub width: ImageSize,
    pub height: ImageSize,
    pub alt: Option<String>,
    pub page: Option<usize>,
    pub fit: ImageFit,
    pub scaling: ImageScaling,
    pub filter: ImageFilter,
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            format: ImageFormat::Auto,
            width: ImageSize::Auto,
            height: ImageSize::Auto,
            alt: None,
            page: None,
            fit: ImageFit::Contain,
            scaling: ImageScaling::Auto,
            filter: ImageFilter::default(),
        }
    }
}

/// 图像
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub source: String,
    pub config: ImageConfig,
}

impl Image {
    pub fn new(source: String) -> Self {
        Self {
            source,
            config: ImageConfig::default(),
        }
    }

    pub fn with_format(mut self, format: ImageFormat) -> Self {
        self.config.format = format;
        self
    }

    pub fn with_width(mut self, width: ImageSize) -> Self {
        self.config.width = width;
        self
    }

    pub fn with_height(mut self, height: ImageSize) -> Self {
        self.config.height = height;
        self
    }

    pub fn with_alt(mut self, alt: String) -> Self {
        self.config.alt = Some(alt);
        self
    }

    pub fn with_page(mut self, page: usize) -> Self {
        self.config.page = Some(page);
        self
    }

    pub fn with_fit(mut self, fit: ImageFit) -> Self {
        self.config.fit = fit;
        self
    }

    pub fn with_scaling(mut self, scaling: ImageScaling) -> Self {
        self.config.scaling = scaling;
        self
    }

    pub fn with_filter(mut self, filter: ImageFilter) -> Self {
        self.config.filter = filter;
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        typst.push_str("#image(");
        typst.push_str(&format!("\"{}\"", self.source));

        // 添加格式
        if self.config.format != ImageFormat::Auto {
            typst.push_str(&format!(", format: \"{}\"", self.config.format.extension()));
        }

        // 添加宽度
        match &self.config.width {
            ImageSize::Fixed(size) => typst.push_str(&format!(", width: {}pt", size)),
            ImageSize::Relative(size) => typst.push_str(&format!(", width: {}fr", size)),
            ImageSize::Fraction(size) => typst.push_str(&format!(", width: {}%", size * 100.0)),
            ImageSize::Auto => {}
        }

        // 添加高度
        match &self.config.height {
            ImageSize::Fixed(size) => typst.push_str(&format!(", height: {}pt", size)),
            ImageSize::Relative(size) => typst.push_str(&format!(", height: {}fr", size)),
            ImageSize::Fraction(size) => typst.push_str(&format!(", height: {}%", size * 100.0)),
            ImageSize::Auto => {}
        }

        // 添加 alt 文本
        if let Some(alt) = &self.config.alt {
            typst.push_str(&format!(", alt: \"{}\"", html_escape(alt)));
        }

        // 添加页码
        if let Some(page) = self.config.page {
            typst.push_str(&format!(", page: {}", page));
        }

        // 添加适配方式
        if self.config.fit != ImageFit::Contain {
            typst.push_str(&format!(", fit: \"{}\"", self.fit_to_typst()));
        }

        // 添加缩放方式
        if self.config.scaling != ImageScaling::Auto {
            typst.push_str(&format!(", scaling: \"{}\"", self.scaling_to_typst()));
        }

        typst.push_str(")\n");

        typst
    }

    fn fit_to_typst(&self) -> String {
        match self.config.fit {
            ImageFit::Contain => "contain".to_string(),
            ImageFit::Cover => "cover".to_string(),
            ImageFit::Fill => "fill".to_string(),
            ImageFit::None => "none".to_string(),
            ImageFit::ScaleDown => "scale-down".to_string(),
        }
    }

    fn scaling_to_typst(&self) -> String {
        match self.config.scaling {
            ImageScaling::Auto => "auto".to_string(),
            ImageScaling::Bilinear => "bilinear".to_string(),
            ImageScaling::Nearest => "nearest".to_string(),
            ImageScaling::Lanczos => "lanczos".to_string(),
        }
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        let width_attr = match &self.config.width {
            ImageSize::Fixed(size) => format!(" width=\"{}px\"", size),
            ImageSize::Relative(size) => format!(" width=\"{}%\"", size * 100.0),
            ImageSize::Fraction(size) => format!(" width=\"{}%\"", size * 100.0),
            ImageSize::Auto => String::new(),
        };

        let height_attr = match &self.config.height {
            ImageSize::Fixed(size) => format!(" height=\"{}px\"", size),
            ImageSize::Relative(size) => format!(" height=\"{}%\"", size * 100.0),
            ImageSize::Fraction(size) => format!(" height=\"{}%\"", size * 100.0),
            ImageSize::Auto => String::new(),
        };

        let alt_attr = if let Some(alt) = &self.config.alt {
            format!(" alt=\"{}\"", html_escape(alt))
        } else {
            String::new()
        };

        let style = match self.config.fit {
            ImageFit::Contain => "object-fit: contain;",
            ImageFit::Cover => "object-fit: cover;",
            ImageFit::Fill => "object-fit: fill;",
            ImageFit::None => "object-fit: none;",
            ImageFit::ScaleDown => "object-fit: scale-down;",
        };

        html.push_str(&format!(
            "<img src=\"{}\"{}{}{} style=\"{}\" class=\"typst-image\" />\n",
            html_escape(&self.source),
            width_attr,
            height_attr,
            alt_attr,
            style
        ));

        html
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new("".to_string())
    }
}

/// 图像构建器
pub struct ImageBuilder {
    image: Image,
}

impl ImageBuilder {
    pub fn new(source: String) -> Self {
        Self {
            image: Image::new(source),
        }
    }

    pub fn format(mut self, format: ImageFormat) -> Self {
        self.image = self.image.with_format(format);
        self
    }

    pub fn width(mut self, width: ImageSize) -> Self {
        self.image = self.image.with_width(width);
        self
    }

    pub fn height(mut self, height: ImageSize) -> Self {
        self.image = self.image.with_height(height);
        self
    }

    pub fn alt(mut self, alt: String) -> Self {
        self.image = self.image.with_alt(alt);
        self
    }

    pub fn page(mut self, page: usize) -> Self {
        self.image = self.image.with_page(page);
        self
    }

    pub fn fit(mut self, fit: ImageFit) -> Self {
        self.image = self.image.with_fit(fit);
        self
    }

    pub fn scaling(mut self, scaling: ImageScaling) -> Self {
        self.image = self.image.with_scaling(scaling);
        self
    }

    pub fn filter(mut self, filter: ImageFilter) -> Self {
        self.image = self.image.with_filter(filter);
        self
    }

    pub fn build(self) -> Image {
        self.image
    }
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_creation() {
        let image = Image::new("test.png".to_string());
        assert_eq!(image.source, "test.png");
    }

    #[test]
    fn test_image_default() {
        let image = Image::default();
        assert_eq!(image.source, "");
    }

    #[test]
    fn test_image_config_default() {
        let config = ImageConfig::default();
        assert_eq!(config.format, ImageFormat::Auto);
        assert_eq!(config.fit, ImageFit::Contain);
    }

    #[test]
    fn test_image_with_format() {
        let image = Image::new("test.png".to_string()).with_format(ImageFormat::Png);
        assert_eq!(image.config.format, ImageFormat::Png);
    }

    #[test]
    fn test_image_with_width() {
        let image = Image::new("test.png".to_string()).with_width(ImageSize::Fixed(100.0));
        assert!(matches!(image.config.width, ImageSize::Fixed(100.0)));
    }

    #[test]
    fn test_image_with_height() {
        let image = Image::new("test.png".to_string()).with_height(ImageSize::Fixed(100.0));
        assert!(matches!(image.config.height, ImageSize::Fixed(100.0)));
    }

    #[test]
    fn test_image_with_alt() {
        let image = Image::new("test.png".to_string()).with_alt("Test image".to_string());
        assert_eq!(image.config.alt, Some("Test image".to_string()));
    }

    #[test]
    fn test_image_with_page() {
        let image = Image::new("test.pdf".to_string()).with_page(2);
        assert_eq!(image.config.page, Some(2));
    }

    #[test]
    fn test_image_with_fit() {
        let image = Image::new("test.png".to_string()).with_fit(ImageFit::Cover);
        assert_eq!(image.config.fit, ImageFit::Cover);
    }

    #[test]
    fn test_image_with_scaling() {
        let image = Image::new("test.png".to_string()).with_scaling(ImageScaling::Nearest);
        assert_eq!(image.config.scaling, ImageScaling::Nearest);
    }

    #[test]
    fn test_image_format_from_str() {
        assert_eq!(ImageFormat::from_str("png"), Ok(ImageFormat::Png));
        assert_eq!(ImageFormat::from_str("jpg"), Ok(ImageFormat::Jpeg));
        assert_eq!(ImageFormat::from_str("svg"), Ok(ImageFormat::Svg));
        assert_eq!(ImageFormat::from_str("auto"), Ok(ImageFormat::Auto));
    }

    #[test]
    fn test_image_format_extension() {
        assert_eq!(ImageFormat::Png.extension(), "png");
        assert_eq!(ImageFormat::Jpeg.extension(), "jpeg");
        assert_eq!(ImageFormat::Svg.extension(), "svg");
    }

    #[test]
    fn test_image_fit_partial_eq() {
        assert_eq!(ImageFit::Contain, ImageFit::Contain);
        assert_ne!(ImageFit::Contain, ImageFit::Cover);
    }

    #[test]
    fn test_image_scaling_partial_eq() {
        assert_eq!(ImageScaling::Auto, ImageScaling::Auto);
        assert_ne!(ImageScaling::Auto, ImageScaling::Nearest);
    }

    #[test]
    fn test_image_filter_default() {
        let filter = ImageFilter::default();
        assert!(filter.blur.is_none());
        assert!(filter.brightness.is_none());
    }

    #[test]
    fn test_to_typst() {
        let image = Image::new("test.png".to_string());
        let typst = image.to_typst();
        assert!(typst.contains("#image("));
        assert!(typst.contains("\"test.png\""));
    }

    #[test]
    fn test_to_typst_with_format() {
        let image = Image::new("test.png".to_string()).with_format(ImageFormat::Png);
        let typst = image.to_typst();
        assert!(typst.contains("format: \"png\""));
    }

    #[test]
    fn test_to_typst_with_width() {
        let image = Image::new("test.png".to_string()).with_width(ImageSize::Fixed(100.0));
        let typst = image.to_typst();
        assert!(typst.contains("width: 100pt"));
    }

    #[test]
    fn test_to_typst_with_alt() {
        let image = Image::new("test.png".to_string()).with_alt("Test".to_string());
        let typst = image.to_typst();
        assert!(typst.contains("alt: \"Test\""));
    }

    #[test]
    fn test_to_html() {
        let image = Image::new("test.png".to_string());
        let html = image.to_html();
        assert!(html.contains("<img"));
        assert!(html.contains("src=\"test.png\""));
    }

    #[test]
    fn test_to_html_with_width() {
        let image = Image::new("test.png".to_string()).with_width(ImageSize::Fixed(100.0));
        let html = image.to_html();
        assert!(html.contains("width=\"100px\""));
    }

    #[test]
    fn test_to_html_with_alt() {
        let image = Image::new("test.png".to_string()).with_alt("Test".to_string());
        let html = image.to_html();
        assert!(html.contains("alt=\"Test\""));
    }

    #[test]
    fn test_image_builder() {
        let image = ImageBuilder::new("test.png".to_string())
            .format(ImageFormat::Png)
            .width(ImageSize::Fixed(100.0))
            .build();

        assert_eq!(image.source, "test.png");
        assert_eq!(image.config.format, ImageFormat::Png);
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_image_with_filter() {
        let filter = ImageFilter {
            blur: Some(2.0),
            brightness: Some(1.2),
            contrast: Some(1.1),
            grayscale: None,
            invert: None,
            saturate: None,
            sepia: None,
        };
        let image = Image::new("test.png".to_string()).with_filter(filter);
        assert!(image.config.filter.blur.is_some());
    }

    #[test]
    fn test_image_size_variants() {
        let fixed = ImageSize::Fixed(100.0);
        let relative = ImageSize::Relative(0.5);
        let fraction = ImageSize::Fraction(0.5);
        let auto = ImageSize::Auto;

        assert!(matches!(fixed, ImageSize::Fixed(_)));
        assert!(matches!(relative, ImageSize::Relative(_)));
        assert!(matches!(fraction, ImageSize::Fraction(_)));
        assert!(matches!(auto, ImageSize::Auto));
    }

    #[test]
    fn test_image_fit_variants() {
        assert_eq!(ImageFit::Contain, ImageFit::Contain);
        assert_eq!(ImageFit::Cover, ImageFit::Cover);
        assert_eq!(ImageFit::Fill, ImageFit::Fill);
        assert_eq!(ImageFit::None, ImageFit::None);
        assert_eq!(ImageFit::ScaleDown, ImageFit::ScaleDown);
    }

    #[test]
    fn test_image_scaling_variants() {
        assert_eq!(ImageScaling::Auto, ImageScaling::Auto);
        assert_eq!(ImageScaling::Bilinear, ImageScaling::Bilinear);
        assert_eq!(ImageScaling::Nearest, ImageScaling::Nearest);
        assert_eq!(ImageScaling::Lanczos, ImageScaling::Lanczos);
    }

    #[test]
    fn test_to_typst_with_fit() {
        let image = Image::new("test.png".to_string()).with_fit(ImageFit::Cover);
        let typst = image.to_typst();
        assert!(typst.contains("fit: \"cover\""));
    }

    #[test]
    fn test_to_typst_with_scaling() {
        let image = Image::new("test.png".to_string()).with_scaling(ImageScaling::Nearest);
        let typst = image.to_typst();
        assert!(typst.contains("scaling: \"nearest\""));
    }

    #[test]
    fn test_to_html_object_fit() {
        let image = Image::new("test.png".to_string()).with_fit(ImageFit::Cover);
        let html = image.to_html();
        assert!(html.contains("object-fit: cover"));
    }
}
