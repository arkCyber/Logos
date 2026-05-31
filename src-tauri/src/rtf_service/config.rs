use serde::{Deserialize, Serialize};

/// 代码页
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CodePage {
    /// ANSI
    Ansi,
    /// Mac
    Mac,
    /// PC
    Pc,
    /// PCA
    Pca,
}

/// 字符集
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CharacterSet {
    /// ANSI
    Ansi,
    /// Mac
    Mac,
    /// PC
    Pc,
    /// PCA
    Pca,
}

/// RTF 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtfConfig {
    /// 代码页
    pub code_page: CodePage,
    /// 字符集
    pub character_set: CharacterSet,
    /// 默认字体
    pub default_font: String,
    /// 默认字体大小（半点）
    pub default_font_size: u16,
    /// 是否使用 Unicode
    pub use_unicode: bool,
}

impl RtfConfig {
    /// 创建新的 RTF 配置
    pub fn new() -> Self {
        Self {
            code_page: CodePage::Ansi,
            character_set: CharacterSet::Ansi,
            default_font: "Times New Roman".to_string(),
            default_font_size: 24, // 12pt in half-points
            use_unicode: true,
        }
    }

    /// 设置代码页
    #[allow(dead_code)]
    pub fn with_code_page(mut self, code_page: CodePage) -> Self {
        self.code_page = code_page;
        self
    }

    /// 设置字符集
    #[allow(dead_code)]
    pub fn with_character_set(mut self, character_set: CharacterSet) -> Self {
        self.character_set = character_set;
        self
    }

    /// 设置默认字体
    #[allow(dead_code)]
    pub fn with_default_font(mut self, font: String) -> Self {
        self.default_font = font;
        self
    }

    /// 设置默认字体大小（点）
    #[allow(dead_code)]
    pub fn with_font_size(mut self, size: u16) -> Self {
        self.default_font_size = size * 2; // Convert to half-points
        self
    }

    /// 设置是否使用 Unicode
    #[allow(dead_code)]
    pub fn with_unicode(mut self, use_unicode: bool) -> Self {
        self.use_unicode = use_unicode;
        self
    }
}

impl Default for RtfConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtf_config_new() {
        let config = RtfConfig::new();
        assert_eq!(config.default_font, "Times New Roman");
        assert_eq!(config.default_font_size, 24);
    }

    #[test]
    fn test_rtf_config_with_code_page() {
        let config = RtfConfig::new().with_code_page(CodePage::Mac);
        assert_eq!(config.code_page, CodePage::Mac);
    }

    #[test]
    fn test_rtf_config_with_font_size() {
        let config = RtfConfig::new().with_font_size(14);
        assert_eq!(config.default_font_size, 28);
    }

    #[test]
    fn test_rtf_config_chaining() {
        let config = RtfConfig::new()
            .with_default_font("Arial".to_string())
            .with_font_size(12)
            .with_unicode(false);
        assert_eq!(config.default_font, "Arial");
        assert!(!config.use_unicode);
    }

    #[test]
    fn test_rtf_config_serialization() {
        let config = RtfConfig::new();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_rtf_config_deserialization() {
        let json = r#"{"default_font":"Times New Roman","default_font_size":24,"code_page":"Ansi","character_set":"Ansi","use_unicode":true}"#;
        let config: RtfConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.default_font, "Times New Roman");
    }

    #[test]
    fn test_rtf_config_default() {
        let config = RtfConfig::default();
        assert_eq!(config.default_font, "Times New Roman");
        assert_eq!(config.default_font_size, 24);
    }

    #[test]
    fn test_rtf_config_with_default_font() {
        let config = RtfConfig::new().with_default_font("Arial".to_string());
        assert_eq!(config.default_font, "Arial");
    }

    #[test]
    fn test_rtf_config_with_unicode() {
        let config = RtfConfig::new().with_unicode(false);
        assert!(!config.use_unicode);
    }

    #[test]
    fn test_code_page_ansi() {
        assert_eq!(CodePage::Ansi, CodePage::Ansi);
    }

    #[test]
    fn test_code_page_mac() {
        assert_eq!(CodePage::Mac, CodePage::Mac);
    }

    #[test]
    fn test_code_page_pc() {
        assert_eq!(CodePage::Pc, CodePage::Pc);
    }

    #[test]
    fn test_code_page_serialization() {
        let code_page = CodePage::Ansi;
        let json = serde_json::to_string(&code_page);
        assert!(json.is_ok());
    }

    #[test]
    fn test_code_page_deserialization() {
        let json = r#""Ansi""#;
        let code_page: CodePage = serde_json::from_str(json).unwrap();
        assert_eq!(code_page, CodePage::Ansi);
    }

    #[test]
    fn test_rtf_config_font_size_bounds() {
        let config = RtfConfig::new().with_font_size(100);
        assert_eq!(config.default_font_size, 200);
    }

    #[test]
    fn test_rtf_config_font_size_zero() {
        let config = RtfConfig::new().with_font_size(0);
        assert_eq!(config.default_font_size, 0);
    }

    #[test]
    fn test_rtf_config_empty_font() {
        let config = RtfConfig::new().with_default_font("".to_string());
        assert_eq!(config.default_font, "");
    }

    #[test]
    fn test_rtf_config_long_font_name() {
        let long_font = "A".repeat(1000);
        let config = RtfConfig::new().with_default_font(long_font.clone());
        assert_eq!(config.default_font.len(), 1000);
    }
}
