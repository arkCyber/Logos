/*!
 * 航空航天级主控页面系统
 * 实现模板页面、页面样式继承、主控页面管理
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 页面样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageStyle {
    pub background_color: Option<String>,
    pub margin_top: f64,
    pub margin_bottom: f64,
    pub margin_left: f64,
    pub margin_right: f64,
    pub header_height: f64,
    pub footer_height: f64,
}

impl Default for PageStyle {
    fn default() -> Self {
        Self {
            background_color: None,
            margin_top: 72.0,
            margin_bottom: 72.0,
            margin_left: 72.0,
            margin_right: 72.0,
            header_height: 48.0,
            footer_height: 48.0,
        }
    }
}

/// 页面元素类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PageElementType {
    /// 文本
    Text,
    /// 图像
    Image,
    /// 形状
    Shape,
    /// 页码
    PageNumber,
    /// 日期
    Date,
    /// 自定义
    Custom,
}

/// 页面元素
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageElement {
    pub element_type: PageElementType,
    pub content: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub style: HashMap<String, String>,
}

impl PageElement {
    pub fn new(element_type: PageElementType, content: String) -> Self {
        Self {
            element_type,
            content,
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            style: HashMap::new(),
        }
    }

    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_style(mut self, key: String, value: String) -> Self {
        self.style.insert(key, value);
        self
    }
}

/// 主控页面
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterPage {
    pub name: String,
    pub description: String,
    pub page_style: PageStyle,
    pub header_elements: Vec<PageElement>,
    pub footer_elements: Vec<PageElement>,
    pub background_elements: Vec<PageElement>,
}

impl MasterPage {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: String::new(),
            page_style: PageStyle::default(),
            header_elements: Vec::new(),
            footer_elements: Vec::new(),
            background_elements: Vec::new(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_page_style(mut self, style: PageStyle) -> Self {
        self.page_style = style;
        self
    }

    pub fn add_header_element(mut self, element: PageElement) -> Self {
        self.header_elements.push(element);
        self
    }

    pub fn add_footer_element(mut self, element: PageElement) -> Self {
        self.footer_elements.push(element);
        self
    }

    pub fn add_background_element(mut self, element: PageElement) -> Self {
        self.background_elements.push(element);
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();
        
        // 页面设置
        typst.push_str("#set page(");
        typst.push_str(&format!(
            "margin: (left: {}pt, right: {}pt, top: {}pt, bottom: {}pt), ",
            self.page_style.margin_left,
            self.page_style.margin_right,
            self.page_style.margin_top,
            self.page_style.margin_bottom
        ));
        
        if let Some(ref bg) = self.page_style.background_color {
            typst.push_str(&format!("fill: {}, ", bg));
        }
        
        typst.push_str(")\n");
        
        // 背景元素
        for element in &self.background_elements {
            typst.push_str(&self.element_to_typst(element));
        }
        
        // 页眉
        if !self.header_elements.is_empty() {
            typst.push_str("#set page(header: [\n");
            for element in &self.header_elements {
                typst.push_str(&self.element_to_typst(element));
            }
            typst.push_str("])\n");
        }
        
        // 页脚
        if !self.footer_elements.is_empty() {
            typst.push_str("#set page(footer: [\n");
            for element in &self.footer_elements {
                typst.push_str(&self.element_to_typst(element));
            }
            typst.push_str("])\n");
        }
        
        typst
    }

    fn element_to_typst(&self, element: &PageElement) -> String {
        match element.element_type {
            PageElementType::Text => {
                format!(
                    "#place(horizon + top, dx: {}pt, dy: {}pt)[{}]\n",
                    element.x, element.y, element.content
                )
            }
            PageElementType::PageNumber => {
                format!(
                    "#place(horizon + top, dx: {}pt, dy: {}pt)[#counter(page).display()]\n",
                    element.x, element.y
                )
            }
            PageElementType::Date => {
                format!(
                    "#place(horizon + top, dx: {}pt, dy: {}pt)[#datetime.today().display()]\n",
                    element.x, element.y
                )
            }
            _ => {
                format!(
                    "#place(horizon + top, dx: {}pt, dy: {}pt)[{}]\n",
                    element.x, element.y, element.content
                )
            }
        }
    }
}

/// 页面实例
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInstance {
    pub master_page_name: String,
    pub page_number: usize,
    pub custom_elements: Vec<PageElement>,
    pub style_overrides: HashMap<String, String>,
}

impl PageInstance {
    pub fn new(master_page_name: String, page_number: usize) -> Self {
        Self {
            master_page_name,
            page_number,
            custom_elements: Vec::new(),
            style_overrides: HashMap::new(),
        }
    }

    pub fn add_custom_element(mut self, element: PageElement) -> Self {
        self.custom_elements.push(element);
        self
    }

    pub fn with_style_override(mut self, key: String, value: String) -> Self {
        self.style_overrides.insert(key, value);
        self
    }
}

/// 主控页面系统
pub struct MasterPageSystem {
    master_pages: HashMap<String, MasterPage>,
    page_instances: Vec<PageInstance>,
}

impl MasterPageSystem {
    pub fn new() -> Self {
        Self {
            master_pages: HashMap::new(),
            page_instances: Vec::new(),
        }
    }

    /// 注册主控页面
    pub fn register_master_page(&mut self, master_page: MasterPage) -> Result<(), String> {
        if self.master_pages.contains_key(&master_page.name) {
            return Err(format!("Master page '{}' already exists", master_page.name));
        }
        self.master_pages.insert(master_page.name.clone(), master_page);
        Ok(())
    }

    /// 获取主控页面
    pub fn get_master_page(&self, name: &str) -> Option<&MasterPage> {
        self.master_pages.get(name)
    }

    /// 创建页面实例
    pub fn create_page_instance(&mut self, master_page_name: String, page_number: usize) -> Result<PageInstance, String> {
        if !self.master_pages.contains_key(&master_page_name) {
            return Err(format!("Master page '{}' not found", master_page_name));
        }
        
        let instance = PageInstance::new(master_page_name, page_number);
        self.page_instances.push(instance.clone());
        Ok(instance)
    }

    /// 获取所有主控页面名称
    pub fn get_master_page_names(&self) -> Vec<String> {
        self.master_pages.keys().cloned().collect()
    }

    /// 生成完整的文档 Typst 代码
    pub fn generate_document_typst(&self) -> String {
        let mut typst = String::new();
        
        // 首先定义所有主控页面
        for (name, master_page) in &self.master_pages {
            typst.push_str(&format!("// Master page: {}\n", name));
            typst.push_str(&master_page.to_typst());
            typst.push_str("\n");
        }
        
        // 然后生成页面实例
        for instance in &self.page_instances {
            if let Some(master_page) = self.get_master_page(&instance.master_page_name) {
                typst.push_str(&format!("// Page {} using {}\n", instance.page_number, instance.master_page_name));
                
                // 应用样式覆盖
                for (key, value) in &instance.style_overrides {
                    typst.push_str(&format!("#set page({}: {})\n", key, value));
                }
                
                // 添加自定义元素
                for element in &instance.custom_elements {
                    typst.push_str(&master_page.element_to_typst(element));
                }
                
                typst.push_str("\n");
            }
        }
        
        typst
    }

    /// 获取默认主控页面
    pub fn get_default_master_page() -> MasterPage {
        let mut master = MasterPage::new("Default".to_string());
        master.description = "Default master page with header and footer".to_string();
        
        // 添加页码到页脚
        let page_number = PageElement::new(PageElementType::PageNumber, String::new())
            .with_position(0.0, 0.0);
        master = master.add_footer_element(page_number);
        
        master
    }
}

impl Default for MasterPageSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_style_default() {
        let style = PageStyle::default();
        assert_eq!(style.margin_top, 72.0);
    }

    #[test]
    fn test_page_element_creation() {
        let element = PageElement::new(PageElementType::Text, "Test".to_string());
        assert_eq!(element.content, "Test");
    }

    #[test]
    fn test_page_element_with_position() {
        let element = PageElement::new(PageElementType::Text, "Test".to_string())
            .with_position(10.0, 20.0);
        assert_eq!(element.x, 10.0);
        assert_eq!(element.y, 20.0);
    }

    #[test]
    fn test_page_element_with_size() {
        let element = PageElement::new(PageElementType::Text, "Test".to_string())
            .with_size(100.0, 200.0);
        assert_eq!(element.width, 100.0);
        assert_eq!(element.height, 200.0);
    }

    #[test]
    fn test_master_page_creation() {
        let master = MasterPage::new("Test".to_string());
        assert_eq!(master.name, "Test");
    }

    #[test]
    fn test_master_page_with_description() {
        let master = MasterPage::new("Test".to_string())
            .with_description("Test page".to_string());
        assert_eq!(master.description, "Test page");
    }

    #[test]
    fn test_master_page_add_header_element() {
        let element = PageElement::new(PageElementType::Text, "Header".to_string());
        let master = MasterPage::new("Test".to_string())
            .add_header_element(element);
        assert_eq!(master.header_elements.len(), 1);
    }

    #[test]
    fn test_master_page_add_footer_element() {
        let element = PageElement::new(PageElementType::Text, "Footer".to_string());
        let master = MasterPage::new("Test".to_string())
            .add_footer_element(element);
        assert_eq!(master.footer_elements.len(), 1);
    }

    #[test]
    fn test_master_page_to_typst() {
        let master = MasterPage::new("Test".to_string());
        let typst = master.to_typst();
        assert!(typst.contains("#set page("));
    }

    #[test]
    fn test_page_instance_creation() {
        let instance = PageInstance::new("Default".to_string(), 1);
        assert_eq!(instance.page_number, 1);
    }

    #[test]
    fn test_page_instance_add_custom_element() {
        let element = PageElement::new(PageElementType::Text, "Custom".to_string());
        let instance = PageInstance::new("Default".to_string(), 1)
            .add_custom_element(element);
        assert_eq!(instance.custom_elements.len(), 1);
    }

    #[test]
    fn test_master_page_system_creation() {
        let system = MasterPageSystem::new();
        assert!(system.master_pages.is_empty());
    }

    #[test]
    fn test_register_master_page() {
        let mut system = MasterPageSystem::new();
        let master = MasterPage::new("Test".to_string());
        assert!(system.register_master_page(master).is_ok());
    }

    #[test]
    fn test_register_duplicate_master_page() {
        let mut system = MasterPageSystem::new();
        let master = MasterPage::new("Test".to_string());
        system.register_master_page(master.clone()).unwrap();
        assert!(system.register_master_page(master).is_err());
    }

    #[test]
    fn test_get_master_page() {
        let mut system = MasterPageSystem::new();
        let master = MasterPage::new("Test".to_string());
        system.register_master_page(master).unwrap();
        assert!(system.get_master_page("Test").is_some());
    }

    #[test]
    fn test_create_page_instance() {
        let mut system = MasterPageSystem::new();
        let master = MasterPage::new("Default".to_string());
        system.register_master_page(master).unwrap();
        
        let instance = system.create_page_instance("Default".to_string(), 1);
        assert!(instance.is_ok());
    }

    #[test]
    fn test_create_page_instance_nonexistent() {
        let mut system = MasterPageSystem::new();
        let instance = system.create_page_instance("Nonexistent".to_string(), 1);
        assert!(instance.is_err());
    }

    #[test]
    fn test_get_master_page_names() {
        let mut system = MasterPageSystem::new();
        let master = MasterPage::new("Test".to_string());
        system.register_master_page(master).unwrap();
        
        let names = system.get_master_page_names();
        assert_eq!(names.len(), 1);
    }

    #[test]
    fn test_get_default_master_page() {
        let master = MasterPageSystem::get_default_master_page();
        assert_eq!(master.name, "Default");
        assert!(!master.footer_elements.is_empty());
    }

    #[test]
    fn test_generate_document_typst() {
        let mut system = MasterPageSystem::new();
        let master = MasterPage::new("Default".to_string());
        system.register_master_page(master).unwrap();
        system.create_page_instance("Default".to_string(), 1).unwrap();
        
        let typst = system.generate_document_typst();
        assert!(typst.contains("#set page("));
    }
}
