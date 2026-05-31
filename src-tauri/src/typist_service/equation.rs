/*!
 * 航空航天级方程系统
 * 实现 Typst 的方程功能（数学方程、编号、对齐、补充文本）
 */

use serde::{Deserialize, Serialize};

/// 方程对齐方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EquationAlign {
    Left,
    Center,
    Right,
}

/// 方程配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquationConfig {
    pub block: bool,
    pub numbering: Option<String>,
    pub number_align: EquationAlign,
    pub supplement: Option<String>,
    pub alt: Option<String>,
}

impl Default for EquationConfig {
    fn default() -> Self {
        Self {
            block: false,
            numbering: None,
            number_align: EquationAlign::Right,
            supplement: None,
            alt: None,
        }
    }
}

/// 方程
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equation {
    pub content: String,
    pub config: EquationConfig,
    pub label: Option<String>,
}

impl Equation {
    pub fn new(content: String) -> Self {
        Self {
            content,
            config: EquationConfig::default(),
            label: None,
        }
    }

    pub fn with_config(mut self, config: EquationConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_block(mut self, block: bool) -> Self {
        self.config.block = block;
        self
    }

    pub fn with_numbering(mut self, numbering: String) -> Self {
        self.config.numbering = Some(numbering);
        self
    }

    pub fn with_number_align(mut self, align: EquationAlign) -> Self {
        self.config.number_align = align;
        self
    }

    pub fn with_supplement(mut self, supplement: String) -> Self {
        self.config.supplement = Some(supplement);
        self
    }

    pub fn with_alt(mut self, alt: String) -> Self {
        self.config.alt = Some(alt);
        self
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        typst.push_str("#equation(");

        // 添加标签
        if let Some(label) = &self.label {
            typst.push_str(&format!("<{}> ", label));
        }

        // 添加内容
        typst.push_str(&format!("${}$", html_escape(&self.content)));

        // 添加块级
        if self.config.block {
            typst.push_str(", block: true");
        }

        // 添加编号
        if let Some(numbering) = &self.config.numbering {
            typst.push_str(&format!(", numbering: \"{}\"", numbering));
        }

        // 添加编号对齐
        if !matches!(self.config.number_align, EquationAlign::Right) {
            typst.push_str(&format!(", number-align: {}", self.align_to_typst()));
        }

        // 添加补充文本
        if let Some(supplement) = &self.config.supplement {
            typst.push_str(&format!(", supplement: [{}]", html_escape(supplement)));
        }

        // 添加 Alt 文本
        if let Some(alt) = &self.config.alt {
            typst.push_str(&format!(", alt: \"{}\"", html_escape(alt)));
        }

        typst.push_str(")\n");

        typst
    }

    fn align_to_typst(&self) -> String {
        match self.config.number_align {
            EquationAlign::Left => "left".to_string(),
            EquationAlign::Center => "center".to_string(),
            EquationAlign::Right => "right".to_string(),
        }
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        let id_attr = if let Some(label) = &self.label {
            format!(" id=\"{}\"", label)
        } else {
            String::new()
        };

        let block_attr = if self.config.block {
            " data-block=\"true\""
        } else {
            " data-block=\"false\""
        };

        let numbering_attr = if self.config.numbering.is_some() {
            " data-numbered=\"true\""
        } else {
            " data-numbered=\"false\""
        };

        let align_attr = format!(" data-align=\"{}\"", self.align_to_typst());

        if self.config.block {
            html.push_str(&format!(
                "<div class=\"typst-equation\"{}{}{}{}>\n",
                id_attr, block_attr, numbering_attr, align_attr
            ));
            html.push_str(&format!(
                "  <div class=\"equation-content\">${}$</div>\n",
                html_escape(&self.content)
            ));

            if let Some(numbering) = &self.config.numbering {
                html.push_str(&format!(
                    "  <div class=\"equation-number\">({})</div>\n",
                    numbering
                ));
            }

            html.push_str("</div>\n");
        } else {
            html.push_str(&format!(
                "<span class=\"typst-equation-inline\"{}{}{}{}>${}$</span>",
                id_attr,
                block_attr,
                numbering_attr,
                align_attr,
                html_escape(&self.content)
            ));
        }

        html
    }
}

impl Default for Equation {
    fn default() -> Self {
        Self::new("".to_string())
    }
}

/// 方程构建器
pub struct EquationBuilder {
    equation: Equation,
}

impl EquationBuilder {
    pub fn new(content: String) -> Self {
        Self {
            equation: Equation::new(content),
        }
    }

    pub fn block(mut self, block: bool) -> Self {
        self.equation = self.equation.with_block(block);
        self
    }

    pub fn numbering(mut self, numbering: String) -> Self {
        self.equation = self.equation.with_numbering(numbering);
        self
    }

    pub fn number_align(mut self, align: EquationAlign) -> Self {
        self.equation = self.equation.with_number_align(align);
        self
    }

    pub fn supplement(mut self, supplement: String) -> Self {
        self.equation = self.equation.with_supplement(supplement);
        self
    }

    pub fn alt(mut self, alt: String) -> Self {
        self.equation = self.equation.with_alt(alt);
        self
    }

    pub fn label(mut self, label: String) -> Self {
        self.equation = self.equation.with_label(label);
        self
    }

