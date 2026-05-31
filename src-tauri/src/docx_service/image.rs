use serde::{Deserialize, Serialize};

/// 图片位置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImagePosition {
    /// 内联（与文本在同一行）
    Inline,
    /// 浮动
    Floating,
    /// 绝对位置
    Absolute { x: f64, y: f64 },
}

/// 图片尺寸
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageSize {
    /// 原始尺寸
    Original,
    /// 自定义尺寸（宽度、高度，单位：点）
    Custom { width: f64, height: f64 },
    /// 按宽度缩放
    ScaleWidth { width: f64 },
    /// 按高度缩放
    ScaleHeight { height: f64 },
    /// 按比例缩放
    Scale { ratio: f64 },
}

/// 环绕方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WrapType {
    /// 内联
    Inline,
    /// 无环绕
    None,
    /// 四周环绕
    Square,
    /// 紧密环绕
    Tight,
    /// 穿透环绕
    Through,
    /// 上下环绕
    TopAndBottom,
    /// 衬于文字下方
    Behind,
    /// 浮于文字上方
    InFront,
}

/// 图片
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    /// 图片 ID
    pub id: String,
    /// 图片数据
    pub data: Vec<u8>,
    /// 图片格式
    pub format: String,
    /// 图片位置
    pub position: ImagePosition,
    /// 图片尺寸
    pub size: ImageSize,
    /// 环绕方式
    pub wrap_type: WrapType,
    /// 旋转角度（度）
    pub rotation: f64,
    /// 透明度（0.0 - 1.0）
    pub opacity: f64,
    /// 替代文本
    pub alt_text: String,
}

impl Image {
    /// 创建新的图片
    #[allow(dead_code)]
    pub fn new(id: String, data: Vec<u8>, format: String) -> Self {
        Self {
            id,
            data,
            format,
            position: ImagePosition::Inline,
            size: ImageSize::Original,
            wrap_type: WrapType::Inline,
            rotation: 0.0,
            opacity: 1.0,
            alt_text: String::new(),
        }
    }

    /// 设置位置
    #[allow(dead_code)]
    pub fn with_position(mut self, position: ImagePosition) -> Self {
        self.position = position;
        self
    }

    /// 设置尺寸
    #[allow(dead_code)]
    pub fn with_size(mut self, size: ImageSize) -> Self {
        self.size = size;
        self
    }

    /// 设置环绕方式
    #[allow(dead_code)]
    pub fn with_wrap_type(mut self, wrap_type: WrapType) -> Self {
        self.wrap_type = wrap_type;
        self
    }

    /// 设置旋转
    #[allow(dead_code)]
    pub fn with_rotation(mut self, rotation: f64) -> Self {
        self.rotation = rotation;
        self
    }

    /// 设置透明度
    #[allow(dead_code)]
    pub fn with_opacity(mut self, opacity: f64) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// 设置替代文本
    #[allow(dead_code)]
    pub fn with_alt_text(mut self, alt_text: String) -> Self {
        self.alt_text = alt_text;
        self
    }

    /// 创建内联图片
    #[allow(dead_code)]
    pub fn inline(id: String, data: Vec<u8>, format: String) -> Self {
        Self::new(id, data, format)
            .with_position(ImagePosition::Inline)
            .with_wrap_type(WrapType::Inline)
    }

    /// 创建浮动图片
    #[allow(dead_code)]
    pub fn floating(id: String, data: Vec<u8>, format: String) -> Self {
        Self::new(id, data, format)
            .with_position(ImagePosition::Floating)
            .with_wrap_type(WrapType::Square)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_new() {
        let image = Image::new("1".to_string(), vec![1, 2, 3], "png".to_string());
        assert_eq!(image.id, "1");
        assert_eq!(image.format, "png");
    }

    #[test]
    fn test_image_with_position() {
        let image = Image::new("1".to_string(), vec![], "png".to_string())
            .with_position(ImagePosition::Floating);
        assert_eq!(image.position, ImagePosition::Floating);
    }

    #[test]
    fn test_image_with_size() {
        let size = ImageSize::Custom {
            width: 100.0,
            height: 100.0,
        };
        let image = Image::new("1".to_string(), vec![], "png".to_string()).with_size(size);
        match image.size {
            ImageSize::Custom { width, height } => {
                assert_eq!(width, 100.0);
                assert_eq!(height, 100.0);
            }
            _ => panic!("Expected custom size"),
        }
    }

    #[test]
    fn test_image_with_rotation() {
        let image = Image::new("1".to_string(), vec![], "png".to_string()).with_rotation(45.0);
        assert_eq!(image.rotation, 45.0);
    }

    #[test]
    fn test_image_with_opacity() {
        let image = Image::new("1".to_string(), vec![], "png".to_string()).with_opacity(0.5);
        assert_eq!(image.opacity, 0.5);
    }

    #[test]
    fn test_image_opacity_clamp() {
        let image = Image::new("1".to_string(), vec![], "png".to_string()).with_opacity(1.5);
        assert_eq!(image.opacity, 1.0);
    }

    #[test]
    fn test_image_inline() {
        let image = Image::inline("1".to_string(), vec![], "png".to_string());
        assert_eq!(image.position, ImagePosition::Inline);
        assert_eq!(image.wrap_type, WrapType::Inline);
    }

    #[test]
    fn test_image_floating() {
        let image = Image::floating("1".to_string(), vec![], "png".to_string());
        assert_eq!(image.position, ImagePosition::Floating);
        assert_eq!(image.wrap_type, WrapType::Square);
    }

    #[test]
    fn test_image_chaining() {
        let image = Image::new("1".to_string(), vec![], "png".to_string())
            .with_rotation(30.0)
            .with_opacity(0.8)
            .with_alt_text("Test image".to_string());
        assert_eq!(image.rotation, 30.0);
        assert_eq!(image.alt_text, "Test image");
    }

    #[test]
    fn test_image_serialization() {
        let image = Image::new("1".to_string(), vec![], "png".to_string());
        let json = serde_json::to_string(&image);
        assert!(json.is_ok());
    }
}
