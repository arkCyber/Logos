/*!
 * 航空航天级引用系统
 * 实现 Typst 的引用功能（引用、块引用、归属、引号）
 */

use serde::{Deserialize, Serialize};

/// 引用归属类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Attribution {
    None,
    Label(String),
    Content(String),
}

/// 引用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteConfig {
    pub block: bool,
    pub quotes: Option<bool>,
    pub attribution: Attribution,
}

impl Default for QuoteConfig {
    fn default() -> Self {
        Self {
            block: false,
            quotes: None,
            attribution: Attribution::None,
        }
    }
}

/// 引用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub content: String,
    pub config: QuoteConfig,
}

impl Quote {
    pub fn new(content: String) -> Self {
        Self {
            content,
            config: QuoteConfig::default(),
        }
    }

    pub fn with_config(mut self, config: QuoteConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_block(mut self, block: bool) -> Self {
        self.config.block = block;
        self
    }

    pub fn with_quotes(mut self, quotes: bool) -> Self {
        self.config.quotes = Some(quotes);
        self
    }

    pub fn with_attribution(mut self, attribution: Attribution) -> Self {
        self.config.attribution = attribution;
        self
    }

    pub fn with_attribution_label(mut self, label: String) -> Self {
        self.config.attribution = Attribution::Label(label);
        self
    }

    pub fn with_attribution_content(mut self, content: String) -> Self {
        self.config.attribution = Attribution::Content(content);
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        typst.push_str("#quote(");

        // 添加归属
        match &self.config.attribution {
            Attribution::Label(label) => {
                typst.push_str(&format!("attribution: [{}], ", html_escape(label)));
            }
            Attribution::Content(content) => {
                typst.push_str(&format!("attribution: [{}], ", html_escape(content)));
            }
            Attribution::None => {}
        }

        // 添加块级
        if self.config.block {
            typst.push_str("block: true, ");
        }

        // 添加引号
        if let Some(quotes) = self.config.quotes {
            typst.push_str(&format!("quotes: {}, ", quotes));
        }

        // 添加内容
        typst.push_str(&format!("[{}])\n", html_escape(&self.content)));

        typst
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        let block_attr = if self.config.block {
            " data-block=\"true\""
        } else {
            " data-block=\"false\""
        };

        let quotes_attr = if let Some(quotes) = self.config.quotes {
            format!(" data-quotes=\"{}\"", quotes)
        } else {
            String::new()
        };

        if self.config.block {
            html.push_str(&format!(
                "<blockquote class=\"typst-quote\"{}{}>\n",
                block_attr, quotes_attr
            ));
            html.push_str(&format!("  <p>{}</p>\n", html_escape(&self.content)));

            match &self.config.attribution {
                Attribution::Label(label) => {
                    html.push_str(&format!(
                        "  <cite class=\"quote-attribution\">— {}</cite>\n",
                        html_escape(label)
                    ));
                }
                Attribution::Content(content) => {
                    html.push_str(&format!(
                        "  <cite class=\"quote-attribution\">— {}</cite>\n",
                        html_escape(content)
                    ));
                }
                Attribution::None => {}
            }

            html.push_str("</blockquote>\n");
        } else {
            let quote_content = if self.config.quotes.unwrap_or(false) {
                format!("\"{}\"", html_escape(&self.content))
            } else {
                html_escape(&self.content).to_string()
            };

            html.push_str(&format!(
                "<q class=\"typst-quote-inline\"{}{}>{}</q>",
                block_attr, quotes_attr, quote_content
            ));
        }

        html
    }
}

impl Default for Quote {
    fn default() -> Self {
        Self::new("".to_string())
    }
}

/// 引用构建器
pub struct QuoteBuilder {
    quote: Quote,
}

impl QuoteBuilder {
    pub fn new(content: String) -> Self {
        Self {
            quote: Quote::new(content),
        }
    }

    pub fn block(mut self, block: bool) -> Self {
        self.quote = self.quote.with_block(block);
        self
    }

    pub fn quotes(mut self, quotes: bool) -> Self {
        self.quote = self.quote.with_quotes(quotes);
        self
    }

    pub fn attribution(mut self, attribution: Attribution) -> Self {
        self.quote = self.quote.with_attribution(attribution);
        self
    }

    pub fn attribution_label(mut self, label: String) -> Self {
        self.quote = self.quote.with_attribution_label(label);
        self
    }

    pub fn attribution_content(mut self, content: String) -> Self {
        self.quote = self.quote.with_attribution_content(content);
        self
    }

    pub fn build(self) -> Quote {
        self.quote
    }
}

impl Default for QuoteBuilder {
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
    fn test_quote_creation() {
        let quote = Quote::new("To be or not to be".to_string());
        assert_eq!(quote.content, "To be or not to be");
    }

