/*!
 * 航空航天级文本格式化系统
 * 实现 Typst 的文本样式功能（字体大小、粗细、样式、颜色、间距）
 */

use serde::{Deserialize, Serialize};

/// 字体粗细
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
    Custom(f64),
}

/// 字体样式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

/// 文本装饰
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TextDecoration {
    None,
    Underline,
    LineThrough,
    Overline,
}

/// 文本样式配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextStyle {
    pub font_size: Option<f64>,
    pub font_weight: Option<FontWeight>,
    pub font_style: Option<FontStyle>,
    pub color: Option<String>,
    pub letter_spacing: Option<f64>,
    pub line_height: Option<f64>,
    pub decoration: Option<TextDecoration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextFormatter {
    pub enable_small_caps: bool,
    pub enable_smart_quotes: bool,
    pub quote_style: QuoteStyle,
    pub style: TextStyle,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum QuoteStyle {
    English, // "..."
    French,  // «...»
    German,  // „...“
    Swedish, // ”...”
}

impl Default for TextFormatter {
    fn default() -> Self {
        Self {
            enable_small_caps: true,
            enable_smart_quotes: true,
            quote_style: QuoteStyle::English,
            style: TextStyle::default(),
        }
    }
}

impl TextFormatter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_small_caps(mut self, enabled: bool) -> Self {
        self.enable_small_caps = enabled;
        self
    }

    pub fn with_smart_quotes(mut self, enabled: bool) -> Self {
        self.enable_smart_quotes = enabled;
        self
    }

    pub fn with_quote_style(mut self, style: QuoteStyle) -> Self {
        self.quote_style = style;
        self
    }

    pub fn with_font_size(mut self, size: f64) -> Self {
        self.style.font_size = Some(size);
        self
    }

    pub fn with_font_weight(mut self, weight: FontWeight) -> Self {
        self.style.font_weight = Some(weight);
        self
    }

    pub fn with_font_style(mut self, style: FontStyle) -> Self {
        self.style.font_style = Some(style);
        self
    }

    pub fn with_color(mut self, color: String) -> Self {
        self.style.color = Some(color);
        self
    }

    pub fn with_letter_spacing(mut self, spacing: f64) -> Self {
        self.style.letter_spacing = Some(spacing);
        self
    }

    pub fn with_line_height(mut self, height: f64) -> Self {
        self.style.line_height = Some(height);
        self
    }

    pub fn with_decoration(mut self, decoration: TextDecoration) -> Self {
        self.style.decoration = Some(decoration);
        self
    }

    /// 转换为小型大写字母
    pub fn to_small_caps(&self, text: &str) -> String {
        if !self.enable_small_caps {
            return text.to_string();
        }

        let mut result = String::new();
        let mut capitalize_next = true;

        for c in text.chars() {
            if c.is_whitespace() || c == '-' {
                capitalize_next = true;
                result.push(c);
            } else if capitalize_next {
                result.push(c.to_uppercase().collect::<Vec<_>>()[0]);
                capitalize_next = false;
            } else {
                // 小型大写字母：大写但字号较小
                result.push(c.to_uppercase().collect::<Vec<_>>()[0]);
            }
        }

        result
    }

    /// 转换为小型大写字母（HTML 格式）
    pub fn to_small_caps_html(&self, text: &str) -> String {
        if !self.enable_small_caps {
            return text.to_string();
        }

        let mut result = String::new();
        let mut in_word = false;
        let mut word_buffer = String::new();

        for c in text.chars() {
            if c.is_alphabetic() {
                word_buffer.push(c);
                in_word = true;
            } else {
                if in_word {
                    result.push_str(&self.format_word_small_caps(&word_buffer));
                    word_buffer.clear();
                    in_word = false;
                }
                result.push(c);
            }
        }

        if in_word {
            result.push_str(&self.format_word_small_caps(&word_buffer));
        }

        result
    }

    fn format_word_small_caps(&self, word: &str) -> String {
        if word.len() <= 1 {
            return word.to_string();
        }

        let first_char = word.chars().next().unwrap();
        let rest: String = word.chars().skip(1).collect();

        format!(
            "{}<span class=\"small-caps\">{}</span>",
            first_char.to_uppercase(),
            rest.to_uppercase()
        )
    }

    /// 转换为智能引号
    pub fn to_smart_quotes(&self, text: &str) -> String {
        if !self.enable_smart_quotes {
            return text.to_string();
        }

        let mut result = String::new();
        let mut chars = text.chars().peekable();
        let mut in_double_quote = false;
        let mut in_single_quote = false;

        while let Some(c) = chars.next() {
            match c {
                '"' => {
                    in_double_quote = !in_double_quote;
                    if in_double_quote {
                        result.push_str(self.get_opening_double_quote());
                    } else {
                        result.push_str(self.get_closing_double_quote());
                    }
                }
                '\'' => {
                    // 检查是否是撇号（如 "don't"）
                    if let Some(&next_char) = chars.peek() {
                        if next_char.is_alphabetic() {
                            // 撇号
                            result.push('\'');
                        } else {
                            in_single_quote = !in_single_quote;
                            if in_single_quote {
                                result.push_str(self.get_opening_single_quote());
                            } else {
                                result.push_str(self.get_closing_single_quote());
                            }
                        }
                    } else {
                        in_single_quote = !in_single_quote;
                        if in_single_quote {
                            result.push_str(self.get_opening_single_quote());
                        } else {
                            result.push_str(self.get_closing_single_quote());
                        }
                    }
                }
                _ => {
                    result.push(c);
                }
            }
        }

        result
    }

    fn get_opening_double_quote(&self) -> &str {
        match self.quote_style {
            QuoteStyle::English => "“",
            QuoteStyle::French => "«",
            QuoteStyle::German => "„",
            QuoteStyle::Swedish => "”",
        }
    }

    fn get_closing_double_quote(&self) -> &str {
        match self.quote_style {
            QuoteStyle::English => "”",
            QuoteStyle::French => "»",
            QuoteStyle::German => "“",
            QuoteStyle::Swedish => "”",
        }
    }

    fn get_opening_single_quote(&self) -> &str {
        match self.quote_style {
            QuoteStyle::English => "‘",
            QuoteStyle::French => "‹",
            QuoteStyle::German => "‚",
            QuoteStyle::Swedish => "’",
        }
    }

    fn get_closing_single_quote(&self) -> &str {
        match self.quote_style {
            QuoteStyle::English => "’",
            QuoteStyle::French => "›",
            QuoteStyle::German => "‘",
            QuoteStyle::Swedish => "’",
        }
    }

    /// 应用所有文本格式化
    pub fn format(&self, text: &str) -> String {
        let mut result = text.to_string();

        if self.enable_smart_quotes {
            result = self.to_smart_quotes(&result);
        }

        if self.enable_small_caps {
            result = self.to_small_caps(&result);
        }

        result
    }

    /// 转换为 Typst 格式
    pub fn to_typst(&self, text: &str) -> String {
        let mut result = text.to_string();

        // 应用字体大小
        if let Some(size) = self.style.font_size {
            result = format!("#text(size: {}pt)[{}]", size, result);
        }

        // 应用字体粗细
        if let Some(weight) = &self.style.font_weight {
            result = format!(
                "#text(weight: \"{}\")[{}]",
                self.font_weight_to_typst(weight),
                result
            );
        }

        // 应用字体样式
        if let Some(style) = &self.style.font_style {
            result = format!(
                "#text(style: \"{}\")[{}]",
                self.font_style_to_typst(style),
                result
            );
        }

        // 应用颜色
        if let Some(color) = &self.style.color {
            result = format!("#text(fill: \"{}\")[{}]", color, result);
        }

        // 应用字间距
        if let Some(spacing) = self.style.letter_spacing {
            result = format!("#text(tracking: {}em)[{}]", spacing, result);
        }

        // 应用行高
        if let Some(height) = self.style.line_height {
            result = format!("#text(leading: {}em)[{}]", height, result);
        }

        // 应用装饰
        if let Some(decoration) = &self.style.decoration {
            result = format!(
                "#text(deco: \"{}\")[{}]",
                self.decoration_to_typst(decoration),
                result
            );
        }

        if self.enable_small_caps {
            result = format!("#smallcaps[{}]", result);
        }

        if self.enable_smart_quotes {
            result = format!("#smartquote[{}]", result);
        }

        result
    }

    fn font_weight_to_typst(&self, weight: &FontWeight) -> String {
        match weight {
            FontWeight::Thin => "thin".to_string(),
            FontWeight::ExtraLight => "extralight".to_string(),
            FontWeight::Light => "light".to_string(),
            FontWeight::Normal => "normal".to_string(),
            FontWeight::Medium => "medium".to_string(),
            FontWeight::SemiBold => "semibold".to_string(),
            FontWeight::Bold => "bold".to_string(),
            FontWeight::ExtraBold => "extrabold".to_string(),
            FontWeight::Black => "black".to_string(),
            FontWeight::Custom(value) => format!("{}", value),
        }
    }

    fn font_style_to_typst(&self, style: &FontStyle) -> String {
        match style {
            FontStyle::Normal => "normal".to_string(),
            FontStyle::Italic => "italic".to_string(),
            FontStyle::Oblique => "oblique".to_string(),
        }
    }

    fn decoration_to_typst(&self, decoration: &TextDecoration) -> String {
        match decoration {
            TextDecoration::None => "none".to_string(),
            TextDecoration::Underline => "underline".to_string(),
            TextDecoration::LineThrough => "line-through".to_string(),
            TextDecoration::Overline => "overline".to_string(),
        }
    }

    /// 转换为 HTML 格式
    pub fn to_html(&self, text: &str) -> String {
        let mut result = String::new();

        let mut style_attrs = Vec::new();

        if let Some(size) = self.style.font_size {
            style_attrs.push(format!("font-size: {}pt", size));
        }

        if let Some(weight) = &self.style.font_weight {
            style_attrs.push(format!("font-weight: {}", self.font_weight_to_css(weight)));
        }

        if let Some(style) = &self.style.font_style {
            style_attrs.push(format!("font-style: {}", self.font_style_to_css(style)));
        }

        if let Some(color) = &self.style.color {
            style_attrs.push(format!("color: {}", color));
        }

        if let Some(spacing) = self.style.letter_spacing {
            style_attrs.push(format!("letter-spacing: {}em", spacing));
        }

        if let Some(height) = self.style.line_height {
            style_attrs.push(format!("line-height: {}", height));
        }

        if let Some(decoration) = &self.style.decoration {
            style_attrs.push(format!(
                "text-decoration: {}",
                self.decoration_to_css(decoration)
            ));
        }

        let style_attr = if style_attrs.is_empty() {
            String::new()
        } else {
            format!(" style=\"{}\"", style_attrs.join("; "))
        };

        result.push_str(&format!("<span{}>{}</span>", style_attr, html_escape(text)));

        result
    }

    fn font_weight_to_css(&self, weight: &FontWeight) -> String {
        match weight {
            FontWeight::Thin => "100".to_string(),
            FontWeight::ExtraLight => "200".to_string(),
            FontWeight::Light => "300".to_string(),
            FontWeight::Normal => "400".to_string(),
            FontWeight::Medium => "500".to_string(),
            FontWeight::SemiBold => "600".to_string(),
            FontWeight::Bold => "700".to_string(),
            FontWeight::ExtraBold => "800".to_string(),
            FontWeight::Black => "900".to_string(),
            FontWeight::Custom(value) => format!("{}", value),
        }
    }

    fn font_style_to_css(&self, style: &FontStyle) -> String {
        match style {
            FontStyle::Normal => "normal".to_string(),
            FontStyle::Italic => "italic".to_string(),
            FontStyle::Oblique => "oblique".to_string(),
        }
    }

    fn decoration_to_css(&self, decoration: &TextDecoration) -> String {
        match decoration {
            TextDecoration::None => "none".to_string(),
            TextDecoration::Underline => "underline".to_string(),
            TextDecoration::LineThrough => "line-through".to_string(),
            TextDecoration::Overline => "overline".to_string(),
        }
    }

    /// 检测文本中的引号样式
    pub fn detect_quote_style(&self, text: &str) -> Option<QuoteStyle> {
        if text.contains("«") && text.contains("»") {
            Some(QuoteStyle::French)
        } else if text.contains("„") && text.contains("“") {
            Some(QuoteStyle::German)
        } else if text.contains("”") && !text.contains("“") {
            Some(QuoteStyle::Swedish)
        } else if text.contains("“") || text.contains("”") {
            Some(QuoteStyle::English)
        } else {
            None
        }
    }
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextTransform {
    pub uppercase: bool,
    pub lowercase: bool,
    pub title_case: bool,
    pub sentence_case: bool,
}

impl TextTransform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn uppercase(mut self) -> Self {
        self.uppercase = true;
        self
    }

    pub fn lowercase(mut self) -> Self {
        self.lowercase = true;
        self
    }

    pub fn title_case(mut self) -> Self {
        self.title_case = true;
        self
    }

    pub fn sentence_case(mut self) -> Self {
        self.sentence_case = true;
        self
    }

    pub fn apply(&self, text: &str) -> String {
        if self.uppercase {
            return text.to_uppercase();
        }

        if self.lowercase {
            return text.to_lowercase();
        }

        if self.title_case {
            return self.to_title_case(text);
        }

        if self.sentence_case {
            return self.to_sentence_case(text);
        }

        text.to_string()
    }

    fn to_title_case(&self, text: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;

        for c in text.chars() {
            if c.is_whitespace() || c == '-' {
                capitalize_next = true;
                result.push(c);
            } else if capitalize_next {
                result.extend(c.to_uppercase());
                capitalize_next = false;
            } else {
                result.extend(c.to_lowercase());
            }
        }

        result
    }

    fn to_sentence_case(&self, text: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;

        for c in text.chars() {
            if c == '.' || c == '!' || c == '?' {
                capitalize_next = true;
                result.push(c);
            } else if c.is_whitespace() {
                result.push(c);
            } else if capitalize_next {
                result.extend(c.to_uppercase());
                capitalize_next = false;
            } else {
                result.extend(c.to_lowercase());
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_formatter_creation() {
        let formatter = TextFormatter::new();
        assert!(formatter.enable_small_caps);
        assert!(formatter.enable_smart_quotes);
    }

    #[test]
    fn test_to_small_caps() {
        let formatter = TextFormatter::new();
        let result = formatter.to_small_caps("hello world");
        assert_eq!(result, "HELLO WORLD");
    }

    #[test]
    fn test_to_small_caps_html() {
        let formatter = TextFormatter::new();
        let result = formatter.to_small_caps_html("hello");
        assert!(result.contains("small-caps"));
    }

    #[test]
    fn test_to_smart_quotes_english() {
        let formatter = TextFormatter::new().with_quote_style(QuoteStyle::English);
        let result = formatter.to_smart_quotes("hello \"world\"");
        assert!(result.contains("“"));
        assert!(result.contains("”"));
    }

    #[test]
    fn test_to_smart_quotes_french() {
        let formatter = TextFormatter::new().with_quote_style(QuoteStyle::French);
        let result = formatter.to_smart_quotes("hello \"world\"");
        assert!(result.contains("«"));
        assert!(result.contains("»"));
    }

    #[test]
    fn test_apostrophe_handling() {
        let formatter = TextFormatter::new();
        let result = formatter.to_smart_quotes("don't");
        assert!(result.contains("'"));
    }

    #[test]
    fn test_format() {
        let formatter = TextFormatter::new();
        let result = formatter.format("hello \"world\"");
        assert!(result.contains("HELLO"));
    }

    #[test]
    fn test_to_typst() {
        let formatter = TextFormatter::new();
        let result = formatter.to_typst("hello");
        assert!(result.contains("#smallcaps"));
    }

    #[test]
    fn test_detect_quote_style() {
        let formatter = TextFormatter::new();
        assert_eq!(
            formatter.detect_quote_style("«test»"),
            Some(QuoteStyle::French)
        );
        assert_eq!(
            formatter.detect_quote_style("„test“"),
            Some(QuoteStyle::German)
        );
        assert_eq!(
            formatter.detect_quote_style("“test”"),
            Some(QuoteStyle::English)
        );
    }

    #[test]
    fn test_text_transform_uppercase() {
        let transform = TextTransform::new().uppercase();
        let result = transform.apply("hello");
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_text_transform_lowercase() {
        let transform = TextTransform::new().lowercase();
        let result = transform.apply("HELLO");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_text_transform_title_case() {
        let transform = TextTransform::new().title_case();
        let result = transform.apply("hello world");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_text_transform_sentence_case() {
        let transform = TextTransform::new().sentence_case();
        let result = transform.apply("HELLO WORLD. TEST");
        assert_eq!(result, "Hello world. Test");
    }

    #[test]
    fn test_quote_style_variations() {
        let formatter = TextFormatter::new().with_quote_style(QuoteStyle::German);
        let result = formatter.to_smart_quotes("test");
        // Should not convert if no quotes present
        assert_eq!(result, "test");
    }

    #[test]
    fn test_text_style_default() {
        let style = TextStyle::default();
        assert!(style.font_size.is_none());
        assert!(style.font_weight.is_none());
    }

    #[test]
    fn test_font_weight_variants() {
        assert_eq!(FontWeight::Normal, FontWeight::Normal);
        assert_eq!(FontWeight::Bold, FontWeight::Bold);
        assert_ne!(FontWeight::Normal, FontWeight::Bold);
    }

    #[test]
    fn test_font_style_variants() {
        assert_eq!(FontStyle::Normal, FontStyle::Normal);
        assert_eq!(FontStyle::Italic, FontStyle::Italic);
        assert_ne!(FontStyle::Normal, FontStyle::Italic);
    }

    #[test]
    fn test_text_decoration_variants() {
        assert_eq!(TextDecoration::None, TextDecoration::None);
        assert_eq!(TextDecoration::Underline, TextDecoration::Underline);
        assert_ne!(TextDecoration::None, TextDecoration::Underline);
    }

    #[test]
    fn test_text_formatter_with_font_size() {
        let formatter = TextFormatter::new().with_font_size(12.0);
        assert_eq!(formatter.style.font_size, Some(12.0));
    }

    #[test]
    fn test_text_formatter_with_font_weight() {
        let formatter = TextFormatter::new().with_font_weight(FontWeight::Bold);
        assert_eq!(formatter.style.font_weight, Some(FontWeight::Bold));
    }

    #[test]
    fn test_text_formatter_with_font_style() {
        let formatter = TextFormatter::new().with_font_style(FontStyle::Italic);
        assert_eq!(formatter.style.font_style, Some(FontStyle::Italic));
    }

    #[test]
    fn test_text_formatter_with_color() {
        let formatter = TextFormatter::new().with_color("red".to_string());
        assert_eq!(formatter.style.color, Some("red".to_string()));
    }

    #[test]
    fn test_text_formatter_with_letter_spacing() {
        let formatter = TextFormatter::new().with_letter_spacing(0.5);
        assert_eq!(formatter.style.letter_spacing, Some(0.5));
    }

    #[test]
    fn test_text_formatter_with_line_height() {
        let formatter = TextFormatter::new().with_line_height(1.5);
        assert_eq!(formatter.style.line_height, Some(1.5));
    }

    #[test]
    fn test_text_formatter_with_decoration() {
        let formatter = TextFormatter::new().with_decoration(TextDecoration::Underline);
        assert_eq!(formatter.style.decoration, Some(TextDecoration::Underline));
    }

    #[test]
    fn test_to_typst_with_font_size() {
        let formatter = TextFormatter::new().with_font_size(12.0);
        let result = formatter.to_typst("test");
        assert!(result.contains("size: 12pt"));
    }

    #[test]
    fn test_to_typst_with_font_weight() {
        let formatter = TextFormatter::new().with_font_weight(FontWeight::Bold);
        let result = formatter.to_typst("test");
        assert!(result.contains("weight: \"bold\""));
    }

    #[test]
    fn test_to_typst_with_font_style() {
        let formatter = TextFormatter::new().with_font_style(FontStyle::Italic);
        let result = formatter.to_typst("test");
        assert!(result.contains("style: \"italic\""));
    }

    #[test]
    fn test_to_typst_with_color() {
        let formatter = TextFormatter::new().with_color("red".to_string());
        let result = formatter.to_typst("test");
        assert!(result.contains("fill: \"red\""));
    }

    #[test]
    fn test_to_html_with_font_size() {
        let formatter = TextFormatter::new().with_font_size(12.0);
        let result = formatter.to_html("test");
        assert!(result.contains("font-size: 12pt"));
    }

    #[test]
    fn test_to_html_with_font_weight() {
        let formatter = TextFormatter::new().with_font_weight(FontWeight::Bold);
        let result = formatter.to_html("test");
        assert!(result.contains("font-weight: 700"));
    }

    #[test]
    fn test_to_html_with_font_style() {
        let formatter = TextFormatter::new().with_font_style(FontStyle::Italic);
        let result = formatter.to_html("test");
        assert!(result.contains("font-style: italic"));
    }

    #[test]
    fn test_to_html_with_color() {
        let formatter = TextFormatter::new().with_color("red".to_string());
        let result = formatter.to_html("test");
        assert!(result.contains("color: red"));
    }

    #[test]
    fn test_font_weight_to_typst() {
        let formatter = TextFormatter::new();
        assert_eq!(formatter.font_weight_to_typst(&FontWeight::Bold), "bold");
        assert_eq!(
            formatter.font_weight_to_typst(&FontWeight::Normal),
            "normal"
        );
    }

    #[test]
    fn test_font_style_to_typst() {
        let formatter = TextFormatter::new();
        assert_eq!(formatter.font_style_to_typst(&FontStyle::Italic), "italic");
        assert_eq!(formatter.font_style_to_typst(&FontStyle::Normal), "normal");
    }

    #[test]
    fn test_decoration_to_typst() {
        let formatter = TextFormatter::new();
        assert_eq!(
            formatter.decoration_to_typst(&TextDecoration::Underline),
            "underline"
        );
        assert_eq!(formatter.decoration_to_typst(&TextDecoration::None), "none");
    }

    #[test]
    fn test_font_weight_to_css() {
        let formatter = TextFormatter::new();
        assert_eq!(formatter.font_weight_to_css(&FontWeight::Bold), "700");
        assert_eq!(formatter.font_weight_to_css(&FontWeight::Normal), "400");
    }

    #[test]
    fn test_font_style_to_css() {
        let formatter = TextFormatter::new();
        assert_eq!(formatter.font_style_to_css(&FontStyle::Italic), "italic");
        assert_eq!(formatter.font_style_to_css(&FontStyle::Normal), "normal");
    }

    #[test]
    fn test_decoration_to_css() {
        let formatter = TextFormatter::new();
        assert_eq!(
            formatter.decoration_to_css(&TextDecoration::Underline),
            "underline"
        );
        assert_eq!(formatter.decoration_to_css(&TextDecoration::None), "none");
    }
}
