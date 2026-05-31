use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typst::syntax::Source;

/// AST 节点类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AstNodeType {
    /// 表格
    Table,
    /// 表格行
    TableRow,
    /// 表格单元格
    TableCell,
    /// 图片
    Image,
    /// 公式
    Equation,
    /// 标题
    Heading,
    /// 段落
    Paragraph,
    /// 列表
    List,
    /// 列表项
    ListItem,
    /// 代码块
    CodeBlock,
    /// 引用
    Quote,
    /// 链接
    Link,
    /// 文本
    Text,
    /// 自定义函数调用
    FunctionCall,
    /// 其他
    Other,
}

/// 源码位置信息
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLocation {
    /// 起始字节偏移
    pub start: usize,
    /// 结束字节偏移
    pub end: usize,
    /// 起始行号
    pub start_line: usize,
    /// 结束行号
    pub end_line: usize,
    /// 起始列号
    pub start_column: usize,
    /// 结束列号
    pub end_column: usize,
}

impl SourceLocation {
    pub fn new(start: usize, end: usize, source: &Source) -> Self {
        let start_line = source.byte_to_line(start).unwrap_or(0);
        let end_line = source.byte_to_line(end).unwrap_or(0);
        let start_column = source.byte_to_column(start).unwrap_or(0);
        let end_column = source.byte_to_column(end).unwrap_or(0);

        Self {
            start,
            end,
            start_line,
            end_line,
            start_column,
            end_column,
        }
    }

    pub fn contains(&self, offset: usize) -> bool {
        offset >= self.start && offset <= self.end
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

/// 视觉位置信息（用于预览渲染）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisualLocation {
    /// 页面索引
    pub page: usize,
    /// X 坐标（点）
    pub x: f64,
    /// Y 坐标（点）
    pub y: f64,
    /// 宽度（点）
    pub width: f64,
    /// 高度（点）
    pub height: f64,
}

impl VisualLocation {
    pub fn new(page: usize, x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            page,
            x,
            y,
            width,
            height,
        }
    }

    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}

/// AST 节点映射条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstNodeMapping {
    /// 节点 ID
    pub id: String,
    /// 节点类型
    pub node_type: AstNodeType,
    /// 源码位置
    pub source_location: SourceLocation,
    /// 视觉位置
    pub visual_location: Option<VisualLocation>,
    /// 父节点 ID
    pub parent_id: Option<String>,
    /// 子节点 ID 列表
    pub child_ids: Vec<String>,
    /// 节点属性
    pub attributes: HashMap<String, String>,
}

impl AstNodeMapping {
    pub fn new(id: String, node_type: AstNodeType, source_location: SourceLocation) -> Self {
        Self {
            id,
            node_type,
            source_location,
            visual_location: None,
            parent_id: None,
            child_ids: Vec::new(),
            attributes: HashMap::new(),
        }
    }

    pub fn with_visual_location(mut self, visual_location: VisualLocation) -> Self {
        self.visual_location = Some(visual_location);
        self
    }

    pub fn with_parent(mut self, parent_id: String) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    pub fn with_child(mut self, child_id: String) -> Self {
        self.child_ids.push(child_id);
        self
    }

    pub fn with_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.insert(key, value);
        self
    }
}

/// AST 映射表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstMapping {
    /// 源码文件 ID
    pub source_id: String,
    /// 节点映射表
    pub nodes: HashMap<String, AstNodeMapping>,
    /// 版本号
    pub version: u64,
}

impl AstMapping {
    pub fn new(source_id: String) -> Self {
        Self {
            source_id,
            nodes: HashMap::new(),
            version: 0,
        }
    }

    /// 添加节点映射
    pub fn add_node(&mut self, node: AstNodeMapping) {
        self.nodes.insert(node.id.clone(), node);
        self.version += 1;
    }

    /// 根据源码位置查找节点
    pub fn find_by_source_offset(&self, offset: usize) -> Option<&AstNodeMapping> {
        self.nodes
            .values()
            .find(|node| node.source_location.contains(offset))
    }

    /// 根据视觉位置查找节点
    pub fn find_by_visual_position(&self, page: usize, x: f64, y: f64) -> Option<&AstNodeMapping> {
        self.nodes.values().find(|node| {
            if let Some(visual) = &node.visual_location {
                visual.page == page && visual.contains_point(x, y)
            } else {
                false
            }
        })
    }

    /// 根据节点 ID 获取节点
    pub fn get_node(&self, id: &str) -> Option<&AstNodeMapping> {
        self.nodes.get(id)
    }

