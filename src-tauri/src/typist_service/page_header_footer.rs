/*!
 * 航空航天级页眉页脚系统
 * 实现 Typst 的页眉页脚功能（页眉内容、页脚内容、页码、分节支持）
 */

use serde::{Deserialize, Serialize};

/// 页码对齐方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PageNumberAlign {
    Left,
    Center,
    Right,
}

/// 页码样式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PageNumberStyle {
    /// 数字 (1, 2, 3)
    Numeric,
    /// 罗马数字 (i, ii, iii)
    Roman,
    /// 字母 (a, b, c)
    Letter,
    /// 自定义
    Custom(String),
}

/// 页眉页脚内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HeaderFooterContent {
    Text(String),
    PageNumber,
    PageCount,
    SectionTitle,
    Author,
    Date,
    Custom(String),
}

/// 页眉配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderConfig {
    pub content: Option<HeaderFooterContent>,
    pub ascent: f64,
    pub show_on_first: bool,
    pub show_on_odd: bool,
    pub show_on_even: bool,
}

impl Default for HeaderConfig {
    fn default() -> Self {
        Self {
            content: None,
            ascent: 0.5,
            show_on_first: true,
            show_on_odd: true,
            show_on_even: true,
        }
    }
}

/// 页脚配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooterConfig {
    pub content: Option<HeaderFooterContent>,
    pub descent: f64,
    pub show_on_first: bool,
    pub show_on_odd: bool,
    pub show_on_even: bool,
}

impl Default for FooterConfig {
    fn default() -> Self {
        Self {
            content: None,
            descent: 0.5,
            show_on_first: true,
            show_on_odd: true,
            show_on_even: true,
        }
    }
}

/// 页码配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageNumberConfig {
    pub style: PageNumberStyle,
    pub align: PageNumberAlign,
    pub supplement: Option<String>,
}

impl Default for PageNumberConfig {
    fn default() -> Self {
        Self {
            style: PageNumberStyle::Numeric,
            align: PageNumberAlign::Center,
            supplement: None,
        }
    }
}

/// 页眉页脚系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageHeaderFooter {
    pub header: HeaderConfig,
    pub footer: FooterConfig,
    pub page_number: PageNumberConfig,
    pub section_title: Option<String>,
    pub author: Option<String>,
}

impl PageHeaderFooter {
    pub fn new() -> Self {
        Self {
            header: HeaderConfig::default(),
            footer: FooterConfig::default(),
            page_number: PageNumberConfig::default(),
            section_title: None,
            author: None,
        }
    }

    pub fn with_header(mut self, header: HeaderConfig) -> Self {
        self.header = header;
        self
    }

    pub fn with_footer(mut self, footer: FooterConfig) -> Self {
        self.footer = footer;
        self
    }

    pub fn with_page_number(mut self, page_number: PageNumberConfig) -> Self {
        self.page_number = page_number;
        self
    }

    pub fn with_section_title(mut self, title: String) -> Self {
        self.section_title = Some(title);
        self
    }

    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        typst.push_str("#set page(\n");

        // 添加页眉
        if let Some(content) = &self.header.content {
            typst.push_str("  header: ");
            typst.push_str(&self.content_to_typst(content));
            typst.push_str(",\n");

            if self.header.ascent != 0.5 {
                typst.push_str(&format!("  header-ascent: {}em,\n", self.header.ascent));
            }
        }

        // 添加页脚
        if let Some(content) = &self.footer.content {
            typst.push_str("  footer: ");
            typst.push_str(&self.content_to_typst(content));
            typst.push_str(",\n");

            if self.footer.descent != 0.5 {
                typst.push_str(&format!("  footer-descent: {}em,\n", self.footer.descent));
            }
        }

        // 添加页码
        typst.push_str("  numbering: \"");
        typst.push_str(&self.page_number_style_to_typst());
        typst.push_str("\",\n");

        typst.push_str(&format!("  number-align: {},\n", self.align_to_typst()));

