/*!
 * 航空航天级段落系统
 * 实现 Typst 的段落功能（对齐、缩进、间距、断行）
 */

use serde::{Deserialize, Serialize};

/// 段落对齐方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParagraphAlign {
    Left,
    Center,
    Right,
    Justify,
}

/// 段落缩进
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParagraphIndent {
    Uniform(f64),
    Sides { first: f64, rest: f64 },
}

/// 段落间距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParagraphSpacing {
    pub before: f64,
    pub after: f64,
}

impl Default for ParagraphSpacing {
    fn default() -> Self {
        Self {
            before: 0.0,
            after: 0.0,
        }
    }
}

/// 段落配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParagraphConfig {
    pub align: ParagraphAlign,
    pub indent: ParagraphIndent,
    pub spacing: ParagraphSpacing,
    pub line_height: Option<f64>,
    pub justify: bool,
    pub hanging_indent: Option<f64>,
}

impl Default for ParagraphConfig {
    fn default() -> Self {
        Self {
            align: ParagraphAlign::Left,
            indent: ParagraphIndent::Uniform(0.0),
            spacing: ParagraphSpacing::default(),
            line_height: None,
            justify: false,
            hanging_indent: None,
        }
    }
}

/// 段落
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paragraph {
    pub content: String,
    pub config: ParagraphConfig,
}

impl Paragraph {
    pub fn new(content: String) -> Self {
        Self {
            content,
            config: ParagraphConfig::default(),
        }
    }

    pub fn with_config(mut self, config: ParagraphConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_align(mut self, align: ParagraphAlign) -> Self {
        self.config.align = align;
        self
    }

    pub fn with_indent(mut self, indent: ParagraphIndent) -> Self {
        self.config.indent = indent;
        self
    }

    pub fn with_spacing(mut self, spacing: ParagraphSpacing) -> Self {
        self.config.spacing = spacing;
        self
    }

    pub fn with_line_height(mut self, height: f64) -> Self {
        self.config.line_height = Some(height);
        self
    }

    pub fn with_justify(mut self, justify: bool) -> Self {
        self.config.justify = justify;
        self
    }

    pub fn with_hanging_indent(mut self, indent: f64) -> Self {
        self.config.hanging_indent = Some(indent);
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        typst.push_str("#set par(\n");

        // 添加对齐
        typst.push_str(&format!("  align: {},\n", self.align_to_typst()));

        // 添加缩进
        match &self.config.indent {
            ParagraphIndent::Uniform(size) => {
                if *size != 0.0 {
                    typst.push_str(&format!("  first-line-indent: {}em,\n", size));
                }
            }
            ParagraphIndent::Sides { first, rest } => {
                typst.push_str(&format!("  first-line-indent: {}em,\n", first));
                typst.push_str(&format!("  hanging-indent: {}em,\n", rest));
            }
        }

        // 添加间距
        if self.config.spacing.before != 0.0 {
            typst.push_str(&format!(
                "  spacing: (before: {}em),\n",
                self.config.spacing.before
            ));
        }
        if self.config.spacing.after != 0.0 {
            typst.push_str(&format!(
                "  spacing: (after: {}em),\n",
                self.config.spacing.after
            ));
        }

        // 添加行高
        if let Some(height) = self.config.line_height {
            typst.push_str(&format!("  leading: {}em,\n", height));
        }

        // 添加对齐
        if self.config.justify {
            typst.push_str("  justify: true,\n");
        }

        // 添加悬挂缩进
        if let Some(indent) = self.config.hanging_indent {
            typst.push_str(&format!("  hanging-indent: {}em,\n", indent));
        }

        typst.push_str(")\n");

        typst.push_str(&format!("{}\n", html_escape(&self.content)));

        typst
    }

    fn align_to_typst(&self) -> String {
        match self.config.align {
            ParagraphAlign::Left => "left".to_string(),
            ParagraphAlign::Center => "center".to_string(),
            ParagraphAlign::Right => "right".to_string(),
            ParagraphAlign::Justify => "justify".to_string(),
        }
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        let align_attr = match self.config.align {
            ParagraphAlign::Left => "text-align: left;",
            ParagraphAlign::Center => "text-align: center;",
            ParagraphAlign::Right => "text-align: right;",
            ParagraphAlign::Justify => "text-align: justify;",
        };

        let indent_attr = match &self.config.indent {
            ParagraphIndent::Uniform(size) => {
                if *size != 0.0 {
                    format!("text-indent: {}em;", size)
                } else {
                    String::new()
                }
            }
            ParagraphIndent::Sides { first, rest } => {
                format!("text-indent: {}em; padding-left: {}em;", first, rest)
            }
        };

        let spacing_attr = if self.config.spacing.before != 0.0 || self.config.spacing.after != 0.0
        {
            format!(
                "margin: {}em 0 {}em 0;",
                self.config.spacing.before, self.config.spacing.after
            )
        } else {
            String::new()
        };

        let line_height_attr = if let Some(height) = self.config.line_height {
            format!("line-height: {};", height)
        } else {
            String::new()
        };

        html.push_str(&format!(
            "<p class=\"typst-paragraph\" style=\"{}{}{}{}\">\n",
            align_attr, indent_attr, spacing_attr, line_height_attr
        ));
        html.push_str(&format!("  {}\n", html_escape(&self.content)));
        html.push_str("</p>\n");

        html
    }
}

impl Default for Paragraph {
    fn default() -> Self {
        Self::new("".to_string())
    }
}

/// 段落构建器
pub struct ParagraphBuilder {
    paragraph: Paragraph,
}

impl ParagraphBuilder {
    pub fn new(content: String) -> Self {
        Self {
            paragraph: Paragraph::new(content),
        }
    }

