/*!
 * 航空航天级字体系统
 * 实现 Typst 的字体功能（字体族、字体大小、字体样式、字体粗细）
 */

use serde::{Deserialize, Serialize};

/// 字体族
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FontFamily {
    Serif,
    Sans,
    Mono,
    Cursive,
    Fantasy,
    Custom(String),
}

/// 字体样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

/// 字体粗细
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
    Custom(u16),
}

/// 字体配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    pub family: FontFamily,
    pub size: Option<f64>,
    pub style: FontStyle,
    pub weight: FontWeight,
    pub line_height: Option<f64>,
    pub letter_spacing: Option<f64>,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            family: FontFamily::Sans,
            size: Some(12.0),
            style: FontStyle::Normal,
            weight: FontWeight::Regular,
            line_height: None,
            letter_spacing: None,
        }
    }
}

/// 字体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Font {
    pub config: FontConfig,
}

impl Font {
    pub fn new() -> Self {
        Self {
            config: FontConfig::default(),
        }
    }

    pub fn with_config(mut self, config: FontConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_family(mut self, family: FontFamily) -> Self {
        self.config.family = family;
        self
    }

    pub fn with_size(mut self, size: f64) -> Self {
        self.config.size = Some(size);
        self
    }

    pub fn with_style(mut self, style: FontStyle) -> Self {
        self.config.style = style;
        self
    }

    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.config.weight = weight;
        self
    }

    pub fn with_line_height(mut self, line_height: f64) -> Self {
        self.config.line_height = Some(line_height);
        self
    }

    pub fn with_letter_spacing(mut self, letter_spacing: f64) -> Self {
        self.config.letter_spacing = Some(letter_spacing);
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        typst.push_str("#set text(");

        // 添加字体族
        match &self.config.family {
            FontFamily::Serif => typst.push_str("font: \"New Computer Modern Serif\", "),
            FontFamily::Sans => typst.push_str("font: \"New Computer Modern Sans\", "),
            FontFamily::Mono => typst.push_str("font: \"New Computer Modern Mono\", "),
            FontFamily::Cursive => typst.push_str("font: \"cursive\", "),
            FontFamily::Fantasy => typst.push_str("font: \"fantasy\", "),
            FontFamily::Custom(name) => typst.push_str(&format!("font: \"{}\", ", name)),
        }

        // 添加字体大小
        if let Some(size) = self.config.size {
            typst.push_str(&format!("size: {}pt, ", size));
        }

        // 添加字体样式
        match self.config.style {
            FontStyle::Italic => typst.push_str("style: italic, "),
            FontStyle::Oblique => typst.push_str("style: oblique, "),
            FontStyle::Normal => {}
        }

        // 添加字体粗细
        match self.config.weight {
            FontWeight::Thin => typst.push_str("weight: 100, "),
            FontWeight::ExtraLight => typst.push_str("weight: 200, "),
            FontWeight::Light => typst.push_str("weight: 300, "),
            FontWeight::Regular => typst.push_str("weight: 400, "),
            FontWeight::Medium => typst.push_str("weight: 500, "),
            FontWeight::SemiBold => typst.push_str("weight: 600, "),
            FontWeight::Bold => typst.push_str("weight: 700, "),
            FontWeight::ExtraBold => typst.push_str("weight: 800, "),
            FontWeight::Black => typst.push_str("weight: 900, "),
            FontWeight::Custom(w) => typst.push_str(&format!("weight: {}, ", w)),
        }

        // 添加行高
        if let Some(line_height) = self.config.line_height {
            typst.push_str(&format!("leading: {}em, ", line_height));
        }

        // 添加字间距
        if let Some(letter_spacing) = self.config.letter_spacing {
            typst.push_str(&format!("tracking: {}em, ", letter_spacing));
        }

        // 移除最后的逗号和空格
        if typst.ends_with(", ") {
            typst.pop();
            typst.pop();
        }

        typst.push_str(")\n");

        typst
    }

