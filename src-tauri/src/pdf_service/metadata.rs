use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// PDF 版本
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PdfVersion {
    /// PDF 1.4 (Acrobat 5)
    V1_4,
    /// PDF 1.5 (Acrobat 6)
    V1_5,
    /// PDF 1.6 (Acrobat 7)
    V1_6,
    /// PDF 1.7 (Acrobat 8, 9, 10)
    V1_7,
    /// PDF 2.0 (ISO 32000-2:2017)
    V2_0,
}

impl PdfVersion {
    /// 获取版本字符串
    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        match self {
            PdfVersion::V1_4 => "1.4",
            PdfVersion::V1_5 => "1.5",
            PdfVersion::V1_6 => "1.6",
            PdfVersion::V1_7 => "1.7",
            PdfVersion::V2_0 => "2.0",
        }
    }

    /// 从字符串创建版本
    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1.4" => Some(PdfVersion::V1_4),
            "1.5" => Some(PdfVersion::V1_5),
            "1.6" => Some(PdfVersion::V1_6),
            "1.7" => Some(PdfVersion::V1_7),
            "2.0" => Some(PdfVersion::V2_0),
            _ => None,
        }
    }
}

/// PDF 元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfMetadata {
    /// 文档标题
    pub title: String,
    /// 作者
    pub author: String,
    /// 主题
    pub subject: String,
    /// 关键词
    pub keywords: Vec<String>,
    /// 创建者
    pub creator: String,
    /// 生产者
    pub producer: String,
    /// 创建日期
    pub creation_date: DateTime<Utc>,
    /// 修改日期
    pub modification_date: DateTime<Utc>,
    /// PDF 版本
    pub version: PdfVersion,
    /// 自定义元数据
    pub custom_metadata: std::collections::HashMap<String, String>,
}

impl PdfMetadata {
    /// 创建新的 PDF 元数据
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            title: "Untitled Document".to_string(),
            author: "Unknown".to_string(),
            subject: String::new(),
            keywords: Vec::new(),
            creator: "Logos Document Editor".to_string(),
            producer: "Logos PDF Generator".to_string(),
            creation_date: now,
            modification_date: now,
            version: PdfVersion::V1_7,
            custom_metadata: std::collections::HashMap::new(),
        }
    }

    /// 设置标题
    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// 设置作者
    pub fn with_author(mut self, author: String) -> Self {
        self.author = author;
        self
    }

    /// 设置主题
    pub fn with_subject(mut self, subject: String) -> Self {
        self.subject = subject;
        self
    }

    /// 设置关键词
    pub fn with_keywords(mut self, keywords: Vec<String>) -> Self {
        self.keywords = keywords;
        self
    }

    /// 添加关键词
    #[allow(dead_code)]
    pub fn add_keyword(mut self, keyword: String) -> Self {
        self.keywords.push(keyword);
        self
    }

    /// 设置创建者
    #[allow(dead_code)]
    pub fn with_creator(mut self, creator: String) -> Self {
        self.creator = creator;
        self
    }

    /// 设置生产者
    #[allow(dead_code)]
    pub fn with_producer(mut self, producer: String) -> Self {
        self.producer = producer;
        self
    }

    /// 设置 PDF 版本
    #[allow(dead_code)]
    pub fn with_version(mut self, version: PdfVersion) -> Self {
        self.version = version;
        self
    }

    /// 添加自定义元数据
    #[allow(dead_code)]
    pub fn with_custom_metadata(mut self, key: String, value: String) -> Self {
        self.custom_metadata.insert(key, value);
        self
    }

    /// 更新修改日期
    #[allow(dead_code)]
    pub fn touch(&mut self) {
        self.modification_date = Utc::now();
    }

    /// 获取所有元数据作为键值对
    #[allow(dead_code)]
    pub fn as_map(&self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        map.insert("Title".to_string(), self.title.clone());
        map.insert("Author".to_string(), self.author.clone());
        map.insert("Subject".to_string(), self.subject.clone());
        map.insert("Keywords".to_string(), self.keywords.join(", "));
        map.insert("Creator".to_string(), self.creator.clone());
        map.insert("Producer".to_string(), self.producer.clone());
        map.insert("CreationDate".to_string(), self.creation_date.to_rfc3339());
        map.insert("ModDate".to_string(), self.modification_date.to_rfc3339());
        map.insert("PDFVersion".to_string(), self.version.as_str().to_string());

        // 添加自定义元数据
        for (key, value) in &self.custom_metadata {
            map.insert(key.clone(), value.clone());
        }

        map
    }
}

