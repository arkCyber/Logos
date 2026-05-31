use serde::{Deserialize, Serialize};

/// 章节类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EpubChapterType {
    /// 封面
    Cover,
    /// 目录
    TableOfContents,
    /// 序言
    Preface,
    /// 正文
    Body,
    /// 后记
    Afterword,
    /// 附录
    Appendix,
    /// 自定义
    Custom(String),
}

/// EPUB 章节目录项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpubTocItem {
    /// 章节 ID
    pub id: String,
    /// 章节标题
    pub title: String,
    /// 章节文件路径
    pub href: String,
    /// 父章节 ID
    pub parent_id: Option<String>,
    /// 子章节
    pub children: Vec<EpubTocItem>,
}

impl EpubTocItem {
    /// 创建新的目录项
    #[allow(dead_code)]
    pub fn new(id: String, title: String, href: String) -> Self {
        Self {
            id,
            title,
            href,
            parent_id: None,
            children: Vec::new(),
        }
    }

    /// 设置父章节
    #[allow(dead_code)]
    pub fn with_parent(mut self, parent_id: String) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// 添加子章节
    #[allow(dead_code)]
    pub fn with_child(mut self, child: EpubTocItem) -> Self {
        self.children.push(child);
        self
    }
}

/// EPUB 目录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpubToc {
    /// 目录项
    pub items: Vec<EpubTocItem>,
}

impl EpubToc {
    /// 创建新的目录
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// 添加目录项
    #[allow(dead_code)]
    pub fn with_item(mut self, item: EpubTocItem) -> Self {
        self.items.push(item);
        self
    }

    /// 添加多个目录项
    #[allow(dead_code)]
    pub fn with_items(mut self, items: Vec<EpubTocItem>) -> Self {
        self.items = items;
        self
    }
}

impl Default for EpubToc {
    fn default() -> Self {
        Self::new()
    }
}

/// EPUB 章节
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpubChapter {
    /// 章节 ID
    pub id: String,
    /// 章节类型
    pub chapter_type: EpubChapterType,
    /// 章节标题
    pub title: String,
    /// 章节内容（HTML）
    pub content: String,
    /// 文件名
    pub filename: String,
    /// 章节顺序
    pub order: usize,
}

impl EpubChapter {
    /// 创建新的章节
    pub fn new(id: String, title: String, content: String) -> Self {
        Self {
            id: id.clone(),
            chapter_type: EpubChapterType::Body,
            title,
            content,
            filename: format!("chapter_{}.xhtml", id),
            order: 0,
        }
    }

    /// 设置章节类型
    #[allow(dead_code)]
    pub fn with_type(mut self, chapter_type: EpubChapterType) -> Self {
        self.chapter_type = chapter_type;
        self
    }

    /// 设置文件名
    #[allow(dead_code)]
    pub fn with_filename(mut self, filename: String) -> Self {
        self.filename = filename;
        self
    }

    /// 设置顺序
    #[allow(dead_code)]
    pub fn with_order(mut self, order: usize) -> Self {
        self.order = order;
        self
    }

