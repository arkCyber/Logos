use serde::{Deserialize, Serialize};

/// 字体样式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FontStyle {
    /// 常规
    Regular,
    /// 加粗
    Bold,
    /// 斜体
    Italic,
    /// 加粗斜体
    BoldItalic,
}

/// 对齐方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Alignment {
    /// 左对齐
    Left,
    /// 居中
    Center,
    /// 右对齐
    Right,
    /// 两端对齐
    Justify,
}

/// 段落间距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacing {
    /// 段前间距（点）
    pub before: f64,
    /// 段后间距（点）
    pub after: f64,
    /// 行间距（倍数）
    pub line: f64,
}

impl Spacing {
    /// 创建标准间距
    pub fn standard() -> Self {
        Self {
            before: 0.0,
            after: 12.0,
            line: 1.15,
        }
    }

    /// 创建紧凑间距
    #[allow(dead_code)]
    pub fn tight() -> Self {
        Self {
            before: 0.0,
            after: 6.0,
            line: 1.0,
        }
    }

    /// 创建宽松间距
    #[allow(dead_code)]
    pub fn loose() -> Self {
        Self {
            before: 0.0,
            after: 24.0,
            line: 1.5,
        }
    }
}

impl Default for Spacing {
    fn default() -> Self {
        Self::standard()
    }
}

/// 文本样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
    /// 字体名称
    pub font_name: String,
    /// 字体大小（点）
    pub font_size: f64,
    /// 字体样式
    pub font_style: FontStyle,
    /// 文本颜色（RGB）
    pub color: (u8, u8, u8),
    /// 高亮颜色（RGB，可选）
    pub highlight: Option<(u8, u8, u8)>,
    /// 下划线
    pub underline: bool,
    /// 删除线
    pub strikethrough: bool,
    /// 上标
    pub superscript: bool,
    /// 下标
    pub subscript: bool,
}

impl TextStyle {
    /// 创建默认文本样式
    pub fn new() -> Self {
        Self {
            font_name: "Calibri".to_string(),
            font_size: 11.0,
            font_style: FontStyle::Regular,
            color: (0, 0, 0),
            highlight: None,
            underline: false,
            strikethrough: false,
            superscript: false,
            subscript: false,
        }
    }

    /// 设置字体
    pub fn with_font(mut self, font: String) -> Self {
        self.font_name = font;
        self
    }

    /// 设置字体大小
    pub fn with_size(mut self, size: f64) -> Self {
        self.font_size = size;
        self
    }

    /// 设置字体样式
    pub fn with_font_style(mut self, style: FontStyle) -> Self {
        self.font_style = style;
        self
    }

    /// 设置颜色
    #[allow(dead_code)]
    pub fn with_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = (r, g, b);
        self
    }

    /// 设置高亮
    #[allow(dead_code)]
    pub fn with_highlight(mut self, r: u8, g: u8, b: u8) -> Self {
        self.highlight = Some((r, g, b));
        self
    }

    /// 创建标题样式
    pub fn heading1() -> Self {
        Self::new()
            .with_font("Arial".to_string())
            .with_size(16.0)
            .with_font_style(FontStyle::Bold)
    }

    /// 创建标题2样式
    pub fn heading2() -> Self {
        Self::new()
            .with_font("Arial".to_string())
            .with_size(14.0)
            .with_font_style(FontStyle::Bold)
    }

    /// 创建标题3样式
    pub fn heading3() -> Self {
        Self::new()
            .with_font("Arial".to_string())
            .with_size(12.0)
            .with_font_style(FontStyle::Bold)
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// 段落样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParagraphStyle {
    /// 对齐方式
    pub alignment: Alignment,
    /// 首行缩进（点）
    pub first_line_indent: f64,
    /// 左缩进（点）
    pub left_indent: f64,
    /// 右缩进（点）
    pub right_indent: f64,
    /// 段落间距
    pub spacing: Spacing,
    /// 是否保持行不换行
    pub keep_lines: bool,
    /// 是否与下一段保持在一起
    pub keep_with_next: bool,
}

