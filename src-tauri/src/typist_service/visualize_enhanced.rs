/*!
 * 航空航天级 Visualize 增强模块
 * 实现 Typst 的 Visualize 增强功能（polygon、curve、color、stroke）
 */

use serde::{Deserialize, Serialize};

/// 点坐标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizePoint {
    pub x: f64,
    pub y: f64,
}

impl VisualizePoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// 多边形
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Polygon {
    pub points: Vec<VisualizePoint>,
    pub fill: Option<String>,
    pub stroke: Option<String>,
}

impl Polygon {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            fill: None,
            stroke: None,
        }
    }

    pub fn with_points(mut self, points: Vec<VisualizePoint>) -> Self {
        self.points = points;
        self
    }

    pub fn with_fill(mut self, fill: String) -> Self {
        self.fill = Some(fill);
        self
    }

    pub fn with_stroke(mut self, stroke: String) -> Self {
        self.stroke = Some(stroke);
        self
    }

    pub fn add_point(&mut self, point: VisualizePoint) {
        self.points.push(point);
    }

    pub fn to_typst(&self) -> String {
        let points_str: Vec<String> = self
            .points
            .iter()
            .map(|p| format!("({}, {})", p.x, p.y))
            .collect();

        let mut parts = vec![format!("({})", points_str.join(", "))];

        if let Some(fill) = &self.fill {
            parts.push(format!("fill: \"{}\"", fill));
        }

        if let Some(stroke) = &self.stroke {
            parts.push(format!("stroke: \"{}\"", stroke));
        }

        format!("#polygon({})", parts.join(", "))
    }

    pub fn to_svg(&self) -> String {
        let points_str: Vec<String> = self
            .points
            .iter()
            .map(|p| format!("{},{}", p.x, p.y))
            .collect();

        let mut attrs = vec![format!("points=\"{}\"", points_str.join(" "))];

        if let Some(fill) = &self.fill {
            attrs.push(format!("fill=\"{}\"", fill));
        } else {
            attrs.push("fill=\"none\"".to_string());
        }

        if let Some(stroke) = &self.stroke {
            attrs.push(format!("stroke=\"{}\"", stroke));
        }

        format!("<polygon {} />", attrs.join(" "))
    }
}

impl Default for Polygon {
    fn default() -> Self {
        Self::new()
    }
}

/// 曲线类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CurveType {
    Line,
    Quadratic,
    Cubic,
}

/// 曲线
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Curve {
    pub curve_type: CurveType,
    pub points: Vec<VisualizePoint>,
    pub stroke: Option<String>,
    pub stroke_width: Option<f64>,
}

impl Curve {
    pub fn new() -> Self {
        Self {
            curve_type: CurveType::Line,
            points: Vec::new(),
            stroke: None,
            stroke_width: None,
        }
    }

    pub fn with_type(mut self, curve_type: CurveType) -> Self {
        self.curve_type = curve_type;
        self
    }

    pub fn with_points(mut self, points: Vec<VisualizePoint>) -> Self {
        self.points = points;
        self
    }

    pub fn with_stroke(mut self, stroke: String) -> Self {
        self.stroke = Some(stroke);
        self
    }

    pub fn with_stroke_width(mut self, width: f64) -> Self {
        self.stroke_width = Some(width);
        self
    }

    pub fn add_point(&mut self, point: VisualizePoint) {
        self.points.push(point);
    }

    pub fn to_typst(&self) -> String {
        let points_str: Vec<String> = self
            .points
            .iter()
            .map(|p| format!("({}, {})", p.x, p.y))
            .collect();

        let mut parts = vec![format!("({})", points_str.join(", "))];

        if let Some(stroke) = &self.stroke {
            parts.push(format!("stroke: \"{}\"", stroke));
        }

        if let Some(width) = self.stroke_width {
            parts.push(format!("stroke-width: {}pt", width));
        }

        format!("#curve({})", parts.join(", "))
    }

