/*!
 * 航空航天级列表系统
 * 实现 Typst 的列表功能（有序列表、无序列表、嵌套列表）
 */

use serde::{Deserialize, Serialize};

/// 列表项类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ListType {
    /// 无序列表（项目符号）
    Unordered,
    /// 有序列表（数字）
    Ordered,
    /// 字母列表（a, b, c）
    Letter,
    /// 罗马数字列表（i, ii, iii）
    Roman,
}

/// 列表项标记样式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ListMarker {
    /// 圆点
    Bullet,
    /// 方块
    Square,
    /// 圆圈
    Circle,
    /// 数字
    Number,
    /// 字母
    Letter,
    /// 罗马数字
    Roman,
}

/// 列表项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    pub content: String,
    pub children: Vec<ListItem>,
    pub depth: usize,
}

impl ListItem {
    pub fn new(content: String) -> Self {
        Self {
            content,
            children: Vec::new(),
            depth: 0,
        }
    }

    pub fn with_children(content: String, children: Vec<ListItem>) -> Self {
        Self {
            content,
            children,
            depth: 0,
        }
    }

    pub fn add_child(&mut self, child: ListItem) {
        self.children.push(child);
    }
}

/// 列表配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListConfig {
    pub list_type: ListType,
    pub marker: ListMarker,
    pub start_number: usize,
    pub indent: f64,
    pub spacing: f64,
}

impl Default for ListConfig {
    fn default() -> Self {
        Self {
            list_type: ListType::Unordered,
            marker: ListMarker::Bullet,
            start_number: 1,
            indent: 1.5,
            spacing: 0.5,
        }
    }
}

/// 列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub items: Vec<ListItem>,
    pub config: ListConfig,
}