    /// 转换为 CSS
    pub fn to_css(&self) -> String {
        let mut css = String::new();

        css.push_str("font-family: ");

        match &self.config.family {
            FontFamily::Serif => css.push_str("\"Times New Roman\", serif"),
            FontFamily::Sans => css.push_str("Arial, sans-serif"),
            FontFamily::Mono => css.push_str("\"Courier New\", monospace"),
            FontFamily::Cursive => css.push_str("cursive"),
            FontFamily::Fantasy => css.push_str("fantasy"),
            FontFamily::Custom(name) => css.push_str(&format!("\"{}\", sans-serif", name)),
        }

        css.push_str(";\n");

        if let Some(size) = self.config.size {
            css.push_str(&format!("font-size: {}px;\n", size));
        }

        match self.config.style {
            FontStyle::Italic => css.push_str("font-style: italic;\n"),
            FontStyle::Oblique => css.push_str("font-style: oblique;\n"),
            FontStyle::Normal => css.push_str("font-style: normal;\n"),
        }

        match self.config.weight {
            FontWeight::Thin => css.push_str("font-weight: 100;\n"),
            FontWeight::ExtraLight => css.push_str("font-weight: 200;\n"),
            FontWeight::Light => css.push_str("font-weight: 300;\n"),
            FontWeight::Regular => css.push_str("font-weight: 400;\n"),
            FontWeight::Medium => css.push_str("font-weight: 500;\n"),
            FontWeight::SemiBold => css.push_str("font-weight: 600;\n"),
            FontWeight::Bold => css.push_str("font-weight: 700;\n"),
            FontWeight::ExtraBold => css.push_str("font-weight: 800;\n"),
            FontWeight::Black => css.push_str("font-weight: 900;\n"),
            FontWeight::Custom(w) => css.push_str(&format!("font-weight: {};\n", w)),
        }

        if let Some(line_height) = self.config.line_height {
            css.push_str(&format!("line-height: {};\n", line_height));
        }

        if let Some(letter_spacing) = self.config.letter_spacing {
            css.push_str(&format!("letter-spacing: {}em;\n", letter_spacing));
        }

        css
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::new()
    }
}

/// 字体构建器
pub struct FontBuilder {
    font: Font,
}

impl FontBuilder {
    pub fn new() -> Self {
        Self { font: Font::new() }
    }

    pub fn family(mut self, family: FontFamily) -> Self {
        self.font = self.font.with_family(family);
        self
    }

    pub fn size(mut self, size: f64) -> Self {
        self.font = self.font.with_size(size);
        self
    }

    pub fn style(mut self, style: FontStyle) -> Self {
        self.font = self.font.with_style(style);
        self
    }

    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.font = self.font.with_weight(weight);
        self
    }

    pub fn line_height(mut self, line_height: f64) -> Self {
        self.font = self.font.with_line_height(line_height);
        self
    }

    pub fn letter_spacing(mut self, letter_spacing: f64) -> Self {
        self.font = self.font.with_letter_spacing(letter_spacing);
        self
    }

    pub fn build(self) -> Font {
        self.font
    }
}