    pub fn to_svg(&self) -> String {
        if self.points.len() < 2 {
            return String::new();
        }

        let path_data = match self.curve_type {
            CurveType::Line => {
                let first = &self.points[0];
                let rest: Vec<String> = self
                    .points
                    .iter()
                    .skip(1)
                    .map(|p| format!("L {},{}", p.x, p.y))
                    .collect();
                format!("M {},{} {}", first.x, first.y, rest.join(" "))
            }
            CurveType::Quadratic => {
                if self.points.len() >= 3 {
                    let first = &self.points[0];
                    let rest: Vec<String> = self
                        .points
                        .chunks(2)
                        .skip(1)
                        .filter_map(|chunk| {
                            if chunk.len() >= 2 {
                                Some(format!(
                                    "Q {},{} {},{}",
                                    chunk[0].x, chunk[0].y, chunk[1].x, chunk[1].y
                                ))
                            } else {
                                None
                            }
                        })
                        .collect();
                    format!("M {},{} {}", first.x, first.y, rest.join(" "))
                } else {
                    String::new()
                }
            }
            CurveType::Cubic => {
                if self.points.len() >= 4 {
                    let first = &self.points[0];
                    let rest: Vec<String> = self
                        .points
                        .chunks(3)
                        .skip(1)
                        .filter_map(|chunk| {
                            if chunk.len() >= 3 {
                                Some(format!(
                                    "C {},{} {},{} {},{}",
                                    chunk[0].x,
                                    chunk[0].y,
                                    chunk[1].x,
                                    chunk[1].y,
                                    chunk[2].x,
                                    chunk[2].y
                                ))
                            } else {
                                None
                            }
                        })
                        .collect();
                    format!("M {},{} {}", first.x, first.y, rest.join(" "))
                } else {
                    String::new()
                }
            }
        };

        let mut attrs = vec![format!("d=\"{}\"", path_data)];

        if let Some(stroke) = &self.stroke {
            attrs.push(format!("stroke=\"{}\"", stroke));
        } else {
            attrs.push("stroke=\"black\"".to_string());
        }

        if let Some(width) = self.stroke_width {
            attrs.push(format!("stroke-width=\"{}\"", width));
        }

        attrs.push("fill=\"none\"".to_string());

        format!("<path {} />", attrs.join(" "))
    }
}

impl Default for Curve {
    fn default() -> Self {
        Self::new()
    }
}

/// 颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizeColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: Option<f64>,
}

impl VisualizeColor {
    pub fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
            alpha: None,
        }
    }

    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: None,
        }
    }

    pub fn rgba(red: u8, green: u8, blue: u8, alpha: f64) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: Some(alpha),
        }
    }

    pub fn with_alpha(mut self, alpha: f64) -> Self {
        self.alpha = Some(alpha);
        self
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }

    pub fn to_typst(&self) -> String {
        if let Some(alpha) = self.alpha {
            format!("rgb(\"{}\", {})", self.to_hex(), alpha)
        } else {
            format!("rgb(\"{}\")", self.to_hex())
        }
    }
}

impl Default for VisualizeColor {
    fn default() -> Self {
        Self::new()
    }
}

/// 描边样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizeStroke {
    pub color: String,
    pub width: f64,
    pub dash: Option<String>,
    pub cap: Option<String>,
}

impl VisualizeStroke {
    pub fn new() -> Self {
        Self {
            color: "black".to_string(),
            width: 1.0,
            dash: None,
            cap: None,
        }
    }

    pub fn with_color(mut self, color: String) -> Self {
        self.color = color;
        self
    }

    pub fn with_width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    pub fn with_dash(mut self, dash: String) -> Self {
        self.dash = Some(dash);
        self
    }

    pub fn with_cap(mut self, cap: String) -> Self {
        self.cap = Some(cap);
        self
    }

    pub fn to_typst(&self) -> String {
        let mut parts = vec![format!("stroke: \"{}\"", self.color)];
        parts.push(format!("stroke-width: {}pt", self.width));

        if let Some(dash) = &self.dash {
            parts.push(format!("dash: \"{}\"", dash));
        }

        if let Some(cap) = &self.cap {
            parts.push(format!("cap: \"{}\"", cap));
        }

        format!("({})", parts.join(", "))
    }
}

