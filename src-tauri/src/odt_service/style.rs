use serde::{Deserialize, Serialize};

/// ODT 对齐方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OdtAlignment {
    /// 左对齐
    Left,
    /// 居中
    Center,
    /// 右对齐
    Right,
    /// 两端对齐
    Justify,
    /// 起始
    Start,
    /// 结束
    End,
}

/// ODT 文本样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdtTextStyle {
    /// 字体名称
    pub font_name: String,
    /// 字体大小（点）
    pub font_size: f64,
    /// 是否加粗
    pub bold: bool,
    /// 是否斜体
    pub italic: bool,
    /// 是否下划线
    pub underline: bool,
    /// 文本颜色（RGB）
    pub color: (u8, u8, u8),
}

impl OdtTextStyle {
    /// 创建默认文本样式
    pub fn new() -> Self {
        Self {
            font_name: "Liberation Serif".to_string(),
            font_size: 12.0,
            bold: false,
            italic: false,
            underline: false,
            color: (0, 0, 0),
        }
    }

    /// 设置字体
    #[allow(dead_code)]
    pub fn with_font(mut self, font: String) -> Self {
        self.font_name = font;
        self
    }

    /// 设置字体大小
    pub fn with_size(mut self, size: f64) -> Self {
        self.font_size = size;
        self
    }

    /// 设置加粗
    pub fn with_bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }
}

impl Default for OdtTextStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// ODT 段落样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdtParagraphStyle {
    /// 对齐方式
    pub alignment: OdtAlignment,
    /// 左缩进（厘米）
    pub left_indent: f64,
    /// 右缩进（厘米）
    pub right_indent: f64,
    /// 首行缩进（厘米）
    pub first_line_indent: f64,
    /// 段前间距（厘米）
    pub space_before: f64,
    /// 段后间距（厘米）
    pub space_after: f64,
    /// 行间距
    pub line_spacing: f64,
}

impl OdtParagraphStyle {
    /// 创建默认段落样式
    pub fn new() -> Self {
        Self {
            alignment: OdtAlignment::Left,
            left_indent: 0.0,
            right_indent: 0.0,
            first_line_indent: 0.0,
            space_before: 0.0,
            space_after: 0.2,
            line_spacing: 1.15,
        }
    }

    /// 设置对齐
    #[allow(dead_code)]
    pub fn with_alignment(mut self, alignment: OdtAlignment) -> Self {
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

    /// 创建居中段落样式
    #[allow(dead_code)]
    pub fn centered() -> Self {
        Self::new().with_alignment(OdtAlignment::Center)
    }
}

impl Default for OdtParagraphStyle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odt_text_style_new() {
        let style = OdtTextStyle::new();
        assert_eq!(style.font_name, "Liberation Serif");
        assert_eq!(style.font_size, 12.0);
    }

    #[test]
    fn test_odt_text_style_with_bold() {
        let style = OdtTextStyle::new().with_bold(true);
        assert!(style.bold);
    }

    #[test]
    fn test_odt_paragraph_style_new() {
        let style = OdtParagraphStyle::new();
        assert_eq!(style.alignment, OdtAlignment::Left);
    }

    #[test]
    fn test_odt_paragraph_style_centered() {
        let style = OdtParagraphStyle::centered();
        assert_eq!(style.alignment, OdtAlignment::Center);
    }

    #[test]
    fn test_odt_text_style_serialization() {
        let style = OdtTextStyle::new();
        let json = serde_json::to_string(&style);
        assert!(json.is_ok());
    }

    #[test]
    fn test_odt_text_style_deserialization() {
        let json = r#"{"font_name":"Liberation Serif","font_size":12.0,"bold":false,"italic":false,"underline":false,"color":[0,0,0]}"#;
        let style: OdtTextStyle = serde_json::from_str(json).unwrap();
        assert_eq!(style.font_name, "Liberation Serif");
    }

