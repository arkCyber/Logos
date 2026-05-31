use serde::{Deserialize, Serialize};

/// RTF 对齐方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RtfAlignment {
    /// 左对齐
    Left,
    /// 居中
    Center,
    /// 右对齐
    Right,
    /// 两端对齐
    Justify,
}

/// RTF 字体
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct RtfFont {
    /// 字体名称
    pub name: String,
    /// 字体表中的索引
    pub index: usize,
}

impl RtfFont {
    /// 创建新的字体
    #[allow(dead_code)]
    pub fn new(name: String, index: usize) -> Self {
        Self { name, index }
    }
}

/// RTF 文本样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtfTextStyle {
    /// 是否加粗
    pub bold: bool,
    /// 是否斜体
    pub italic: bool,
    /// 是否下划线
    pub underline: bool,
    /// 是否删除线
    pub strikethrough: bool,
    /// 字体大小（半点）
    pub font_size: Option<u16>,
    /// 字体颜色索引
    pub color_index: Option<usize>,
    /// 背景颜色索引
    pub background_index: Option<usize>,
    /// 字体索引
    pub font_index: Option<usize>,
}

impl RtfTextStyle {
    /// 创建默认文本样式
    pub fn new() -> Self {
        Self {
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
            font_size: None,
            color_index: None,
            background_index: None,
            font_index: None,
        }
    }

    /// 设置加粗
    pub fn with_bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// 设置斜体
    #[allow(dead_code)]
    pub fn with_italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    /// 设置下划线
    #[allow(dead_code)]
    pub fn with_underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    /// 设置字体大小（点）
    pub fn with_font_size(mut self, size: u16) -> Self {
        self.font_size = Some(size * 2);
        self
    }
}

impl Default for RtfTextStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// RTF 段落样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtfParagraphStyle {
    /// 对齐方式
    pub alignment: RtfAlignment,
    /// 左缩进（缇）
    pub left_indent: i32,
    /// 右缩进（缇）
    pub right_indent: i32,
    /// 首行缩进（缇）
    pub first_line_indent: i32,
    /// 段前间距（缇）
    pub space_before: i32,
    /// 段后间距（缇）
    pub space_after: i32,
    /// 行间距（缇）
    pub line_spacing: i32,
}

impl RtfParagraphStyle {
    /// 创建默认段落样式
    pub fn new() -> Self {
        Self {
            alignment: RtfAlignment::Left,
            left_indent: 0,
            right_indent: 0,
            first_line_indent: 0,
            space_before: 0,
            space_after: 0,
            line_spacing: 240, // 单倍行间距
        }
    }

    /// 设置对齐
    #[allow(dead_code)]
    pub fn with_alignment(mut self, alignment: RtfAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// 设置首行缩进（缇，1缇 = 1/1440英寸）
    #[allow(dead_code)]
    pub fn with_first_line_indent(mut self, indent: i32) -> Self {
        self.first_line_indent = indent;
        self
    }

    /// 设置左缩进（缇）
    #[allow(dead_code)]
    pub fn with_left_indent(mut self, indent: i32) -> Self {
        self.left_indent = indent;
        self
    }

    /// 创建居中段落样式
    #[allow(dead_code)]
    pub fn centered() -> Self {
        Self::new().with_alignment(RtfAlignment::Center)
    }
}

impl Default for RtfParagraphStyle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtf_font_new() {
        let font = RtfFont::new("Arial".to_string(), 0);
        assert_eq!(font.name, "Arial");
        assert_eq!(font.index, 0);
    }

    #[test]
    fn test_rtf_text_style_new() {
        let style = RtfTextStyle::new();
        assert!(!style.bold);
        assert!(!style.italic);
    }

    #[test]
    fn test_rtf_text_style_with_bold() {
        let style = RtfTextStyle::new().with_bold(true);
        assert!(style.bold);
    }

    #[test]
    fn test_rtf_text_style_with_font_size() {
        let style = RtfTextStyle::new().with_font_size(12);
        assert_eq!(style.font_size, Some(24));
    }

    #[test]
    fn test_rtf_paragraph_style_new() {
        let style = RtfParagraphStyle::new();
        assert_eq!(style.alignment, RtfAlignment::Left);
        assert_eq!(style.line_spacing, 240);
    }

    #[test]
    fn test_rtf_paragraph_style_centered() {
        let style = RtfParagraphStyle::centered();
        assert_eq!(style.alignment, RtfAlignment::Center);
    }

    #[test]
    fn test_rtf_paragraph_style_with_first_line_indent() {
        let style = RtfParagraphStyle::new().with_first_line_indent(360);
        assert_eq!(style.first_line_indent, 360);
    }

    #[test]
    fn test_rtf_text_style_serialization() {
        let style = RtfTextStyle::new();
        let json = serde_json::to_string(&style);
        assert!(json.is_ok());
    }

