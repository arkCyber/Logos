/*!
 * 航空航天级链接系统
 * 实现 Typst 的链接功能（外部链接、内部链接、交叉引用、链接样式）
 */

use serde::{Deserialize, Serialize};

/// 链接目标类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkDestination {
    Url(String),
    Label(String),
    Location {
        element: String,
        position: Option<String>,
    },
}

/// 链接配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkConfig {
    pub destination: LinkDestination,
    pub underline: bool,
    pub color: Option<String>,
}

impl Default for LinkConfig {
    fn default() -> Self {
        Self {
            destination: LinkDestination::Url("".to_string()),
            underline: false,
            color: None,
        }
    }
}

/// 链接
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub content: String,
    pub config: LinkConfig,
}

impl Link {
    pub fn new(content: String, destination: LinkDestination) -> Self {
        Self {
            content,
            config: LinkConfig {
                destination,
                ..Default::default()
            },
        }
    }

    pub fn with_config(mut self, config: LinkConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_underline(mut self, underline: bool) -> Self {
        self.config.underline = underline;
        self
    }

    pub fn with_color(mut self, color: String) -> Self {
        self.config.color = Some(color);
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        typst.push_str("#link(");

        // 添加目标
        match &self.config.destination {
            LinkDestination::Url(url) => {
                typst.push_str(&format!("\"{}\"", url)); // Don't escape for Typst
            }
            LinkDestination::Label(label) => {
                typst.push_str(&format!("@{}", label));
            }
            LinkDestination::Location { element, position } => {
                typst.push_str(&format!("@{}", element));
                if let Some(pos) = position {
                    typst.push_str(&format!("({})", pos));
                }
            }
        }

        // 添加内容
        typst.push_str(&format!("[{}])", html_escape(&self.content)));

        typst
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        let href_attr = match &self.config.destination {
            LinkDestination::Url(url) => format!(" href=\"{}\"", html_escape(url)),
            LinkDestination::Label(label) => format!(" href=\"#{}\"", html_escape(label)),
            LinkDestination::Location { element, position } => {
                if let Some(pos) = position {
                    format!(" href=\"#{}-{}\"", html_escape(element), html_escape(pos))
                } else {
                    format!(" href=\"#{}\"", html_escape(element))
                }
            }
        };

        let class_attr = " class=\"typst-link\"";

        let style_attr = if self.config.underline {
            " style=\"text-decoration: underline;\""
        } else {
            ""
        };

        let color_attr = if let Some(color) = &self.config.color {
            format!(" color: {};", color)
        } else {
            String::new()
        };

        let combined_style = if !color_attr.is_empty() && !style_attr.is_empty() {
            format!(
                " style=\"text-decoration: underline; color: {};\"",
                color_attr
            )
        } else if !color_attr.is_empty() {
            format!(" style=\"{}\"", color_attr)
        } else {
            style_attr.to_string()
        };

        html.push_str(&format!(
            "<a{}{}{}>{}</a>",
            href_attr,
            class_attr,
            combined_style,
            html_escape(&self.content)
        ));

        html
    }
}

impl Default for Link {
    fn default() -> Self {
        Self::new("".to_string(), LinkDestination::Url("".to_string()))
    }
}

/// 链接构建器
pub struct LinkBuilder {
    link: Link,
}

impl LinkBuilder {
    pub fn new(content: String, destination: LinkDestination) -> Self {
        Self {
            link: Link::new(content, destination),
        }
    }

    pub fn underline(mut self, underline: bool) -> Self {
        self.link = self.link.with_underline(underline);
        self
    }

    pub fn color(mut self, color: String) -> Self {
        self.link = self.link.with_color(color);
        self
    }

