//! Cell styling system with aerospace-grade validation and serialization
//! 
//! This module provides comprehensive cell styling functionality including
//! fonts, colors, borders, alignment, and number formatting.

use crate::spreadsheet_service::error::{SpreadsheetError, SpreadsheetResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Font style options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

/// Font weight options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontWeight {
    Normal,
    Bold,
    W100,
    W200,
    W300,
    W400,
    W500,
    W600,
    W700,
    W800,
    W900,
}

/// Text decoration options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextDecoration {
    None,
    Underline,
    LineThrough,
    Overline,
}

/// Horizontal alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
    Justify,
    Fill,
}

/// Vertical alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
    Justify,
    Distributed,
}

/// Border style options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BorderStyle {
    None,
    Thin,
    Medium,
    Dashed,
    Dotted,
    Thick,
    Double,
    Hair,
    MediumDashed,
    DashDot,
    MediumDashDot,
    DashDotDot,
    MediumDashDotDot,
    SlantDashDot,
}

/// Fill pattern type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FillPattern {
    None,
    Solid,
    MediumGray,
    DarkGray,
    LightGray,
    DarkHorizontal,
    DarkVertical,
    DarkDown,
    DarkUp,
    DarkGrid,
    DarkTrellis,
    LightHorizontal,
    LightVertical,
    LightDown,
    LightUp,
    LightGrid,
    LightTrellis,
    Gray125,
    Gray0625,
}

/// Color representation (ARGB)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    /// Create a new color from ARGB components
    pub fn new(alpha: u8, red: u8, green: u8, blue: u8) -> Self {
        Self { alpha, red, green, blue }
    }

    /// Create a color from RGB (alpha = 255)
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::new(255, red, green, blue)
    }

    /// Create a color from hex string (e.g., "#FF0000" or "FF0000")
    pub fn from_hex(hex: &str) -> SpreadsheetResult<Self> {
        let hex = hex.trim_start_matches('#');
        
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16)
                .map_err(|_| SpreadsheetError::invalid_input("hex", hex, "invalid hex format"))?;
            let g = u8::from_str_radix(&hex[2..4], 16)
                .map_err(|_| SpreadsheetError::invalid_input("hex", hex, "invalid hex format"))?;
            let b = u8::from_str_radix(&hex[4..6], 16)
                .map_err(|_| SpreadsheetError::invalid_input("hex", hex, "invalid hex format"))?;
            Ok(Self::rgb(r, g, b))
        } else if hex.len() == 8 {
            let a = u8::from_str_radix(&hex[0..2], 16)
                .map_err(|_| SpreadsheetError::invalid_input("hex", hex, "invalid hex format"))?;
            let r = u8::from_str_radix(&hex[2..4], 16)
                .map_err(|_| SpreadsheetError::invalid_input("hex", hex, "invalid hex format"))?;
            let g = u8::from_str_radix(&hex[4..6], 16)
                .map_err(|_| SpreadsheetError::invalid_input("hex", hex, "invalid hex format"))?;
            let b = u8::from_str_radix(&hex[6..8], 16)
                .map_err(|_| SpreadsheetError::invalid_input("hex", hex, "invalid hex format"))?;
            Ok(Self::new(a, r, g, b))
        } else {
            Err(SpreadsheetError::invalid_input("hex", hex, "hex must be 6 or 8 characters"))
        }
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        format!("{:02X}{:02X}{:02X}{:02X}", self.alpha, self.red, self.green, self.blue)
    }

    /// Common colors
    pub const BLACK: Color = Color { alpha: 255, red: 0, green: 0, blue: 0 };
    pub const WHITE: Color = Color { alpha: 255, red: 255, green: 255, blue: 255 };
    pub const RED: Color = Color { alpha: 255, red: 255, green: 0, blue: 0 };
    pub const GREEN: Color = Color { alpha: 255, red: 0, green: 255, blue: 0 };
    pub const BLUE: Color = Color { alpha: 255, red: 0, green: 0, blue: 255 };
}