impl Default for FontBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_creation() {
        let font = Font::new();
        assert!(matches!(font.config.family, FontFamily::Sans));
    }

    #[test]
    fn test_font_default() {
        let font = Font::default();
        assert!(matches!(font.config.family, FontFamily::Sans));
    }

    #[test]
    fn test_font_config_default() {
        let config = FontConfig::default();
        assert!(matches!(config.family, FontFamily::Sans));
        assert_eq!(config.size, Some(12.0));
    }

    #[test]
    fn test_font_with_family() {
        let font = Font::new().with_family(FontFamily::Serif);
        assert!(matches!(font.config.family, FontFamily::Serif));
    }

    #[test]
    fn test_font_with_size() {
        let font = Font::new().with_size(14.0);
        assert_eq!(font.config.size, Some(14.0));
    }

    #[test]
    fn test_font_with_style() {
        let font = Font::new().with_style(FontStyle::Italic);
        assert!(matches!(font.config.style, FontStyle::Italic));
    }

    #[test]
    fn test_font_with_weight() {
        let font = Font::new().with_weight(FontWeight::Bold);
        assert!(matches!(font.config.weight, FontWeight::Bold));
    }

    #[test]
    fn test_font_with_line_height() {
        let font = Font::new().with_line_height(1.5);
        assert_eq!(font.config.line_height, Some(1.5));
    }

    #[test]
    fn test_font_with_letter_spacing() {
        let font = Font::new().with_letter_spacing(0.1);
        assert_eq!(font.config.letter_spacing, Some(0.1));
    }

    #[test]
    fn test_font_family_variants() {
        assert!(matches!(FontFamily::Serif, FontFamily::Serif));
        assert!(matches!(FontFamily::Sans, FontFamily::Sans));
        assert!(matches!(
            FontFamily::Custom("test".to_string()),
            FontFamily::Custom(_)
        ));
    }

    #[test]
    fn test_font_style_variants() {
        assert!(matches!(FontStyle::Normal, FontStyle::Normal));
        assert!(matches!(FontStyle::Italic, FontStyle::Italic));
        assert!(matches!(FontStyle::Oblique, FontStyle::Oblique));
    }

    #[test]
    fn test_font_weight_variants() {
        assert!(matches!(FontWeight::Regular, FontWeight::Regular));
        assert!(matches!(FontWeight::Bold, FontWeight::Bold));
        assert!(matches!(FontWeight::Custom(500), FontWeight::Custom(_)));
    }

    #[test]
    fn test_to_typst() {
        let font = Font::new();
        let typst = font.to_typst();
        assert!(typst.contains("#set text("));
        assert!(typst.contains("font:"));
    }

    #[test]
    fn test_to_typst_with_size() {
        let font = Font::new().with_size(14.0);
        let typst = font.to_typst();
        assert!(typst.contains("size: 14pt"));
    }

    #[test]
    fn test_to_typst_with_style() {
        let font = Font::new().with_style(FontStyle::Italic);
        let typst = font.to_typst();
        assert!(typst.contains("style: italic"));
    }

    #[test]
    fn test_to_typst_with_weight() {
        let font = Font::new().with_weight(FontWeight::Bold);
        let typst = font.to_typst();
        assert!(typst.contains("weight: 700"));
    }

    #[test]
    fn test_to_css() {
        let font = Font::new();
        let css = font.to_css();
        assert!(css.contains("font-family:"));
    }

    #[test]
    fn test_to_css_with_size() {
        let font = Font::new().with_size(14.0);
        let css = font.to_css();
        assert!(css.contains("font-size: 14px"));
    }

    #[test]
    fn test_to_css_with_style() {
        let font = Font::new().with_style(FontStyle::Italic);
        let css = font.to_css();
        assert!(css.contains("font-style: italic"));
    }

    #[test]
    fn test_to_css_with_weight() {
        let font = Font::new().with_weight(FontWeight::Bold);
        let css = font.to_css();
        assert!(css.contains("font-weight: 700"));
    }

    #[test]
    fn test_font_builder() {
        let font = FontBuilder::new()
            .family(FontFamily::Serif)
            .size(14.0)
            .weight(FontWeight::Bold)
            .build();

        assert!(matches!(font.config.family, FontFamily::Serif));
        assert_eq!(font.config.size, Some(14.0));
    }

    #[test]
    fn test_font_builder_default() {
        let builder = FontBuilder::default();
        let font = builder.build();
        assert!(matches!(font.config.family, FontFamily::Sans));
    }

    #[test]
    fn test_to_typst_serif() {
        let font = Font::new().with_family(FontFamily::Serif);
        let typst = font.to_typst();
        assert!(typst.contains("New Computer Modern Serif"));
    }

    #[test]
    fn test_to_typst_mono() {
        let font = Font::new().with_family(FontFamily::Mono);
        let typst = font.to_typst();
        assert!(typst.contains("New Computer Modern Mono"));
    }

    #[test]
    fn test_to_typst_custom_family() {
        let font = Font::new().with_family(FontFamily::Custom("Custom Font".to_string()));
        let typst = font.to_typst();
        assert!(typst.contains("Custom Font"));
    }

    #[test]
    fn test_to_css_serif() {
        let font = Font::new().with_family(FontFamily::Serif);
        let css = font.to_css();
        assert!(css.contains("Times New Roman"));
    }

    #[test]
    fn test_to_css_mono() {
        let font = Font::new().with_family(FontFamily::Mono);
        let css = font.to_css();
        assert!(css.contains("Courier New"));
    }

    #[test]
    fn test_to_typst_with_line_height() {
        let font = Font::new().with_line_height(1.5);
        let typst = font.to_typst();
        assert!(typst.contains("leading: 1.5em"));
    }

    #[test]
    fn test_to_css_with_line_height() {
        let font = Font::new().with_line_height(1.5);
        let css = font.to_css();
        assert!(css.contains("line-height: 1.5"));
    }

    #[test]
    fn test_to_typst_with_letter_spacing() {
        let font = Font::new().with_letter_spacing(0.1);
        let typst = font.to_typst();
        assert!(typst.contains("tracking: 0.1em"));
    }

    #[test]
    fn test_to_css_with_letter_spacing() {
        let font = Font::new().with_letter_spacing(0.1);
        let css = font.to_css();
        assert!(css.contains("letter-spacing: 0.1em"));
    }
}
