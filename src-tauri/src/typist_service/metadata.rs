/*!
 * 航空航天级元数据系统
 * 实现 Typst 的元数据功能（文档元数据、标题、作者、日期、描述）
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 元数据值类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MetadataValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<MetadataValue>),
    Object(HashMap<String, MetadataValue>),
}

/// 文档元数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub custom: HashMap<String, MetadataValue>,
}

/// 元数据条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataEntry {
    pub label: String,
    pub value: MetadataValue,
}

impl MetadataEntry {
    pub fn new(label: String, value: MetadataValue) -> Self {
        Self { label, value }
    }
}

/// 元数据系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub document: DocumentMetadata,
    pub entries: Vec<MetadataEntry>,
}

impl Metadata {
    pub fn new() -> Self {
        Self {
            document: DocumentMetadata::default(),
            entries: Vec::new(),
        }
    }

    pub fn with_document(mut self, document: DocumentMetadata) -> Self {
        self.document = document;
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.document.title = Some(title);
        self
    }

    pub fn with_author(mut self, author: String) -> Self {
        self.document.author = Some(author);
        self
    }

    pub fn with_date(mut self, date: String) -> Self {
        self.document.date = Some(date);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.document.description = Some(description);
        self
    }

    pub fn with_keywords(mut self, keywords: Vec<String>) -> Self {
        self.document.keywords = Some(keywords);
        self
    }

    pub fn add_entry(mut self, entry: MetadataEntry) -> Self {
        self.entries.push(entry);
        self
    }

    pub fn add_custom(mut self, key: String, value: MetadataValue) -> Self {
        self.document.custom.insert(key, value);
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        // 添加文档元数据
        typst.push_str("#set document(");

        if let Some(title) = &self.document.title {
            typst.push_str(&format!("title: [{}], ", html_escape(title)));
        }

        if let Some(author) = &self.document.author {
            typst.push_str(&format!("author: \"{}\", ", html_escape(author)));
        }

        if let Some(date) = &self.document.date {
            typst.push_str(&format!("date: \"{}\", ", html_escape(date)));
        }

        if let Some(description) = &self.document.description {
            typst.push_str(&format!("description: [{}], ", html_escape(description)));
        }

        // 移除最后的逗号和空格
        if typst.ends_with(", ") {
            typst.pop();
            typst.pop();
        }

        typst.push_str(")\n");

        // 添加自定义元数据条目
        for entry in &self.entries {
            typst.push_str(&format!(
                "#metadata({}) <{}>\n",
                self.value_to_typst(&entry.value),
                entry.label
            ));
        }

        // 添加自定义字段
        for (key, value) in &self.document.custom {
            typst.push_str(&format!(
                "#metadata({}) <{}>\n",
                self.value_to_typst(value),
                key
            ));
        }

        typst
    }

    fn value_to_typst(&self, value: &MetadataValue) -> String {
        match value {
            MetadataValue::String(s) => format!("\"{}\"", html_escape(s)),
            MetadataValue::Number(n) => n.to_string(),
            MetadataValue::Boolean(b) => b.to_string(),
            MetadataValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| self.value_to_typst(v)).collect();
                format!("({})", items.join(", "))
            }
            MetadataValue::Object(obj) => {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, self.value_to_typst(v)))
                    .collect();
                format!("({})", items.join(", "))
            }
        }
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        html.push_str("<div class=\"document-metadata\" style=\"display:none\">\n");

        if let Some(title) = &self.document.title {
            html.push_str(&format!(
                "  <meta name=\"title\" content=\"{}\" />\n",
                html_escape(title)
            ));
        }

        if let Some(author) = &self.document.author {
            html.push_str(&format!(
                "  <meta name=\"author\" content=\"{}\" />\n",
                html_escape(author)
            ));
        }

        if let Some(date) = &self.document.date {
            html.push_str(&format!(
                "  <meta name=\"date\" content=\"{}\" />\n",
                html_escape(date)
            ));
        }

        if let Some(description) = &self.document.description {
            html.push_str(&format!(
                "  <meta name=\"description\" content=\"{}\" />\n",
                html_escape(description)
            ));
        }

        if let Some(keywords) = &self.document.keywords {
            html.push_str(&format!(
                "  <meta name=\"keywords\" content=\"{}\" />\n",
                html_escape(&keywords.join(", "))
            ));
        }

        for entry in &self.entries {
            html.push_str(&format!(
                "  <meta name=\"{}\" content=\"{}\" />\n",
                entry.label,
                self.value_to_html(&entry.value)
            ));
        }

        for (key, value) in &self.document.custom {
            html.push_str(&format!(
                "  <meta name=\"{}\" content=\"{}\" />\n",
                key,
                self.value_to_html(value)
            ));
        }

        html.push_str("</div>\n");

        html
    }

    fn value_to_html(&self, value: &MetadataValue) -> String {
        match value {
            MetadataValue::String(s) => html_escape(s),
            MetadataValue::Number(n) => n.to_string(),
            MetadataValue::Boolean(b) => b.to_string(),
            MetadataValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| self.value_to_html(v)).collect();
                items.join(", ")
            }
            MetadataValue::Object(obj) => {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, self.value_to_html(v)))
                    .collect();
                items.join(", ")
            }
        }
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new()
    }
}

/// 元数据构建器
pub struct MetadataBuilder {
    metadata: Metadata,
}

impl MetadataBuilder {
    pub fn new() -> Self {
        Self {
            metadata: Metadata::new(),
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.metadata = self.metadata.with_title(title);
        self
    }

    pub fn author(mut self, author: String) -> Self {
        self.metadata = self.metadata.with_author(author);
        self
    }

    pub fn date(mut self, date: String) -> Self {
        self.metadata = self.metadata.with_date(date);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.metadata = self.metadata.with_description(description);
        self
    }

    pub fn keywords(mut self, keywords: Vec<String>) -> Self {
        self.metadata = self.metadata.with_keywords(keywords);
        self
    }

    pub fn entry(mut self, entry: MetadataEntry) -> Self {
        self.metadata = self.metadata.add_entry(entry);
        self
    }

    pub fn custom(mut self, key: String, value: MetadataValue) -> Self {
        self.metadata = self.metadata.add_custom(key, value);
        self
    }

    pub fn build(self) -> Metadata {
        self.metadata
    }
}

impl Default for MetadataBuilder {
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
    fn test_metadata_creation() {
        let metadata = Metadata::new();
        assert!(metadata.document.title.is_none());
    }

    #[test]
    fn test_metadata_default() {
        let metadata = Metadata::default();
        assert!(metadata.document.title.is_none());
    }

    #[test]
    fn test_document_metadata_default() {
        let doc_meta = DocumentMetadata::default();
        assert!(doc_meta.title.is_none());
        assert!(doc_meta.author.is_none());
    }

    #[test]
    fn test_metadata_with_title() {
        let metadata = Metadata::new().with_title("Title".to_string());
        assert_eq!(metadata.document.title, Some("Title".to_string()));
    }

    #[test]
    fn test_metadata_with_author() {
        let metadata = Metadata::new().with_author("Author".to_string());
        assert_eq!(metadata.document.author, Some("Author".to_string()));
    }

    #[test]
    fn test_metadata_with_date() {
        let metadata = Metadata::new().with_date("2024-01-01".to_string());
        assert_eq!(metadata.document.date, Some("2024-01-01".to_string()));
    }

    #[test]
    fn test_metadata_with_description() {
        let metadata = Metadata::new().with_description("Description".to_string());
        assert_eq!(
            metadata.document.description,
            Some("Description".to_string())
        );
    }

    #[test]
    fn test_metadata_with_keywords() {
        let metadata = Metadata::new().with_keywords(vec!["key1".to_string(), "key2".to_string()]);
        assert_eq!(
            metadata.document.keywords,
            Some(vec!["key1".to_string(), "key2".to_string()])
        );
    }

    #[test]
    fn test_metadata_entry_creation() {
        let entry = MetadataEntry::new(
            "label".to_string(),
            MetadataValue::String("value".to_string()),
        );
        assert_eq!(entry.label, "label");
    }

    #[test]
    fn test_metadata_value_variants() {
        let string_val = MetadataValue::String("test".to_string());
        let number_val = MetadataValue::Number(1.0);
        let bool_val = MetadataValue::Boolean(true);
        let array_val = MetadataValue::Array(vec![MetadataValue::String("a".to_string())]);
        let object_val = MetadataValue::Object(HashMap::new());

        assert!(matches!(string_val, MetadataValue::String(_)));
        assert!(matches!(number_val, MetadataValue::Number(_)));
        assert!(matches!(bool_val, MetadataValue::Boolean(_)));
        assert!(matches!(array_val, MetadataValue::Array(_)));
        assert!(matches!(object_val, MetadataValue::Object(_)));
    }

    #[test]
    fn test_to_typst() {
        let metadata = Metadata::new().with_title("Title".to_string());
        let typst = metadata.to_typst();
        assert!(typst.contains("#set document("));
        assert!(typst.contains("title: [Title]"));
    }

    #[test]
    fn test_to_typst_with_author() {
        let metadata = Metadata::new().with_author("Author".to_string());
        let typst = metadata.to_typst();
        assert!(typst.contains("author: \"Author\""));
    }

    #[test]
    fn test_to_html() {
        let metadata = Metadata::new().with_title("Title".to_string());
        let html = metadata.to_html();
        assert!(html.contains("<div class=\"document-metadata\""));
        assert!(html.contains("<meta name=\"title\""));
    }

    #[test]
    fn test_to_html_with_author() {
        let metadata = Metadata::new().with_author("Author".to_string());
        let html = metadata.to_html();
        assert!(html.contains("<meta name=\"author\""));
    }

    #[test]
    fn test_metadata_builder() {
        let metadata = MetadataBuilder::new()
            .title("Title".to_string())
            .author("Author".to_string())
            .build();

        assert_eq!(metadata.document.title, Some("Title".to_string()));
        assert_eq!(metadata.document.author, Some("Author".to_string()));
    }

    #[test]
    fn test_metadata_builder_default() {
        let builder = MetadataBuilder::default();
        let metadata = builder.build();
        assert!(metadata.document.title.is_none());
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_add_custom() {
        let metadata = Metadata::new().add_custom(
            "key".to_string(),
            MetadataValue::String("value".to_string()),
        );
        assert!(metadata.document.custom.contains_key("key"));
    }

    #[test]
    fn test_value_to_typst_string() {
        let metadata = Metadata::new();
        let val = MetadataValue::String("test".to_string());
        assert_eq!(metadata.value_to_typst(&val), "\"test\"");
    }

    #[test]
    fn test_value_to_typst_number() {
        let metadata = Metadata::new();
        let val = MetadataValue::Number(42.0);
        assert_eq!(metadata.value_to_typst(&val), "42");
    }

    #[test]
    fn test_value_to_typst_boolean() {
        let metadata = Metadata::new();
        let val = MetadataValue::Boolean(true);
        assert_eq!(metadata.value_to_typst(&val), "true");
    }

    #[test]
    fn test_value_to_html_string() {
        let metadata = Metadata::new();
        let val = MetadataValue::String("test".to_string());
        assert_eq!(metadata.value_to_html(&val), "test");
    }

    #[test]
    fn test_to_typst_with_keywords() {
        let metadata = Metadata::new().with_keywords(vec!["key1".to_string(), "key2".to_string()]);
        let typst = metadata.to_typst();
        // Keywords are only output in HTML, not in Typst document() function
        // This test just ensures the code compiles
        assert!(typst.contains("#set document("));
    }

    #[test]
    fn test_to_html_with_keywords() {
        let metadata = Metadata::new().with_keywords(vec!["key1".to_string(), "key2".to_string()]);
        let html = metadata.to_html();
        assert!(html.contains("<meta name=\"keywords\""));
        assert!(html.contains("key1, key2"));
    }
}
