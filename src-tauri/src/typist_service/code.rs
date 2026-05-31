/*!
 * 航空航天级代码块系统
 * 实现 Typst 的代码块功能（语法高亮、行号、主题、多语言支持）
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 代码语言
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CodeLanguage {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Java,
    Cpp,
    C,
    Go,
    Ruby,
    PHP,
    Swift,
    Kotlin,
    Scala,
    Haskell,
    Lisp,
    Scheme,
    Clojure,
    Elixir,
    Erlang,
    FSharp,
    OCaml,
    R,
    Matlab,
    Julia,
    Lua,
    Perl,
    Shell,
    Bash,
    PowerShell,
    SQL,
    HTML,
    CSS,
    XML,
    JSON,
    YAML,
    TOML,
    Markdown,
    LaTeX,
    Typst,
    PlainText,
    Custom(String),
}

impl CodeLanguage {
    pub fn extension(&self) -> &str {
        match self {
            CodeLanguage::Rust => "rs",
            CodeLanguage::Python => "py",
            CodeLanguage::JavaScript => "js",
            CodeLanguage::TypeScript => "ts",
            CodeLanguage::Java => "java",
            CodeLanguage::Cpp => "cpp",
            CodeLanguage::C => "c",
            CodeLanguage::Go => "go",
            CodeLanguage::Ruby => "rb",
            CodeLanguage::PHP => "php",
            CodeLanguage::Swift => "swift",
            CodeLanguage::Kotlin => "kt",
            CodeLanguage::Scala => "scala",
            CodeLanguage::Haskell => "hs",
            CodeLanguage::Lisp => "lisp",
            CodeLanguage::Scheme => "scm",
            CodeLanguage::Clojure => "clj",
            CodeLanguage::Elixir => "ex",
            CodeLanguage::Erlang => "erl",
            CodeLanguage::FSharp => "fs",
            CodeLanguage::OCaml => "ml",
            CodeLanguage::R => "r",
            CodeLanguage::Matlab => "m",
            CodeLanguage::Julia => "jl",
            CodeLanguage::Lua => "lua",
            CodeLanguage::Perl => "pl",
            CodeLanguage::Bash => "sh",
            CodeLanguage::PowerShell => "ps1",
            CodeLanguage::SQL => "sql",
            CodeLanguage::HTML => "html",
            CodeLanguage::CSS => "css",
            CodeLanguage::XML => "xml",
            CodeLanguage::JSON => "json",
            CodeLanguage::YAML => "yaml",
            CodeLanguage::TOML => "toml",
            CodeLanguage::Markdown => "md",
            CodeLanguage::LaTeX => "tex",
            CodeLanguage::Typst => "typ",
            CodeLanguage::PlainText => "txt",
            CodeLanguage::Shell => "sh",
            CodeLanguage::Custom(_) => "txt",
        }
    }
}

use std::str::FromStr;

impl FromStr for CodeLanguage {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rust" => Ok(CodeLanguage::Rust),
            "py" | "python" => Ok(CodeLanguage::Python),
            "js" | "javascript" => Ok(CodeLanguage::JavaScript),
            "ts" | "typescript" => Ok(CodeLanguage::TypeScript),
            "java" => Ok(CodeLanguage::Java),
            "cpp" | "c++" => Ok(CodeLanguage::Cpp),
            "c" => Ok(CodeLanguage::C),
            "go" | "golang" => Ok(CodeLanguage::Go),
            "rb" | "ruby" => Ok(CodeLanguage::Ruby),
            "php" => Ok(CodeLanguage::PHP),
            "swift" => Ok(CodeLanguage::Swift),
            "kt" | "kotlin" => Ok(CodeLanguage::Kotlin),
            "scala" => Ok(CodeLanguage::Scala),
            "hs" | "haskell" => Ok(CodeLanguage::Haskell),
            "lisp" => Ok(CodeLanguage::Lisp),
            "scheme" => Ok(CodeLanguage::Scheme),
            "clj" | "clojure" => Ok(CodeLanguage::Clojure),
            "ex" | "elixir" => Ok(CodeLanguage::Elixir),
            "erl" | "erlang" => Ok(CodeLanguage::Erlang),
            "fs" | "fsharp" => Ok(CodeLanguage::FSharp),
            "ml" | "ocaml" => Ok(CodeLanguage::OCaml),
            "r" => Ok(CodeLanguage::R),
            "matlab" => Ok(CodeLanguage::Matlab),
            "jl" | "julia" => Ok(CodeLanguage::Julia),
            "lua" => Ok(CodeLanguage::Lua),
            "pl" | "perl" => Ok(CodeLanguage::Perl),
            "sh" | "shell" | "bash" => Ok(CodeLanguage::Bash),
            "ps1" | "powershell" => Ok(CodeLanguage::PowerShell),
            "sql" => Ok(CodeLanguage::SQL),
            "html" => Ok(CodeLanguage::HTML),
            "css" => Ok(CodeLanguage::CSS),
            "xml" => Ok(CodeLanguage::XML),
            "json" => Ok(CodeLanguage::JSON),
            "yaml" | "yml" => Ok(CodeLanguage::YAML),
            "toml" => Ok(CodeLanguage::TOML),
            "md" | "markdown" => Ok(CodeLanguage::Markdown),
            "tex" | "latex" => Ok(CodeLanguage::LaTeX),
            "typ" | "typst" => Ok(CodeLanguage::Typst),
            "txt" | "text" | "plain" => Ok(CodeLanguage::PlainText),
            _ => Ok(CodeLanguage::Custom(s.to_string())),
        }
    }
}

/// 代码主题
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CodeTheme {
    /// 明亮主题
    Light,
    /// 暗色主题
    Dark,
    /// 高对比度主题
    HighContrast,
    /// 自定义主题
    Custom(String),
}

/// 代码块配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlockConfig {
    pub language: CodeLanguage,
    pub theme: CodeTheme,
    pub show_line_numbers: bool,
    pub line_number_start: usize,
    pub highlight_lines: Vec<usize>,
    pub wrap_lines: bool,
    pub tab_size: usize,
}

impl Default for CodeBlockConfig {
    fn default() -> Self {
        Self {
            language: CodeLanguage::PlainText,
            theme: CodeTheme::Light,
            show_line_numbers: false,
            line_number_start: 1,
            highlight_lines: Vec::new(),
            wrap_lines: false,
            tab_size: 4,
        }
    }
}

/// 代码块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlock {
    pub content: String,
    pub config: CodeBlockConfig,
}

impl CodeBlock {
    pub fn new(content: String) -> Self {
        Self {
            content,
            config: CodeBlockConfig::default(),
        }
    }

    pub fn with_language(mut self, language: CodeLanguage) -> Self {
        self.config.language = language;
        self
    }

    pub fn with_theme(mut self, theme: CodeTheme) -> Self {
        self.config.theme = theme;
        self
    }

    pub fn with_line_numbers(mut self, show: bool) -> Self {
        self.config.show_line_numbers = show;
        self
    }

    pub fn with_line_number_start(mut self, start: usize) -> Self {
        self.config.line_number_start = start;
        self
    }

    pub fn with_highlight_lines(mut self, lines: Vec<usize>) -> Self {
        self.config.highlight_lines = lines;
        self
    }

    pub fn with_wrap_lines(mut self, wrap: bool) -> Self {
        self.config.wrap_lines = wrap;
        self
    }

    pub fn with_tab_size(mut self, size: usize) -> Self {
        self.config.tab_size = size;
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        typst.push_str("```");
        typst.push_str(self.config.language.extension());
        typst.push('\n');
        typst.push_str(&self.content);
        typst.push_str("\n```\n");

        typst
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        let theme_class = match self.config.theme {
            CodeTheme::Light => "code-light",
            CodeTheme::Dark => "code-dark",
            CodeTheme::HighContrast => "code-high-contrast",
            CodeTheme::Custom(ref name) => name,
        };

        html.push_str(&format!("<div class=\"code-block {}\">\n", theme_class));

        if self.config.show_line_numbers {
            html.push_str("<div class=\"code-line-numbers\">\n");
            let lines: Vec<&str> = self.content.lines().collect();
            for (i, _) in lines.iter().enumerate() {
                let line_num = i + self.config.line_number_start;
                let highlight = if self.config.highlight_lines.contains(&line_num) {
                    " highlighted"
                } else {
                    ""
                };
                html.push_str(&format!(
                    "  <span class=\"line-number{}\">{}</span>\n",
                    highlight, line_num
                ));
            }
            html.push_str("</div>\n");
        }

        html.push_str("<pre class=\"code-content\"><code class=\"language-");
        html.push_str(self.config.language.extension());
        html.push_str("\">\n");

        for (i, line) in self.content.lines().enumerate() {
            let line_num = i + self.config.line_number_start;
            let highlight = if self.config.highlight_lines.contains(&line_num) {
                " highlighted"
            } else {
                ""
            };
            html.push_str(&format!(
                "  <div class=\"code-line{}\">{}</div>\n",
                highlight,
                html_escape(line)
            ));
        }

        html.push_str("</code></pre>\n");
        html.push_str("</div>\n");

        html
    }

    /// 获取行数
    pub fn line_count(&self) -> usize {
        self.content.lines().count()
    }

    /// 获取指定行
    pub fn get_line(&self, line_number: usize) -> Option<String> {
        self.content
            .lines()
            .nth(line_number.saturating_sub(1))
            .map(|s| s.to_string())
    }
}

/// 代码块构建器
pub struct CodeBlockBuilder {
    block: CodeBlock,
}

impl CodeBlockBuilder {
    pub fn new(content: String) -> Self {
        Self {
            block: CodeBlock::new(content),
        }
    }

    pub fn language(mut self, language: CodeLanguage) -> Self {
        self.block = self.block.with_language(language);
        self
    }

    pub fn theme(mut self, theme: CodeTheme) -> Self {
        self.block = self.block.with_theme(theme);
        self
    }

    pub fn line_numbers(mut self, show: bool) -> Self {
        self.block = self.block.with_line_numbers(show);
        self
    }

    pub fn line_number_start(mut self, start: usize) -> Self {
        self.block = self.block.with_line_number_start(start);
        self
    }

    pub fn highlight_lines(mut self, lines: Vec<usize>) -> Self {
        self.block = self.block.with_highlight_lines(lines);
        self
    }

    pub fn wrap_lines(mut self, wrap: bool) -> Self {
        self.block = self.block.with_wrap_lines(wrap);
        self
    }

    pub fn tab_size(mut self, size: usize) -> Self {
        self.block = self.block.with_tab_size(size);
        self
    }

    pub fn build(self) -> CodeBlock {
        self.block
    }
}

/// 代码块管理器
pub struct CodeBlockManager {
    blocks: HashMap<String, CodeBlock>,
}

impl CodeBlockManager {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, id: String, block: CodeBlock) {
        self.blocks.insert(id, block);
    }

    pub fn get_block(&self, id: &str) -> Option<&CodeBlock> {
        self.blocks.get(id)
    }

    pub fn remove_block(&mut self, id: &str) -> Option<CodeBlock> {
        self.blocks.remove(id)
    }

    pub fn get_all_blocks(&self) -> Vec<&CodeBlock> {
        self.blocks.values().collect()
    }
}

impl Default for CodeBlockManager {
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
    fn test_code_block_creation() {
        let block = CodeBlock::new("let x = 10;".to_string());
        assert_eq!(block.content, "let x = 10;");
    }

    #[test]
    fn test_code_block_default() {
        let block = CodeBlock::new("test".to_string());
        assert_eq!(block.config.language, CodeLanguage::PlainText);
        assert_eq!(block.config.theme, CodeTheme::Light);
        assert!(!block.config.show_line_numbers);
    }

    #[test]
    fn test_code_block_with_language() {
        let block = CodeBlock::new("let x = 10;".to_string()).with_language(CodeLanguage::Rust);
        assert_eq!(block.config.language, CodeLanguage::Rust);
    }

    #[test]
    fn test_code_block_with_theme() {
        let block = CodeBlock::new("let x = 10;".to_string()).with_theme(CodeTheme::Dark);
        assert_eq!(block.config.theme, CodeTheme::Dark);
    }

    #[test]
    fn test_code_block_with_line_numbers() {
        let block = CodeBlock::new("let x = 10;".to_string()).with_line_numbers(true);
        assert!(block.config.show_line_numbers);
    }

    #[test]
    fn test_code_block_with_line_number_start() {
        let block = CodeBlock::new("let x = 10;".to_string()).with_line_number_start(10);
        assert_eq!(block.config.line_number_start, 10);
    }

    #[test]
    fn test_code_block_with_highlight_lines() {
        let block = CodeBlock::new("let x = 10;".to_string()).with_highlight_lines(vec![1, 2, 3]);
        assert_eq!(block.config.highlight_lines, vec![1, 2, 3]);
    }

    #[test]
    fn test_code_block_with_wrap_lines() {
        let block = CodeBlock::new("let x = 10;".to_string()).with_wrap_lines(true);
        assert!(block.config.wrap_lines);
    }

    #[test]
    fn test_code_block_with_tab_size() {
        let block = CodeBlock::new("let x = 10;".to_string()).with_tab_size(8);
        assert_eq!(block.config.tab_size, 8);
    }

    #[test]
    fn test_to_typst() {
        let block = CodeBlock::new("let x = 10;".to_string()).with_language(CodeLanguage::Rust);
        let typst = block.to_typst();
        assert!(typst.contains("```rs"));
        assert!(typst.contains("let x = 10;"));
    }

    #[test]
    fn test_to_html() {
        let block = CodeBlock::new("let x = 10;".to_string()).with_language(CodeLanguage::Rust);
        let html = block.to_html();
        assert!(html.contains("<div class=\"code-block"));
        assert!(html.contains("language-rs"));
        assert!(html.contains("let x = 10;"));
    }

    #[test]
    fn test_to_html_with_line_numbers() {
        let block = CodeBlock::new("line1\nline2".to_string()).with_line_numbers(true);
        let html = block.to_html();
        assert!(html.contains("code-line-numbers"));
        assert!(html.contains("line-number"));
    }

    #[test]
    fn test_to_html_with_highlight() {
        let block = CodeBlock::new("line1\nline2".to_string())
            .with_line_numbers(true)
            .with_highlight_lines(vec![1]);
        let html = block.to_html();
        assert!(html.contains("highlighted"));
    }

    #[test]
    fn test_line_count() {
        let block = CodeBlock::new("line1\nline2\nline3".to_string());
        assert_eq!(block.line_count(), 3);
    }

    #[test]
    fn test_get_line() {
        let block = CodeBlock::new("line1\nline2\nline3".to_string());
        assert_eq!(block.get_line(1), Some("line1".to_string()));
        assert_eq!(block.get_line(2), Some("line2".to_string()));
        assert_eq!(block.get_line(3), Some("line3".to_string()));
        assert_eq!(block.get_line(4), None);
    }

    #[test]
    fn test_code_language_from_str() {
        assert_eq!(CodeLanguage::from_str("rust"), Ok(CodeLanguage::Rust));
        assert_eq!(CodeLanguage::from_str("python"), Ok(CodeLanguage::Python));
        assert_eq!(CodeLanguage::from_str("js"), Ok(CodeLanguage::JavaScript));
        assert_eq!(
            CodeLanguage::from_str("unknown"),
            Ok(CodeLanguage::Custom("unknown".to_string()))
        );
    }

    #[test]
    fn test_code_language_extension() {
        assert_eq!(CodeLanguage::Rust.extension(), "rs");
        assert_eq!(CodeLanguage::Python.extension(), "py");
        assert_eq!(CodeLanguage::JavaScript.extension(), "js");
        assert_eq!(CodeLanguage::Typst.extension(), "typ");
    }

    #[test]
    fn test_code_theme_partial_eq() {
        assert_eq!(CodeTheme::Light, CodeTheme::Light);
        assert_ne!(CodeTheme::Light, CodeTheme::Dark);
    }

    #[test]
    fn test_code_language_partial_eq() {
        assert_eq!(CodeLanguage::Rust, CodeLanguage::Rust);
        assert_ne!(CodeLanguage::Rust, CodeLanguage::Python);
    }

    #[test]
    fn test_code_block_builder() {
        let block = CodeBlockBuilder::new("let x = 10;".to_string())
            .language(CodeLanguage::Rust)
            .theme(CodeTheme::Dark)
            .line_numbers(true)
            .build();

        assert_eq!(block.config.language, CodeLanguage::Rust);
        assert_eq!(block.config.theme, CodeTheme::Dark);
        assert!(block.config.show_line_numbers);
    }

    #[test]
    fn test_code_block_manager() {
        let mut manager = CodeBlockManager::new();
        let block = CodeBlock::new("test".to_string());
        manager.add_block("block1".to_string(), block);

        assert!(manager.get_block("block1").is_some());
        assert!(manager.get_block("block2").is_none());
    }

    #[test]
    fn test_code_block_manager_default() {
        let manager = CodeBlockManager::default();
        assert!(manager.get_all_blocks().is_empty());
    }

    #[test]
    fn test_code_block_manager_remove() {
        let mut manager = CodeBlockManager::new();
        let block = CodeBlock::new("test".to_string());
        manager.add_block("block1".to_string(), block);

        let removed = manager.remove_block("block1");
        assert!(removed.is_some());
        assert!(manager.get_block("block1").is_none());
    }

    #[test]
    fn test_code_block_manager_get_all() {
        let mut manager = CodeBlockManager::new();
        manager.add_block("block1".to_string(), CodeBlock::new("test1".to_string()));
        manager.add_block("block2".to_string(), CodeBlock::new("test2".to_string()));

        let blocks = manager.get_all_blocks();
        assert_eq!(blocks.len(), 2);
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_code_config_default() {
        let config = CodeBlockConfig::default();
        assert_eq!(config.language, CodeLanguage::PlainText);
        assert_eq!(config.theme, CodeTheme::Light);
        assert!(!config.show_line_numbers);
        assert_eq!(config.line_number_start, 1);
        assert!(config.highlight_lines.is_empty());
        assert!(!config.wrap_lines);
        assert_eq!(config.tab_size, 4);
    }

    #[test]
    fn test_empty_code_block() {
        let block = CodeBlock::new("".to_string());
        assert_eq!(block.line_count(), 0);
    }

    #[test]
    fn test_single_line_code_block() {
        let block = CodeBlock::new("single line".to_string());
        assert_eq!(block.line_count(), 1);
    }

    #[test]
    fn test_multiline_code_block() {
        let block = CodeBlock::new("line1\nline2\nline3".to_string());
        assert_eq!(block.line_count(), 3);
    }

    #[test]
    fn test_code_block_to_html_theme_classes() {
        let light_block = CodeBlock::new("test".to_string()).with_theme(CodeTheme::Light);
        let dark_block = CodeBlock::new("test".to_string()).with_theme(CodeTheme::Dark);
        let high_contrast_block =
            CodeBlock::new("test".to_string()).with_theme(CodeTheme::HighContrast);

        assert!(light_block.to_html().contains("code-light"));
        assert!(dark_block.to_html().contains("code-dark"));
        assert!(high_contrast_block.to_html().contains("code-high-contrast"));
    }
}
