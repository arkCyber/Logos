/*!
 * 航空航天级色彩管理系统
 * 实现 CMYK、Pantone、ICC 配置文件等专业色彩管理功能
 */

use serde::{Deserialize, Serialize};

/// 色彩空间
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ColorSpace {
    /// RGB 色彩空间
    RGB,
    /// CMYK 色彩空间
    CMYK,
    /// Lab 色彩空间
    Lab,
    /// HSL 色彩空间
    HSL,
    /// HSV 色彩空间
    HSV,
    /// 灰度
    Grayscale,
}

/// RGB 颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RGBColor {
    pub r: u8, // 0-255
    pub g: u8, // 0-255
    pub b: u8, // 0-255
    pub a: f32, // 0.0-1.0
}

impl RGBColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn with_alpha(mut self, a: f32) -> Self {
        self.a = a;
        self
    }

    /// 转换为十六进制字符串
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// 从十六进制字符串创建
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err("Invalid hex color format".to_string());
        }

        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| "Invalid red component")?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| "Invalid green component")?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| "Invalid blue component")?;

        Ok(Self::new(r, g, b))
    }

    /// 转换为 CSS 字符串
    pub fn to_css(&self) -> String {
        if self.a < 1.0 {
            format!("rgba({}, {}, {}, {:.2})", self.r, self.g, self.b, self.a)
        } else {
            format!("rgb({}, {}, {})", self.r, self.g, self.b)
        }
    }
}

/// CMYK 颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CMYKColor {
    pub c: f32, // 0.0-1.0
    pub m: f32, // 0.0-1.0
    pub y: f32, // 0.0-1.0
    pub k: f32, // 0.0-1.0
}

impl CMYKColor {
    pub fn new(c: f32, m: f32, y: f32, k: f32) -> Self {
        Self { c, m, y, k }
    }

    /// 从 RGB 转换为 CMYK
    pub fn from_rgb(rgb: &RGBColor) -> Self {
        let r = rgb.r as f32 / 255.0;
        let g = rgb.g as f32 / 255.0;
        let b = rgb.b as f32 / 255.0;

        let k = 1.0 - r.max(g).max(b);
        let c = if k == 1.0 { 0.0 } else { (1.0 - r - k) / (1.0 - k) };
        let m = if k == 1.0 { 0.0 } else { (1.0 - g - k) / (1.0 - k) };
        let y = if k == 1.0 { 0.0 } else { (1.0 - b - k) / (1.0 - k) };

        Self { c, m, y, k }
    }

    /// 转换为 RGB
    pub fn to_rgb(&self) -> RGBColor {
        let r = (1.0 - self.c) * (1.0 - self.k);
        let g = (1.0 - self.m) * (1.0 - self.k);
        let b = (1.0 - self.y) * (1.0 - self.k);

        RGBColor::new(
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
        )
    }

    /// 转换为百分比字符串
    pub fn to_percentage(&self) -> String {
        format!(
            "cmyk({:.0}%, {:.0}%, {:.0}%, {:.0}%)",
            self.c * 100.0,
            self.m * 100.0,
            self.y * 100.0,
            self.k * 100.0
        )
    }
}

/// Pantone 颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PantoneColor {
    pub name: String,
    pub code: String,
    pub rgb: RGBColor,
    pub cmyk: CMYKColor,
}

impl PantoneColor {
    pub fn new(name: String, code: String, rgb: RGBColor) -> Self {
        let cmyk = CMYKColor::from_rgb(&rgb);
        Self { name, code, rgb, cmyk }
    }

    /// 获取常用 Pantone 颜色
    pub fn get_common_colors() -> Vec<Self> {
        vec![
            Self::new("Pantone 186 C".to_string(), "186 C".to_string(), RGBColor::new(237, 28, 36)),
            Self::new("Pantone 286 C".to_string(), "286 C".to_string(), RGBColor::new(0, 85, 191)),
            Self::new("Pantone 361 C".to_string(), "361 C".to_string(), RGBColor::new(0, 166, 81)),
            Self::new("Pantone 109 C".to_string(), "109 C".to_string(), RGBColor::new(255, 221, 0)),
            Self::new("Pantone 7406 C".to_string(), "7406 C".to_string(), RGBColor::new(255, 159, 28)),
        ]
    }
}

