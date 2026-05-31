/*!
 * 航空航天级页面系统
 * 实现 Typst 的页面功能（页面尺寸、边距、页眉页脚、编号、背景）
 */

use serde::{Deserialize, Serialize};

/// 纸张尺寸
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaperSize {
    A4,
    A5,
    Letter,
    Legal,
    Custom { width: f64, height: f64 },
}

/// 页面对齐方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PageAlignment {
    Auto,
    Left,
    Right,
    Center,
    Top,
    Bottom,
}

/// 页面配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageConfig {
    pub paper: PaperSize,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub flipped: bool,
    pub margin: Option<f64>,
    pub binding: PageAlignment,
    pub columns: Option<usize>,
    pub fill: Option<String>,
    pub numbering: Option<String>,
    pub supplement: Option<String>,
    pub number_align: PageAlignment,
    pub header: Option<String>,
    pub footer: Option<String>,
    pub background: Option<String>,
    pub foreground: Option<String>,
}

impl Default for PageConfig {
    fn default() -> Self {
        Self {
            paper: PaperSize::A4,
            width: None,
            height: None,
            flipped: false,
            margin: Some(1.0),
            binding: PageAlignment::Auto,
            columns: None,
            fill: None,
            numbering: None,
            supplement: None,
            number_align: PageAlignment::Center,
            header: None,
            footer: None,
            background: None,
            foreground: None,
        }
    }
}

/// 页面
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub content: String,
    pub config: PageConfig,
}

impl Page {
    pub fn new(content: String) -> Self {
        Self {
            content,
            config: PageConfig::default(),
        }
    }