impl List {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            config: ListConfig::default(),
        }
    }

    pub fn with_config(config: ListConfig) -> Self {
        Self {
            items: Vec::new(),
            config,
        }
    }

    pub fn add_item(&mut self, item: ListItem) {
        self.items.push(item);
    }

    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        for (index, item) in self.items.iter().enumerate() {
            let marker = self.get_marker(index + self.config.start_number);
            typst.push_str(&format!("{} {}\n", marker, item.content));

            // 处理嵌套列表
            for child in &item.children {
                typst.push_str(&self.format_nested_item(child, 1));
            }
        }

        typst
    }

    fn format_nested_item(&self, item: &ListItem, depth: usize) -> String {
        let indent = "  ".repeat(depth);
        let mut output = String::new();

        if self.config.list_type == ListType::Unordered {
            output.push_str(&format!("{}- {}\n", indent, item.content));
        } else {
            output.push_str(&format!("{}1. {}\n", indent, item.content));
        }

        for child in &item.children {
            output.push_str(&self.format_nested_item(child, depth + 1));
        }

        output
    }

    fn get_marker(&self, index: usize) -> String {
        match self.config.marker {
            ListMarker::Bullet => "-".to_string(),
            ListMarker::Square => "■".to_string(),
            ListMarker::Circle => "○".to_string(),
            ListMarker::Number => format!("{}.", index),
            ListMarker::Letter => {
                let letter = char::from_u32(((index - 1) % 26) as u32 + b'a' as u32).unwrap();
                format!("{}.", letter)
            }
            ListMarker::Roman => {
                format!("{}.", self.to_roman(index))
            }
        }
    }

    fn to_roman(&self, num: usize) -> String {
        let values = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut result = String::new();
        let mut n = num;

        for (value, symbol) in values.iter() {
            while n >= *value {
                result.push_str(symbol);
                n -= value;
            }
        }

        result
    }

    pub fn to_html(&self) -> String {
        let tag = if self.config.list_type == ListType::Unordered {
            "ul"
        } else {
            "ol"
        };

        let mut html = format!("<{}>\n", tag);

        for item in self.items.iter() {
            html.push_str("  <li>\n");
            html.push_str(&format!("    {}\n", html_escape(&item.content)));

            // 处理嵌套列表
            if !item.children.is_empty() {
                html.push_str("    <ul>\n");
                for child in &item.children {
                    html.push_str("      <li>\n");
                    html.push_str(&format!("        {}\n", html_escape(&child.content)));
                    html.push_str("      </li>\n");
                }
                html.push_str("    </ul>\n");
            }

            html.push_str("  </li>\n");
        }

        html.push_str(&format!("</{}>\n", tag));
        html
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

/// 列表构建器
pub struct ListBuilder {
    list: List,
}

impl ListBuilder {
    pub fn new() -> Self {
        Self { list: List::new() }
    }

    pub fn with_type(mut self, list_type: ListType) -> Self {
        self.list.config.list_type = list_type;
        self
    }

    pub fn with_marker(mut self, marker: ListMarker) -> Self {
        self.list.config.marker = marker;
        self
    }

    pub fn with_start_number(mut self, start: usize) -> Self {
        self.list.config.start_number = start;
        self
    }

    pub fn with_indent(mut self, indent: f64) -> Self {
        self.list.config.indent = indent;
        self
    }

    pub fn with_spacing(mut self, spacing: f64) -> Self {
        self.list.config.spacing = spacing;
        self
    }

    pub fn add_item(mut self, content: String) -> Self {
        self.list.add_item(ListItem::new(content));
        self
    }

    pub fn add_nested_item(mut self, parent_index: usize, content: String) -> Self {
        if parent_index < self.list.items.len() {
            self.list.items[parent_index].add_child(ListItem::new(content));
        }
        self
    }

    pub fn build(self) -> List {
        self.list
    }
}

impl Default for ListBuilder {
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
    fn test_list_creation() {
        let list = List::new();
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_list_default() {
        let list = List::default();
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_list_config_default() {
        let config = ListConfig::default();
        assert_eq!(config.list_type, ListType::Unordered);
        assert_eq!(config.marker, ListMarker::Bullet);
        assert_eq!(config.start_number, 1);
    }

    #[test]
    fn test_list_item_creation() {
        let item = ListItem::new("Test item".to_string());
        assert_eq!(item.content, "Test item");
        assert!(item.children.is_empty());
    }

    #[test]
    fn test_list_item_with_children() {
        let child = ListItem::new("Child item".to_string());
        let item = ListItem::with_children("Parent item".to_string(), vec![child]);
        assert_eq!(item.children.len(), 1);
    }

    #[test]
    fn test_list_add_item() {
        let mut list = List::new();
        list.add_item(ListItem::new("Item 1".to_string()));
        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn test_list_add_child() {
        let mut item = ListItem::new("Parent".to_string());
        item.add_child(ListItem::new("Child".to_string()));
        assert_eq!(item.children.len(), 1);
    }

    #[test]
    fn test_to_typst_unordered() {
        let mut list = List::new();
        list.add_item(ListItem::new("Item 1".to_string()));
        list.add_item(ListItem::new("Item 2".to_string()));
        let typst = list.to_typst();
        assert!(typst.contains("- Item 1"));
        assert!(typst.contains("- Item 2"));
    }

    #[test]
    fn test_to_typst_ordered() {
        let mut list = List::with_config(ListConfig {
            list_type: ListType::Ordered,
            marker: ListMarker::Number,
            start_number: 1,
            indent: 1.5,
            spacing: 0.5,
        });
        list.add_item(ListItem::new("Item 1".to_string()));
        list.add_item(ListItem::new("Item 2".to_string()));
        let typst = list.to_typst();
        assert!(typst.contains("1. Item 1"));
        assert!(typst.contains("2. Item 2"));
    }

    #[test]
    fn test_to_typst_nested() {
        let mut parent = ListItem::new("Parent".to_string());
        parent.add_child(ListItem::new("Child".to_string()));
        let mut list = List::new();
        list.add_item(parent);
        let typst = list.to_typst();
        assert!(typst.contains("- Parent"));
        assert!(typst.contains("  - Child"));
    }

    #[test]
    fn test_to_html_unordered() {
        let mut list = List::new();
        list.add_item(ListItem::new("Item 1".to_string()));
        let html = list.to_html();
        assert!(html.contains("<ul>"));
        assert!(html.contains("<li>"));
        assert!(html.contains("Item 1"));
    }

    #[test]
    fn test_to_html_ordered() {
        let mut list = List::with_config(ListConfig {
            list_type: ListType::Ordered,
            marker: ListMarker::Number,
            start_number: 1,
            indent: 1.5,
            spacing: 0.5,
        });
        list.add_item(ListItem::new("Item 1".to_string()));
        let html = list.to_html();
        assert!(html.contains("<ol>"));
        assert!(html.contains("<li>"));
    }

    #[test]
    fn test_to_html_nested() {
        let mut parent = ListItem::new("Parent".to_string());
        parent.add_child(ListItem::new("Child".to_string()));
        let mut list = List::new();
        list.add_item(parent);
        let html = list.to_html();
        assert!(html.contains("<ul>"));
        assert!(html.contains("<li>"));
        assert!(html.contains("Parent"));
        assert!(html.contains("Child"));
    }

    #[test]
    fn test_get_marker_bullet() {
        let list = List::new();
        assert_eq!(list.get_marker(1), "-");
    }

    #[test]
    fn test_get_marker_number() {
        let list = List::with_config(ListConfig {
            list_type: ListType::Ordered,
            marker: ListMarker::Number,
            start_number: 1,
            indent: 1.5,
            spacing: 0.5,
        });
        assert_eq!(list.get_marker(1), "1.");
        assert_eq!(list.get_marker(2), "2.");
    }

    #[test]
    fn test_get_marker_letter() {
        let list = List::with_config(ListConfig {
            list_type: ListType::Letter,
            marker: ListMarker::Letter,
            start_number: 1,
            indent: 1.5,
            spacing: 0.5,
        });
        assert_eq!(list.get_marker(1), "a.");
        assert_eq!(list.get_marker(2), "b.");
    }

    #[test]
    fn test_get_marker_roman() {
        let list = List::with_config(ListConfig {
            list_type: ListType::Roman,
            marker: ListMarker::Roman,
            start_number: 1,
            indent: 1.5,
            spacing: 0.5,
        });
        assert_eq!(list.get_marker(1), "I.");
        assert_eq!(list.get_marker(2), "II.");
        assert_eq!(list.get_marker(3), "III.");
    }

    #[test]
    fn test_to_roman() {
        let list = List::new();
        assert_eq!(list.to_roman(1), "I");
        assert_eq!(list.to_roman(2), "II");
        assert_eq!(list.to_roman(3), "III");
        assert_eq!(list.to_roman(4), "IV");
        assert_eq!(list.to_roman(5), "V");
        assert_eq!(list.to_roman(10), "X");
    }

    #[test]
    fn test_list_builder() {
        let list = ListBuilder::new()
            .with_type(ListType::Ordered)
            .with_marker(ListMarker::Number)
            .add_item("Item 1".to_string())
            .add_item("Item 2".to_string())
            .build();

        assert_eq!(list.items.len(), 2);
        assert_eq!(list.config.list_type, ListType::Ordered);
    }

    #[test]
    fn test_list_builder_default() {
        let builder = ListBuilder::default();
        let list = builder.build();
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_list_type_partial_eq() {
        assert_eq!(ListType::Unordered, ListType::Unordered);
        assert_ne!(ListType::Unordered, ListType::Ordered);
    }

    #[test]
    fn test_list_marker_partial_eq() {
        assert_eq!(ListMarker::Bullet, ListMarker::Bullet);
        assert_ne!(ListMarker::Bullet, ListMarker::Square);
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_list_with_config() {
        let config = ListConfig {
            list_type: ListType::Ordered,
            marker: ListMarker::Letter,
            start_number: 5,
            indent: 2.0,
            spacing: 1.0,
        };
        let list = List::with_config(config);
        assert_eq!(list.config.start_number, 5);
        assert_eq!(list.config.indent, 2.0);
    }

    #[test]
    fn test_nested_list_depth() {
        let mut parent = ListItem::new("Parent".to_string());
        parent.add_child(ListItem::new("Child".to_string()));
        assert_eq!(parent.depth, 0);
        assert_eq!(parent.children[0].depth, 0);
    }

    #[test]
    fn test_letter_marker_wrap() {
        let list = List::with_config(ListConfig {
            list_type: ListType::Letter,
            marker: ListMarker::Letter,
            start_number: 1,
            indent: 1.5,
            spacing: 0.5,
        });
        assert_eq!(list.get_marker(26), "z.");
        assert_eq!(list.get_marker(27), "a."); // wraps around
    }

    #[test]
    fn test_roman_large_number() {
        let list = List::new();
        assert_eq!(list.to_roman(50), "L");
        assert_eq!(list.to_roman(100), "C");
        assert_eq!(list.to_roman(500), "D");
        assert_eq!(list.to_roman(1000), "M");
    }
}
