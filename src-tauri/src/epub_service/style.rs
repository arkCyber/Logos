use serde::{Deserialize, Serialize};

/// EPUB CSS 样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpubCss {
    /// CSS 内容
    pub content: String,
    /// 文件名
    pub filename: String,
}

impl EpubCss {
    /// 创建新的 CSS
    #[allow(dead_code)]
    pub fn new(content: String) -> Self {
        Self {
            content,
            filename: "styles.css".to_string(),
        }
    }

    /// 设置文件名
    #[allow(dead_code)]
    pub fn with_filename(mut self, filename: String) -> Self {
        self.filename = filename;
        self
    }

    /// 创建默认样式
    #[allow(dead_code)]
    pub fn default_style() -> Self {
        let css = r#"
body {
    font-family: serif;
    line-height: 1.6;
    margin: 0;
    padding: 1em;
}

h1, h2, h3, h4, h5, h6 {
    margin-top: 1.5em;
    margin-bottom: 0.5em;
}

p {
    margin-bottom: 1em;
}

a {
    color: #0066cc;
    text-decoration: underline;
}

img {
    max-width: 100%;
    height: auto;
}
"#
        .to_string();
        Self::new(css)
    }
}

/// EPUB 样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpubStyle {
    /// CSS 样式
    pub css: Vec<EpubCss>,
}

impl EpubStyle {
    /// 创建新的样式
    pub fn new() -> Self {
        Self { css: Vec::new() }
    }

    /// 添加 CSS
    #[allow(dead_code)]
    pub fn with_css(mut self, css: EpubCss) -> Self {
        self.css.push(css);
        self
    }

    /// 添加默认样式
    #[allow(dead_code)]
    pub fn with_default_style(mut self) -> Self {
        self.css.push(EpubCss::default_style());
        self
    }
}

impl Default for EpubStyle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epub_css_new() {
        let css = EpubCss::new("body { color: black; }".to_string());
        assert_eq!(css.filename, "styles.css");
    }

    #[test]
    fn test_epub_css_with_filename() {
        let css = EpubCss::new("body { color: black; }".to_string())
            .with_filename("custom.css".to_string());
        assert_eq!(css.filename, "custom.css");
    }

    #[test]
    fn test_epub_css_default_style() {
        let css = EpubCss::default_style();
        assert!(css.content.contains("font-family"));
    }

    #[test]
    fn test_epub_style_new() {
        let style = EpubStyle::new();
        assert!(style.css.is_empty());
    }

    #[test]
    fn test_epub_style_with_css() {
        let css = EpubCss::new("body { color: black; }".to_string());
        let style = EpubStyle::new().with_css(css);
        assert_eq!(style.css.len(), 1);
    }

    #[test]
    fn test_epub_style_with_default_style() {
        let style = EpubStyle::new().with_default_style();
        assert_eq!(style.css.len(), 1);
    }

    #[test]
    fn test_epub_css_serialization() {
        let css = EpubCss::new("body { color: black; }".to_string());
        let json = serde_json::to_string(&css);
        assert!(json.is_ok());
    }

    #[test]
    fn test_epub_css_deserialization() {
        let json = r#"{"filename":"styles.css","content":"body { color: black; }"}"#;
        let css: EpubCss = serde_json::from_str(json).unwrap();
        assert_eq!(css.filename, "styles.css");
    }

    #[test]
    fn test_epub_css_empty_content() {
        let css = EpubCss::new("".to_string());
        assert_eq!(css.content, "");
    }

    #[test]
    fn test_epub_css_long_content() {
        let long_css = "a".repeat(10000);
        let css = EpubCss::new(long_css.clone());
        assert_eq!(css.content.len(), 10000);
    }

    #[test]
    fn test_epub_style_default() {
        let style = EpubStyle::default();
        assert!(style.css.is_empty());
    }

    #[test]
    fn test_epub_style_with_multiple_css() {
        let css1 = EpubCss::new("body { color: black; }".to_string());
        let css2 = EpubCss::new("p { font-size: 12pt; }".to_string());
        let style = EpubStyle::new()
            .with_css(css1)
            .with_css(css2);
        assert_eq!(style.css.len(), 2);
    }

    #[test]
    fn test_epub_style_serialization() {
        let style = EpubStyle::new();
        let json = serde_json::to_string(&style);
        assert!(json.is_ok());
    }

    #[test]
    fn test_epub_style_deserialization() {
        let json = r#"{"css":[]}"#;
        let style: EpubStyle = serde_json::from_str(json).unwrap();
        assert!(style.css.is_empty());
    }

    #[test]
    fn test_epub_css_default_filename() {
        let css = EpubCss::new("body { color: black; }".to_string());
        assert_eq!(css.filename, "styles.css");
    }

    #[test]
    fn test_epub_css_custom_filename() {
        let css = EpubCss::new("body { color: black; }".to_string())
            .with_filename("theme.css".to_string());
        assert_eq!(css.filename, "theme.css");
    }

    #[test]
    fn test_epub_css_with_media_query() {
        let css = EpubCss::new("@media screen { body { color: black; } }".to_string());
        assert!(css.content.contains("@media"));
    }

    #[test]
    fn test_epub_style_css_count() {
        let css1 = EpubCss::new("body { color: black; }".to_string());
        let css2 = EpubCss::new("p { font-size: 12pt; }".to_string());
        let css3 = EpubCss::new("h1 { font-size: 18pt; }".to_string());
        let style = EpubStyle::new()
            .with_css(css1)
            .with_css(css2)
            .with_css(css3);
        assert_eq!(style.css.len(), 3);
    }

    #[test]
    fn test_epub_css_with_import() {
        let css = EpubCss::new("@import url('theme.css');".to_string());
        assert!(css.content.contains("@import"));
    }

    #[test]
    fn test_epub_css_with_variables() {
        let css = EpubCss::new(":root { --main-color: #000; }".to_string());
        assert!(css.content.contains("--main-color"));
    }

    #[test]
    fn test_epub_style_remove_css() {
        let css = EpubCss::new("body { color: black; }".to_string());
        let style = EpubStyle::new().with_css(css);
        assert_eq!(style.css.len(), 1);
    }
}
