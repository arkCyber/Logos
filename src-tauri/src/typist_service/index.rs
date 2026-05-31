/*!
 * 航空航天级索引系统
 * 实现 Typst 的索引功能（索引条目、页码引用、分类索引）
 */

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// 索引条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub term: String,
    pub page_numbers: Vec<usize>,
    pub subentries: Vec<IndexEntry>,
    pub category: Option<String>,
}

impl IndexEntry {
    pub fn new(term: String) -> Self {
        Self {
            term,
            page_numbers: Vec::new(),
            subentries: Vec::new(),
            category: None,
        }
    }

    pub fn with_page(mut self, page: usize) -> Self {
        self.page_numbers.push(page);
        self
    }

    pub fn with_pages(mut self, pages: Vec<usize>) -> Self {
        self.page_numbers = pages;
        self
    }

    pub fn with_subentry(mut self, subentry: IndexEntry) -> Self {
        self.subentries.push(subentry);
        self
    }

    pub fn with_category(mut self, category: String) -> Self {
        self.category = Some(category);
        self
    }

    pub fn add_page(&mut self, page: usize) {
        if !self.page_numbers.contains(&page) {
            self.page_numbers.push(page);
            self.page_numbers.sort();
        }
    }
}

/// 索引配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexConfig {
    pub title: String,
    pub style: IndexStyle,
    pub alphabetically: bool,
    pub group_by_letter: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndexStyle {
    List,
    Tree,
    Compact,
}

impl Default for IndexConfig {
    fn default() -> Self {
        Self {
            title: "Index".to_string(),
            style: IndexStyle::Tree,
            alphabetically: true,
            group_by_letter: true,
        }
    }
}

/// 索引
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub entries: Vec<IndexEntry>,
    pub config: IndexConfig,
}

impl Index {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            config: IndexConfig::default(),
        }
    }

    pub fn with_config(mut self, config: IndexConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.config.title = title;
        self
    }

    pub fn with_style(mut self, style: IndexStyle) -> Self {
        self.config.style = style;
        self
    }

    pub fn with_alphabetically(mut self, alphabetically: bool) -> Self {
        self.config.alphabetically = alphabetically;
        self
    }

    pub fn with_group_by_letter(mut self, group_by_letter: bool) -> Self {
        self.config.group_by_letter = group_by_letter;
        self
    }

    pub fn add_entry(mut self, entry: IndexEntry) -> Self {
        self.entries.push(entry);
        self
    }

    pub fn add_term(mut self, term: String, page: usize) -> Self {
        self.entries.push(IndexEntry::new(term).with_page(page));
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        // 添加标题
        typst.push_str(&format!("= {}\n\n", html_escape(&self.config.title)));

        // 按字母顺序排序
        let mut entries = self.entries.clone();
        if self.config.alphabetically {
            entries.sort_by(|a, b| a.term.cmp(&b.term));
        }

        if self.config.group_by_letter {
            // 按字母分组
            let mut grouped: BTreeMap<char, Vec<IndexEntry>> = BTreeMap::new();
            for entry in entries {
                let first_char = entry.term.chars().next().unwrap_or('#');
                grouped
                    .entry(first_char)
                    .or_default()
                    .push(entry);
            }

            for (letter, group_entries) in grouped {
                typst.push_str(&format!("== {}\n\n", letter));
                for entry in group_entries {
                    self.render_entry_to_typst(&entry, &mut typst, 0);
                }
            }
        } else {
            for entry in entries {
                self.render_entry_to_typst(&entry, &mut typst, 0);
            }
        }

        typst
    }

    fn render_entry_to_typst(&self, entry: &IndexEntry, typst: &mut String, depth: usize) {
        let indent = "  ".repeat(depth);
        let pages_str = entry
            .page_numbers
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        match self.config.style {
            IndexStyle::List => {
                typst.push_str(&format!(
                    "{}* {} -- {}\n",
                    indent,
                    html_escape(&entry.term),
                    pages_str
                ));
            }
            IndexStyle::Tree => {
                typst.push_str(&format!("{}* {}\n", indent, html_escape(&entry.term)));
                typst.push_str(&format!("{}  {}\n", indent, pages_str));
            }
            IndexStyle::Compact => {
                typst.push_str(&format!(
                    "{}* {} {}\n",
                    indent,
                    html_escape(&entry.term),
                    pages_str
                ));
            }
        }

        for subentry in &entry.subentries {
            self.render_entry_to_typst(subentry, typst, depth + 1);
        }
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        // 添加标题
        html.push_str(&format!("<h2>{}</h2>\n", html_escape(&self.config.title)));

        // 按字母顺序排序
        let mut entries = self.entries.clone();
        if self.config.alphabetically {
            entries.sort_by(|a, b| a.term.cmp(&b.term));
        }

        if self.config.group_by_letter {
            // 按字母分组
            let mut grouped: BTreeMap<char, Vec<IndexEntry>> = BTreeMap::new();
            for entry in entries {
                let first_char = entry.term.chars().next().unwrap_or('#');
                grouped
                    .entry(first_char)
                    .or_default()
                    .push(entry);
            }

            for (letter, group_entries) in grouped {
                html.push_str(&format!("<h3>{}</h3>\n", letter));
                html.push_str("<ul class=\"index-group\">\n");
                for entry in group_entries {
                    self.render_entry_to_html(&entry, &mut html, 0);
                }
                html.push_str("</ul>\n");
            }
        } else {
            html.push_str("<ul class=\"index-list\">\n");
            for entry in entries {
                self.render_entry_to_html(&entry, &mut html, 0);
            }
            html.push_str("</ul>\n");
        }

        html
    }

    fn render_entry_to_html(&self, entry: &IndexEntry, html: &mut String, depth: usize) {
        let indent = "  ".repeat(depth);
        let pages_str = entry
            .page_numbers
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        match self.config.style {
            IndexStyle::List => {
                html.push_str(&format!(
                    "{}<li><strong>{}</strong> -- {}</li>\n",
                    indent,
                    html_escape(&entry.term),
                    pages_str
                ));
            }
            IndexStyle::Tree => {
                html.push_str(&format!(
                    "{}<li><strong>{}</strong></li>\n",
                    indent,
                    html_escape(&entry.term)
                ));
                html.push_str(&format!(
                    "{}<li class=\"index-pages\">{}</li>\n",
                    indent, pages_str
                ));
            }
            IndexStyle::Compact => {
                html.push_str(&format!(
                    "{}<li><strong>{}</strong> {}</li>\n",
                    indent,
                    html_escape(&entry.term),
                    pages_str
                ));
            }
        }

        if !entry.subentries.is_empty() {
            html.push_str(&format!("{}<ul class=\"index-subentries\">\n", indent));
            for subentry in &entry.subentries {
                self.render_entry_to_html(subentry, html, depth + 1);
            }
            html.push_str(&format!("{}</ul>\n", indent));
        }
    }

    /// 获取术语到页码的映射
    pub fn get_term_map(&self) -> HashMap<String, Vec<usize>> {
        let mut map = HashMap::new();
        for entry in &self.entries {
            map.insert(entry.term.clone(), entry.page_numbers.clone());
        }
        map
    }
}

