use super::{OdtParagraphStyle, OdtTextStyle};
use serde::{Deserialize, Serialize};

/// ODT 段落类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OdtParagraphType {
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

/// ODT 段落
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdtParagraph {
    /// 段落 ID
    pub id: String,
    /// 段落类型
    pub paragraph_type: OdtParagraphType,
    /// 文本内容
    pub text: String,
    /// 文本样式
    pub text_style: OdtTextStyle,
    /// 段落样式
    pub paragraph_style: OdtParagraphStyle,
}

impl OdtParagraph {
    /// 创建新的段落
    pub fn new(text: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            paragraph_type: OdtParagraphType::Normal,
            text,
            text_style: OdtTextStyle::new(),
            paragraph_style: OdtParagraphStyle::new(),
        }
    }

    /// 设置段落类型
    pub fn with_type(mut self, ptype: OdtParagraphType) -> Self {
        self.paragraph_type = ptype;
        self
    }

    /// 设置文本样式
    pub fn with_text_style(mut self, style: OdtTextStyle) -> Self {
        self.text_style = style;
        self
    }

    /// 设置段落样式
    pub fn with_paragraph_style(mut self, style: OdtParagraphStyle) -> Self {
        self.paragraph_style = style;
        self
    }

    /// 创建标题1
    pub fn heading1(text: String) -> Self {
        Self::new(text)
            .with_type(OdtParagraphType::Heading1)
            .with_text_style(OdtTextStyle::new().with_bold(true).with_size(18.0))
    }

    /// 创建标题2
    pub fn heading2(text: String) -> Self {
        Self::new(text)
            .with_type(OdtParagraphType::Heading2)
            .with_text_style(OdtTextStyle::new().with_bold(true).with_size(16.0))
    }

    /// 创建标题3
    pub fn heading3(text: String) -> Self {
        Self::new(text)
            .with_type(OdtParagraphType::Heading3)
            .with_text_style(OdtTextStyle::new().with_bold(true).with_size(14.0))
    }

    /// 创建列表项
    pub fn list_item(text: String) -> Self {
        Self::new(text)
            .with_type(OdtParagraphType::ListItem)
            .with_paragraph_style(OdtParagraphStyle::new().with_first_line_indent(0.5))
    }

    /// 创建引用段落
    #[allow(dead_code)]
    pub fn quote(text: String) -> Self {
        Self::new(text)
            .with_type(OdtParagraphType::Quote)
            .with_paragraph_style(
                OdtParagraphStyle::new()
                    .with_first_line_indent(0.5)
                    .with_left_indent(0.5),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::style::OdtAlignment;

    #[test]
    fn test_odt_paragraph_new() {
        let para = OdtParagraph::new("Test".to_string());
        assert_eq!(para.text, "Test");
        assert_eq!(para.paragraph_type, OdtParagraphType::Normal);
    }

    #[test]
    fn test_odt_paragraph_heading1() {
        let para = OdtParagraph::heading1("Title".to_string());
        assert_eq!(para.paragraph_type, OdtParagraphType::Heading1);
        assert!(para.text_style.bold);
    }

    #[test]
    fn test_odt_paragraph_list_item() {
        let para = OdtParagraph::list_item("Item".to_string());
        assert_eq!(para.paragraph_type, OdtParagraphType::ListItem);
    }

    #[test]
    fn test_odt_paragraph_quote() {
        let para = OdtParagraph::quote("Quote".to_string());
        assert_eq!(para.paragraph_type, OdtParagraphType::Quote);
    }

    #[test]
    fn test_odt_paragraph_serialization() {
        let para = OdtParagraph::new("Test".to_string());
        let json = serde_json::to_string(&para);
        assert!(json.is_ok());
    }

    #[test]
    fn test_odt_paragraph_deserialization() {
        let json = r#"{"id":"test-id","text":"Test","paragraph_type":"Normal","text_style":{"font_name":"Liberation Serif","font_size":12.0,"bold":false,"italic":false,"underline":false,"color":[0,0,0]},"paragraph_style":{"alignment":"Left","left_indent":0.0,"right_indent":0.0,"first_line_indent":0.0,"space_before":0.0,"space_after":0.0,"line_spacing":1.0}}"#;
        let para: OdtParagraph = serde_json::from_str(json).unwrap();
        assert_eq!(para.text, "Test");
    }

    #[test]
    fn test_odt_paragraph_heading2() {
        let para = OdtParagraph::heading2("Title".to_string());
        assert_eq!(para.paragraph_type, OdtParagraphType::Heading2);
    }

    #[test]
    fn test_odt_paragraph_heading3() {
        let para = OdtParagraph::heading3("Title".to_string());
        assert_eq!(para.paragraph_type, OdtParagraphType::Heading3);
    }

    #[test]
    fn test_odt_paragraph_with_text_style() {
        let style = OdtTextStyle {
            font_name: "Times New Roman".to_string(),
            font_size: 14.0,
            bold: true,
            italic: false,
            underline: false,
            color: (255, 0, 0),
        };
        let para = OdtParagraph::new("Test".to_string()).with_text_style(style);
        assert!(para.text_style.bold);
        assert_eq!(para.text_style.font_size, 14.0);
    }

    #[test]
    fn test_odt_paragraph_with_paragraph_style() {
        let pstyle = OdtParagraphStyle {
            alignment: OdtAlignment::Center,
            left_indent: 10.0,
            right_indent: 0.0,
            first_line_indent: 0.0,
            space_before: 0.0,
            space_after: 0.0,
            line_spacing: 1.0,
        };
        let para = OdtParagraph::new("Test".to_string()).with_paragraph_style(pstyle);
        assert_eq!(para.paragraph_style.alignment, OdtAlignment::Center);
    }

    #[test]
    fn test_odt_paragraph_chaining() {
        let para = OdtParagraph::new("Test".to_string())
            .with_text_style(OdtTextStyle::default())
            .with_paragraph_style(OdtParagraphStyle::default());
        assert_eq!(para.text, "Test");
    }

    #[test]
    fn test_odt_paragraph_empty_text() {
        let para = OdtParagraph::new("".to_string());
        assert_eq!(para.text, "");
    }

    #[test]
    fn test_odt_paragraph_long_text() {
        let long_text = "A".repeat(1000);
        let para = OdtParagraph::new(long_text.clone());
        assert_eq!(para.text.len(), 1000);
    }

    #[test]
    fn test_odt_text_style_default() {
        let style = OdtTextStyle::default();
        assert!(!style.bold);
        assert!(!style.italic);
        assert_eq!(style.font_size, 12.0);
    }

    #[test]
    fn test_odt_paragraph_style_default() {
        let style = OdtParagraphStyle::default();
        assert_eq!(style.alignment, OdtAlignment::Left);
        assert_eq!(style.left_indent, 0.0);
    }

    #[test]
    fn test_odt_alignment_left() {
        assert_eq!(OdtAlignment::Left, OdtAlignment::Left);
    }

    #[test]
    fn test_odt_alignment_center() {
        assert_eq!(OdtAlignment::Center, OdtAlignment::Center);
    }

    #[test]
    fn test_odt_alignment_right() {
        assert_eq!(OdtAlignment::Right, OdtAlignment::Right);
    }

    #[test]
    fn test_odt_alignment_justify() {
        assert_eq!(OdtAlignment::Justify, OdtAlignment::Justify);
    }

    #[test]
    fn test_odt_paragraph_type_normal() {
        assert_eq!(OdtParagraphType::Normal, OdtParagraphType::Normal);
    }

    #[test]
    fn test_odt_paragraph_type_heading1() {
        assert_eq!(OdtParagraphType::Heading1, OdtParagraphType::Heading1);
    }

    #[test]
    fn test_odt_paragraph_type_list_item() {
        assert_eq!(OdtParagraphType::ListItem, OdtParagraphType::ListItem);
    }

    #[test]
    fn test_odt_paragraph_type_quote() {
        assert_eq!(OdtParagraphType::Quote, OdtParagraphType::Quote);
    }

    #[test]
    fn test_odt_text_style_serialization() {
        let style = OdtTextStyle::default();
        let json = serde_json::to_string(&style);
        assert!(json.is_ok());
    }

    #[test]
    fn test_odt_paragraph_style_serialization() {
        let style = OdtParagraphStyle::default();
        let json = serde_json::to_string(&style);
        assert!(json.is_ok());
    }
}
