/*!
 * Aerospace-Grade SVG Element Model
 *
 * Defines validated SVG primitive types (rect, circle, ellipse, line, path,
 * text, polygon, polyline) with builder helpers and serialization support.
 */

use super::SvgStyle;
use serde::{Deserialize, Serialize};

/// Maximum absolute coordinate value (prevents overflow / invalid SVG output)
pub const MAX_COORDINATE_ABS: f64 = 1_000_000.0;

/// Maximum path data string length in bytes
pub const MAX_PATH_DATA_LENGTH: usize = 1_048_576;

/// Maximum number of points in polygon / polyline elements
pub const MAX_POINTS_PER_SHAPE: usize = 10_000;

/// Maximum text content length per text element in bytes
pub const MAX_TEXT_CONTENT_LENGTH: usize = 1_048_576;

/// SVG 元素类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SvgElementType {
    /// 矩形
    Rect,
    /// 圆形
    Circle,
    /// 椭圆
    Ellipse,
    /// 线条
    Line,
    /// 路径
    Path,
    /// 文本
    Text,
    /// 多边形
    Polygon,
    /// 折线
    Polyline,
}

/// SVG 矩形
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgRect {
    /// x 坐标
    pub x: f64,
    /// y 坐标
    pub y: f64,
    /// 宽度
    pub width: f64,
    /// 高度
    pub height: f64,
    /// 圆角 x
    pub rx: Option<f64>,
    /// 圆角 y
    pub ry: Option<f64>,
}

impl SvgRect {
    /// 创建新的矩形
    #[allow(dead_code)]
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
            rx: None,
            ry: None,
        }
    }

    /// 设置圆角
    #[allow(dead_code)]
    pub fn with_radius(mut self, rx: f64, ry: f64) -> Self {
        self.rx = Some(rx);
        self.ry = Some(ry);
        self
    }
}

/// SVG 圆形
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgCircle {
    /// 圆心 x 坐标
    pub cx: f64,
    /// 圆心 y 坐标
    pub cy: f64,
    /// 半径
    pub r: f64,
}

impl SvgCircle {
    /// 创建新的圆形
    #[allow(dead_code)]
    pub fn new(cx: f64, cy: f64, r: f64) -> Self {
        Self { cx, cy, r }
    }
}

/// SVG 路径
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgPath {
    /// 路径数据
    pub d: String,
}

impl SvgPath {
    /// 创建新的路径
    #[allow(dead_code)]
    pub fn new(d: String) -> Self {
        Self { d }
    }
}

/// SVG 文本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgText {
    /// x 坐标
    pub x: f64,
    /// y 坐标
    pub y: f64,
    /// 文本内容
    pub text: String,
}

impl SvgText {
    /// 创建新的文本
    pub fn new(x: f64, y: f64, text: String) -> Self {
        Self { x, y, text }
    }
}

/// SVG 椭圆
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgEllipse {
    /// 圆心 x 坐标
    pub cx: f64,
    /// 圆心 y 坐标
    pub cy: f64,
    /// x 轴半径
    pub rx: f64,
    /// y 轴半径
    pub ry: f64,
}

impl SvgEllipse {
    /// 创建新的椭圆
    #[allow(dead_code)]
    pub fn new(cx: f64, cy: f64, rx: f64, ry: f64) -> Self {
        Self { cx, cy, rx, ry }
    }
}

/// SVG 线条
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgLine {
    /// 起点 x
    pub x1: f64,
    /// 起点 y
    pub y1: f64,
    /// 终点 x
    pub x2: f64,
    /// 终点 y
    pub y2: f64,
}

impl SvgLine {
    /// 创建新的线条
    #[allow(dead_code)]
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self { x1, y1, x2, y2 }
    }
}

/// SVG 坐标点（用于 polygon / polyline）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgPoint {
    /// x 坐标
    pub x: f64,
    /// y 坐标
    pub y: f64,
}

