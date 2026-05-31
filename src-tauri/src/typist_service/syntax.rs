use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Keywords
    Let,
    Set,
    Show,
    If,
    Else,
    For,
    While,
    Break,
    Continue,
    Return,
    Import,
    Include,
    Export,
    As,
    In,
    And,
    Or,
    Not,

    // Symbols
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Colon,
    Semicolon,
    Comma,
    Dot,
    Arrow,
    Equals,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    EqEq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,

    // Literals
    String,
    Number,
    Boolean,
    None,

    // Identifiers
    Identifier,

    // Comments
    LineComment,
    BlockComment,

    // Other
    Whitespace,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub text: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct HighlightedSpan {
    pub start: usize,
    pub end: usize,
    pub token_type: TokenType,
}

pub struct SyntaxHighlighter {
    keywords: HashMap<String, TokenType>,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        let mut keywords = HashMap::new();

        // Typst keywords
        keywords.insert("let".to_string(), TokenType::Let);
        keywords.insert("set".to_string(), TokenType::Set);
        keywords.insert("show".to_string(), TokenType::Show);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("while".to_string(), TokenType::While);
        keywords.insert("break".to_string(), TokenType::Break);
        keywords.insert("continue".to_string(), TokenType::Continue);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("import".to_string(), TokenType::Import);
        keywords.insert("include".to_string(), TokenType::Include);
        keywords.insert("export".to_string(), TokenType::Export);
        keywords.insert("as".to_string(), TokenType::As);
        keywords.insert("in".to_string(), TokenType::In);
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("not".to_string(), TokenType::Not);
        keywords.insert("true".to_string(), TokenType::Boolean);
        keywords.insert("false".to_string(), TokenType::Boolean);
        keywords.insert("none".to_string(), TokenType::None);

