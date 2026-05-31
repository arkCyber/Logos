/*!
 * 航空航天级参考文献系统
 * 实现 Typst 的参考文献功能（BibTeX 解析、引用生成、样式支持）
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 参考文献类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BibEntryType {
    Article,
    Book,
    Booklet,
    Conference,
    InBook,
    InCollection,
    InProceedings,
    Manual,
    MasterThesis,
    Misc,
    PhDThesis,
    Proceedings,
    TechReport,
    Unpublished,
    Custom(String),
}

/// 参考文献条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BibEntry {
    pub key: String,
    pub entry_type: BibEntryType,
    pub author: Option<String>,
    pub title: Option<String>,
    pub year: Option<String>,
    pub journal: Option<String>,
    pub volume: Option<String>,
    pub number: Option<String>,
    pub pages: Option<String>,
    pub publisher: Option<String>,
    pub address: Option<String>,
    pub edition: Option<String>,
    pub month: Option<String>,
    pub note: Option<String>,
    pub url: Option<String>,
    pub doi: Option<String>,
    pub extra_fields: HashMap<String, String>,
}

impl BibEntry {
    pub fn new(key: String, entry_type: BibEntryType) -> Self {
        Self {
            key,
            entry_type,
            author: None,
            title: None,
            year: None,
            journal: None,
            volume: None,
            number: None,
            pages: None,
            publisher: None,
            address: None,
            edition: None,
            month: None,
            note: None,
            url: None,
            doi: None,
            extra_fields: HashMap::new(),
        }
    }

    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_year(mut self, year: String) -> Self {
        self.year = Some(year);
        self
    }

    pub fn with_journal(mut self, journal: String) -> Self {
        self.journal = Some(journal);
        self
    }

    pub fn with_publisher(mut self, publisher: String) -> Self {
        self.publisher = Some(publisher);
        self
    }

    pub fn add_extra_field(mut self, key: String, value: String) -> Self {
        self.extra_fields.insert(key, value);
        self
    }
}

/// 引用样式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CitationStyle {
    /// APA 样式
    APA,
    /// MLA 样式
    MLA,
    /// Chicago 样式
    Chicago,
    /// IEEE 样式
    IEEE,
    /// Harvard 样式
    Harvard,
    /// Vancouver 样式
    Vancouver,
    /// 自定义样式
    Custom(String),
}

/// 参考文献配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BibliographyConfig {
    pub title: Option<String>,
    pub style: CitationStyle,
    pub full: bool,
    pub sort_by: Option<String>,
}

impl Default for BibliographyConfig {
    fn default() -> Self {
        Self {
            title: Some("References".to_string()),
            style: CitationStyle::APA,
            full: false,
            sort_by: None,
        }
    }
}

/// 参考文献系统
pub struct Bibliography {
    pub entries: Vec<BibEntry>,
    pub config: BibliographyConfig,
    pub cited_keys: Vec<String>,
}

impl Bibliography {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            config: BibliographyConfig::default(),
            cited_keys: Vec::new(),
        }
    }

    pub fn with_config(config: BibliographyConfig) -> Self {
        Self {
            entries: Vec::new(),
            config,
            cited_keys: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: BibEntry) {
        self.entries.push(entry);
    }

    pub fn cite(&mut self, key: String) {
        if !self.cited_keys.contains(&key) {
            self.cited_keys.push(key);
        }
    }

    /// 解析 BibTeX 格式
    pub fn parse_bibtex(&mut self, bibtex: &str) -> Result<(), String> {
        let lines: Vec<&str> = bibtex.lines().collect();
        let mut current_entry: Option<BibEntry> = None;
        let mut in_entry = false;

        for line in lines {
            let line = line.trim();

            if line.starts_with('@') {
                // 保存上一个条目
                if let Some(entry) = current_entry.take() {
                    self.add_entry(entry);
                }

                // 开始新条目
                let parts: Vec<&str> = line.split('{').collect();
                if parts.len() >= 2 {
                    let type_str = parts[0].trim_start_matches('@').to_lowercase();
                    let entry_type = Self::parse_entry_type(&type_str);

                    let key_part = parts[1].split(',').next().unwrap_or("");
                    let key = key_part.trim().to_string();

                    current_entry = Some(BibEntry::new(key, entry_type));
                    in_entry = true;
                }
            } else if in_entry && line == "}" {
                // 结束条目
                if let Some(entry) = current_entry.take() {
                    self.add_entry(entry);
                }
                in_entry = false;
            } else if in_entry {
                // 解析字段
                if let Some(entry) = &mut current_entry {
                    if let Some((key, value)) = Self::parse_field(line) {
                        Self::set_field(entry, key, value);
                    }
                }
            }
        }

        // 保存最后一个条目
        if let Some(entry) = current_entry {
            self.add_entry(entry);
        }

        Ok(())
    }

    fn parse_entry_type(type_str: &str) -> BibEntryType {
        match type_str {
            "article" => BibEntryType::Article,
            "book" => BibEntryType::Book,
            "booklet" => BibEntryType::Booklet,
            "conference" | "inproceedings" => BibEntryType::Conference,
            "inbook" => BibEntryType::InBook,
            "incollection" => BibEntryType::InCollection,
            "manual" => BibEntryType::Manual,
            "mastersthesis" => BibEntryType::MasterThesis,
            "misc" => BibEntryType::Misc,
            "phdthesis" => BibEntryType::PhDThesis,
            "proceedings" => BibEntryType::Proceedings,
            "techreport" => BibEntryType::TechReport,
            "unpublished" => BibEntryType::Unpublished,
            _ => BibEntryType::Custom(type_str.to_string()),
        }
    }

    fn parse_field(line: &str) -> Option<(String, String)> {
        let line = line.trim_end_matches(',');
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_lowercase();
            let value = parts[1]
                .trim()
                .trim_matches('{')
                .trim_matches('}')
                .trim_matches('"')
                .to_string();
            Some((key, value))
        } else {
            None
        }
    }

    fn set_field(entry: &mut BibEntry, key: String, value: String) {
        match key.as_str() {
            "author" => entry.author = Some(value),
            "title" => entry.title = Some(value),
            "year" => entry.year = Some(value),
            "journal" => entry.journal = Some(value),
            "volume" => entry.volume = Some(value),
            "number" => entry.number = Some(value),
            "pages" => entry.pages = Some(value),
            "publisher" => entry.publisher = Some(value),
            "address" => entry.address = Some(value),
            "edition" => entry.edition = Some(value),
            "month" => entry.month = Some(value),
            "note" => entry.note = Some(value),
            "url" => entry.url = Some(value),
            "doi" => entry.doi = Some(value),
            _ => {
                entry.extra_fields.insert(key, value);
            }
        }
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        typst.push_str("#bibliography(\n");

        // 添加标题
        if let Some(title) = &self.config.title {
            typst.push_str(&format!("  title: [{}],\n", title));
        }

        // 添加样式
        typst.push_str(&format!("  style: \"{}\",\n", self.style_to_typst()));

        // 添加 full
        if self.config.full {
            typst.push_str("  full: true,\n");
        }

        typst.push_str(")\n");

        typst
    }

    fn style_to_typst(&self) -> String {
        match self.config.style {
            CitationStyle::APA => "apa".to_string(),
            CitationStyle::MLA => "mla".to_string(),
            CitationStyle::Chicago => "chicago".to_string(),
            CitationStyle::IEEE => "ieee".to_string(),
            CitationStyle::Harvard => "harvard".to_string(),
            CitationStyle::Vancouver => "vancouver".to_string(),
            CitationStyle::Custom(ref style) => style.clone(),
        }
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        html.push_str("<div class=\"typst-bibliography\">\n");

        if let Some(title) = &self.config.title {
            html.push_str(&format!(
                "  <h2 class=\"bibliography-title\">{}</h2>\n",
                html_escape(title)
            ));
        }

        html.push_str("  <ul class=\"bibliography-list\">\n");

        for entry in &self.entries {
            if self.config.full || self.cited_keys.contains(&entry.key) {
                html.push_str("    <li class=\"bibliography-entry\">\n");
                html.push_str(&format!(
                    "      <span class=\"entry-key\">[{}]</span> ",
                    entry.key
                ));
                html.push_str(&self.format_entry_html(entry));
                html.push_str("    </li>\n");
            }
        }

        html.push_str("  </ul>\n");
        html.push_str("</div>\n");

        html
    }

    fn format_entry_html(&self, entry: &BibEntry) -> String {
        let mut html = String::new();

        if let Some(author) = &entry.author {
            html.push_str(&format!("{}. ", html_escape(author)));
        }

        if let Some(title) = &entry.title {
            html.push_str(&format!("\"{}\". ", html_escape(title)));
        }

        if let Some(journal) = &entry.journal {
            html.push_str(&format!("*{}*, ", html_escape(journal)));
        }

        if let Some(volume) = &entry.volume {
            html.push_str(&format!(
                "{}{}, ",
                volume,
                if entry.number.is_some() { "(" } else { "" }
            ));
        }

        if let Some(number) = &entry.number {
            html.push_str(&format!("{}) ", number));
        }

        if let Some(year) = &entry.year {
            html.push_str(&format!("{}.", year));
        }

        html
    }

    /// 获取条目
    pub fn get_entry(&self, key: &str) -> Option<&BibEntry> {
        self.entries.iter().find(|e| e.key == key)
    }
}

impl Default for Bibliography {
    fn default() -> Self {
        Self::new()
    }
}

/// 参考文献构建器
pub struct BibliographyBuilder {
    bibliography: Bibliography,
}

impl BibliographyBuilder {
    pub fn new() -> Self {
        Self {
            bibliography: Bibliography::new(),
        }
    }

    pub fn with_config(mut self, config: BibliographyConfig) -> Self {
        self.bibliography.config = config;
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.bibliography.config.title = Some(title);
        self
    }

    pub fn with_style(mut self, style: CitationStyle) -> Self {
        self.bibliography.config.style = style;
        self
    }

    pub fn full(mut self, full: bool) -> Self {
        self.bibliography.config.full = full;
        self
    }

    pub fn add_entry(mut self, entry: BibEntry) -> Self {
        self.bibliography.add_entry(entry);
        self
    }

    pub fn build(self) -> Bibliography {
        self.bibliography
    }
}

impl Default for BibliographyBuilder {
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
    fn test_bibliography_creation() {
        let bib = Bibliography::new();
        assert!(bib.entries.is_empty());
    }

    #[test]
    fn test_bibliography_default() {
        let bib = Bibliography::default();
        assert!(bib.entries.is_empty());
        assert_eq!(bib.config.title, Some("References".to_string()));
    }

    #[test]
    fn test_bib_entry_creation() {
        let entry = BibEntry::new("key1".to_string(), BibEntryType::Article);
        assert_eq!(entry.key, "key1");
        assert_eq!(entry.entry_type, BibEntryType::Article);
    }

    #[test]
    fn test_bib_entry_with_author() {
        let entry = BibEntry::new("key1".to_string(), BibEntryType::Article)
            .with_author("John Doe".to_string());
        assert_eq!(entry.author, Some("John Doe".to_string()));
    }

    #[test]
    fn test_bib_entry_with_title() {
        let entry = BibEntry::new("key1".to_string(), BibEntryType::Article)
            .with_title("Test Title".to_string());
        assert_eq!(entry.title, Some("Test Title".to_string()));
    }

    #[test]
    fn test_bibliography_add_entry() {
        let mut bib = Bibliography::new();
        bib.add_entry(BibEntry::new("key1".to_string(), BibEntryType::Article));
        assert_eq!(bib.entries.len(), 1);
    }

    #[test]
    fn test_bibliography_cite() {
        let mut bib = Bibliography::new();
        bib.cite("key1".to_string());
        bib.cite("key2".to_string());
        bib.cite("key1".to_string()); // 重复引用
        assert_eq!(bib.cited_keys.len(), 2);
    }

    #[test]
    fn test_parse_bibtex() {
        let bibtex = r#"@article{key1,
  author = {John Doe},
  title = {Test Title},
  year = {2020}
}"#;
        let mut bib = Bibliography::new();
        assert!(bib.parse_bibtex(bibtex).is_ok());
        assert_eq!(bib.entries.len(), 1);
    }

    #[test]
    fn test_parse_entry_type() {
        assert_eq!(
            Bibliography::parse_entry_type("article"),
            BibEntryType::Article
        );
        assert_eq!(Bibliography::parse_entry_type("book"), BibEntryType::Book);
        assert_eq!(
            Bibliography::parse_entry_type("unknown"),
            BibEntryType::Custom("unknown".to_string())
        );
    }

    #[test]
    fn test_parse_field() {
        let result = Bibliography::parse_field("author = {John Doe},");
        assert!(result.is_some());
        let (key, value) = result.unwrap();
        assert_eq!(key, "author");
        assert_eq!(value, "John Doe");
    }

    #[test]
    fn test_to_typst() {
        let bib = Bibliography::new();
        let typst = bib.to_typst();
        assert!(typst.contains("#bibliography("));
        assert!(typst.contains("title: [References]"));
    }

    #[test]
    fn test_to_html() {
        let bib = Bibliography::new();
        let html = bib.to_html();
        assert!(html.contains("<div class=\"typst-bibliography\""));
        assert!(html.contains("<h2 class=\"bibliography-title\">References</h2>"));
    }

    #[test]
    fn test_citation_style_partial_eq() {
        assert_eq!(CitationStyle::APA, CitationStyle::APA);
        assert_ne!(CitationStyle::APA, CitationStyle::MLA);
    }

    #[test]
    fn test_bib_entry_type_partial_eq() {
        assert_eq!(BibEntryType::Article, BibEntryType::Article);
        assert_ne!(BibEntryType::Article, BibEntryType::Book);
    }

    #[test]
    fn test_bibliography_builder() {
        let bib = BibliographyBuilder::new()
            .with_title("Works Cited".to_string())
            .with_style(CitationStyle::MLA)
            .add_entry(BibEntry::new("key1".to_string(), BibEntryType::Article))
            .build();

        assert_eq!(bib.config.title, Some("Works Cited".to_string()));
        assert_eq!(bib.config.style, CitationStyle::MLA);
    }

    #[test]
    fn test_bibliography_builder_default() {
        let builder = BibliographyBuilder::default();
        let bib = builder.build();
        assert!(bib.entries.is_empty());
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_get_entry() {
        let mut bib = Bibliography::new();
        bib.add_entry(BibEntry::new("key1".to_string(), BibEntryType::Article));
        assert!(bib.get_entry("key1").is_some());
        assert!(bib.get_entry("key2").is_none());
    }

    #[test]
    fn test_bibliography_config_default() {
        let config = BibliographyConfig::default();
        assert_eq!(config.title, Some("References".to_string()));
        assert_eq!(config.style, CitationStyle::APA);
        assert!(!config.full);
    }

    #[test]
    fn test_bib_entry_with_extra_field() {
        let entry = BibEntry::new("key1".to_string(), BibEntryType::Article)
            .add_extra_field("custom".to_string(), "value".to_string());
        assert!(entry.extra_fields.contains_key("custom"));
    }

    #[test]
    fn test_citation_style_variants() {
        assert_eq!(CitationStyle::APA, CitationStyle::APA);
        assert_eq!(CitationStyle::MLA, CitationStyle::MLA);
        assert_eq!(CitationStyle::Chicago, CitationStyle::Chicago);
        assert_eq!(CitationStyle::IEEE, CitationStyle::IEEE);
        assert_eq!(CitationStyle::Harvard, CitationStyle::Harvard);
        assert_eq!(CitationStyle::Vancouver, CitationStyle::Vancouver);
    }

    #[test]
    fn test_bib_entry_type_variants() {
        assert_eq!(BibEntryType::Article, BibEntryType::Article);
        assert_eq!(BibEntryType::Book, BibEntryType::Book);
        assert_eq!(BibEntryType::Conference, BibEntryType::Conference);
        assert_eq!(BibEntryType::PhDThesis, BibEntryType::PhDThesis);
    }

    #[test]
    fn test_bibliography_with_config() {
        let config = BibliographyConfig {
            title: Some("Works Cited".to_string()),
            style: CitationStyle::MLA,
            full: true,
            sort_by: Some("author".to_string()),
        };
        let bib = Bibliography::with_config(config);
        assert_eq!(bib.config.title, Some("Works Cited".to_string()));
        assert!(bib.config.full);
    }

    #[test]
    fn test_bibliography_to_html_with_entries() {
        let mut bib = Bibliography::new();
        let entry = BibEntry::new("key1".to_string(), BibEntryType::Article)
            .with_author("John Doe".to_string())
            .with_title("Test Title".to_string())
            .with_year("2020".to_string());
        bib.add_entry(entry);
        bib.cite("key1".to_string());
        let html = bib.to_html();
        assert!(html.contains("John Doe"));
        assert!(html.contains("Test Title"));
    }

    #[test]
    fn test_bibliography_full_mode() {
        let mut bib = Bibliography::new();
        bib.config.full = true;
        bib.add_entry(BibEntry::new("key1".to_string(), BibEntryType::Article));
        let html = bib.to_html();
        assert!(html.contains("key1"));
    }

    #[test]
    fn test_bibliography_cited_only_mode() {
        let mut bib = Bibliography::new();
        bib.config.full = false;
        bib.add_entry(BibEntry::new("key1".to_string(), BibEntryType::Article));
        bib.add_entry(BibEntry::new("key2".to_string(), BibEntryType::Article));
        bib.cite("key1".to_string());
        let html = bib.to_html();
        assert!(html.contains("key1"));
        assert!(!html.contains("key2"));
    }
}