impl SvgPoint {
    /// 创建新的坐标点
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// SVG 多边形
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgPolygon {
    /// 顶点列表
    pub points: Vec<SvgPoint>,
}

impl SvgPolygon {
    /// 创建新的多边形
    #[allow(dead_code)]
    pub fn new(points: Vec<SvgPoint>) -> Self {
        Self { points }
    }

    /// 将顶点格式化为 SVG points 属性值
    pub fn points_attribute(&self) -> String {
        self.points
            .iter()
            .map(|p| format!("{},{}", p.x, p.y))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// SVG 折线
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgPolyline {
    /// 顶点列表
    pub points: Vec<SvgPoint>,
}

impl SvgPolyline {
    /// 创建新的折线
    #[allow(dead_code)]
    pub fn new(points: Vec<SvgPoint>) -> Self {
        Self { points }
    }

    /// 将顶点格式化为 SVG points 属性值
    pub fn points_attribute(&self) -> String {
        self.points
            .iter()
            .map(|p| format!("{},{}", p.x, p.y))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// Validate a single coordinate value for finite range and bounds.
pub fn validate_coordinate(value: f64, name: &str) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("{} must be a finite number", name));
    }
    if value.abs() > MAX_COORDINATE_ABS {
        return Err(format!(
            "{} absolute value exceeds maximum of {}",
            name, MAX_COORDINATE_ABS
        ));
    }
    Ok(())
}

/// Validate polygon / polyline point list length and coordinate bounds.
pub fn validate_points(points: &[SvgPoint], label: &str) -> Result<(), String> {
    if points.is_empty() {
        return Err(format!("{} must contain at least one point", label));
    }
    if points.len() > MAX_POINTS_PER_SHAPE {
        return Err(format!(
            "{} point count exceeds maximum of {}",
            label, MAX_POINTS_PER_SHAPE
        ));
    }
    for (idx, point) in points.iter().enumerate() {
        validate_coordinate(point.x, &format!("{} point[{}].x", label, idx))?;
        validate_coordinate(point.y, &format!("{} point[{}].y", label, idx))?;
    }
    Ok(())
}

/// Validate path data string length and character safety.
pub fn validate_path_data(d: &str) -> Result<(), String> {
    if d.is_empty() {
        return Err("Path data must not be empty".to_string());
    }
    if d.len() > MAX_PATH_DATA_LENGTH {
        return Err(format!(
            "Path data exceeds maximum length of {}",
            MAX_PATH_DATA_LENGTH
        ));
    }
    if d.contains('\0') {
        return Err("Path data must not contain null bytes".to_string());
    }
    Ok(())
}

/// Validate text content length.
pub fn validate_text_content(text: &str) -> Result<(), String> {
    if text.len() > MAX_TEXT_CONTENT_LENGTH {
        return Err(format!(
            "Text content exceeds maximum length of {}",
            MAX_TEXT_CONTENT_LENGTH
        ));
    }
    Ok(())
}

/// SVG 元素
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgElement {
    /// 元素 ID
    pub id: String,
    /// 元素类型
    pub element_type: SvgElementType,
    /// 样式
    pub style: SvgStyle,
    /// 矩形数据（如果是矩形）
    pub rect: Option<SvgRect>,
    /// 圆形数据（如果是圆形）
    pub circle: Option<SvgCircle>,
    /// 路径数据（如果是路径）
    pub path: Option<SvgPath>,
    /// 文本数据（如果是文本）
    pub text: Option<SvgText>,
    /// 椭圆数据（如果是椭圆）
    pub ellipse: Option<SvgEllipse>,
    /// 线条数据（如果是线条）
    pub line: Option<SvgLine>,
    /// 多边形数据（如果是多边形）
    pub polygon: Option<SvgPolygon>,
    /// 折线数据（如果是折线）
    pub polyline: Option<SvgPolyline>,
}

impl SvgElement {
    /// 创建新的元素
    pub fn new(id: String, element_type: SvgElementType) -> Self {
        Self {
            id,
            element_type,
            style: SvgStyle::new(),
            rect: None,
            circle: None,
            path: None,
            text: None,
            ellipse: None,
            line: None,
            polygon: None,
            polyline: None,
        }
    }

