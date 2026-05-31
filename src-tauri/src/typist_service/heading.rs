/*!
 * 航空航天级标题系统
 * 实现 Typst 的标题功能（层级、编号、样式、大纲显示）
 */

use serde::{Deserialize, Serialize};

/// 标题层级（1-6级）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HeadingLevel {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl HeadingLevel {
    pub fn to_int(&self) -> usize {
        match self {
            HeadingLevel::One => 1,
            HeadingLevel::Two => 2,
            HeadingLevel::Three => 3,
            HeadingLevel::Four => 4,
            HeadingLevel::Five => 5,
            HeadingLevel::Six => 6,
        }
    }

    pub fn from_int(level: usize) -> Option<Self> {
        match level {
            1 => Some(HeadingLevel::One),
            2 => Some(HeadingLevel::Two),
            3 => Some(HeadingLevel::Three),
            4 => Some(HeadingLevel::Four),
            5 => Some(HeadingLevel::Five),
            6 => Some(HeadingLevel::Six),
            _ => None,
        }
    }
}

/// 编号样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NumberingStyle {
    None,
    Decimal,
    LowerAlpha,
    UpperAlpha,
    LowerRoman,
    UpperRoman,
    Custom(String),
}

/// 标题配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingConfig {
    pub level: HeadingLevel,
    pub numbering: NumberingStyle,
    pub outlined: bool,
    pub bookmarked: bool,
    pub hanging_indent: Option<f64>,
}

impl Default for HeadingConfig {
    fn default() -> Self {
        Self {
            level: HeadingLevel::One,
            numbering: NumberingStyle::None,
            outlined: true,
            bookmarked: true,
            hanging_indent: None,
        }
    }
}

/// 标题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heading {
    pub content: String,
    pub config: HeadingConfig,
    pub label: Option<String>,
}

impl Heading {
    pub fn new(content: String) -> Self {
        Self {
            content,
            config: HeadingConfig::default(),
            label: None,
        }
    }

