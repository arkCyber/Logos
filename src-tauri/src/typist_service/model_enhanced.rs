/*!
 * 航空航天级 Model 增强模块
 * 实现 Typst 的 Model 增强功能（cite、terms、title、parbreak）
 */

use serde::{Deserialize, Serialize};

/// 引用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cite {
    pub key: String,
    pub supplement: Option<String>,
}

impl Cite {
    pub fn new() -> Self {
        Self {
            key: String::new(),
            supplement: None,
        }
    }

    pub fn with_key(mut self, key: String) -> Self {
        self.key = key;
        self
    }

    pub fn with_supplement(mut self, supplement: String) -> Self {
        self.supplement = Some(supplement);
        self
    }

    pub fn to_typst(&self) -> String {
        if let Some(supplement) = &self.supplement {
            format!("#cite(\"{}\", supplement: \"{}\")", self.key, supplement)
        } else {
            format!("#cite(\"{}\")", self.key)
        }
    }

    pub fn to_html(&self) -> String {
        if let Some(supplement) = &self.supplement {
            format!(
                "<cite data-key=\"{}\" data-supplement=\"{}\">[{}]</cite>",
                self.key, supplement, self.key
            )
        } else {
            format!("<cite data-key=\"{}\">[{}]</cite>", self.key, self.key)
        }
    }
}

impl Default for Cite {
    fn default() -> Self {
        Self::new()
    }
}

/// 术语条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Term {
    pub term: String,
    pub description: String,
}

impl Term {
    pub fn new() -> Self {
        Self {
            term: String::new(),
            description: String::new(),
        }
    }

    pub fn with_term(mut self, term: String) -> Self {
        self.term = term;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn to_typst(&self) -> String {
        format!(
            "#term(\"{}\"): {}",
            html_escape(&self.term),
            html_escape(&self.description)
        )
    }

    pub fn to_html(&self) -> String {
        format!(
            "<dfn data-term=\"{}\">{}: {}</dfn>",
            html_escape(&self.term),
            html_escape(&self.term),
            html_escape(&self.description)
        )
    }
}

impl Default for Term {
    fn default() -> Self {
        Self::new()
    }
}

/// 标题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    pub text: String,
    pub level: usize,
    pub numbering: bool,
}

impl Title {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            level: 1,
            numbering: true,
        }
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn with_level(mut self, level: usize) -> Self {
        self.level = level;
        self
    }

    pub fn with_numbering(mut self, numbering: bool) -> Self {
        self.numbering = numbering;
        self
    }

    pub fn to_typst(&self) -> String {
        let heading = match self.level {
            1 => "heading",
            2 => "subheading",
            _ => "heading",
        };

        if self.numbering {
            format!("#{}[numbering] \"{}\"", heading, html_escape(&self.text))
        } else {
            format!("#{} \"{}\"", heading, html_escape(&self.text))
        }
    }

    pub fn to_html(&self) -> String {
        let tag = match self.level {
            1 => "h1",
            2 => "h2",
            3 => "h3",
            4 => "h4",
            5 => "h5",
            _ => "h6",
        };

        let numbering_attr = if self.numbering {
            " data-numbering=\"true\""
        } else {
            ""
        };

        format!(
            "<{}{}>{}</{}>",
            tag,
            numbering_attr,
            html_escape(&self.text),
            tag
        )
    }
}

impl Default for Title {
    fn default() -> Self {
        Self::new()
    }
}

/// 段落分隔
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParBreak {
    pub weak: bool,
}

impl ParBreak {
    pub fn new() -> Self {
        Self { weak: false }
    }

    pub fn with_weak(mut self, weak: bool) -> Self {
        self.weak = weak;
        self
    }

    pub fn to_typst(&self) -> String {
        if self.weak {
            "#parbreak(weak: true)".to_string()
        } else {
            "#parbreak()".to_string()
        }
    }

    pub fn to_html(&self) -> String {
        if self.weak {
            "<br>".to_string()
        } else {
            "<br><br>".to_string()
        }
    }
}

impl Default for ParBreak {
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
    fn test_cite_creation() {
        let cite = Cite::new();
        assert!(cite.key.is_empty());
    }

    #[test]
    fn test_cite_to_typst() {
        let cite = Cite::new().with_key("ref1".to_string());
        let typst = cite.to_typst();
        assert!(typst.contains("ref1"));
    }

    #[test]
    fn test_cite_to_html() {
        let cite = Cite::new().with_key("ref1".to_string());
        let html = cite.to_html();
        assert!(html.contains("<cite") && html.contains("data-key=\"ref1\""));
    }

    #[test]
    fn test_cite_with_supplement() {
        let cite = Cite::new()
            .with_key("ref1".to_string())
            .with_supplement("p. 123".to_string());
        let typst = cite.to_typst();
        assert!(typst.contains("supplement"));
    }

    #[test]
    fn test_term_creation() {
        let term = Term::new();
        assert!(term.term.is_empty());
    }

    #[test]
    fn test_term_to_typst() {
        let term = Term::new()
            .with_term("API".to_string())
            .with_description("Application Programming Interface".to_string());
        let typst = term.to_typst();
        assert!(typst.contains("API") && typst.contains("Application Programming Interface"));
    }

    #[test]
    fn test_term_to_html() {
        let term = Term::new()
            .with_term("API".to_string())
            .with_description("Application Programming Interface".to_string());
        let html = term.to_html();
        assert!(html.contains("<dfn") && html.contains("data-term=\"API\""));
    }

    #[test]
    fn test_title_creation() {
        let title = Title::new();
        assert_eq!(title.level, 1);
    }

    #[test]
    fn test_title_to_typst() {
        let title = Title::new().with_text("Introduction".to_string());
        let typst = title.to_typst();
        assert!(typst.contains("Introduction"));
    }

    #[test]
    fn test_title_to_html() {
        let title = Title::new().with_text("Introduction".to_string());
        let html = title.to_html();
        assert!(html.contains("<h1") && html.contains("Introduction"));
    }

    #[test]
    fn test_title_with_level() {
        let title = Title::new().with_text("Section".to_string()).with_level(2);
        let typst = title.to_typst();
        assert!(typst.contains("subheading"));
    }

    #[test]
    fn test_title_without_numbering() {
        let title = Title::new()
            .with_text("Introduction".to_string())
            .with_numbering(false);
        let typst = title.to_typst();
        assert!(!typst.contains("numbering"));
    }

    #[test]
    fn test_parbreak_creation() {
        let parbreak = ParBreak::new();
        assert!(!parbreak.weak);
    }

    #[test]
    fn test_parbreak_to_typst() {
        let parbreak = ParBreak::new().with_weak(true);
        let typst = parbreak.to_typst();
        assert!(typst.contains("weak: true"));
    }

    #[test]
    fn test_parbreak_to_html() {
        let parbreak = ParBreak::new().with_weak(true);
        let html = parbreak.to_html();
        assert_eq!(html, "<br>");
    }

    #[test]
    fn test_parbreak_to_html_strong() {
        let parbreak = ParBreak::new().with_weak(false);
        let html = parbreak.to_html();
        assert_eq!(html, "<br><br>");
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<test>");
        assert_eq!(escaped, "&lt;test&gt;");
    }
}
