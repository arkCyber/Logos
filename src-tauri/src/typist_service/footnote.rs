/*!
 * 航空航天级脚注系统
 * 实现 Typst 的脚注功能（脚注、编号、标签、内容）
 */

use serde::{Deserialize, Serialize};

/// 脚注配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FootnoteConfig {
    pub numbering: Option<String>,
}

/// 脚注
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Footnote {
    pub content: String,
    pub config: FootnoteConfig,
    pub label: Option<String>,
}

impl Footnote {
    pub fn new(content: String) -> Self {
        Self {
            content,
            config: FootnoteConfig::default(),
            label: None,
        }
    }

    pub fn with_config(mut self, config: FootnoteConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_numbering(mut self, numbering: String) -> Self {
        self.config.numbering = Some(numbering);
        self
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        typst.push_str("#footnote(");

        // 添加标签
        if let Some(label) = &self.label {
            typst.push_str(&format!("<{}> ", label));
        }

        // 添加编号
        if let Some(numbering) = &self.config.numbering {
            typst.push_str(&format!("numbering: \"{}\", ", numbering));
        }

        // 添加内容
        typst.push_str(&format!("[{}])\n", html_escape(&self.content)));

        typst
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        let id_attr = if let Some(label) = &self.label {
            format!(" id=\"fn-{}\"", label)
        } else {
            String::new()
        };

        let numbering_attr = if self.config.numbering.is_some() {
            " data-numbered=\"true\""
        } else {
            " data-numbered=\"false\""
        };

        // 脚注引用（上标）
        html.push_str(&format!(
            "<sup class=\"footnote-ref\"{}{}><a href=\"#fn-content-{}\">[1]</a></sup>",
            id_attr,
            numbering_attr,
            self.label.as_deref().unwrap_or("1")
        ));

        // 脚注内容
        html.push_str(&format!(
            "<div class=\"footnote-content\" id=\"fn-content-{}\"{}>\n",
            self.label.as_deref().unwrap_or("1"),
            numbering_attr
        ));
        html.push_str(&format!("  <p>{}</p>\n", html_escape(&self.content)));
        html.push_str("</div>\n");

        html
    }
}

impl Default for Footnote {
    fn default() -> Self {
        Self::new("".to_string())
    }
}

/// 脚注构建器
pub struct FootnoteBuilder {
    footnote: Footnote,
}

impl FootnoteBuilder {
    pub fn new(content: String) -> Self {
        Self {
            footnote: Footnote::new(content),
        }
    }

    pub fn numbering(mut self, numbering: String) -> Self {
        self.footnote = self.footnote.with_numbering(numbering);
        self
    }

    pub fn label(mut self, label: String) -> Self {
        self.footnote = self.footnote.with_label(label);
        self
    }

    pub fn build(self) -> Footnote {
        self.footnote
    }
}

impl Default for FootnoteBuilder {
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
    fn test_footnote_creation() {
        let footnote = Footnote::new("Note".to_string());
        assert_eq!(footnote.content, "Note");
    }

    #[test]
    fn test_footnote_default() {
        let footnote = Footnote::default();
        assert_eq!(footnote.content, "");
    }

    #[test]
    fn test_footnote_config_default() {
        let config = FootnoteConfig::default();
        assert!(config.numbering.is_none());
    }

    #[test]
    fn test_footnote_with_numbering() {
        let footnote = Footnote::new("Note".to_string()).with_numbering("1".to_string());
        assert_eq!(footnote.config.numbering, Some("1".to_string()));
    }

    #[test]
    fn test_footnote_with_label() {
        let footnote = Footnote::new("Note".to_string()).with_label("fn1".to_string());
        assert_eq!(footnote.label, Some("fn1".to_string()));
    }

    #[test]
    fn test_to_typst() {
        let footnote = Footnote::new("Note".to_string());
        let typst = footnote.to_typst();
        assert!(typst.contains("#footnote("));
        assert!(typst.contains("[Note]"));
    }

    #[test]
    fn test_to_typst_with_numbering() {
        let footnote = Footnote::new("Note".to_string()).with_numbering("1".to_string());
        let typst = footnote.to_typst();
        assert!(typst.contains("numbering: \"1\""));
    }

    #[test]
    fn test_to_typst_with_label() {
        let footnote = Footnote::new("Note".to_string()).with_label("fn1".to_string());
        let typst = footnote.to_typst();
        assert!(typst.contains("<fn1>"));
    }

    #[test]
    fn test_to_html() {
        let footnote = Footnote::new("Note".to_string());
        let html = footnote.to_html();
        assert!(html.contains("<sup class=\"footnote-ref\""));
        assert!(html.contains("<div class=\"footnote-content\""));
    }

    #[test]
    fn test_to_html_with_label() {
        let footnote = Footnote::new("Note".to_string()).with_label("fn1".to_string());
        let html = footnote.to_html();
        assert!(html.contains("id=\"fn-fn1\""));
        assert!(html.contains("href=\"#fn-content-fn1\""));
    }

    #[test]
    fn test_to_html_with_numbering() {
        let footnote = Footnote::new("Note".to_string()).with_numbering("1".to_string());
        let html = footnote.to_html();
        assert!(html.contains("data-numbered=\"true\""));
    }

    #[test]
    fn test_footnote_builder() {
        let footnote = FootnoteBuilder::new("Note".to_string())
            .numbering("1".to_string())
            .label("fn1".to_string())
            .build();

        assert_eq!(footnote.content, "Note");
        assert_eq!(footnote.config.numbering, Some("1".to_string()));
    }

    #[test]
    fn test_footnote_builder_default() {
        let builder = FootnoteBuilder::default();
        let footnote = builder.build();
        assert_eq!(footnote.content, "");
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_to_typst_with_config() {
        let config = FootnoteConfig {
            numbering: Some("1".to_string()),
        };
        let footnote = Footnote::new("Note".to_string()).with_config(config);
        let typst = footnote.to_typst();
        assert!(typst.contains("numbering: \"1\""));
    }

    #[test]
    fn test_to_html_contains_link() {
        let footnote = Footnote::new("Note".to_string()).with_label("fn1".to_string());
        let html = footnote.to_html();
        assert!(html.contains("<a href=\"#fn-content-fn1\">"));
    }
}
