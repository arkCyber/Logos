use serde::{Deserialize, Serialize};

/// Markdown 风格
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MarkdownFlavor {
    /// 标准 Markdown
    Standard,
    /// GitHub Flavored Markdown
    Gfm,
    /// CommonMark
    CommonMark,
    /// MultiMarkdown
    MultiMarkdown,
}

/// Markdown 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownConfig {
    /// Markdown 风格
    pub flavor: MarkdownFlavor,
    /// 是否保留 HTML 标签
    pub preserve_html: bool,
    /// 是否使用 GFM 表格
    pub use_gfm_tables: bool,
    /// 是否使用任务列表
    pub use_task_lists: bool,
}

impl MarkdownConfig {
    /// 创建新的 Markdown 配置
    pub fn new() -> Self {
        Self {
            flavor: MarkdownFlavor::Gfm,
            preserve_html: false,
            use_gfm_tables: true,
            use_task_lists: true,
        }
    }

    /// 设置风格
    #[allow(dead_code)]
    pub fn with_flavor(mut self, flavor: MarkdownFlavor) -> Self {
        self.flavor = flavor;
        self
    }

    /// 设置是否保留 HTML
    #[allow(dead_code)]
    pub fn with_preserve_html(mut self, preserve: bool) -> Self {
        self.preserve_html = preserve;
        self
    }
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_config_new() {
        let config = MarkdownConfig::new();
        assert_eq!(config.flavor, MarkdownFlavor::Gfm);
    }

    #[test]
    fn test_markdown_config_with_flavor() {
        let config = MarkdownConfig::new().with_flavor(MarkdownFlavor::Standard);
        assert_eq!(config.flavor, MarkdownFlavor::Standard);
    }

    #[test]
    fn test_markdown_config_serialization() {
        let config = MarkdownConfig::new();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }
}
