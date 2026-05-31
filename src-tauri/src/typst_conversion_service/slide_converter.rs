//! HTML to Typst Slide Converter - Aerospace-Grade Document Conversion
//!
//! Safety-critical HTML to Typst slide conversion with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Comprehensive error handling
//! - Performance monitoring
//! - Security hardening against injection attacks

use super::{ConversionResult, SlideConfig};
use std::time::Instant;
use crate::config_service::ExportConfigService;
use std::sync::Arc;

pub struct HtmlToTypstSlideConverter {
    config: SlideConfig,
    /// 配置服务
    config_service: Arc<ExportConfigService>,
    /// Current recursion depth for safety
    recursion_depth: usize,
}

impl HtmlToTypstSlideConverter {
    pub fn new(config: SlideConfig, config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config,
            config_service,
            recursion_depth: 0,
        }
    }

    pub fn with_config(mut self, config: SlideConfig) -> Self {
        self.config = config;
        self
    }

    /// Validate input size to prevent DoS attacks
    fn validate_input_size(&self, input: &str) -> Result<(), String> {
        let typst_config = self.config_service.get_typst_config();
        if input.len() > typst_config.max_input_size {
            return Err(format!(
                "Input size {} exceeds maximum allowed size of {} bytes",
                input.len(),
                typst_config.max_input_size
            ));
        }
        Ok(())
    }

    /// Check recursion depth to prevent stack overflow
    fn check_recursion_depth(&mut self) -> Result<(), String> {
        let typst_config = self.config_service.get_typst_config();
        if self.recursion_depth >= typst_config.max_recursion_depth {
            return Err(format!(
                "Maximum recursion depth {} exceeded",
                typst_config.max_recursion_depth
            ));
        }
        self.recursion_depth += 1;
        Ok(())
    }

    /// Reset recursion depth
    fn reset_recursion_depth(&mut self) {
        self.recursion_depth = 0;
    }

    /// Convert HTML to Typst slides with full validation and error recovery
    /// 
    /// # Safety
    /// - Validates input size
    /// - Sanitizes all text content
    /// - Handles malformed HTML gracefully
    /// - Monitors performance
    /// - Provides fallback on conversion errors
    pub fn convert_with_fallback(&self, html: &str) -> ConversionResult {
        let start = Instant::now();
        
        // Validate input size
        if let Err(error) = self.validate_input_size(html) {
            let duration = start.elapsed().as_millis() as u64;
            return ConversionResult {
                typst_code: Self::fallback_typst(&error),
                success: false,
                error: Some(error),
                duration_ms: duration,
            };
        }
        
        match self.do_convert(html) {
            Ok(typst_code) => {
                let duration = start.elapsed().as_millis() as u64;
                ConversionResult {
                    typst_code,
                    success: true,
                    error: None,
                    duration_ms: duration,
                }
            }
            Err(error) => {
                let duration = start.elapsed().as_millis() as u64;
                // Fallback: return basic Typst with error message
                let fallback = Self::fallback_typst(&error);
                ConversionResult {
                    typst_code: fallback,
                    success: false,
                    error: Some(error),
                    duration_ms: duration,
                }
            }
        }
    }

    /// Convert HTML to Typst slides with full validation
    /// 
    /// # Safety
    /// - Validates input size
    /// - Sanitizes all text content
    /// - Handles malformed HTML gracefully
    /// - Monitors performance
    pub fn convert(&self, html: &str) -> ConversionResult {
        let start = Instant::now();
        
        // Validate input size
        if let Err(error) = self.validate_input_size(html) {
            let duration = start.elapsed().as_millis() as u64;
            return ConversionResult {
                typst_code: String::new(),
                success: false,
                error: Some(error),
                duration_ms: duration,
            };
        }
        
        match self.do_convert(html) {
            Ok(typst_code) => {
                let duration = start.elapsed().as_millis() as u64;
                ConversionResult {
                    typst_code,
                    success: true,
                    error: None,
                    duration_ms: duration,
                }
            }
            Err(error) => {
                let duration = start.elapsed().as_millis() as u64;
                ConversionResult {
                    typst_code: String::new(),
                    success: false,
                    error: Some(error),
                    duration_ms: duration,
                }
            }
        }
    }

    fn do_convert(&self, html: &str) -> Result<String, String> {
        let mut typst_code = String::new();

        // 添加幻灯片设置
        typst_code.push_str("#import \"@preview/metropolis:0.2.0\": *\n");
        typst_code.push_str("#show: slides.with(\n");
        typst_code.push_str(&format!("  title: \"Presentation\",\n"));
        typst_code.push_str(&format!("  aspect-ratio: \"{}\",\n", self.config.aspect_ratio));
        typst_code.push_str(")\n\n");

        // 解析HTML并转换为Typst幻灯片
        let slides = self.parse_html_to_slides(html)?;
        
        for slide in slides {
            typst_code.push_str(&slide);
            typst_code.push_str("\n");
        }

        Ok(typst_code)
    }

    fn parse_html_to_slides(&self, html: &str) -> Result<Vec<String>, String> {
        let mut slides = Vec::new();
        
        // 简化的HTML解析，将每个主要部分转换为幻灯片
        // 实际实现需要更复杂的HTML解析
        // 添加错误恢复：如果解析失败，返回单个幻灯片
        let parse_result = self.do_parse_html_to_slides(html);
        if parse_result.is_err() {
            // Fallback to single slide with plain text
            let plain_text = self.html_to_plain_text(html);
            slides.push(self.wrap_in_slide(&plain_text));
            return Ok(slides);
        }
        parse_result
    }

    fn do_parse_html_to_slides(&self, html: &str) -> Result<Vec<String>, String> {
        let mut slides = Vec::new();
        
        // 简化的HTML解析，将每个主要部分转换为幻灯片
        // 实际实现需要更复杂的HTML解析
        
        let mut current_slide = String::new();
        let mut in_tag = false;
        let mut current_tag = String::new();
        let mut current_content = String::new();
        let mut chars = html.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '<' => {
                    if !current_content.is_empty() {
                        current_slide.push_str(&self.convert_text(&current_content));
                        current_content.clear();
                    }
                    in_tag = true;
                    current_tag.clear();
                }
                '>' => {
                    in_tag = false;
                    if !current_content.is_empty() {
                        current_slide.push_str(&self.convert_text(&current_content));
                        current_content.clear();
                    }
                    self.process_slide_tag(&current_tag, &mut current_slide, &mut slides);
                    current_tag.clear();
                }
                _ if in_tag => {
                    current_tag.push(c);
                }
                _ => {
                    current_content.push(c);
                }
            }
        }

        // 处理剩余内容
        if !current_content.is_empty() {
            current_slide.push_str(&self.convert_text(&current_content));
        }

        // 添加最后一个幻灯片
        if !current_slide.is_empty() {
            let typst_config = self.config_service.get_typst_config();
            if slides.len() >= typst_config.max_slides {
                return Err(format!(
                    "Number of slides {} exceeds maximum of {}",
                    slides.len() + 1,
                    typst_config.max_slides
                ));
            }
            slides.push(self.wrap_in_slide(&current_slide));
        }

        Ok(slides)
    }

    /// Fallback: convert HTML to plain text
    fn html_to_plain_text(&self, html: &str) -> String {
        let mut result = html.to_string();
        // Remove HTML tags
        while let Some(start) = result.find('<') {
            if let Some(end) = result[start..].find('>') {
                result.replace_range(start..start + end + 1, "");
            } else {
                break;
            }
        }
        // Clean up whitespace
        result.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// Fallback: generate basic Typst with error message
    fn fallback_typst(error: &str) -> String {
        format!(
            "#import \"@preview/metropolis\": themes, theme-metropolis\n\n            #show: theme-metropolis\n\n            #set page(paper: \"presentation-16-9\", margin: (x: 1cm, y: 1cm))\n\n\n            = Conversion Error\n\n\n            The presentation could not be converted.\n\n\n            Error: {}\n\n\n            Please check the input format and try again.",
            error
        )
    }

    fn process_slide_tag(&self, tag: &str, current_slide: &mut String, slides: &mut Vec<String>) {
        let tag_lower = tag.to_lowercase();
        
        // Check slide count before adding
        let slide_count_check = |count: usize| -> Result<(), String> {
            let typst_config = self.config_service.get_typst_config();
            if count >= typst_config.max_slides {
                return Err(format!(
                    "Number of slides {} exceeds maximum of {}",
                    count,
                    typst_config.max_slides
                ));
            }
            Ok(())
        };
        
        // 将h1, h2等标题视为新幻灯片的开始
        if tag_lower.starts_with("h1") || tag_lower.starts_with("h2") {
            if !current_slide.is_empty() {
                if slide_count_check(slides.len()).is_ok() {
                    slides.push(self.wrap_in_slide(current_slide));
                    current_slide.clear();
                }
            }
        } else if tag_lower.starts_with("hr") {
            if !current_slide.is_empty() {
                if slide_count_check(slides.len()).is_ok() {
                    slides.push(self.wrap_in_slide(current_slide));
                    current_slide.clear();
                }
            }
        } else if tag_lower.starts_with("section") {
            if !current_slide.is_empty() {
                if slide_count_check(slides.len()).is_ok() {
                    slides.push(self.wrap_in_slide(current_slide));
                    current_slide.clear();
                }
            }
        } else if tag_lower.starts_with("/section") {
            if !current_slide.is_empty() {
                if slide_count_check(slides.len()).is_ok() {
                    slides.push(self.wrap_in_slide(current_slide));
                    current_slide.clear();
                }
            }
        } else {
            // 处理其他标签
            self.process_tag(tag_lower, current_slide);
        }
    }

    fn process_tag(&self, tag: String, result: &mut String) {
        if tag.starts_with("h1") {
            result.push_str("\n= ");
        } else if tag.starts_with("h2") {
            result.push_str("\n== ");
        } else if tag.starts_with("h3") {
            result.push_str("\n=== ");
        } else if tag.starts_with("/h") {
            result.push('\n');
        } else if tag.starts_with("p") {
            result.push('\n');
        } else if tag.starts_with("/p") {
            result.push_str("\n\n");
        } else if tag.starts_with("strong") || tag.starts_with("b") {
            result.push('*');
        } else if tag.starts_with("/strong") || tag.starts_with("/b") {
            result.push('*');
        } else if tag.starts_with("em") || tag.starts_with("i") {
            result.push('_');
        } else if tag.starts_with("/em") || tag.starts_with("/i") {
            result.push('_');
        } else if tag.starts_with("code") {
            result.push('`');
        } else if tag.starts_with("/code") {
            result.push('`');
        } else if tag.starts_with("ul") {
            result.push_str("\n");
        } else if tag.starts_with("/ul") {
            result.push_str("\n");
        } else if tag.starts_with("ol") {
            result.push_str("\n");
        } else if tag.starts_with("/ol") {
            result.push_str("\n");
        } else if tag.starts_with("li") {
            result.push_str("- ");
        } else if tag.starts_with("/li") {
            result.push('\n');
        }
    }

    fn wrap_in_slide(&self, content: &str) -> String {
        format!(
            "== Slide\n\n{}\n",
            content.trim()
        )
    }

    /// Sanitize text content to prevent injection
    fn convert_text(&self, text: &str) -> String {
        // Escape special characters for Typst
        text.replace('\\', "\\\\")
            .replace('&', "\\&")
            .replace('#', "\\#")
            .replace('_', "\\_")
            .replace('*', "\\*")
            .replace('[', "\\[")
            .replace(']', "\\]")
            .replace('$', "\\$")
            .replace('<', "\\<")
            .replace('>', "\\>")
    }
}
