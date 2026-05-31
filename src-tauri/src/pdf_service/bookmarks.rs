use serde::{Deserialize, Serialize};

/// 书签层级
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BookmarkLevel {
    /// 一级标题
    Level1,
    /// 二级标题
    Level2,
    /// 三级标题
    Level3,
    /// 四级标题
    Level4,
    /// 五级标题
    Level5,
    /// 六级标题
    Level6,
}

impl BookmarkLevel {
    /// 获取层级数值
    #[allow(dead_code)]
    pub fn as_number(&self) -> u8 {
        match self {
            BookmarkLevel::Level1 => 1,
            BookmarkLevel::Level2 => 2,
            BookmarkLevel::Level3 => 3,
            BookmarkLevel::Level4 => 4,
            BookmarkLevel::Level5 => 5,
            BookmarkLevel::Level6 => 6,
        }
    }

    /// 从数值创建层级
    #[allow(dead_code)]
    pub fn from_number(n: u8) -> Option<Self> {
        match n {
            1 => Some(BookmarkLevel::Level1),
            2 => Some(BookmarkLevel::Level2),
            3 => Some(BookmarkLevel::Level3),
            4 => Some(BookmarkLevel::Level4),
            5 => Some(BookmarkLevel::Level5),
            6 => Some(BookmarkLevel::Level6),
            _ => None,
        }
    }
}

/// PDF 书签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfBookmark {
    /// 书签标题
    pub title: String,
    /// 目标页码（从 0 开始）
    pub page_index: usize,
    /// 书签层级
    pub level: BookmarkLevel,
    /// Y 坐标（页面内的垂直位置）
    pub y_position: Option<f64>,
    /// 是否展开
    pub open: bool,
    /// 子书签
    pub children: Vec<PdfBookmark>,
}

impl PdfBookmark {
    /// 创建新的书签
    #[allow(dead_code)]
    pub fn new(title: String, page_index: usize) -> Self {
        Self {
            title,
            page_index,
            level: BookmarkLevel::Level1,
            y_position: None,
            open: true,
            children: Vec::new(),
        }
    }

    /// 设置层级
    #[allow(dead_code)]
    pub fn with_level(mut self, level: BookmarkLevel) -> Self {
        self.level = level;
        self
    }

    /// 设置 Y 坐标
    #[allow(dead_code)]
    pub fn with_y_position(mut self, y: f64) -> Self {
        self.y_position = Some(y);
        self
    }

    /// 设置是否展开
    #[allow(dead_code)]
    pub fn with_open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// 添加子书签
    #[allow(dead_code)]
    pub fn with_child(mut self, child: PdfBookmark) -> Self {
        self.children.push(child);
        self
    }

    /// 添加多个子书签
    #[allow(dead_code)]
    pub fn with_children(mut self, children: Vec<PdfBookmark>) -> Self {
        self.children = children;
        self
    }

    /// 获取所有书签（包括子书签）
    #[allow(dead_code)]
    pub fn flatten(&self) -> Vec<&PdfBookmark> {
        let mut result = vec![self];
        for child in &self.children {
            result.extend(child.flatten());
        }
        result
    }

    /// 获取书签总数（包括子书签）
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        1 + self.children.iter().map(|c| c.count()).sum::<usize>()
    }
}

/// PDF 目录（大纲）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfOutline {
    /// 根书签
    pub bookmarks: Vec<PdfBookmark>,
    /// 是否在文档开头显示目录
    pub show_in_document: bool,
    /// 目录标题
    pub toc_title: String,
}

impl PdfOutline {
    /// 创建新的目录
    pub fn new() -> Self {
        Self {
            bookmarks: Vec::new(),
            show_in_document: false,
            toc_title: "Table of Contents".to_string(),
        }
    }

    /// 添加书签
    #[allow(dead_code)]
    pub fn add_bookmark(mut self, bookmark: PdfBookmark) -> Self {
        self.bookmarks.push(bookmark);
        self
    }

    /// 添加多个书签
    #[allow(dead_code)]
    pub fn with_bookmarks(mut self, bookmarks: Vec<PdfBookmark>) -> Self {
        self.bookmarks = bookmarks;
        self
    }

    /// 设置是否在文档中显示目录
    #[allow(dead_code)]
    pub fn with_show_in_document(mut self, show: bool) -> Self {
        self.show_in_document = show;
        self
    }

    /// 设置目录标题
    #[allow(dead_code)]
    pub fn with_toc_title(mut self, title: String) -> Self {
        self.toc_title = title;
        self
    }

    /// 获取所有书签（扁平化）
    #[allow(dead_code)]
    pub fn flatten(&self) -> Vec<&PdfBookmark> {
        let mut result = Vec::new();
        for bookmark in &self.bookmarks {
            result.extend(bookmark.flatten());
        }
        result
    }

    /// 获取书签总数
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.bookmarks.iter().map(|b| b.count()).sum()
    }

    /// 按页码排序书签
    #[allow(dead_code)]
    pub fn sort_by_page(&mut self) {
        Self::sort_recursive(&mut self.bookmarks);
    }

    fn sort_recursive(bookmarks: &mut [PdfBookmark]) {
        bookmarks.sort_by(|a, b| a.page_index.cmp(&b.page_index));
        for bookmark in bookmarks {
            if !bookmark.children.is_empty() {
                Self::sort_recursive(&mut bookmark.children);
            }
        }
    }

    /// 从标题列表自动生成书签
    #[allow(dead_code)]
    pub fn from_headings(headings: Vec<(String, usize, BookmarkLevel)>) -> Self {
        let mut outline = Self::new();
        for (title, page, level) in headings {
            outline = outline.add_bookmark(PdfBookmark::new(title, page).with_level(level));
        }
        outline
    }
}