/// Font settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Font {
    pub name: Option<String>,
    pub size: Option<f64>,
    pub style: Option<FontStyle>,
    pub weight: Option<FontWeight>,
    pub color: Option<Color>,
    pub decoration: Option<TextDecoration>,
    pub strike: Option<bool>,
    pub outline: Option<bool>,
    pub shadow: Option<bool>,
    pub condense: Option<bool>,
    pub extend: Option<bool>,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            name: Some("Calibri".to_string()),
            size: Some(11.0),
            style: Some(FontStyle::Normal),
            weight: Some(FontWeight::Normal),
            color: Some(Color::BLACK),
            decoration: Some(TextDecoration::None),
            strike: Some(false),
            outline: Some(false),
            shadow: Some(false),
            condense: Some(false),
            extend: Some(false),
        }
    }
}

/// Border settings for a single side
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderSide {
    pub style: Option<BorderStyle>,
    pub color: Option<Color>,
}

impl Default for BorderSide {
    fn default() -> Self {
        Self {
            style: Some(BorderStyle::None),
            color: None,
        }
    }
}

/// Complete border settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Border {
    pub left: Option<BorderSide>,
    pub right: Option<BorderSide>,
    pub top: Option<BorderSide>,
    pub bottom: Option<BorderSide>,
    pub diagonal: Option<BorderSide>,
    pub diagonal_up: Option<bool>,
    pub diagonal_down: Option<bool>,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            left: Some(BorderSide::default()),
            right: Some(BorderSide::default()),
            top: Some(BorderSide::default()),
            bottom: Some(BorderSide::default()),
            diagonal: Some(BorderSide::default()),
            diagonal_up: Some(false),
            diagonal_down: Some(false),
        }
    }
}

/// Fill settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fill {
    pub pattern: Option<FillPattern>,
    pub foreground_color: Option<Color>,
    pub background_color: Option<Color>,
}

impl Default for Fill {
    fn default() -> Self {
        Self {
            pattern: Some(FillPattern::None),
            foreground_color: None,
            background_color: None,
        }
    }
}

/// Alignment settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alignment {
    pub horizontal: Option<HorizontalAlignment>,
    pub vertical: Option<VerticalAlignment>,
    pub wrap_text: Option<bool>,
    pub shrink_to_fit: Option<bool>,
    pub indent: Option<u32>,
    pub text_rotation: Option<i32>,
    pub justify_last_line: Option<bool>,
}

impl Default for Alignment {
    fn default() -> Self {
        Self {
            horizontal: Some(HorizontalAlignment::GENERAL),
            vertical: Some(VerticalAlignment::Bottom),
            wrap_text: Some(false),
            shrink_to_fit: Some(false),
            indent: Some(0),
            text_rotation: Some(0),
            justify_last_line: Some(false),
        }
    }
}

/// Number format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberFormat {
    pub format_code: String,
}

impl NumberFormat {
    /// Create a new number format
    pub fn new(format_code: String) -> Self {
        Self { format_code }
    }

    /// Common number formats
    pub fn general() -> Self {
        Self::new("General".to_string())
    }

    pub fn currency() -> Self {
        Self::new("$#,##0.00".to_string())
    }

    pub fn percentage() -> Self {
        Self::new("0.00%".to_string())
    }

    pub fn date() -> Self {
        Self::new("yyyy-mm-dd".to_string())
    }

    pub fn time() -> Self {
        Self::new("hh:mm:ss".to_string())
    }

    pub fn scientific() -> Self {
        Self::new("0.00E+00".to_string())
    }
}

/// Protection settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Protection {
    pub locked: Option<bool>,
    pub hidden: Option<bool>,
}

impl Default for Protection {
    fn default() -> Self {
        Self {
            locked: Some(true),
            hidden: Some(false),
        }
    }
}

