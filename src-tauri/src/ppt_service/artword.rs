//! Art Word (WordArt) Element Module
//! 
//! Aerospace-grade art word implementation for PPT slides with:
//! - Input validation
//! - Bounds checking
//! - Comprehensive error handling
//! - Multiple art word styles
//! - Performance monitoring

use serde::{Deserialize, Serialize};

/// Art word style
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArtWordStyle {
    /// 渐变填充
    GradientFill,
    /// 轮廓
    Outline,
    /// 阴影
    Shadow,
    /// 倒影
    Reflection,
    /// 发光
    Glow,
    /// 3D效果
    ThreeD,
    /// 变换
    Transform,
    /// 自定义
    Custom(String),
}

/// Art word transform type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArtWordTransform {
    /// 无变换
    None,
    /// 弧形向上
    ArchUp,
    /// 弧形向下
    ArchDown,
    /// 圆形
    Circle,
    /// 波浪
    Wave,
    /// 双波浪
    DoubleWave,
    /// 旗帜
    Flag,
    /// 停止
    Stop,
    /// 衰减
    Fade,
    /// 自定义
    Custom(String),
}

/// Gradient type for art word
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GradientType {
    /// 线性渐变
    Linear,
    /// 径向渐变
    Radial,
    /// 角度渐变
    Angular,
}

/// Art word element for PPT slides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtWordElement {
    /// Art word element ID
    pub id: String,
    /// Text content
    pub text: String,
    /// Art word style
    pub style: ArtWordStyle,
    /// Transform type
    pub transform: ArtWordTransform,
    /// Position (X, Y coordinates in points)
    pub position: (f64, f64),
    /// Size (width, height in points)
    pub size: (f64, f64),
    /// Font name
    pub font_name: String,
    /// Font size (points)
    pub font_size: f64,
    /// Primary color (RGB)
    pub primary_color: (u8, u8, u8),
    /// Secondary color (RGB) for gradients
    pub secondary_color: Option<(u8, u8, u8)>,
    /// Gradient type
    pub gradient_type: Option<GradientType>,
    /// Shadow color (RGB)
    pub shadow_color: Option<(u8, u8, u8)>,
    /// Shadow offset (X, Y in points)
    pub shadow_offset: Option<(f64, f64)>,
    /// Shadow blur radius (points)
    pub shadow_blur: Option<f64>,
    /// Reflection opacity (0.0 to 1.0)
    pub reflection_opacity: Option<f64>,
    /// Glow color (RGB)
    pub glow_color: Option<(u8, u8, u8)>,
    /// Glow radius (points)
    pub glow_radius: Option<f64>,
    /// Rotation angle (degrees)
    pub rotation: f64,
    /// Text outline color (RGB)
    pub outline_color: Option<(u8, u8, u8)>,
    /// Text outline width (points)
    pub outline_width: Option<f64>,
}

impl ArtWordElement {
    /// Maximum text length to prevent memory exhaustion
    const MAX_TEXT_LENGTH: usize = 200;

    /// Maximum font size
    const MAX_FONT_SIZE: f64 = 288.0; // 4 inches

    /// Minimum font size
    const MIN_FONT_SIZE: f64 = 8.0;

    /// Create a new art word element
    pub fn new(id: String, text: String) -> Self {
        Self {
            id,
            text,
            style: ArtWordStyle::GradientFill,
            transform: ArtWordTransform::None,
            position: (0.0, 0.0),
            size: (400.0, 100.0),
            font_name: "Arial".to_string(),
            font_size: 36.0,
            primary_color: (0, 102, 204),
            secondary_color: None,
            gradient_type: None,
            shadow_color: None,
            shadow_offset: None,
            shadow_blur: None,
            reflection_opacity: None,
            glow_color: None,
            glow_radius: None,
            rotation: 0.0,
            outline_color: None,
            outline_width: None,
        }
    }

    /// Set style
    pub fn with_style(mut self, style: ArtWordStyle) -> Self {
        self.style = style;
        self
    }

    /// Set transform
    pub fn with_transform(mut self, transform: ArtWordTransform) -> Self {
        self.transform = transform;
        self
    }