    /// 设置样式
    #[allow(dead_code)]
    pub fn with_style(mut self, style: SvgStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置矩形数据
    #[allow(dead_code)]
    pub fn with_rect(mut self, rect: SvgRect) -> Self {
        self.rect = Some(rect);
        self
    }

    /// 设置圆形数据
    #[allow(dead_code)]
    pub fn with_circle(mut self, circle: SvgCircle) -> Self {
        self.circle = Some(circle);
        self
    }

    /// 设置路径数据
    #[allow(dead_code)]
    pub fn with_path(mut self, path: SvgPath) -> Self {
        self.path = Some(path);
        self
    }

    /// 设置文本数据
    #[allow(dead_code)]
    pub fn with_text(mut self, text: SvgText) -> Self {
        self.text = Some(text);
        self
    }

    /// 设置椭圆数据
    #[allow(dead_code)]
    pub fn with_ellipse(mut self, ellipse: SvgEllipse) -> Self {
        self.ellipse = Some(ellipse);
        self
    }

    /// 设置线条数据
    #[allow(dead_code)]
    pub fn with_line(mut self, line: SvgLine) -> Self {
        self.line = Some(line);
        self
    }

    /// 设置多边形数据
    #[allow(dead_code)]
    pub fn with_polygon(mut self, polygon: SvgPolygon) -> Self {
        self.polygon = Some(polygon);
        self
    }

    /// 设置折线数据
    #[allow(dead_code)]
    pub fn with_polyline(mut self, polyline: SvgPolyline) -> Self {
        self.polyline = Some(polyline);
        self
    }

    /// 创建矩形元素
    #[allow(dead_code)]
    pub fn rect(id: String, x: f64, y: f64, width: f64, height: f64) -> Self {
        Self::new(id, SvgElementType::Rect).with_rect(SvgRect::new(x, y, width, height))
    }

    /// 创建圆形元素
    #[allow(dead_code)]
    pub fn circle(id: String, cx: f64, cy: f64, r: f64) -> Self {
        Self::new(id, SvgElementType::Circle).with_circle(SvgCircle::new(cx, cy, r))
    }

    /// 创建文本元素
    pub fn text(id: String, x: f64, y: f64, text: String) -> Self {
        Self::new(id, SvgElementType::Text).with_text(SvgText::new(x, y, text))
    }

    /// 创建椭圆元素
    #[allow(dead_code)]
    pub fn ellipse(id: String, cx: f64, cy: f64, rx: f64, ry: f64) -> Self {
        Self::new(id, SvgElementType::Ellipse).with_ellipse(SvgEllipse::new(cx, cy, rx, ry))
    }

    /// 创建线条元素
    #[allow(dead_code)]
    pub fn line(id: String, x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self::new(id, SvgElementType::Line).with_line(SvgLine::new(x1, y1, x2, y2))
    }

    /// 创建多边形元素
    #[allow(dead_code)]
    pub fn polygon(id: String, points: Vec<SvgPoint>) -> Self {
        Self::new(id, SvgElementType::Polygon).with_polygon(SvgPolygon::new(points))
    }

    /// 创建折线元素
    #[allow(dead_code)]
    pub fn polyline(id: String, points: Vec<SvgPoint>) -> Self {
        Self::new(id, SvgElementType::Polyline).with_polyline(SvgPolyline::new(points))
    }

    /// Validate element geometry and payload before export.
    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("Element id must not be empty".to_string());
        }
        if self.id.len() > 256 {
            return Err("Element id exceeds maximum length of 256".to_string());
        }

