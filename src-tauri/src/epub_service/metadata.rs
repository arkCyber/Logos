use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// EPUB 标识符
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpubIdentifier {
    /// 标识符类型（ISBN、UUID等）
    pub id_type: String,
    /// 标识符值
    pub value: String,
}

impl EpubIdentifier {
    /// 创建新的标识符
    pub fn new(id_type: String, value: String) -> Self {
        Self { id_type, value }
    }

    /// 创建 UUID 标识符
    pub fn uuid() -> Self {
        Self::new("uuid".to_string(), uuid::Uuid::new_v4().to_string())
    }
}

/// EPUB 创建者
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpubCreator {
    /// 创建者名称
    pub name: String,
    /// 角色（作者、编辑、译者等）
    pub role: Option<String>,
}

impl EpubCreator {
    /// 创建新的创建者
    #[allow(dead_code)]
    pub fn new(name: String) -> Self {
        Self { name, role: None }
    }

    /// 设置角色
    #[allow(dead_code)]
    pub fn with_role(mut self, role: String) -> Self {
        self.role = Some(role);
        self
    }
}

/// EPUB 元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpubMetadata {
    /// 标题
    pub title: String,
    /// 语言
    pub language: String,
    /// 标识符
    pub identifier: EpubIdentifier,
    /// 创建者
    pub creators: Vec<EpubCreator>,
    /// 主题/关键词
    pub subjects: Vec<String>,
    /// 描述
    pub description: Option<String>,
    /// 出版商
    pub publisher: Option<String>,
    /// 出版日期
    pub date: Option<DateTime<Utc>>,
    /// 版权
    pub rights: Option<String>,
    /// 封面图片（可选）
    pub cover_image: Option<Vec<u8>>,
}

impl EpubMetadata {
    /// 创建新的元数据
    pub fn new(title: String, language: String) -> Self {
        Self {
            title,
            language,
            identifier: EpubIdentifier::uuid(),
            creators: Vec::new(),
            subjects: Vec::new(),
            description: None,
            publisher: None,
            date: Some(Utc::now()),
            rights: None,
            cover_image: None,
        }
    }

    /// 设置标识符
    #[allow(dead_code)]
    pub fn with_identifier(mut self, identifier: EpubIdentifier) -> Self {
        self.identifier = identifier;
        self
    }

    /// 添加创建者
    #[allow(dead_code)]
    pub fn with_creator(mut self, creator: EpubCreator) -> Self {
        self.creators.push(creator);
        self
    }

    /// 添加主题
    #[allow(dead_code)]
    pub fn with_subject(mut self, subject: String) -> Self {
        self.subjects.push(subject);
        self
    }

    /// 设置描述
    #[allow(dead_code)]
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置出版商
    #[allow(dead_code)]
    pub fn with_publisher(mut self, publisher: String) -> Self {
        self.publisher = Some(publisher);
        self
    }

    /// 设置版权
    #[allow(dead_code)]
    pub fn with_rights(mut self, rights: String) -> Self {
        self.rights = Some(rights);
        self
    }