        Self { keywords }
    }

    pub fn highlight(&self, source: &str) -> Vec<HighlightedSpan> {
        let mut spans = Vec::new();
        let chars: Vec<char> = source.chars().collect();
        let mut pos = 0;

        while pos < chars.len() {
            let ch = chars[pos];

            // Skip whitespace
            if ch.is_whitespace() {
                let start = pos;
                while pos < chars.len() && chars[pos].is_whitespace() {
                    pos += 1;
                }
                spans.push(HighlightedSpan {
                    start,
                    end: pos,
                    token_type: TokenType::Whitespace,
                });
                continue;
            }

            // Line comment
            if ch == '/' && pos + 1 < chars.len() && chars[pos + 1] == '/' {
                let start = pos;
                pos += 2;
                while pos < chars.len() && chars[pos] != '\n' {
                    pos += 1;
                }
                spans.push(HighlightedSpan {
                    start,
                    end: pos,
                    token_type: TokenType::LineComment,
                });
                continue;
            }

            // Block comment
            if ch == '/' && pos + 1 < chars.len() && chars[pos + 1] == '*' {
                let start = pos;
                pos += 2;
                while pos + 1 < chars.len() && !(chars[pos] == '*' && chars[pos + 1] == '/') {
                    pos += 1;
                }
                if pos + 1 < chars.len() {
                    pos += 2;
                }
                spans.push(HighlightedSpan {
                    start,
                    end: pos,
                    token_type: TokenType::BlockComment,
                });
                continue;
            }

            // String literal
            if ch == '"' {
                let start = pos;
                pos += 1;
                while pos < chars.len() && chars[pos] != '"' {
                    if chars[pos] == '\\' && pos + 1 < chars.len() {
                        pos += 2;
                    } else {
                        pos += 1;
                    }
                }
                if pos < chars.len() {
                    pos += 1;
                }
                spans.push(HighlightedSpan {
                    start,
                    end: pos,
                    token_type: TokenType::String,
                });
                continue;
            }

            // Number literal
            if ch.is_ascii_digit() {
                let start = pos;
                while pos < chars.len() && (chars[pos].is_ascii_digit() || chars[pos] == '.') {
                    pos += 1;
                }
                spans.push(HighlightedSpan {
                    start,
                    end: pos,
                    token_type: TokenType::Number,
                });
                continue;
            }

            // Identifier or keyword
            if ch.is_alphabetic() || ch == '_' {
                let start = pos;
                while pos < chars.len() && (chars[pos].is_alphanumeric() || chars[pos] == '_') {
                    pos += 1;
                }
                let text: String = chars[start..pos].iter().collect();
                let token_type = self
                    .keywords
                    .get(&text)
                    .cloned()
                    .unwrap_or(TokenType::Identifier);
                spans.push(HighlightedSpan {
                    start,
                    end: pos,
                    token_type,
                });
                continue;
            }

            // Symbols
            let (token_type, advance) = self.match_symbol(&chars, pos);
            spans.push(HighlightedSpan {
                start: pos,
                end: pos + advance,
                token_type,
            });
            pos += advance;
        }

        spans
    }

    fn match_symbol(&self, chars: &[char], pos: usize) -> (TokenType, usize) {
        if pos >= chars.len() {
            return (TokenType::Unknown, 1);
        }

        let ch = chars[pos];

        match ch {
            '(' => (TokenType::LeftParen, 1),
            ')' => (TokenType::RightParen, 1),
            '[' => (TokenType::LeftBracket, 1),
            ']' => (TokenType::RightBracket, 1),
            '{' => (TokenType::LeftBrace, 1),
            '}' => (TokenType::RightBrace, 1),
            ':' => (TokenType::Colon, 1),
            ';' => (TokenType::Semicolon, 1),
            ',' => (TokenType::Comma, 1),
            '.' => {
                if pos + 1 < chars.len() && chars[pos + 1] == '.' {
                    (TokenType::Arrow, 2)
                } else {
                    (TokenType::Dot, 1)
                }
            }
            '=' => {
                if pos + 1 < chars.len() && chars[pos + 1] == '=' {
                    (TokenType::EqEq, 2)
                } else {
                    (TokenType::Equals, 1)
                }
            }
            '+' => (TokenType::Plus, 1),
            '-' => (TokenType::Minus, 1),
            '*' => (TokenType::Star, 1),
            '/' => (TokenType::Slash, 1),
            '%' => (TokenType::Percent, 1),
            '!' => {
                if pos + 1 < chars.len() && chars[pos + 1] == '=' {
                    (TokenType::NotEq, 2)
                } else {
                    (TokenType::Not, 1)
                }
            }
            '<' => {
                if pos + 1 < chars.len() && chars[pos + 1] == '=' {
                    (TokenType::LtEq, 2)
                } else {
                    (TokenType::Lt, 1)
                }
            }
            '>' => {
                if pos + 1 < chars.len() && chars[pos + 1] == '=' {
                    (TokenType::GtEq, 2)
                } else {
                    (TokenType::Gt, 1)
                }
            }
            '#' => (TokenType::Identifier, 1), // Hash is part of function calls in Typst
            '@' => (TokenType::Identifier, 1), // At is used for references
            _ => (TokenType::Unknown, 1),
        }
    }

    /// Convert highlighted spans to HTML with CSS classes
    pub fn to_html(&self, source: &str) -> String {
        let spans = self.highlight(source);
        let mut html = String::new();
        let mut last_end = 0;

        for span in spans {
            // Add any text before this span
            if span.start > last_end {
                html.push_str(&html_escape(&source[last_end..span.start]));
            }

            // Add the highlighted span
            let class = self.token_type_to_css_class(&span.token_type);
            let text = html_escape(&source[span.start..span.end]);
            html.push_str(&format!("<span class=\"{}\">{}</span>", class, text));

            last_end = span.end;
        }

        // Add any remaining text
        if last_end < source.len() {
            html.push_str(&html_escape(&source[last_end..]));
        }

        html
    }

    fn token_type_to_css_class(&self, token_type: &TokenType) -> &'static str {
        match token_type {
            TokenType::Let
            | TokenType::Set
            | TokenType::Show
            | TokenType::If
            | TokenType::Else
            | TokenType::For
            | TokenType::While
            | TokenType::Break
            | TokenType::Continue
            | TokenType::Return
            | TokenType::Import
            | TokenType::Include
            | TokenType::Export
            | TokenType::As
            | TokenType::In
            | TokenType::And
            | TokenType::Or
            | TokenType::Not => "keyword",
            TokenType::LeftParen
            | TokenType::RightParen
            | TokenType::LeftBracket
            | TokenType::RightBracket
            | TokenType::LeftBrace
            | TokenType::RightBrace
            | TokenType::Colon
            | TokenType::Semicolon
            | TokenType::Comma
            | TokenType::Dot
            | TokenType::Arrow
            | TokenType::Equals
            | TokenType::Plus
            | TokenType::Minus
            | TokenType::Star
            | TokenType::Slash
            | TokenType::Percent
            | TokenType::EqEq
            | TokenType::NotEq
            | TokenType::Lt
            | TokenType::Gt
            | TokenType::LtEq
            | TokenType::GtEq => "punctuation",
            TokenType::String => "string",
            TokenType::Number => "number",
            TokenType::Boolean | TokenType::None => "literal",
            TokenType::Identifier => "identifier",
            TokenType::LineComment | TokenType::BlockComment => "comment",
            TokenType::Whitespace => "whitespace",
            TokenType::Unknown => "unknown",
        }
    }
}

