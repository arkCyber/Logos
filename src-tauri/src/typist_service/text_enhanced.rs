/*!
 * 航空航天级 Text 增强模块
 * 实现 Typst 的 Text 增强功能（highlight、linebreak、lorem、overline、strike）
 */

use serde::{Deserialize, Serialize};

/// 高亮文本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Highlight {
    pub text: String,
    pub color: String,
}

impl Highlight {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            color: "yellow".to_string(),
        }
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn with_color(mut self, color: String) -> Self {
        self.color = color;
        self
    }

    pub fn to_typst(&self) -> String {
        format!(
            "#highlight(text: \"{}\", fill: \"{}\")",
            html_escape(&self.text),
            self.color
        )
    }

    pub fn to_html(&self) -> String {
        format!(
            "<mark style=\"background-color: {}\">{}</mark>",
            self.color,
            html_escape(&self.text)
        )
    }
}

impl Default for Highlight {
    fn default() -> Self {
        Self::new()
    }
}

/// 换行类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LineBreakType {
    Weak,
    Strong,
}

/// 换行
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineBreak {
    pub break_type: LineBreakType,
}

impl LineBreak {
    pub fn new() -> Self {
        Self {
            break_type: LineBreakType::Weak,
        }
    }

    pub fn with_type(mut self, break_type: LineBreakType) -> Self {
        self.break_type = break_type;
        self
    }

    pub fn to_typst(&self) -> String {
        match self.break_type {
            LineBreakType::Weak => "#linebreak()".to_string(),
            LineBreakType::Strong => "#linebreak(weak: false)".to_string(),
        }
    }

    pub fn to_html(&self) -> String {
        match self.break_type {
            LineBreakType::Weak => "<br>".to_string(),
            LineBreakType::Strong => "<br><br>".to_string(),
        }
    }
}

impl Default for LineBreak {
    fn default() -> Self {
        Self::new()
    }
}

/// Lorem Ipsum 生成器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lorem {
    pub words: usize,
    pub paragraphs: usize,
}

impl Lorem {
    pub fn new() -> Self {
        Self {
            words: 100,
            paragraphs: 1,
        }
    }

    pub fn with_words(mut self, words: usize) -> Self {
        self.words = words;
        self
    }

    pub fn with_paragraphs(mut self, paragraphs: usize) -> Self {
        self.paragraphs = paragraphs;
        self
    }

    pub fn generate(&self) -> String {
        let lorem_words = vec![
            "lorem",
            "ipsum",
            "dolor",
            "sit",
            "amet",
            "consectetur",
            "adipiscing",
            "elit",
            "sed",
            "do",
            "eiusmod",
            "tempor",
            "incididunt",
            "ut",
            "labore",
            "et",
            "dolore",
            "magna",
            "aliqua",
            "enim",
            "ad",
            "minim",
            "veniam",
            "quis",
            "nostrud",
            "exercitation",
            "ullamco",
            "laboris",
            "nisi",
            "ut",
            "aliquip",
            "ex",
            "ea",
            "commodo",
            "consequat",
            "duis",
            "aute",
            "irure",
            "dolor",
            "in",
            "reprehenderit",
            "voluptate",
            "velit",
            "esse",
            "cillum",
            "dolore",
            "eu",
            "fugiat",
            "nulla",
            "pariatur",
            "excepteur",
            "sint",
            "occaecat",
            "cupidatat",
            "non",
            "proident",
            "sunt",
            "in",
            "culpa",
            "qui",
            "officia",
            "deserunt",
            "mollit",
            "anim",
            "id",
            "est",
            "laborum",
        ];

        let mut result = String::new();
        let mut word_count = 0;
        let words_per_paragraph = self.words / self.paragraphs.max(1);

        for p in 0..self.paragraphs {
            let mut paragraph_words = Vec::new();
            let target_words = if p == self.paragraphs - 1 {
                self.words - word_count
            } else {
                words_per_paragraph
            };

            for _ in 0..target_words {
                let word = lorem_words[word_count % lorem_words.len()];
                paragraph_words.push(word);
                word_count += 1;
            }

            result.push_str(&paragraph_words.join(" "));
            if p < self.paragraphs - 1 {
                result.push_str("\n\n");
            }
        }

        result
    }

    pub fn to_typst(&self) -> String {
        format!("#lorem({})", self.words)
    }
}