    pub fn with_config(mut self, config: PageConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_paper(mut self, paper: PaperSize) -> Self {
        self.config.paper = paper;
        self
    }

    pub fn with_width(mut self, width: f64) -> Self {
        self.config.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: f64) -> Self {
        self.config.height = Some(height);
        self
    }

    pub fn with_flipped(mut self, flipped: bool) -> Self {
        self.config.flipped = flipped;
        self
    }

    pub fn with_margin(mut self, margin: f64) -> Self {
        self.config.margin = Some(margin);
        self
    }

    pub fn with_binding(mut self, binding: PageAlignment) -> Self {
        self.config.binding = binding;
        self
    }

    pub fn with_columns(mut self, columns: usize) -> Self {
        self.config.columns = Some(columns);
        self
    }

    pub fn with_fill(mut self, fill: String) -> Self {
        self.config.fill = Some(fill);
        self
    }

    pub fn with_numbering(mut self, numbering: String) -> Self {
        self.config.numbering = Some(numbering);
        self
    }

    pub fn with_supplement(mut self, supplement: String) -> Self {
        self.config.supplement = Some(supplement);
        self
    }

    pub fn with_number_align(mut self, align: PageAlignment) -> Self {
        self.config.number_align = align;
        self
    }

    pub fn with_header(mut self, header: String) -> Self {
        self.config.header = Some(header);
        self
    }

    pub fn with_footer(mut self, footer: String) -> Self {
        self.config.footer = Some(footer);
        self
    }

    pub fn with_background(mut self, background: String) -> Self {
        self.config.background = Some(background);
        self
    }

    pub fn with_foreground(mut self, foreground: String) -> Self {
        self.config.foreground = Some(foreground);
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        typst.push_str("#set page(");

        // 添加纸张
        match &self.config.paper {
            PaperSize::A4 => typst.push_str("paper: \"a4\", "),
            PaperSize::A5 => typst.push_str("paper: \"a5\", "),
            PaperSize::Letter => typst.push_str("paper: \"letter\", "),
            PaperSize::Legal => typst.push_str("paper: \"legal\", "),
            PaperSize::Custom { width, height } => {
                typst.push_str(&format!("paper: ({}cm, {}cm), ", width, height));
            }
        }

        // 添加宽度
        if let Some(width) = self.config.width {
            typst.push_str(&format!("width: {}cm, ", width));
        }

        // 添加高度
        if let Some(height) = self.config.height {
            typst.push_str(&format!("height: {}cm, ", height));
        }

        // 添加翻转
        if self.config.flipped {
            typst.push_str("flipped: true, ");
        }

        // 添加边距
        if let Some(margin) = self.config.margin {
            typst.push_str(&format!("margin: {}cm, ", margin));
        }

        // 添加装订
        if !matches!(self.config.binding, PageAlignment::Auto) {
            typst.push_str(&format!("binding: {}, ", self.alignment_to_typst()));
        }

        // 添加栏数
        if let Some(columns) = self.config.columns {
            typst.push_str(&format!("columns: {}, ", columns));
        }

        // 添加填充
        if let Some(fill) = &self.config.fill {
            typst.push_str(&format!("fill: {}, ", fill));
        }

        // 添加编号
        if let Some(numbering) = &self.config.numbering {
            typst.push_str(&format!("numbering: \"{}\", ", numbering));
        }

        // 添加补充文本
        if let Some(supplement) = &self.config.supplement {
            typst.push_str(&format!("supplement: [{}], ", html_escape(supplement)));
        }

        // 添加编号对齐
        if !matches!(self.config.number_align, PageAlignment::Center) {
            typst.push_str(&format!("number-align: {}, ", self.alignment_to_typst()));
        }

        // 添加页眉
        if let Some(header) = &self.config.header {
            typst.push_str(&format!("header: [{}], ", html_escape(header)));
        }

        // 添加页脚
        if let Some(footer) = &self.config.footer {
            typst.push_str(&format!("footer: [{}], ", html_escape(footer)));
        }

        // 添加背景
        if let Some(background) = &self.config.background {
            typst.push_str(&format!("background: {}, ", background));
        }

        // 添加前景
        if let Some(foreground) = &self.config.foreground {
            typst.push_str(&format!("foreground: {}, ", foreground));
        }

        // 移除最后的逗号和空格
        if typst.ends_with(", ") {
            typst.pop();
            typst.pop();
        }

        typst.push_str(")\n");

        // 添加内容
        typst.push_str(&format!("{}\n", html_escape(&self.content)));

        typst
    }

    fn alignment_to_typst(&self) -> String {
        match self.config.binding {
            PageAlignment::Auto => "auto".to_string(),
            PageAlignment::Left => "left".to_string(),
            PageAlignment::Right => "right".to_string(),
            PageAlignment::Center => "center".to_string(),
            PageAlignment::Top => "top".to_string(),
            PageAlignment::Bottom => "bottom".to_string(),
        }
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        let paper_attr = format!(" data-paper=\"{}\"", self.paper_to_string());
        let margin_attr = if let Some(margin) = self.config.margin {
            format!(" style=\"margin: {}cm;\"", margin)
        } else {
            String::new()
        };
        let columns_attr = if let Some(columns) = self.config.columns {
            format!(" data-columns=\"{}\"", columns)
        } else {
            String::new()
        };
        let _fill_attr = if let Some(fill) = &self.config.fill {
            format!(" style=\"background-color: {};\"", fill)
        } else {
            String::new()
        };

        html.push_str(&format!(
            "<div class=\"typst-page\"{}{}{}>\n",
            paper_attr, columns_attr, margin_attr
        ));

        // 添加页眉
        if let Some(header) = &self.config.header {
            html.push_str(&format!(
                "  <header class=\"page-header\">{}</header>\n",
                html_escape(header)
            ));
        }

        // 添加内容
        html.push_str(&format!(
            "  <main class=\"page-content\">{}</main>\n",
            html_escape(&self.content)
        ));

        // 添加页脚
        if let Some(footer) = &self.config.footer {
            html.push_str(&format!(
                "  <footer class=\"page-footer\">{}</footer>\n",
                html_escape(footer)
            ));
        }

        html.push_str("</div>\n");

        html
    }

    fn paper_to_string(&self) -> String {
        match &self.config.paper {
            PaperSize::A4 => "a4".to_string(),
            PaperSize::A5 => "a5".to_string(),
            PaperSize::Letter => "letter".to_string(),
            PaperSize::Legal => "legal".to_string(),
            PaperSize::Custom { width, height } => format!("custom-{}x{}", width, height),
        }
    }
}

impl Default for Page {
    fn default() -> Self {
        Self::new("".to_string())
    }
}

/// 页面构建器
pub struct PageBuilder {
    page: Page,
}

impl PageBuilder {
    pub fn new(content: String) -> Self {
        Self {
            page: Page::new(content),
        }
    }