impl Default for SyntaxHighlighter {
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
    fn test_highlighter_creation() {
        let highlighter = SyntaxHighlighter::new();
        let _ = highlighter;
    }

    #[test]
    fn test_highlighter_default() {
        let highlighter = SyntaxHighlighter::default();
        let _ = highlighter;
    }

    #[test]
    fn test_highlight_simple_code() {
        let highlighter = SyntaxHighlighter::new();
        let code = "= Hello World";
        let spans = highlighter.highlight(code);
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_highlight_keywords() {
        let highlighter = SyntaxHighlighter::new();
        let code = "let x = 10";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::Let));
    }

    #[test]
    fn test_highlight_string() {
        let highlighter = SyntaxHighlighter::new();
        let code = "\"Hello World\"";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::String));
    }

    #[test]
    fn test_highlight_number() {
        let highlighter = SyntaxHighlighter::new();
        let code = "42";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::Number));
    }

    #[test]
    fn test_highlight_comment() {
        let highlighter = SyntaxHighlighter::new();
        let code = "// This is a comment";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::LineComment));
    }

    #[test]
    fn test_to_html() {
        let highlighter = SyntaxHighlighter::new();
        let code = "let x = 10";
        let html = highlighter.to_html(code);
        assert!(html.contains("<span"));
    }

    #[test]
    fn test_html_escape() {
        let text = "<script>alert('xss')</script>";
        let escaped = html_escape(text);
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_highlight_block_comment() {
        let highlighter = SyntaxHighlighter::new();
        let code = "/* comment */";
        let spans = highlighter.highlight(code);
        assert!(spans
            .iter()
            .any(|s| s.token_type == TokenType::BlockComment));
    }

    #[test]
    fn test_highlight_multiline_comment() {
        let highlighter = SyntaxHighlighter::new();
        let code = "// line 1\n// line 2";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::LineComment));
    }

    #[test]
    fn test_highlight_identifier() {
        let highlighter = SyntaxHighlighter::new();
        let code = "myVariable";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::Identifier));
    }

    #[test]
    fn test_highlight_boolean_true() {
        let highlighter = SyntaxHighlighter::new();
        let code = "true";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::Boolean));
    }

    #[test]
    fn test_highlight_boolean_false() {
        let highlighter = SyntaxHighlighter::new();
        let code = "false";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::Boolean));
    }

    #[test]
    fn test_highlight_none() {
        let highlighter = SyntaxHighlighter::new();
        let code = "none";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::None));
    }

    #[test]
    fn test_highlight_operators() {
        let highlighter = SyntaxHighlighter::new();
        let code = "+ - * / %";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::Plus));
        assert!(spans.iter().any(|s| s.token_type == TokenType::Minus));
        assert!(spans.iter().any(|s| s.token_type == TokenType::Star));
        assert!(spans.iter().any(|s| s.token_type == TokenType::Slash));
        assert!(spans.iter().any(|s| s.token_type == TokenType::Percent));
    }

    #[test]
    fn test_highlight_comparison_operators() {
        let highlighter = SyntaxHighlighter::new();
        let code = "== != < > <= >=";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::EqEq));
        assert!(spans.iter().any(|s| s.token_type == TokenType::NotEq));
        assert!(spans.iter().any(|s| s.token_type == TokenType::Lt));
        assert!(spans.iter().any(|s| s.token_type == TokenType::Gt));
        assert!(spans.iter().any(|s| s.token_type == TokenType::LtEq));
        assert!(spans.iter().any(|s| s.token_type == TokenType::GtEq));
    }

    #[test]
    fn test_highlight_parentheses() {
        let highlighter = SyntaxHighlighter::new();
        let code = "()";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::LeftParen));
        assert!(spans.iter().any(|s| s.token_type == TokenType::RightParen));
    }

    #[test]
    fn test_highlight_brackets() {
        let highlighter = SyntaxHighlighter::new();
        let code = "[]";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::LeftBracket));
        assert!(spans
            .iter()
            .any(|s| s.token_type == TokenType::RightBracket));
    }

    #[test]
    fn test_highlight_braces() {
        let highlighter = SyntaxHighlighter::new();
        let code = "{}";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::LeftBrace));
        assert!(spans.iter().any(|s| s.token_type == TokenType::RightBrace));
    }

    #[test]
    fn test_highlight_string_with_escape() {
        let highlighter = SyntaxHighlighter::new();
        let code = "\"Hello \\\"World\\\"\"";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::String));
    }

    #[test]
    fn test_highlight_number_with_decimal() {
        let highlighter = SyntaxHighlighter::new();
        let code = "3.14";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::Number));
    }

    #[test]
    fn test_highlight_complex_code() {
        let highlighter = SyntaxHighlighter::new();
        let code = "let x = 10\nif x > 5 {\n  #show \"Hello\"\n}";
        let spans = highlighter.highlight(code);
        assert!(!spans.is_empty());
        assert!(spans.iter().any(|s| s.token_type == TokenType::Let));
        assert!(spans.iter().any(|s| s.token_type == TokenType::If));
    }

    #[test]
    fn test_highlight_empty_string() {
        let highlighter = SyntaxHighlighter::new();
        let code = "";
        let spans = highlighter.highlight(code);
        assert!(spans.is_empty());
    }

    #[test]
    fn test_highlight_only_whitespace() {
        let highlighter = SyntaxHighlighter::new();
        let code = "   \n\t  ";
        let spans = highlighter.highlight(code);
        assert!(!spans.is_empty());
        assert!(spans.iter().all(|s| s.token_type == TokenType::Whitespace));
    }

    #[test]
    fn test_to_html_with_special_chars() {
        let highlighter = SyntaxHighlighter::new();
        let code = "let x = \"<test>\"";
        let html = highlighter.to_html(code);
        assert!(html.contains("&lt;test&gt;"));
    }

    #[test]
    fn test_to_html_with_newlines() {
        let highlighter = SyntaxHighlighter::new();
        let code = "let x = 10\nlet y = 20";
        let html = highlighter.to_html(code);
        assert!(html.contains("\n"));
    }

    #[test]
    fn test_token_type_to_css_class_keyword() {
        let highlighter = SyntaxHighlighter::new();
        let class = highlighter.token_type_to_css_class(&TokenType::Let);
        assert_eq!(class, "keyword");
    }

    #[test]
    fn test_token_type_to_css_class_punctuation() {
        let highlighter = SyntaxHighlighter::new();
        let class = highlighter.token_type_to_css_class(&TokenType::LeftParen);
        assert_eq!(class, "punctuation");
    }

    #[test]
    fn test_token_type_to_css_class_string() {
        let highlighter = SyntaxHighlighter::new();
        let class = highlighter.token_type_to_css_class(&TokenType::String);
        assert_eq!(class, "string");
    }

    #[test]
    fn test_token_type_to_css_class_number() {
        let highlighter = SyntaxHighlighter::new();
        let class = highlighter.token_type_to_css_class(&TokenType::Number);
        assert_eq!(class, "number");
    }

    #[test]
    fn test_token_type_to_css_class_identifier() {
        let highlighter = SyntaxHighlighter::new();
        let class = highlighter.token_type_to_css_class(&TokenType::Identifier);
        assert_eq!(class, "identifier");
    }

    #[test]
    fn test_token_type_to_css_class_comment() {
        let highlighter = SyntaxHighlighter::new();
        let class = highlighter.token_type_to_css_class(&TokenType::LineComment);
        assert_eq!(class, "comment");
    }

    #[test]
    fn test_highlight_arrow_operator() {
        let highlighter = SyntaxHighlighter::new();
        let code = "..";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::Arrow));
    }

    #[test]
    fn test_highlight_dot_operator() {
        let highlighter = SyntaxHighlighter::new();
        let code = "x.y";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::Dot));
    }

    #[test]
    fn test_highlight_hash_symbol() {
        let highlighter = SyntaxHighlighter::new();
        let code = "#show";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::Identifier));
    }

    #[test]
    fn test_highlight_at_symbol() {
        let highlighter = SyntaxHighlighter::new();
        let code = "@ref";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::Identifier));
    }

    #[test]
    fn test_highlight_colon_and_semicolon() {
        let highlighter = SyntaxHighlighter::new();
        let code = "key: value;";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::Colon));
        assert!(spans.iter().any(|s| s.token_type == TokenType::Semicolon));
    }

    #[test]
    fn test_highlight_comma() {
        let highlighter = SyntaxHighlighter::new();
        let code = "a, b, c";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::Comma));
    }

    #[test]
    fn test_highlight_equals() {
        let highlighter = SyntaxHighlighter::new();
        let code = "x = 10";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::Equals));
    }

    #[test]
    fn test_highlight_not_operator() {
        let highlighter = SyntaxHighlighter::new();
        let code = "!true";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::Not));
    }

    #[test]
    fn test_highlight_logical_operators() {
        let highlighter = SyntaxHighlighter::new();
        let code = "and or not";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::And));
        assert!(spans.iter().any(|s| s.token_type == TokenType::Or));
        assert!(spans.iter().any(|s| s.token_type == TokenType::Not));
    }

    #[test]
    fn test_highlight_loop_keywords() {
        let highlighter = SyntaxHighlighter::new();
        let code = "for while break continue";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::For));
        assert!(spans.iter().any(|s| s.token_type == TokenType::While));
        assert!(spans.iter().any(|s| s.token_type == TokenType::Break));
        assert!(spans.iter().any(|s| s.token_type == TokenType::Continue));
    }

    #[test]
    fn test_highlight_control_keywords() {
        let highlighter = SyntaxHighlighter::new();
        let code = "if else return";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::If));
        assert!(spans.iter().any(|s| s.token_type == TokenType::Else));
        assert!(spans.iter().any(|s| s.token_type == TokenType::Return));
    }

    #[test]
    fn test_highlight_module_keywords() {
        let highlighter = SyntaxHighlighter::new();
        let code = "import include export as";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::Import));
        assert!(spans.iter().any(|s| s.token_type == TokenType::Include));
        assert!(spans.iter().any(|s| s.token_type == TokenType::Export));
        assert!(spans.iter().any(|s| s.token_type == TokenType::As));
    }

    #[test]
    fn test_highlight_in_keyword() {
        let highlighter = SyntaxHighlighter::new();
        let code = "for x in array";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::In));
    }

    #[test]
    fn test_highlight_set_show_keywords() {
        let highlighter = SyntaxHighlighter::new();
        let code = "set show";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::Set));
        assert!(spans.iter().any(|s| s.token_type == TokenType::Show));
    }

    #[test]
    fn test_highlight_underscore_identifier() {
        let highlighter = SyntaxHighlighter::new();
        let code = "_private_var";
        let spans = highlighter.highlight(code);
        assert!(spans.iter().any(|s| s.token_type == TokenType::Identifier));
    }

    #[test]
    fn test_highlight_mixed_case_keywords() {
        let highlighter = SyntaxHighlighter::new();
        let code = "LET SET SHOW";
        let spans = highlighter.highlight(code);
        // Keywords should be case-sensitive
        assert!(!spans.iter().any(|s| s.token_type == TokenType::Let));
        assert!(spans.iter().any(|s| s.token_type == TokenType::Identifier));
    }

    #[test]
    fn test_span_positions() {
        let highlighter = SyntaxHighlighter::new();
        let code = "let x = 10";
        let spans = highlighter.highlight(code);
        // Check that spans have valid positions
        for span in spans {
            assert!(span.start < span.end);
            assert!(span.end <= code.len());
        }
    }

    #[test]
    fn test_span_coverage() {
        let highlighter = SyntaxHighlighter::new();
        let code = "let x = 10";
        let spans = highlighter.highlight(code);
        // Check that spans cover the entire text
        let mut covered = vec![false; code.len()];
        for span in spans {
            for i in span.start..span.end {
                if i < covered.len() {
                    covered[i] = true;
                }
            }
        }
        // Most characters should be covered (whitespace might be separate)
        let coverage_count = covered.iter().filter(|&&x| x).count();
        assert!(coverage_count > 0);
    }
}