    pub fn align(mut self, align: ParagraphAlign) -> Self {
        self.paragraph = self.paragraph.with_align(align);
        self
    }

    pub fn indent(mut self, indent: ParagraphIndent) -> Self {
        self.paragraph = self.paragraph.with_indent(indent);
        self
    }

    pub fn spacing(mut self, spacing: ParagraphSpacing) -> Self {
        self.paragraph = self.paragraph.with_spacing(spacing);
        self
    }

    pub fn line_height(mut self, height: f64) -> Self {
        self.paragraph = self.paragraph.with_line_height(height);
        self
    }

    pub fn justify(mut self, justify: bool) -> Self {
        self.paragraph = self.paragraph.with_justify(justify);
        self
    }

    pub fn hanging_indent(mut self, indent: f64) -> Self {
        self.paragraph = self.paragraph.with_hanging_indent(indent);
        self
    }

    pub fn build(self) -> Paragraph {
        self.paragraph
    }
}

impl Default for ParagraphBuilder {
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
    fn test_paragraph_creation() {
        let paragraph = Paragraph::new("Test paragraph".to_string());
        assert_eq!(paragraph.content, "Test paragraph");
    }

    #[test]
    fn test_paragraph_default() {
        let paragraph = Paragraph::default();
        assert_eq!(paragraph.content, "");
    }

    #[test]
    fn test_paragraph_config_default() {
        let config = ParagraphConfig::default();
        assert_eq!(config.align, ParagraphAlign::Left);
        assert!(!config.justify);
    }

    #[test]
    fn test_paragraph_with_align() {
        let paragraph = Paragraph::new("Test".to_string()).with_align(ParagraphAlign::Center);
        assert_eq!(paragraph.config.align, ParagraphAlign::Center);
    }

    #[test]
    fn test_paragraph_with_indent() {
        let paragraph =
            Paragraph::new("Test".to_string()).with_indent(ParagraphIndent::Uniform(2.0));
        assert!(matches!(
            paragraph.config.indent,
            ParagraphIndent::Uniform(2.0)
        ));
    }

    #[test]
    fn test_paragraph_with_spacing() {
        let spacing = ParagraphSpacing {
            before: 1.0,
            after: 1.0,
        };
        let paragraph = Paragraph::new("Test".to_string()).with_spacing(spacing);
        assert_eq!(paragraph.config.spacing.before, 1.0);
    }

    #[test]
    fn test_paragraph_with_line_height() {
        let paragraph = Paragraph::new("Test".to_string()).with_line_height(1.5);
        assert_eq!(paragraph.config.line_height, Some(1.5));
    }

    #[test]
    fn test_paragraph_with_justify() {
        let paragraph = Paragraph::new("Test".to_string()).with_justify(true);
        assert!(paragraph.config.justify);
    }

    #[test]
    fn test_paragraph_with_hanging_indent() {
        let paragraph = Paragraph::new("Test".to_string()).with_hanging_indent(2.0);
        assert_eq!(paragraph.config.hanging_indent, Some(2.0));
    }