        if let Some(supplement) = &self.page_number.supplement {
            typst.push_str(&format!("  supplement: [{}],\n", supplement));
        }

        typst.push_str(")\n");

        typst
    }

    fn content_to_typst(&self, content: &HeaderFooterContent) -> String {
        match content {
            HeaderFooterContent::Text(text) => format!("[{}]", html_escape(text)),
            HeaderFooterContent::PageNumber => "counter(page)".to_string(),
            HeaderFooterContent::PageCount => "counter(page).final(1)".to_string(),
            HeaderFooterContent::SectionTitle => {
                if let Some(title) = &self.section_title {
                    format!("[{}]", html_escape(title))
                } else {
                    "[Section Title]".to_string()
                }
            }
            HeaderFooterContent::Author => {
                if let Some(author) = &self.author {
                    format!("[{}]", html_escape(author))
                } else {
                    "[Author]".to_string()
                }
            }
            HeaderFooterContent::Date => "[datetime.today()]".to_string(),
            HeaderFooterContent::Custom(custom) => format!("[{}]", html_escape(custom)),
        }
    }

    fn page_number_style_to_typst(&self) -> String {
        match self.page_number.style {
            PageNumberStyle::Numeric => "1".to_string(),
            PageNumberStyle::Roman => "i".to_string(),
            PageNumberStyle::Letter => "a".to_string(),
            PageNumberStyle::Custom(ref style) => style.clone(),
        }
    }

    fn align_to_typst(&self) -> String {
        match self.page_number.align {
            PageNumberAlign::Left => "left".to_string(),
            PageNumberAlign::Center => "center".to_string(),
            PageNumberAlign::Right => "right".to_string(),
        }
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        html.push_str("<div class=\"typst-page-header-footer\">\n");

        // 页眉
        if let Some(content) = &self.header.content {
            html.push_str("  <div class=\"page-header\">\n");
            html.push_str(&format!("    {}\n", self.content_to_html(content)));
            html.push_str("  </div>\n");
        }

        // 页脚
        if let Some(content) = &self.footer.content {
            html.push_str("  <div class=\"page-footer\">\n");
            html.push_str(&format!("    {}\n", self.content_to_html(content)));
            html.push_str("  </div>\n");
        }

        html.push_str("</div>\n");

        html
    }

    fn content_to_html(&self, content: &HeaderFooterContent) -> String {
        match content {
            HeaderFooterContent::Text(text) => format!(
                "<span class=\"header-footer-text\">{}</span>",
                html_escape(text)
            ),
            HeaderFooterContent::PageNumber => {
                "<span class=\"page-number\">{{page}}</span>".to_string()
            }
            HeaderFooterContent::PageCount => {
                "<span class=\"page-count\">{{total}}</span>".to_string()
            }
            HeaderFooterContent::SectionTitle => {
                if let Some(title) = &self.section_title {
                    format!(
                        "<span class=\"section-title\">{}</span>",
                        html_escape(title)
                    )
                } else {
                    "<span class=\"section-title\">Section Title</span>".to_string()
                }
            }
            HeaderFooterContent::Author => {
                if let Some(author) = &self.author {
                    format!("<span class=\"author\">{}</span>", html_escape(author))
                } else {
                    "<span class=\"author\">Author</span>".to_string()
                }
            }
            HeaderFooterContent::Date => "<span class=\"date\">{{date}}</span>".to_string(),
            HeaderFooterContent::Custom(custom) => {
                format!("<span class=\"custom\">{}</span>", html_escape(custom))
            }
        }
    }
}

impl Default for PageHeaderFooter {
    fn default() -> Self {
        Self::new()
    }
}

/// 页眉页脚构建器
pub struct PageHeaderFooterBuilder {
    system: PageHeaderFooter,
}

impl PageHeaderFooterBuilder {
    pub fn new() -> Self {
        Self {
            system: PageHeaderFooter::new(),
        }
    }