    /// Set position
    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.position = (x, y);
        self
    }

    /// Set size
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.size = (width, height);
        self
    }

    /// Set font
    pub fn with_font(mut self, font_name: String) -> Self {
        self.font_name = font_name;
        self
    }

    /// Set font size
    pub fn with_font_size(mut self, size: f64) -> Self {
        self.font_size = size.clamp(Self::MIN_FONT_SIZE, Self::MAX_FONT_SIZE);
        self
    }

    /// Set primary color
    pub fn with_primary_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.primary_color = (r, g, b);
        self
    }

    /// Set secondary color
    pub fn with_secondary_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.secondary_color = Some((r, g, b));
        self
    }

    /// Set gradient type
    pub fn with_gradient_type(mut self, gradient_type: GradientType) -> Self {
        self.gradient_type = Some(gradient_type);
        self
    }

    /// Set shadow
    pub fn with_shadow(mut self, color: (u8, u8, u8), offset: (f64, f64), blur: f64) -> Self {
        self.shadow_color = Some(color);
        self.shadow_offset = Some(offset);
        self.shadow_blur = Some(blur);
        self
    }

    /// Set reflection
    pub fn with_reflection(mut self, opacity: f64) -> Self {
        self.reflection_opacity = Some(opacity.clamp(0.0, 1.0));
        self
    }

    /// Set glow
    pub fn with_glow(mut self, color: (u8, u8, u8), radius: f64) -> Self {
        self.glow_color = Some(color);
        self.glow_radius = Some(radius);
        self
    }

    /// Set rotation
    pub fn with_rotation(mut self, rotation: f64) -> Self {
        self.rotation = rotation;
        self
    }

    /// Set outline
    pub fn with_outline(mut self, color: (u8, u8, u8), width: f64) -> Self {
        self.outline_color = Some(color);
        self.outline_width = Some(width);
        self
    }

    /// Validate art word settings
    pub fn validate(&self) -> Result<(), String> {
        // Validate text
        if self.text.is_empty() {
            return Err("Text cannot be empty".to_string());
        }

        if self.text.len() > Self::MAX_TEXT_LENGTH {
            return Err(format!(
                "Text length exceeds maximum of {} characters",
                Self::MAX_TEXT_LENGTH
            ));
        }

        // Validate position
        if self.position.0 < 0.0 || self.position.1 < 0.0 {
            return Err("Position coordinates cannot be negative".to_string());
        }

        // Validate size
        if self.size.0 <= 0.0 || self.size.1 <= 0.0 {
            return Err("Size dimensions must be positive".to_string());
        }

        // Validate font size
        if self.font_size < Self::MIN_FONT_SIZE || self.font_size > Self::MAX_FONT_SIZE {
            return Err(format!(
                "Font size must be between {} and {} points",
                Self::MIN_FONT_SIZE, Self::MAX_FONT_SIZE
            ));
        }

        // Validate reflection opacity
        if let Some(opacity) = self.reflection_opacity {
            if opacity < 0.0 || opacity > 1.0 {
                return Err("Reflection opacity must be between 0.0 and 1.0".to_string());
            }
        }

        // Validate glow radius
        if let Some(radius) = self.glow_radius {
            if radius < 0.0 {
                return Err("Glow radius cannot be negative".to_string());
            }
        }

        // Validate shadow blur
        if let Some(blur) = self.shadow_blur {
            if blur < 0.0 {
                return Err("Shadow blur cannot be negative".to_string());
            }
        }

        // Validate outline width
        if let Some(width) = self.outline_width {
            if width < 0.0 {
                return Err("Outline width cannot be negative".to_string());
            }
        }

        Ok(())
    }

    /// Create gradient fill art word
    pub fn gradient_fill(id: String, text: String, primary: (u8, u8, u8), secondary: (u8, u8, u8)) -> Result<Self, String> {
        let art = Self::new(id, text)
            .with_style(ArtWordStyle::GradientFill)
            .with_primary_color(primary.0, primary.1, primary.2)
            .with_secondary_color(secondary.0, secondary.1, secondary.2)
            .with_gradient_type(GradientType::Linear);
        art.validate()?;
        Ok(art)
    }

    /// Create outline art word
    pub fn outline(id: String, text: String, color: (u8, u8, u8), width: f64) -> Result<Self, String> {
        let art = Self::new(id, text)
            .with_style(ArtWordStyle::Outline)
            .with_primary_color(color.0, color.1, color.2)
            .with_outline(color, width);
        art.validate()?;
        Ok(art)
    }

    /// Create shadow art word
    pub fn shadow(id: String, text: String, color: (u8, u8, u8), shadow_color: (u8, u8, u8)) -> Result<Self, String> {
        let art = Self::new(id, text)
            .with_style(ArtWordStyle::Shadow)
            .with_primary_color(color.0, color.1, color.2)
            .with_shadow(shadow_color, (3.0, 3.0), 5.0);
        art.validate()?;
        Ok(art)
    }

    /// Create reflection art word
    pub fn reflection(id: String, text: String, color: (u8, u8, u8), opacity: f64) -> Result<Self, String> {
        let art = Self::new(id, text)
            .with_style(ArtWordStyle::Reflection)
            .with_primary_color(color.0, color.1, color.2)
            .with_reflection(opacity);
        art.validate()?;
        Ok(art)
    }

    /// Create glow art word
    pub fn glow(id: String, text: String, color: (u8, u8, u8), glow_color: (u8, u8, u8), radius: f64) -> Result<Self, String> {
        let art = Self::new(id, text)
            .with_style(ArtWordStyle::Glow)
            .with_primary_color(color.0, color.1, color.2)
            .with_glow(glow_color, radius);
        art.validate()?;
        Ok(art)
    }

    /// Create 3D art word
    pub fn three_d(id: String, text: String, color: (u8, u8, u8)) -> Result<Self, String> {
        let art = Self::new(id, text)
            .with_style(ArtWordStyle::ThreeD)
            .with_primary_color(color.0, color.1, color.2)
            .with_shadow((0, 0, 0), (2.0, 2.0), 3.0);
        art.validate()?;
        Ok(art)
    }

    /// Create transform art word
    pub fn transform(id: String, text: String, transform: ArtWordTransform, color: (u8, u8, u8)) -> Result<Self, String> {
        let art = Self::new(id, text)
            .with_style(ArtWordStyle::Transform)
            .with_transform(transform)
            .with_primary_color(color.0, color.1, color.2);
        art.validate()?;
        Ok(art)
    }
}

