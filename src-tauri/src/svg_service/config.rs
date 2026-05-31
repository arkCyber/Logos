use serde::{Deserialize, Serialize};

/// SVG 版本
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SvgVersion {
    /// SVG 1.0
    V1_0,
    /// SVG 1.1
    V1_1,
    /// SVG 2.0
    V2_0,
}

/// 视口框
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewBox {
    /// x 坐标
    pub x: f64,
    /// y 坐标
    pub y: f64,
    /// 宽度
    pub width: f64,
    /// 高度
    pub height: f64,
}

impl ViewBox {
    /// 创建新的视口框
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// 创建标准视口（0, 0, 100, 100）
    pub fn standard() -> Self {
        Self::new(0.0, 0.0, 100.0, 100.0)
    }
}

impl Default for ViewBox {
    fn default() -> Self {
        Self::standard()
    }
}

/// SVG 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgConfig {
    /// SVG 版本
    pub version: SvgVersion,
    /// 视口框
    pub view_box: ViewBox,
    /// 宽度
    pub width: f64,
    /// 高度
    pub height: f64,
    /// 是否保留空白
    pub preserve_aspect_ratio: bool,
}

impl SvgConfig {
    /// 创建新的 SVG 配置
    pub fn new() -> Self {
        Self {
            version: SvgVersion::V1_1,
            view_box: ViewBox::standard(),
            width: 100.0,
            height: 100.0,
            preserve_aspect_ratio: true,
        }
    }

    /// 设置版本
    #[allow(dead_code)]
    pub fn with_version(mut self, version: SvgVersion) -> Self {
        self.version = version;
        self
    }

    /// 设置视口框
    #[allow(dead_code)]
    pub fn with_view_box(mut self, view_box: ViewBox) -> Self {
        self.view_box = view_box;
        self
    }

    /// 设置尺寸
    #[allow(dead_code)]
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

impl Default for SvgConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_box_new() {
        let vb = ViewBox::new(0.0, 0.0, 100.0, 100.0);
        assert_eq!(vb.width, 100.0);
    }

    #[test]
    fn test_view_box_standard() {
        let vb = ViewBox::standard();
        assert_eq!(vb.x, 0.0);
        assert_eq!(vb.y, 0.0);
    }

    #[test]
    fn test_svg_config_new() {
        let config = SvgConfig::new();
        assert_eq!(config.version, SvgVersion::V1_1);
        assert_eq!(config.width, 100.0);
    }

    #[test]
    fn test_svg_config_with_size() {
        let config = SvgConfig::new().with_size(200.0, 150.0);
        assert_eq!(config.width, 200.0);
        assert_eq!(config.height, 150.0);
    }

    #[test]
    fn test_svg_config_serialization() {
        let config = SvgConfig::new();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_svg_config_deserialization() {
        let json = r#"{"version":"V1_1","width":100.0,"height":100.0,"view_box":{"x":0.0,"y":0.0,"width":100.0,"height":100.0},"preserve_aspect_ratio":true}"#;
        let config: SvgConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.width, 100.0);
    }

    #[test]
    fn test_svg_config_default() {
        let config = SvgConfig::default();
        assert_eq!(config.version, SvgVersion::V1_1);
        assert_eq!(config.width, 100.0);
    }

    #[test]
    fn test_svg_config_with_version() {
        let config = SvgConfig::new().with_version(SvgVersion::V2_0);
        assert_eq!(config.version, SvgVersion::V2_0);
    }

    #[test]
    fn test_svg_config_with_view_box() {
        let vb = ViewBox::new(0.0, 0.0, 200.0, 200.0);
        let config = SvgConfig::new().with_view_box(vb);
        assert_eq!(config.view_box.width, 200.0);
    }

    #[test]
    fn test_svg_config_chaining() {
        let config = SvgConfig::new()
            .with_size(300.0, 200.0)
            .with_version(SvgVersion::V2_0);
        assert_eq!(config.width, 300.0);
        assert_eq!(config.height, 200.0);
        assert_eq!(config.version, SvgVersion::V2_0);
    }

    #[test]
    fn test_svg_version_v1_1() {
        assert_eq!(SvgVersion::V1_1, SvgVersion::V1_1);
    }

    #[test]
    fn test_svg_version_v2_0() {
        assert_eq!(SvgVersion::V2_0, SvgVersion::V2_0);
    }

    #[test]
    fn test_svg_version_serialization() {
        let version = SvgVersion::V1_1;
        let json = serde_json::to_string(&version);
        assert!(json.is_ok());
    }

    #[test]
    fn test_svg_version_deserialization() {
        let json = r#""V1_1""#;
        let version: SvgVersion = serde_json::from_str(json).unwrap();
        assert_eq!(version, SvgVersion::V1_1);
    }

    #[test]
    fn test_view_box_serialization() {
        let vb = ViewBox::new(0.0, 0.0, 100.0, 100.0);
        let json = serde_json::to_string(&vb);
        assert!(json.is_ok());
    }

    #[test]
    fn test_view_box_deserialization() {
        let json = r#"{"x":0.0,"y":0.0,"width":100.0,"height":100.0}"#;
        let vb: ViewBox = serde_json::from_str(json).unwrap();
        assert_eq!(vb.width, 100.0);
    }

    #[test]
    fn test_view_box_with_coordinates() {
        let vb = ViewBox::new(10.0, 20.0, 100.0, 100.0);
        assert_eq!(vb.x, 10.0);
        assert_eq!(vb.y, 20.0);
    }

    #[test]
    fn test_svg_config_large_dimensions() {
        let config = SvgConfig::new().with_size(10000.0, 10000.0);
        assert_eq!(config.width, 10000.0);
        assert_eq!(config.height, 10000.0);
    }

    #[test]
    fn test_svg_config_zero_dimensions() {
        let config = SvgConfig::new().with_size(0.0, 0.0);
        assert_eq!(config.width, 0.0);
        assert_eq!(config.height, 0.0);
    }

    #[test]
    fn test_view_box_negative_coordinates() {
        let vb = ViewBox::new(-10.0, -20.0, 100.0, 100.0);
        assert_eq!(vb.x, -10.0);
        assert_eq!(vb.y, -20.0);
    }
}
