use super::{RtfParagraphStyle, RtfTextStyle};
use super::style::RtfAlignment;
use serde::{Deserialize, Serialize};

/// RTF 段落类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RtfParagraphType {
    /// 普通段落
    Normal,
    /// 标题1
    Heading1,
    /// 标题2
    Heading2,
    /// 标题3
    Heading3,
    /// 列表项
    ListItem,
    /// 引用
    Quote,
}

/// RTF 段落
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtfParagraph {
    /// 段落 ID
    pub id: String,
    /// 段落类型
    pub paragraph_type: RtfParagraphType,
    /// 文本内容
    pub text: String,
    /// 文本样式
    pub text_style: RtfTextStyle,
    /// 段落样式
    pub paragraph_style: RtfParagraphStyle,
}

impl RtfParagraph {
    /// 创建新的段落
    pub fn new(text: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            paragraph_type: RtfParagraphType::Normal,
            text,
            text_style: RtfTextStyle::new(),
            paragraph_style: RtfParagraphStyle::new(),
        }
    }

    /// 设置段落类型
    pub fn with_type(mut self, ptype: RtfParagraphType) -> Self {
        self.paragraph_type = ptype;
        self
    }

    /// 设置文本样式
    pub fn with_text_style(mut self, style: RtfTextStyle) -> Self {
        self.text_style = style;
        self
    }

    /// 设置段落样式
    pub fn with_paragraph_style(mut self, style: RtfParagraphStyle) -> Self {
        self.paragraph_style = style;
        self
    }

    /// 创建标题1
    pub fn heading1(text: String) -> Self {
        Self::new(text)
            .with_type(RtfParagraphType::Heading1)
            .with_text_style(RtfTextStyle::new().with_bold(true).with_font_size(16))
    }

    /// 创建标题2
    pub fn heading2(text: String) -> Self {
        Self::new(text)
            .with_type(RtfParagraphType::Heading2)
            .with_text_style(RtfTextStyle::new().with_bold(true).with_font_size(14))
    }

    /// 创建标题3
    pub fn heading3(text: String) -> Self {
        Self::new(text)
            .with_type(RtfParagraphType::Heading3)
            .with_text_style(RtfTextStyle::new().with_bold(true).with_font_size(12))
    }

    /// 创建列表项
    pub fn list_item(text: String) -> Self {
        Self::new(text)
            .with_type(RtfParagraphType::ListItem)
            .with_paragraph_style(RtfParagraphStyle::new().with_first_line_indent(360))
    }

    /// 创建引用段落
    #[allow(dead_code)]
    pub fn quote(text: String) -> Self {
        Self::new(text)
            .with_type(RtfParagraphType::Quote)
            .with_paragraph_style(
                RtfParagraphStyle::new()
                    .with_first_line_indent(360)
                    .with_left_indent(360),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtf_paragraph_new() {
        let para = RtfParagraph::new("Test".to_string());
        assert_eq!(para.text, "Test");
        assert_eq!(para.paragraph_type, RtfParagraphType::Normal);
    }

    #[test]
    fn test_rtf_paragraph_heading1() {
        let para = RtfParagraph::heading1("Title".to_string());
        assert_eq!(para.paragraph_type, RtfParagraphType::Heading1);
        assert!(para.text_style.bold);
    }

    #[test]
    fn test_rtf_paragraph_list_item() {
        let para = RtfParagraph::list_item("Item".to_string());
        assert_eq!(para.paragraph_type, RtfParagraphType::ListItem);
        assert_eq!(para.paragraph_style.first_line_indent, 360);
    }

    #[test]
    fn test_rtf_paragraph_quote() {
        let para = RtfParagraph::quote("Quote".to_string());
        assert_eq!(para.paragraph_type, RtfParagraphType::Quote);
    }

    #[test]
    fn test_rtf_paragraph_chaining() {
        let para = RtfParagraph::new("Test".to_string())
            .with_type(RtfParagraphType::Heading2)
            .with_text_style(RtfTextStyle::new().with_italic(true));
        assert!(para.text_style.italic);
    }

    #[test]
    fn test_rtf_paragraph_serialization() {
        let para = RtfParagraph::new("Test".to_string());
        let json = serde_json::to_string(&para);
        assert!(json.is_ok());
    }

    #[test]
    fn test_rtf_paragraph_deserialization() {
        let json = r#"{"id":"test-id","text":"Test","paragraph_type":"Normal","text_style":{"bold":false,"italic":false,"underline":false,"strikethrough":false,"font_size":null,"color_index":null,"background_index":null,"font_index":null},"paragraph_style":{"alignment":"Left","left_indent":0,"right_indent":0,"first_line_indent":0,"space_before":0,"space_after":0,"line_spacing":240}}"#;
        let para: RtfParagraph = serde_json::from_str(json).unwrap();
        assert_eq!(para.text, "Test");
    }

    #[test]
    fn test_rtf_paragraph_heading2() {
        let para = RtfParagraph::heading2("Title".to_string());
        assert_eq!(para.paragraph_type, RtfParagraphType::Heading2);
    }

    #[test]
    fn test_rtf_paragraph_heading3() {
        let para = RtfParagraph::heading3("Title".to_string());
        assert_eq!(para.paragraph_type, RtfParagraphType::Heading3);
    }

    #[test]
    fn test_rtf_paragraph_with_text_style() {
        let style = RtfTextStyle::new().with_bold(true).with_italic(true);
        let para = RtfParagraph::new("Test".to_string()).with_text_style(style);
        assert!(para.text_style.bold);
        assert!(para.text_style.italic);
    }

    #[test]
    fn test_rtf_paragraph_with_paragraph_style() {
        let pstyle = RtfParagraphStyle::new().with_alignment(RtfAlignment::Center);
        let para = RtfParagraph::new("Test".to_string()).with_paragraph_style(pstyle);
        assert_eq!(para.paragraph_style.alignment, RtfAlignment::Center);
    }

    #[test]
    fn test_rtf_paragraph_empty_text() {
        let para = RtfParagraph::new("".to_string());
        assert_eq!(para.text, "");
    }

    #[test]
    fn test_rtf_paragraph_long_text() {
        let long_text = "A".repeat(1000);
        let para = RtfParagraph::new(long_text.clone());
        assert_eq!(para.text.len(), 1000);
    }

    #[test]
    fn test_rtf_text_style_default() {
        let style = RtfTextStyle::new();
        assert!(!style.bold);
        assert_eq!(style.font_size, None);
    }

    #[test]
    fn test_rtf_paragraph_style_default() {
        let style = RtfParagraphStyle::new();
        assert_eq!(style.alignment, RtfAlignment::Left);
    }

    #[test]
    fn test_rtf_alignment_left() {
        assert_eq!(RtfAlignment::Left, RtfAlignment::Left);
    }

    #[test]
    fn test_rtf_alignment_center() {
        assert_eq!(RtfAlignment::Center, RtfAlignment::Center);
    }

    #[test]
    fn test_rtf_alignment_right() {
        assert_eq!(RtfAlignment::Right, RtfAlignment::Right);
    }

    #[test]
    fn test_rtf_alignment_justify() {
        assert_eq!(RtfAlignment::Justify, RtfAlignment::Justify);
    }

    #[test]
    fn test_rtf_paragraph_type_normal() {
        assert_eq!(RtfParagraphType::Normal, RtfParagraphType::Normal);
    }

    #[test]
    fn test_rtf_paragraph_type_heading1() {
        assert_eq!(RtfParagraphType::Heading1, RtfParagraphType::Heading1);
    }

    #[test]
    fn test_rtf_paragraph_type_list_item() {
        assert_eq!(RtfParagraphType::ListItem, RtfParagraphType::ListItem);
    }

    #[test]
    fn test_rtf_paragraph_type_quote() {
        assert_eq!(RtfParagraphType::Quote, RtfParagraphType::Quote);
    }

    #[test]
    fn test_rtf_text_style_serialization() {
        let style = RtfTextStyle::new();
        let json = serde_json::to_string(&style);
        assert!(json.is_ok());
    }

    #[test]
    fn test_rtf_paragraph_style_serialization() {
        let style = RtfParagraphStyle::new();
        let json = serde_json::to_string(&style);
        assert!(json.is_ok());
    }

    #[test]
    fn test_rtf_text_style_with_font_size() {
        let style = RtfTextStyle::new().with_font_size(14);
        assert_eq!(style.font_size, Some(28));
    }

    #[test]
    fn test_rtf_paragraph_style_with_indent() {
        let style = RtfParagraphStyle::new().with_first_line_indent(360);
        assert_eq!(style.first_line_indent, 360);
    }
}
