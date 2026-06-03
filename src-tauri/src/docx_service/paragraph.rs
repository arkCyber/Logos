use super::{ParagraphStyle, TextStyle};
use serde::{Deserialize, Serialize};

/// 列表类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ListType {
    /// 无序列表（项目符号）
    Bullet,
    /// 有序列表（数字）
    Numbered,
    /// 字母列表
    Lettered,
    /// 罗马数字
    Roman,
}

/// 段落类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParagraphType {
    /// 普通段落
    Normal,
    /// 标题1
    Heading1,
    /// 标题2
    Heading2,
    /// 标题3
    Heading3,
    /// 标题4
    Heading4,
    /// 标题5
    Heading5,
    /// 标题6
    Heading6,
    /// 列表项
    ListItem,
    /// 引用
    Quote,
    /// 代码
    Code,
    /// 自定义
    Custom(String),
}

/// 段落
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paragraph {
    /// 段落 ID
    pub id: String,
    /// 段落类型
    pub paragraph_type: ParagraphType,
    /// 文本内容
    pub text: String,
    /// 文本样式
    pub text_style: TextStyle,
    /// 段落样式
    pub paragraph_style: ParagraphStyle,
    /// 列表类型（如果是列表项）
    pub list_type: Option<ListType>,
    /// 列表级别（缩进级别）
    pub list_level: usize,
}

