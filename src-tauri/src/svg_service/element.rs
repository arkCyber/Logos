use super::SvgStyle;
use serde::{Deserialize, Serialize};

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
        let json = r#"{"id":"1","element_type":"Rect","style":{"fill":{"color":null,"opacity":1.0},"stroke":{"color":null,"width":1.0,"opacity":1.0},"stroke_width":1.0},"rect":null,"circle":null,"path":null,"text":null}"#;
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
}