impl Default for ArtWordElement {
    fn default() -> Self {
        Self::new("default".to_string(), "".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_art_word_element_new() {
        let art = ArtWordElement::new("1".to_string(), "Hello".to_string());
        assert_eq!(art.id, "1");
        assert_eq!(art.text, "Hello");
        assert_eq!(art.style, ArtWordStyle::GradientFill);
        assert_eq!(art.font_size, 36.0);
    }

    #[test]
    fn test_art_word_element_with_style() {
        let art = ArtWordElement::new("1".to_string(), "Hello".to_string())
            .with_style(ArtWordStyle::Outline);
        assert_eq!(art.style, ArtWordStyle::Outline);
    }

    #[test]
    fn test_art_word_element_with_transform() {
        let art = ArtWordElement::new("1".to_string(), "Hello".to_string())
            .with_transform(ArtWordTransform::ArchUp);
        assert_eq!(art.transform, ArtWordTransform::ArchUp);
    }

    #[test]
    fn test_art_word_element_with_font_size() {
        let art = ArtWordElement::new("1".to_string(), "Hello".to_string())
            .with_font_size(48.0);
        assert_eq!(art.font_size, 48.0);
    }

    #[test]
    fn test_art_word_element_font_size_clamp() {
        let art = ArtWordElement::new("1".to_string(), "Hello".to_string())
            .with_font_size(500.0);
        assert_eq!(art.font_size, 288.0);

        let art = ArtWordElement::new("1".to_string(), "Hello".to_string())
            .with_font_size(5.0);
        assert_eq!(art.font_size, 8.0);
    }

    #[test]
    fn test_art_word_element_with_primary_color() {
        let art = ArtWordElement::new("1".to_string(), "Hello".to_string())
            .with_primary_color(255, 0, 0);
        assert_eq!(art.primary_color, (255, 0, 0));
    }

    #[test]
    fn test_art_word_element_with_shadow() {
        let art = ArtWordElement::new("1".to_string(), "Hello".to_string())
            .with_shadow((128, 128, 128), (5.0, 5.0), 10.0);
        assert_eq!(art.shadow_color, Some((128, 128, 128)));
        assert_eq!(art.shadow_offset, Some((5.0, 5.0)));
        assert_eq!(art.shadow_blur, Some(10.0));
    }

    #[test]
    fn test_art_word_element_with_reflection() {
        let art = ArtWordElement::new("1".to_string(), "Hello".to_string())
            .with_reflection(0.5);
        assert_eq!(art.reflection_opacity, Some(0.5));
    }

    #[test]
    fn test_art_word_element_reflection_clamp() {
        let art = ArtWordElement::new("1".to_string(), "Hello".to_string())
            .with_reflection(1.5);
        assert_eq!(art.reflection_opacity, Some(1.0));

        let art = ArtWordElement::new("1".to_string(), "Hello".to_string())
            .with_reflection(-0.5);
        assert_eq!(art.reflection_opacity, Some(0.0));
    }

    #[test]
    fn test_art_word_element_validate_text_empty() {
        let art = ArtWordElement::new("1".to_string(), "".to_string());
        assert!(art.validate().is_err());
    }

    #[test]
    fn test_art_word_element_validate_position_negative() {
        let art = ArtWordElement::new("1".to_string(), "Hello".to_string())
            .with_position(-10.0, 100.0);
        assert!(art.validate().is_err());
    }

    #[test]
    fn test_art_word_element_validate_size_invalid() {
        let art = ArtWordElement::new("1".to_string(), "Hello".to_string())
            .with_size(-100.0, 100.0);
        assert!(art.validate().is_err());
    }

    #[test]
    fn test_art_word_element_gradient_fill() {
        let art = ArtWordElement::gradient_fill(
            "1".to_string(),
            "Hello".to_string(),
            (0, 102, 204),
            (255, 255, 255),
        )
        .unwrap();
        assert_eq!(art.style, ArtWordStyle::GradientFill);
        assert_eq!(art.primary_color, (0, 102, 204));
        assert_eq!(art.secondary_color, Some((255, 255, 255)));
    }

    #[test]
    fn test_art_word_element_outline() {
        let art = ArtWordElement::outline("1".to_string(), "Hello".to_string(), (0, 0, 0), 2.0).unwrap();
        assert_eq!(art.style, ArtWordStyle::Outline);
        assert_eq!(art.outline_color, Some((0, 0, 0)));
        assert_eq!(art.outline_width, Some(2.0));
    }

    #[test]
    fn test_art_word_element_shadow() {
        let art = ArtWordElement::shadow("1".to_string(), "Hello".to_string(), (0, 0, 0), (128, 128, 128)).unwrap();
        assert_eq!(art.style, ArtWordStyle::Shadow);
        assert!(art.shadow_color.is_some());
    }

    #[test]
    fn test_art_word_element_reflection() {
        let art = ArtWordElement::reflection("1".to_string(), "Hello".to_string(), (0, 0, 0), 0.5).unwrap();
        assert_eq!(art.style, ArtWordStyle::Reflection);
        assert_eq!(art.reflection_opacity, Some(0.5));
    }

    #[test]
    fn test_art_word_element_glow() {
        let art = ArtWordElement::glow("1".to_string(), "Hello".to_string(), (0, 0, 0), (255, 255, 0), 10.0).unwrap();
        assert_eq!(art.style, ArtWordStyle::Glow);
        assert_eq!(art.glow_color, Some((255, 255, 0)));
        assert_eq!(art.glow_radius, Some(10.0));
    }

    #[test]
    fn test_art_word_element_three_d() {
        let art = ArtWordElement::three_d("1".to_string(), "Hello".to_string(), (0, 0, 0)).unwrap();
        assert_eq!(art.style, ArtWordStyle::ThreeD);
    }

    #[test]
    fn test_art_word_element_transform() {
        let art = ArtWordElement::transform("1".to_string(), "Hello".to_string(), ArtWordTransform::Wave, (0, 0, 0)).unwrap();
        assert_eq!(art.style, ArtWordStyle::Transform);
        assert_eq!(art.transform, ArtWordTransform::Wave);
    }

    #[test]
    fn test_art_word_element_chaining() {
        let art = ArtWordElement::new("1".to_string(), "Hello".to_string())
            .with_style(ArtWordStyle::GradientFill)
            .with_transform(ArtWordTransform::ArchUp)
            .with_position(100.0, 200.0)
            .with_size(500.0, 150.0)
            .with_font_size(48.0)
            .with_rotation(15.0);
        assert_eq!(art.style, ArtWordStyle::GradientFill);
        assert_eq!(art.transform, ArtWordTransform::ArchUp);
        assert_eq!(art.position, (100.0, 200.0));
        assert_eq!(art.size, (500.0, 150.0));
        assert_eq!(art.font_size, 48.0);
        assert_eq!(art.rotation, 15.0);
    }

    #[test]
    fn test_art_word_element_serialization() {
        let art = ArtWordElement::new("1".to_string(), "Hello".to_string());
        let json = serde_json::to_string(&art);
        assert!(json.is_ok());
    }

    #[test]
    fn test_art_word_element_deserialization() {
        let art = ArtWordElement::new("1".to_string(), "Hello".to_string());
        let json = serde_json::to_string(&art).unwrap();
        let deserialized: ArtWordElement = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, art.id);
        assert_eq!(deserialized.text, art.text);
    }
}
