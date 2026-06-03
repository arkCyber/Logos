/*!
 * Aerospace-Grade SVG Style Model
 *
 * Fill, stroke, and font presentation attributes for SVG elements.
 */

use serde::{Deserialize, Serialize};

/// SVG 填充
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgFill {
    /// 填充颜色（RGB，可选）
    pub color: Option<(u8, u8, u8)>,
    /// 渐变引用（例如 `grad1` -> `url(#grad1)`）
    pub gradient_ref: Option<String>,
    /// 填充不透明度（0.0 - 1.0）
    pub opacity: f64,
}

impl SvgFill {
    /// 创建新的填充
    pub fn new() -> Self {
        Self {
            color: None,
            gradient_ref: None,
            opacity: 1.0,
        }
    }

    /// 设置颜色
    #[allow(dead_code)]
    pub fn with_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = Some((r, g, b));
        self
    }

    /// 设置渐变引用 ID
    #[allow(dead_code)]
    pub fn with_gradient_ref(mut self, gradient_ref: String) -> Self {
        self.gradient_ref = Some(gradient_ref);
        self
    }

    /// 设置不透明度
    #[allow(dead_code)]
    pub fn with_opacity(mut self, opacity: f64) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// 创建无填充
    #[allow(dead_code)]
    pub fn none() -> Self {
        Self {
            color: None,
            gradient_ref: None,
            opacity: 0.0,
        }
    }
}

impl Default for SvgFill {
    fn default() -> Self {
        Self::new()
    }
}

/// SVG 描边
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgStroke {
    /// 描边颜色（RGB，可选）
    pub color: Option<(u8, u8, u8)>,
    /// 描边宽度
    pub width: f64,
    /// 描边不透明度（0.0 - 1.0）
    pub opacity: f64,
}

impl SvgStroke {
    /// 创建新的描边
    pub fn new() -> Self {
        Self {
            color: None,
            width: 1.0,
            opacity: 1.0,
        }
    }

    /// 设置颜色
    #[allow(dead_code)]
    pub fn with_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = Some((r, g, b));
        self
    }

    /// 设置宽度
    #[allow(dead_code)]
    pub fn with_width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    /// 创建无描边
    #[allow(dead_code)]
    pub fn none() -> Self {
        Self {
            color: None,
            width: 0.0,
            opacity: 0.0,
        }
    }
}

impl Default for SvgStroke {
    fn default() -> Self {
        Self::new()
    }
}

/// SVG 字体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgFont {
    /// 字体名称
    pub family: String,
    /// 字体大小
    pub size: f64,
    /// 字体粗细
    pub weight: String,
    /// 字体样式
    pub style: String,
}

impl SvgFont {
    /// 创建新的字体
    pub fn new() -> Self {
        Self {
            family: "Arial".to_string(),
            size: 12.0,
            weight: "normal".to_string(),
            style: "normal".to_string(),
        }
    }

    /// 设置字体名称
    #[allow(dead_code)]
    pub fn with_family(mut self, family: String) -> Self {
        self.family = family;
        self
    }

    /// 设置字体大小
    #[allow(dead_code)]
    pub fn with_size(mut self, size: f64) -> Self {
        self.size = size;
        self
    }

    /// 设置粗体
    #[allow(dead_code)]
    pub fn bold() -> Self {
        Self::new().with_weight("bold".to_string())
    }

    /// 设置粗细
    #[allow(dead_code)]
    pub fn with_weight(mut self, weight: String) -> Self {
        self.weight = weight;
        self
    }

    /// 设置字体样式（normal / italic / oblique）
    #[allow(dead_code)]
    pub fn with_style(mut self, style: String) -> Self {
        self.style = style;
        self
    }
}

impl Default for SvgFont {
    fn default() -> Self {
        Self::new()
    }
}

/// SVG 样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgStyle {
    /// 填充
    pub fill: SvgFill,
    /// 描边
    pub stroke: SvgStroke,
    /// 字体
    pub font: Option<SvgFont>,
}

impl SvgStyle {
    /// 创建新的样式
    pub fn new() -> Self {
        Self {
            fill: SvgFill::new(),
            stroke: SvgStroke::new(),
            font: None,
        }
    }

    /// 设置填充
    #[allow(dead_code)]
    pub fn with_fill(mut self, fill: SvgFill) -> Self {
        self.fill = fill;
        self
    }

    /// 设置描边
    #[allow(dead_code)]
    pub fn with_stroke(mut self, stroke: SvgStroke) -> Self {
        self.stroke = stroke;
        self
    }

    /// 设置字体
    #[allow(dead_code)]
    pub fn with_font(mut self, font: SvgFont) -> Self {
        self.font = Some(font);
        self
    }
}

