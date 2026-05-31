use serde::{Deserialize, Serialize};

/// 渲染质量
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RenderQuality {
    /// 低质量
    Low,
    /// 中等质量
    Medium,
    /// 高质量
    High,
    /// 超高质量
    Ultra,
}

/// 渲染模式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RenderMode {
    /// 标准渲染
    Standard,
    /// 抗锯齿
    Antialiased,
    /// 子像素渲染
    Subpixel,
}

/// PNG 渲染器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PngRenderer {
    /// 渲染质量
    pub quality: RenderQuality,
    /// 渲染模式
    pub mode: RenderMode,
    /// 是否使用硬件加速
    pub hardware_acceleration: bool,
}

impl PngRenderer {
    /// 创建新的渲染器
    pub fn new() -> Self {
        Self {
            quality: RenderQuality::High,
            mode: RenderMode::Antialiased,
            hardware_acceleration: true,
        }
    }

    /// 设置质量
    #[allow(dead_code)]
    pub fn with_quality(mut self, quality: RenderQuality) -> Self {
        self.quality = quality;
        self
    }

    /// 设置模式
    #[allow(dead_code)]
    pub fn with_mode(mut self, mode: RenderMode) -> Self {
        self.mode = mode;
        self
    }

    /// 设置硬件加速
    #[allow(dead_code)]
    pub fn with_hardware_acceleration(mut self, enabled: bool) -> Self {
        self.hardware_acceleration = enabled;
        self
    }
}

impl Default for PngRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_png_renderer_new() {
        let renderer = PngRenderer::new();
        assert_eq!(renderer.quality, RenderQuality::High);
    }

    #[test]
    fn test_png_renderer_with_quality() {
        let renderer = PngRenderer::new().with_quality(RenderQuality::Ultra);
        assert_eq!(renderer.quality, RenderQuality::Ultra);
    }

    #[test]
    fn test_png_renderer_serialization() {
        let renderer = PngRenderer::new();
        let json = serde_json::to_string(&renderer);
        assert!(json.is_ok());
    }

    #[test]
    fn test_png_renderer_deserialization() {
        let json = r#"{"quality":"High","mode":"Antialiased","hardware_acceleration":true}"#;
        let renderer: PngRenderer = serde_json::from_str(json).unwrap();
        assert_eq!(renderer.quality, RenderQuality::High);
    }

    #[test]
    fn test_png_renderer_default() {
        let renderer = PngRenderer::default();
        assert_eq!(renderer.quality, RenderQuality::High);
    }

    #[test]
    fn test_render_quality_low() {
        assert_eq!(RenderQuality::Low, RenderQuality::Low);
    }

    #[test]
    fn test_render_quality_medium() {
        assert_eq!(RenderQuality::Medium, RenderQuality::Medium);
    }

    #[test]
    fn test_render_quality_high() {
        assert_eq!(RenderQuality::High, RenderQuality::High);
    }

    #[test]
    fn test_render_quality_ultra() {
        assert_eq!(RenderQuality::Ultra, RenderQuality::Ultra);
    }

    #[test]
    fn test_render_quality_serialization() {
        let quality = RenderQuality::High;
        let json = serde_json::to_string(&quality);
        assert!(json.is_ok());
    }

    #[test]
    fn test_render_quality_deserialization() {
        let json = r#""High""#;
        let quality: RenderQuality = serde_json::from_str(json).unwrap();
        assert_eq!(quality, RenderQuality::High);
    }

    #[test]
    fn test_png_renderer_with_low_quality() {
        let renderer = PngRenderer::new().with_quality(RenderQuality::Low);
        assert_eq!(renderer.quality, RenderQuality::Low);
    }

    #[test]
    fn test_png_renderer_with_medium_quality() {
        let renderer = PngRenderer::new().with_quality(RenderQuality::Medium);
        assert_eq!(renderer.quality, RenderQuality::Medium);
    }

    #[test]
    fn test_png_renderer_quality_chaining() {
        let renderer = PngRenderer::new()
            .with_quality(RenderQuality::Medium)
            .with_quality(RenderQuality::High);
        assert_eq!(renderer.quality, RenderQuality::High);
    }
}