impl Default for Index {
    fn default() -> Self {
        Self::new()
    }
}

/// 索引构建器
pub struct IndexBuilder {
    index: Index,
}

impl IndexBuilder {
    pub fn new() -> Self {
        Self {
            index: Index::new(),
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.index = self.index.with_title(title);
        self
    }

    pub fn style(mut self, style: IndexStyle) -> Self {
        self.index = self.index.with_style(style);
        self
    }

    pub fn alphabetically(mut self, alphabetically: bool) -> Self {
        self.index = self.index.with_alphabetically(alphabetically);
        self
    }

    pub fn group_by_letter(mut self, group_by_letter: bool) -> Self {
        self.index = self.index.with_group_by_letter(group_by_letter);
        self
    }

    pub fn entry(mut self, entry: IndexEntry) -> Self {
        self.index = self.index.add_entry(entry);
        self
    }

    pub fn term(mut self, term: String, page: usize) -> Self {
        self.index = self.index.add_term(term, page);
        self
    }

    pub fn build(self) -> Index {
        self.index
    }
}

impl Default for IndexBuilder {
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
    fn test_index_creation() {
        let index = Index::new();
        assert_eq!(index.entries.len(), 0);
    }

    #[test]
    fn test_index_default() {
        let index = Index::default();
        assert_eq!(index.entries.len(), 0);
    }

    #[test]
    fn test_index_config_default() {
        let config = IndexConfig::default();
        assert_eq!(config.title, "Index");
        assert!(matches!(config.style, IndexStyle::Tree));
    }

    #[test]
    fn test_index_entry_creation() {
        let entry = IndexEntry::new("Term".to_string());
        assert_eq!(entry.term, "Term");
        assert!(entry.page_numbers.is_empty());
    }

    #[test]
    fn test_index_entry_with_page() {
        let entry = IndexEntry::new("Term".to_string()).with_page(1);
        assert_eq!(entry.page_numbers, vec![1]);
    }

    #[test]
    fn test_index_entry_with_pages() {
        let entry = IndexEntry::new("Term".to_string()).with_pages(vec![1, 2, 3]);
        assert_eq!(entry.page_numbers, vec![1, 2, 3]);
    }

    #[test]
    fn test_index_entry_with_subentry() {
        let subentry = IndexEntry::new("Subterm".to_string());
        let entry = IndexEntry::new("Term".to_string()).with_subentry(subentry);
        assert_eq!(entry.subentries.len(), 1);
    }

    #[test]
    fn test_index_entry_with_category() {
        let entry = IndexEntry::new("Term".to_string()).with_category("Category".to_string());
        assert_eq!(entry.category, Some("Category".to_string()));
    }

    #[test]
    fn test_index_entry_add_page() {
        let mut entry = IndexEntry::new("Term".to_string());
        entry.add_page(1);
        entry.add_page(2);
        entry.add_page(1); // Duplicate should not be added
        assert_eq!(entry.page_numbers, vec![1, 2]);
    }

    #[test]
    fn test_index_with_title() {
        let index = Index::new().with_title("Topics".to_string());
        assert_eq!(index.config.title, "Topics");
    }