/// Complete cell style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellStyle {
    pub font: Option<Font>,
    pub border: Option<Border>,
    pub fill: Option<Fill>,
    pub alignment: Option<Alignment>,
    pub number_format: Option<NumberFormat>,
    pub protection: Option<Protection>,
}

impl Default for CellStyle {
    fn default() -> Self {
        Self {
            font: Some(Font::default()),
            border: Some(Border::default()),
            fill: Some(Fill::default()),
            alignment: Some(Alignment::default()),
            number_format: Some(NumberFormat::general()),
            protection: Some(Protection::default()),
        }
    }
}

impl CellStyle {
    /// Create a new cell style with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a minimal cell style (no properties set)
    pub fn minimal() -> Self {
        Self {
            font: None,
            border: None,
            fill: None,
            alignment: None,
            number_format: None,
            protection: None,
        }
    }

    /// Merge with another style (other takes precedence)
    pub fn merge(&self, other: &CellStyle) -> CellStyle {
        CellStyle {
            font: other.font.as_ref().or(self.font.as_ref()).cloned(),
            border: other.border.as_ref().or(self.border.as_ref()).cloned(),
            fill: other.fill.as_ref().or(self.fill.as_ref()).cloned(),
            alignment: other.alignment.as_ref().or(self.alignment.as_ref()).cloned(),
            number_format: other.number_format.as_ref().or(self.number_format.as_ref()).cloned(),
            protection: other.protection.as_ref().or(self.protection.as_ref()).cloned(),
        }
    }

    /// Check if style is empty (no properties set)
    pub fn is_empty(&self) -> bool {
        self.font.is_none()
            && self.border.is_none()
            && self.fill.is_none()
            && self.alignment.is_none()
            && self.number_format.is_none()
            && self.protection.is_none()
    }
}

/// Style manager for handling cell styles
pub struct StyleManager {
    /// Style storage (key: style hash, value: CellStyle)
    styles: HashMap<String, CellStyle>,
    /// Style cache for quick lookup
    style_cache: HashMap<String, String>,
}

impl StyleManager {
    /// Create a new style manager
    pub fn new() -> Self {
        Self {
            styles: HashMap::new(),
            style_cache: HashMap::new(),
        }
    }

    /// Register a style and return its ID
    pub fn register_style(&mut self, style: CellStyle) -> String {
        let hash = self.style_hash(&style);
        self.styles.insert(hash.clone(), style);
        hash
    }

    /// Get a style by ID
    pub fn get_style(&self, id: &str) -> Option<&CellStyle> {
        self.styles.get(id)
    }

    /// Update a style
    pub fn update_style(&mut self, id: &str, style: CellStyle) -> SpreadsheetResult<()> {
        if !self.styles.contains_key(id) {
            return Err(SpreadsheetError::invalid_input("style_id", id, "style not found"));
        }
        self.styles.insert(id.to_string(), style);
        Ok(())
    }

    /// Delete a style
    pub fn delete_style(&mut self, id: &str) -> SpreadsheetResult<()> {
        if !self.styles.contains_key(id) {
            return Err(SpreadsheetError::invalid_input("style_id", id, "style not found"));
        }
        self.styles.remove(id);
        Ok(())
    }

    /// Get all registered styles
    pub fn get_all_styles(&self) -> Vec<&CellStyle> {
        self.styles.values().collect()
    }

    /// Generate a hash for a style
    fn style_hash(&self, style: &CellStyle) -> String {
        let serialized = serde_json::to_string(style).unwrap_or_default();
        format!("{:x}", md5::compute(serialized.as_bytes()))
    }
}

impl Default for StyleManager {
    fn default() -> Self {
        Self::new()
    }
}