    pub fn build(self) -> Link {
        self.link
    }
}

impl Default for LinkBuilder {
    fn default() -> Self {
        Self::new("".to_string(), LinkDestination::Url("".to_string()))
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
    fn test_link_creation() {
        let link = Link::new(
            "Click here".to_string(),
            LinkDestination::Url("https://example.com".to_string()),
        );
        assert_eq!(link.content, "Click here");
    }

    #[test]
    fn test_link_default() {
        let link = Link::default();
        assert_eq!(link.content, "");
    }

    #[test]
    fn test_link_config_default() {
        let config = LinkConfig::default();
        assert!(!config.underline);
        assert!(config.color.is_none());
    }

    #[test]
    fn test_link_with_underline() {
        let link = Link::new(
            "Click".to_string(),
            LinkDestination::Url("https://example.com".to_string()),
        )
        .with_underline(true);
        assert!(link.config.underline);
    }

    #[test]
    fn test_link_with_color() {
        let link = Link::new(
            "Click".to_string(),
            LinkDestination::Url("https://example.com".to_string()),
        )
        .with_color("blue".to_string());
        assert_eq!(link.config.color, Some("blue".to_string()));
    }

    #[test]
    fn test_link_destination_url() {
        let dest = LinkDestination::Url("https://example.com".to_string());
        assert!(matches!(dest, LinkDestination::Url(_)));
    }

    #[test]
    fn test_link_destination_label() {
        let dest = LinkDestination::Label("intro".to_string());
        assert!(matches!(dest, LinkDestination::Label(_)));
    }

    #[test]
    fn test_link_destination_location() {
        let dest = LinkDestination::Location {
            element: "section".to_string(),
            position: Some("1".to_string()),
        };
        assert!(matches!(dest, LinkDestination::Location { .. }));
    }

    #[test]
    fn test_to_typst_url() {
        let link = Link::new(
            "Click".to_string(),
            LinkDestination::Url("https://example.com".to_string()),
        );
        let typst = link.to_typst();
        assert!(typst.contains("#link("));
        assert!(typst.contains("\"https://example.com\""));
        assert!(typst.contains("[Click]"));
    }

    #[test]
    fn test_to_typst_label() {
        let link = Link::new(
            "Click".to_string(),
            LinkDestination::Label("intro".to_string()),
        );
        let typst = link.to_typst();
        assert!(typst.contains("@intro"));
    }

    #[test]
    fn test_to_typst_location() {
        let link = Link::new(
            "Click".to_string(),
            LinkDestination::Location {
                element: "section".to_string(),
                position: Some("1".to_string()),
            },
        );
        let typst = link.to_typst();
        assert!(typst.contains("@section"));
        assert!(typst.contains("(1)"));
    }

    #[test]
    fn test_to_html_url() {
        let link = Link::new(
            "Click".to_string(),
            LinkDestination::Url("https://example.com".to_string()),
        );
        let html = link.to_html();
        assert!(html.contains("<a"));
        assert!(html.contains("href=\"https://example.com\""));
        assert!(html.contains("Click"));
    }

    #[test]
    fn test_to_html_label() {
        let link = Link::new(
            "Click".to_string(),
            LinkDestination::Label("intro".to_string()),
        );
        let html = link.to_html();
        assert!(html.contains("href=\"#intro\""));
    }

    #[test]
    fn test_to_html_location() {
        let link = Link::new(
            "Click".to_string(),
            LinkDestination::Location {
                element: "section".to_string(),
                position: Some("1".to_string()),
            },
        );
        let html = link.to_html();
        assert!(html.contains("href=\"#section-1\""));
    }

    #[test]
    fn test_to_html_with_underline() {
        let link = Link::new(
            "Click".to_string(),
            LinkDestination::Url("https://example.com".to_string()),
        )
        .with_underline(true);
        let html = link.to_html();
        assert!(html.contains("text-decoration: underline"));
    }

    #[test]
    fn test_to_html_with_color() {
        let link = Link::new(
            "Click".to_string(),
            LinkDestination::Url("https://example.com".to_string()),
        )
        .with_color("blue".to_string());
        let html = link.to_html();
        assert!(html.contains("color: blue"));
    }

    #[test]
    fn test_link_builder() {
        let link = LinkBuilder::new(
            "Click".to_string(),
            LinkDestination::Url("https://example.com".to_string()),
        )
        .underline(true)
        .color("blue".to_string())
        .build();

        assert_eq!(link.content, "Click");
        assert!(link.config.underline);
    }

    #[test]
    fn test_link_builder_default() {
        let builder = LinkBuilder::default();
        let link = builder.build();
        assert_eq!(link.content, "");
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_to_html_with_underline_and_color() {
        let link = Link::new(
            "Click".to_string(),
            LinkDestination::Url("https://example.com".to_string()),
        )
        .with_underline(true)
        .with_color("blue".to_string());
        let html = link.to_html();
        assert!(html.contains("text-decoration: underline"));
        assert!(html.contains("color: blue"));
    }

    #[test]
    fn test_to_typst_special_chars_in_url() {
        let link = Link::new(
            "Click".to_string(),
            LinkDestination::Url("https://example.com?param=value&other=test".to_string()),
        );
        let typst = link.to_typst();
        assert!(typst.contains("https://example.com?param=value&other=test"));
    }

    #[test]
    fn test_to_html_special_chars_in_url() {
        let link = Link::new(
            "Click".to_string(),
            LinkDestination::Url("https://example.com?param=value&other=test".to_string()),
        );
        let html = link.to_html();
        assert!(html.contains("href=\"https://example.com?param=value&amp;other=test\""));
    }

    #[test]
    fn test_link_destination_variants() {
        let url = LinkDestination::Url("test".to_string());
        let label = LinkDestination::Label("test".to_string());
        let location = LinkDestination::Location {
            element: "test".to_string(),
            position: None,
        };

        assert!(matches!(url, LinkDestination::Url(_)));
        assert!(matches!(label, LinkDestination::Label(_)));
        assert!(matches!(location, LinkDestination::Location { .. }));
    }

    #[test]
    fn test_to_html_location_without_position() {
        let link = Link::new(
            "Click".to_string(),
            LinkDestination::Location {
                element: "section".to_string(),
                position: None,
            },
        );
        let html = link.to_html();
        assert!(html.contains("href=\"#section\""));
    }
}
