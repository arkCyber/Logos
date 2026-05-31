use serde::{Deserialize, Serialize};

/// EPUB 版本
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EpubVersion {
    /// EPUB 2.0
    V2,
    /// EPUB 3.0
    V3,
    /// EPUB 3.1
    V31,
}

/// 阅读方向
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpineDirection {
    /// 从左到右
    Ltr,
    /// 从右到左
    Rtl,
    /// 默认
    Default,
}

/// EPUB 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpubConfig {
    /// EPUB 版本
    pub version: EpubVersion,
    /// 阅读方向
    pub direction: SpineDirection,
    /// 是否包含目录
    pub include_toc: bool,
    /// 是否包含页码
    pub include_page_numbers: bool,
}

impl EpubConfig {
    /// 创建新的 EPUB 配置
    pub fn new() -> Self {
        Self {
            version: EpubVersion::V3,
            direction: SpineDirection::Default,
            include_toc: true,
            include_page_numbers: false,
        }
    }

    /// 设置版本
    #[allow(dead_code)]
    pub fn with_version(mut self, version: EpubVersion) -> Self {
        self.version = version;
        self
    }

    /// 设置方向
    #[allow(dead_code)]
    pub fn with_direction(mut self, direction: SpineDirection) -> Self {
        self.direction = direction;
        self
    }

    /// 设置是否包含目录
    #[allow(dead_code)]
    pub fn with_toc(mut self, include: bool) -> Self {
        self.include_toc = include;
        self
    }
}

impl Default for EpubConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epub_config_new() {
        let config = EpubConfig::new();
        assert_eq!(config.version, EpubVersion::V3);
        assert!(config.include_toc);
    }

    #[test]
    fn test_epub_config_with_version() {
        let config = EpubConfig::new().with_version(EpubVersion::V2);
        assert_eq!(config.version, EpubVersion::V2);
    }

    #[test]
    fn test_epub_config_chaining() {
        let config = EpubConfig::new()
            .with_version(EpubVersion::V31)
            .with_direction(SpineDirection::Ltr);
        assert_eq!(config.direction, SpineDirection::Ltr);
    }

    #[test]
    fn test_epub_config_serialization() {
        let config = EpubConfig::new();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_epub_config_deserialization() {
        let json = r#"{"version":"V3","include_toc":true,"direction":"Ltr","include_page_numbers":false}"#;
        let config: EpubConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.version, EpubVersion::V3);
        assert!(config.include_toc);
    }

    #[test]
    fn test_epub_config_default() {
        let config = EpubConfig::default();
        assert_eq!(config.version, EpubVersion::V3);
        assert!(config.include_toc);
    }

    #[test]
    fn test_epub_config_with_direction() {
        let config = EpubConfig::new().with_direction(SpineDirection::Rtl);
        assert_eq!(config.direction, SpineDirection::Rtl);
    }

    #[test]
    fn test_epub_version_v2() {
        assert_eq!(EpubVersion::V2, EpubVersion::V2);
    }

    #[test]
    fn test_epub_version_v3() {
        assert_eq!(EpubVersion::V3, EpubVersion::V3);
    }

    #[test]
    fn test_epub_version_v31() {
        assert_eq!(EpubVersion::V31, EpubVersion::V31);
    }

    #[test]
    fn test_spine_direction_ltr() {
        assert_eq!(SpineDirection::Ltr, SpineDirection::Ltr);
    }

    #[test]
    fn test_spine_direction_rtl() {
        assert_eq!(SpineDirection::Rtl, SpineDirection::Rtl);
    }

    #[test]
    fn test_spine_direction_default() {
        assert_eq!(SpineDirection::Default, SpineDirection::Default);
    }
}
