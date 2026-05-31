use serde::{Deserialize, Serialize};

/// 图像位置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImagePosition {
    /// 居中
    Center,
    /// 左上角
    TopLeft,
    /// 右上角
    TopRight,
    /// 左下角
    BottomLeft,
    /// 右下角
    BottomRight,
    /// 自定义位置（X, Y 坐标，单位：点）
    Custom { x: f64, y: f64 },
}

/// 图像尺寸
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// 图像效果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageEffect {
    /// 亮度（-100 到 100）
    pub brightness: i32,
    /// 对比度（-100 到 100）
    pub contrast: i32,
    /// 饱和度（-100 到 100）
    pub saturation: i32,
    /// 模糊（0 到 100）
    pub blur: u32,
    /// 是否灰度
    pub grayscale: bool,
    /// 是否透明
    pub transparent: bool,
}

impl ImageEffect {
    /// 创建默认效果
    pub fn new() -> Self {
        Self {
            brightness: 0,
            contrast: 0,
            saturation: 0,
            blur: 0,
            grayscale: false,
            transparent: false,
        }
    }

    /// 设置亮度
    pub fn with_brightness(mut self, brightness: i32) -> Self {
        self.brightness = brightness.clamp(-100, 100);
        self
    }

    /// 设置对比度
    pub fn with_contrast(mut self, contrast: i32) -> Self {
        self.contrast = contrast.clamp(-100, 100);
        self
    }

    /// 设置灰度
    pub fn with_grayscale(mut self, grayscale: bool) -> Self {
        self.grayscale = grayscale;
        self
    }
}

impl Default for ImageEffect {
    fn default() -> Self {
        Self::new()
    }
}

/// 图像元素
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageElement {
    /// 图像 ID
    pub id: String,
    /// 图像数据
    pub data: Vec<u8>,
    /// 图像格式
    pub format: String,
    /// 图像位置
    pub position: ImagePosition,
    /// 图像尺寸
    pub size: ImageSize,
    /// 图像效果
    pub effect: ImageEffect,
    /// 旋转角度（度）
    pub rotation: f64,
    /// 透明度（0.0 - 1.0）
    pub opacity: f64,
    /// 是否裁剪
    pub cropped: bool,
}

impl ImageElement {
    /// 创建新的图像元素
    pub fn new(id: String, data: Vec<u8>, format: String) -> Self {
        Self {
            id,
            data,
            format,
            position: ImagePosition::Center,
            size: ImageSize::Original,
            effect: ImageEffect::new(),
            rotation: 0.0,
            opacity: 1.0,
            cropped: false,
        }
    }

    /// 设置位置
    pub fn with_position(mut self, position: ImagePosition) -> Self {
        self.position = position;
        self
    }

    /// 设置尺寸
    pub fn with_size(mut self, size: ImageSize) -> Self {
        self.size = size;
        self
    }

    /// 设置效果
    pub fn with_effect(mut self, effect: ImageEffect) -> Self {
        self.effect = effect;
        self
    }

    /// 设置旋转
    pub fn with_rotation(mut self, rotation: f64) -> Self {
        self.rotation = rotation;
        self
    }

    /// 设置透明度
    pub fn with_opacity(mut self, opacity: f64) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// 从文件路径创建（占位符）
    pub fn from_file(id: String, _path: String) -> Self {
        Self::new(id, Vec::new(), "png".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_effect_new() {
        let effect = ImageEffect::new();
        assert_eq!(effect.brightness, 0);
        assert_eq!(effect.contrast, 0);
    }

    #[test]
    fn test_image_effect_with_brightness() {
        let effect = ImageEffect::new().with_brightness(50);
        assert_eq!(effect.brightness, 50);
    }

    #[test]
    fn test_image_effect_brightness_clamp() {
        let effect = ImageEffect::new().with_brightness(150);
        assert_eq!(effect.brightness, 100);
    }

    #[test]
    fn test_image_effect_with_grayscale() {
        let effect = ImageEffect::new().with_grayscale(true);
        assert!(effect.grayscale);
    }

    #[test]
    fn test_image_element_new() {
        let image = ImageElement::new("1".to_string(), vec![1, 2, 3], "png".to_string());
        assert_eq!(image.id, "1");
        assert_eq!(image.format, "png");
    }

    #[test]
    fn test_image_element_with_position() {
        let image = ImageElement::new("1".to_string(), vec![], "png".to_string())
            .with_position(ImagePosition::TopLeft);
        assert_eq!(image.position, ImagePosition::TopLeft);
    }

    #[test]
    fn test_image_element_with_size() {
        let size = ImageSize::Custom {
            width: 100.0,
            height: 100.0,
        };
        let image = ImageElement::new("1".to_string(), vec![], "png".to_string()).with_size(size);
        assert_eq!(
            image.size,
            ImageSize::Custom {
                width: 100.0,
                height: 100.0
            }
        );
    }

    #[test]
    fn test_image_element_chaining() {
        let image = ImageElement::new("1".to_string(), vec![], "png".to_string())
            .with_position(ImagePosition::Center)
            .with_rotation(45.0)
            .with_opacity(0.8);
        assert_eq!(image.rotation, 45.0);
        assert_eq!(image.opacity, 0.8);
    }

    #[test]
    fn test_image_element_serialization() {
        let image = ImageElement::new("1".to_string(), vec![], "png".to_string());
        let json = serde_json::to_string(&image);
        assert!(json.is_ok());
    }
}
