/*!
 * 航空航天级图形系统
 * 实现 Typst 的图形功能（图形容器、图形标题、图形标签、图形位置控制、图形引用）
 */

use serde::{Deserialize, Serialize};

/// 图形类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FigureKind {
    Auto,
    Image,
    Table,
    Code,
    Diagram,
    Custom(String),
}

/// 图形位置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FigurePlacement {
    Auto,
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

/// 图形配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FigureConfig {
    pub kind: FigureKind,
    pub placement: FigurePlacement,
    pub caption: Option<String>,
    pub alt: Option<String>,
    pub supplement: Option<String>,
    pub numbering: Option<String>,
    pub gap: Option<f64>,
    pub outlined: bool,
}

impl Default for FigureConfig {
    fn default() -> Self {
        Self {
            kind: FigureKind::Auto,
            placement: FigurePlacement::Auto,
            caption: None,
            alt: None,
            supplement: None,
            numbering: None,
            gap: None,
            outlined: true,
        }
    }
}

/// 图形
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Figure {
    pub content: String,
    pub config: FigureConfig,
    pub label: Option<String>,
}

impl Figure {
    pub fn new(content: String) -> Self {
        Self {
            content,
            config: FigureConfig::default(),
            label: None,
        }
    }

    pub fn with_config(mut self, config: FigureConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_kind(mut self, kind: FigureKind) -> Self {
        self.config.kind = kind;
        self
    }

    pub fn with_placement(mut self, placement: FigurePlacement) -> Self {
        self.config.placement = placement;
        self
    }

    pub fn with_caption(mut self, caption: String) -> Self {
        self.config.caption = Some(caption);
        self
    }

    pub fn with_alt(mut self, alt: String) -> Self {
        self.config.alt = Some(alt);
        self
    }

    pub fn with_supplement(mut self, supplement: String) -> Self {
        self.config.supplement = Some(supplement);
        self
    }

    pub fn with_numbering(mut self, numbering: String) -> Self {
        self.config.numbering = Some(numbering);
        self
    }

    pub fn with_gap(mut self, gap: f64) -> Self {
        self.config.gap = Some(gap);
        self
    }

    pub fn with_outlined(mut self, outlined: bool) -> Self {
        self.config.outlined = outlined;
        self
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        typst.push_str("#figure(");

        // 添加标签
        if let Some(label) = &self.label {
            typst.push_str(&format!("<{}> ", label));
        }

        // 添加内容
        typst.push_str(&format!("[{}]", html_escape(&self.content)));

        // 添加标题
        if let Some(caption) = &self.config.caption {
            typst.push_str(&format!(", caption: [{}]", html_escape(caption)));
        }

        // 添加 Alt 文本
        if let Some(alt) = &self.config.alt {
            typst.push_str(&format!(", alt: \"{}\"", html_escape(alt)));
        }

        // 添加位置
        if !matches!(self.config.placement, FigurePlacement::Auto) {
            typst.push_str(&format!(", placement: {}", self.placement_to_typst()));
        }

        // 添加类型
        if !matches!(self.config.kind, FigureKind::Auto) {
            typst.push_str(&format!(", kind: \"{}\"", self.kind_to_typst()));
        }

        // 添加补充文本
        if let Some(supplement) = &self.config.supplement {
            typst.push_str(&format!(", supplement: [{}]", html_escape(supplement)));
        }

        // 添加编号
        if let Some(numbering) = &self.config.numbering {
            typst.push_str(&format!(", numbering: \"{}\"", numbering));
        }

        // 添加间距
        if let Some(gap) = self.config.gap {
            typst.push_str(&format!(", gap: {}em", gap));
        }

        // 添加大纲显示
        if !self.config.outlined {
            typst.push_str(", outlined: false");
        }

        typst.push_str(")\n");

        typst
    }

    fn placement_to_typst(&self) -> String {
        match self.config.placement {
            FigurePlacement::Auto => "auto".to_string(),
            FigurePlacement::Top => "top".to_string(),
            FigurePlacement::Bottom => "bottom".to_string(),
            FigurePlacement::Left => "left".to_string(),
            FigurePlacement::Right => "right".to_string(),
            FigurePlacement::Center => "center".to_string(),
        }
    }

    fn kind_to_typst(&self) -> String {
        match &self.config.kind {
            FigureKind::Auto => "auto".to_string(),
            FigureKind::Image => "image".to_string(),
            FigureKind::Table => "table".to_string(),
            FigureKind::Code => "code".to_string(),
            FigureKind::Diagram => "diagram".to_string(),
            FigureKind::Custom(custom) => custom.clone(),
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

        let kind_attr = format!(" data-kind=\"{}\"", self.kind_to_typst());
        let placement_attr = format!(" data-placement=\"{}\"", self.placement_to_typst());

        let outlined_attr = if self.config.outlined {
            " data-outlined=\"true\""
        } else {
            " data-outlined=\"false\""
        };

        html.push_str(&format!(
            "<figure class=\"typst-figure\"{}{}{}{}>\n",
            id_attr, kind_attr, placement_attr, outlined_attr
        ));

        // 添加内容
        html.push_str("  <div class=\"figure-content\">\n");
        html.push_str(&format!("    {}\n", html_escape(&self.content)));
        html.push_str("  </div>\n");

        // 添加标题
        if let Some(caption) = &self.config.caption {
            html.push_str(&format!(
                "  <figcaption>{}</figcaption>\n",
                html_escape(caption)
            ));
        }

        html.push_str("</figure>\n");

        html
    }
}

impl Default for Figure {
    fn default() -> Self {
        Self::new("".to_string())
    }
}

/// 图形构建器
pub struct FigureBuilder {
    figure: Figure,
}

impl FigureBuilder {
    pub fn new(content: String) -> Self {
        Self {
            figure: Figure::new(content),
        }
    }

    pub fn kind(mut self, kind: FigureKind) -> Self {
        self.figure = self.figure.with_kind(kind);
        self
    }

    pub fn placement(mut self, placement: FigurePlacement) -> Self {
        self.figure = self.figure.with_placement(placement);
        self
    }

    pub fn caption(mut self, caption: String) -> Self {
        self.figure = self.figure.with_caption(caption);
        self
    }

    pub fn alt(mut self, alt: String) -> Self {
        self.figure = self.figure.with_alt(alt);
        self
    }

    pub fn supplement(mut self, supplement: String) -> Self {
        self.figure = self.figure.with_supplement(supplement);
        self
    }

    pub fn numbering(mut self, numbering: String) -> Self {
        self.figure = self.figure.with_numbering(numbering);
        self
    }

    pub fn gap(mut self, gap: f64) -> Self {
        self.figure = self.figure.with_gap(gap);
        self
    }

    pub fn outlined(mut self, outlined: bool) -> Self {
        self.figure = self.figure.with_outlined(outlined);
        self
    }

    pub fn label(mut self, label: String) -> Self {
        self.figure = self.figure.with_label(label);
        self
    }

    pub fn build(self) -> Figure {
        self.figure
    }
}

impl Default for FigureBuilder {
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
    fn test_figure_creation() {
        let figure = Figure::new("Content".to_string());
        assert_eq!(figure.content, "Content");
    }

    #[test]
    fn test_figure_default() {
        let figure = Figure::default();
        assert_eq!(figure.content, "");
    }

    #[test]
    fn test_figure_config_default() {
        let config = FigureConfig::default();
        assert!(matches!(config.kind, FigureKind::Auto));
        assert!(config.outlined);
    }

    #[test]
    fn test_figure_with_kind() {
        let figure = Figure::new("Content".to_string()).with_kind(FigureKind::Image);
        assert!(matches!(figure.config.kind, FigureKind::Image));
    }

    #[test]
    fn test_figure_with_placement() {
        let figure = Figure::new("Content".to_string()).with_placement(FigurePlacement::Top);
        assert_eq!(figure.config.placement, FigurePlacement::Top);
    }

    #[test]
    fn test_figure_with_caption() {
        let figure = Figure::new("Content".to_string()).with_caption("Figure 1".to_string());
        assert_eq!(figure.config.caption, Some("Figure 1".to_string()));
    }

    #[test]
    fn test_figure_with_alt() {
        let figure = Figure::new("Content".to_string()).with_alt("Alt text".to_string());
        assert_eq!(figure.config.alt, Some("Alt text".to_string()));
    }

    #[test]
    fn test_figure_with_supplement() {
        let figure = Figure::new("Content".to_string()).with_supplement("Fig".to_string());
        assert_eq!(figure.config.supplement, Some("Fig".to_string()));
    }

    #[test]
    fn test_figure_with_numbering() {
        let figure = Figure::new("Content".to_string()).with_numbering("1.".to_string());
        assert_eq!(figure.config.numbering, Some("1.".to_string()));
    }

    #[test]
    fn test_figure_with_gap() {
        let figure = Figure::new("Content".to_string()).with_gap(1.5);
        assert_eq!(figure.config.gap, Some(1.5));
    }

    #[test]
    fn test_figure_with_outlined() {
        let figure = Figure::new("Content".to_string()).with_outlined(false);
        assert!(!figure.config.outlined);
    }

    #[test]
    fn test_figure_with_label() {
        let figure = Figure::new("Content".to_string()).with_label("fig1".to_string());
        assert_eq!(figure.label, Some("fig1".to_string()));
    }

    #[test]
    fn test_to_typst() {
        let figure = Figure::new("Content".to_string());
        let typst = figure.to_typst();
        assert!(typst.contains("#figure("));
        assert!(typst.contains("[Content]"));
    }

    #[test]
    fn test_to_typst_with_caption() {
        let figure = Figure::new("Content".to_string()).with_caption("Figure 1".to_string());
        let typst = figure.to_typst();
        assert!(typst.contains("caption: [Figure 1]"));
    }

    #[test]
    fn test_to_typst_with_label() {
        let figure = Figure::new("Content".to_string()).with_label("fig1".to_string());
        let typst = figure.to_typst();
        assert!(typst.contains("<fig1>"));
    }

    #[test]
    fn test_to_typst_with_placement() {
        let figure = Figure::new("Content".to_string()).with_placement(FigurePlacement::Top);
        let typst = figure.to_typst();
        assert!(typst.contains("placement: top"));
    }

    #[test]
    fn test_to_html() {
        let figure = Figure::new("Content".to_string());
        let html = figure.to_html();
        assert!(html.contains("<figure class=\"typst-figure\""));
        assert!(html.contains("Content"));
    }

    #[test]
    fn test_to_html_with_caption() {
        let figure = Figure::new("Content".to_string()).with_caption("Figure 1".to_string());
        let html = figure.to_html();
        assert!(html.contains("<figcaption>Figure 1</figcaption>"));
    }

    #[test]
    fn test_to_html_with_label() {
        let figure = Figure::new("Content".to_string()).with_label("fig1".to_string());
        let html = figure.to_html();
        assert!(html.contains("id=\"fig1\""));
    }

    #[test]
    fn test_figure_builder() {
        let figure = FigureBuilder::new("Content".to_string())
            .kind(FigureKind::Image)
            .caption("Figure 1".to_string())
            .label("fig1".to_string())
            .build();

        assert_eq!(figure.content, "Content");
        assert!(matches!(figure.config.kind, FigureKind::Image));
    }

    #[test]
    fn test_figure_builder_default() {
        let builder = FigureBuilder::default();
        let figure = builder.build();
        assert_eq!(figure.content, "");
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_figure_kind_variants() {
        assert!(matches!(FigureKind::Auto, FigureKind::Auto));
        assert!(matches!(FigureKind::Image, FigureKind::Image));
        assert!(matches!(
            FigureKind::Custom("test".to_string()),
            FigureKind::Custom(_)
        ));
    }

    #[test]
    fn test_figure_placement_variants() {
        assert_eq!(FigurePlacement::Auto, FigurePlacement::Auto);
        assert_eq!(FigurePlacement::Top, FigurePlacement::Top);
        assert_eq!(FigurePlacement::Bottom, FigurePlacement::Bottom);
    }

    #[test]
    fn test_placement_to_typst() {
        let figure = Figure::new("Content".to_string());
        assert_eq!(figure.placement_to_typst(), "auto");
    }

    #[test]
    fn test_kind_to_typst() {
        let figure = Figure::new("Content".to_string());
        assert_eq!(figure.kind_to_typst(), "auto");
    }

    #[test]
    fn test_to_typst_with_kind() {
        let figure = Figure::new("Content".to_string()).with_kind(FigureKind::Table);
        let typst = figure.to_typst();
        assert!(typst.contains("kind: \"table\""));
    }

    #[test]
    fn test_to_html_with_kind() {
        let figure = Figure::new("Content".to_string()).with_kind(FigureKind::Image);
        let html = figure.to_html();
        assert!(html.contains("data-kind=\"image\""));
    }

    #[test]
    fn test_to_html_with_placement() {
        let figure = Figure::new("Content".to_string()).with_placement(FigurePlacement::Top);
        let html = figure.to_html();
        assert!(html.contains("data-placement=\"top\""));
    }

    #[test]
    fn test_to_html_without_outlined() {
        let figure = Figure::new("Content".to_string()).with_outlined(false);
        let html = figure.to_html();
        assert!(html.contains("data-outlined=\"false\""));
    }
}
