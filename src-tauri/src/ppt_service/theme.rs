use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 主题颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColor {
    /// 颜色名称
    pub name: String,
    /// RGB 值
    pub rgb: (u8, u8, u8),
    /// 十六进制值
    pub hex: String,
}

impl ThemeColor {
    /// 创建新的主题颜色
    pub fn new(name: String, rgb: (u8, u8, u8)) -> Self {
        let hex = format!("{:02X}{:02X}{:02X}", rgb.0, rgb.1, rgb.2);
        Self { name, rgb, hex }
    }

    /// 从十六进制创建
    pub fn from_hex(name: String, hex: &str) -> Result<Self, String> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err("Invalid hex color".to_string());
        }
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex color".to_string())?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex color".to_string())?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex color".to_string())?;
        Ok(Self::new(name, (r, g, b)))
    }
}

/// 主题字体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeFont {
    /// 字体名称
    pub name: String,
    /// 拉丁字体
    pub latin: String,
    /// 东亚字体
    pub east_asian: String,
    /// 复杂脚本字体
    pub complex_script: String,
}

impl ThemeFont {
    /// 创建新的主题字体
    pub fn new(name: String, latin: String) -> Self {
        Self {
            name,
            latin: latin.clone(),
            east_asian: latin.clone(),
            complex_script: latin,
        }
    }

    /// 设置东亚字体
    pub fn with_east_asian(mut self, font: String) -> Self {
        self.east_asian = font;
        self
    }

    /// 设置复杂脚本字体
    pub fn with_complex_script(mut self, font: String) -> Self {
        self.complex_script = font;
        self
    }
}

/// 主题效果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeEffect {
    /// 效果名称
    pub name: String,
    /// 效果类型
    pub effect_type: String,
    /// 效果参数
    pub parameters: HashMap<String, String>,
}

impl ThemeEffect {
    /// 创建新的主题效果
    pub fn new(name: String, effect_type: String) -> Self {
        Self {
            name,
            effect_type,
            parameters: HashMap::new(),
        }
    }

    /// 添加参数
    pub fn with_parameter(mut self, key: String, value: String) -> Self {
        self.parameters.insert(key, value);
        self
    }
}

/// PPT 主题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PptTheme {
    /// 主题名称
    pub name: String,
    /// 主题颜色
    pub colors: HashMap<String, ThemeColor>,
    /// 主题字体
    pub fonts: HashMap<String, ThemeFont>,
    /// 主题效果
    pub effects: HashMap<String, ThemeEffect>,
    /// 背景样式
    pub background_style: String,
}

impl PptTheme {
    /// 创建新的主题
    pub fn new(name: String) -> Self {
        Self {
            name,
            colors: HashMap::new(),
            fonts: HashMap::new(),
            effects: HashMap::new(),
            background_style: "solid".to_string(),
        }
    }

    /// 添加颜色
    pub fn with_color(mut self, key: String, color: ThemeColor) -> Self {
        self.colors.insert(key, color);
        self
    }

    /// 添加字体
    pub fn with_font(mut self, key: String, font: ThemeFont) -> Self {
        self.fonts.insert(key, font);
        self
    }

    /// 添加效果
    pub fn with_effect(mut self, key: String, effect: ThemeEffect) -> Self {
        self.effects.insert(key, effect);
        self
    }

    /// 设置背景样式
    pub fn with_background_style(mut self, style: String) -> Self {
        self.background_style = style;
        self
    }

    /// 创建默认主题
    pub fn default_theme() -> Self {
        Self::new("Default".to_string())
            .with_color(
                "accent1".to_string(),
                ThemeColor::new("Accent 1".to_string(), (31, 78, 120)),
            )
            .with_color(
                "accent2".to_string(),
                ThemeColor::new("Accent 2".to_string(), (79, 129, 189)),
            )
            .with_color(
                "accent3".to_string(),
                ThemeColor::new("Accent 3".to_string(), (192, 80, 77)),
            )
            .with_color(
                "accent4".to_string(),
                ThemeColor::new("Accent 4".to_string(), (155, 187, 89)),
            )
            .with_color(
                "accent5".to_string(),
                ThemeColor::new("Accent 5".to_string(), (128, 100, 162)),
            )
            .with_color(
                "accent6".to_string(),
                ThemeColor::new("Accent 6".to_string(), (247, 150, 70)),
            )
            .with_font(
                "heading".to_string(),
                ThemeFont::new("Heading".to_string(), "Calibri".to_string()),
            )
            .with_font(
                "body".to_string(),
                ThemeFont::new("Body".to_string(), "Calibri".to_string()),
            )
    }