/// ICC 配置文件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ICCProfile {
    pub name: String,
    pub color_space: ColorSpace,
    pub description: String,
}

impl ICCProfile {
    pub fn new(name: String, color_space: ColorSpace, description: String) -> Self {
        Self {
            name,
            color_space,
            description,
        }
    }

    /// 获取常用 ICC 配置文件
    pub fn get_common_profiles() -> Vec<Self> {
        vec![
            Self::new(
                "sRGB".to_string(),
                ColorSpace::RGB,
                "Standard RGB color space for web and digital displays".to_string(),
            ),
            Self::new(
                "Adobe RGB".to_string(),
                ColorSpace::RGB,
                "Wide gamut RGB color space for professional photography".to_string(),
            ),
            Self::new(
                "U.S. Web Coated (SWOP)".to_string(),
                ColorSpace::CMYK,
                "Standard CMYK profile for web offset printing".to_string(),
            ),
            Self::new(
                "Japan Color 2001 Coated".to_string(),
                ColorSpace::CMYK,
                "Standard CMYK profile for Japanese printing".to_string(),
            ),
        ]
    }
}

/// 色彩管理配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorManagementConfig {
    pub working_space: ColorSpace,
    pub rendering_intent: RenderingIntent,
    pub black_point_compensation: bool,
    pub icc_profile: Option<String>,
}

/// 渲染意图
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RenderingIntent {
    /// 感知意图
    Perceptual,
    /// 相对色度
    RelativeColorimetric,
    /// 饱和度
    Saturation,
    /// 绝对色度
    AbsoluteColorimetric,
}

impl Default for ColorManagementConfig {
    fn default() -> Self {
        Self {
            working_space: ColorSpace::RGB,
            rendering_intent: RenderingIntent::RelativeColorimetric,
            black_point_compensation: true,
            icc_profile: Some("sRGB".to_string()),
        }
    }
}

/// 色彩管理系统
pub struct ColorManagementSystem {
    config: ColorManagementConfig,
    pantone_colors: Vec<PantoneColor>,
    icc_profiles: Vec<ICCProfile>,
}

impl ColorManagementSystem {
    pub fn new(config: ColorManagementConfig) -> Self {
        Self {
            config,
            pantone_colors: PantoneColor::get_common_colors(),
            icc_profiles: ICCProfile::get_common_profiles(),
        }
    }

    /// RGB 转 CMYK
    pub fn rgb_to_cmyk(&self, rgb: &RGBColor) -> CMYKColor {
        CMYKColor::from_rgb(rgb)
    }

    /// CMYK 转 RGB
    pub fn cmyk_to_rgb(&self, cmyk: &CMYKColor) -> RGBColor {
        cmyk.to_rgb()
    }

    /// 查找最接近的 Pantone 颜色
    pub fn find_nearest_pantone(&self, rgb: &RGBColor) -> Option<&PantoneColor> {
        let mut min_distance = f32::MAX;
        let mut nearest = None;

        for pantone in &self.pantone_colors {
            let distance = self.color_distance(rgb, &pantone.rgb);
            if distance < min_distance {
                min_distance = distance;
                nearest = Some(pantone);
            }
        }

        nearest
    }

    /// 计算颜色距离（欧几里得距离）
    fn color_distance(&self, c1: &RGBColor, c2: &RGBColor) -> f32 {
        let dr = c1.r as f32 - c2.r as f32;
        let dg = c1.g as f32 - c2.g as f32;
        let db = c1.b as f32 - c2.b as f32;
        (dr * dr + dg * dg + db * db).sqrt()
    }

    /// 获取 ICC 配置文件
    pub fn get_icc_profile(&self, name: &str) -> Option<&ICCProfile> {
        self.icc_profiles.iter().find(|p| p.name == name)
    }

    /// 生成色彩管理配置 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();
        
        // 设置工作色彩空间
        match self.config.working_space {
            ColorSpace::RGB => {
                typst.push_str("#set text(fill: rgb)\n");
            }
            ColorSpace::CMYK => {
                typst.push_str("#set text(fill: cmyk)\n");
            }
            ColorSpace::Grayscale => {
                typst.push_str("#set text(fill: gray)\n");
            }
            _ => {
                typst.push_str("#set text(fill: rgb)\n");
            }
        }
        
        typst
    }
}