impl Paragraph {
    /// 创建新的段落
    pub fn new(text: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            paragraph_type: ParagraphType::Normal,
            text,
            text_style: TextStyle::new(),
            paragraph_style: ParagraphStyle::new(),
            list_type: None,
            list_level: 0,
        }
    }

    /// 设置段落类型
    pub fn with_type(mut self, ptype: ParagraphType) -> Self {
        self.paragraph_type = ptype;
        self
    }

    /// 设置文本样式
    pub fn with_text_style(mut self, style: TextStyle) -> Self {
        self.text_style = style;
        self
    }

    /// 设置段落样式
    #[allow(dead_code)]
    pub fn with_paragraph_style(mut self, style: ParagraphStyle) -> Self {
        self.paragraph_style = style;
        self
    }

    /// 设置列表类型
    #[allow(dead_code)]
    pub fn with_list_type(mut self, list_type: ListType) -> Self {
        self.list_type = Some(list_type);
        self.paragraph_type = ParagraphType::ListItem;
        self
    }

    /// 设置列表级别
    #[allow(dead_code)]
    pub fn with_list_level(mut self, level: usize) -> Self {
        self.list_level = level;
        self
    }

    /// 创建标题1
    pub fn heading1(text: String) -> Self {
        Self::new(text)
            .with_type(ParagraphType::Heading1)
            .with_text_style(TextStyle::heading1())
    }

    /// 创建标题2
    pub fn heading2(text: String) -> Self {
        Self::new(text)
            .with_type(ParagraphType::Heading2)
            .with_text_style(TextStyle::heading2())
    }

    /// 创建标题3
    pub fn heading3(text: String) -> Self {
        Self::new(text)
            .with_type(ParagraphType::Heading3)
            .with_text_style(TextStyle::heading3())
    }

    /// 创建项目符号列表项
    pub fn bullet_item(text: String) -> Self {
        Self::new(text).with_list_type(ListType::Bullet)
    }

    /// 创建编号列表项
    #[allow(dead_code)]
    pub fn numbered_item(text: String) -> Self {
        Self::new(text).with_list_type(ListType::Numbered)
    }

    /// 创建引用段落
    #[allow(dead_code)]
    pub fn quote(text: String) -> Self {
        Self::new(text)
            .with_type(ParagraphType::Quote)
            .with_paragraph_style(ParagraphStyle::quote())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::style::{FontStyle, Alignment, Spacing};

    #[test]
    fn test_paragraph_new() {
        let para = Paragraph::new("Test".to_string());
        assert_eq!(para.text, "Test");
        assert_eq!(para.paragraph_type, ParagraphType::Normal);
    }

    #[test]
    fn test_paragraph_with_type() {
        let para = Paragraph::new("Test".to_string()).with_type(ParagraphType::Heading1);
        assert_eq!(para.paragraph_type, ParagraphType::Heading1);
    }

    #[test]
    fn test_paragraph_heading1() {
        let para = Paragraph::heading1("Title".to_string());
        assert_eq!(para.paragraph_type, ParagraphType::Heading1);
        assert_eq!(para.text_style.font_size, 16.0);
    }

    #[test]
    fn test_paragraph_bullet_item() {
        let para = Paragraph::bullet_item("Item".to_string());
        assert_eq!(para.list_type, Some(ListType::Bullet));
    }

    #[test]
    fn test_paragraph_numbered_item() {
        let para = Paragraph::numbered_item("Item".to_string());
        assert_eq!(para.list_type, Some(ListType::Numbered));
    }

    #[test]
    fn test_paragraph_quote() {
        let para = Paragraph::quote("Quote".to_string());
        assert_eq!(para.paragraph_type, ParagraphType::Quote);
    }

    #[test]
    fn test_paragraph_chaining() {
        let para = Paragraph::new("Test".to_string())
            .with_type(ParagraphType::Heading2)
            .with_list_level(2);
        assert_eq!(para.list_level, 2);
    }

    #[test]
    fn test_paragraph_serialization() {
        let para = Paragraph::new("Test".to_string());
        let json = serde_json::to_string(&para);
        assert!(json.is_ok());
    }

    #[test]
    fn test_paragraph_with_style() {
        let para = Paragraph::new("Test".to_string())
            .with_text_style(TextStyle {
                font_name: "Arial".to_string(),
                font_size: 14.0,
                font_style: FontStyle::Regular,
                color: (0, 0, 0),
                highlight: None,
                underline: false,
                strikethrough: false,
                superscript: false,
                subscript: false,
            });
        assert_eq!(para.text_style.font_size, 14.0);
    }

    #[test]
    fn test_paragraph_with_spacing() {
        let para = Paragraph::new("Test".to_string())
            .with_paragraph_style(ParagraphStyle {
                alignment: Alignment::Left,
                first_line_indent: 0.0,
                left_indent: 0.0,
                right_indent: 0.0,
                spacing: Spacing {
                    before: 12.0,
                    after: 12.0,
                    line: 1.5,
                },
                keep_lines: false,
                keep_with_next: false,
            });
        assert_eq!(para.paragraph_style.spacing.before, 12.0);
    }

    #[test]
    fn test_paragraph_alignment() {
        let para = Paragraph::new("Test".to_string())
            .with_paragraph_style(ParagraphStyle {
                alignment: Alignment::Center,
                first_line_indent: 0.0,
                left_indent: 0.0,
                right_indent: 0.0,
                spacing: Spacing::standard(),
                keep_lines: false,
                keep_with_next: false,
            });
        assert_eq!(para.paragraph_style.alignment, Alignment::Center);
    }

    #[test]
    fn test_paragraph_empty_text() {
        let para = Paragraph::new("".to_string());
        assert_eq!(para.text, "");
    }

    #[test]
    fn test_paragraph_long_text() {
        let long_text = "A".repeat(1000);
        let para = Paragraph::new(long_text.clone());
        assert_eq!(para.text.len(), 1000);
    }

    #[test]
    fn test_paragraph_heading2() {
        let para = Paragraph::heading2("Subtitle".to_string());
        assert_eq!(para.paragraph_type, ParagraphType::Heading2);
        assert_eq!(para.text_style.font_size, 14.0);
    }

    #[test]
    fn test_paragraph_heading3() {
        let para = Paragraph::heading3("Section".to_string());
        assert_eq!(para.paragraph_type, ParagraphType::Heading3);
        assert_eq!(para.text_style.font_size, 12.0);
    }

    #[test]
    fn test_paragraph_deserialization() {
        let json = r#"{"id":"test-id","paragraph_type":"Normal","text":"Test","text_style":{"font_name":"Calibri","font_size":11.0,"font_style":"Regular","color":[0,0,0],"highlight":null,"underline":false,"strikethrough":false,"superscript":false,"subscript":false},"paragraph_style":{"alignment":"Left","first_line_indent":0.0,"left_indent":0.0,"right_indent":0.0,"spacing":{"before":0.0,"after":12.0,"line":1.15},"keep_lines":false,"keep_with_next":false},"list_type":null,"list_level":0}"#;
        let para: Paragraph = serde_json::from_str(json).unwrap();
        assert_eq!(para.text, "Test");
    }

    #[test]
    fn test_paragraph_type_default() {
        let para = Paragraph::new("Test".to_string());
        assert_eq!(para.paragraph_type, ParagraphType::Normal);
    }

    #[test]
    fn test_paragraph_list_level_default() {
        let para = Paragraph::new("Test".to_string());
        assert_eq!(para.list_level, 0);
    }

    #[test]
    fn test_paragraph_alignment_default() {
        let para = Paragraph::new("Test".to_string());
        assert_eq!(para.paragraph_style.alignment, Alignment::Left);
    }

    #[test]
    fn test_paragraph_with_list_type_bullet() {
        let para = Paragraph::new("Test".to_string())
            .with_list_type(ListType::Bullet);
        assert_eq!(para.list_type, Some(ListType::Bullet));
    }

    #[test]
    fn test_paragraph_with_list_type_numbered() {
        let para = Paragraph::new("Test".to_string())
            .with_list_type(ListType::Numbered);
        assert_eq!(para.list_type, Some(ListType::Numbered));
    }

    #[test]
    fn test_paragraph_with_text_style_underline() {
        let para = Paragraph::new("Test".to_string())
            .with_text_style(TextStyle {
                font_name: "Arial".to_string(),
                font_size: 11.0,
                font_style: FontStyle::Regular,
                color: (0, 0, 0),
                highlight: None,
                underline: true,
                strikethrough: false,
                superscript: false,
                subscript: false,
            });
        assert!(para.text_style.underline);
    }

    #[test]
    fn test_paragraph_with_text_style_color() {
        let para = Paragraph::new("Test".to_string())
            .with_text_style(TextStyle {
                font_name: "Arial".to_string(),
                font_size: 11.0,
                font_style: FontStyle::Regular,
                color: (255, 0, 0),
                highlight: None,
                underline: false,
                strikethrough: false,
                superscript: false,
                subscript: false,
            });
        assert_eq!(para.text_style.color, (255, 0, 0));
    }

    #[test]
    fn test_paragraph_with_text_style_bold() {
        let para = Paragraph::new("Test".to_string())
            .with_text_style(TextStyle {
                font_name: "Arial".to_string(),
                font_size: 11.0,
                font_style: FontStyle::Bold,
                color: (0, 0, 0),
                highlight: None,
                underline: false,
                strikethrough: false,
                superscript: false,
                subscript: false,
            });
        assert_eq!(para.text_style.font_style, FontStyle::Bold);
    }

    #[test]
    fn test_paragraph_with_text_style_italic() {
        let para = Paragraph::new("Test".to_string())
            .with_text_style(TextStyle {
                font_name: "Arial".to_string(),
                font_size: 11.0,
                font_style: FontStyle::Italic,
                color: (0, 0, 0),
                highlight: None,
                underline: false,
                strikethrough: false,
                superscript: false,
                subscript: false,
            });
        assert_eq!(para.text_style.font_style, FontStyle::Italic);
    }
}
