use serde::{Deserialize, Serialize};

/// 表单字段类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub enum FieldType {
    /// 文本输入框
    Text,
    /// 多行文本框
    TextArea,
    /// 复选框
    Checkbox,
    /// 单选按钮
    Radio,
    /// 下拉选择框
    ComboBox,
    /// 列表框
    ListBox,
    /// 按钮
    Button,
    /// 签名字段
    Signature,
}

/// 表单字段选项
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct FormFieldOptions {
    /// 字段名称
    pub name: String,
    /// 字段标签
    pub label: String,
    /// 默认值
    pub default_value: Option<String>,
    /// 是否必填
    pub required: bool,
    /// 最大长度
    pub max_length: Option<usize>,
    /// 可选值（用于下拉框、单选按钮等）
    pub options: Vec<String>,
    /// 只读
    pub read_only: bool,
    /// 是否可见
    pub visible: bool,
}

impl FormFieldOptions {
    /// 创建新的字段选项
    #[allow(dead_code)]
    pub fn new(name: String, label: String) -> Self {
        Self {
            name,
            label,
            default_value: None,
            required: false,
            max_length: None,
            options: Vec::new(),
            read_only: false,
            visible: true,
        }
    }

    /// 设置默认值
    #[allow(dead_code)]
    pub fn with_default_value(mut self, value: String) -> Self {
        self.default_value = Some(value);
        self
    }

    /// 设置是否必填
    #[allow(dead_code)]
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// 设置最大长度
    #[allow(dead_code)]
    pub fn with_max_length(mut self, length: usize) -> Self {
        self.max_length = Some(length);
        self
    }

    /// 添加选项
    #[allow(dead_code)]
    pub fn with_option(mut self, option: String) -> Self {
        self.options.push(option);
        self
    }

    /// 设置多个选项
    #[allow(dead_code)]
    pub fn with_options(mut self, options: Vec<String>) -> Self {
        self.options = options;
        self
    }

    /// 设置只读
    #[allow(dead_code)]
    pub fn with_read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// 设置可见性
    #[allow(dead_code)]
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}

/// PDF 表单字段
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct PdfFormField {
    /// 字段类型
    pub field_type: FieldType,
    /// 字段选项
    pub options: FormFieldOptions,
    /// 字段位置（X, Y 坐标，单位：点）
    pub position: (f64, f64),
    /// 字段尺寸（宽度、高度，单位：点）
    pub size: (f64, f64),
    /// 字体大小
    pub font_size: f64,
    /// 字体颜色（RGB）
    pub font_color: (u8, u8, u8),
    /// 背景颜色（RGB）
    pub background_color: (u8, u8, u8),
    /// 边框颜色（RGB）
    pub border_color: (u8, u8, u8),
    /// 边框宽度
    pub border_width: f64,
}

impl PdfFormField {
    /// 创建新的表单字段
    #[allow(dead_code)]
    pub fn new(field_type: FieldType, name: String, label: String) -> Self {
        Self {
            field_type,
            options: FormFieldOptions::new(name, label),
            position: (0.0, 0.0),
            size: (100.0, 20.0),
            font_size: 12.0,
            font_color: (0, 0, 0),
            background_color: (255, 255, 255),
            border_color: (0, 0, 0),
            border_width: 1.0,
        }
    }