    /// 获取节点的所有子节点
    pub fn get_children(&self, node_id: &str) -> Vec<&AstNodeMapping> {
        if let Some(node) = self.nodes.get(node_id) {
            node.child_ids
                .iter()
                .filter_map(|id| self.nodes.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// 获取节点的父节点
    pub fn get_parent(&self, node_id: &str) -> Option<&AstNodeMapping> {
        if let Some(node) = self.nodes.get(node_id) {
            if let Some(parent_id) = &node.parent_id {
                self.nodes.get(parent_id)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// 删除节点及其子节点
    pub fn remove_node(&mut self, node_id: &str) {
        if let Some(node) = self.nodes.remove(node_id) {
            // 递归删除子节点
            for child_id in &node.child_ids {
                self.remove_node(child_id);
            }
            // 从父节点的子节点列表中移除
            if let Some(parent_id) = &node.parent_id {
                if let Some(parent) = self.nodes.get_mut(parent_id) {
                    parent.child_ids.retain(|id| id != node_id);
                }
            }
            self.version += 1;
        }
    }

    /// 更新节点的源码位置
    pub fn update_source_location(&mut self, node_id: &str, new_location: SourceLocation) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.source_location = new_location;
            self.version += 1;
        }
    }

    /// 更新节点的视觉位置
    pub fn update_visual_location(&mut self, node_id: &str, new_location: VisualLocation) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.visual_location = Some(new_location);
            self.version += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_location_creation() {
        let source = Source::new(
            typst::syntax::FileId::new(None, typst::syntax::VirtualPath::new("test.typ")),
            "Hello\nWorld".to_string(),
        );
        let location = SourceLocation::new(0, 5, &source);
        assert_eq!(location.start, 0);
        assert_eq!(location.end, 5);
        assert_eq!(location.start_line, 0);
        assert_eq!(location.end_line, 0);
    }

    #[test]
    fn test_source_location_contains() {
        let source = Source::new(
            typst::syntax::FileId::new(None, typst::syntax::VirtualPath::new("test.typ")),
            "Hello World".to_string(),
        );
        let location = SourceLocation::new(0, 5, &source);
        assert!(location.contains(3));
        assert!(!location.contains(6));
    }

    #[test]
    fn test_visual_location_contains_point() {
        let location = VisualLocation::new(0, 10.0, 20.0, 100.0, 50.0);
        assert!(location.contains_point(50.0, 40.0));
        assert!(!location.contains_point(5.0, 40.0));
        assert!(!location.contains_point(50.0, 80.0));
    }

    #[test]
    fn test_ast_mapping_add_node() {
        let mut mapping = AstMapping::new("test".to_string());
        let node = AstNodeMapping::new(
            "node1".to_string(),
            AstNodeType::Table,
            SourceLocation::new(
                0,
                10,
                &Source::new(
                    typst::syntax::FileId::new(None, typst::syntax::VirtualPath::new("test.typ")),
                    "test content".to_string(),
                ),
            ),
        );
        mapping.add_node(node);
        assert_eq!(mapping.nodes.len(), 1);
        assert_eq!(mapping.version, 1);
    }

    #[test]
    fn test_ast_mapping_find_by_source_offset() {
        let mut mapping = AstMapping::new("test".to_string());
        let source = Source::new(
            typst::syntax::FileId::new(None, typst::syntax::VirtualPath::new("test.typ")),
            "test content".to_string(),
        );
        let node = AstNodeMapping::new(
            "node1".to_string(),
            AstNodeType::Table,
            SourceLocation::new(0, 10, &source),
        );
        mapping.add_node(node);
        let found = mapping.find_by_source_offset(5);
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "node1");
    }

    #[test]
    fn test_ast_mapping_find_by_visual_position() {
        let mut mapping = AstMapping::new("test".to_string());
        let source = Source::new(
            typst::syntax::FileId::new(None, typst::syntax::VirtualPath::new("test.typ")),
            "test content".to_string(),
        );
        let node = AstNodeMapping::new(
            "node1".to_string(),
            AstNodeType::Table,
            SourceLocation::new(0, 10, &source),
        )
        .with_visual_location(VisualLocation::new(0, 10.0, 20.0, 100.0, 50.0));
        mapping.add_node(node);
        let found = mapping.find_by_visual_position(0, 50.0, 40.0);
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "node1");
    }

    #[test]
    fn test_ast_mapping_parent_child() {
        let mut mapping = AstMapping::new("test".to_string());
        let source = Source::new(
            typst::syntax::FileId::new(None, typst::syntax::VirtualPath::new("test.typ")),
            "test content".to_string(),
        );
        let parent = AstNodeMapping::new(
            "parent".to_string(),
            AstNodeType::Table,
            SourceLocation::new(0, 10, &source),
        )
        .with_child("child".to_string());
        let child = AstNodeMapping::new(
            "child".to_string(),
            AstNodeType::TableRow,
            SourceLocation::new(10, 20, &source),
        )
        .with_parent("parent".to_string());
        mapping.add_node(parent);
        mapping.add_node(child.clone());

        let children = mapping.get_children("parent");
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].id, "child");

        let parent_node = mapping.get_parent("child");
        assert!(parent_node.is_some());
        assert_eq!(parent_node.unwrap().id, "parent");
    }

    #[test]
    fn test_ast_mapping_remove_node() {
        let mut mapping = AstMapping::new("test".to_string());
        let source = Source::new(
            typst::syntax::FileId::new(None, typst::syntax::VirtualPath::new("test.typ")),
            "test content".to_string(),
        );
        let parent = AstNodeMapping::new(
            "parent".to_string(),
            AstNodeType::Table,
            SourceLocation::new(0, 10, &source),
        )
        .with_child("child".to_string());
        let child = AstNodeMapping::new(
            "child".to_string(),
            AstNodeType::TableRow,
            SourceLocation::new(10, 20, &source),
        )
        .with_parent("parent".to_string());
        mapping.add_node(parent);
        mapping.add_node(child);

        mapping.remove_node("parent");
        assert_eq!(mapping.nodes.len(), 0);
    }
}