impl Default for Lorem {
    fn default() -> Self {
        Self::new()
    }
}

/// 上划线文本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Overline {
    pub text: String,
}

impl Overline {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn to_typst(&self) -> String {
        format!("#overline(\"{}\")", html_escape(&self.text))
    }

    pub fn to_html(&self) -> String {
        format!(
            "<span style=\"text-decoration: overline\">{}</span>",
            html_escape(&self.text)
        )
    }
}

impl Default for Overline {
    fn default() -> Self {
        Self::new()
    }
}

/// 删除线文本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Strike {
    pub text: String,
}

impl Strike {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn to_typst(&self) -> String {
        format!("#strike(\"{}\")", html_escape(&self.text))
    }

    pub fn to_html(&self) -> String {
        format!("<del>{}</del>", html_escape(&self.text))
    }
}

impl Default for Strike {
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
    fn test_highlight_creation() {
        let highlight = Highlight::new();
        assert_eq!(highlight.color, "yellow");
    }

    #[test]
    fn test_highlight_to_typst() {
        let highlight = Highlight::new()
            .with_text("test".to_string())
            .with_color("red".to_string());
        let typst = highlight.to_typst();
        assert!(typst.contains("test") && typst.contains("red"));
    }

    #[test]
    fn test_highlight_to_html() {
        let highlight = Highlight::new()
            .with_text("test".to_string())
            .with_color("yellow".to_string());
        let html = highlight.to_html();
        assert!(
            html.contains("<mark")
                && html.contains("background-color: yellow")
                && html.contains("test")
        );
    }

    #[test]
    fn test_linebreak_creation() {
        let linebreak = LineBreak::new();
        assert_eq!(linebreak.break_type, LineBreakType::Weak);
    }

    #[test]
    fn test_linebreak_to_typst() {
        let linebreak = LineBreak::new().with_type(LineBreakType::Strong);
        let typst = linebreak.to_typst();
        assert!(typst.contains("weak: false"));
    }

    #[test]
    fn test_linebreak_to_html() {
        let linebreak = LineBreak::new().with_type(LineBreakType::Weak);
        let html = linebreak.to_html();
        assert_eq!(html, "<br>");
    }

    #[test]
    fn test_linebreak_to_html_strong() {
        let linebreak = LineBreak::new().with_type(LineBreakType::Strong);
        let html = linebreak.to_html();
        assert_eq!(html, "<br><br>");
    }

    #[test]
    fn test_lorem_creation() {
        let lorem = Lorem::new();
        assert_eq!(lorem.words, 100);
    }

    #[test]
    fn test_lorem_generate() {
        let lorem = Lorem::new().with_words(10);
        let text = lorem.generate();
        let word_count = text.split_whitespace().count();
        assert_eq!(word_count, 10);
    }

    #[test]
    fn test_lorem_to_typst() {
        let lorem = Lorem::new().with_words(50);
        let typst = lorem.to_typst();
        assert!(typst.contains("50"));
    }

    #[test]
    fn test_overline_creation() {
        let overline = Overline::new();
        assert!(overline.text.is_empty());
    }

    #[test]
    fn test_overline_to_typst() {
        let overline = Overline::new().with_text("test".to_string());
        let typst = overline.to_typst();
        assert!(typst.contains("test"));
    }

    #[test]
    fn test_overline_to_html() {
        let overline = Overline::new().with_text("test".to_string());
        let html = overline.to_html();
        assert!(
            html.contains("<span")
                && html.contains("text-decoration: overline")
                && html.contains("test")
        );
    }

    #[test]
    fn test_strike_creation() {
        let strike = Strike::new();
        assert!(strike.text.is_empty());
    }

    #[test]
    fn test_strike_to_typst() {
        let strike = Strike::new().with_text("test".to_string());
        let typst = strike.to_typst();
        assert!(typst.contains("test"));
    }

    #[test]
    fn test_strike_to_html() {
        let strike = Strike::new().with_text("test".to_string());
        let html = strike.to_html();
        assert!(html.contains("<del>") && html.contains("test") && html.contains("</del>"));
    }

    #[test]
    fn test_linebreak_type_variants() {
        assert_eq!(LineBreakType::Weak, LineBreakType::Weak);
        assert_eq!(LineBreakType::Strong, LineBreakType::Strong);
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<test>");
        assert_eq!(escaped, "&lt;test&gt;");
    }
}