    pub fn build(self) -> Equation {
        self.equation
    }
}

impl Default for EquationBuilder {
    fn default() -> Self {
        Self::new("".to_string())
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
    fn test_equation_creation() {
        let equation = Equation::new("x^2 + y^2 = z^2".to_string());
        assert_eq!(equation.content, "x^2 + y^2 = z^2");
    }

    #[test]
    fn test_equation_default() {
        let equation = Equation::default();
        assert_eq!(equation.content, "");
    }

    #[test]
    fn test_equation_config_default() {
        let config = EquationConfig::default();
        assert!(!config.block);
        assert_eq!(config.number_align, EquationAlign::Right);
    }

    #[test]
    fn test_equation_with_block() {
        let equation = Equation::new("x^2".to_string()).with_block(true);
        assert!(equation.config.block);
    }

    #[test]
    fn test_equation_with_numbering() {
        let equation = Equation::new("x^2".to_string()).with_numbering("(1)".to_string());
        assert_eq!(equation.config.numbering, Some("(1)".to_string()));
    }

    #[test]
    fn test_equation_with_number_align() {
        let equation = Equation::new("x^2".to_string()).with_number_align(EquationAlign::Center);
        assert_eq!(equation.config.number_align, EquationAlign::Center);
    }

    #[test]
    fn test_equation_with_supplement() {
        let equation = Equation::new("x^2".to_string()).with_supplement("Eq".to_string());
        assert_eq!(equation.config.supplement, Some("Eq".to_string()));
    }

    #[test]
    fn test_equation_with_alt() {
        let equation = Equation::new("x^2".to_string()).with_alt("Pythagorean theorem".to_string());
        assert_eq!(equation.config.alt, Some("Pythagorean theorem".to_string()));
    }

    #[test]
    fn test_equation_with_label() {
        let equation = Equation::new("x^2".to_string()).with_label("eq1".to_string());
        assert_eq!(equation.label, Some("eq1".to_string()));
    }

    #[test]
    fn test_to_typst() {
        let equation = Equation::new("x^2".to_string());
        let typst = equation.to_typst();
        assert!(typst.contains("#equation("));
        assert!(typst.contains("$x^2$"));
    }

    #[test]
    fn test_to_typst_with_block() {
        let equation = Equation::new("x^2".to_string()).with_block(true);
        let typst = equation.to_typst();
        assert!(typst.contains("block: true"));
    }

    #[test]
    fn test_to_typst_with_numbering() {
        let equation = Equation::new("x^2".to_string()).with_numbering("(1)".to_string());
        let typst = equation.to_typst();
        assert!(typst.contains("numbering: \"(1)\""));
    }

    #[test]
    fn test_to_typst_with_label() {
        let equation = Equation::new("x^2".to_string()).with_label("eq1".to_string());
        let typst = equation.to_typst();
        assert!(typst.contains("<eq1>"));
    }

    #[test]
    fn test_to_html_inline() {
        let equation = Equation::new("x^2".to_string());
        let html = equation.to_html();
        assert!(html.contains("<span class=\"typst-equation-inline\""));
        assert!(html.contains("$x^2$"));
    }

    #[test]
    fn test_to_html_block() {
        let equation = Equation::new("x^2".to_string()).with_block(true);
        let html = equation.to_html();
        assert!(html.contains("<div class=\"typst-equation\""));
        assert!(html.contains("data-block=\"true\""));
    }

    #[test]
    fn test_to_html_with_numbering() {
        let equation = Equation::new("x^2".to_string())
            .with_block(true)
            .with_numbering("(1)".to_string());
        let html = equation.to_html();
        assert!(html.contains("data-numbered=\"true\""));
        assert!(html.contains("equation-number"));
    }

    #[test]
    fn test_to_html_with_label() {
        let equation = Equation::new("x^2".to_string()).with_label("eq1".to_string());
        let html = equation.to_html();
        assert!(html.contains("id=\"eq1\""));
    }

    #[test]
    fn test_equation_builder() {
        let equation = EquationBuilder::new("x^2".to_string())
            .block(true)
            .numbering("(1)".to_string())
            .label("eq1".to_string())
            .build();

        assert_eq!(equation.content, "x^2");
        assert!(equation.config.block);
    }

    #[test]
    fn test_equation_builder_default() {
        let builder = EquationBuilder::default();
        let equation = builder.build();
        assert_eq!(equation.content, "");
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_align_to_typst() {
        let equation = Equation::new("x^2".to_string());
        assert_eq!(equation.align_to_typst(), "right");
    }

    #[test]
    fn test_align_variants() {
        assert_eq!(EquationAlign::Left, EquationAlign::Left);
        assert_eq!(EquationAlign::Center, EquationAlign::Center);
        assert_eq!(EquationAlign::Right, EquationAlign::Right);
    }

    #[test]
    fn test_to_typst_with_number_align() {
        let equation = Equation::new("x^2".to_string()).with_number_align(EquationAlign::Center);
        let typst = equation.to_typst();
        assert!(typst.contains("number-align: center"));
    }

    #[test]
    fn test_to_html_with_align() {
        let equation = Equation::new("x^2".to_string()).with_number_align(EquationAlign::Left);
        let html = equation.to_html();
        assert!(html.contains("data-align=\"left\""));
    }

    #[test]
    fn test_to_typst_with_supplement() {
        let equation = Equation::new("x^2".to_string()).with_supplement("Eq".to_string());
        let typst = equation.to_typst();
        assert!(typst.contains("supplement: [Eq]"));
    }

    #[test]
    fn test_to_typst_with_alt() {
        let equation = Equation::new("x^2".to_string()).with_alt("Alt text".to_string());
        let typst = equation.to_typst();
        assert!(typst.contains("alt: \"Alt text\""));
    }
}
