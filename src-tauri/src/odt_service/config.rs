use serde::{Deserialize, Serialize};

/// ODT 版本
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OdtVersion {
    /// ODF 1.0
    V1_0,
    /// ODF 1.1
    V1_1,
    /// ODF 1.2
    V1_2,
    /// ODF 1.3
    V1_3,
}

/// 页面布局
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PageLayout {
    /// A4
    A4,
    /// A5
    A5,
    /// Letter
    Letter,
    /// Legal
    Legal,
    /// 自定义
    Custom { width: f64, height: f64 },
}

/// ODT 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdtConfig {
    /// ODT 版本
    pub version: OdtVersion,
    /// 页面布局
    pub page_layout: PageLayout,
    /// 是否显示页码
    pub show_page_numbers: bool,
}

impl OdtConfig {
    /// 创建新的 ODT 配置
    pub fn new() -> Self {
        Self {
            version: OdtVersion::V1_2,
            page_layout: PageLayout::A4,
            show_page_numbers: true,
        }
    }

    /// 设置版本
    #[allow(dead_code)]
    pub fn with_version(mut self, version: OdtVersion) -> Self {
        self.version = version;
        self
    }

    /// 设置页面布局
    #[allow(dead_code)]
    pub fn with_page_layout(mut self, layout: PageLayout) -> Self {
        self.page_layout = layout;
        self
    }
}

impl Default for OdtConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odt_config_new() {
        let config = OdtConfig::new();
        assert_eq!(config.version, OdtVersion::V1_2);
        assert_eq!(config.page_layout, PageLayout::A4);
    }

    #[test]
    fn test_odt_config_with_version() {
        let config = OdtConfig::new().with_version(OdtVersion::V1_3);
        assert_eq!(config.version, OdtVersion::V1_3);
    }

    #[test]
    fn test_odt_config_serialization() {
        let config = OdtConfig::new();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_odt_config_deserialization() {
        let json = r#"{"version":"V1_2","page_layout":"A4","show_page_numbers":true}"#;
        let config: OdtConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.version, OdtVersion::V1_2);
    }

    #[test]
    fn test_odt_config_default() {
        let config = OdtConfig::default();
        assert_eq!(config.version, OdtVersion::V1_2);
    }

    #[test]
    fn test_odt_config_with_page_layout() {
        let config = OdtConfig::new().with_page_layout(PageLayout::Letter);
        assert_eq!(config.page_layout, PageLayout::Letter);
    }

    #[test]
    fn test_odt_config_chaining() {
        let config = OdtConfig::new()
            .with_version(OdtVersion::V1_3)
            .with_page_layout(PageLayout::Letter);
        assert_eq!(config.version, OdtVersion::V1_3);
        assert_eq!(config.page_layout, PageLayout::Letter);
    }

    #[test]
    fn test_odt_version_v1_2() {
        assert_eq!(OdtVersion::V1_2, OdtVersion::V1_2);
    }

    #[test]
    fn test_odt_version_v1_3() {
        assert_eq!(OdtVersion::V1_3, OdtVersion::V1_3);
    }

    #[test]
    fn test_page_layout_a4() {
        assert_eq!(PageLayout::A4, PageLayout::A4);
    }

    #[test]
    fn test_page_layout_letter() {
        assert_eq!(PageLayout::Letter, PageLayout::Letter);
    }

    #[test]
    fn test_page_layout_legal() {
        assert_eq!(PageLayout::Legal, PageLayout::Legal);
    }
}