    #[test]
    fn test_paragraph_align_partial_eq() {
        assert_eq!(ParagraphAlign::Left, ParagraphAlign::Left);
        assert_ne!(ParagraphAlign::Left, ParagraphAlign::Center);
    }

    #[test]
    fn test_paragraph_spacing_default() {
        let spacing = ParagraphSpacing::default();
        assert_eq!(spacing.before, 0.0);
        assert_eq!(spacing.after, 0.0);
    }

    #[test]
    fn test_to_typst() {
        let paragraph = Paragraph::new("Test".to_string());
        let typst = paragraph.to_typst();
        assert!(typst.contains("#set par("));
        assert!(typst.contains("align: left"));
    }

    #[test]
    fn test_to_typst_with_align() {
        let paragraph = Paragraph::new("Test".to_string()).with_align(ParagraphAlign::Center);
        let typst = paragraph.to_typst();
        assert!(typst.contains("align: center"));
    }

    #[test]
    fn test_to_typst_with_justify() {
        let paragraph = Paragraph::new("Test".to_string()).with_justify(true);
        let typst = paragraph.to_typst();
        assert!(typst.contains("justify: true"));
    }

    #[test]
    fn test_to_html() {
        let paragraph = Paragraph::new("Test".to_string());
        let html = paragraph.to_html();
        assert!(html.contains("<p class=\"typst-paragraph\""));
        assert!(html.contains("text-align: left"));
    }

    #[test]
    fn test_to_html_with_align() {
        let paragraph = Paragraph::new("Test".to_string()).with_align(ParagraphAlign::Center);
        let html = paragraph.to_html();
        assert!(html.contains("text-align: center"));
    }

    #[test]
    fn test_to_html_with_indent() {
        let paragraph =
            Paragraph::new("Test".to_string()).with_indent(ParagraphIndent::Uniform(2.0));
        let html = paragraph.to_html();
        assert!(html.contains("text-indent: 2em"));
    }

    #[test]
    fn test_paragraph_builder() {
        let paragraph = ParagraphBuilder::new("Test".to_string())
            .align(ParagraphAlign::Center)
            .justify(true)
            .build();

        assert_eq!(paragraph.content, "Test");
        assert_eq!(paragraph.config.align, ParagraphAlign::Center);
    }

    #[test]
    fn test_paragraph_builder_default() {
        let builder = ParagraphBuilder::default();
        let paragraph = builder.build();
        assert_eq!(paragraph.content, "");
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_paragraph_align_variants() {
        assert_eq!(ParagraphAlign::Left, ParagraphAlign::Left);
        assert_eq!(ParagraphAlign::Center, ParagraphAlign::Center);
        assert_eq!(ParagraphAlign::Right, ParagraphAlign::Right);
        assert_eq!(ParagraphAlign::Justify, ParagraphAlign::Justify);
    }

    #[test]
    fn test_paragraph_indent_sides() {
        let indent = ParagraphIndent::Sides {
            first: 1.0,
            rest: 2.0,
        };
        assert!(matches!(indent, ParagraphIndent::Sides { .. }));
    }

    #[test]
    fn test_to_typst_with_indent_sides() {
        let paragraph = Paragraph::new("Test".to_string()).with_indent(ParagraphIndent::Sides {
            first: 1.0,
            rest: 2.0,
        });
        let typst = paragraph.to_typst();
        assert!(typst.contains("first-line-indent: 1em"));
        assert!(typst.contains("hanging-indent: 2em"));
    }

    #[test]
    fn test_to_html_with_indent_sides() {
        let paragraph = Paragraph::new("Test".to_string()).with_indent(ParagraphIndent::Sides {
            first: 1.0,
            rest: 2.0,
        });
        let html = paragraph.to_html();
        assert!(html.contains("text-indent: 1em"));
        assert!(html.contains("padding-left: 2em"));
    }

    #[test]
    fn test_to_html_with_spacing() {
        let spacing = ParagraphSpacing {
            before: 1.0,
            after: 2.0,
        };
        let paragraph = Paragraph::new("Test".to_string()).with_spacing(spacing);
        let html = paragraph.to_html();
        assert!(html.contains("margin: 1em 0 2em 0"));
    }

    #[test]
    fn test_to_html_with_line_height() {
        let paragraph = Paragraph::new("Test".to_string()).with_line_height(1.5);
        let html = paragraph.to_html();
        assert!(html.contains("line-height: 1.5"));
    }
}