    pub fn with_config(mut self, config: HeadingConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_level(mut self, level: HeadingLevel) -> Self {
        self.config.level = level;
        self
    }

    pub fn with_numbering(mut self, numbering: NumberingStyle) -> Self {
        self.config.numbering = numbering;
        self
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_outlined(mut self, outlined: bool) -> Self {
        self.config.outlined = outlined;
        self
    }

    pub fn with_bookmarked(mut self, bookmarked: bool) -> Self {
        self.config.bookmarked = bookmarked;
        self
    }

    pub fn with_hanging_indent(mut self, indent: f64) -> Self {
        self.config.hanging_indent = Some(indent);
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        let level_prefix = "=".repeat(self.config.level.to_int());

        typst.push_str(&format!("{} ", level_prefix));

        // 添加标签
        if let Some(label) = &self.label {
            typst.push_str(&format!("<{}> ", label));
        }

        typst.push_str(&format!("{}\n", html_escape(&self.content)));

        typst
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        let level = self.config.level.to_int();
        let tag = format!("h{}", level);

        let id_attr = if let Some(label) = &self.label {
            format!(" id=\"{}\"", label)
        } else {
            String::new()
        };

        let numbering_attr = match &self.config.numbering {
            NumberingStyle::None => String::new(),
            _ => " data-numbering=\"true\"".to_string(),
        };

        html.push_str(&format!(
            "<{} class=\"typst-heading\"{}{}>\n",
            tag, id_attr, numbering_attr
        ));
        html.push_str(&format!("  {}\n", html_escape(&self.content)));
        html.push_str(&format!("</{}>\n", tag));

        html
    }
}

impl Default for Heading {
    fn default() -> Self {
        Self::new("".to_string())
    }
}

/// 标题构建器
pub struct HeadingBuilder {
    heading: Heading,
}

impl HeadingBuilder {
    pub fn new(content: String) -> Self {
        Self {
            heading: Heading::new(content),
        }
    }

    pub fn level(mut self, level: HeadingLevel) -> Self {
        self.heading = self.heading.with_level(level);
        self
    }

    pub fn numbering(mut self, numbering: NumberingStyle) -> Self {
        self.heading = self.heading.with_numbering(numbering);
        self
    }

    pub fn label(mut self, label: String) -> Self {
        self.heading = self.heading.with_label(label);
        self
    }

    pub fn outlined(mut self, outlined: bool) -> Self {
        self.heading = self.heading.with_outlined(outlined);
        self
    }

    pub fn bookmarked(mut self, bookmarked: bool) -> Self {
        self.heading = self.heading.with_bookmarked(bookmarked);
        self
    }

    pub fn hanging_indent(mut self, indent: f64) -> Self {
        self.heading = self.heading.with_hanging_indent(indent);
        self
    }

    pub fn build(self) -> Heading {
        self.heading
    }
}

impl Default for HeadingBuilder {
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
    fn test_heading_creation() {
        let heading = Heading::new("Test Heading".to_string());
        assert_eq!(heading.content, "Test Heading");
    }

    #[test]
    fn test_heading_default() {
        let heading = Heading::default();
        assert_eq!(heading.content, "");
    }

    #[test]
    fn test_heading_config_default() {
        let config = HeadingConfig::default();
        assert_eq!(config.level, HeadingLevel::One);
        assert!(config.outlined);
    }

    #[test]
    fn test_heading_level_to_int() {
        assert_eq!(HeadingLevel::One.to_int(), 1);
        assert_eq!(HeadingLevel::Two.to_int(), 2);
        assert_eq!(HeadingLevel::Six.to_int(), 6);
    }

    #[test]
    fn test_heading_level_from_int() {
        assert_eq!(HeadingLevel::from_int(1), Some(HeadingLevel::One));
        assert_eq!(HeadingLevel::from_int(6), Some(HeadingLevel::Six));
        assert_eq!(HeadingLevel::from_int(7), None);
    }

    #[test]
    fn test_heading_with_level() {
        let heading = Heading::new("Test".to_string()).with_level(HeadingLevel::Two);
        assert_eq!(heading.config.level, HeadingLevel::Two);
    }

    #[test]
    fn test_heading_with_numbering() {
        let heading = Heading::new("Test".to_string()).with_numbering(NumberingStyle::Decimal);
        assert!(matches!(heading.config.numbering, NumberingStyle::Decimal));
    }

    #[test]
    fn test_heading_with_label() {
        let heading = Heading::new("Test".to_string()).with_label("intro".to_string());
        assert_eq!(heading.label, Some("intro".to_string()));
    }

    #[test]
    fn test_heading_with_outlined() {
        let heading = Heading::new("Test".to_string()).with_outlined(false);
        assert!(!heading.config.outlined);
    }

    #[test]
    fn test_heading_with_bookmarked() {
        let heading = Heading::new("Test".to_string()).with_bookmarked(false);
        assert!(!heading.config.bookmarked);
    }

    #[test]
    fn test_heading_with_hanging_indent() {
        let heading = Heading::new("Test".to_string()).with_hanging_indent(2.0);
        assert_eq!(heading.config.hanging_indent, Some(2.0));
    }

    #[test]
    fn test_heading_level_partial_eq() {
        assert_eq!(HeadingLevel::One, HeadingLevel::One);
        assert_ne!(HeadingLevel::One, HeadingLevel::Two);
    }

    #[test]
    fn test_to_typst() {
        let heading = Heading::new("Test".to_string());
        let typst = heading.to_typst();
        assert!(typst.contains("="));
        assert!(typst.contains("Test"));
    }

    #[test]
    fn test_to_typst_with_level() {
        let heading = Heading::new("Test".to_string()).with_level(HeadingLevel::Two);
        let typst = heading.to_typst();
        assert!(typst.contains("=="));
    }

    #[test]
    fn test_to_typst_with_label() {
        let heading = Heading::new("Test".to_string()).with_label("intro".to_string());
        let typst = heading.to_typst();
        assert!(typst.contains("<intro>"));
    }

    #[test]
    fn test_to_html() {
        let heading = Heading::new("Test".to_string());
        let html = heading.to_html();
        assert!(html.contains("<h1 class=\"typst-heading\""));
        assert!(html.contains("Test"));
    }

    #[test]
    fn test_to_html_with_level() {
        let heading = Heading::new("Test".to_string()).with_level(HeadingLevel::Two);
        let html = heading.to_html();
        assert!(html.contains("<h2 class=\"typst-heading\""));
    }

    #[test]
    fn test_to_html_with_label() {
        let heading = Heading::new("Test".to_string()).with_label("intro".to_string());
        let html = heading.to_html();
        assert!(html.contains("id=\"intro\""));
    }

    #[test]
    fn test_heading_builder() {
        let heading = HeadingBuilder::new("Test".to_string())
            .level(HeadingLevel::Two)
            .label("intro".to_string())
            .build();

        assert_eq!(heading.content, "Test");
        assert_eq!(heading.config.level, HeadingLevel::Two);
    }

    #[test]
    fn test_heading_builder_default() {
        let builder = HeadingBuilder::default();
        let heading = builder.build();
        assert_eq!(heading.content, "");
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_heading_level_variants() {
        assert_eq!(HeadingLevel::One, HeadingLevel::One);
        assert_eq!(HeadingLevel::Two, HeadingLevel::Two);
        assert_eq!(HeadingLevel::Three, HeadingLevel::Three);
        assert_eq!(HeadingLevel::Four, HeadingLevel::Four);
        assert_eq!(HeadingLevel::Five, HeadingLevel::Five);
        assert_eq!(HeadingLevel::Six, HeadingLevel::Six);
    }

    #[test]
    fn test_numbering_style_variants() {
        let style1 = NumberingStyle::Decimal;
        let style2 = NumberingStyle::LowerAlpha;
        let style3 = NumberingStyle::Custom("1.1".to_string());
        assert!(matches!(style1, NumberingStyle::Decimal));
        assert!(matches!(style2, NumberingStyle::LowerAlpha));
        assert!(matches!(style3, NumberingStyle::Custom(_)));
    }

    #[test]
    fn test_to_html_with_numbering() {
        let heading = Heading::new("Test".to_string()).with_numbering(NumberingStyle::Decimal);
        let html = heading.to_html();
        assert!(html.contains("data-numbering=\"true\""));
    }

    #[test]
    fn test_to_typst_with_level_six() {
        let heading = Heading::new("Test".to_string()).with_level(HeadingLevel::Six);
        let typst = heading.to_typst();
        assert!(typst.contains("======"));
    }

    #[test]
    fn test_to_html_with_level_six() {
        let heading = Heading::new("Test".to_string()).with_level(HeadingLevel::Six);
        let html = heading.to_html();
        assert!(html.contains("<h6 class=\"typst-heading\""));
    }
}
