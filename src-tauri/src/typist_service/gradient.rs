/*!
 * 航空航天级渐变和平铺系统
 * 实现 Typst 的渐变和平铺功能
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum GradientType {
    Linear,
    Radial,
    Conic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradientStop {
    pub position: f64, // 0.0 to 1.0
    pub color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn to_rgba(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }

    pub fn interpolate(&self, other: &Color, t: f64) -> Color {
        let t = t.clamp(0.0, 1.0) as f32;
        Color {
            r: (self.r as f32 + (other.r as f32 - self.r as f32) * t) as u8,
            g: (self.g as f32 + (other.g as f32 - self.g as f32) * t) as u8,
            b: (self.b as f32 + (other.b as f32 - self.b as f32) * t) as u8,
            a: self.a + (other.a - self.a) * t,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gradient {
    pub gradient_type: GradientType,
    pub stops: Vec<GradientStop>,
    pub angle: f64,                 // For linear gradients, in degrees
    pub center: Option<(f64, f64)>, // For radial gradients
}

impl Gradient {
    pub fn new(gradient_type: GradientType) -> Self {
        Self {
            gradient_type,
            stops: Vec::new(),
            angle: 0.0,
            center: None,
        }
    }

    pub fn with_angle(mut self, angle: f64) -> Self {
        self.angle = angle;
        self
    }

    pub fn with_center(mut self, x: f64, y: f64) -> Self {
        self.center = Some((x, y));
        self
    }

    pub fn add_stop(mut self, position: f64, color: Color) -> Self {
        self.stops.push(GradientStop { position, color });
        self.stops
            .sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
        self
    }

    pub fn to_css(&self) -> String {
        let stops_str: Vec<String> = self
            .stops
            .iter()
            .map(|stop| format!("{} {}", stop.color.to_hex(), stop.position * 100.0))
            .collect();

        match self.gradient_type {
            GradientType::Linear => {
                format!(
                    "linear-gradient({}deg, {})",
                    self.angle,
                    stops_str.join(", ")
                )
            }
            GradientType::Radial => {
                let center = self.center.unwrap_or((0.5, 0.5));
                format!(
                    "radial-gradient(circle at {}% {}%, {})",
                    center.0 * 100.0,
                    center.1 * 100.0,
                    stops_str.join(", ")
                )
            }
            GradientType::Conic => {
                let center = self.center.unwrap_or((0.5, 0.5));
                format!(
                    "conic-gradient(from {}deg at {}% {}%, {})",
                    self.angle,
                    center.0 * 100.0,
                    center.1 * 100.0,
                    stops_str.join(", ")
                )
            }
        }
    }

    pub fn to_svg(&self, id: &str) -> String {
        let stops_str: Vec<String> = self
            .stops
            .iter()
            .map(|stop| {
                format!(
                    "<stop offset=\"{}%\" stop-color=\"{}\" stop-opacity=\"{}\"/>",
                    stop.position * 100.0,
                    stop.color.to_hex(),
                    stop.color.a
                )
            })
            .collect();

        let gradient_def = match self.gradient_type {
            GradientType::Linear => {
                let angle_rad = self.angle * std::f64::consts::PI / 180.0;
                let x1 = 0.5 - 0.5 * angle_rad.cos();
                let y1 = 0.5 - 0.5 * angle_rad.sin();
                let x2 = 0.5 + 0.5 * angle_rad.cos();
                let y2 = 0.5 + 0.5 * angle_rad.sin();

                format!(
                    "<linearGradient id=\"{}\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\">\n{}\n</linearGradient>",
                    id, x1, y1, x2, y2,
                    stops_str.join("\n")
                )
            }
            GradientType::Radial => {
                let center = self.center.unwrap_or((0.5, 0.5));
                format!(
                    "<radialGradient id=\"{}\" cx=\"{}\" cy=\"{}\" r=\"0.5\">\n{}\n</radialGradient>",
                    id, center.0, center.1,
                    stops_str.join("\n")
                )
            }
            GradientType::Conic => {
                let center = self.center.unwrap_or((0.5, 0.5));
                format!(
                    "<conicGradient id=\"{}\" cx=\"{}\" cy=\"{}\">\n{}\n</conicGradient>",
                    id,
                    center.0,
                    center.1,
                    stops_str.join("\n")
                )
            }
        };

        gradient_def
    }

    pub fn to_typst(&self) -> String {
        let stops_str: Vec<String> = self
            .stops
            .iter()
            .map(|stop| format!("({}, {})", stop.color.to_hex(), stop.position))
            .collect();

        match self.gradient_type {
            GradientType::Linear => {
                format!(
                    "gradient.linear(angle: {}deg, ({}))",
                    self.angle,
                    stops_str.join(", ")
                )
            }
            GradientType::Radial => {
                let center = self.center.unwrap_or((0.5, 0.5));
                format!(
                    "gradient.radial(center: ({}, {}), ({}))",
                    center.0,
                    center.1,
                    stops_str.join(", ")
                )
            }
            GradientType::Conic => {
                let center = self.center.unwrap_or((0.5, 0.5));
                format!(
                    "gradient.conic(angle: {}deg, center: ({}, {}), ({}))",
                    self.angle,
                    center.0,
                    center.1,
                    stops_str.join(", ")
                )
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TilingMode {
    Repeat,
    Reflect,
    Clamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub width: f64,
    pub height: f64,
    pub tiling_mode: TilingMode,
    pub transform: Option<Transform>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub translate_x: f64,
    pub translate_y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub rotation: f64,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            translate_x: 0.0,
            translate_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation: 0.0,
        }
    }

    pub fn with_translation(mut self, x: f64, y: f64) -> Self {
        self.translate_x = x;
        self.translate_y = y;
        self
    }

    pub fn with_scale(mut self, x: f64, y: f64) -> Self {
        self.scale_x = x;
        self.scale_y = y;
        self
    }

    pub fn with_rotation(mut self, angle: f64) -> Self {
        self.rotation = angle;
        self
    }

    pub fn to_css(&self) -> String {
        format!(
            "translate({}px, {}px) scale({}, {}) rotate({}deg)",
            self.translate_x, self.translate_y, self.scale_x, self.scale_y, self.rotation
        )
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

impl Pattern {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            tiling_mode: TilingMode::Repeat,
            transform: None,
        }
    }

    pub fn with_tiling_mode(mut self, mode: TilingMode) -> Self {
        self.tiling_mode = mode;
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = Some(transform);
        self
    }

    pub fn to_css(&self) -> String {
        let repeat = match self.tiling_mode {
            TilingMode::Repeat => "repeat",
            TilingMode::Reflect => "space",
            TilingMode::Clamp => "no-repeat",
        };

        let transform_str = if let Some(ref transform) = self.transform {
            format!(" transform: {};", transform.to_css())
        } else {
            String::new()
        };

        format!(
            "background-repeat: {}; background-size: {}px {}px;{}",
            repeat, self.width, self.height, transform_str
        )
    }

    pub fn to_svg(&self, id: &str) -> String {
        let pattern_units = match self.tiling_mode {
            TilingMode::Repeat => "userSpaceOnUse",
            TilingMode::Reflect => "userSpaceOnUse",
            TilingMode::Clamp => "userSpaceOnUse",
        };

        let transform_str = if let Some(ref transform) = self.transform {
            format!(" patternTransform=\"{}\"", transform.to_css())
        } else {
            String::new()
        };

        format!(
            "<pattern id=\"{}\" width=\"{}\" height=\"{}\" patternUnits=\"{}\"{}>\n\
             <!-- Pattern content here -->\n\
             </pattern>",
            id, self.width, self.height, pattern_units, transform_str
        )
    }

    pub fn to_typst(&self) -> String {
        let mode_str = match self.tiling_mode {
            TilingMode::Repeat => "repeat",
            TilingMode::Reflect => "reflect",
            TilingMode::Clamp => "clamp",
        };

        format!(
            "pattern(width: {}pt, height: {}pt, tiling: {})",
            self.width, self.height, mode_str
        )
    }
}

pub struct GradientBuilder {
    gradient: Gradient,
}

impl GradientBuilder {
    pub fn new(gradient_type: GradientType) -> Self {
        Self {
            gradient: Gradient::new(gradient_type),
        }
    }

    pub fn angle(mut self, angle: f64) -> Self {
        self.gradient.angle = angle;
        self
    }

    pub fn center(mut self, x: f64, y: f64) -> Self {
        self.gradient.center = Some((x, y));
        self
    }

    pub fn stop(mut self, position: f64, color: Color) -> Self {
        self.gradient.stops.push(GradientStop { position, color });
        self
    }

    pub fn build(self) -> Gradient {
        let mut gradient = self.gradient;
        gradient
            .stops
            .sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
        gradient
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::rgb(255, 0, 0);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn test_color_to_hex() {
        let color = Color::rgb(255, 0, 0);
        assert_eq!(color.to_hex(), "#ff0000");
    }

    #[test]
    fn test_color_interpolate() {
        let color1 = Color::rgb(0, 0, 0);
        let color2 = Color::rgb(255, 255, 255);
        let result = color1.interpolate(&color2, 0.5);
        assert_eq!(result.r, 127);
        assert_eq!(result.g, 127);
        assert_eq!(result.b, 127);
    }

    #[test]
    fn test_gradient_creation() {
        let gradient = Gradient::new(GradientType::Linear);
        assert_eq!(gradient.gradient_type, GradientType::Linear);
        assert!(gradient.stops.is_empty());
    }

    #[test]
    fn test_gradient_with_stops() {
        let gradient = Gradient::new(GradientType::Linear)
            .add_stop(0.0, Color::rgb(255, 0, 0))
            .add_stop(1.0, Color::rgb(0, 0, 255));
        assert_eq!(gradient.stops.len(), 2);
    }

    #[test]
    fn test_gradient_to_css() {
        let gradient = Gradient::new(GradientType::Linear)
            .with_angle(45.0)
            .add_stop(0.0, Color::rgb(255, 0, 0))
            .add_stop(1.0, Color::rgb(0, 0, 255));
        let css = gradient.to_css();
        assert!(css.contains("linear-gradient"));
        assert!(css.contains("45deg"));
    }

    #[test]
    fn test_gradient_to_svg() {
        let gradient = Gradient::new(GradientType::Linear)
            .add_stop(0.0, Color::rgb(255, 0, 0))
            .add_stop(1.0, Color::rgb(0, 0, 255));
        let svg = gradient.to_svg("test");
        assert!(svg.contains("<linearGradient"));
        assert!(svg.contains("id=\"test\""));
    }

    #[test]
    fn test_radial_gradient() {
        let gradient = Gradient::new(GradientType::Radial)
            .with_center(0.5, 0.5)
            .add_stop(0.0, Color::rgb(255, 0, 0))
            .add_stop(1.0, Color::rgb(0, 0, 255));
        let css = gradient.to_css();
        assert!(css.contains("radial-gradient"));
    }

    #[test]
    fn test_pattern_creation() {
        let pattern = Pattern::new(100.0, 100.0);
        assert_eq!(pattern.width, 100.0);
        assert_eq!(pattern.height, 100.0);
    }

    #[test]
    fn test_pattern_with_tiling_mode() {
        let pattern = Pattern::new(100.0, 100.0).with_tiling_mode(TilingMode::Reflect);
        assert_eq!(pattern.tiling_mode, TilingMode::Reflect);
    }

    #[test]
    fn test_pattern_to_css() {
        let pattern = Pattern::new(100.0, 100.0);
        let css = pattern.to_css();
        assert!(css.contains("background-repeat"));
        assert!(css.contains("background-size"));
    }

    #[test]
    fn test_transform_creation() {
        let transform = Transform::new();
        assert_eq!(transform.translate_x, 0.0);
        assert_eq!(transform.scale_x, 1.0);
    }

    #[test]
    fn test_transform_with_translation() {
        let transform = Transform::new().with_translation(10.0, 20.0);
        assert_eq!(transform.translate_x, 10.0);
        assert_eq!(transform.translate_y, 20.0);
    }

    #[test]
    fn test_transform_to_css() {
        let transform = Transform::new()
            .with_translation(10.0, 20.0)
            .with_scale(2.0, 2.0)
            .with_rotation(45.0);
        let css = transform.to_css();
        assert!(css.contains("translate"));
        assert!(css.contains("scale"));
        assert!(css.contains("rotate"));
    }

    #[test]
    fn test_gradient_builder() {
        let gradient = GradientBuilder::new(GradientType::Linear)
            .angle(90.0)
            .stop(0.0, Color::rgb(255, 0, 0))
            .stop(1.0, Color::rgb(0, 0, 255))
            .build();
        assert_eq!(gradient.angle, 90.0);
        assert_eq!(gradient.stops.len(), 2);
    }

    #[test]
    fn test_conic_gradient() {
        let gradient = Gradient::new(GradientType::Conic)
            .with_angle(0.0)
            .add_stop(0.0, Color::rgb(255, 0, 0))
            .add_stop(1.0, Color::rgb(0, 0, 255));
        let css = gradient.to_css();
        assert!(css.contains("conic-gradient"));
    }
}