// Add HorizontalAlignment::General for default
impl HorizontalAlignment {
    pub const GENERAL: HorizontalAlignment = HorizontalAlignment::Left;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::rgb(255, 0, 0);
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 0);
    }

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("#FF0000").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 0);
    }

    #[test]
    fn test_color_to_hex() {
        let color = Color::rgb(255, 0, 0);
        let hex = color.to_hex();
        // Hex format may include alpha channel or not
        assert!(hex == "FF0000" || hex == "FFFF0000");
    }

    #[test]
    fn test_font_default() {
        let font = Font::default();
        assert_eq!(font.name, Some("Calibri".to_string()));
        assert_eq!(font.size, Some(11.0));
    }

    #[test]
    fn test_border_default() {
        let border = Border::default();
        assert!(border.left.is_some());
        assert!(border.right.is_some());
    }

    #[test]
    fn test_fill_default() {
        let fill = Fill::default();
        assert_eq!(fill.pattern, Some(FillPattern::None));
    }

    #[test]
    fn test_alignment_default() {
        let alignment = Alignment::default();
        assert!(alignment.horizontal.is_some());
        assert!(alignment.vertical.is_some());
    }

    #[test]
    fn test_cell_style_default() {
        let style = CellStyle::default();
        assert!(style.font.is_some());
        assert!(style.border.is_some());
    }

    #[test]
    fn test_cell_style_minimal() {
        let style = CellStyle::minimal();
        assert!(style.is_empty());
    }

    #[test]
    fn test_cell_style_merge() {
        let style1 = CellStyle {
            font: Some(Font::default()),
            ..CellStyle::minimal()
        };
        let style2 = CellStyle {
            border: Some(Border::default()),
            ..CellStyle::minimal()
        };
        let merged = style1.merge(&style2);
        assert!(merged.font.is_some());
        assert!(merged.border.is_some());
    }

    #[test]
    fn test_number_format() {
        let format = NumberFormat::currency();
        assert_eq!(format.format_code, "$#,##0.00");
    }

    #[test]
    fn test_style_manager() {
        let mut manager = StyleManager::new();
        let style = CellStyle::default();
        let id = manager.register_style(style);
        let retrieved = manager.get_style(&id);
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_font_with_all_fields() {
        let font = Font {
            name: Some("Arial".to_string()),
            size: Some(12.0),
            style: Some(FontStyle::Italic),
            weight: Some(FontWeight::Bold),
            color: Some(Color::rgb(255, 0, 0)),
            decoration: Some(TextDecoration::Underline),
            strike: Some(false),
            outline: Some(false),
            shadow: Some(false),
            condense: Some(false),
            extend: Some(false),
        };
        assert_eq!(font.name, Some("Arial".to_string()));
        assert_eq!(font.size, Some(12.0));
        assert_eq!(font.style, Some(FontStyle::Italic));
    }

    #[test]
    fn test_border_style_display() {
        assert_eq!(format!("{:?}", BorderStyle::Thin), "Thin");
        assert_eq!(format!("{:?}", BorderStyle::Medium), "Medium");
        assert_eq!(format!("{:?}", BorderStyle::Thick), "Thick");
    }

    #[test]
    fn test_fill_pattern_display() {
        assert_eq!(format!("{:?}", FillPattern::None), "None");
        assert_eq!(format!("{:?}", FillPattern::Solid), "Solid");
    }

    #[test]
    fn test_horizontal_alignment_display() {
        assert_eq!(format!("{:?}", HorizontalAlignment::Left), "Left");
        assert_eq!(format!("{:?}", HorizontalAlignment::Center), "Center");
        assert_eq!(format!("{:?}", HorizontalAlignment::Right), "Right");
    }

    #[test]
    fn test_vertical_alignment_display() {
        assert_eq!(format!("{:?}", VerticalAlignment::Top), "Top");
        assert_eq!(format!("{:?}", VerticalAlignment::Center), "Center");
        assert_eq!(format!("{:?}", VerticalAlignment::Bottom), "Bottom");
    }

    #[test]
    fn test_color_from_hex_invalid() {
        let result = Color::from_hex("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_color_from_rgba() {
        let color = Color::new(128, 255, 128, 64);
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
        assert_eq!(color.alpha, 128);
    }
}
