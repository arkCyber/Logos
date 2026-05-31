/*!
 * 航空航天级大纲系统
 * 实现 Typst 的大纲功能（目录、层级、样式）
 */

use serde::{Deserialize, Serialize};

/// 大纲条目类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutlineEntryType {
    Heading,
    Figure,
    Table,
    Equation,
    Bibliography,
    Custom(String),
}

/// 大纲条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlineEntry {
    pub title: String,
    pub level: usize,
    pub entry_type: OutlineEntryType,
    pub page_number: usize,
    pub label: Option<String>,
    pub children: Vec<OutlineEntry>,
}

impl OutlineEntry {
    pub fn new(title: String, level: usize, entry_type: OutlineEntryType) -> Self {
        Self {
            title,
            level,
            entry_type,
            page_number: 1,
            label: None,
            children: Vec::new(),
        }
    }

    pub fn with_page_number(mut self, page: usize) -> Self {
        self.page_number = page;
        self
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn add_child(&mut self, child: OutlineEntry) {
        self.children.push(child);
    }
}

/// 大纲缩进
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutlineIndent {
    Auto,
    Fixed(f64),
    Relative(f64),
}

/// 大纲配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlineConfig {
    pub title: Option<String>,
    pub target: Option<String>,
    pub depth: Option<usize>,
    pub indent: OutlineIndent,
    pub show_page_numbers: bool,
    pub show_prefix: bool,
}

impl Default for OutlineConfig {
    fn default() -> Self {
        Self {
            title: Some("Contents".to_string()),
            target: None,
            depth: None,
            indent: OutlineIndent::Auto,
            show_page_numbers: true,
            show_prefix: true,
        }
    }
}

/// 大纲
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outline {
    pub entries: Vec<OutlineEntry>,
    pub config: OutlineConfig,
}

impl Outline {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            config: OutlineConfig::default(),
        }
    }

    pub fn with_config(config: OutlineConfig) -> Self {
        Self {
            entries: Vec::new(),
            config,
        }
    }

    pub fn add_entry(&mut self, entry: OutlineEntry) {
        self.entries.push(entry);
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        typst.push_str("#outline(\n");

        // 添加标题
        if let Some(title) = &self.config.title {
            typst.push_str(&format!("  title: [{}],\n", title));
        }

        // 添加深度
        if let Some(depth) = self.config.depth {
            typst.push_str(&format!("  depth: {},\n", depth));
        }

        // 添加缩进
        match &self.config.indent {
            OutlineIndent::Auto => {}
            OutlineIndent::Fixed(size) => typst.push_str(&format!("  indent: {}pt,\n", size)),
            OutlineIndent::Relative(size) => typst.push_str(&format!("  indent: {}em,\n", size)),
        }

        typst.push_str(")\n");

        typst
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        html.push_str("<div class=\"typst-outline\">\n");

        if let Some(title) = &self.config.title {
            html.push_str(&format!(
                "  <h2 class=\"outline-title\">{}</h2>\n",
                html_escape(title)
            ));
        }

        html.push_str("  <ul class=\"outline-list\">\n");

        for entry in &self.entries {
            html.push_str(&self.format_entry(entry, 0));
        }

        html.push_str("  </ul>\n");
        html.push_str("</div>\n");

        html
    }

    fn format_entry(&self, entry: &OutlineEntry, depth: usize) -> String {
        let indent = "  ".repeat(depth + 1);
        let mut html = String::new();

        let page_number = if self.config.show_page_numbers {
            format!(" <span class=\"outline-page\">{}</span>", entry.page_number)
        } else {
            String::new()
        };

        let prefix = if self.config.show_prefix {
            format!("{}.", entry.level)
        } else {
            String::new()
        };

        html.push_str(&format!(
            "{}<li class=\"outline-item level-{}\">\n",
            indent, entry.level
        ));
        html.push_str(&format!(
            "{}  <a href=\"#{}\" class=\"outline-link\">{} {}{}</a>\n",
            indent,
            entry.label.as_deref().unwrap_or(""),
            prefix,
            html_escape(&entry.title),
            page_number
        ));

        if !entry.children.is_empty() {
            html.push_str(&format!("{}  <ul class=\"outline-sublist\">\n", indent));
            for child in &entry.children {
                html.push_str(&self.format_entry(child, depth + 1));
            }
            html.push_str(&format!("{}  </ul>\n", indent));
        }

        html.push_str(&format!("{}</li>\n", indent));

        html
    }

    /// 获取条目总数
    pub fn entry_count(&self) -> usize {
        self.count_entries_recursive(&self.entries)
    }

    fn count_entries_recursive(&self, entries: &[OutlineEntry]) -> usize {
        entries.iter().fold(0, |count, entry| {
            count + 1 + self.count_entries_recursive(&entry.children)
        })
    }

    /// 获取最大深度
    pub fn max_depth(&self) -> usize {
        self.max_depth_recursive(&self.entries, 0)
    }

    fn max_depth_recursive(&self, entries: &[OutlineEntry], current: usize) -> usize {
        entries.iter().fold(current, |max, entry| {
            let child_max = self.max_depth_recursive(&entry.children, current + 1);
            std::cmp::max(max, child_max)
        })
    }
}

