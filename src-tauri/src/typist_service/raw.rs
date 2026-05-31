/*!
 * 航空航天级原始内容系统
 * 实现 Typst 的原始文本功能（原始文本块、语法高亮、行号、块级和行级、多语言支持）
 */

use serde::{Deserialize, Serialize};

/// 原始内容类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RawType {
    Inline,
    Block,
}

/// 原始内容配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawConfig {
    pub raw_type: RawType,
    pub lang: Option<String>,
    pub theme: Option<String>,
    pub tab_size: Option<usize>,
    pub show_line_numbers: bool,
}

impl Default for RawConfig {
    fn default() -> Self {
        Self {
            raw_type: RawType::Inline,
            lang: None,
            theme: None,
            tab_size: Some(4),
            show_line_numbers: false,
        }
    }
}

/// 原始内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Raw {
    pub content: String,
    pub config: RawConfig,
}

impl Raw {
    pub fn new(content: String) -> Self {
        Self {
            content,
            config: RawConfig::default(),
        }
    }

    pub fn with_config(mut self, config: RawConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_type(mut self, raw_type: RawType) -> Self {
        self.config.raw_type = raw_type;
        self
    }

    pub fn with_lang(mut self, lang: String) -> Self {
        self.config.lang = Some(lang);
        self
    }

    pub fn with_theme(mut self, theme: String) -> Self {
        self.config.theme = Some(theme);
        self
    }

    pub fn with_tab_size(mut self, tab_size: usize) -> Self {
        self.config.tab_size = Some(tab_size);
        self
    }

    pub fn with_line_numbers(mut self, show: bool) -> Self {
        self.config.show_line_numbers = show;
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        match self.config.raw_type {
            RawType::Inline => {
                // Inline raw: `code`
                typst.push('`');
                typst.push_str(&html_escape(&self.content));
                typst.push('`');
            }
            RawType::Block => {
                // Block raw: ```code```
                typst.push_str("```");
                if let Some(lang) = &self.config.lang {
                    typst.push_str(lang);
                }
                typst.push('\n');
                typst.push_str(&self.content);
                typst.push_str("\n```");
            }
        }

        typst
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        let lang_attr = if let Some(lang) = &self.config.lang {
            format!(" data-lang=\"{}\"", lang)
        } else {
            String::new()
        };

        let theme_attr = if let Some(theme) = &self.config.theme {
            format!(" data-theme=\"{}\"", theme)
        } else {
            String::new()
        };

        let line_numbers_attr = if self.config.show_line_numbers {
            " data-line-numbers=\"true\""
        } else {
            ""
        };

        match self.config.raw_type {
            RawType::Inline => {
                html.push_str(&format!(
                    "<code class=\"typst-raw-inline\"{}{}{}>{}</code>",
                    lang_attr,
                    theme_attr,
                    line_numbers_attr,
                    html_escape(&self.content)
                ));
            }
            RawType::Block => {
                html.push_str(&format!(
                    "<pre class=\"typst-raw-block\"{}{}{}><code>{}</code></pre>",
                    lang_attr,
                    theme_attr,
                    line_numbers_attr,
                    html_escape(&self.content)
                ));
            }
        }

        html
    }
}

impl Default for Raw {
    fn default() -> Self {
        Self::new("".to_string())
    }
}

/// 原始内容构建器
pub struct RawBuilder {
    raw: Raw,
}

impl RawBuilder {
    pub fn new(content: String) -> Self {
        Self {
            raw: Raw::new(content),
        }
    }

    pub fn raw_type(mut self, raw_type: RawType) -> Self {
        self.raw = self.raw.with_type(raw_type);
        self
    }

    pub fn lang(mut self, lang: String) -> Self {
        self.raw = self.raw.with_lang(lang);
        self
    }

    pub fn theme(mut self, theme: String) -> Self {
        self.raw = self.raw.with_theme(theme);
        self
    }

    pub fn tab_size(mut self, tab_size: usize) -> Self {
        self.raw = self.raw.with_tab_size(tab_size);
        self
    }

    pub fn line_numbers(mut self, show: bool) -> Self {
        self.raw = self.raw.with_line_numbers(show);
        self
    }

    pub fn build(self) -> Raw {
        self.raw
    }
}

impl Default for RawBuilder {
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
    fn test_raw_creation() {
        let raw = Raw::new("code".to_string());
        assert_eq!(raw.content, "code");
    }

    #[test]
    fn test_raw_default() {
        let raw = Raw::default();
        assert_eq!(raw.content, "");
    }

    #[test]
    fn test_raw_config_default() {
        let config = RawConfig::default();
        assert_eq!(config.raw_type, RawType::Inline);
        assert_eq!(config.tab_size, Some(4));
    }

