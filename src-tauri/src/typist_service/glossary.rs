/*!
 * 航空航天级词汇表系统
 * 实现 Typst 的词汇表功能（术语定义、短形式、长形式、分类）
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// 词汇表条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlossaryEntry {
    pub term: String,
    pub short_form: Option<String>,
    pub long_form: Option<String>,
    pub definition: String,
    pub category: Option<String>,
}

impl GlossaryEntry {
    pub fn new(term: String, definition: String) -> Self {
        Self {
            term,
            short_form: None,
            long_form: None,
            definition,
            category: None,
        }
    }

    pub fn with_short_form(mut self, short_form: String) -> Self {
        self.short_form = Some(short_form);
        self
    }

    pub fn with_long_form(mut self, long_form: String) -> Self {
        self.long_form = Some(long_form);
        self
    }

    pub fn with_category(mut self, category: String) -> Self {
        self.category = Some(category);
        self
    }
}

/// 词汇表配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlossaryConfig {
    pub title: String,
    pub style: GlossaryStyle,
    pub alphabetically: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GlossaryStyle {
    List,
    Table,
    DescriptionList,
}

impl Default for GlossaryConfig {
    fn default() -> Self {
        Self {
            title: "Glossary".to_string(),
            style: GlossaryStyle::DescriptionList,
            alphabetically: true,
        }
    }
}

/// 词汇表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Glossary {
    pub entries: Vec<GlossaryEntry>,
    pub config: GlossaryConfig,
}

impl Glossary {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            config: GlossaryConfig::default(),
        }
    }

    pub fn with_config(mut self, config: GlossaryConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.config.title = title;
        self
    }

    pub fn with_style(mut self, style: GlossaryStyle) -> Self {
        self.config.style = style;
        self
    }

    pub fn with_alphabetically(mut self, alphabetically: bool) -> Self {
        self.config.alphabetically = alphabetically;
        self
    }

    pub fn add_entry(mut self, entry: GlossaryEntry) -> Self {
        self.entries.push(entry);
        self
    }

    pub fn add_term(mut self, term: String, definition: String) -> Self {
        self.entries.push(GlossaryEntry::new(term, definition));
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        // 添加标题
        typst.push_str(&format!("= {}\n\n", escape_html(&self.config.title)));

        // 按字母顺序排序
        let mut entries = self.entries.clone();
        if self.config.alphabetically {
            entries.sort_by(|a, b| a.term.cmp(&b.term));
        }

        match self.config.style {
            GlossaryStyle::List => {
                for entry in &entries {
                    typst.push_str(&format!(
                        "* **{}**: {}\n",
                        escape_html(&entry.term),
                        escape_html(&entry.definition)
                    ));
                }
            }
            GlossaryStyle::Table => {
                typst.push_str("#table(\n");
                typst.push_str("  columns: 2,\n");
                typst.push_str("  stroke: none,\n");
                for entry in &entries {
                    typst.push_str(&format!(
                        "  [*{}*][{}],\n",
                        escape_html(&entry.term),
                        escape_html(&entry.definition)
                    ));
                }
                typst.push_str(")\n");
            }
            GlossaryStyle::DescriptionList => {
                for entry in &entries {
                    typst.push_str(&format!("* /{}/\n", escape_html(&entry.term)));
                    typst.push_str(&format!("  {}\n", escape_html(&entry.definition)));
                }
            }
        }

        typst
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        // 添加标题
        html.push_str(&format!("<h2>{}</h2>\n", escape_html(&self.config.title)));

        // 按字母顺序排序
        let mut entries = self.entries.clone();
        if self.config.alphabetically {
            entries.sort_by(|a, b| a.term.cmp(&b.term));
        }

        match self.config.style {
            GlossaryStyle::List => {
                html.push_str("<ul class=\"glossary-list\">\n");
                for entry in &entries {
                    html.push_str(&format!(
                        "  <li><strong>{}</strong>: {}</li>\n",
                        escape_html(&entry.term),
                        escape_html(&entry.definition)
                    ));
                }
                html.push_str("</ul>\n");
            }
            GlossaryStyle::Table => {
                html.push_str("<table class=\"glossary-table\">\n");
                html.push_str("  <thead>\n");
                html.push_str("    <tr><th>Term</th><th>Definition</th></tr>\n");
                html.push_str("  </thead>\n");
                html.push_str("  <tbody>\n");
                for entry in &entries {
                    html.push_str(&format!(
                        "    <tr><td>{}</td><td>{}</td></tr>\n",
                        escape_html(&entry.term),
                        escape_html(&entry.definition)
                    ));
                }
                html.push_str("  </tbody>\n");
                html.push_str("</table>\n");
            }
            GlossaryStyle::DescriptionList => {
                html.push_str("<dl class=\"glossary-dl\">\n");
                for entry in &entries {
                    html.push_str(&format!("  <dt>{}</dt>\n", escape_html(&entry.term)));
                    html.push_str(&format!("  <dd>{}</dd>\n", escape_html(&entry.definition)));
                }
                html.push_str("</dl>\n");
            }
        }

        html
    }

    /// 获取术语映射
    pub fn get_term_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for entry in &self.entries {
            map.insert(entry.term.clone(), entry.definition.clone());
        }
        map
    }
}

impl Default for Glossary {
    fn default() -> Self {
        Self::new()
    }
}

/// 词汇表构建器
pub struct GlossaryBuilder {
    glossary: Glossary,
}

impl GlossaryBuilder {
    pub fn new() -> Self {
        Self {
            glossary: Glossary::new(),
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.glossary = self.glossary.with_title(title);
        self
    }

    pub fn style(mut self, style: GlossaryStyle) -> Self {
        self.glossary = self.glossary.with_style(style);
        self
    }

    pub fn alphabetically(mut self, alphabetically: bool) -> Self {
        self.glossary = self.glossary.with_alphabetically(alphabetically);
        self
    }

    pub fn entry(mut self, entry: GlossaryEntry) -> Self {
        self.glossary = self.glossary.add_entry(entry);
        self
    }

    pub fn term(mut self, term: String, definition: String) -> Self {
        self.glossary = self.glossary.add_term(term, definition);
        self
    }

    pub fn build(self) -> Glossary {
        self.glossary
    }
}

impl Default for GlossaryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glossary_creation() {
        let glossary = Glossary::new();
        assert_eq!(glossary.entries.len(), 0);
    }

    #[test]
    fn test_glossary_default() {
        let glossary = Glossary::default();
        assert_eq!(glossary.entries.len(), 0);
    }

    #[test]
    fn test_glossary_config_default() {
        let config = GlossaryConfig::default();
        assert_eq!(config.title, "Glossary");
        assert!(matches!(config.style, GlossaryStyle::DescriptionList));
    }

    #[test]
    fn test_glossary_entry_creation() {
        let entry = GlossaryEntry::new("Term".to_string(), "Definition".to_string());
        assert_eq!(entry.term, "Term");
        assert_eq!(entry.definition, "Definition");
    }

    #[test]
    fn test_glossary_entry_with_short_form() {
        let entry = GlossaryEntry::new("Term".to_string(), "Definition".to_string())
            .with_short_form("SF".to_string());
        assert_eq!(entry.short_form, Some("SF".to_string()));
    }

    #[test]
    fn test_glossary_entry_with_long_form() {
        let entry = GlossaryEntry::new("Term".to_string(), "Definition".to_string())
            .with_long_form("Long Form".to_string());
        assert_eq!(entry.long_form, Some("Long Form".to_string()));
    }

    #[test]
    fn test_glossary_entry_with_category() {
        let entry = GlossaryEntry::new("Term".to_string(), "Definition".to_string())
            .with_category("Category".to_string());
        assert_eq!(entry.category, Some("Category".to_string()));
    }

    #[test]
    fn test_glossary_with_title() {
        let glossary = Glossary::new().with_title("Terms".to_string());
        assert_eq!(glossary.config.title, "Terms");
    }

    #[test]
    fn test_glossary_with_style() {
        let glossary = Glossary::new().with_style(GlossaryStyle::Table);
        assert!(matches!(glossary.config.style, GlossaryStyle::Table));
    }

    #[test]
    fn test_glossary_with_alphabetically() {
        let glossary = Glossary::new().with_alphabetically(false);
        assert!(!glossary.config.alphabetically);
    }

    #[test]
    fn test_glossary_add_entry() {
        let glossary = Glossary::new().add_entry(GlossaryEntry::new(
            "Term".to_string(),
            "Definition".to_string(),
        ));
        assert_eq!(glossary.entries.len(), 1);
    }

    #[test]
    fn test_glossary_add_term() {
        let glossary = Glossary::new().add_term("Term".to_string(), "Definition".to_string());
        assert_eq!(glossary.entries.len(), 1);
    }

    #[test]
    fn test_glossary_style_variants() {
        assert!(matches!(GlossaryStyle::List, GlossaryStyle::List));
        assert!(matches!(GlossaryStyle::Table, GlossaryStyle::Table));
        assert!(matches!(
            GlossaryStyle::DescriptionList,
            GlossaryStyle::DescriptionList
        ));
    }

    #[test]
    fn test_to_typst() {
        let glossary = Glossary::new()
            .with_style(GlossaryStyle::List)
            .add_term("Term".to_string(), "Definition".to_string());
        let typst = glossary.to_typst();
        assert!(typst.contains("= Glossary"));
        assert!(typst.contains("* **Term**: Definition"));
    }

    #[test]
    fn test_to_typst_with_title() {
        let glossary = Glossary::new()
            .with_title("Terms".to_string())
            .add_term("Term".to_string(), "Definition".to_string());
        let typst = glossary.to_typst();
        assert!(typst.contains("= Terms"));
    }

    #[test]
    fn test_to_typst_table_style() {
        let glossary = Glossary::new()
            .with_style(GlossaryStyle::Table)
            .add_term("Term".to_string(), "Definition".to_string());
        let typst = glossary.to_typst();
        assert!(typst.contains("#table("));
    }

    #[test]
    fn test_to_html() {
        let glossary = Glossary::new().add_term("Term".to_string(), "Definition".to_string());
        let html = glossary.to_html();
        assert!(html.contains("<h2>Glossary</h2>"));
        assert!(html.contains("<dl class=\"glossary-dl\">"));
    }

    #[test]
    fn test_to_html_list_style() {
        let glossary = Glossary::new()
            .with_style(GlossaryStyle::List)
            .add_term("Term".to_string(), "Definition".to_string());
        let html = glossary.to_html();
        assert!(html.contains("<ul class=\"glossary-list\">"));
    }

    #[test]
    fn test_to_html_table_style() {
        let glossary = Glossary::new()
            .with_style(GlossaryStyle::Table)
            .add_term("Term".to_string(), "Definition".to_string());
        let html = glossary.to_html();
        assert!(html.contains("<table class=\"glossary-table\">"));
    }

    #[test]
    fn test_get_term_map() {
        let glossary = Glossary::new().add_term("Term".to_string(), "Definition".to_string());
        let map = glossary.get_term_map();
        assert_eq!(map.get("Term"), Some(&"Definition".to_string()));
    }

    #[test]
    fn test_glossary_builder() {
        let glossary = GlossaryBuilder::new()
            .title("Terms".to_string())
            .style(GlossaryStyle::List)
            .term("Term".to_string(), "Definition".to_string())
            .build();

        assert_eq!(glossary.config.title, "Terms");
        assert_eq!(glossary.entries.len(), 1);
    }

    #[test]
    fn test_glossary_builder_default() {
        let builder = GlossaryBuilder::default();
        let glossary = builder.build();
        assert_eq!(glossary.entries.len(), 0);
    }

    #[test]
    fn test_escape_html() {
        let escaped = escape_html("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_alphabetical_sorting() {
        let glossary = Glossary::new()
            .with_alphabetically(true)
            .add_term("B".to_string(), "Def B".to_string())
            .add_term("A".to_string(), "Def A".to_string());
        let typst = glossary.to_typst();
        let a_pos = typst.find("A");
        let b_pos = typst.find("B");
        assert!(a_pos < b_pos);
    }

    #[test]
    fn test_to_typst_description_list() {
        let glossary = Glossary::new()
            .with_style(GlossaryStyle::DescriptionList)
            .add_term("Term".to_string(), "Definition".to_string());
        let typst = glossary.to_typst();
        assert!(typst.contains("/Term/"));
    }

    #[test]
    fn test_to_html_description_list() {
        let glossary = Glossary::new()
            .with_style(GlossaryStyle::DescriptionList)
            .add_term("Term".to_string(), "Definition".to_string());
        let html = glossary.to_html();
        assert!(html.contains("<dt>Term</dt>"));
        assert!(html.contains("<dd>Definition</dd>"));
    }
}
