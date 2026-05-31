use serde::{Deserialize, Serialize};

/// 文本方向
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TextDirection {
    /// 横向
    Horizontal,
    /// 纵向
    Vertical,
    /// 堆叠
    Stacked,
}

/// 文本对齐
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TextAlignment {
    /// 左对齐
    Left,
    /// 居中
    Center,
    /// 右对齐
    Right,
    /// 两端对齐
    Justify,
}

/// 编号样式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NumberingStyle {
    /// 阿拉伯数字 (1, 2, 3)
    Arabic,
    /// 罗马数字大写 (I, II, III)
    RomanUpper,
    /// 罗马数字小写 (i, ii, iii)
    RomanLower,
    /// 字母大写 (A, B, C)
    LetterUpper,
    /// 字母小写 (a, b, c)
    LetterLower,
    /// 自定义
    Custom(String),
}

/// 文本样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
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
    /// 是否删除线
    pub strikethrough: bool,
    /// 文本颜色（RGB）
    pub color: (u8, u8, u8),
    /// 背景颜色（RGB）
    pub background_color: Option<(u8, u8, u8)>,
    /// 字间距
    pub letter_spacing: f64,
    /// 行间距
    pub line_spacing: f64,
    /// 文本方向
    pub direction: TextDirection,
}

impl TextStyle {
    /// 创建默认文本样式
    pub fn new() -> Self {
        Self {
            font_name: "Calibri".to_string(),
            font_size: 18.0,
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
            color: (0, 0, 0),
            background_color: None,
            letter_spacing: 0.0,
            line_spacing: 1.0,
            direction: TextDirection::Horizontal,
        }
    }

    /// 设置文本方向
    pub fn with_direction(mut self, direction: TextDirection) -> Self {
        self.direction = direction;
        self
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

    /// 设置加粗
    pub fn with_bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// 设置斜体
    pub fn with_italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    /// 设置下划线
    pub fn with_underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    /// 设置颜色
    pub fn with_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = (r, g, b);
        self
    }

    /// 创建标题样式
    pub fn heading() -> Self {
        Self::new()
            .with_font("Arial".to_string())
            .with_size(36.0)
            .with_bold(true)
    }

    /// 创建副标题样式
    pub fn subtitle() -> Self {
        Self::new()
            .with_font("Arial".to_string())
            .with_size(24.0)
            .with_bold(true)
    }

    /// 创建正文样式
    pub fn body() -> Self {
        Self::new().with_font("Calibri".to_string()).with_size(18.0)
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
    pub alignment: TextAlignment,
    /// 首行缩进（点）
    pub first_line_indent: f64,
    /// 左缩进（点）
    pub left_indent: f64,
    /// 右缩进（点）
    pub right_indent: f64,
    /// 段前间距（点）
    pub space_before: f64,
    /// 段后间距（点）
    pub space_after: f64,
    /// 项目符号
    pub bullet: bool,
    /// 编号
    pub numbered: bool,
    /// 编号样式
    pub numbering_style: NumberingStyle,
    /// 编号起始值
    pub numbering_start: u32,
}

impl ParagraphStyle {
    /// 创建默认段落样式
    pub fn new() -> Self {
        Self {
            alignment: TextAlignment::Left,
            first_line_indent: 0.0,
            left_indent: 0.0,
            right_indent: 0.0,
            space_before: 0.0,
            space_after: 0.0,
            bullet: false,
            numbered: false,
            numbering_style: NumberingStyle::Arabic,
            numbering_start: 1,
        }
    }

    /// 设置编号样式
    pub fn with_numbering_style(mut self, style: NumberingStyle) -> Self {
        self.numbering_style = style;
        self
    }

    /// 设置编号起始值
    pub fn with_numbering_start(mut self, start: u32) -> Self {
        self.numbering_start = start;
        self
    }

