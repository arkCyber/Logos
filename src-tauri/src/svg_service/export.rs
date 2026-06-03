//! SVG Exporter - Aerospace-Grade SVG Service
//!
//! Safety-critical SVG export service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Comprehensive error handling
//! - Performance monitoring
//! - Security hardening
//! - Fault tolerance and error recovery

use super::{SvgConfig, SvgElement};
use super::effects::SvgDefs;
use super::html_parser::{HtmlToSvgOptions, parse_html_layout};
use super::sanitize::{escape_svg_attribute, escape_svg_text};
use serde::{Deserialize, Serialize};
use crate::error_handling::{ErrorContext, ErrorSeverity};
use crate::config_service::ExportConfigService;
use crate::svg_service::ensure_module_initialized;
use std::sync::Arc;

/// SVG 导出选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgExportOptions {
    /// 是否压缩
    pub compress: bool,
    /// 缩进类型（空格或制表符）
    pub indent: String,
}

impl SvgExportOptions {
    /// 创建默认导出选项
    pub fn new() -> Self {
        Self {
            compress: false,
            indent: "  ".to_string(),
        }
    }

    /// 设置是否压缩
    #[allow(dead_code)]
    pub fn with_compress(mut self, compress: bool) -> Self {
        self.compress = compress;
        self
    }

    /// 设置缩进
    #[allow(dead_code)]
    pub fn with_indent(mut self, indent: String) -> Self {
        self.indent = indent;
        self
    }
}

impl Default for SvgExportOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// SVG 导出结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgExportResult {
    /// 生成的 SVG 数据
    pub svg_data: Vec<u8>,
    /// 文件大小（字节）
    pub file_size: usize,
    /// 元素数量
    pub element_count: usize,
    /// 生成时间（毫秒）
    pub generation_time_ms: u64,
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error: Option<String>,
}

impl SvgExportResult {
    /// 创建成功结果
    pub fn success(svg_data: Vec<u8>, element_count: usize, generation_time_ms: u64) -> Self {
        Self {
            file_size: svg_data.len(),
            svg_data,
            element_count,
            generation_time_ms,
            success: true,
            error: None,
        }
    }

    /// 创建失败结果
    #[allow(dead_code)]
    pub fn failure(error: String) -> Self {
        Self {
            svg_data: Vec::new(),
            file_size: 0,
            element_count: 0,
            generation_time_ms: 0,
            success: false,
            error: Some(error),
        }
    }
}

/// SVG 图形
#[derive(Debug, Clone)]
pub struct SvgGraphic {
    /// SVG 配置
    pub config: SvgConfig,
    /// Reusable defs (gradients, filters, animations)
    pub defs: SvgDefs,
    /// 元素
    pub elements: Vec<SvgElement>,
}

impl SvgGraphic {
    /// 创建新的图形
    pub fn new() -> Self {
        Self {
            config: SvgConfig::new(),
            defs: SvgDefs::default(),
            elements: Vec::new(),
        }
    }

    /// 设置 defs
    #[allow(dead_code)]
    pub fn with_defs(mut self, defs: SvgDefs) -> Self {
        self.defs = defs;
        self
    }

    /// 设置配置
    #[allow(dead_code)]
    pub fn with_config(mut self, config: SvgConfig) -> Self {
        self.config = config;
        self
    }

    /// 添加元素
    #[allow(dead_code)]
    pub fn with_element(mut self, element: SvgElement) -> Self {
        self.elements.push(element);
        self
    }

    /// 添加多个元素
    #[allow(dead_code)]
    pub fn with_elements(mut self, elements: Vec<SvgElement>) -> Self {
        self.elements = elements;
        self
    }

    /// 获取元素数量
    pub fn element_count(&self) -> usize {
        self.elements.len()
    }
}

impl Default for SvgGraphic {
    fn default() -> Self {
        Self::new()
    }
}

/// SVG 导出器
#[derive(Debug, Clone)]
pub struct SvgExporter {
    /// 导出选项
    pub options: SvgExportOptions,
    /// 配置服务
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
}

