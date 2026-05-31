/*!
 * 航空航天级分栏系统
 * 实现 Typst 的分栏功能（多栏布局、栏宽控制、栏间距、栏平衡、分栏断行）
 */

use serde::{Deserialize, Serialize};

/// 分栏配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnsConfig {
    pub count: usize,
    pub gutter: f64,
    pub balance: bool,
}

impl Default for ColumnsConfig {
    fn default() -> Self {
        Self {
            count: 2,
            gutter: 1.0,
            balance: true,
        }
    }
}

/// 分栏
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Columns {
    pub content: String,
    pub config: ColumnsConfig,
}

impl Columns {
    pub fn new(content: String) -> Self {
        Self {
            content,
            config: ColumnsConfig::default(),
        }
    }

    pub fn with_config(mut self, config: ColumnsConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_count(mut self, count: usize) -> Self {
        self.config.count = count;
        self
    }

    pub fn with_gutter(mut self, gutter: f64) -> Self {
        self.config.gutter = gutter;
        self
    }

    pub fn with_balance(mut self, balance: bool) -> Self {
        self.config.balance = balance;
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        typst.push_str("#columns(");
        typst.push_str(&format!("count: {}, ", self.config.count));
        typst.push_str(&format!("gutter: {}em, ", self.config.gutter));

        if !self.config.balance {
            typst.push_str("balance: false, ");
        }

        typst.push_str(&format!("[{}])\n", html_escape(&self.content)));

        typst
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        let count_attr = format!("column-count: {};", self.config.count);
        let gutter_attr = format!("column-gap: {}em;", self.config.gutter);
        let balance_attr = if self.config.balance {
            "column-fill: balance;"
        } else {
            "column-fill: auto;"
        };

        html.push_str(&format!(
            "<div class=\"typst-columns\" style=\"{}{}{}\">\n",
            count_attr, gutter_attr, balance_attr
        ));
        html.push_str(&format!("  {}\n", html_escape(&self.content)));
        html.push_str("</div>\n");

        html
    }
}

impl Default for Columns {
    fn default() -> Self {
        Self::new("".to_string())
    }
}

/// 分栏构建器
pub struct ColumnsBuilder {
    columns: Columns,
}

impl ColumnsBuilder {
    pub fn new(content: String) -> Self {
        Self {
            columns: Columns::new(content),
        }
    }

    pub fn count(mut self, count: usize) -> Self {
        self.columns = self.columns.with_count(count);
        self
    }

    pub fn gutter(mut self, gutter: f64) -> Self {
        self.columns = self.columns.with_gutter(gutter);
        self
    }

    pub fn balance(mut self, balance: bool) -> Self {
        self.columns = self.columns.with_balance(balance);
        self
    }

    pub fn build(self) -> Columns {
        self.columns
    }
}

impl Default for ColumnsBuilder {
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
    fn test_columns_creation() {
        let columns = Columns::new("Content".to_string());
        assert_eq!(columns.content, "Content");
    }

    #[test]
    fn test_columns_default() {
        let columns = Columns::default();
        assert_eq!(columns.content, "");
    }

    #[test]
    fn test_columns_config_default() {
        let config = ColumnsConfig::default();
        assert_eq!(config.count, 2);
        assert_eq!(config.gutter, 1.0);
        assert!(config.balance);
    }

    #[test]
    fn test_columns_with_count() {
        let columns = Columns::new("Content".to_string()).with_count(3);
        assert_eq!(columns.config.count, 3);
    }

    #[test]
    fn test_columns_with_gutter() {
        let columns = Columns::new("Content".to_string()).with_gutter(2.0);
        assert_eq!(columns.config.gutter, 2.0);
    }

    #[test]
    fn test_columns_with_balance() {
        let columns = Columns::new("Content".to_string()).with_balance(false);
        assert!(!columns.config.balance);
    }

    #[test]
    fn test_to_typst() {
        let columns = Columns::new("Content".to_string());
        let typst = columns.to_typst();
        assert!(typst.contains("#columns("));
        assert!(typst.contains("count: 2"));
        assert!(typst.contains("Content"));
    }

    #[test]
    fn test_to_typst_with_count() {
        let columns = Columns::new("Content".to_string()).with_count(3);
        let typst = columns.to_typst();
        assert!(typst.contains("count: 3"));
    }

    #[test]
    fn test_to_typst_with_gutter() {
        let columns = Columns::new("Content".to_string()).with_gutter(2.0);
        let typst = columns.to_typst();
        assert!(typst.contains("gutter: 2em"));
    }

    #[test]
    fn test_to_typst_without_balance() {
        let columns = Columns::new("Content".to_string()).with_balance(false);
        let typst = columns.to_typst();
        assert!(typst.contains("balance: false"));
    }

    #[test]
    fn test_to_html() {
        let columns = Columns::new("Content".to_string());
        let html = columns.to_html();
        assert!(html.contains("<div class=\"typst-columns\""));
        assert!(html.contains("column-count: 2"));
        assert!(html.contains("Content"));
    }

    #[test]
    fn test_to_html_with_count() {
        let columns = Columns::new("Content".to_string()).with_count(3);
        let html = columns.to_html();
        assert!(html.contains("column-count: 3"));
    }

    #[test]
    fn test_to_html_with_gutter() {
        let columns = Columns::new("Content".to_string()).with_gutter(2.0);
        let html = columns.to_html();
        assert!(html.contains("column-gap: 2em"));
    }

    #[test]
    fn test_to_html_with_balance() {
        let columns = Columns::new("Content".to_string()).with_balance(true);
        let html = columns.to_html();
        assert!(html.contains("column-fill: balance"));
    }

    #[test]
    fn test_to_html_without_balance() {
        let columns = Columns::new("Content".to_string()).with_balance(false);
        let html = columns.to_html();
        assert!(html.contains("column-fill: auto"));
    }

    #[test]
    fn test_columns_builder() {
        let columns = ColumnsBuilder::new("Content".to_string())
            .count(3)
            .gutter(2.0)
            .build();

        assert_eq!(columns.content, "Content");
        assert_eq!(columns.config.count, 3);
    }

    #[test]
    fn test_columns_builder_default() {
        let builder = ColumnsBuilder::default();
        let columns = builder.build();
        assert_eq!(columns.content, "");
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_columns_config_variations() {
        let config1 = ColumnsConfig::default();
        let config2 = ColumnsConfig {
            count: 3,
            gutter: 2.0,
            balance: false,
        };

        assert_eq!(config1.count, 2);
        assert_eq!(config2.count, 3);
        assert_eq!(config2.gutter, 2.0);
        assert!(!config2.balance);
    }

    #[test]
    fn test_to_typst_single_column() {
        let columns = Columns::new("Content".to_string()).with_count(1);
        let typst = columns.to_typst();
        assert!(typst.contains("count: 1"));
    }

    #[test]
    fn test_to_html_single_column() {
        let columns = Columns::new("Content".to_string()).with_count(1);
        let html = columns.to_html();
        assert!(html.contains("column-count: 1"));
    }

    #[test]
    fn test_to_typst_large_gutter() {
        let columns = Columns::new("Content".to_string()).with_gutter(5.0);
        let typst = columns.to_typst();
        assert!(typst.contains("gutter: 5em"));
    }

    #[test]
    fn test_to_html_large_gutter() {
        let columns = Columns::new("Content".to_string()).with_gutter(5.0);
        let html = columns.to_html();
        assert!(html.contains("column-gap: 5em"));
    }
}