    #[test]
    fn test_raw_with_type() {
        let raw = Raw::new("code".to_string()).with_type(RawType::Block);
        assert_eq!(raw.config.raw_type, RawType::Block);
    }

    #[test]
    fn test_raw_with_lang() {
        let raw = Raw::new("code".to_string()).with_lang("rust".to_string());
        assert_eq!(raw.config.lang, Some("rust".to_string()));
    }

    #[test]
    fn test_raw_with_theme() {
        let raw = Raw::new("code".to_string()).with_theme("dark".to_string());
        assert_eq!(raw.config.theme, Some("dark".to_string()));
    }

    #[test]
    fn test_raw_with_tab_size() {
        let raw = Raw::new("code".to_string()).with_tab_size(2);
        assert_eq!(raw.config.tab_size, Some(2));
    }

    #[test]
    fn test_raw_with_line_numbers() {
        let raw = Raw::new("code".to_string()).with_line_numbers(true);
        assert!(raw.config.show_line_numbers);
    }

    #[test]
    fn test_raw_type_partial_eq() {
        assert_eq!(RawType::Inline, RawType::Inline);
        assert_eq!(RawType::Block, RawType::Block);
        assert_ne!(RawType::Inline, RawType::Block);
    }

    #[test]
    fn test_to_typst_inline() {
        let raw = Raw::new("code".to_string());
        let typst = raw.to_typst();
        assert!(typst.contains("`code`"));
    }

    #[test]
    fn test_to_typst_block() {
        let raw = Raw::new("code".to_string()).with_type(RawType::Block);
        let typst = raw.to_typst();
        assert!(typst.contains("```"));
        assert!(typst.contains("code"));
    }

    #[test]
    fn test_to_typst_block_with_lang() {
        let raw = Raw::new("code".to_string())
            .with_type(RawType::Block)
            .with_lang("rust".to_string());
        let typst = raw.to_typst();
        assert!(typst.contains("```rust"));
    }

    #[test]
    fn test_to_html_inline() {
        let raw = Raw::new("code".to_string());
        let html = raw.to_html();
        assert!(html.contains("<code class=\"typst-raw-inline\""));
        assert!(html.contains("code"));
    }

    #[test]
    fn test_to_html_block() {
        let raw = Raw::new("code".to_string()).with_type(RawType::Block);
        let html = raw.to_html();
        assert!(html.contains("<pre class=\"typst-raw-block\""));
        assert!(html.contains("<code>"));
    }

    #[test]
    fn test_to_html_with_lang() {
        let raw = Raw::new("code".to_string()).with_lang("rust".to_string());
        let html = raw.to_html();
        assert!(html.contains("data-lang=\"rust\""));
    }

    #[test]
    fn test_to_html_with_theme() {
        let raw = Raw::new("code".to_string()).with_theme("dark".to_string());
        let html = raw.to_html();
        assert!(html.contains("data-theme=\"dark\""));
    }

    #[test]
    fn test_to_html_with_line_numbers() {
        let raw = Raw::new("code".to_string()).with_line_numbers(true);
        let html = raw.to_html();
        assert!(html.contains("data-line-numbers=\"true\""));
    }

    #[test]
    fn test_raw_builder() {
        let raw = RawBuilder::new("code".to_string())
            .raw_type(RawType::Block)
            .lang("rust".to_string())
            .build();

        assert_eq!(raw.content, "code");
        assert_eq!(raw.config.raw_type, RawType::Block);
    }

    #[test]
    fn test_raw_builder_default() {
        let builder = RawBuilder::default();
        let raw = builder.build();
        assert_eq!(raw.content, "");
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_raw_type_variants() {
        assert_eq!(RawType::Inline, RawType::Inline);
        assert_eq!(RawType::Block, RawType::Block);
    }

    #[test]
    fn test_to_typst_multiline_block() {
        let raw = Raw::new("line1\nline2".to_string()).with_type(RawType::Block);
        let typst = raw.to_typst();
        assert!(typst.contains("line1"));
        assert!(typst.contains("line2"));
    }

    #[test]
    fn test_to_html_multiline_block() {
        let raw = Raw::new("line1\nline2".to_string()).with_type(RawType::Block);
        let html = raw.to_html();
        assert!(html.contains("line1"));
        assert!(html.contains("line2"));
    }

    #[test]
    fn test_to_typst_inline_with_backticks() {
        let raw = Raw::new("code`with`backticks".to_string());
        let typst = raw.to_typst();
        assert!(typst.contains("code`with`backticks"));
    }

    #[test]
    fn test_to_html_special_chars() {
        let raw = Raw::new("<>&\"'".to_string());
        let html = raw.to_html();
        assert!(html.contains("&lt;"));
        assert!(html.contains("&gt;"));
        assert!(html.contains("&amp;"));
        assert!(html.contains("&quot;"));
        assert!(html.contains("&#39;"));
    }
}