    /// 设置位置
    #[allow(dead_code)]
    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.position = (x, y);
        self
    }

    /// 设置尺寸
    #[allow(dead_code)]
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.size = (width, height);
        self
    }

    /// 设置字体大小
    #[allow(dead_code)]
    pub fn with_font_size(mut self, size: f64) -> Self {
        self.font_size = size;
        self
    }

    /// 设置字体颜色
    #[allow(dead_code)]
    pub fn with_font_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.font_color = (r, g, b);
        self
    }

    /// 设置背景颜色
    #[allow(dead_code)]
    pub fn with_background_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.background_color = (r, g, b);
        self
    }

    /// 设置边框颜色
    #[allow(dead_code)]
    pub fn with_border_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.border_color = (r, g, b);
        self
    }

    /// 设置边框宽度
    #[allow(dead_code)]
    pub fn with_border_width(mut self, width: f64) -> Self {
        self.border_width = width;
        self
    }

    /// 设置字段选项
    #[allow(dead_code)]
    pub fn with_options(mut self, options: FormFieldOptions) -> Self {
        self.options = options;
        self
    }

    /// 创建文本输入框
    #[allow(dead_code)]
    pub fn text_input(name: String, label: String) -> Self {
        Self::new(FieldType::Text, name, label)
    }

    /// 创建多行文本框
    #[allow(dead_code)]
    pub fn text_area(name: String, label: String) -> Self {
        Self::new(FieldType::TextArea, name, label).with_size(200.0, 60.0)
    }

    /// 创建复选框
    #[allow(dead_code)]
    pub fn checkbox(name: String, label: String) -> Self {
        Self::new(FieldType::Checkbox, name, label).with_size(20.0, 20.0)
    }

    /// 创建单选按钮
    #[allow(dead_code)]
    pub fn radio(name: String, label: String) -> Self {
        Self::new(FieldType::Radio, name, label).with_size(20.0, 20.0)
    }

    /// 创建下拉选择框
    #[allow(dead_code)]
    pub fn combo_box(name: String, label: String, options: Vec<String>) -> Self {
        Self::new(FieldType::ComboBox, name.clone(), label.clone())
            .with_options(FormFieldOptions::new(name, label).with_options(options))
    }

    /// 创建签名字段
    #[allow(dead_code)]
    pub fn signature(name: String, label: String) -> Self {
        Self::new(FieldType::Signature, name, label).with_size(200.0, 50.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_field_options_new() {
        let options = FormFieldOptions::new("field1".to_string(), "Field 1".to_string());
        assert_eq!(options.name, "field1");
        assert_eq!(options.label, "Field 1");
        assert!(!options.required);
    }

    #[test]
    fn test_form_field_options_with_default_value() {
        let options = FormFieldOptions::new("field1".to_string(), "Field 1".to_string())
            .with_default_value("default".to_string());
        assert_eq!(options.default_value, Some("default".to_string()));
    }

    #[test]
    fn test_form_field_options_with_required() {
        let options =
            FormFieldOptions::new("field1".to_string(), "Field 1".to_string()).with_required(true);
        assert!(options.required);
    }

    #[test]
    fn test_form_field_options_with_max_length() {
        let options =
            FormFieldOptions::new("field1".to_string(), "Field 1".to_string()).with_max_length(100);
        assert_eq!(options.max_length, Some(100));
    }

    #[test]
    fn test_form_field_options_with_option() {
        let options = FormFieldOptions::new("field1".to_string(), "Field 1".to_string())
            .with_option("opt1".to_string());
        assert_eq!(options.options.len(), 1);
    }

    #[test]
    fn test_form_field_options_chaining() {
        let options = FormFieldOptions::new("field1".to_string(), "Field 1".to_string())
            .with_required(true)
            .with_max_length(100)
            .with_read_only(true);
        assert!(options.required);
        assert_eq!(options.max_length, Some(100));
        assert!(options.read_only);
    }

    #[test]
    fn test_pdf_form_field_new() {
        let field = PdfFormField::new(FieldType::Text, "field1".to_string(), "Field 1".to_string());
        assert_eq!(field.field_type, FieldType::Text);
        assert_eq!(field.options.name, "field1");
    }

    #[test]
    fn test_pdf_form_field_with_position() {
        let field = PdfFormField::new(FieldType::Text, "field1".to_string(), "Field 1".to_string())
            .with_position(100.0, 200.0);
        assert_eq!(field.position, (100.0, 200.0));
    }

    #[test]
    fn test_pdf_form_field_with_size() {
        let field = PdfFormField::new(FieldType::Text, "field1".to_string(), "Field 1".to_string())
            .with_size(150.0, 30.0);
        assert_eq!(field.size, (150.0, 30.0));
    }

    #[test]
    fn test_pdf_form_field_with_font_size() {
        let field = PdfFormField::new(FieldType::Text, "field1".to_string(), "Field 1".to_string())
            .with_font_size(14.0);
        assert_eq!(field.font_size, 14.0);
    }

    #[test]
    fn test_pdf_form_field_with_font_color() {
        let field = PdfFormField::new(FieldType::Text, "field1".to_string(), "Field 1".to_string())
            .with_font_color(255, 0, 0);
        assert_eq!(field.font_color, (255, 0, 0));
    }

    #[test]
    fn test_pdf_form_field_text_input() {
        let field = PdfFormField::text_input("name".to_string(), "Name".to_string());
        assert_eq!(field.field_type, FieldType::Text);
        assert_eq!(field.options.name, "name");
    }

    #[test]
    fn test_pdf_form_field_text_area() {
        let field = PdfFormField::text_area("description".to_string(), "Description".to_string());
        assert_eq!(field.field_type, FieldType::TextArea);
        assert_eq!(field.size, (200.0, 60.0));
    }

    #[test]
    fn test_pdf_form_field_checkbox() {
        let field = PdfFormField::checkbox("agree".to_string(), "I agree".to_string());
        assert_eq!(field.field_type, FieldType::Checkbox);
        assert_eq!(field.size, (20.0, 20.0));
    }

    #[test]
    fn test_pdf_form_field_radio() {
        let field = PdfFormField::radio("choice".to_string(), "Choice".to_string());
        assert_eq!(field.field_type, FieldType::Radio);
        assert_eq!(field.size, (20.0, 20.0));
    }

    #[test]
    fn test_pdf_form_field_combo_box() {
        let options = vec!["Option 1".to_string(), "Option 2".to_string()];
        let field = PdfFormField::combo_box("select".to_string(), "Select".to_string(), options);
        assert_eq!(field.field_type, FieldType::ComboBox);
        assert_eq!(field.options.options.len(), 2);
    }

    #[test]
    fn test_pdf_form_field_signature() {
        let field = PdfFormField::signature("signature".to_string(), "Sign here".to_string());
        assert_eq!(field.field_type, FieldType::Signature);
        assert_eq!(field.size, (200.0, 50.0));
    }

    #[test]
    fn test_pdf_form_field_chaining() {
        let field = PdfFormField::new(FieldType::Text, "field1".to_string(), "Field 1".to_string())
            .with_position(100.0, 200.0)
            .with_size(150.0, 30.0)
            .with_font_size(14.0);
        assert_eq!(field.position, (100.0, 200.0));
        assert_eq!(field.size, (150.0, 30.0));
        assert_eq!(field.font_size, 14.0);
    }

    #[test]
    fn test_pdf_form_field_serialization() {
        let field = PdfFormField::text_input("name".to_string(), "Name".to_string());
        let json = serde_json::to_string(&field);
        assert!(json.is_ok());
    }
}