    pub fn paper(mut self, paper: PaperSize) -> Self {
        self.page = self.page.with_paper(paper);
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.page = self.page.with_width(width);
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.page = self.page.with_height(height);
        self
    }

    pub fn flipped(mut self, flipped: bool) -> Self {
        self.page = self.page.with_flipped(flipped);
        self
    }

    pub fn margin(mut self, margin: f64) -> Self {
        self.page = self.page.with_margin(margin);
        self
    }

    pub fn binding(mut self, binding: PageAlignment) -> Self {
        self.page = self.page.with_binding(binding);
        self
    }

    pub fn columns(mut self, columns: usize) -> Self {
        self.page = self.page.with_columns(columns);
        self
    }

    pub fn fill(mut self, fill: String) -> Self {
        self.page = self.page.with_fill(fill);
        self
    }

    pub fn numbering(mut self, numbering: String) -> Self {
        self.page = self.page.with_numbering(numbering);
        self
    }

    pub fn supplement(mut self, supplement: String) -> Self {
        self.page = self.page.with_supplement(supplement);
        self
    }

    pub fn number_align(mut self, align: PageAlignment) -> Self {
        self.page = self.page.with_number_align(align);
        self
    }

    pub fn header(mut self, header: String) -> Self {
        self.page = self.page.with_header(header);
        self
    }

    pub fn footer(mut self, footer: String) -> Self {
        self.page = self.page.with_footer(footer);
        self
    }

    pub fn background(mut self, background: String) -> Self {
        self.page = self.page.with_background(background);
        self
    }

    pub fn foreground(mut self, foreground: String) -> Self {
        self.page = self.page.with_foreground(foreground);
        self
    }

