use serde::{Deserialize, Serialize};

/// 页眉页脚类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HeaderFooterType {
    /// 默认
    Default,
    /// 首页不同
    First,
    /// 奇偶页不同
    EvenOdd,
}

/// 页眉
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    /// 页眉 ID
    pub id: String,
    /// 页眉类型
    pub header_type: HeaderFooterType,
    /// 页眉文本
    pub text: String,
    /// 是否显示页码
    pub show_page_number: bool,
    /// 对齐方式
    pub alignment: String,
}

impl Header {
    /// 创建新的页眉
    #[allow(dead_code)]
    pub fn new(text: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            header_type: HeaderFooterType::Default,
            text,
            show_page_number: false,
            alignment: "center".to_string(),
        }
    }

    /// 设置页眉类型
    #[allow(dead_code)]
    pub fn with_type(mut self, htype: HeaderFooterType) -> Self {
        self.header_type = htype;
        self
    }

    /// 设置是否显示页码
    #[allow(dead_code)]
    pub fn with_page_number(mut self, show: bool) -> Self {
        self.show_page_number = show;
        self
    }

    /// 设置对齐
    #[allow(dead_code)]
    pub fn with_alignment(mut self, alignment: String) -> Self {
        self.alignment = alignment;
        self
    }

    /// 创建带页码的页眉
    #[allow(dead_code)]
    pub fn with_page_number_only() -> Self {
        Self::new(String::new()).with_page_number(true)
    }
}

/// 页脚
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Footer {
    /// 页脚 ID
    pub id: String,
    /// 页脚类型
    pub footer_type: HeaderFooterType,
    /// 页脚文本
    pub text: String,
    /// 是否显示页码
    pub show_page_number: bool,
    /// 对齐方式
    pub alignment: String,
}

impl Footer {
    /// 创建新的页脚
    #[allow(dead_code)]
    pub fn new(text: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            footer_type: HeaderFooterType::Default,
            text,
            show_page_number: false,
            alignment: "center".to_string(),
        }
    }

    /// 设置页脚类型
    #[allow(dead_code)]
    pub fn with_type(mut self, ftype: HeaderFooterType) -> Self {
        self.footer_type = ftype;
        self
    }

    /// 设置是否显示页码
    #[allow(dead_code)]
    pub fn with_page_number(mut self, show: bool) -> Self {
        self.show_page_number = show;
        self
    }

    /// 设置对齐
    #[allow(dead_code)]
    pub fn with_alignment(mut self, alignment: String) -> Self {
        self.alignment = alignment;
        self
    }

    /// 创建带页码的页脚
    #[allow(dead_code)]
    pub fn with_page_number_only() -> Self {
        Self::new(String::new()).with_page_number(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_new() {
        let header = Header::new("Test Header".to_string());
        assert_eq!(header.text, "Test Header");
        assert_eq!(header.header_type, HeaderFooterType::Default);
    }

    #[test]
    fn test_header_with_type() {
        let header = Header::new("Test".to_string()).with_type(HeaderFooterType::First);
        assert_eq!(header.header_type, HeaderFooterType::First);
    }

    #[test]
    fn test_header_with_page_number() {
        let header = Header::new("Test".to_string()).with_page_number(true);
        assert!(header.show_page_number);
    }

    #[test]
    fn test_header_with_page_number_only() {
        let header = Header::with_page_number_only();
        assert!(header.show_page_number);
        assert!(header.text.is_empty());
    }

    #[test]
    fn test_footer_new() {
        let footer = Footer::new("Test Footer".to_string());
        assert_eq!(footer.text, "Test Footer");
    }

    #[test]
    fn test_footer_with_page_number() {
        let footer = Footer::new("Test".to_string()).with_page_number(true);
        assert!(footer.show_page_number);
    }

    #[test]
    fn test_footer_with_page_number_only() {
        let footer = Footer::with_page_number_only();
        assert!(footer.show_page_number);
    }

    #[test]
    fn test_header_serialization() {
        let header = Header::new("Test".to_string());
        let json = serde_json::to_string(&header);
        assert!(json.is_ok());
    }

    #[test]
    fn test_footer_serialization() {
        let footer = Footer::new("Test".to_string());
        let json = serde_json::to_string(&footer);
        assert!(json.is_ok());
    }
}