    #[test]
    fn test_rtf_text_style_deserialization() {
        let json = r#"{"bold":false,"italic":false,"underline":false,"strikethrough":false,"font_size":null,"color_index":null,"background_index":null,"font_index":null}"#;
        let style: RtfTextStyle = serde_json::from_str(json).unwrap();
        assert!(!style.bold);
    }

    #[test]
    fn test_rtf_paragraph_style_serialization() {
        let style = RtfParagraphStyle::new();
        let json = serde_json::to_string(&style);
        assert!(json.is_ok());
    }

    #[test]
    fn test_rtf_paragraph_style_deserialization() {
        let json = r#"{"alignment":"Left","left_indent":0,"right_indent":0,"first_line_indent":0,"space_before":0,"space_after":0,"line_spacing":240}"#;
        let style: RtfParagraphStyle = serde_json::from_str(json).unwrap();
        assert_eq!(style.alignment, RtfAlignment::Left);
    }

    #[test]
    fn test_rtf_font_serialization() {
        let font = RtfFont::new("Arial".to_string(), 0);
        let json = serde_json::to_string(&font);
        assert!(json.is_ok());
    }

    #[test]
    fn test_rtf_font_deserialization() {
        let json = r#"{"name":"Arial","index":0}"#;
        let font: RtfFont = serde_json::from_str(json).unwrap();
        assert_eq!(font.name, "Arial");
    }

    #[test]
    fn test_rtf_alignment_serialization() {
        let alignment = RtfAlignment::Center;
        let json = serde_json::to_string(&alignment);
        assert!(json.is_ok());
    }

    #[test]
    fn test_rtf_alignment_deserialization() {
        let json = r#""Center""#;
        let alignment: RtfAlignment = serde_json::from_str(json).unwrap();
        assert_eq!(alignment, RtfAlignment::Center);
    }

    #[test]
    fn test_rtf_text_style_default() {
        let style = RtfTextStyle::default();
        assert!(!style.bold);
        assert!(!style.italic);
        assert_eq!(style.font_size, None);
    }

    #[test]
    fn test_rtf_paragraph_style_default() {
        let style = RtfParagraphStyle::default();
        assert_eq!(style.alignment, RtfAlignment::Left);
        assert_eq!(style.line_spacing, 240);
    }

    #[test]
    fn test_rtf_text_style_with_italic() {
        let style = RtfTextStyle::new().with_italic(true);
        assert!(style.italic);
    }

    #[test]
    fn test_rtf_text_style_with_underline() {
        let style = RtfTextStyle::new().with_underline(true);
        assert!(style.underline);
    }

    #[test]
    fn test_rtf_text_style_chaining() {
        let style = RtfTextStyle::new()
            .with_bold(true)
            .with_italic(true)
            .with_underline(true)
            .with_font_size(14);
        assert!(style.bold);
        assert!(style.italic);
        assert!(style.underline);
        assert_eq!(style.font_size, Some(28));
    }

    #[test]
    fn test_rtf_paragraph_style_with_alignment() {
        let style = RtfParagraphStyle::new().with_alignment(RtfAlignment::Right);
        assert_eq!(style.alignment, RtfAlignment::Right);
    }

    #[test]
    fn test_rtf_alignment_variants() {
        assert_eq!(RtfAlignment::Left, RtfAlignment::Left);
        assert_eq!(RtfAlignment::Center, RtfAlignment::Center);
        assert_eq!(RtfAlignment::Right, RtfAlignment::Right);
        assert_eq!(RtfAlignment::Justify, RtfAlignment::Justify);
    }

    #[test]
    fn test_rtf_text_style_font_size_none() {
        let style = RtfTextStyle::new();
        assert_eq!(style.font_size, None);
    }

    #[test]
    fn test_rtf_text_style_color_index_none() {
        let style = RtfTextStyle::new();
        assert_eq!(style.color_index, None);
    }

    #[test]
    fn test_rtf_paragraph_style_indent_values() {
        let style = RtfParagraphStyle::new();
        assert_eq!(style.left_indent, 0);
        assert_eq!(style.right_indent, 0);
        assert_eq!(style.first_line_indent, 0);
    }

    #[test]
    fn test_rtf_paragraph_style_spacing_values() {
        let style = RtfParagraphStyle::new();
        assert_eq!(style.space_before, 0);
        assert_eq!(style.space_after, 0);
        assert_eq!(style.line_spacing, 240);
    }

    #[test]
    fn test_rtf_font_with_large_index() {
        let font = RtfFont::new("Arial".to_string(), 1000);
        assert_eq!(font.index, 1000);
    }

    #[test]
    fn test_rtf_font_empty_name() {
        let font = RtfFont::new("".to_string(), 0);
        assert_eq!(font.name, "");
    }

    #[test]
    fn test_rtf_text_style_with_large_font_size() {
        let style = RtfTextStyle::new().with_font_size(100);
        assert_eq!(style.font_size, Some(200));
    }
}