        match self.element_type {
            SvgElementType::Rect => {
                let rect = self
                    .rect
                    .as_ref()
                    .ok_or_else(|| "Rect element missing rect data".to_string())?;
                validate_coordinate(rect.x, "rect.x")?;
                validate_coordinate(rect.y, "rect.y")?;
                validate_coordinate(rect.width, "rect.width")?;
                validate_coordinate(rect.height, "rect.height")?;
                if rect.width < 0.0 || rect.height < 0.0 {
                    return Err("Rect width and height must be non-negative".to_string());
                }
                if let Some(rx) = rect.rx {
                    validate_coordinate(rx, "rect.rx")?;
                }
                if let Some(ry) = rect.ry {
                    validate_coordinate(ry, "rect.ry")?;
                }
            }
            SvgElementType::Circle => {
                let circle = self
                    .circle
                    .as_ref()
                    .ok_or_else(|| "Circle element missing circle data".to_string())?;
                validate_coordinate(circle.cx, "circle.cx")?;
                validate_coordinate(circle.cy, "circle.cy")?;
                validate_coordinate(circle.r, "circle.r")?;
                if circle.r < 0.0 {
                    return Err("Circle radius must be non-negative".to_string());
                }
            }
            SvgElementType::Ellipse => {
                let ellipse = self
                    .ellipse
                    .as_ref()
                    .ok_or_else(|| "Ellipse element missing ellipse data".to_string())?;
                validate_coordinate(ellipse.cx, "ellipse.cx")?;
                validate_coordinate(ellipse.cy, "ellipse.cy")?;
                validate_coordinate(ellipse.rx, "ellipse.rx")?;
                validate_coordinate(ellipse.ry, "ellipse.ry")?;
                if ellipse.rx < 0.0 || ellipse.ry < 0.0 {
                    return Err("Ellipse radii must be non-negative".to_string());
                }
            }
            SvgElementType::Line => {
                let line = self
                    .line
                    .as_ref()
                    .ok_or_else(|| "Line element missing line data".to_string())?;
                validate_coordinate(line.x1, "line.x1")?;
                validate_coordinate(line.y1, "line.y1")?;
                validate_coordinate(line.x2, "line.x2")?;
                validate_coordinate(line.y2, "line.y2")?;
            }
            SvgElementType::Path => {
                let path = self
                    .path
                    .as_ref()
                    .ok_or_else(|| "Path element missing path data".to_string())?;
                validate_path_data(&path.d)?;
            }
            SvgElementType::Text => {
                let text = self
                    .text
                    .as_ref()
                    .ok_or_else(|| "Text element missing text data".to_string())?;
                validate_coordinate(text.x, "text.x")?;
                validate_coordinate(text.y, "text.y")?;
                validate_text_content(&text.text)?;
            }
            SvgElementType::Polygon => {
                let polygon = self
                    .polygon
                    .as_ref()
                    .ok_or_else(|| "Polygon element missing polygon data".to_string())?;
                validate_points(&polygon.points, "polygon")?;
            }
            SvgElementType::Polyline => {
                let polyline = self
                    .polyline
                    .as_ref()
                    .ok_or_else(|| "Polyline element missing polyline data".to_string())?;
                validate_points(&polyline.points, "polyline")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_rect_new() {
        let rect = SvgRect::new(10.0, 10.0, 100.0, 50.0);
        assert_eq!(rect.width, 100.0);
    }

    #[test]
    fn test_svg_rect_with_radius() {
        let rect = SvgRect::new(10.0, 10.0, 100.0, 50.0).with_radius(5.0, 5.0);
        assert_eq!(rect.rx, Some(5.0));
    }

    #[test]
    fn test_svg_circle_new() {
        let circle = SvgCircle::new(50.0, 50.0, 25.0);
        assert_eq!(circle.r, 25.0);
    }

    #[test]
    fn test_svg_path_new() {
        let path = SvgPath::new("M 10 10 L 100 100".to_string());
        assert_eq!(path.d, "M 10 10 L 100 100");
    }

    #[test]
    fn test_svg_text_new() {
        let text = SvgText::new(10.0, 10.0, "Hello".to_string());
        assert_eq!(text.text, "Hello");
    }

    #[test]
    fn test_svg_element_new() {
        let element = SvgElement::new("1".to_string(), SvgElementType::Rect);
        assert_eq!(element.element_type, SvgElementType::Rect);
    }

    #[test]
    fn test_svg_element_rect() {
        let element = SvgElement::rect("1".to_string(), 10.0, 10.0, 100.0, 50.0);
        assert!(element.rect.is_some());
    }

    #[test]
    fn test_svg_element_circle() {
        let element = SvgElement::circle("1".to_string(), 50.0, 50.0, 25.0);
        assert!(element.circle.is_some());
    }

    #[test]
    fn test_svg_element_text() {
        let element = SvgElement::text("1".to_string(), 10.0, 10.0, "Hello".to_string());
        assert!(element.text.is_some());
    }

    #[test]
    fn test_svg_element_serialization() {
        let element = SvgElement::new("1".to_string(), SvgElementType::Rect);
        let json = serde_json::to_string(&element);
        assert!(json.is_ok());
    }

    #[test]
    fn test_svg_element_deserialization() {
        let json = r#"{"id":"1","element_type":"Rect","style":{"fill":{"color":null,"gradient_ref":null,"opacity":1.0},"stroke":{"color":null,"width":1.0,"opacity":1.0},"font":null},"rect":null,"circle":null,"path":null,"text":null,"ellipse":null,"line":null,"polygon":null,"polyline":null}"#;
        let element: SvgElement = serde_json::from_str(json).unwrap();
        assert_eq!(element.element_type, SvgElementType::Rect);
    }

    #[test]
    fn test_svg_rect_serialization() {
        let rect = SvgRect::new(10.0, 10.0, 100.0, 50.0);
        let json = serde_json::to_string(&rect);
        assert!(json.is_ok());
    }

    #[test]
    fn test_svg_circle_serialization() {
        let circle = SvgCircle::new(50.0, 50.0, 25.0);
        let json = serde_json::to_string(&circle);
        assert!(json.is_ok());
    }

    #[test]
    fn test_svg_path_serialization() {
        let path = SvgPath::new("M 10 10 L 100 100".to_string());
        let json = serde_json::to_string(&path);
        assert!(json.is_ok());
    }

    #[test]
    fn test_svg_text_serialization() {
        let text = SvgText::new(10.0, 10.0, "Hello".to_string());
        let json = serde_json::to_string(&text);
        assert!(json.is_ok());
    }

    #[test]
    fn test_svg_element_type_variants() {
        assert_eq!(SvgElementType::Rect, SvgElementType::Rect);
        assert_eq!(SvgElementType::Circle, SvgElementType::Circle);
        assert_eq!(SvgElementType::Path, SvgElementType::Path);
        assert_eq!(SvgElementType::Text, SvgElementType::Text);
    }

    #[test]
    fn test_svg_element_type_serialization() {
        let etype = SvgElementType::Rect;
        let json = serde_json::to_string(&etype);
        assert!(json.is_ok());
    }

    #[test]
    fn test_svg_element_type_deserialization() {
        let json = r#""Rect""#;
        let etype: SvgElementType = serde_json::from_str(json).unwrap();
        assert_eq!(etype, SvgElementType::Rect);
    }

    #[test]
    fn test_svg_rect_with_position() {
        let rect = SvgRect::new(50.0, 50.0, 100.0, 50.0);
        assert_eq!(rect.x, 50.0);
        assert_eq!(rect.y, 50.0);
    }

    #[test]
    fn test_svg_rect_with_size() {
        let rect = SvgRect::new(10.0, 10.0, 200.0, 150.0);
        assert_eq!(rect.width, 200.0);
        assert_eq!(rect.height, 150.0);
    }

    #[test]
    fn test_svg_circle_with_position() {
        let circle = SvgCircle::new(100.0, 100.0, 25.0);
        assert_eq!(circle.cx, 100.0);
        assert_eq!(circle.cy, 100.0);
    }

    #[test]
    fn test_svg_circle_with_radius() {
        let circle = SvgCircle::new(50.0, 50.0, 50.0);
        assert_eq!(circle.r, 50.0);
    }

    #[test]
    fn test_svg_path_with_complex_d() {
        let path = SvgPath::new("M 10 10 L 100 100 L 200 50 Z".to_string());
        assert_eq!(path.d, "M 10 10 L 100 100 L 200 50 Z");
    }

    #[test]
    fn test_svg_text_with_content() {
        let text = SvgText::new(10.0, 10.0, "Hello World".to_string());
        assert_eq!(text.text, "Hello World");
    }

    #[test]
    fn test_svg_text_with_position() {
        let text = SvgText::new(100.0, 200.0, "Test".to_string());
        assert_eq!(text.x, 100.0);
        assert_eq!(text.y, 200.0);
    }

    #[test]
    fn test_svg_rect_zero_dimensions() {
        let rect = SvgRect::new(0.0, 0.0, 0.0, 0.0);
        assert_eq!(rect.width, 0.0);
        assert_eq!(rect.height, 0.0);
    }

    #[test]
    fn test_svg_circle_zero_radius() {
        let circle = SvgCircle::new(50.0, 50.0, 0.0);
        assert_eq!(circle.r, 0.0);
    }

    #[test]
    fn test_svg_text_empty_content() {
        let text = SvgText::new(10.0, 10.0, "".to_string());
        assert_eq!(text.text, "");
    }

    #[test]
    fn test_svg_path_empty_d() {
        let path = SvgPath::new("".to_string());
        assert_eq!(path.d, "");
    }

    #[test]
    fn test_svg_ellipse_element() {
        let element = SvgElement::ellipse("e1".to_string(), 50.0, 50.0, 20.0, 10.0);
        assert!(element.ellipse.is_some());
        assert!(element.validate().is_ok());
    }

    #[test]
    fn test_svg_line_element() {
        let element = SvgElement::line("l1".to_string(), 0.0, 0.0, 100.0, 100.0);
        assert!(element.line.is_some());
        assert!(element.validate().is_ok());
    }

    #[test]
    fn test_svg_polygon_element() {
        let points = vec![
            SvgPoint::new(0.0, 0.0),
            SvgPoint::new(100.0, 0.0),
            SvgPoint::new(50.0, 100.0),
        ];
        let element = SvgElement::polygon("p1".to_string(), points);
        assert!(element.polygon.is_some());
        assert!(element.validate().is_ok());
    }

    #[test]
    fn test_svg_polyline_element() {
        let points = vec![SvgPoint::new(0.0, 0.0), SvgPoint::new(10.0, 10.0)];
        let element = SvgElement::polyline("pl1".to_string(), points);
        assert!(element.polyline.is_some());
        assert!(element.validate().is_ok());
    }

    #[test]
    fn test_validate_rejects_non_finite_coordinate() {
        let element = SvgElement::circle("c1".to_string(), f64::NAN, 0.0, 10.0);
        assert!(element.validate().is_err());
    }

    #[test]
    fn test_validate_rejects_empty_polygon() {
        let element = SvgElement::polygon("p-empty".to_string(), vec![]);
        assert!(element.validate().is_err());
    }

    #[test]
    fn test_validate_rejects_empty_path() {
        let element = SvgElement::new("path-empty".to_string(), SvgElementType::Path)
            .with_path(SvgPath::new(String::new()));
        assert!(element.validate().is_err());
    }

    #[test]
    fn test_polygon_points_attribute() {
        let polygon = SvgPolygon::new(vec![SvgPoint::new(1.0, 2.0), SvgPoint::new(3.0, 4.0)]);
        assert_eq!(polygon.points_attribute(), "1,2 3,4");
    }
}