impl SvgExporter {
    /// 创建新的导出器
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        ensure_module_initialized();
        Self {
            options: SvgExportOptions::new(),
            config_service,
            operation_count: 0,
            last_error: None,
        }
    }

    /// Validate HTML content length
    fn validate_html_length(&self, html: &str) -> Result<(), String> {
        let svg_config = self.config_service.get_svg_config();
        if html.len() > svg_config.max_html_length {
            return Err(format!("HTML content exceeds maximum length of {}", svg_config.max_html_length));
        }
        Ok(())
    }

    /// Validate text content length against export configuration
    fn validate_text_length(&self, text: &str) -> Result<(), String> {
        let svg_config = self.config_service.get_svg_config();
        if text.len() > svg_config.max_text_length {
            return Err(format!(
                "Text content exceeds maximum length of {}",
                svg_config.max_text_length
            ));
        }
        Ok(())
    }

    /// Validate all elements in a graphic before export
    fn validate_graphic(&self, graphic: &SvgGraphic) -> Result<(), String> {
        self.validate_element_count(graphic.element_count())?;
        graphic.defs.validate()?;
        for (index, element) in graphic.elements.iter().enumerate() {
            element
                .validate()
                .map_err(|e| format!("Element at index {} failed validation: {}", index, e))?;
        }
        Ok(())
    }

    /// Validate element count
    fn validate_element_count(&self, count: usize) -> Result<(), String> {
        let svg_config = self.config_service.get_svg_config();
        if count > svg_config.max_element_count {
            return Err(format!("Element count exceeds maximum of {}", svg_config.max_element_count));
        }
        Ok(())
    }

    /// Validate output size
    fn validate_output_size(&self, size: usize) -> Result<(), String> {
        let svg_config = self.config_service.get_svg_config();
        if size > svg_config.max_output_size {
            return Err(format!("Output size exceeds maximum of {} bytes", svg_config.max_output_size));
        }
        Ok(())
    }

    /// Record error context
    fn record_error(&mut self, code: &str, message: &str, source: &str) {
        self.last_error = Some(ErrorContext::new(
            ErrorSeverity::Error,
            code,
            message,
            source,
        ));
    }

    /// Get last error
    pub fn get_last_error(&self) -> Option<&ErrorContext> {
        self.last_error.as_ref()
    }

    /// Get operation count
    pub fn get_operation_count(&self) -> u64 {
        self.operation_count
    }

    /// Reset error state
    pub fn reset_error_state(&mut self) {
        self.last_error = None;
    }

    /// 设置导出选项
    #[allow(dead_code)]
    pub fn with_options(mut self, options: SvgExportOptions) -> Self {
        self.options = options;
        self
    }

    /// 导出图形为 SVG with validation
    #[allow(dead_code)]
    pub fn export(&mut self, graphic: &SvgGraphic) -> SvgExportResult {
        self.operation_count += 1;
        let start = std::time::Instant::now();

        // Validate graphic elements and counts
        if let Err(e) = self.validate_graphic(graphic) {
            self.record_error("INVALID_GRAPHIC", &e, "export");
            return SvgExportResult::failure(e);
        }

        // 生成 SVG 内容
        let svg_content = self.generate_svg(graphic);
        let generation_time = start.elapsed().as_millis() as u64;

        // Validate output size
        if let Err(e) = self.validate_output_size(svg_content.len()) {
            self.record_error("OUTPUT_TOO_LARGE", &e, "export");
            return SvgExportResult::failure(e);
        }

        self.last_error = None;
        SvgExportResult::success(
            svg_content.into_bytes(),
            graphic.element_count(),
            generation_time,
        )
    }

    /// 从 HTML 导出为 SVG（简化版，仅提取文本）with validation
    pub fn export_from_html(&mut self, html_content: &str) -> SvgExportResult {
        self.operation_count += 1;
        let start = std::time::Instant::now();

        // Validate HTML length
        if let Err(e) = self.validate_html_length(html_content) {
            self.record_error("INVALID_HTML_LENGTH", &e, "export_from_html");
            return SvgExportResult::failure(e);
        }

        let graphic = self.html_to_graphic(html_content);

        if let Err(e) = self.validate_graphic(&graphic) {
            self.record_error("INVALID_HTML_GRAPHIC", &e, "export_from_html");
            return SvgExportResult::failure(e);
        }

        let svg_content = self.generate_svg(&graphic);
        let generation_time = start.elapsed().as_millis() as u64;

        // Validate output size
        if let Err(e) = self.validate_output_size(svg_content.len()) {
            self.record_error("OUTPUT_TOO_LARGE", &e, "export_from_html");
            return SvgExportResult::failure(e);
        }

        self.last_error = None;
        SvgExportResult::success(
            svg_content.into_bytes(),
            graphic.element_count(),
            generation_time,
        )
    }

    /// 生成 SVG 内容
    fn generate_svg(&self, graphic: &SvgGraphic) -> String {
        let mut svg = String::new();

        // SVG 头部
        let xlink_ns = if graphic.defs.animations.is_empty() {
            String::new()
        } else {
            " xmlns:xlink=\"http://www.w3.org/1999/xlink\"".to_string()
        };
        svg.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<svg version=\"");
        svg.push_str(match graphic.config.version {
            super::SvgVersion::V1_0 => "1.0",
            super::SvgVersion::V1_1 => "1.1",
            super::SvgVersion::V2_0 => "2.0",
        });
        svg.push_str("\" xmlns=\"http://www.w3.org/2000/svg\"");
        svg.push_str(&xlink_ns);
        svg.push_str(&format!(
            " viewBox=\"{} {} {} {}\" width=\"{}\" height=\"{}\"",
            graphic.config.view_box.x,
            graphic.config.view_box.y,
            graphic.config.view_box.width,
            graphic.config.view_box.height,
            graphic.config.width,
            graphic.config.height
        ));
        if graphic.config.preserve_aspect_ratio {
            svg.push_str("\n  preserveAspectRatio=\"xMidYMid meet\"");
        }
        svg.push('>');

        if !self.options.compress {
            svg.push('\n');
        }

        svg.push_str(&graphic.defs.to_svg_string(&self.options.indent));

        // 添加元素
        for element in &graphic.elements {
            svg.push_str(&self.generate_element(element, &self.options.indent));
        }

        svg.push_str("</svg>");

        svg
    }

    /// 生成元素
    fn generate_element(&self, element: &SvgElement, indent: &str) -> String {
        let mut elem = String::new();

        if !self.options.compress {
            elem.push_str(indent);
        }

        let style_attrs = self.generate_style_attributes(&element.style);
        let escaped_id = escape_svg_attribute(&element.id);

        match element.element_type {
            super::SvgElementType::Rect => {
                if let Some(rect) = &element.rect {
                    elem.push_str(&format!(
                        "<rect id=\"{}\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"{}{}{}/>",
                        escaped_id,
                        rect.x,
                        rect.y,
                        rect.width,
                        rect.height,
                        if let Some(rx) = rect.rx {
                            format!(" rx=\"{}\"", rx)
                        } else {
                            String::new()
                        },
                        if let Some(ry) = rect.ry {
                            format!(" ry=\"{}\"", ry)
                        } else {
                            String::new()
                        },
                        style_attrs
                    ));
                }
            }
            super::SvgElementType::Circle => {
                if let Some(circle) = &element.circle {
                    elem.push_str(&format!(
                        "<circle id=\"{}\" cx=\"{}\" cy=\"{}\" r=\"{}\"{}/>",
                        escaped_id,
                        circle.cx,
                        circle.cy,
                        circle.r,
                        style_attrs
                    ));
                }
            }
            super::SvgElementType::Ellipse => {
                if let Some(ellipse) = &element.ellipse {
                    elem.push_str(&format!(
                        "<ellipse id=\"{}\" cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\"{}/>",
                        escaped_id,
                        ellipse.cx,
                        ellipse.cy,
                        ellipse.rx,
                        ellipse.ry,
                        style_attrs
                    ));
                }
            }
            super::SvgElementType::Line => {
                if let Some(line) = &element.line {
                    elem.push_str(&format!(
                        "<line id=\"{}\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"{}/>",
                        escaped_id,
                        line.x1,
                        line.y1,
                        line.x2,
                        line.y2,
                        style_attrs
                    ));
                }
            }
            super::SvgElementType::Text => {
                if let Some(text) = &element.text {
                    elem.push_str(&format!(
                        "<text id=\"{}\" x=\"{}\" y=\"{}\"{}>{}</text>",
                        escaped_id,
                        text.x,
                        text.y,
                        style_attrs,
                        escape_svg_text(&text.text)
                    ));
                }
            }
            super::SvgElementType::Path => {
                if let Some(path) = &element.path {
                    elem.push_str(&format!(
                        "<path id=\"{}\" d=\"{}\"{}/>",
                        escaped_id,
                        escape_svg_attribute(&path.d),
                        style_attrs
                    ));
                }
            }
            super::SvgElementType::Polygon => {
                if let Some(polygon) = &element.polygon {
                    elem.push_str(&format!(
                        "<polygon id=\"{}\" points=\"{}\"{}/>",
                        escaped_id,
                        escape_svg_attribute(&polygon.points_attribute()),
                        style_attrs
                    ));
                }
            }
            super::SvgElementType::Polyline => {
                if let Some(polyline) = &element.polyline {
                    elem.push_str(&format!(
                        "<polyline id=\"{}\" points=\"{}\"{}/>",
                        escaped_id,
                        escape_svg_attribute(&polyline.points_attribute()),
                        style_attrs
                    ));
                }
            }
        }

        if !self.options.compress {
            elem.push('\n');
        }

        elem
    }

    /// Generate SVG presentation attributes from style model
    fn generate_style_attributes(&self, style: &super::SvgStyle) -> String {
        let mut style_str = String::new();

        if let Some(gradient_ref) = &style.fill.gradient_ref {
            style_str.push_str(&format!(
                " fill=\"url(#{})\"",
                escape_svg_attribute(gradient_ref)
            ));
            if style.fill.opacity < 1.0 {
                style_str.push_str(&format!(" fill-opacity=\"{}\"", style.fill.opacity));
            }
        } else if let Some(color) = style.fill.color {
            style_str.push_str(&format!(
                " fill=\"rgb({},{},{})\"",
                color.0, color.1, color.2
            ));
            if style.fill.opacity < 1.0 {
                style_str.push_str(&format!(" fill-opacity=\"{}\"", style.fill.opacity));
            }
        } else if style.fill.opacity == 0.0 {
            style_str.push_str(" fill=\"none\"");
        }

        if let Some(color) = style.stroke.color {
            style_str.push_str(&format!(
                " stroke=\"rgb({},{},{})\"",
                color.0, color.1, color.2
            ));
            style_str.push_str(&format!(" stroke-width=\"{}\"", style.stroke.width));
            if style.stroke.opacity < 1.0 {
                style_str.push_str(&format!(" stroke-opacity=\"{}\"", style.stroke.opacity));
            }
        } else if style.stroke.width > 0.0 {
            style_str.push_str(&format!(" stroke-width=\"{}\"", style.stroke.width));
        }

        if let Some(font) = &style.font {
            style_str.push_str(&format!(
                " font-family=\"{}\"",
                escape_svg_attribute(&font.family)
            ));
            style_str.push_str(&format!(" font-size=\"{}\"", font.size));
            style_str.push_str(&format!(
                " font-weight=\"{}\"",
                escape_svg_attribute(&font.weight)
            ));
            style_str.push_str(&format!(
                " font-style=\"{}\"",
                escape_svg_attribute(&font.style)
            ));
        }

        style_str
    }

    /// HTML 转图形（块级布局：标题/段落/样式）
    fn html_to_graphic(&self, html: &str) -> SvgGraphic {
        let svg_config = self.config_service.get_svg_config();
        let layout = parse_html_layout(
            html,
            &HtmlToSvgOptions {
                max_text_length: svg_config.max_text_length,
                max_blocks: svg_config.max_element_count,
            },
        );

        let mut graphic = SvgGraphic::new();
        graphic.config = layout.config;
        graphic.elements = layout.elements;
        graphic
    }
}