    #[test]
    fn test_quote_default() {
        let quote = Quote::default();
        assert_eq!(quote.content, "");
    }

    #[test]
    fn test_quote_config_default() {
        let config = QuoteConfig::default();
        assert!(!config.block);
        assert!(matches!(config.attribution, Attribution::None));
    }

    #[test]
    fn test_quote_with_block() {
        let quote = Quote::new("Test".to_string()).with_block(true);
        assert!(quote.config.block);
    }

    #[test]
    fn test_quote_with_quotes() {
        let quote = Quote::new("Test".to_string()).with_quotes(true);
        assert_eq!(quote.config.quotes, Some(true));
    }

    #[test]
    fn test_quote_with_attribution_label() {
        let quote =
            Quote::new("Test".to_string()).with_attribution_label("Shakespeare".to_string());
        assert!(matches!(quote.config.attribution, Attribution::Label(_)));
    }

    #[test]
    fn test_quote_with_attribution_content() {
        let quote = Quote::new("Test".to_string())
            .with_attribution_content("William Shakespeare".to_string());
        assert!(matches!(quote.config.attribution, Attribution::Content(_)));
    }

    #[test]
    fn test_attribution_variants() {
        let none = Attribution::None;
        let label = Attribution::Label("test".to_string());
        let content = Attribution::Content("test".to_string());

        assert!(matches!(none, Attribution::None));
        assert!(matches!(label, Attribution::Label(_)));
        assert!(matches!(content, Attribution::Content(_)));
    }

    #[test]
    fn test_to_typst() {
        let quote = Quote::new("Test".to_string());
        let typst = quote.to_typst();
        assert!(typst.contains("#quote("));
        assert!(typst.contains("[Test]"));
    }

    #[test]
    fn test_to_typst_with_block() {
        let quote = Quote::new("Test".to_string()).with_block(true);
        let typst = quote.to_typst();
        assert!(typst.contains("block: true"));
    }

    #[test]
    fn test_to_typst_with_quotes() {
        let quote = Quote::new("Test".to_string()).with_quotes(true);
        let typst = quote.to_typst();
        assert!(typst.contains("quotes: true"));
    }

    #[test]
    fn test_to_typst_with_attribution() {
        let quote = Quote::new("Test".to_string()).with_attribution_label("Author".to_string());
        let typst = quote.to_typst();
        assert!(typst.contains("attribution: [Author]"));
    }

    #[test]
    fn test_to_html_inline() {
        let quote = Quote::new("Test".to_string());
        let html = quote.to_html();
        assert!(html.contains("<q class=\"typst-quote-inline\""));
        assert!(html.contains("Test"));
    }

    #[test]
    fn test_to_html_block() {
        let quote = Quote::new("Test".to_string()).with_block(true);
        let html = quote.to_html();
        assert!(html.contains("<blockquote class=\"typst-quote\""));
        assert!(html.contains("data-block=\"true\""));
    }

    #[test]
    fn test_to_html_with_quotes() {
        let quote = Quote::new("Test".to_string()).with_quotes(true);
        let html = quote.to_html();
        assert!(html.contains("\"Test\""));
    }

    #[test]
    fn test_to_html_with_attribution() {
        let quote = Quote::new("Test".to_string())
            .with_block(true)
            .with_attribution_label("Author".to_string());
        let html = quote.to_html();
        assert!(html.contains("quote-attribution"));
        assert!(html.contains("— Author"));
    }

    #[test]
    fn test_quote_builder() {
        let quote = QuoteBuilder::new("Test".to_string())
            .block(true)
            .quotes(true)
            .attribution_label("Author".to_string())
            .build();

        assert_eq!(quote.content, "Test");
        assert!(quote.config.block);
    }

    #[test]
    fn test_quote_builder_default() {
        let builder = QuoteBuilder::default();
        let quote = builder.build();
        assert_eq!(quote.content, "");
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_to_html_block_with_content_attribution() {
        let quote = Quote::new("Test".to_string())
            .with_block(true)
            .with_attribution_content("William Shakespeare".to_string());
        let html = quote.to_html();
        assert!(html.contains("— William Shakespeare"));
    }

    #[test]
    fn test_to_typst_with_content_attribution() {
        let quote =
            Quote::new("Test".to_string()).with_attribution_content("Full Name".to_string());
        let typst = quote.to_typst();
        assert!(typst.contains("attribution: [Full Name]"));
    }

    #[test]
    fn test_to_html_inline_without_quotes() {
        let quote = Quote::new("Test".to_string()).with_quotes(false);
        let html = quote.to_html();
        assert!(!html.contains("\"Test\""));
        assert!(html.contains("Test"));
    }
}