impl ParagraphStyle {
    /// 创建默认段落样式
    pub fn new() -> Self {
        Self {
            alignment: Alignment::Left,
            first_line_indent: 0.0,
            left_indent: 0.0,
            right_indent: 0.0,
            spacing: Spacing::standard(),
            keep_lines: false,
            keep_with_next: false,
        }
    }

    /// 设置对齐
    #[allow(dead_code)]
    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// 设置首行缩进
    #[allow(dead_code)]
    pub fn with_first_line_indent(mut self, indent: f64) -> Self {
        self.first_line_indent = indent;
        self
    }

    /// 设置左缩进
    #[allow(dead_code)]
    pub fn with_left_indent(mut self, indent: f64) -> Self {
        self.left_indent = indent;
        self
    }

    /// 设置间距
    #[allow(dead_code)]
    pub fn with_spacing(mut self, spacing: Spacing) -> Self {
        self.spacing = spacing;
        self
    }

    /// 创建居中段落样式
    #[allow(dead_code)]
    pub fn centered() -> Self {
        Self::new().with_alignment(Alignment::Center)
    }

    /// 创建引用段落样式
    #[allow(dead_code)]
    pub fn quote() -> Self {
        Self::new()
            .with_alignment(Alignment::Left)
            .with_first_line_indent(24.0)
            .with_left_indent(24.0)
    }
}

