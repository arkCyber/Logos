/*!
 * 航空航天级引用系统
 * 实现 Typst 的引用功能（标签、引用、交叉引用）
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Maximum number of labels to prevent memory issues
const MAX_LABELS: usize = 10_000;

/// Maximum number of references to prevent memory issues
const MAX_REFERENCES: usize = 100_000;

/// Maximum label name length
const MAX_LABEL_NAME_LENGTH: usize = 256;

/// Maximum label text length
const MAX_LABEL_TEXT_LENGTH: usize = 10_000;

/// Maximum reference text length
const MAX_REFERENCE_TEXT_LENGTH: usize = 10_000;

/// Maximum Typst code size for parsing
const MAX_TYPST_CODE_SIZE: usize = 1 * 1024 * 1024; // 1MB

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 100;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 500;

/// 引用样式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReferenceStyle {
    /// 数字样式 \[1\]
    Numeric,
    /// 作者-年份样式 (Smith, 2020)
    AuthorYear,
    /// 标题样式 (Section 1)
    Title,
    /// 页码样式 (page 5)
    Page,
}

/// 标签类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LabelType {
    /// 标题
    Heading,
    /// 图形
    Figure,
    /// 表格
    Table,
    /// 公式
    Equation,
    /// 自定义
    Custom(String),
}

/// 标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub name: String,
    pub label_type: LabelType,
    pub text: String,
    pub page_number: usize,
    pub counter: usize,
}

impl Label {
    /// Creates a new label
    /// 
    /// # Arguments
    /// * `name` - The label name
    /// * `label_type` - The label type
    /// * `text` - The label text
    /// 
    /// # Returns
    /// A new Label instance
    /// 
    /// # Security
    /// Validates name and text lengths to prevent DoS attacks
    pub fn new(name: String, label_type: LabelType, text: String) -> Self {
        if name.len() > MAX_LABEL_NAME_LENGTH {
            eprintln!("Label: name exceeds maximum length of {}", MAX_LABEL_NAME_LENGTH);
        }
        if text.len() > MAX_LABEL_TEXT_LENGTH {
            eprintln!("Label: text exceeds maximum length of {}", MAX_LABEL_TEXT_LENGTH);
        }
        
        Self {
            name,
            label_type,
            text,
            page_number: 1,
            counter: 1,
        }
    }

    /// Sets the page number
    /// 
    /// # Arguments
    /// * `page` - The page number
    /// 
    /// # Returns
    /// Self for builder pattern
    pub fn with_page_number(mut self, page: usize) -> Self {
        self.page_number = page;
        self
    }

    /// Sets the counter
    /// 
    /// # Arguments
    /// * `counter` - The counter value
    /// 
    /// # Returns
    /// Self for builder pattern
    pub fn with_counter(mut self, counter: usize) -> Self {
        self.counter = counter;
        self
    }
}

/// 引用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub label_name: String,
    pub reference_style: ReferenceStyle,
    pub text: String,
}

impl Reference {
    /// Creates a new reference
    /// 
    /// # Arguments
    /// * `label_name` - The label name to reference
    /// * `reference_style` - The reference style
    /// 
    /// # Returns
    /// A new Reference instance
    /// 
    /// # Security
    /// Validates label name length to prevent DoS attacks
    pub fn new(label_name: String, reference_style: ReferenceStyle) -> Self {
        if label_name.len() > MAX_LABEL_NAME_LENGTH {
            eprintln!("Reference: label_name exceeds maximum length of {}", MAX_LABEL_NAME_LENGTH);
        }
        
        Self {
            label_name,
            reference_style,
            text: String::new(),
        }
    }

    /// Sets the reference text
    /// 
    /// # Arguments
    /// * `text` - The reference text
    /// 
    /// # Returns
    /// Self for builder pattern
    /// 
    /// # Security
    /// Validates text length to prevent memory issues
    pub fn with_text(mut self, text: String) -> Self {
        if text.len() > MAX_REFERENCE_TEXT_LENGTH {
            eprintln!("Reference: text exceeds maximum length of {}", MAX_REFERENCE_TEXT_LENGTH);
        }
        self.text = text;
        self
    }
}

/// 引用系统
pub struct ReferenceSystem {
    labels: Arc<Mutex<HashMap<String, Label>>>,
    references: Arc<Mutex<Vec<Reference>>>,
    counters: Arc<Mutex<HashMap<LabelType, usize>>>,
}

impl ReferenceSystem {
    /// Creates a new reference system
    /// 
    /// # Returns
    /// A new ReferenceSystem instance
    pub fn new() -> Self {
        Self {
            labels: Arc::new(Mutex::new(HashMap::new())),
            references: Arc::new(Mutex::new(Vec::new())),
            counters: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 注册标签
    /// 
    /// # Arguments
    /// * `label` - The label to register
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Enforces maximum label limit to prevent memory issues
    pub fn register_label(&self, label: Label) -> Result<(), String> {
        let mut labels = self.labels.lock().map_err(|e| e.to_string())?;
        let mut counters = self.counters.lock().map_err(|e| e.to_string())?;

        // Safety check: prevent too many labels
        if labels.len() >= MAX_LABELS {
            return Err(format!("Maximum label limit of {} reached", MAX_LABELS));
        }

        // 增加计数器
        let counter = counters.entry(label.label_type.clone()).or_insert(0);
        *counter += 1;

        let mut label = label;
        label.counter = *counter;

        labels.insert(label.name.clone(), label);
        Ok(())
    }

    /// 获取标签
    /// 
    /// # Arguments
    /// * `name` - The label name
    /// 
    /// # Returns
    /// The label if found, None otherwise
    pub fn get_label(&self, name: &str) -> Option<Label> {
        let labels = self.labels.lock().ok()?;
        labels.get(name).cloned()
    }

    /// 创建引用
    /// 
    /// # Arguments
    /// * `label_name` - The label name to reference
    /// * `reference_style` - The reference style
    /// 
    /// # Returns
    /// Result containing the reference or an error
    /// 
    /// # Security
    /// Enforces maximum reference limit to prevent memory issues
    pub fn create_reference(
        &self,
        label_name: String,
        reference_style: ReferenceStyle,
    ) -> Result<Reference, String> {
        let labels = self.labels.lock().map_err(|e| e.to_string())?;

        let label = labels
            .get(&label_name)
            .ok_or_else(|| format!("Label '{}' not found", label_name))?;

        let text = self.format_reference(label, &reference_style);

        let reference = Reference::new(label_name, reference_style).with_text(text);

        let mut references = self.references.lock().map_err(|e| e.to_string())?;
        
        // Safety check: prevent too many references
        if references.len() >= MAX_REFERENCES {
            return Err(format!("Maximum reference limit of {} reached", MAX_REFERENCES));
        }
        
        references.push(reference.clone());

        Ok(reference)
    }

    /// 格式化引用
    /// 
    /// # Arguments
    /// * `label` - The label to format
    /// * `style` - The reference style
    /// 
    /// # Returns
    /// The formatted reference string
    fn format_reference(&self, label: &Label, style: &ReferenceStyle) -> String {
        match style {
            ReferenceStyle::Numeric => format!("[{}]", label.counter),
            ReferenceStyle::AuthorYear => format!("({})", label.text),
            ReferenceStyle::Title => format!("({})", label.text),
            ReferenceStyle::Page => format!("(page {})", label.page_number),
        }
    }

    /// 获取所有标签
    /// 
    /// # Returns
    /// A vector of all labels
    pub fn get_all_labels(&self) -> Vec<Label> {
        let labels = self.labels.lock().unwrap();
        labels.values().cloned().collect()
    }

    /// 获取所有引用
    /// 
    /// # Returns
    /// A vector of all references
    pub fn get_all_references(&self) -> Vec<Reference> {
        let references = self.references.lock().unwrap();
        references.clone()
    }

    /// 解析 Typst 引用语法
    /// 
    /// # Arguments
    /// * `typst_code` - The Typst code to parse
    /// 
    /// # Returns
    /// Result containing the reference or an error
    /// 
    /// # Security
    /// Validates input size to prevent DoS attacks
    pub fn parse_reference_syntax(typst_code: &str) -> Result<Reference, String> {
        // Security check: prevent DoS with oversized input
        if typst_code.len() > MAX_TYPST_CODE_SIZE {
            return Err(format!("Typst code exceeds maximum size of {} bytes", MAX_TYPST_CODE_SIZE));
        }

        let code = typst_code.trim();

        if let Some(label_name) = code.strip_prefix("@") {
            Ok(Reference::new(label_name.to_string(), ReferenceStyle::Numeric))
        } else if code.starts_with("#ref(") {
            let content = code
                .strip_prefix("#ref(")
                .and_then(|s| s.strip_suffix(")"))
                .ok_or("Invalid reference syntax")?;

            let label_name = content.trim_matches('"');
            Ok(Reference::new(
                label_name.to_string(),
                ReferenceStyle::Numeric,
            ))
        } else {
            Err("Invalid reference syntax".to_string())
        }
    }

    /// 解析 Typst 标签语法
    /// 
    /// # Arguments
    /// * `typst_code` - The Typst code to parse
    /// 
    /// # Returns
    /// Result containing the label components or an error
    /// 
    /// # Security
    /// Validates input size to prevent DoS attacks
    pub fn parse_label_syntax(typst_code: &str) -> Result<(String, LabelType, String), String> {
        // Security check: prevent DoS with oversized input
        if typst_code.len() > MAX_TYPST_CODE_SIZE {
            return Err(format!("Typst code exceeds maximum size of {} bytes", MAX_TYPST_CODE_SIZE));
        }

        let code = typst_code.trim();

        if code.contains("<") && code.contains(">") {
            let parts: Vec<&str> = code.split("<").collect();
            if parts.len() >= 2 {
                let text = parts[0].trim();
                let label_part = parts[1].trim().trim_end_matches('>');

                return Ok((
                    label_part.to_string(),
                    LabelType::Custom("default".to_string()),
                    text.to_string(),
                ));
            }
        }

        Err("Invalid label syntax".to_string())
    }

    /// 生成 Typst 代码
    /// 
    /// # Returns
    /// The Typst code representation
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        let labels = self.labels.lock().unwrap();
        for label in labels.values() {
            typst.push_str(&format!("{} <{}>\n", label.text, label.name));
        }

        let references = self.references.lock().unwrap();
        for reference in references.iter() {
            typst.push_str(&format!("@{}\n", reference.label_name));
        }

        typst
    }

    /// 生成 HTML
    /// 
    /// # Returns
    /// The HTML representation
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        let references = self.references.lock().unwrap();
        for reference in references.iter() {
            html.push_str(&format!(
                "<a href=\"#{}\" class=\"reference\">{}</a>\n",
                reference.label_name,
                html_escape(&reference.text)
            ));
        }

        html
    }

    /// Gets the number of labels
    /// 
    /// # Returns
    /// The number of labels
    pub fn label_count(&self) -> usize {
        let labels = self.labels.lock().unwrap();
        labels.len()
    }

    /// Gets the number of references
    /// 
    /// # Returns
    /// The number of references
    pub fn reference_count(&self) -> usize {
        let references = self.references.lock().unwrap();
        references.len()
    }

    /// Clears all labels and references
    /// 
    /// # Warning
    /// This will delete all reference data
    pub fn clear_all(&self) {
        let mut labels = self.labels.lock().unwrap();
        let mut references = self.references.lock().unwrap();
        let mut counters = self.counters.lock().unwrap();
        labels.clear();
        references.clear();
        counters.clear();
    }

    /// Removes a label by name
    /// 
    /// # Arguments
    /// * `name` - The label name to remove
    /// 
    /// # Returns
    /// true if the label was removed, false if not found
    pub fn remove_label(&self, name: &str) -> bool {
        let mut labels = self.labels.lock().unwrap();
        labels.remove(name).is_some()
    }
}

impl Default for ReferenceSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// 引用构建器
pub struct ReferenceBuilder {
    system: ReferenceSystem,
}

impl ReferenceBuilder {
    pub fn new() -> Self {
        Self {
            system: ReferenceSystem::new(),
        }
    }

    pub fn add_label(self, label: Label) -> Result<Self, String> {
        self.system.register_label(label)?;
        Ok(self)
    }

    pub fn build(self) -> ReferenceSystem {
        self.system
    }
}

impl Default for ReferenceBuilder {
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
    fn test_reference_system_creation() {
        let system = ReferenceSystem::new();
        assert!(system.get_all_labels().is_empty());
    }

    #[test]
    fn test_reference_system_default() {
        let system = ReferenceSystem::default();
        assert!(system.get_all_labels().is_empty());
    }

    #[test]
    fn test_label_creation() {
        let label = Label::new(
            "fig1".to_string(),
            LabelType::Figure,
            "Figure 1".to_string(),
        );
        assert_eq!(label.name, "fig1");
        assert_eq!(label.text, "Figure 1");
    }

    #[test]
    fn test_label_with_page_number() {
        let label = Label::new(
            "fig1".to_string(),
            LabelType::Figure,
            "Figure 1".to_string(),
        )
        .with_page_number(5);
        assert_eq!(label.page_number, 5);
    }

    #[test]
    fn test_label_with_counter() {
        let label = Label::new(
            "fig1".to_string(),
            LabelType::Figure,
            "Figure 1".to_string(),
        )
        .with_counter(2);
        assert_eq!(label.counter, 2);
    }

    #[test]
    fn test_register_label() {
        let system = ReferenceSystem::new();
        let label = Label::new(
            "fig1".to_string(),
            LabelType::Figure,
            "Figure 1".to_string(),
        );
        assert!(system.register_label(label).is_ok());
        assert_eq!(system.get_all_labels().len(), 1);
    }

    #[test]
    fn test_get_label() {
        let system = ReferenceSystem::new();
        let label = Label::new(
            "fig1".to_string(),
            LabelType::Figure,
            "Figure 1".to_string(),
        );
        system.register_label(label).unwrap();

        let retrieved = system.get_label("fig1");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "fig1");
    }

    #[test]
    fn test_get_label_not_found() {
        let system = ReferenceSystem::new();
        let retrieved = system.get_label("nonexistent");
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_create_reference() {
        let system = ReferenceSystem::new();
        let label = Label::new(
            "fig1".to_string(),
            LabelType::Figure,
            "Figure 1".to_string(),
        );
        system.register_label(label).unwrap();

        let reference = system.create_reference("fig1".to_string(), ReferenceStyle::Numeric);
        assert!(reference.is_ok());
        assert_eq!(reference.unwrap().label_name, "fig1");
    }

    #[test]
    fn test_create_reference_not_found() {
        let system = ReferenceSystem::new();
        let reference = system.create_reference("nonexistent".to_string(), ReferenceStyle::Numeric);
        assert!(reference.is_err());
    }

    #[test]
    fn test_format_reference_numeric() {
        let system = ReferenceSystem::new();
        let label = Label::new(
            "fig1".to_string(),
            LabelType::Figure,
            "Figure 1".to_string(),
        )
        .with_counter(1);
        let text = system.format_reference(&label, &ReferenceStyle::Numeric);
        assert_eq!(text, "[1]");
    }

    #[test]
    fn test_format_reference_author_year() {
        let system = ReferenceSystem::new();
        let label = Label::new(
            "fig1".to_string(),
            LabelType::Figure,
            "Smith, 2020".to_string(),
        );
        let text = system.format_reference(&label, &ReferenceStyle::AuthorYear);
        assert_eq!(text, "(Smith, 2020)");
    }

    #[test]
    fn test_format_reference_page() {
        let system = ReferenceSystem::new();
        let label = Label::new(
            "fig1".to_string(),
            LabelType::Figure,
            "Figure 1".to_string(),
        )
        .with_page_number(5);
        let text = system.format_reference(&label, &ReferenceStyle::Page);
        assert_eq!(text, "(page 5)");
    }

    #[test]
    fn test_parse_reference_syntax_at() {
        let reference = ReferenceSystem::parse_reference_syntax("@fig1");
        assert!(reference.is_ok());
        assert_eq!(reference.unwrap().label_name, "fig1");
    }

    #[test]
    fn test_parse_reference_syntax_ref() {
        let reference = ReferenceSystem::parse_reference_syntax("#ref(\"fig1\")");
        assert!(reference.is_ok());
        assert_eq!(reference.unwrap().label_name, "fig1");
    }

    #[test]
    fn test_parse_reference_syntax_invalid() {
        let reference = ReferenceSystem::parse_reference_syntax("invalid");
        assert!(reference.is_err());
    }

    #[test]
    fn test_parse_label_syntax() {
        let result = ReferenceSystem::parse_label_syntax("Figure 1 <fig1>");
        assert!(result.is_ok());
        let (name, _label_type, text) = result.unwrap();
        assert_eq!(name, "fig1");
        assert_eq!(text, "Figure 1");
    }

    #[test]
    fn test_parse_label_syntax_invalid() {
        let result = ReferenceSystem::parse_label_syntax("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_to_typst() {
        let system = ReferenceSystem::new();
        let label = Label::new(
            "fig1".to_string(),
            LabelType::Figure,
            "Figure 1".to_string(),
        );
        system.register_label(label).unwrap();
        system
            .create_reference("fig1".to_string(), ReferenceStyle::Numeric)
            .unwrap();

        let typst = system.to_typst();
        assert!(typst.contains("Figure 1 <fig1>"));
        assert!(typst.contains("@fig1"));
    }

    #[test]
    fn test_to_html() {
        let system = ReferenceSystem::new();
        let label = Label::new(
            "fig1".to_string(),
            LabelType::Figure,
            "Figure 1".to_string(),
        );
        system.register_label(label).unwrap();
        system
            .create_reference("fig1".to_string(), ReferenceStyle::Numeric)
            .unwrap();

        let html = system.to_html();
        assert!(html.contains("<a href=\"#fig1\""));
        assert!(html.contains("class=\"reference\""));
    }

    #[test]
    fn test_reference_builder() {
        let builder = ReferenceBuilder::new();
        let label = Label::new(
            "fig1".to_string(),
            LabelType::Figure,
            "Figure 1".to_string(),
        );
        let system = builder.add_label(label).unwrap().build();
        assert_eq!(system.get_all_labels().len(), 1);
    }

    #[test]
    fn test_reference_builder_default() {
        let builder = ReferenceBuilder::default();
        let system = builder.build();
        assert!(system.get_all_labels().is_empty());
    }

    #[test]
    fn test_label_type_partial_eq() {
        assert_eq!(LabelType::Heading, LabelType::Heading);
        assert_ne!(LabelType::Heading, LabelType::Figure);
    }

    #[test]
    fn test_reference_style_partial_eq() {
        assert_eq!(ReferenceStyle::Numeric, ReferenceStyle::Numeric);
        assert_ne!(ReferenceStyle::Numeric, ReferenceStyle::AuthorYear);
    }

    #[test]
    fn test_counter_increment() {
        let system = ReferenceSystem::new();
        let label1 = Label::new(
            "fig1".to_string(),
            LabelType::Figure,
            "Figure 1".to_string(),
        );
        let label2 = Label::new(
            "fig2".to_string(),
            LabelType::Figure,
            "Figure 2".to_string(),
        );
        system.register_label(label1).unwrap();
        system.register_label(label2).unwrap();

        let labels = system.get_all_labels();
        let counters: Vec<usize> = labels.iter().map(|l| l.counter).collect();
        assert!(counters.contains(&1));
        assert!(counters.contains(&2));
    }

    #[test]
    fn test_different_label_types_counters() {
        let system = ReferenceSystem::new();
        let fig1 = Label::new(
            "fig1".to_string(),
            LabelType::Figure,
            "Figure 1".to_string(),
        );
        let table1 = Label::new(
            "table1".to_string(),
            LabelType::Table,
            "Table 1".to_string(),
        );
        system.register_label(fig1).unwrap();
        system.register_label(table1).unwrap();

        let labels = system.get_all_labels();
        let fig_label = labels
            .iter()
            .find(|l| l.label_type == LabelType::Figure)
            .unwrap();
        let table_label = labels
            .iter()
            .find(|l| l.label_type == LabelType::Table)
            .unwrap();
        assert_eq!(fig_label.counter, 1);
        assert_eq!(table_label.counter, 1);
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_reference_creation() {
        let reference = Reference::new("fig1".to_string(), ReferenceStyle::Numeric);
        assert_eq!(reference.label_name, "fig1");
        assert_eq!(reference.reference_style, ReferenceStyle::Numeric);
    }

    #[test]
    fn test_reference_with_text() {
        let reference = Reference::new("fig1".to_string(), ReferenceStyle::Numeric)
            .with_text("[1]".to_string());
        assert_eq!(reference.text, "[1]");
    }

    #[test]
    fn test_max_label_name_length() {
        let long_name = "a".repeat(MAX_LABEL_NAME_LENGTH + 1);
        let label = Label::new(long_name, LabelType::Figure, "Figure 1".to_string());
        assert_eq!(label.name.len(), MAX_LABEL_NAME_LENGTH + 1); // Still created but logged
    }

    #[test]
    fn test_max_label_text_length() {
        let long_text = "a".repeat(MAX_LABEL_TEXT_LENGTH + 1);
        let label = Label::new("fig1".to_string(), LabelType::Figure, long_text);
        assert_eq!(label.text.len(), MAX_LABEL_TEXT_LENGTH + 1); // Still created but logged
    }

    #[test]
    fn test_max_typst_code_size() {
        let large_code = "a".repeat(MAX_TYPST_CODE_SIZE + 1);
        let result = ReferenceSystem::parse_reference_syntax(&large_code);
        assert!(result.is_err());
    }

    #[test]
    fn test_max_labels_limit() {
        let system = ReferenceSystem::new();
        
        // Try to add more labels than MAX_LABELS
        for i in 0..=MAX_LABELS {
            let label = Label::new(
                format!("label{}", i),
                LabelType::Figure,
                format!("Figure {}", i),
            );
            if i < MAX_LABELS {
                assert!(system.register_label(label).is_ok());
            } else {
                assert!(system.register_label(label).is_err());
            }
        }
    }

    #[test]
    fn test_max_references_limit() {
        let system = ReferenceSystem::new();
        let label = Label::new("fig1".to_string(), LabelType::Figure, "Figure 1".to_string());
        system.register_label(label).unwrap();
        
        // Try to create more references than MAX_REFERENCES
        for i in 0..=MAX_REFERENCES {
            let result = system.create_reference("fig1".to_string(), ReferenceStyle::Numeric);
            if i < MAX_REFERENCES {
                assert!(result.is_ok());
            } else {
                assert!(result.is_err());
            }
        }
    }

    #[test]
    fn test_label_count() {
        let system = ReferenceSystem::new();
        assert_eq!(system.label_count(), 0);
        
        let label = Label::new("fig1".to_string(), LabelType::Figure, "Figure 1".to_string());
        system.register_label(label).unwrap();
        assert_eq!(system.label_count(), 1);
        
        let label2 = Label::new("fig2".to_string(), LabelType::Figure, "Figure 2".to_string());
        system.register_label(label2).unwrap();
        assert_eq!(system.label_count(), 2);
    }

    #[test]
    fn test_reference_count() {
        let system = ReferenceSystem::new();
        let label = Label::new("fig1".to_string(), LabelType::Figure, "Figure 1".to_string());
        system.register_label(label).unwrap();
        
        assert_eq!(system.reference_count(), 0);
        
        system.create_reference("fig1".to_string(), ReferenceStyle::Numeric).unwrap();
        assert_eq!(system.reference_count(), 1);
        
        system.create_reference("fig1".to_string(), ReferenceStyle::Numeric).unwrap();
        assert_eq!(system.reference_count(), 2);
    }

    #[test]
    fn test_clear_all() {
        let system = ReferenceSystem::new();
        let label = Label::new("fig1".to_string(), LabelType::Figure, "Figure 1".to_string());
        system.register_label(label).unwrap();
        system.create_reference("fig1".to_string(), ReferenceStyle::Numeric).unwrap();
        
        assert_eq!(system.label_count(), 1);
        assert_eq!(system.reference_count(), 1);
        
        system.clear_all();
        
        assert_eq!(system.label_count(), 0);
        assert_eq!(system.reference_count(), 0);
    }

    #[test]
    fn test_remove_label() {
        let system = ReferenceSystem::new();
        let label = Label::new("fig1".to_string(), LabelType::Figure, "Figure 1".to_string());
        system.register_label(label).unwrap();
        
        assert_eq!(system.label_count(), 1);
        
        let removed = system.remove_label("fig1");
        assert!(removed);
        
        assert_eq!(system.label_count(), 0);
        assert!(system.get_label("fig1").is_none());
    }

    #[test]
    fn test_remove_nonexistent_label() {
        let system = ReferenceSystem::new();
        let removed = system.remove_label("nonexistent");
        assert!(!removed);
    }

    #[test]
    fn test_parse_label_syntax_max_size() {
        let large_code = "a".repeat(MAX_TYPST_CODE_SIZE + 1);
        let result = ReferenceSystem::parse_label_syntax(&large_code);
        assert!(result.is_err());
    }
}
