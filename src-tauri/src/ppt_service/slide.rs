use super::config::SlideLayout;
use serde::{Deserialize, Serialize};

/// 幻灯片
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slide {
    /// 幻灯片 ID
    pub id: String,
    /// 幻灯片标题
    pub title: String,
    /// 幻灯片布局
    pub layout: SlideLayout,
    /// 幻灯片索引
    pub index: usize,
    /// 是否隐藏
    pub hidden: bool,
    /// 是否过渡幻灯片
    pub transition: bool,
    /// 备注文本
    pub notes: String,
}

impl Slide {
    /// 创建新的幻灯片
    pub fn new(id: String, title: String, layout: SlideLayout) -> Self {
        Self {
            id,
            title,
            layout,
            index: 0,
            hidden: false,
            transition: false,
            notes: String::new(),
        }
    }

    /// 设置索引
    pub fn with_index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    /// 设置是否隐藏
    pub fn with_hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// 设置是否过渡
    pub fn with_transition(mut self, transition: bool) -> Self {
        self.transition = transition;
        self
    }

    /// 设置备注
    pub fn with_notes(mut self, notes: String) -> Self {
        self.notes = notes;
        self
    }

    /// 创建标题幻灯片
    pub fn title_slide(id: String, title: String) -> Self {
        Self::new(id, title, SlideLayout::Title)
    }

    /// 创建内容幻灯片
    pub fn content_slide(id: String, title: String) -> Self {
        Self::new(id, title, SlideLayout::TitleAndContent)
    }

    /// 创建空白幻灯片
    pub fn blank_slide(id: String) -> Self {
        Self::new(id, String::new(), SlideLayout::Blank)
    }
}

/// 幻灯片母版
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlideMaster {
    /// 母版 ID
    pub id: String,
    /// 母版名称
    pub name: String,
    /// 母版布局
    pub layouts: Vec<SlideLayout>,
}

impl SlideMaster {
    /// 创建新的母版
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            layouts: vec![
                SlideLayout::Title,
                SlideLayout::TitleAndContent,
                SlideLayout::TwoContent,
                SlideLayout::Blank,
            ],
        }
    }

    /// 添加布局
    pub fn with_layout(mut self, layout: SlideLayout) -> Self {
        self.layouts.push(layout);
        self
    }

    /// 设置布局列表
    pub fn with_layouts(mut self, layouts: Vec<SlideLayout>) -> Self {
        self.layouts = layouts;
        self
    }
}

/// 幻灯片布局类型（用于母版）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SlideLayoutType {
    /// 标题
    Title,
    /// 内容
    Content,
    /// 两栏
    TwoColumn,
    /// 对比
    Comparison,
    /// 自定义
    Custom(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_new() {
        let slide = Slide::new("1".to_string(), "Test".to_string(), SlideLayout::Title);
        assert_eq!(slide.id, "1");
        assert_eq!(slide.title, "Test");
        assert_eq!(slide.layout, SlideLayout::Title);
    }

    #[test]
    fn test_slide_with_index() {
        let slide =
            Slide::new("1".to_string(), "Test".to_string(), SlideLayout::Title).with_index(5);
        assert_eq!(slide.index, 5);
    }

    #[test]
    fn test_slide_with_hidden() {
        let slide =
            Slide::new("1".to_string(), "Test".to_string(), SlideLayout::Title).with_hidden(true);
        assert!(slide.hidden);
    }

    #[test]
    fn test_slide_title_slide() {
        let slide = Slide::title_slide("1".to_string(), "Presentation".to_string());
        assert_eq!(slide.layout, SlideLayout::Title);
    }

    #[test]
    fn test_slide_content_slide() {
        let slide = Slide::content_slide("1".to_string(), "Content".to_string());
        assert_eq!(slide.layout, SlideLayout::TitleAndContent);
    }

    #[test]
    fn test_slide_blank_slide() {
        let slide = Slide::blank_slide("1".to_string());
        assert_eq!(slide.layout, SlideLayout::Blank);
        assert!(slide.title.is_empty());
    }

    #[test]
    fn test_slide_chaining() {
        let slide = Slide::new("1".to_string(), "Test".to_string(), SlideLayout::Title)
            .with_index(1)
            .with_hidden(false)
            .with_notes("Speaker notes".to_string());
        assert_eq!(slide.index, 1);
        assert_eq!(slide.notes, "Speaker notes");
    }

    #[test]
    fn test_slide_master_new() {
        let master = SlideMaster::new("master1".to_string(), "Default".to_string());
        assert_eq!(master.id, "master1");
        assert_eq!(master.name, "Default");
        assert!(!master.layouts.is_empty());
    }

    #[test]
    fn test_slide_master_with_layout() {
        let master = SlideMaster::new("master1".to_string(), "Default".to_string())
            .with_layout(SlideLayout::SectionHeader);
        assert!(master.layouts.contains(&SlideLayout::SectionHeader));
    }

    #[test]
    fn test_slide_serialization() {
        let slide = Slide::new("1".to_string(), "Test".to_string(), SlideLayout::Title);
        let json = serde_json::to_string(&slide);
        assert!(json.is_ok());
    }
}