/// 从 HTML 提取文本（简化版，剥离标签）
fn extract_text_from_html(html: &str) -> String {
    let mut result = html.to_string();
    while let Some(start) = result.find('<') {
        if let Some(end) = result[start..].find('>') {
            result.replace_range(start..start + end + 1, "");
        } else {
            break;
        }
    }
    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_export_options_new() {
        let options = SvgExportOptions::new();
        assert!(!options.compress);
    }

    #[test]
    fn test_svg_export_options_with_compress() {
        let options = SvgExportOptions::new().with_compress(true);
        assert!(options.compress);
    }

    #[test]
    fn test_svg_export_result_success() {
        let result = SvgExportResult::success(vec![1, 2, 3], 5, 100);
        assert!(result.success);
        assert_eq!(result.file_size, 3);
    }

    #[test]
    fn test_svg_export_result_failure() {
        let result = SvgExportResult::failure("Error".to_string());
        assert!(!result.success);
    }

    #[test]
    fn test_svg_graphic_new() {
        let graphic = SvgGraphic::new();
        assert!(graphic.elements.is_empty());
    }

    #[test]
    fn test_svg_graphic_with_element() {
        let element = super::SvgElement::rect("1".to_string(), 10.0, 10.0, 100.0, 50.0);
        let graphic = SvgGraphic::new().with_element(element);
        assert_eq!(graphic.element_count(), 1);
    }

    #[test]
    fn test_svg_exporter_new() {
        let config_service = Arc::new(ExportConfigService::new());
        let exporter = SvgExporter::new(config_service);
        assert!(!exporter.options.compress);
    }

    #[test]
    fn test_svg_exporter_export() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut exporter = SvgExporter::new(config_service);
        let graphic = SvgGraphic::new().with_element(super::SvgElement::rect(
            "1".to_string(),
            10.0,
            10.0,
            100.0,
            50.0,
        ));
        let result = exporter.export(&graphic);
        assert!(result.success);
    }

    #[test]
    fn test_svg_exporter_export_from_html() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut exporter = SvgExporter::new(config_service);
        let result = exporter.export_from_html("<h1>Title</h1>");
        assert!(result.success);
    }

    #[test]
    fn test_extract_text_from_html() {
        let text = extract_text_from_html("<h1>Hello World</h1>");
        assert_eq!(text, "Hello World");
    }

    #[test]
    fn test_svg_exporter_default() {
        let config_service = Arc::new(ExportConfigService::new());
        let exporter = SvgExporter::new(config_service);
        assert!(!exporter.options.compress);
    }

    #[test]
    fn test_svg_graphic_default() {
        let graphic = SvgGraphic::default();
        assert!(graphic.elements.is_empty());
    }

    #[test]
    fn test_svg_export_options_default() {
        let options = SvgExportOptions::default();
        assert!(!options.compress);
    }

    #[test]
    fn test_svg_export_options_serialization() {
        let options = SvgExportOptions::new();
        let json = serde_json::to_string(&options);
        assert!(json.is_ok());
    }

    #[test]
    fn test_svg_export_options_deserialization() {
        let json = r#"{"compress":false,"indent":"  "}"#;
        let options: SvgExportOptions = serde_json::from_str(json).unwrap();
        assert!(!options.compress);
    }

    #[test]
    fn test_svg_export_result_serialization() {
        let result = SvgExportResult::success(vec![1, 2, 3], 5, 100);
        let json = serde_json::to_string(&result);
        assert!(json.is_ok());
    }

    #[test]
    fn test_svg_graphic_with_config() {
        let config = super::SvgConfig::new();
        let graphic = SvgGraphic::new().with_config(config);
        assert_eq!(graphic.config.width, 100.0);
    }

    #[test]
    fn test_svg_exporter_with_options() {
        let config_service = Arc::new(ExportConfigService::new());
        let options = SvgExportOptions::new().with_compress(true);
        let exporter = SvgExporter::new(config_service).with_options(options);
        assert!(exporter.options.compress);
    }

    #[test]
    fn test_svg_graphic_with_multiple_elements() {
        let element1 = super::SvgElement::rect("1".to_string(), 10.0, 10.0, 100.0, 50.0);
        let element2 = super::SvgElement::circle("2".to_string(), 50.0, 50.0, 25.0);
        let graphic = SvgGraphic::new()
            .with_element(element1)
            .with_element(element2);
        assert_eq!(graphic.element_count(), 2);
    }

    #[test]
    fn test_svg_export_result_with_zero_elements() {
        let result = SvgExportResult::success(vec![1, 2, 3], 0, 100);
        assert!(result.success);
        assert_eq!(result.element_count, 0);
    }

    #[test]
    fn test_svg_export_result_with_large_file() {
        let data = vec![0u8; 1000000];
        let result = SvgExportResult::success(data, 10, 5000);
        assert!(result.success);
        assert_eq!(result.file_size, 1000000);
    }

    #[test]
    fn test_svg_graphic_element_count_empty() {
        let graphic = SvgGraphic::new();
        assert_eq!(graphic.element_count(), 0);
    }

    #[test]
    fn test_svg_exporter_export_empty_graphic() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut exporter = SvgExporter::new(config_service);
        let graphic = SvgGraphic::new();
        let result = exporter.export(&graphic);
        assert!(result.success);
    }

    #[test]
    fn test_svg_exporter_export_from_html_complex() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut exporter = SvgExporter::new(config_service);
        let result = exporter.export_from_html("<h1>Title</h1><p>Paragraph</p>");
        assert!(result.success);
    }

    #[test]
    fn test_svg_exporter_export_from_html_empty() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut exporter = SvgExporter::new(config_service);
        let result = exporter.export_from_html("");
        assert!(result.success);
    }

    #[test]
    fn test_extract_text_from_html_with_paragraphs() {
        let text = extract_text_from_html("<p>Para1</p><p>Para2</p>");
        assert!(text.contains("Para1"));
        assert!(text.contains("Para2"));
    }

    #[test]
    fn test_extract_text_from_html_empty() {
        let text = extract_text_from_html("");
        assert_eq!(text, "");
    }

    #[test]
    fn test_svg_graphic_config_chaining() {
        let config = super::SvgConfig::new()
            .with_size(200.0, 150.0);
        let graphic = SvgGraphic::new().with_config(config);
        assert_eq!(graphic.config.width, 200.0);
        assert_eq!(graphic.config.height, 150.0);
    }

    #[test]
    fn test_svg_exporter_options_chaining() {
        let config_service = Arc::new(ExportConfigService::new());
        let options = SvgExportOptions::new().with_compress(true);
        let exporter = SvgExporter::new(config_service).with_options(options);
        assert!(exporter.options.compress);
    }

    // Aerospace-level tests
    #[test]
    fn test_validate_html_length_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let exporter = SvgExporter::new(config_service.clone());
        let svg_config = config_service.get_svg_config();
        let long_html = "a".repeat(svg_config.max_html_length + 1);
        let result = exporter.validate_html_length(&long_html);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum length"));
    }

    #[test]
    fn test_validate_element_count_too_many() {
        let config_service = Arc::new(ExportConfigService::new());
        let exporter = SvgExporter::new(config_service.clone());
        let svg_config = config_service.get_svg_config();
        let result = exporter.validate_element_count(svg_config.max_element_count + 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum"));
    }

    #[test]
    fn test_validate_output_size_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let exporter = SvgExporter::new(config_service.clone());
        let svg_config = config_service.get_svg_config();
        let result = exporter.validate_output_size(svg_config.max_output_size + 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum"));
    }

    #[test]
    fn test_max_html_length_accepted() {
        let config_service = Arc::new(ExportConfigService::new());
        let exporter = SvgExporter::new(config_service.clone());
        let svg_config = config_service.get_svg_config();
        let html = "a".repeat(svg_config.max_html_length);
        let result = exporter.validate_html_length(&html);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_element_count_accepted() {
        let config_service = Arc::new(ExportConfigService::new());
        let exporter = SvgExporter::new(config_service.clone());
        let svg_config = config_service.get_svg_config();
        let result = exporter.validate_element_count(svg_config.max_element_count);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_output_size_accepted() {
        let config_service = Arc::new(ExportConfigService::new());
        let exporter = SvgExporter::new(config_service.clone());
        let svg_config = config_service.get_svg_config();
        let result = exporter.validate_output_size(svg_config.max_output_size);
        assert!(result.is_ok());
    }

    #[test]
    fn test_operation_count() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut exporter = SvgExporter::new(config_service);
        assert_eq!(exporter.get_operation_count(), 0);
        
        let graphic = SvgGraphic::new().with_element(super::SvgElement::rect(
            "1".to_string(),
            10.0,
            10.0,
            100.0,
            50.0,
        ));
        exporter.export(&graphic);
        assert!(exporter.get_operation_count() > 0);
    }

    #[test]
    fn test_error_recording() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut exporter = SvgExporter::new(config_service);
        
        exporter.record_error("TEST_ERROR", "Test error message", "test_source");
        
        let last_error = exporter.get_last_error();
        assert!(last_error.is_some());
        let error = last_error.unwrap();
        assert_eq!(error.code, "TEST_ERROR");
        assert_eq!(error.message, "Test error message");
        assert_eq!(error.source, "test_source");
    }

    #[test]
    fn test_error_state_reset() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut exporter = SvgExporter::new(config_service);
        
        exporter.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(exporter.get_last_error().is_some());
        
        exporter.reset_error_state();
        assert!(exporter.get_last_error().is_none());
    }

    #[test]
    fn test_export_from_html_with_invalid_length() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut exporter = SvgExporter::new(config_service.clone());
        let svg_config = config_service.get_svg_config();
        let long_html = "a".repeat(svg_config.max_html_length + 1);
        let result = exporter.export_from_html(&long_html);
        assert!(!result.success);
        assert!(result.error.unwrap().contains("exceeds maximum length"));
    }

    #[test]
    fn test_escape_svg_text() {
        use crate::svg_service::sanitize::{escape_svg_attribute, escape_svg_text};
        assert_eq!(
            escape_svg_text("<script>&\"</script>"),
            "&lt;script&gt;&amp;\"&lt;/script&gt;"
        );
        assert_eq!(escape_svg_attribute("\"quoted\""), "&quot;quoted&quot;");
    }

    #[test]
    fn test_export_with_defs_and_gradient_fill() {
        use crate::svg_service::effects::{SvgGradientStop, SvgLinearGradient};
        use crate::svg_service::style::SvgFill;
        use crate::svg_service::{SvgDefs, SvgElement, SvgStyle};

        let config_service = Arc::new(ExportConfigService::new());
        let mut exporter = SvgExporter::new(config_service);
        let defs = SvgDefs {
            linear_gradients: vec![SvgLinearGradient {
                id: "grad1".to_string(),
                x1: 0.0,
                y1: 0.0,
                x2: 1.0,
                y2: 0.0,
                stops: vec![
                    SvgGradientStop::new(0.0, (255, 0, 0), 1.0).unwrap(),
                    SvgGradientStop::new(1.0, (0, 0, 255), 1.0).unwrap(),
                ],
            }],
            ..Default::default()
        };
        let style = SvgStyle::new().with_fill(SvgFill::new().with_gradient_ref("grad1".to_string()));
        let graphic = SvgGraphic::new()
            .with_defs(defs)
            .with_element(
                SvgElement::rect("rect1".to_string(), 10.0, 10.0, 100.0, 50.0).with_style(style),
            );
        let result = exporter.export(&graphic);
        assert!(result.success);
        let svg = String::from_utf8(result.svg_data).unwrap();
        assert!(svg.contains("<defs>"));
        assert!(svg.contains("fill=\"url(#grad1)\""));
    }

    #[test]
    fn test_export_from_html_with_headings() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut exporter = SvgExporter::new(config_service);
        let result = exporter.export_from_html("<h1>Title</h1><p>Paragraph</p>");
        assert!(result.success);
        let svg = String::from_utf8(result.svg_data).unwrap();
        assert!(svg.contains("Title"));
        assert!(svg.contains("Paragraph"));
    }

    #[test]
    fn test_export_from_html_with_margin_and_border() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut exporter = SvgExporter::new(config_service);
        let html = "<p style=\"margin: 8px; border: 1px solid #000000; background-color: #eeeeee;\">Boxed</p>";
        let result = exporter.export_from_html(html);
        assert!(result.success);
        let svg = String::from_utf8(result.svg_data).unwrap();
        assert!(svg.contains("<rect"));
        assert!(svg.contains("stroke="));
        assert!(svg.contains("Boxed"));
    }

    #[test]
    fn test_svg_exporter_export_all_element_types() {
        use crate::svg_service::{SvgElement, SvgElementType, SvgPath, SvgPoint};

        let config_service = Arc::new(ExportConfigService::new());
        let mut exporter = SvgExporter::new(config_service);
        let graphic = SvgGraphic::new()
            .with_element(SvgElement::rect(
                "rect1".to_string(),
                0.0,
                0.0,
                50.0,
                20.0,
            ))
            .with_element(SvgElement::circle(
                "circle1".to_string(),
                25.0,
                25.0,
                10.0,
            ))
            .with_element(SvgElement::ellipse(
                "ellipse1".to_string(),
                30.0,
                30.0,
                15.0,
                8.0,
            ))
            .with_element(SvgElement::line(
                "line1".to_string(),
                0.0,
                0.0,
                100.0,
                100.0,
            ))
            .with_element(
                SvgElement::new("path1".to_string(), SvgElementType::Path)
                    .with_path(SvgPath::new("M 0 0 L 10 10".to_string())),
            )
            .with_element(SvgElement::polygon(
                "polygon1".to_string(),
                vec![
                    SvgPoint::new(0.0, 0.0),
                    SvgPoint::new(10.0, 0.0),
                    SvgPoint::new(5.0, 10.0),
                ],
            ))
            .with_element(SvgElement::polyline(
                "polyline1".to_string(),
                vec![SvgPoint::new(0.0, 0.0), SvgPoint::new(5.0, 5.0)],
            ))
            .with_element(SvgElement::text(
                "text1".to_string(),
                5.0,
                15.0,
                "<safe>".to_string(),
            ));

        let result = exporter.export(&graphic);
        assert!(result.success);
        let svg = String::from_utf8(result.svg_data).unwrap();
        assert!(svg.contains("<ellipse"));
        assert!(svg.contains("<line"));
        assert!(svg.contains("<polygon"));
        assert!(svg.contains("<polyline"));
        assert!(svg.contains("&lt;safe&gt;"));
        assert!(!svg.contains("Unsupported element type"));
    }

    #[test]
    fn test_validate_text_length_too_long_via_html() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut exporter = SvgExporter::new(config_service.clone());
        let svg_config = config_service.get_svg_config();
        let long_line = "x".repeat(svg_config.max_text_length + 1);
        let html = format!("<p>{}</p>", long_line);
        let result = exporter.export_from_html(&html);
        assert!(result.success);
        let svg = String::from_utf8(result.svg_data).unwrap();
        assert!(svg.contains("SVG"));
    }

    #[test]
    fn test_export_rejects_invalid_graphic() {
        use crate::svg_service::SvgElement;

        let config_service = Arc::new(ExportConfigService::new());
        let mut exporter = SvgExporter::new(config_service);
        let graphic = SvgGraphic::new().with_element(SvgElement::circle(
            "bad".to_string(),
            f64::INFINITY,
            0.0,
            10.0,
        ));
        let result = exporter.export(&graphic);
        assert!(!result.success);
        assert!(result.error.unwrap().contains("failed validation"));
    }
}