impl Default for VisualizeStroke {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let point = VisualizePoint::new(10.0, 20.0);
        assert_eq!(point.x, 10.0);
        assert_eq!(point.y, 20.0);
    }

    #[test]
    fn test_polygon_creation() {
        let polygon = Polygon::new();
        assert!(polygon.points.is_empty());
    }

    #[test]
    fn test_polygon_add_point() {
        let mut polygon = Polygon::new();
        polygon.add_point(VisualizePoint::new(10.0, 20.0));
        assert_eq!(polygon.points.len(), 1);
    }

    #[test]
    fn test_polygon_to_typst() {
        let polygon = Polygon::new()
            .with_points(vec![
                VisualizePoint::new(0.0, 0.0),
                VisualizePoint::new(10.0, 10.0),
            ])
            .with_fill("red".to_string());
        let typst = polygon.to_typst();
        assert!(typst.contains("polygon") && typst.contains("red"));
    }

    #[test]
    fn test_polygon_to_svg() {
        let polygon = Polygon::new()
            .with_points(vec![
                VisualizePoint::new(0.0, 0.0),
                VisualizePoint::new(10.0, 10.0),
            ])
            .with_fill("blue".to_string());
        let svg = polygon.to_svg();
        assert!(svg.contains("<polygon") && svg.contains("fill=\"blue\""));
    }

    #[test]
    fn test_curve_creation() {
        let curve = Curve::new();
        assert!(curve.points.is_empty());
    }

    #[test]
    fn test_curve_add_point() {
        let mut curve = Curve::new();
        curve.add_point(VisualizePoint::new(10.0, 20.0));
        assert_eq!(curve.points.len(), 1);
    }

    #[test]
    fn test_curve_to_typst() {
        let curve = Curve::new()
            .with_points(vec![
                VisualizePoint::new(0.0, 0.0),
                VisualizePoint::new(10.0, 10.0),
            ])
            .with_stroke("blue".to_string());
        let typst = curve.to_typst();
        assert!(typst.contains("curve") && typst.contains("blue"));
    }

    #[test]
    fn test_curve_to_svg() {
        let curve = Curve::new()
            .with_type(CurveType::Line)
            .with_points(vec![
                VisualizePoint::new(0.0, 0.0),
                VisualizePoint::new(10.0, 10.0),
            ])
            .with_stroke("red".to_string());
        let svg = curve.to_svg();
        assert!(svg.contains("<path") && svg.contains("stroke=\"red\""));
    }

    #[test]
    fn test_color_creation() {
        let color = VisualizeColor::new();
        assert_eq!(color.red, 0);
    }

    #[test]
    fn test_color_rgb() {
        let color = VisualizeColor::rgb(255, 128, 64);
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
    }

    #[test]
    fn test_color_to_hex() {
        let color = VisualizeColor::rgb(255, 128, 64);
        let hex = color.to_hex();
        assert_eq!(hex, "#FF8040");
    }

    #[test]
    fn test_color_to_typst() {
        let color = VisualizeColor::rgb(255, 128, 64);
        let typst = color.to_typst();
        assert!(typst.contains("#FF8040"));
    }

    #[test]
    fn test_color_rgba() {
        let color = VisualizeColor::rgba(255, 128, 64, 0.5);
        let typst = color.to_typst();
        assert!(typst.contains("0.5"));
    }

    #[test]
    fn test_stroke_creation() {
        let stroke = VisualizeStroke::new();
        assert_eq!(stroke.color, "black");
    }

    #[test]
    fn test_stroke_to_typst() {
        let stroke = VisualizeStroke::new()
            .with_color("red".to_string())
            .with_width(2.0);
        let typst = stroke.to_typst();
        assert!(typst.contains("red") && typst.contains("2pt"));
    }

    #[test]
    fn test_curve_type_variants() {
        assert_eq!(CurveType::Line, CurveType::Line);
        assert_eq!(CurveType::Quadratic, CurveType::Quadratic);
        assert_eq!(CurveType::Cubic, CurveType::Cubic);
    }
}