    /// 设置封面图片
    #[allow(dead_code)]
    pub fn with_cover_image(mut self, image: Vec<u8>) -> Self {
        self.cover_image = Some(image);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epub_identifier_new() {
        let id = EpubIdentifier::new("isbn".to_string(), "1234567890".to_string());
        assert_eq!(id.id_type, "isbn");
    }

    #[test]
    fn test_epub_identifier_uuid() {
        let id = EpubIdentifier::uuid();
        assert_eq!(id.id_type, "uuid");
    }

    #[test]
    fn test_epub_creator_new() {
        let creator = EpubCreator::new("Author".to_string());
        assert_eq!(creator.name, "Author");
    }

    #[test]
    fn test_epub_creator_with_role() {
        let creator = EpubCreator::new("Author".to_string()).with_role("author".to_string());
        assert_eq!(creator.role, Some("author".to_string()));
    }

    #[test]
    fn test_epub_metadata_new() {
        let metadata = EpubMetadata::new("Book Title".to_string(), "en".to_string());
        assert_eq!(metadata.title, "Book Title");
        assert_eq!(metadata.language, "en");
    }

    #[test]
    fn test_epub_metadata_with_creator() {
        let creator = EpubCreator::new("Author".to_string());
        let metadata =
            EpubMetadata::new("Title".to_string(), "en".to_string()).with_creator(creator);
        assert_eq!(metadata.creators.len(), 1);
    }

    #[test]
    fn test_epub_metadata_serialization() {
        let metadata = EpubMetadata::new("Title".to_string(), "en".to_string());
        let json = serde_json::to_string(&metadata);
        assert!(json.is_ok());
    }

    #[test]
    fn test_epub_metadata_with_publisher() {
        let metadata = EpubMetadata::new("Title".to_string(), "en".to_string())
            .with_publisher("Publisher".to_string());
        assert_eq!(metadata.publisher, Some("Publisher".to_string()));
    }

    #[test]
    fn test_epub_metadata_with_description() {
        let metadata = EpubMetadata::new("Title".to_string(), "en".to_string())
            .with_description("Description".to_string());
        assert_eq!(metadata.description, Some("Description".to_string()));
    }

    #[test]
    fn test_epub_metadata_with_rights() {
        let metadata = EpubMetadata::new("Title".to_string(), "en".to_string())
            .with_rights("Copyright".to_string());
        assert_eq!(metadata.rights, Some("Copyright".to_string()));
    }

    #[test]
    fn test_epub_metadata_with_identifier() {
        let id = EpubIdentifier::new("isbn".to_string(), "1234567890".to_string());
        let metadata = EpubMetadata::new("Title".to_string(), "en".to_string())
            .with_identifier(id);
        assert_eq!(metadata.identifier.id_type, "isbn");
    }

    #[test]
    fn test_epub_metadata_with_multiple_creators() {
        let creator1 = EpubCreator::new("Author1".to_string());
        let creator2 = EpubCreator::new("Author2".to_string());
        let metadata = EpubMetadata::new("Title".to_string(), "en".to_string())
            .with_creator(creator1)
            .with_creator(creator2);
        assert_eq!(metadata.creators.len(), 2);
    }

    #[test]
    fn test_epub_identifier_serialization() {
        let id = EpubIdentifier::new("isbn".to_string(), "1234567890".to_string());
        let json = serde_json::to_string(&id);
        assert!(json.is_ok());
    }

    #[test]
    fn test_epub_identifier_deserialization() {
        let json = r#"{"id_type":"isbn","value":"1234567890"}"#;
        let id: EpubIdentifier = serde_json::from_str(json).unwrap();
        assert_eq!(id.id_type, "isbn");
        assert_eq!(id.value, "1234567890");
    }

    #[test]
    fn test_epub_creator_serialization() {
        let creator = EpubCreator::new("Author".to_string());
        let json = serde_json::to_string(&creator);
        assert!(json.is_ok());
    }

    #[test]
    fn test_epub_creator_deserialization() {
        let json = r#"{"name":"Author","role":null}"#;
        let creator: EpubCreator = serde_json::from_str(json).unwrap();
        assert_eq!(creator.name, "Author");
    }

    #[test]
    fn test_epub_metadata_empty_title() {
        let metadata = EpubMetadata::new("".to_string(), "en".to_string());
        assert_eq!(metadata.title, "");
    }

    #[test]
    fn test_epub_metadata_long_title() {
        let long_title = "A".repeat(1000);
        let metadata = EpubMetadata::new(long_title.clone(), "en".to_string());
        assert_eq!(metadata.title.len(), 1000);
    }

    #[test]
    fn test_epub_metadata_with_subjects() {
        let metadata = EpubMetadata::new("Title".to_string(), "en".to_string())
            .with_subject("Fiction".to_string());
        assert_eq!(metadata.subjects.len(), 1);
    }
}