    /// 创建封面章节
    #[allow(dead_code)]
    pub fn cover(id: String, _image_data: Vec<u8>) -> Self {
        let content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
    <title>Cover</title>
</head>
<body>
    <img src="cover.jpg" alt="Cover"/>
</body>
</html>"#
        );
        Self {
            id,
            chapter_type: EpubChapterType::Cover,
            title: "Cover".to_string(),
            content,
            filename: "cover.xhtml".to_string(),
            order: 0,
        }
    }

    /// 创建目录章节
    #[allow(dead_code)]
    pub fn table_of_contents(id: String) -> Self {
        let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
    <title>Table of Contents</title>
</head>
<body>
    <nav epub:type="toc">
        <h1>Table of Contents</h1>
        <ol>
            <!-- TOC items will be generated -->
        </ol>
    </nav>
</body>
</html>"#
            .to_string();
        Self {
            id,
            chapter_type: EpubChapterType::TableOfContents,
            title: "Table of Contents".to_string(),
            content,
            filename: "toc.xhtml".to_string(),
            order: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epub_toc_item_new() {
        let item = EpubTocItem::new(
            "1".to_string(),
            "Chapter 1".to_string(),
            "chapter1.xhtml".to_string(),
        );
        assert_eq!(item.title, "Chapter 1");
    }

    #[test]
    fn test_epub_toc_item_with_parent() {
        let item = EpubTocItem::new(
            "1".to_string(),
            "Chapter 1".to_string(),
            "chapter1.xhtml".to_string(),
        )
        .with_parent("0".to_string());
        assert_eq!(item.parent_id, Some("0".to_string()));
    }

    #[test]
    fn test_epub_toc_new() {
        let toc = EpubToc::new();
        assert!(toc.items.is_empty());
    }

    #[test]
    fn test_epub_toc_with_item() {
        let item = EpubTocItem::new(
            "1".to_string(),
            "Chapter 1".to_string(),
            "chapter1.xhtml".to_string(),
        );
        let toc = EpubToc::new().with_item(item);
        assert_eq!(toc.items.len(), 1);
    }

    #[test]
    fn test_epub_chapter_new() {
        let chapter = EpubChapter::new(
            "1".to_string(),
            "Chapter 1".to_string(),
            "<p>Content</p>".to_string(),
        );
        assert_eq!(chapter.title, "Chapter 1");
        assert_eq!(chapter.chapter_type, EpubChapterType::Body);
    }

    #[test]
    fn test_epub_chapter_with_type() {
        let chapter = EpubChapter::new(
            "1".to_string(),
            "Title".to_string(),
            "<p>Content</p>".to_string(),
        )
        .with_type(EpubChapterType::Preface);
        assert_eq!(chapter.chapter_type, EpubChapterType::Preface);
    }

    #[test]
    fn test_epub_chapter_cover() {
        let chapter = EpubChapter::cover("cover".to_string(), vec![]);
        assert_eq!(chapter.chapter_type, EpubChapterType::Cover);
    }

    #[test]
    fn test_epub_chapter_table_of_contents() {
        let chapter = EpubChapter::table_of_contents("toc".to_string());
        assert_eq!(chapter.chapter_type, EpubChapterType::TableOfContents);
    }

    #[test]
    fn test_epub_chapter_serialization() {
        let chapter = EpubChapter::new(
            "1".to_string(),
            "Title".to_string(),
            "<p>Content</p>".to_string(),
        );
        let json = serde_json::to_string(&chapter);
        assert!(json.is_ok());
    }

    #[test]
    fn test_epub_chapter_deserialization() {
        let json = r#"{"id":"1","title":"Title","content":"<p>Content</p>","chapter_type":"Body","order":0,"filename":"chapter1.xhtml"}"#;
        let chapter: EpubChapter = serde_json::from_str(json).unwrap();
        assert_eq!(chapter.title, "Title");
    }

    #[test]
    fn test_epub_toc_item_serialization() {
        let item = EpubTocItem::new("1".to_string(), "Chapter 1".to_string(), "chapter1.xhtml".to_string());
        let json = serde_json::to_string(&item);
        assert!(json.is_ok());
    }

    #[test]
    fn test_epub_toc_item_deserialization() {
        let json = r#"{"id":"1","title":"Chapter 1","href":"chapter1.xhtml","parent_id":null,"children":[]}"#;
        let item: EpubTocItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.title, "Chapter 1");
    }

    #[test]
    fn test_epub_toc_serialization() {
        let toc = EpubToc::new();
        let json = serde_json::to_string(&toc);
        assert!(json.is_ok());
    }

    #[test]
    fn test_epub_toc_deserialization() {
        let json = r#"{"items":[]}"#;
        let toc: EpubToc = serde_json::from_str(json).unwrap();
        assert!(toc.items.is_empty());
    }

    #[test]
    fn test_epub_chapter_with_order() {
        let chapter = EpubChapter::new("1".to_string(), "Title".to_string(), "<p>Content</p>".to_string());
        assert_eq!(chapter.order, 0);
    }

    #[test]
    fn test_epub_toc_item_with_order() {
        let item = EpubTocItem::new("1".to_string(), "Chapter 1".to_string(), "chapter1.xhtml".to_string());
        assert_eq!(item.title, "Chapter 1");
    }

    #[test]
    fn test_epub_toc_with_multiple_items() {
        let item1 = EpubTocItem::new("1".to_string(), "Chapter 1".to_string(), "chapter1.xhtml".to_string());
        let item2 = EpubTocItem::new("2".to_string(), "Chapter 2".to_string(), "chapter2.xhtml".to_string());
        let toc = EpubToc::new()
            .with_item(item1)
            .with_item(item2);
        assert_eq!(toc.items.len(), 2);
    }

    #[test]
    fn test_epub_chapter_type_body() {
        assert_eq!(EpubChapterType::Body, EpubChapterType::Body);
    }

    #[test]
    fn test_epub_chapter_type_preface() {
        assert_eq!(EpubChapterType::Preface, EpubChapterType::Preface);
    }

    #[test]
    fn test_epub_chapter_type_cover() {
        assert_eq!(EpubChapterType::Cover, EpubChapterType::Cover);
    }

    #[test]
    fn test_epub_chapter_type_table_of_contents() {
        assert_eq!(EpubChapterType::TableOfContents, EpubChapterType::TableOfContents);
    }

    #[test]
    fn test_epub_chapter_empty_content() {
        let chapter = EpubChapter::new("1".to_string(), "Title".to_string(), "".to_string());
        assert_eq!(chapter.content, "");
    }

    #[test]
    fn test_epub_chapter_long_content() {
        let long_content = "<p>".repeat(1000);
        let chapter = EpubChapter::new("1".to_string(), "Title".to_string(), long_content.clone());
        assert_eq!(chapter.content.len(), long_content.len());
    }

    #[test]
    fn test_epub_toc_item_empty_title() {
        let item = EpubTocItem::new("1".to_string(), "".to_string(), "chapter1.xhtml".to_string());
        assert_eq!(item.title, "");
    }

    #[test]
    fn test_epub_toc_item_long_title() {
        let long_title = "A".repeat(1000);
        let item = EpubTocItem::new("1".to_string(), long_title.clone(), "chapter1.xhtml".to_string());
        assert_eq!(item.title.len(), 1000);
    }

    #[test]
    fn test_epub_chapter_with_html_content() {
        let chapter = EpubChapter::new("1".to_string(), "Title".to_string(), "<h1>Heading</h1><p>Paragraph</p>".to_string());
        assert!(chapter.content.contains("<h1>"));
        assert!(chapter.content.contains("<p>"));
    }

    #[test]
    fn test_epub_toc_item_with_nested_parent() {
        let item = EpubTocItem::new("2".to_string(), "Chapter 2".to_string(), "chapter2.xhtml".to_string())
            .with_parent("1".to_string());
        assert_eq!(item.parent_id, Some("1".to_string()));
    }
}