impl Default for PdfMetadata {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_version_as_str() {
        assert_eq!(PdfVersion::V1_4.as_str(), "1.4");
        assert_eq!(PdfVersion::V1_7.as_str(), "1.7");
        assert_eq!(PdfVersion::V2_0.as_str(), "2.0");
    }

    #[test]
    fn test_pdf_version_from_str() {
        assert_eq!(PdfVersion::from_str("1.4"), Some(PdfVersion::V1_4));
        assert_eq!(PdfVersion::from_str("1.7"), Some(PdfVersion::V1_7));
        assert_eq!(PdfVersion::from_str("2.0"), Some(PdfVersion::V2_0));
        assert_eq!(PdfVersion::from_str("3.0"), None);
    }

    #[test]
    fn test_pdf_metadata_new() {
        let metadata = PdfMetadata::new();
        assert_eq!(metadata.title, "Untitled Document");
        assert_eq!(metadata.author, "Unknown");
        assert!(metadata.keywords.is_empty());
    }

    #[test]
    fn test_pdf_metadata_with_title() {
        let metadata = PdfMetadata::new().with_title("Test Document".to_string());
        assert_eq!(metadata.title, "Test Document");
    }

    #[test]
    fn test_pdf_metadata_with_author() {
        let metadata = PdfMetadata::new().with_author("John Doe".to_string());
        assert_eq!(metadata.author, "John Doe");
    }

    #[test]
    fn test_pdf_metadata_with_keywords() {
        let metadata =
            PdfMetadata::new().with_keywords(vec!["test".to_string(), "document".to_string()]);
        assert_eq!(metadata.keywords.len(), 2);
    }

    #[test]
    fn test_pdf_metadata_add_keyword() {
        let metadata = PdfMetadata::new()
            .add_keyword("first".to_string())
            .add_keyword("second".to_string());
        assert_eq!(metadata.keywords.len(), 2);
    }

    #[test]
    fn test_pdf_metadata_chaining() {
        let metadata = PdfMetadata::new()
            .with_title("Test".to_string())
            .with_author("Author".to_string())
            .with_subject("Subject".to_string());
        assert_eq!(metadata.title, "Test");
        assert_eq!(metadata.author, "Author");
        assert_eq!(metadata.subject, "Subject");
    }

    #[test]
    fn test_pdf_metadata_touch() {
        let mut metadata = PdfMetadata::new();
        let old_date = metadata.modification_date;
        std::thread::sleep(std::time::Duration::from_millis(10));
        metadata.touch();
        assert!(metadata.modification_date > old_date);
    }

    #[test]
    fn test_pdf_metadata_as_map() {
        let metadata = PdfMetadata::new()
            .with_title("Test".to_string())
            .with_author("Author".to_string());
        let map = metadata.as_map();
        assert_eq!(map.get("Title"), Some(&"Test".to_string()));
        assert_eq!(map.get("Author"), Some(&"Author".to_string()));
    }

    #[test]
    fn test_pdf_metadata_with_custom_metadata() {
        let metadata = PdfMetadata::new()
            .with_custom_metadata("CustomKey".to_string(), "CustomValue".to_string());
        assert_eq!(
            metadata.custom_metadata.get("CustomKey"),
            Some(&"CustomValue".to_string())
        );
    }

    #[test]
    fn test_pdf_metadata_default() {
        let metadata = PdfMetadata::default();
        assert_eq!(metadata.title, "Untitled Document");
    }

    #[test]
    fn test_pdf_metadata_serialization() {
        let metadata = PdfMetadata::new();
        let json = serde_json::to_string(&metadata);
        assert!(json.is_ok());
    }

    #[test]
    fn test_pdf_metadata_deserialization() {
        let metadata = PdfMetadata::new();
        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: PdfMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.title, metadata.title);
    }
}