    #[test]
    fn test_index_with_style() {
        let index = Index::new().with_style(IndexStyle::List);
        assert!(matches!(index.config.style, IndexStyle::List));
    }

    #[test]
    fn test_index_with_alphabetically() {
        let index = Index::new().with_alphabetically(false);
        assert!(!index.config.alphabetically);
    }

    #[test]
    fn test_index_add_entry() {
        let index = Index::new().add_entry(IndexEntry::new("Term".to_string()));
        assert_eq!(index.entries.len(), 1);
    }

    #[test]
    fn test_index_add_term() {
        let index = Index::new().add_term("Term".to_string(), 1);
        assert_eq!(index.entries.len(), 1);
    }

    #[test]
    fn test_index_style_variants() {
        assert!(matches!(IndexStyle::List, IndexStyle::List));
        assert!(matches!(IndexStyle::Tree, IndexStyle::Tree));
        assert!(matches!(IndexStyle::Compact, IndexStyle::Compact));
    }

    #[test]
    fn test_to_typst() {
        let index = Index::new().add_term("Term".to_string(), 1);
        let typst = index.to_typst();
        assert!(typst.contains("= Index"));
        assert!(typst.contains("Term"));
    }

    #[test]
    fn test_to_typst_with_title() {
        let index = Index::new()
            .with_title("Topics".to_string())
            .add_term("Term".to_string(), 1);
        let typst = index.to_typst();
        assert!(typst.contains("= Topics"));
    }

    #[test]
    fn test_to_typst_tree_style() {
        let index = Index::new()
            .with_style(IndexStyle::Tree)
            .add_term("Term".to_string(), 1);
        let typst = index.to_typst();
        assert!(typst.contains("* Term"));
    }

    #[test]
    fn test_to_html() {
        let index = Index::new().add_term("Term".to_string(), 1);
        let html = index.to_html();
        assert!(html.contains("<h2>Index</h2>"));
        assert!(html.contains("<ul class=\"index-group\">"));
    }

    #[test]
    fn test_to_html_list_style() {
        let index = Index::new()
            .with_style(IndexStyle::List)
            .add_term("Term".to_string(), 1);
        let html = index.to_html();
        assert!(html.contains("<strong>Term</strong> -- 1"));
    }

    #[test]
    fn test_to_html_tree_style() {
        let index = Index::new()
            .with_style(IndexStyle::Tree)
            .add_term("Term".to_string(), 1);
        let html = index.to_html();
        assert!(html.contains("<strong>Term</strong>"));
        assert!(html.contains("index-pages"));
    }

    #[test]
    fn test_get_term_map() {
        let index = Index::new().add_term("Term".to_string(), 1);
        let map = index.get_term_map();
        assert_eq!(map.get("Term"), Some(&vec![1]));
    }

    #[test]
    fn test_index_builder() {
        let index = IndexBuilder::new()
            .title("Topics".to_string())
            .style(IndexStyle::List)
            .term("Term".to_string(), 1)
            .build();

        assert_eq!(index.config.title, "Topics");
        assert_eq!(index.entries.len(), 1);
    }

    #[test]
    fn test_index_builder_default() {
        let builder = IndexBuilder::default();
        let index = builder.build();
        assert_eq!(index.entries.len(), 0);
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_alphabetical_sorting() {
        let index = Index::new()
            .with_alphabetically(true)
            .add_term("B".to_string(), 1)
            .add_term("A".to_string(), 2);
        let typst = index.to_typst();
        let a_pos = typst.find("A");
        let b_pos = typst.find("B");
        assert!(a_pos < b_pos);
    }

    #[test]
    fn test_to_typst_with_subentries() {
        let subentry = IndexEntry::new("Subterm".to_string()).with_page(2);
        let entry = IndexEntry::new("Term".to_string())
            .with_page(1)
            .with_subentry(subentry);
        let index = Index::new().add_entry(entry);
        let typst = index.to_typst();
        assert!(typst.contains("Term"));
        assert!(typst.contains("Subterm"));
    }

    #[test]
    fn test_to_html_with_subentries() {
        let subentry = IndexEntry::new("Subterm".to_string()).with_page(2);
        let entry = IndexEntry::new("Term".to_string())
            .with_page(1)
            .with_subentry(subentry);
        let index = Index::new().add_entry(entry);
        let html = index.to_html();
        assert!(html.contains("index-subentries"));
    }

    #[test]
    fn test_to_typst_group_by_letter() {
        let index = Index::new()
            .with_group_by_letter(true)
            .add_term("Apple".to_string(), 1)
            .add_term("Banana".to_string(), 2);
        let typst = index.to_typst();
        assert!(typst.contains("== A"));
        assert!(typst.contains("== B"));
    }

    #[test]
    fn test_to_html_group_by_letter() {
        let index = Index::new()
            .with_group_by_letter(true)
            .add_term("Apple".to_string(), 1)
            .add_term("Banana".to_string(), 2);
        let html = index.to_html();
        assert!(html.contains("<h3>A</h3>"));
        assert!(html.contains("<h3>B</h3>"));
    }
}