    #[test]
    fn test_odt_paragraph_style_serialization() {
        let style = OdtParagraphStyle::new();
        let json = serde_json::to_string(&style);
        assert!(json.is_ok());
    }

    #[test]
    fn test_odt_paragraph_style_deserialization() {
        let json = r#"{"alignment":"Left","left_indent":0.0,"right_indent":0.0,"first_line_indent":0.0,"space_before":0.0,"space_after":0.0,"line_spacing":1.0}"#;
        let style: OdtParagraphStyle = serde_json::from_str(json).unwrap();
        assert_eq!(style.alignment, OdtAlignment::Left);
    }

    #[test]
    fn test_odt_text_style_with_font() {
        let style = OdtTextStyle::new().with_font("Arial".to_string());
        assert_eq!(style.font_name, "Arial");
    }

    #[test]
    fn test_odt_text_style_with_size() {
        let style = OdtTextStyle::new().with_size(14.0);
        assert_eq!(style.font_size, 14.0);
    }

    #[test]
    fn test_odt_text_style_chaining() {
        let style = OdtTextStyle::new()
            .with_font("Times New Roman".to_string())
            .with_size(16.0)
            .with_bold(true);
        assert_eq!(style.font_name, "Times New Roman");
        assert_eq!(style.font_size, 16.0);
        assert!(style.bold);
    }

    #[test]
    fn test_odt_text_style_default() {
        let style = OdtTextStyle::default();
        assert_eq!(style.font_name, "Liberation Serif");
        assert_eq!(style.font_size, 12.0);
    }

    #[test]
    fn test_odt_paragraph_style_default() {
        let style = OdtParagraphStyle::default();
        assert_eq!(style.alignment, OdtAlignment::Left);
    }

    #[test]
    fn test_odt_alignment_variants() {
        assert_eq!(OdtAlignment::Left, OdtAlignment::Left);
        assert_eq!(OdtAlignment::Center, OdtAlignment::Center);
        assert_eq!(OdtAlignment::Right, OdtAlignment::Right);
        assert_eq!(OdtAlignment::Justify, OdtAlignment::Justify);
        assert_eq!(OdtAlignment::Start, OdtAlignment::Start);
        assert_eq!(OdtAlignment::End, OdtAlignment::End);
    }

    #[test]
    fn test_odt_text_style_color() {
        let style = OdtTextStyle::new();
        assert_eq!(style.color, (0, 0, 0));
    }

    #[test]
    fn test_odt_paragraph_style_indent() {
        let style = OdtParagraphStyle::new();
        assert_eq!(style.left_indent, 0.0);
        assert_eq!(style.right_indent, 0.0);
        assert_eq!(style.first_line_indent, 0.0);
    }

    #[test]
    fn test_odt_paragraph_style_spacing() {
        let style = OdtParagraphStyle::new();
        assert_eq!(style.space_before, 0.0);
        assert_eq!(style.space_after, 0.2);
        assert_eq!(style.line_spacing, 1.15);
    }

    #[test]
    fn test_odt_text_style_with_size_negative() {
        let style = OdtTextStyle::new().with_size(-5.0);
        assert_eq!(style.font_size, -5.0);
    }

    #[test]
    fn test_odt_text_style_with_size_zero() {
        let style = OdtTextStyle::new().with_size(0.0);
        assert_eq!(style.font_size, 0.0);
    }

    #[test]
    fn test_odt_paragraph_style_with_large_indent() {
        let style = OdtParagraphStyle::new();
        assert_eq!(style.left_indent, 0.0);
    }

    #[test]
    fn test_odt_alignment_serialization() {
        let alignment = OdtAlignment::Center;
        let json = serde_json::to_string(&alignment);
        assert!(json.is_ok());
    }

    #[test]
    fn test_odt_alignment_deserialization() {
        let json = r#""Center""#;
        let alignment: OdtAlignment = serde_json::from_str(json).unwrap();
        assert_eq!(alignment, OdtAlignment::Center);
    }
}