    /// 创建深色主题
    pub fn dark_theme() -> Self {
        Self::new("Dark".to_string())
            .with_color(
                "background".to_string(),
                ThemeColor::new("Background".to_string(), (30, 30, 30)),
            )
            .with_color(
                "text".to_string(),
                ThemeColor::new("Text".to_string(), (255, 255, 255)),
            )
            .with_color(
                "accent1".to_string(),
                ThemeColor::new("Accent 1".to_string(), (0, 120, 215)),
            )
            .with_font(
                "heading".to_string(),
                ThemeFont::new("Heading".to_string(), "Segoe UI".to_string()),
            )
            .with_font(
                "body".to_string(),
                ThemeFont::new("Body".to_string(), "Segoe UI".to_string()),
            )
            .with_background_style("dark".to_string())
    }

    /// 创建大学主题
    pub fn university_theme() -> Self {
        Self::new("University".to_string())
            .with_color(
                "primary".to_string(),
                ThemeColor::new("Primary".to_string(), (0, 51, 102)),
            )
            .with_color(
                "secondary".to_string(),
                ThemeColor::new("Secondary".to_string(), (204, 153, 0)),
            )
            .with_color(
                "accent".to_string(),
                ThemeColor::new("Accent".to_string(), (102, 102, 102)),
            )
            .with_font(
                "heading".to_string(),
                ThemeFont::new("Heading".to_string(), "Times New Roman".to_string()),
            )
            .with_font(
                "body".to_string(),
                ThemeFont::new("Body".to_string(), "Arial".to_string()),
            )
    }
}

impl Default for PptTheme {
    fn default() -> Self {
        Self::default_theme()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_color_new() {
        let color = ThemeColor::new("Red".to_string(), (255, 0, 0));
        assert_eq!(color.name, "Red");
        assert_eq!(color.rgb, (255, 0, 0));
        assert_eq!(color.hex, "FF0000");
    }

    #[test]
    fn test_theme_color_from_hex() {
        let color = ThemeColor::from_hex("Blue".to_string(), "#0000FF").unwrap();
        assert_eq!(color.name, "Blue");
        assert_eq!(color.rgb, (0, 0, 255));
    }

    #[test]
    fn test_theme_color_from_hex_invalid() {
        let result = ThemeColor::from_hex("Test".to_string(), "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_theme_font_new() {
        let font = ThemeFont::new("Arial".to_string(), "Arial".to_string());
        assert_eq!(font.name, "Arial");
        assert_eq!(font.latin, "Arial");
    }

    #[test]
    fn test_theme_font_with_east_asian() {
        let font = ThemeFont::new("Test".to_string(), "Arial".to_string())
            .with_east_asian("SimSun".to_string());
        assert_eq!(font.east_asian, "SimSun");
    }

    #[test]
    fn test_theme_effect_new() {
        let effect = ThemeEffect::new("Shadow".to_string(), "drop-shadow".to_string());
        assert_eq!(effect.name, "Shadow");
        assert_eq!(effect.effect_type, "drop-shadow");
    }

    #[test]
    fn test_theme_effect_with_parameter() {
        let effect = ThemeEffect::new("Shadow".to_string(), "drop-shadow".to_string())
            .with_parameter("blur".to_string(), "5px".to_string());
        assert_eq!(effect.parameters.get("blur"), Some(&"5px".to_string()));
    }

    #[test]
    fn test_ppt_theme_new() {
        let theme = PptTheme::new("Test".to_string());
        assert_eq!(theme.name, "Test");
        assert!(theme.colors.is_empty());
    }

    #[test]
    fn test_ppt_theme_with_color() {
        let color = ThemeColor::new("Red".to_string(), (255, 0, 0));
        let theme = PptTheme::new("Test".to_string()).with_color("accent".to_string(), color);
        assert_eq!(theme.colors.len(), 1);
    }

    #[test]
    fn test_ppt_theme_default() {
        let theme = PptTheme::default();
        assert_eq!(theme.name, "Default");
        assert!(theme.colors.len() > 0);
    }

    #[test]
    fn test_ppt_theme_dark() {
        let theme = PptTheme::dark_theme();
        assert_eq!(theme.name, "Dark");
        assert_eq!(theme.background_style, "dark");
    }

    #[test]
    fn test_ppt_theme_university() {
        let theme = PptTheme::university_theme();
        assert_eq!(theme.name, "University");
        assert!(theme.colors.contains_key("primary"));
    }

    #[test]
    fn test_ppt_theme_chaining() {
        let color = ThemeColor::new("Red".to_string(), (255, 0, 0));
        let font = ThemeFont::new("Arial".to_string(), "Arial".to_string());
        let theme = PptTheme::new("Test".to_string())
            .with_color("accent".to_string(), color)
            .with_font("body".to_string(), font);
        assert_eq!(theme.colors.len(), 1);
        assert_eq!(theme.fonts.len(), 1);
    }

    #[test]
    fn test_ppt_theme_serialization() {
        let theme = PptTheme::new("Test".to_string());
        let json = serde_json::to_string(&theme);
        assert!(json.is_ok());
    }
}
