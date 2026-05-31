/*!
 * 航空航天级定理系统
 * 实现 Typst 的定理功能（定理、引理、命题、推论、定义、示例、备注）
 */

use serde::{Deserialize, Serialize};

/// 定理类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TheoremType {
    Theorem,
    Lemma,
    Proposition,
    Corollary,
    Definition,
    Example,
    Remark,
    Custom(String),
}

/// 定理配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheoremConfig {
    pub theorem_type: TheoremType,
    pub numbering: Option<String>,
    pub title: Option<String>,
    pub show_number: bool,
}

impl Default for TheoremConfig {
    fn default() -> Self {
        Self {
            theorem_type: TheoremType::Theorem,
            numbering: None,
            title: None,
            show_number: true,
        }
    }
}

/// 定理
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theorem {
    pub content: String,
    pub config: TheoremConfig,
    pub label: Option<String>,
}

impl Theorem {
    pub fn new(content: String) -> Self {
        Self {
            content,
            config: TheoremConfig::default(),
            label: None,
        }
    }

    pub fn with_config(mut self, config: TheoremConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_type(mut self, theorem_type: TheoremType) -> Self {
        self.config.theorem_type = theorem_type;
        self
    }

    pub fn with_numbering(mut self, numbering: String) -> Self {
        self.config.numbering = Some(numbering);
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.config.title = Some(title);
        self
    }

    pub fn with_show_number(mut self, show_number: bool) -> Self {
        self.config.show_number = show_number;
        self
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        let type_name = self.type_to_typst();

        typst.push_str(&format!("#{}(", type_name));

        // 添加标签
        if let Some(label) = &self.label {
            typst.push_str(&format!("<{}> ", label));
        }

        // 添加标题
        if let Some(title) = &self.config.title {
            typst.push_str(&format!("[{}]: ", html_escape(title)));
        }

        // 添加内容
        typst.push_str(&format!("[{}])\n", html_escape(&self.content)));

        typst
    }

    fn type_to_typst(&self) -> String {
        match &self.config.theorem_type {
            TheoremType::Theorem => "theorem".to_string(),
            TheoremType::Lemma => "lemma".to_string(),
            TheoremType::Proposition => "proposition".to_string(),
            TheoremType::Corollary => "corollary".to_string(),
            TheoremType::Definition => "definition".to_string(),
            TheoremType::Example => "example".to_string(),
            TheoremType::Remark => "remark".to_string(),
            TheoremType::Custom(custom) => custom.clone(),
        }
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        let type_name = self.type_to_string();
        let id_attr = if let Some(label) = &self.label {
            format!(" id=\"{}\"", label)
        } else {
            String::new()
        };

        html.push_str(&format!(
            "<div class=\"typst-theorem\" data-type=\"{}\"{}>\n",
            type_name, id_attr
        ));

        // 添加标题栏
        html.push_str("  <div class=\"theorem-header\">\n");
        html.push_str(&format!(
            "    <span class=\"theorem-type\">{}</span>\n",
            type_name
        ));

        if self.config.show_number {
            html.push_str("    <span class=\"theorem-number\">1</span>\n");
        }

        if let Some(title) = &self.config.title {
            html.push_str(&format!(
                "    <span class=\"theorem-title\">{}</span>\n",
                html_escape(title)
            ));
        }

        html.push_str("  </div>\n");

        // 添加内容
        html.push_str(&format!(
            "  <div class=\"theorem-content\">{}</div>\n",
            html_escape(&self.content)
        ));

        html.push_str("</div>\n");

        html
    }

    fn type_to_string(&self) -> String {
        match &self.config.theorem_type {
            TheoremType::Theorem => "Theorem".to_string(),
            TheoremType::Lemma => "Lemma".to_string(),
            TheoremType::Proposition => "Proposition".to_string(),
            TheoremType::Corollary => "Corollary".to_string(),
            TheoremType::Definition => "Definition".to_string(),
            TheoremType::Example => "Example".to_string(),
            TheoremType::Remark => "Remark".to_string(),
            TheoremType::Custom(custom) => custom.clone(),
        }
    }
}

impl Default for Theorem {
    fn default() -> Self {
        Self::new("".to_string())
    }
}

/// 定理构建器
pub struct TheoremBuilder {
    theorem: Theorem,
}

impl TheoremBuilder {
    pub fn new(content: String) -> Self {
        Self {
            theorem: Theorem::new(content),
        }
    }

    pub fn theorem_type(mut self, theorem_type: TheoremType) -> Self {
        self.theorem = self.theorem.with_type(theorem_type);
        self
    }

    pub fn numbering(mut self, numbering: String) -> Self {
        self.theorem = self.theorem.with_numbering(numbering);
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.theorem = self.theorem.with_title(title);
        self
    }

    pub fn show_number(mut self, show_number: bool) -> Self {
        self.theorem = self.theorem.with_show_number(show_number);
        self
    }

    pub fn label(mut self, label: String) -> Self {
        self.theorem = self.theorem.with_label(label);
        self
    }

    pub fn build(self) -> Theorem {
        self.theorem
    }
}

impl Default for TheoremBuilder {
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
    fn test_theorem_creation() {
        let theorem = Theorem::new("Content".to_string());
        assert_eq!(theorem.content, "Content");
    }

    #[test]
    fn test_theorem_default() {
        let theorem = Theorem::default();
        assert_eq!(theorem.content, "");
    }