impl Default for SvgStyle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_fill_new() {
        let fill = SvgFill::new();
        assert!(fill.color.is_none());
    }

    #[test]
    fn test_svg_fill_with_color() {
        let fill = SvgFill::new().with_color(255, 0, 0);
        assert_eq!(fill.color, Some((255, 0, 0)));
    }

    #[test]
    fn test_svg_fill_none() {
        let fill = SvgFill::none();
        assert_eq!(fill.opacity, 0.0);
    }

    #[test]
    fn test_svg_stroke_new() {
        let stroke = SvgStroke::new();
        assert_eq!(stroke.width, 1.0);
    }

    #[test]
    fn test_svg_stroke_with_width() {
        let stroke = SvgStroke::new().with_width(2.0);
        assert_eq!(stroke.width, 2.0);
    }

    #[test]
    fn test_svg_font_new() {
        let font = SvgFont::new();
        assert_eq!(font.family, "Arial");
    }

    #[test]
    fn test_svg_font_bold() {
        let font = SvgFont::bold();
        assert_eq!(font.weight, "bold");
    }

    #[test]
    fn test_svg_style_new() {
        let style = SvgStyle::new();
        assert!(style.font.is_none());
    }

    #[test]
    fn test_svg_style_with_fill() {
        let fill = SvgFill::new().with_color(255, 0, 0);
        let style = SvgStyle::new().with_fill(fill);
        assert_eq!(style.fill.color, Some((255, 0, 0)));
    }

    #[test]
    fn test_svg_fill_serialization() {
        let fill = SvgFill::new();
        let json = serde_json::to_string(&fill);
        assert!(json.is_ok());
    }

    #[test]
    fn test_svg_fill_deserialization() {
        let json = r#"{"color":null,"opacity":1.0}"#;
        let fill: SvgFill = serde_json::from_str(json).unwrap();
        assert!(fill.color.is_none());
    }

    #[test]
    fn test_svg_stroke_serialization() {
        let stroke = SvgStroke::new();
        let json = serde_json::to_string(&stroke);
        assert!(json.is_ok());
    }

    #[test]
    fn test_svg_stroke_deserialization() {
        let json = r#"{"color":null,"width":1.0,"opacity":1.0}"#;
        let stroke: SvgStroke = serde_json::from_str(json).unwrap();
        assert_eq!(stroke.width, 1.0);
    }

    #[test]
    fn test_svg_font_serialization() {
        let font = SvgFont::new();
        let json = serde_json::to_string(&font);
        assert!(json.is_ok());
    }

    #[test]
    fn test_svg_font_deserialization() {
        let json = r#"{"family":"Arial","size":12.0,"weight":"normal","style":"normal"}"#;
        let font: SvgFont = serde_json::from_str(json).unwrap();
        assert_eq!(font.family, "Arial");
    }

    #[test]
    fn test_svg_style_serialization() {
        let style = SvgStyle::new();
        let json = serde_json::to_string(&style);
        assert!(json.is_ok());
    }

    #[test]
    fn test_svg_style_deserialization() {
        let json = r#"{"fill":{"color":null,"gradient_ref":null,"opacity":1.0},"stroke":{"color":null,"width":1.0,"opacity":1.0},"font":null}"#;
        let style: SvgStyle = serde_json::from_str(json).unwrap();
        assert_eq!(style.stroke.width, 1.0);
    }

    #[test]
    fn test_svg_fill_default() {
        let fill = SvgFill::default();
        assert!(fill.color.is_none());
        assert_eq!(fill.opacity, 1.0);
    }

    #[test]
    fn test_svg_stroke_default() {
        let stroke = SvgStroke::default();
        assert_eq!(stroke.width, 1.0);
        assert_eq!(stroke.opacity, 1.0);
    }

    #[test]
    fn test_svg_font_default() {
        let font = SvgFont::default();
        assert_eq!(font.family, "Arial");
        assert_eq!(font.size, 12.0);
    }

    #[test]
    fn test_svg_style_default() {
        let style = SvgStyle::default();
        assert!(style.font.is_none());
        assert_eq!(style.stroke.width, 1.0);
    }

    #[test]
    fn test_svg_fill_with_opacity() {
        let fill = SvgFill::new().with_opacity(0.5);
        assert_eq!(fill.opacity, 0.5);
    }

    #[test]
    fn test_svg_fill_opacity_clamp_high() {
        let fill = SvgFill::new().with_opacity(2.0);
        assert_eq!(fill.opacity, 1.0);
    }

    #[test]
    fn test_svg_fill_opacity_clamp_low() {
        let fill = SvgFill::new().with_opacity(-1.0);
        assert_eq!(fill.opacity, 0.0);
    }

    #[test]
    fn test_svg_stroke_with_color() {
        let stroke = SvgStroke::new().with_color(0, 0, 255);
        assert_eq!(stroke.color, Some((0, 0, 255)));
    }

    #[test]
    fn test_svg_font_with_family() {
        let font = SvgFont::new().with_family("Times New Roman".to_string());
        assert_eq!(font.family, "Times New Roman");
    }

    #[test]
    fn test_svg_font_with_size() {
        let font = SvgFont::new().with_size(24.0);
        assert_eq!(font.size, 24.0);
    }

    #[test]
    fn test_svg_style_with_stroke() {
        let stroke = SvgStroke::new().with_width(3.0);
        let style = SvgStyle::new().with_stroke(stroke);
        assert_eq!(style.stroke.width, 3.0);
    }

    #[test]
    fn test_svg_style_with_font() {
        let font = SvgFont::new().with_size(18.0);
        let style = SvgStyle::new().with_font(font);
        assert!(style.font.is_some());
    }

    #[test]
    fn test_svg_style_chaining() {
        let fill = SvgFill::new().with_color(255, 0, 0);
        let stroke = SvgStroke::new().with_width(2.0);
        let style = SvgStyle::new()
            .with_fill(fill)
            .with_stroke(stroke);
        assert_eq!(style.fill.color, Some((255, 0, 0)));
        assert_eq!(style.stroke.width, 2.0);
    }

    #[test]
    fn test_svg_fill_with_rgb() {
        let fill = SvgFill::new().with_color(128, 128, 128);
        assert_eq!(fill.color, Some((128, 128, 128)));
    }

    #[test]
    fn test_svg_stroke_width_zero() {
        let stroke = SvgStroke::new().with_width(0.0);
        assert_eq!(stroke.width, 0.0);
    }

    #[test]
    fn test_svg_font_size_zero() {
        let font = SvgFont::new().with_size(0.0);
        assert_eq!(font.size, 0.0);
    }

    #[test]
    fn test_svg_font_empty_family() {
        let font = SvgFont::new().with_family("".to_string());
        assert_eq!(font.family, "");
    }
}