    pub fn build(self) -> Page {
        self.page
    }
}

impl Default for PageBuilder {
    fn default() -> Self {
        Self::new("".to_string())
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
    fn test_page_creation() {
        let page = Page::new("Content".to_string());
        assert_eq!(page.content, "Content");
    }

    #[test]
    fn test_page_default() {
        let page = Page::default();
        assert_eq!(page.content, "");
    }

    #[test]
    fn test_page_config_default() {
        let config = PageConfig::default();
        assert!(matches!(config.paper, PaperSize::A4));
        assert!(!config.flipped);
    }

    #[test]
    fn test_page_with_paper() {
        let page = Page::new("Content".to_string()).with_paper(PaperSize::Letter);
        assert!(matches!(page.config.paper, PaperSize::Letter));
    }

    #[test]
    fn test_page_with_width() {
        let page = Page::new("Content".to_string()).with_width(21.0);
        assert_eq!(page.config.width, Some(21.0));
    }

    #[test]
    fn test_page_with_height() {
        let page = Page::new("Content".to_string()).with_height(29.7);
        assert_eq!(page.config.height, Some(29.7));
    }

    #[test]
    fn test_page_with_flipped() {
        let page = Page::new("Content".to_string()).with_flipped(true);
        assert!(page.config.flipped);
    }

    #[test]
    fn test_page_with_margin() {
        let page = Page::new("Content".to_string()).with_margin(2.0);
        assert_eq!(page.config.margin, Some(2.0));
    }

    #[test]
    fn test_page_with_columns() {
        let page = Page::new("Content".to_string()).with_columns(2);
        assert_eq!(page.config.columns, Some(2));
    }

    #[test]
    fn test_page_with_fill() {
        let page = Page::new("Content".to_string()).with_fill("white".to_string());
        assert_eq!(page.config.fill, Some("white".to_string()));
    }

    #[test]
    fn test_page_with_numbering() {
        let page = Page::new("Content".to_string()).with_numbering("1".to_string());
        assert_eq!(page.config.numbering, Some("1".to_string()));
    }

    #[test]
    fn test_page_with_header() {
        let page = Page::new("Content".to_string()).with_header("Header".to_string());
        assert_eq!(page.config.header, Some("Header".to_string()));
    }

    #[test]
    fn test_page_with_footer() {
        let page = Page::new("Content".to_string()).with_footer("Footer".to_string());
        assert_eq!(page.config.footer, Some("Footer".to_string()));
    }

    #[test]
    fn test_paper_size_variants() {
        assert!(matches!(PaperSize::A4, PaperSize::A4));
        assert!(matches!(
            PaperSize::Custom {
                width: 10.0,
                height: 20.0
            },
            PaperSize::Custom { .. }
        ));
    }

    #[test]
    fn test_page_alignment_variants() {
        assert_eq!(PageAlignment::Auto, PageAlignment::Auto);
        assert_eq!(PageAlignment::Left, PageAlignment::Left);
        assert_eq!(PageAlignment::Center, PageAlignment::Center);
    }

    #[test]
    fn test_to_typst() {
        let page = Page::new("Content".to_string());
        let typst = page.to_typst();
        assert!(typst.contains("#set page("));
        assert!(typst.contains("paper: \"a4\""));
    }

    #[test]
    fn test_to_typst_with_custom_paper() {
        let page = Page::new("Content".to_string()).with_paper(PaperSize::Custom {
            width: 10.0,
            height: 20.0,
        });
        let typst = page.to_typst();
        assert!(typst.contains("paper: (10cm, 20cm)"));
    }

    #[test]
    fn test_to_typst_with_flipped() {
        let page = Page::new("Content".to_string()).with_flipped(true);
        let typst = page.to_typst();
        assert!(typst.contains("flipped: true"));
    }

    #[test]
    fn test_to_typst_with_numbering() {
        let page = Page::new("Content".to_string()).with_numbering("1".to_string());
        let typst = page.to_typst();
        assert!(typst.contains("numbering: \"1\""));
    }

    #[test]
    fn test_to_html() {
        let page = Page::new("Content".to_string());
        let html = page.to_html();
        assert!(html.contains("<div class=\"typst-page\""));
        assert!(html.contains("data-paper=\"a4\""));
    }

    #[test]
    fn test_to_html_with_header() {
        let page = Page::new("Content".to_string()).with_header("Header".to_string());
        let html = page.to_html();
        assert!(html.contains("<header class=\"page-header\""));
        assert!(html.contains("Header"));
    }

    #[test]
    fn test_to_html_with_footer() {
        let page = Page::new("Content".to_string()).with_footer("Footer".to_string());
        let html = page.to_html();
        assert!(html.contains("<footer class=\"page-footer\""));
        assert!(html.contains("Footer"));
    }

    #[test]
    fn test_page_builder() {
        let page = PageBuilder::new("Content".to_string())
            .paper(PaperSize::Letter)
            .margin(2.0)
            .columns(2)
            .build();

        assert_eq!(page.content, "Content");
        assert!(matches!(page.config.paper, PaperSize::Letter));
    }

    #[test]
    fn test_page_builder_default() {
        let builder = PageBuilder::default();
        let page = builder.build();
        assert_eq!(page.content, "");
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_paper_to_string() {
        let page = Page::new("Content".to_string());
        assert_eq!(page.paper_to_string(), "a4");
    }

    #[test]
    fn test_alignment_to_typst() {
        let page = Page::new("Content".to_string());
        assert_eq!(page.alignment_to_typst(), "auto");
    }

    #[test]
    fn test_to_html_with_columns() {
        let page = Page::new("Content".to_string()).with_columns(2);
        let html = page.to_html();
        assert!(html.contains("data-columns=\"2\""));
    }
}