    #[test]
    fn test_theorem_config_default() {
        let config = TheoremConfig::default();
        assert!(matches!(config.theorem_type, TheoremType::Theorem));
        assert!(config.show_number);
    }

    #[test]
    fn test_theorem_with_type() {
        let theorem = Theorem::new("Content".to_string()).with_type(TheoremType::Lemma);
        assert!(matches!(theorem.config.theorem_type, TheoremType::Lemma));
    }

    #[test]
    fn test_theorem_with_numbering() {
        let theorem = Theorem::new("Content".to_string()).with_numbering("1".to_string());
        assert_eq!(theorem.config.numbering, Some("1".to_string()));
    }

    #[test]
    fn test_theorem_with_title() {
        let theorem =
            Theorem::new("Content".to_string()).with_title("Pythagorean Theorem".to_string());
        assert_eq!(
            theorem.config.title,
            Some("Pythagorean Theorem".to_string())
        );
    }

    #[test]
    fn test_theorem_with_show_number() {
        let theorem = Theorem::new("Content".to_string()).with_show_number(false);
        assert!(!theorem.config.show_number);
    }

    #[test]
    fn test_theorem_with_label() {
        let theorem = Theorem::new("Content".to_string()).with_label("thm1".to_string());
        assert_eq!(theorem.label, Some("thm1".to_string()));
    }

    #[test]
    fn test_theorem_type_variants() {
        assert!(matches!(TheoremType::Theorem, TheoremType::Theorem));
        assert!(matches!(TheoremType::Lemma, TheoremType::Lemma));
        assert!(matches!(
            TheoremType::Custom("test".to_string()),
            TheoremType::Custom(_)
        ));
    }

    #[test]
    fn test_to_typst() {
        let theorem = Theorem::new("Content".to_string());
        let typst = theorem.to_typst();
        assert!(typst.contains("#theorem("));
        assert!(typst.contains("[Content]"));
    }

    #[test]
    fn test_to_typst_with_title() {
        let theorem = Theorem::new("Content".to_string()).with_title("Title".to_string());
        let typst = theorem.to_typst();
        assert!(typst.contains("[Title]:"));
    }

    #[test]
    fn test_to_typst_with_label() {
        let theorem = Theorem::new("Content".to_string()).with_label("thm1".to_string());
        let typst = theorem.to_typst();
        assert!(typst.contains("<thm1>"));
    }

    #[test]
    fn test_to_typst_lemma() {
        let theorem = Theorem::new("Content".to_string()).with_type(TheoremType::Lemma);
        let typst = theorem.to_typst();
        assert!(typst.contains("#lemma("));
    }

    #[test]
    fn test_to_html() {
        let theorem = Theorem::new("Content".to_string());
        let html = theorem.to_html();
        assert!(html.contains("<div class=\"typst-theorem\""));
        assert!(html.contains("data-type=\"Theorem\""));
    }

    #[test]
    fn test_to_html_with_title() {
        let theorem = Theorem::new("Content".to_string()).with_title("Title".to_string());
        let html = theorem.to_html();
        assert!(html.contains("theorem-title"));
        assert!(html.contains("Title"));
    }

    #[test]
    fn test_to_html_with_label() {
        let theorem = Theorem::new("Content".to_string()).with_label("thm1".to_string());
        let html = theorem.to_html();
        assert!(html.contains("id=\"thm1\""));
    }

    #[test]
    fn test_to_html_without_show_number() {
        let theorem = Theorem::new("Content".to_string()).with_show_number(false);
        let html = theorem.to_html();
        assert!(!html.contains("theorem-number"));
    }

    #[test]
    fn test_theorem_builder() {
        let theorem = TheoremBuilder::new("Content".to_string())
            .theorem_type(TheoremType::Lemma)
            .title("Title".to_string())
            .label("thm1".to_string())
            .build();

        assert_eq!(theorem.content, "Content");
        assert!(matches!(theorem.config.theorem_type, TheoremType::Lemma));
    }

    #[test]
    fn test_theorem_builder_default() {
        let builder = TheoremBuilder::default();
        let theorem = builder.build();
        assert_eq!(theorem.content, "");
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_type_to_typst() {
        let theorem = Theorem::new("Content".to_string());
        assert_eq!(theorem.type_to_typst(), "theorem");
    }

    #[test]
    fn test_type_to_string() {
        let theorem = Theorem::new("Content".to_string());
        assert_eq!(theorem.type_to_string(), "Theorem");
    }

    #[test]
    fn test_to_typst_definition() {
        let theorem = Theorem::new("Content".to_string()).with_type(TheoremType::Definition);
        let typst = theorem.to_typst();
        assert!(typst.contains("#definition("));
    }

    #[test]
    fn test_to_html_definition() {
        let theorem = Theorem::new("Content".to_string()).with_type(TheoremType::Definition);
        let html = theorem.to_html();
        assert!(html.contains("data-type=\"Definition\""));
    }

    #[test]
    fn test_to_typst_custom_type() {
        let theorem =
            Theorem::new("Content".to_string()).with_type(TheoremType::Custom("axiom".to_string()));
        let typst = theorem.to_typst();
        assert!(typst.contains("#axiom("));
    }

    #[test]
    fn test_to_html_custom_type() {
        let theorem =
            Theorem::new("Content".to_string()).with_type(TheoremType::Custom("Axiom".to_string()));
        let html = theorem.to_html();
        assert!(html.contains("data-type=\"Axiom\""));
    }
}