    pub fn with_header(mut self, header: HeaderConfig) -> Self {
        self.system = self.system.with_header(header);
        self
    }

    pub fn with_footer(mut self, footer: FooterConfig) -> Self {
        self.system = self.system.with_footer(footer);
        self
    }

    pub fn with_page_number(mut self, page_number: PageNumberConfig) -> Self {
        self.system = self.system.with_page_number(page_number);
        self
    }

    pub fn with_section_title(mut self, title: String) -> Self {
        self.system = self.system.with_section_title(title);
        self
    }

    pub fn with_author(mut self, author: String) -> Self {
        self.system = self.system.with_author(author);
        self
    }

    pub fn build(self) -> PageHeaderFooter {
        self.system
    }
}

impl Default for PageHeaderFooterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_header_footer_creation() {
        let system = PageHeaderFooter::new();
        assert!(system.header.content.is_none());
        assert!(system.footer.content.is_none());
    }

    #[test]
    fn test_page_header_footer_default() {
        let system = PageHeaderFooter::default();
        assert!(system.header.content.is_none());
        assert_eq!(system.page_number.style, PageNumberStyle::Numeric);
    }

    #[test]
    fn test_header_config_default() {
        let config = HeaderConfig::default();
        assert!(config.content.is_none());
        assert_eq!(config.ascent, 0.5);
    }

    #[test]
    fn test_footer_config_default() {
        let config = FooterConfig::default();
        assert!(config.content.is_none());
        assert_eq!(config.descent, 0.5);
    }

    #[test]
    fn test_page_number_config_default() {
        let config = PageNumberConfig::default();
        assert_eq!(config.style, PageNumberStyle::Numeric);
        assert_eq!(config.align, PageNumberAlign::Center);
    }

    #[test]
    fn test_page_header_footer_with_header() {
        let header = HeaderConfig {
            content: Some(HeaderFooterContent::Text("Test Header".to_string())),
            ascent: 0.5,
            show_on_first: true,
            show_on_odd: true,
            show_on_even: true,
        };
        let system = PageHeaderFooter::new().with_header(header);
        assert!(system.header.content.is_some());
    }

    #[test]
    fn test_page_header_footer_with_footer() {
        let footer = FooterConfig {
            content: Some(HeaderFooterContent::PageNumber),
            descent: 0.5,
            show_on_first: true,
            show_on_odd: true,
            show_on_even: true,
        };
        let system = PageHeaderFooter::new().with_footer(footer);
        assert!(system.footer.content.is_some());
    }

    #[test]
    fn test_page_header_footer_with_section_title() {
        let system = PageHeaderFooter::new().with_section_title("Chapter 1".to_string());
        assert_eq!(system.section_title, Some("Chapter 1".to_string()));
    }

    #[test]
    fn test_page_header_footer_with_author() {
        let system = PageHeaderFooter::new().with_author("John Doe".to_string());
        assert_eq!(system.author, Some("John Doe".to_string()));
    }

    #[test]
    fn test_to_typst() {
        let system = PageHeaderFooter::new();
        let typst = system.to_typst();
        assert!(typst.contains("#set page("));
        assert!(typst.contains("numbering:"));
    }

    #[test]
    fn test_to_typst_with_header() {
        let header = HeaderConfig {
            content: Some(HeaderFooterContent::Text("Test".to_string())),
            ascent: 0.5,
            show_on_first: true,
            show_on_odd: true,
            show_on_even: true,
        };
        let system = PageHeaderFooter::new().with_header(header);
        let typst = system.to_typst();
        assert!(typst.contains("header:"));
    }

    #[test]
    fn test_to_html() {
        let system = PageHeaderFooter::new();
        let html = system.to_html();
        assert!(html.contains("<div class=\"typst-page-header-footer\""));
    }

    #[test]
    fn test_to_html_with_header() {
        let header = HeaderConfig {
            content: Some(HeaderFooterContent::Text("Test".to_string())),
            ascent: 0.5,
            show_on_first: true,
            show_on_odd: true,
            show_on_even: true,
        };
        let system = PageHeaderFooter::new().with_header(header);
        let html = system.to_html();
        assert!(html.contains("<div class=\"page-header\""));
    }

    #[test]
    fn test_page_number_align_partial_eq() {
        assert_eq!(PageNumberAlign::Left, PageNumberAlign::Left);
        assert_ne!(PageNumberAlign::Left, PageNumberAlign::Center);
    }

    #[test]
    fn test_page_number_style_partial_eq() {
        assert_eq!(PageNumberStyle::Numeric, PageNumberStyle::Numeric);
        assert_ne!(PageNumberStyle::Numeric, PageNumberStyle::Roman);
    }

    #[test]
    fn test_page_header_footer_builder() {
        let system = PageHeaderFooterBuilder::new()
            .with_section_title("Chapter 1".to_string())
            .with_author("John Doe".to_string())
            .build();

        assert_eq!(system.section_title, Some("Chapter 1".to_string()));
        assert_eq!(system.author, Some("John Doe".to_string()));
    }

    #[test]
    fn test_page_header_footer_builder_default() {
        let builder = PageHeaderFooterBuilder::default();
        let system = builder.build();
        assert!(system.header.content.is_none());
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_header_footer_content_variants() {
        let text = HeaderFooterContent::Text("Test".to_string());
        let page_num = HeaderFooterContent::PageNumber;
        let page_count = HeaderFooterContent::PageCount;
        let section = HeaderFooterContent::SectionTitle;
        let author = HeaderFooterContent::Author;
        let date = HeaderFooterContent::Date;

        assert!(matches!(text, HeaderFooterContent::Text(_)));
        assert!(matches!(page_num, HeaderFooterContent::PageNumber));
        assert!(matches!(page_count, HeaderFooterContent::PageCount));
        assert!(matches!(section, HeaderFooterContent::SectionTitle));
        assert!(matches!(author, HeaderFooterContent::Author));
        assert!(matches!(date, HeaderFooterContent::Date));
    }

    #[test]
    fn test_page_number_align_variants() {
        assert_eq!(PageNumberAlign::Left, PageNumberAlign::Left);
        assert_eq!(PageNumberAlign::Center, PageNumberAlign::Center);
        assert_eq!(PageNumberAlign::Right, PageNumberAlign::Right);
    }

    #[test]
    fn test_page_number_style_variants() {
        assert_eq!(PageNumberStyle::Numeric, PageNumberStyle::Numeric);
        assert_eq!(PageNumberStyle::Roman, PageNumberStyle::Roman);
        assert_eq!(PageNumberStyle::Letter, PageNumberStyle::Letter);
    }

    #[test]
    fn test_content_to_typst() {
        let system = PageHeaderFooter::new();
        let text = system.content_to_typst(&HeaderFooterContent::Text("Test".to_string()));
        assert!(text.contains("Test"));
    }

    #[test]
    fn test_content_to_html() {
        let system = PageHeaderFooter::new();
        let html = system.content_to_html(&HeaderFooterContent::Text("Test".to_string()));
        assert!(html.contains("Test"));
    }

    #[test]
    fn test_page_number_style_to_typst() {
        let system = PageHeaderFooter::new();
        assert_eq!(system.page_number_style_to_typst(), "1");
    }

    #[test]
    fn test_align_to_typst() {
        let system = PageHeaderFooter::new();
        assert_eq!(system.align_to_typst(), "center");
    }

    #[test]
    fn test_page_header_footer_with_page_number() {
        let page_number = PageNumberConfig {
            style: PageNumberStyle::Roman,
            align: PageNumberAlign::Right,
            supplement: Some("of".to_string()),
        };
        let system = PageHeaderFooter::new().with_page_number(page_number);
        assert_eq!(system.page_number.style, PageNumberStyle::Roman);
    }
}
