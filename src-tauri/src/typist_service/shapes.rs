/*!
 * 航空航天级可视化形状系统
 * 实现 Typst 的可视化形状功能（Circle、Rectangle、Ellipse、Square）
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ShapeType {
    Circle,
    Rectangle,
    Ellipse,
    Square,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stroke {
    pub color: Color,
    pub width: f64,
    pub dash_pattern: Option<Vec<f64>>,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            color: Color::rgb(0, 0, 0),
            width: 1.0,
            dash_pattern: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fill {
    pub color: Color,
}

impl Default for Fill {
    fn default() -> Self {
        Self {
            color: Color::rgb(255, 255, 255),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub stroke: Option<Stroke>,
    pub fill: Option<Fill>,
    pub rotation: f64,
    pub attributes: HashMap<String, String>,
}

impl Shape {
    pub fn new(shape_type: ShapeType, x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            shape_type,
            x,
            y,
            width,
            height,
            stroke: None,
            fill: None,
            rotation: 0.0,
            attributes: HashMap::new(),
        }
    }

    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }

    pub fn with_fill(mut self, fill: Fill) -> Self {
        self.fill = Some(fill);
        self
    }

    pub fn with_rotation(mut self, rotation: f64) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn to_svg(&self) -> String {
        let mut svg = String::new();

        let transform = if self.rotation != 0.0 {
            format!(
                " transform=\"rotate({} {:.1} {:.1})\"",
                self.rotation,
                self.x + self.width / 2.0,
                self.y + self.height / 2.0
            )
        } else {
            String::new()
        };

        let stroke_attr = if let Some(ref stroke) = self.stroke {
            format!(
                " stroke=\"{}\" stroke-width=\"{}\"",
                stroke.color.to_hex(),
                stroke.width
            )
        } else {
            String::new()
        };

        let fill_attr = if let Some(ref fill) = self.fill {
            format!(" fill=\"{}\"", fill.color.to_hex())
        } else {
            " fill=\"none\"".to_string()
        };

        match self.shape_type {
            ShapeType::Circle => {
                let radius = (self.width / 2.0).min(self.height / 2.0);
                let cx = self.x + self.width / 2.0;
                let cy = self.y + self.height / 2.0;
                svg.push_str(&format!(
                    "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\"{}{}{}/>\n",
                    cx, cy, radius, stroke_attr, fill_attr, transform
                ));
            }
            ShapeType::Rectangle => {
                svg.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"{}{}{}/>\n",
                    self.x, self.y, self.width, self.height, stroke_attr, fill_attr, transform
                ));
            }
            ShapeType::Ellipse => {
                let cx = self.x + self.width / 2.0;
                let cy = self.y + self.height / 2.0;
                let rx = self.width / 2.0;
                let ry = self.height / 2.0;
                svg.push_str(&format!(
                    "<ellipse cx=\"{:.1}\" cy=\"{:.1}\" rx=\"{:.1}\" ry=\"{:.1}\"{}{}{}/>\n",
                    cx, cy, rx, ry, stroke_attr, fill_attr, transform
                ));
            }
            ShapeType::Square => {
                let size = self.width.min(self.height);
                svg.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"{}{}{}/>\n",
                    self.x, self.y, size, size, stroke_attr, fill_attr, transform
                ));
            }
        }

        svg
    }

    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        match self.shape_type {
            ShapeType::Circle => {
                let radius = (self.width / 2.0).min(self.height / 2.0);
                typst.push_str(&format!("#circle(radius: {}pt)", radius));
            }
            ShapeType::Rectangle => {
                typst.push_str(&format!(
                    "#rect(width: {}pt, height: {}pt)",
                    self.width, self.height
                ));
            }
            ShapeType::Ellipse => {
                let rx = self.width / 2.0;
                let ry = self.height / 2.0;
                typst.push_str(&format!("#ellipse(rx: {}pt, ry: {}pt)", rx, ry));
            }
            ShapeType::Square => {
                let size = self.width.min(self.height);
                typst.push_str(&format!("#square(size: {}pt)", size));
            }
        }

        // Add stroke if present
        if let Some(ref stroke) = self.stroke {
            typst.push_str(&format!(
                "\n  .stroke({}pt + {})",
                stroke.width,
                stroke.color.to_hex()
            ));
        }

        // Add fill if present
        if let Some(ref fill) = self.fill {
            typst.push_str(&format!("\n  .fill({})", fill.color.to_hex()));
        }

        typst.push('\n');
        typst
    }

    pub fn to_html(&self) -> String {
        let mut html = String::new();

        let style = format!(
            "position: absolute; left: {:.1}px; top: {:.1}px; width: {:.1}px; height: {:.1}px;",
            self.x, self.y, self.width, self.height
        );

        let stroke_style = if let Some(ref stroke) = self.stroke {
            format!(
                " border: {}px solid {}; ",
                stroke.width,
                stroke.color.to_hex()
            )
        } else {
            String::new()
        };

        let fill_style = if let Some(ref fill) = self.fill {
            format!(" background-color: {}; ", fill.color.to_hex())
        } else {
            String::new()
        };

        let rotation_style = if self.rotation != 0.0 {
            format!(" transform: rotate({}deg); ", self.rotation)
        } else {
            String::new()
        };

        let border_radius = match self.shape_type {
            ShapeType::Circle => " border-radius: 50%; ",
            ShapeType::Ellipse => " border-radius: 50%; ",
            ShapeType::Rectangle => "",
            ShapeType::Square => "",
        };

        html.push_str(&format!(
            "<div style=\"{}{}{}{}{}\"></div>\n",
            style, stroke_style, fill_style, rotation_style, border_radius
        ));

        html
    }
}

pub struct ShapeBuilder {
    shapes: Vec<Shape>,
}

impl ShapeBuilder {
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    pub fn add_circle(&mut self, x: f64, y: f64, radius: f64) -> &mut Self {
        let shape = Shape::new(ShapeType::Circle, x, y, radius * 2.0, radius * 2.0);
        self.shapes.push(shape);
        self
    }

    pub fn add_rectangle(&mut self, x: f64, y: f64, width: f64, height: f64) -> &mut Self {
        let shape = Shape::new(ShapeType::Rectangle, x, y, width, height);
        self.shapes.push(shape);
        self
    }

    pub fn add_ellipse(&mut self, x: f64, y: f64, rx: f64, ry: f64) -> &mut Self {
        let shape = Shape::new(ShapeType::Ellipse, x, y, rx * 2.0, ry * 2.0);
        self.shapes.push(shape);
        self
    }

    pub fn add_square(&mut self, x: f64, y: f64, size: f64) -> &mut Self {
        let shape = Shape::new(ShapeType::Square, x, y, size, size);
        self.shapes.push(shape);
        self
    }

    pub fn build(&self) -> Vec<Shape> {
        self.shapes.clone()
    }

    pub fn to_svg(&self) -> String {
        let mut svg = String::new();
        svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\">\n");

        for shape in &self.shapes {
            svg.push_str(&shape.to_svg());
        }

        svg.push_str("</svg>\n");
        svg
    }

    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        for shape in &self.shapes {
            typst.push_str(&shape.to_typst());
        }

        typst
    }
}

impl Default for ShapeBuilder {
    fn default() -> Self {
        Self::new()
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
    fn test_color_to_rgba() {
        let color = Color::new(255, 0, 0, 0.5);
        assert_eq!(color.to_rgba(), "rgba(255, 0, 0, 0.5)");
    }

    #[test]
    fn test_shape_creation() {
        let shape = Shape::new(ShapeType::Circle, 10.0, 10.0, 20.0, 20.0);
        assert_eq!(shape.shape_type, ShapeType::Circle);
        assert_eq!(shape.x, 10.0);
        assert_eq!(shape.y, 10.0);
    }

    #[test]
    fn test_shape_with_stroke() {
        let stroke = Stroke::default();
        let shape = Shape::new(ShapeType::Rectangle, 0.0, 0.0, 100.0, 50.0).with_stroke(stroke);
        assert!(shape.stroke.is_some());
    }

    #[test]
    fn test_shape_with_fill() {
        let fill = Fill::default();
        let shape = Shape::new(ShapeType::Rectangle, 0.0, 0.0, 100.0, 50.0).with_fill(fill);
        assert!(shape.fill.is_some());
    }

    #[test]
    fn test_shape_to_svg_circle() {
        let shape = Shape::new(ShapeType::Circle, 10.0, 10.0, 20.0, 20.0);
        let svg = shape.to_svg();
        assert!(svg.contains("<circle"));
        assert!(svg.contains("cx=\"20.0\""));
        assert!(svg.contains("cy=\"20.0\""));
    }

    #[test]
    fn test_shape_to_svg_rectangle() {
        let shape = Shape::new(ShapeType::Rectangle, 0.0, 0.0, 100.0, 50.0);
        let svg = shape.to_svg();
        assert!(svg.contains("<rect"));
        assert!(svg.contains("width=\"100.0\""));
        assert!(svg.contains("height=\"50.0\""));
    }

    #[test]
    fn test_shape_to_typst() {
        let shape = Shape::new(ShapeType::Circle, 0.0, 0.0, 20.0, 20.0);
        let typst = shape.to_typst();
        assert!(typst.contains("#circle"));
    }

    #[test]
    fn test_shape_to_html() {
        let shape = Shape::new(ShapeType::Rectangle, 10.0, 10.0, 100.0, 50.0);
        let html = shape.to_html();
        assert!(html.contains("<div"));
        assert!(html.contains("left: 10.0px"));
        assert!(html.contains("top: 10.0px"));
    }

    #[test]
    fn test_shape_builder() {
        let mut builder = ShapeBuilder::new();
        builder.add_circle(10.0, 10.0, 5.0);
        builder.add_rectangle(0.0, 0.0, 100.0, 50.0);

        let shapes = builder.build();
        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_shape_builder_to_svg() {
        let mut builder = ShapeBuilder::new();
        builder.add_circle(10.0, 10.0, 5.0);

        let svg = builder.to_svg();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("<circle"));
    }

    #[test]
    fn test_square_shape() {
        let shape = Shape::new(ShapeType::Square, 0.0, 0.0, 50.0, 50.0);
        let svg = shape.to_svg();
        assert!(svg.contains("<rect"));
    }

    #[test]
    fn test_ellipse_shape() {
        let shape = Shape::new(ShapeType::Ellipse, 0.0, 0.0, 100.0, 50.0);
        let svg = shape.to_svg();
        assert!(svg.contains("<ellipse"));
    }

    #[test]
    fn test_shape_rotation() {
        let shape = Shape::new(ShapeType::Rectangle, 0.0, 0.0, 100.0, 50.0).with_rotation(45.0);
        let svg = shape.to_svg();
        assert!(svg.contains("rotate(45"));
    }
}
