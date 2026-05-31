/*!
 * 航空航天级样式系统
 * 实现 Typst 的样式功能（样式规则、主题、颜色、字体）
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 样式规则类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StyleRuleType {
    Set,
    Show,
}

/// 样式选择器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StyleSelector {
    All,
    Element(String),
    Class(String),
    Id(String),
    Custom(String),
}

/// 样式属性
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StyleValue {
    String(String),
    Number(f64),
    Color(String),
    Boolean(bool),
    Array(Vec<StyleValue>),
}

/// 样式规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleRule {
    pub rule_type: StyleRuleType,
    pub selector: StyleSelector,
    pub properties: HashMap<String, StyleValue>,
}

impl StyleRule {
    pub fn new(rule_type: StyleRuleType, selector: StyleSelector) -> Self {
        Self {
            rule_type,
            selector,
            properties: HashMap::new(),
        }
    }

    pub fn with_property(mut self, key: String, value: StyleValue) -> Self {
        self.properties.insert(key, value);
        self
    }

    pub fn add_property(&mut self, key: String, value: StyleValue) {
        self.properties.insert(key, value);
    }
}

/// 主题配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub colors: HashMap<String, String>,
    pub fonts: HashMap<String, String>,
    pub spacing: HashMap<String, f64>,
}

impl Default for Theme {
    fn default() -> Self {
        let mut colors = HashMap::new();
        colors.insert("primary".to_string(), "#000000".to_string());
        colors.insert("secondary".to_string(), "#666666".to_string());
        colors.insert("accent".to_string(), "#0066cc".to_string());
        colors.insert("background".to_string(), "#ffffff".to_string());
        colors.insert("text".to_string(), "#000000".to_string());

        let mut fonts = HashMap::new();
        fonts.insert("serif".to_string(), "Times New Roman".to_string());
        fonts.insert("sans".to_string(), "Arial".to_string());
        fonts.insert("mono".to_string(), "Courier New".to_string());

        let mut spacing = HashMap::new();
        spacing.insert("paragraph".to_string(), 1.0);
        spacing.insert("heading".to_string(), 1.5);
        spacing.insert("section".to_string(), 2.0);

        Self {
            name: "default".to_string(),
            colors,
            fonts,
            spacing,
        }
    }
}

/// 样式系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Styling {
    pub theme: Theme,
    pub rules: Vec<StyleRule>,
}

impl Styling {
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
            rules: Vec::new(),
        }
    }

    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn with_rules(mut self, rules: Vec<StyleRule>) -> Self {
        self.rules = rules;
        self
    }

    pub fn add_rule(mut self, rule: StyleRule) -> Self {
        self.rules.push(rule);
        self
    }

    pub fn add_color(mut self, name: String, color: String) -> Self {
        self.theme.colors.insert(name, color);
        self
    }

    pub fn add_font(mut self, name: String, font: String) -> Self {
        self.theme.fonts.insert(name, font);
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        // 添加主题颜色
        for (name, color) in &self.theme.colors {
            typst.push_str(&format!("#let {} = \"{}\"\n", name, color));
        }

        // 添加主题字体
        for (name, font) in &self.theme.fonts {
            typst.push_str(&format!("#let {}-font = \"{}\"\n", name, font));
        }

        // 添加样式规则
        for rule in &self.rules {
            let rule_keyword = match rule.rule_type {
                StyleRuleType::Set => "set",
                StyleRuleType::Show => "show",
            };

            let selector = match &rule.selector {
                StyleSelector::All => "*".to_string(),
                StyleSelector::Element(el) => el.clone(),
                StyleSelector::Class(cls) => format!(".{}", cls),
                StyleSelector::Id(id) => format!("#{}", id),
                StyleSelector::Custom(custom) => custom.clone(),
            };

            typst.push_str(&format!("#{} {}(", rule_keyword, selector));

            for (key, value) in &rule.properties {
                typst.push_str(&format!("{}: {}, ", key, self.value_to_typst(value)));
            }

            // 移除最后的逗号和空格
            if typst.ends_with(", ") {
                typst.pop();
                typst.pop();
            }

            typst.push_str(")\n");
        }

        typst
    }

    fn value_to_typst(&self, value: &StyleValue) -> String {
        match value {
            StyleValue::String(s) => format!("\"{}\"", s),
            StyleValue::Number(n) => n.to_string(),
            StyleValue::Color(c) => format!("\"{}\"", c),
            StyleValue::Boolean(b) => b.to_string(),
            StyleValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| self.value_to_typst(v)).collect();
                format!("({})", items.join(", "))
            }
        }
    }

    /// 转换为 CSS
    pub fn to_css(&self) -> String {
        let mut css = String::new();

        // 添加 CSS 变量
        css.push_str(":root {\n");
        for (name, color) in &self.theme.colors {
            css.push_str(&format!("  --{}: {};\n", name, color));
        }
        for (name, font) in &self.theme.fonts {
            css.push_str(&format!("  --{}-font: {};\n", name, font));
        }
        css.push_str("}\n");

        // 添加样式规则
        for rule in &self.rules {
            let selector = match &rule.selector {
                StyleSelector::All => "*".to_string(),
                StyleSelector::Element(el) => el.clone(),
                StyleSelector::Class(cls) => format!(".{}", cls),
                StyleSelector::Id(id) => format!("#{}", id),
                StyleSelector::Custom(custom) => custom.clone(),
            };

            css.push_str(&format!("{} {{\n", selector));

            for (key, value) in &rule.properties {
                css.push_str(&format!("  {}: {};\n", key, self.value_to_css(value)));
            }

            css.push_str("}\n");
        }

        css
    }

    fn value_to_css(&self, value: &StyleValue) -> String {
        match value {
            StyleValue::String(s) => s.clone(),
            StyleValue::Number(n) => format!("{}px", n),
            StyleValue::Color(c) => c.clone(),
            StyleValue::Boolean(b) => b.to_string(),
            StyleValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| self.value_to_css(v)).collect();
                items.join(", ")
            }
        }
    }
}

impl Default for Styling {
    fn default() -> Self {
        Self::new()
    }
}

/// 样式构建器
pub struct StylingBuilder {
    styling: Styling,
}

impl StylingBuilder {
    pub fn new() -> Self {
        Self {
            styling: Styling::new(),
        }
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.styling = self.styling.with_theme(theme);
        self
    }

    pub fn rule(mut self, rule: StyleRule) -> Self {
        self.styling = self.styling.add_rule(rule);
        self
    }

    pub fn color(mut self, name: String, color: String) -> Self {
        self.styling = self.styling.add_color(name, color);
        self
    }

    pub fn font(mut self, name: String, font: String) -> Self {
        self.styling = self.styling.add_font(name, font);
        self
    }

    pub fn build(self) -> Styling {
        self.styling
    }
}

impl Default for StylingBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_styling_creation() {
        let styling = Styling::new();
        assert_eq!(styling.theme.name, "default");
    }

    #[test]
    fn test_styling_default() {
        let styling = Styling::default();
        assert_eq!(styling.theme.name, "default");
    }

    #[test]
    fn test_theme_default() {
        let theme = Theme::default();
        assert_eq!(theme.name, "default");
        assert!(theme.colors.contains_key("primary"));
    }

    #[test]
    fn test_style_rule_creation() {
        let rule = StyleRule::new(StyleRuleType::Set, StyleSelector::All);
        assert!(matches!(rule.rule_type, StyleRuleType::Set));
    }

    #[test]
    fn test_style_rule_with_property() {
        let rule = StyleRule::new(StyleRuleType::Set, StyleSelector::All)
            .with_property("font-size".to_string(), StyleValue::Number(12.0));
        assert!(rule.properties.contains_key("font-size"));
    }

    #[test]
    fn test_styling_with_theme() {
        let theme = Theme::default();
        let styling = Styling::new().with_theme(theme);
        assert_eq!(styling.theme.name, "default");
    }

    #[test]
    fn test_styling_add_rule() {
        let rule = StyleRule::new(StyleRuleType::Set, StyleSelector::All);
        let styling = Styling::new().add_rule(rule);
        assert_eq!(styling.rules.len(), 1);
    }

    #[test]
    fn test_styling_add_color() {
        let styling = Styling::new().add_color("custom".to_string(), "#ff0000".to_string());
        assert!(styling.theme.colors.contains_key("custom"));
    }

    #[test]
    fn test_styling_add_font() {
        let styling = Styling::new().add_font("custom".to_string(), "Custom Font".to_string());
        assert!(styling.theme.fonts.contains_key("custom"));
    }

    #[test]
    fn test_style_rule_type_variants() {
        assert!(matches!(StyleRuleType::Set, StyleRuleType::Set));
        assert!(matches!(StyleRuleType::Show, StyleRuleType::Show));
    }

    #[test]
    fn test_style_selector_variants() {
        assert!(matches!(StyleSelector::All, StyleSelector::All));
        assert!(matches!(
            StyleSelector::Element("test".to_string()),
            StyleSelector::Element(_)
        ));
    }

    #[test]
    fn test_style_value_variants() {
        let string_val = StyleValue::String("test".to_string());
        let number_val = StyleValue::Number(12.0);
        let color_val = StyleValue::Color("#ff0000".to_string());
        let bool_val = StyleValue::Boolean(true);
        let array_val = StyleValue::Array(vec![StyleValue::String("a".to_string())]);

        assert!(matches!(string_val, StyleValue::String(_)));
        assert!(matches!(number_val, StyleValue::Number(_)));
        assert!(matches!(color_val, StyleValue::Color(_)));
        assert!(matches!(bool_val, StyleValue::Boolean(_)));
        assert!(matches!(array_val, StyleValue::Array(_)));
    }

    #[test]
    fn test_to_typst() {
        let styling = Styling::new();
        let typst = styling.to_typst();
        assert!(typst.contains("#let primary"));
    }

    #[test]
    fn test_to_typst_with_rule() {
        let rule = StyleRule::new(
            StyleRuleType::Set,
            StyleSelector::Element("heading".to_string()),
        )
        .with_property("font-size".to_string(), StyleValue::Number(24.0));
        let styling = Styling::new().add_rule(rule);
        let typst = styling.to_typst();
        assert!(typst.contains("#set heading("));
    }

    #[test]
    fn test_to_css() {
        let styling = Styling::new();
        let css = styling.to_css();
        assert!(css.contains(":root"));
        assert!(css.contains("--primary"));
    }

    #[test]
    fn test_to_css_with_rule() {
        let rule = StyleRule::new(
            StyleRuleType::Set,
            StyleSelector::Element("heading".to_string()),
        )
        .with_property("font-size".to_string(), StyleValue::Number(24.0));
        let styling = Styling::new().add_rule(rule);
        let css = styling.to_css();
        assert!(css.contains("heading {"));
    }

    #[test]
    fn test_value_to_typst_string() {
        let styling = Styling::new();
        let val = StyleValue::String("test".to_string());
        assert_eq!(styling.value_to_typst(&val), "\"test\"");
    }

    #[test]
    fn test_value_to_typst_number() {
        let styling = Styling::new();
        let val = StyleValue::Number(12.0);
        assert_eq!(styling.value_to_typst(&val), "12");
    }

    #[test]
    fn test_value_to_css_string() {
        let styling = Styling::new();
        let val = StyleValue::String("test".to_string());
        assert_eq!(styling.value_to_css(&val), "test");
    }

    #[test]
    fn test_value_to_css_number() {
        let styling = Styling::new();
        let val = StyleValue::Number(12.0);
        assert_eq!(styling.value_to_css(&val), "12px");
    }

    #[test]
    fn test_styling_builder() {
        let rule = StyleRule::new(StyleRuleType::Set, StyleSelector::All);
        let styling = StylingBuilder::new()
            .rule(rule)
            .color("custom".to_string(), "#ff0000".to_string())
            .build();

        assert_eq!(styling.rules.len(), 1);
        assert!(styling.theme.colors.contains_key("custom"));
    }

    #[test]
    fn test_styling_builder_default() {
        let builder = StylingBuilder::default();
        let styling = builder.build();
        assert_eq!(styling.theme.name, "default");
    }

    #[test]
    fn test_theme_colors() {
        let theme = Theme::default();
        assert!(theme.colors.contains_key("primary"));
        assert!(theme.colors.contains_key("secondary"));
        assert!(theme.colors.contains_key("accent"));
    }

    #[test]
    fn test_theme_fonts() {
        let theme = Theme::default();
        assert!(theme.fonts.contains_key("serif"));
        assert!(theme.fonts.contains_key("sans"));
        assert!(theme.fonts.contains_key("mono"));
    }

    #[test]
    fn test_to_typst_show_rule() {
        let rule = StyleRule::new(
            StyleRuleType::Show,
            StyleSelector::Element("figure".to_string()),
        );
        let styling = Styling::new().add_rule(rule);
        let typst = styling.to_typst();
        assert!(typst.contains("#show figure("));
    }

    #[test]
    fn test_to_css_class_selector() {
        let rule = StyleRule::new(
            StyleRuleType::Set,
            StyleSelector::Class("highlight".to_string()),
        );
        let styling = Styling::new().add_rule(rule);
        let css = styling.to_css();
        assert!(css.contains(".highlight"));
    }

    #[test]
    fn test_to_css_id_selector() {
        let rule = StyleRule::new(StyleRuleType::Set, StyleSelector::Id("main".to_string()));
        let styling = Styling::new().add_rule(rule);
        let css = styling.to_css();
        assert!(css.contains("#main"));
    }
}