impl Default for Outline {
    fn default() -> Self {
        Self::new()
    }
}

/// 大纲构建器
pub struct OutlineBuilder {
    outline: Outline,
}

impl OutlineBuilder {
    pub fn new() -> Self {
        Self {
            outline: Outline::new(),
        }
    }

    pub fn with_config(mut self, config: OutlineConfig) -> Self {
        self.outline.config = config;
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.outline.config.title = Some(title);
        self
    }

    pub fn with_depth(mut self, depth: usize) -> Self {
        self.outline.config.depth = Some(depth);
        self
    }

    pub fn with_indent(mut self, indent: OutlineIndent) -> Self {
        self.outline.config.indent = indent;
        self
    }

    pub fn show_page_numbers(mut self, show: bool) -> Self {
        self.outline.config.show_page_numbers = show;
        self
    }

    pub fn show_prefix(mut self, show: bool) -> Self {
        self.outline.config.show_prefix = show;
        self
    }

    pub fn add_entry(mut self, entry: OutlineEntry) -> Self {
        self.outline.add_entry(entry);
        self
    }

    pub fn build(self) -> Outline {
        self.outline
    }
}

impl Default for OutlineBuilder {
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
    fn test_outline_creation() {
        let outline = Outline::new();
        assert!(outline.entries.is_empty());
    }

    #[test]
    fn test_outline_default() {
        let outline = Outline::default();
        assert!(outline.entries.is_empty());
        assert_eq!(outline.config.title, Some("Contents".to_string()));
    }

    #[test]
    fn test_outline_config_default() {
        let config = OutlineConfig::default();
        assert_eq!(config.title, Some("Contents".to_string()));
        assert!(config.show_page_numbers);
    }

    #[test]
    fn test_outline_entry_creation() {
        let entry = OutlineEntry::new("Chapter 1".to_string(), 1, OutlineEntryType::Heading);
        assert_eq!(entry.title, "Chapter 1");
        assert_eq!(entry.level, 1);
    }

    #[test]
    fn test_outline_entry_with_page_number() {
        let entry = OutlineEntry::new("Chapter 1".to_string(), 1, OutlineEntryType::Heading)
            .with_page_number(5);
        assert_eq!(entry.page_number, 5);
    }

    #[test]
    fn test_outline_entry_with_label() {
        let entry = OutlineEntry::new("Chapter 1".to_string(), 1, OutlineEntryType::Heading)
            .with_label("ch1".to_string());
        assert_eq!(entry.label, Some("ch1".to_string()));
    }

    #[test]
    fn test_outline_entry_add_child() {
        let mut parent = OutlineEntry::new("Chapter 1".to_string(), 1, OutlineEntryType::Heading);
        parent.add_child(OutlineEntry::new(
            "Section 1.1".to_string(),
            2,
            OutlineEntryType::Heading,
        ));
        assert_eq!(parent.children.len(), 1);
    }

    #[test]
    fn test_outline_add_entry() {
        let mut outline = Outline::new();
        outline.add_entry(OutlineEntry::new(
            "Chapter 1".to_string(),
            1,
            OutlineEntryType::Heading,
        ));
        assert_eq!(outline.entries.len(), 1);
    }

    #[test]
    fn test_entry_count() {
        let mut outline = Outline::new();
        let mut entry = OutlineEntry::new("Chapter 1".to_string(), 1, OutlineEntryType::Heading);
        entry.add_child(OutlineEntry::new(
            "Section 1.1".to_string(),
            2,
            OutlineEntryType::Heading,
        ));
        outline.add_entry(entry);
        assert_eq!(outline.entry_count(), 2);
    }

    #[test]
    fn test_max_depth() {
        let mut outline = Outline::new();
        let mut entry = OutlineEntry::new("Chapter 1".to_string(), 1, OutlineEntryType::Heading);
        entry.add_child(OutlineEntry::new(
            "Section 1.1".to_string(),
            2,
            OutlineEntryType::Heading,
        ));
        outline.add_entry(entry);
        assert_eq!(outline.max_depth(), 2);
    }

    #[test]
    fn test_to_typst() {
        let outline = Outline::new();
        let typst = outline.to_typst();
        assert!(typst.contains("#outline("));
        assert!(typst.contains("title: [Contents]"));
    }

    #[test]
    fn test_to_html() {
        let outline = Outline::new();
        let html = outline.to_html();
        assert!(html.contains("<div class=\"typst-outline\""));
        assert!(html.contains("<h2 class=\"outline-title\">Contents</h2>"));
    }