impl Default for ColorManagementSystem {
    fn default() -> Self {
        Self::new(ColorManagementConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_color_creation() {
        let color = RGBColor::new(255, 0, 0);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_rgb_to_hex() {
        let color = RGBColor::new(255, 0, 0);
        assert_eq!(color.to_hex(), "#FF0000");
    }

    #[test]
    fn test_rgb_from_hex() {
        let color = RGBColor::from_hex("#FF0000").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_rgb_to_css() {
        let color = RGBColor::new(255, 0, 0);
        assert_eq!(color.to_css(), "rgb(255, 0, 0)");
    }

    #[test]
    fn test_rgb_with_alpha() {
        let color = RGBColor::new(255, 0, 0).with_alpha(0.5);
        assert_eq!(color.a, 0.5);
        assert!(color.to_css().contains("rgba"));
    }

    #[test]
    fn test_cmyk_color_creation() {
        let color = CMYKColor::new(0.0, 1.0, 1.0, 0.0);
        assert_eq!(color.c, 0.0);
        assert_eq!(color.m, 1.0);
    }

    #[test]
    fn test_cmyk_from_rgb() {
        let rgb = RGBColor::new(255, 0, 0);
        let cmyk = CMYKColor::from_rgb(&rgb);
        assert!(cmyk.m > 0.9); // Red should have high magenta
        assert!(cmyk.y > 0.9); // Red should have high yellow
    }

    #[test]
    fn test_cmyk_to_rgb() {
        let cmyk = CMYKColor::new(0.0, 1.0, 1.0, 0.0);
        let rgb = cmyk.to_rgb();
        assert!(rgb.r > 200); // Should be close to red
    }

    #[test]
    fn test_cmyk_to_percentage() {
        let cmyk = CMYKColor::new(0.5, 0.5, 0.5, 0.5);
        let percentage = cmyk.to_percentage();
        assert!(percentage.contains("50%"));
    }

    #[test]
    fn test_pantone_color_creation() {
        let rgb = RGBColor::new(237, 28, 36);
        let pantone = PantoneColor::new("Test".to_string(), "T".to_string(), rgb);
        assert_eq!(pantone.name, "Test");
    }

    #[test]
    fn test_pantone_get_common_colors() {
        let colors = PantoneColor::get_common_colors();
        assert!(!colors.is_empty());
    }

    #[test]
    fn test_icc_profile_creation() {
        let profile = ICCProfile::new(
            "Test".to_string(),
            ColorSpace::RGB,
            "Test profile".to_string(),
        );
        assert_eq!(profile.name, "Test");
    }

    #[test]
    fn test_icc_profile_get_common_profiles() {
        let profiles = ICCProfile::get_common_profiles();
        assert!(!profiles.is_empty());
    }

    #[test]
    fn test_color_management_config_default() {
        let config = ColorManagementConfig::default();
        assert_eq!(config.working_space, ColorSpace::RGB);
    }

    #[test]
    fn test_color_management_system_creation() {
        let system = ColorManagementSystem::default();
        assert!(!system.pantone_colors.is_empty());
    }

    #[test]
    fn test_rgb_to_cmyk_conversion() {
        let system = ColorManagementSystem::default();
        let rgb = RGBColor::new(255, 0, 0);
        let cmyk = system.rgb_to_cmyk(&rgb);
        assert!(cmyk.m > 0.9);
    }

    #[test]
    fn test_cmyk_to_rgb_conversion() {
        let system = ColorManagementSystem::default();
        let cmyk = CMYKColor::new(0.0, 1.0, 1.0, 0.0);
        let rgb = system.cmyk_to_rgb(&cmyk);
        assert!(rgb.r > 200);
    }

    #[test]
    fn test_find_nearest_pantone() {
        let system = ColorManagementSystem::default();
        let rgb = RGBColor::new(237, 28, 36);
        let nearest = system.find_nearest_pantone(&rgb);
        assert!(nearest.is_some());
    }

    #[test]
    fn test_get_icc_profile() {
        let system = ColorManagementSystem::default();
        let profile = system.get_icc_profile("sRGB");
        assert!(profile.is_some());
    }

    #[test]
    fn test_to_typst() {
        let system = ColorManagementSystem::default();
        let typst = system.to_typst();
        assert!(typst.contains("#set text(fill:"));
    }
}