impl Default for PdfOutline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bookmark_level_as_number() {
        assert_eq!(BookmarkLevel::Level1.as_number(), 1);
        assert_eq!(BookmarkLevel::Level6.as_number(), 6);
    }

    #[test]
    fn test_bookmark_level_from_number() {
        assert_eq!(BookmarkLevel::from_number(1), Some(BookmarkLevel::Level1));
        assert_eq!(BookmarkLevel::from_number(6), Some(BookmarkLevel::Level6));
        assert_eq!(BookmarkLevel::from_number(7), None);
    }

    #[test]
    fn test_pdf_bookmark_new() {
        let bookmark = PdfBookmark::new("Chapter 1".to_string(), 0);
        assert_eq!(bookmark.title, "Chapter 1");
        assert_eq!(bookmark.page_index, 0);
        assert_eq!(bookmark.level, BookmarkLevel::Level1);
    }

    #[test]
    fn test_pdf_bookmark_with_level() {
        let bookmark = PdfBookmark::new("Section".to_string(), 1).with_level(BookmarkLevel::Level2);
        assert_eq!(bookmark.level, BookmarkLevel::Level2);
    }

    #[test]
    fn test_pdf_bookmark_with_y_position() {
        let bookmark = PdfBookmark::new("Section".to_string(), 0).with_y_position(100.0);
        assert_eq!(bookmark.y_position, Some(100.0));
    }

    #[test]
    fn test_pdf_bookmark_with_child() {
        let child = PdfBookmark::new("Subsection".to_string(), 1);
        let bookmark = PdfBookmark::new("Chapter".to_string(), 0).with_child(child);
        assert_eq!(bookmark.children.len(), 1);
    }

    #[test]
    fn test_pdf_bookmark_flatten() {
        let child = PdfBookmark::new("Subsection".to_string(), 1);
        let bookmark = PdfBookmark::new("Chapter".to_string(), 0).with_child(child);
        let flat = bookmark.flatten();
        assert_eq!(flat.len(), 2);
    }

    #[test]
    fn test_pdf_bookmark_count() {
        let child = PdfBookmark::new("Subsection".to_string(), 1);
        let bookmark = PdfBookmark::new("Chapter".to_string(), 0).with_child(child);
        assert_eq!(bookmark.count(), 2);
    }

    #[test]
    fn test_pdf_outline_new() {
        let outline = PdfOutline::new();
        assert!(outline.bookmarks.is_empty());
        assert!(!outline.show_in_document);
    }

    #[test]
    fn test_pdf_outline_add_bookmark() {
        let bookmark = PdfBookmark::new("Chapter 1".to_string(), 0);
        let outline = PdfOutline::new().add_bookmark(bookmark);
        assert_eq!(outline.bookmarks.len(), 1);
    }

    #[test]
    fn test_pdf_outline_with_show_in_document() {
        let outline = PdfOutline::new().with_show_in_document(true);
        assert!(outline.show_in_document);
    }

    #[test]
    fn test_pdf_outline_with_toc_title() {
        let outline = PdfOutline::new().with_toc_title("Contents".to_string());
        assert_eq!(outline.toc_title, "Contents");
    }

    #[test]
    fn test_pdf_outline_flatten() {
        let bookmark = PdfBookmark::new("Chapter".to_string(), 0);
        let outline = PdfOutline::new().add_bookmark(bookmark);
        let flat = outline.flatten();
        assert_eq!(flat.len(), 1);
    }

    #[test]
    fn test_pdf_outline_count() {
        let bookmark1 = PdfBookmark::new("Chapter 1".to_string(), 0);
        let bookmark2 = PdfBookmark::new("Chapter 2".to_string(), 1);
        let outline = PdfOutline::new()
            .add_bookmark(bookmark1)
            .add_bookmark(bookmark2);
        assert_eq!(outline.count(), 2);
    }

    #[test]
    fn test_pdf_outline_sort_by_page() {
        let bookmark1 = PdfBookmark::new("Chapter 2".to_string(), 1);
        let bookmark2 = PdfBookmark::new("Chapter 1".to_string(), 0);
        let mut outline = PdfOutline::new()
            .add_bookmark(bookmark1)
            .add_bookmark(bookmark2);
        outline.sort_by_page();
        assert_eq!(outline.bookmarks[0].page_index, 0);
        assert_eq!(outline.bookmarks[1].page_index, 1);
    }

    #[test]
    fn test_pdf_outline_from_headings() {
        let headings = vec![
            ("Chapter 1".to_string(), 0, BookmarkLevel::Level1),
            ("Section 1.1".to_string(), 1, BookmarkLevel::Level2),
        ];
        let outline = PdfOutline::from_headings(headings);
        assert_eq!(outline.bookmarks.len(), 2);
    }

    #[test]
    fn test_pdf_outline_default() {
        let outline = PdfOutline::default();
        assert!(outline.bookmarks.is_empty());
    }

    #[test]
    fn test_pdf_outline_serialization() {
        let outline = PdfOutline::new();
        let json = serde_json::to_string(&outline);
        assert!(json.is_ok());
    }
}