    #[test]
    fn test_to_html_with_entries() {
        let mut outline = Outline::new();
        outline.add_entry(OutlineEntry::new(
            "Chapter 1".to_string(),
            1,
            OutlineEntryType::Heading,
        ));
        let html = outline.to_html();
        assert!(html.contains("Chapter 1"));
    }

    #[test]
    fn test_outline_entry_type_partial_eq() {
        assert_eq!(OutlineEntryType::Heading, OutlineEntryType::Heading);
        assert_ne!(OutlineEntryType::Heading, OutlineEntryType::Figure);
    }

    #[test]
    fn test_outline_builder() {
        let outline = OutlineBuilder::new()
            .with_title("Table of Contents".to_string())
            .with_depth(3)
            .add_entry(OutlineEntry::new(
                "Chapter 1".to_string(),
                1,
                OutlineEntryType::Heading,
            ))
            .build();

        assert_eq!(outline.config.title, Some("Table of Contents".to_string()));
        assert_eq!(outline.config.depth, Some(3));
    }

    #[test]
    fn test_outline_builder_default() {
        let builder = OutlineBuilder::default();
        let outline = builder.build();
        assert!(outline.entries.is_empty());
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_outline_with_config() {
        let config = OutlineConfig {
            title: Some("Table of Contents".to_string()),
            target: Some("main".to_string()),
            depth: Some(3),
            indent: OutlineIndent::Fixed(2.0),
            show_page_numbers: false,
            show_prefix: false,
        };
        let outline = Outline::with_config(config);
        assert_eq!(outline.config.title, Some("Table of Contents".to_string()));
        assert_eq!(outline.config.depth, Some(3));
    }

    #[test]
    fn test_outline_indent_variants() {
        let auto = OutlineIndent::Auto;
        let fixed = OutlineIndent::Fixed(2.0);
        let relative = OutlineIndent::Relative(0.5);

        assert!(matches!(auto, OutlineIndent::Auto));
        assert!(matches!(fixed, OutlineIndent::Fixed(_)));
        assert!(matches!(relative, OutlineIndent::Relative(_)));
    }

    #[test]
    fn test_nested_outline() {
        let mut outline = Outline::new();
        let mut chapter = OutlineEntry::new("Chapter 1".to_string(), 1, OutlineEntryType::Heading);
        chapter.add_child(OutlineEntry::new(
            "Section 1.1".to_string(),
            2,
            OutlineEntryType::Heading,
        ));
        chapter.add_child(OutlineEntry::new(
            "Section 1.2".to_string(),
            2,
            OutlineEntryType::Heading,
        ));
        outline.add_entry(chapter);
        assert_eq!(outline.entry_count(), 3);
    }

    #[test]
    fn test_outline_show_page_numbers() {
        let mut outline = Outline::new();
        outline.config.show_page_numbers = false;
        outline.add_entry(OutlineEntry::new(
            "Chapter 1".to_string(),
            1,
            OutlineEntryType::Heading,
        ));
        let html = outline.to_html();
        assert!(!html.contains("outline-page"));
    }

    #[test]
    fn test_outline_show_prefix() {
        let mut outline = Outline::new();
        outline.config.show_prefix = false;
        outline.add_entry(OutlineEntry::new(
            "Chapter 1".to_string(),
            1,
            OutlineEntryType::Heading,
        ));
        let html = outline.to_html();
        assert!(!html.contains("1."));
    }

    #[test]
    fn test_empty_outline() {
        let outline = Outline::new();
        assert_eq!(outline.entry_count(), 0);
        assert_eq!(outline.max_depth(), 0);
    }

    #[test]
    fn test_outline_entry_type_variants() {
        assert_eq!(OutlineEntryType::Heading, OutlineEntryType::Heading);
        assert_eq!(OutlineEntryType::Figure, OutlineEntryType::Figure);
        assert_eq!(OutlineEntryType::Table, OutlineEntryType::Table);
        assert_eq!(OutlineEntryType::Equation, OutlineEntryType::Equation);
        assert_eq!(
            OutlineEntryType::Bibliography,
            OutlineEntryType::Bibliography
        );
    }

    #[test]
    fn test_outline_to_typst_with_depth() {
        let outline = Outline::with_config(OutlineConfig {
            title: Some("Contents".to_string()),
            target: None,
            depth: Some(2),
            indent: OutlineIndent::Auto,
            show_page_numbers: true,
            show_prefix: true,
        });
        let typst = outline.to_typst();
        assert!(typst.contains("depth: 2"));
    }

    #[test]
    fn test_outline_to_typst_with_indent() {
        let outline = Outline::with_config(OutlineConfig {
            title: Some("Contents".to_string()),
            target: None,
            depth: None,
            indent: OutlineIndent::Fixed(2.0),
            show_page_numbers: true,
            show_prefix: true,
        });
        let typst = outline.to_typst();
        assert!(typst.contains("indent: 2pt"));
    }
}