impl Default for ParagraphStyle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_standard() {
        let spacing = Spacing::standard();
        assert_eq!(spacing.before, 0.0);
        assert_eq!(spacing.after, 12.0);
    }

    #[test]
    fn test_spacing_tight() {
        let spacing = Spacing::tight();
        assert_eq!(spacing.line, 1.0);
    }

    #[test]
    fn test_text_style_new() {
        let style = TextStyle::new();
        assert_eq!(style.font_name, "Calibri");
        assert_eq!(style.font_size, 11.0);
    }

    #[test]
    fn test_text_style_heading1() {
        let style = TextStyle::heading1();
        assert_eq!(style.font_size, 16.0);
        assert_eq!(style.font_style, FontStyle::Bold);
    }

    #[test]
    fn test_text_style_with_color() {
        let style = TextStyle::new().with_color(255, 0, 0);
        assert_eq!(style.color, (255, 0, 0));
    }

    #[test]
    fn test_paragraph_style_new() {
        let style = ParagraphStyle::new();
        assert_eq!(style.alignment, Alignment::Left);
    }

    #[test]
    fn test_paragraph_style_centered() {
        let style = ParagraphStyle::centered();
        assert_eq!(style.alignment, Alignment::Center);
    }

    #[test]
    fn test_paragraph_style_quote() {
        let style = ParagraphStyle::quote();
        assert_eq!(style.first_line_indent, 24.0);
        assert_eq!(style.left_indent, 24.0);
    }

    #[test]
    fn test_text_style_serialization() {
        let style = TextStyle::new();
        let json = serde_json::to_string(&style);
        assert!(json.is_ok());
    }

    #[test]
    fn test_text_style_deserialization() {
        let json = r#"{"font_name":"Calibri","font_size":11.0,"font_style":"Regular","color":[0,0,0],"highlight":null,"underline":false,"strikethrough":false,"superscript":false,"subscript":false}"#;
        let style: TextStyle = serde_json::from_str(json).unwrap();
        assert_eq!(style.font_name, "Calibri");
    }

    #[test]
    fn test_paragraph_style_serialization() {
        let style = ParagraphStyle::new();
        let json = serde_json::to_string(&style);
        assert!(json.is_ok());
    }

    #[test]
    fn test_paragraph_style_deserialization() {
        let json = r#"{"alignment":"Left","first_line_indent":0.0,"left_indent":0.0,"right_indent":0.0,"spacing":{"before":0.0,"after":12.0,"line":1.15},"keep_lines":false,"keep_with_next":false}"#;
        let style: ParagraphStyle = serde_json::from_str(json).unwrap();
        assert_eq!(style.alignment, Alignment::Left);
    }

    #[test]
    fn test_spacing_loose() {
        let spacing = Spacing::loose();
        assert_eq!(spacing.after, 24.0);
        assert_eq!(spacing.line, 1.5);
    }

    #[test]
    fn test_spacing_default() {
        let spacing = Spacing::default();
        assert_eq!(spacing.before, 0.0);
        assert_eq!(spacing.after, 12.0);
    }

    #[test]
    fn test_text_style_heading2() {
        let style = TextStyle::heading2();
        assert_eq!(style.font_size, 14.0);
        assert_eq!(style.font_style, FontStyle::Bold);
    }

    #[test]
    fn test_text_style_heading3() {
        let style = TextStyle::heading3();
        assert_eq!(style.font_size, 12.0);
        assert_eq!(style.font_style, FontStyle::Bold);
    }

    #[test]
    fn test_text_style_with_font() {
        let style = TextStyle::new().with_font("Arial".to_string());
        assert_eq!(style.font_name, "Arial");
    }

    #[test]
    fn test_text_style_with_size() {
        let style = TextStyle::new().with_size(14.0);
        assert_eq!(style.font_size, 14.0);
    }

    #[test]
    fn test_text_style_with_font_style() {
        let style = TextStyle::new().with_font_style(FontStyle::Bold);
        assert_eq!(style.font_style, FontStyle::Bold);
    }

    #[test]
    fn test_text_style_with_highlight() {
        let style = TextStyle::new().with_highlight(255, 255, 0);
        assert_eq!(style.highlight, Some((255, 255, 0)));
    }

    #[test]
    fn test_paragraph_style_with_alignment() {
        let style = ParagraphStyle::new().with_alignment(Alignment::Right);
        assert_eq!(style.alignment, Alignment::Right);
    }

    #[test]
    fn test_paragraph_style_with_first_line_indent() {
        let style = ParagraphStyle::new().with_first_line_indent(24.0);
        assert_eq!(style.first_line_indent, 24.0);
    }

    #[test]
    fn test_paragraph_style_with_left_indent() {
        let style = ParagraphStyle::new().with_left_indent(12.0);
        assert_eq!(style.left_indent, 12.0);
    }

    #[test]
    fn test_paragraph_style_with_spacing() {
        let spacing = Spacing::loose();
        let style = ParagraphStyle::new().with_spacing(spacing);
        assert_eq!(style.spacing.after, 24.0);
    }

    #[test]
    fn test_alignment_left() {
        assert_eq!(Alignment::Left, Alignment::Left);
    }

    #[test]
    fn test_alignment_center() {
        assert_eq!(Alignment::Center, Alignment::Center);
    }

    #[test]
    fn test_alignment_right() {
        assert_eq!(Alignment::Right, Alignment::Right);
    }

    #[test]
    fn test_alignment_justify() {
        assert_eq!(Alignment::Justify, Alignment::Justify);
    }

    #[test]
    fn test_font_style_regular() {
        assert_eq!(FontStyle::Regular, FontStyle::Regular);
    }

    #[test]
    fn test_font_style_bold() {
        assert_eq!(FontStyle::Bold, FontStyle::Bold);
    }

    #[test]
    fn test_font_style_italic() {
        assert_eq!(FontStyle::Italic, FontStyle::Italic);
    }

    #[test]
    fn test_font_style_bold_italic() {
        assert_eq!(FontStyle::BoldItalic, FontStyle::BoldItalic);
    }

    #[test]
    fn test_text_style_default() {
        let style = TextStyle::default();
        assert_eq!(style.font_name, "Calibri");
        assert_eq!(style.font_size, 11.0);
    }

    #[test]
    fn test_paragraph_style_default() {
        let style = ParagraphStyle::default();
        assert_eq!(style.alignment, Alignment::Left);
        assert_eq!(style.spacing.before, 0.0);
    }

    #[test]
    fn test_text_style_underline() {
        let mut style = TextStyle::new();
        style.underline = true;
        assert!(style.underline);
    }

    #[test]
    fn test_text_style_strikethrough() {
        let mut style = TextStyle::new();
        style.strikethrough = true;
        assert!(style.strikethrough);
    }

    #[test]
    fn test_text_style_superscript() {
        let mut style = TextStyle::new();
        style.superscript = true;
        assert!(style.superscript);
    }

    #[test]
    fn test_text_style_subscript() {
        let mut style = TextStyle::new();
        style.subscript = true;
        assert!(style.subscript);
    }
}