    /// 设置对齐
    pub fn with_alignment(mut self, alignment: TextAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// 设置项目符号
    pub fn with_bullet(mut self, bullet: bool) -> Self {
        self.bullet = bullet;
        self
    }

    /// 设置编号
    pub fn with_numbered(mut self, numbered: bool) -> Self {
        self.numbered = numbered;
        self
    }

    /// 创建居中段落
    pub fn centered() -> Self {
        Self::new().with_alignment(TextAlignment::Center)
    }
}

impl Default for ParagraphStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// 文本元素
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextElement {
    /// 文本内容
    pub content: String,
    /// 文本样式
    pub style: TextStyle,
    /// 段落样式
    pub paragraph_style: ParagraphStyle,
    /// 位置（X, Y 坐标，单位：点）
    pub position: (f64, f64),
    /// 尺寸（宽度、高度，单位：点）
    pub size: (f64, f64),
}

impl TextElement {
    /// 创建新的文本元素
    pub fn new(content: String) -> Self {
        Self {
            content,
            style: TextStyle::new(),
            paragraph_style: ParagraphStyle::new(),
            position: (0.0, 0.0),
            size: (100.0, 20.0),
        }
    }

    /// 设置样式
    pub fn with_style(mut self, style: TextStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置段落样式
    pub fn with_paragraph_style(mut self, style: ParagraphStyle) -> Self {
        self.paragraph_style = style;
        self
    }

    /// 设置位置
    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.position = (x, y);
        self
    }

    /// 设置尺寸
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.size = (width, height);
        self
    }

    /// 创建标题文本
    pub fn heading(content: String) -> Self {
        Self::new(content)
            .with_style(TextStyle::heading())
            .with_paragraph_style(ParagraphStyle::centered())
    }

    /// 创建正文文本
    pub fn body(content: String) -> Self {
        Self::new(content).with_style(TextStyle::body())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_style_new() {
        let style = TextStyle::new();
        assert_eq!(style.font_name, "Calibri");
        assert_eq!(style.font_size, 18.0);
    }

    #[test]
    fn test_text_style_with_font() {
        let style = TextStyle::new().with_font("Arial".to_string());
        assert_eq!(style.font_name, "Arial");
    }

    #[test]
    fn test_text_style_heading() {
        let style = TextStyle::heading();
        assert_eq!(style.font_size, 36.0);
        assert!(style.bold);
    }

    #[test]
    fn test_text_style_body() {
        let style = TextStyle::body();
        assert_eq!(style.font_size, 18.0);
    }

    #[test]
    fn test_paragraph_style_new() {
        let style = ParagraphStyle::new();
        assert_eq!(style.alignment, TextAlignment::Left);
    }

    #[test]
    fn test_paragraph_style_with_alignment() {
        let style = ParagraphStyle::new().with_alignment(TextAlignment::Center);
        assert_eq!(style.alignment, TextAlignment::Center);
    }

    #[test]
    fn test_paragraph_style_centered() {
        let style = ParagraphStyle::centered();
        assert_eq!(style.alignment, TextAlignment::Center);
    }

    #[test]
    fn test_text_element_new() {
        let text = TextElement::new("Hello".to_string());
        assert_eq!(text.content, "Hello");
    }

    #[test]
    fn test_text_element_with_style() {
        let style = TextStyle::heading();
        let text = TextElement::new("Title".to_string()).with_style(style);
        assert!(text.style.bold);
    }

    #[test]
    fn test_text_element_heading() {
        let text = TextElement::heading("Presentation".to_string());
        assert_eq!(text.content, "Presentation");
        assert!(text.style.bold);
    }

    #[test]
    fn test_text_element_body() {
        let text = TextElement::body("Content".to_string());
        assert_eq!(text.content, "Content");
    }

    #[test]
    fn test_text_element_chaining() {
        let text = TextElement::new("Test".to_string())
            .with_style(TextStyle::heading())
            .with_position(100.0, 200.0)
            .with_size(300.0, 50.0);
        assert_eq!(text.position, (100.0, 200.0));
        assert_eq!(text.size, (300.0, 50.0));
    }

    #[test]
    fn test_text_element_serialization() {
        let text = TextElement::new("Test".to_string());
        let json = serde_json::to_string(&text);
        assert!(json.is_ok());
    }
}
