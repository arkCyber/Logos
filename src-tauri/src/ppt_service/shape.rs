use serde::{Deserialize, Serialize};

/// 形状类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShapeType {
    /// 矩形
    Rectangle,
    /// 圆角矩形
    RoundedRectangle,
    /// 圆形
    Circle,
    /// 椭圆
    Ellipse,
    /// 三角形
    Triangle,
    /// 菱形
    Diamond,
    /// 五角星
    Star,
    /// 箭头
    Arrow,
    /// 线条
    Line,
    /// 自定义
    Custom(String),
}

/// 形状填充
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShapeFill {
    /// 无填充
    None,
    /// 纯色填充（RGB）
    Solid { color: (u8, u8, u8), opacity: f64 },
    /// 渐变填充
    Gradient {
        start: (u8, u8, u8),
        end: (u8, u8, u8),
        angle: f64,
    },
    /// 图片填充
    Image { data: Vec<u8> },
}

impl ShapeFill {
    /// 创建纯色填充
    pub fn solid(r: u8, g: u8, b: u8) -> Self {
        Self::Solid {
            color: (r, g, b),
            opacity: 1.0,
        }
    }

    /// 创建渐变填充
    pub fn gradient(start: (u8, u8, u8), end: (u8, u8, u8), angle: f64) -> Self {
        Self::Gradient { start, end, angle }
    }
}

/// 形状线条
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeLine {
    /// 线条颜色（RGB）
    pub color: (u8, u8, u8),
    /// 线条宽度（点）
    pub width: f64,
    /// 线条样式
    pub style: LineStyle,
    /// 是否有阴影
    pub shadow: bool,
}

/// 线条样式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LineStyle {
    /// 实线
    Solid,
    /// 虚线
    Dashed,
    /// 点线
    Dotted,
    /// 点划线
    DashDot,
}

impl ShapeLine {
    /// 创建默认线条
    pub fn new() -> Self {
        Self {
            color: (0, 0, 0),
            width: 1.0,
            style: LineStyle::Solid,
            shadow: false,
        }
    }

    /// 设置颜色
    pub fn with_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = (r, g, b);
        self
    }

    /// 设置宽度
    pub fn with_width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    /// 设置样式
    pub fn with_style(mut self, style: LineStyle) -> Self {
        self.style = style;
        self
    }
}

impl Default for ShapeLine {
    fn default() -> Self {
        Self::new()
    }
}

/// 形状样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeStyle {
    /// 填充
    pub fill: ShapeFill,
    /// 线条
    pub line: ShapeLine,
    /// 旋转角度（度）
    pub rotation: f64,
    /// 透明度（0.0 - 1.0）
    pub opacity: f64,
}

impl ShapeStyle {
    /// 创建默认样式
    pub fn new() -> Self {
        Self {
            fill: ShapeFill::solid(200, 200, 200),
            line: ShapeLine::new(),
            rotation: 0.0,
            opacity: 1.0,
        }
    }

    /// 设置填充
    pub fn with_fill(mut self, fill: ShapeFill) -> Self {
        self.fill = fill;
        self
    }

    /// 设置线条
    pub fn with_line(mut self, line: ShapeLine) -> Self {
        self.line = line;
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
}

impl Default for ShapeStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// 形状
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shape {
    /// 形状 ID
    pub id: String,
    /// 形状类型
    pub shape_type: ShapeType,
    /// 形状样式
    pub style: ShapeStyle,
    /// 位置（X, Y 坐标，单位：点）
    pub position: (f64, f64),
    /// 尺寸（宽度、高度，单位：点）
    pub size: (f64, f64),
    /// 形状文本（可选）
    pub text: Option<String>,
}

impl Shape {
    /// 创建新的形状
    pub fn new(id: String, shape_type: ShapeType) -> Self {
        Self {
            id,
            shape_type,
            style: ShapeStyle::new(),
            position: (0.0, 0.0),
            size: (100.0, 100.0),
            text: None,
        }
    }

    /// 设置样式
    pub fn with_style(mut self, style: ShapeStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置位置
    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.position = (x, y);
        self
    }

    /// 设置尺寸
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.size = (width, height);
        self
    }

    /// 设置文本
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// 创建矩形
    pub fn rectangle(id: String) -> Self {
        Self::new(id, ShapeType::Rectangle)
    }

    /// 创建圆形
    pub fn circle(id: String) -> Self {
        Self::new(id, ShapeType::Circle)
    }

    /// 创建三角形
    pub fn triangle(id: String) -> Self {
        Self::new(id, ShapeType::Triangle)
    }

    /// 创建箭头
    pub fn arrow(id: String) -> Self {
        Self::new(id, ShapeType::Arrow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_fill_solid() {
        let fill = ShapeFill::solid(255, 0, 0);
        match fill {
            ShapeFill::Solid { color, .. } => {
                assert_eq!(color, (255, 0, 0));
            }
            _ => panic!("Expected solid fill"),
        }
    }

    #[test]
    fn test_shape_line_new() {
        let line = ShapeLine::new();
        assert_eq!(line.width, 1.0);
        assert_eq!(line.style, LineStyle::Solid);
    }

    #[test]
    fn test_shape_line_with_color() {
        let line = ShapeLine::new().with_color(255, 0, 0);
        assert_eq!(line.color, (255, 0, 0));
    }

    #[test]
    fn test_shape_style_new() {
        let style = ShapeStyle::new();
        assert_eq!(style.rotation, 0.0);
        assert_eq!(style.opacity, 1.0);
    }

    #[test]
    fn test_shape_style_with_fill() {
        let fill = ShapeFill::solid(255, 0, 0);
        let style = ShapeStyle::new().with_fill(fill);
        match style.fill {
            ShapeFill::Solid { color, .. } => {
                assert_eq!(color, (255, 0, 0));
            }
            _ => panic!("Expected solid fill"),
        }
    }

    #[test]
    fn test_shape_new() {
        let shape = Shape::new("1".to_string(), ShapeType::Rectangle);
        assert_eq!(shape.id, "1");
        assert_eq!(shape.shape_type, ShapeType::Rectangle);
    }

    #[test]
    fn test_shape_rectangle() {
        let shape = Shape::rectangle("1".to_string());
        assert_eq!(shape.shape_type, ShapeType::Rectangle);
    }

    #[test]
    fn test_shape_circle() {
        let shape = Shape::circle("1".to_string());
        assert_eq!(shape.shape_type, ShapeType::Circle);
    }

    #[test]
    fn test_shape_triangle() {
        let shape = Shape::triangle("1".to_string());
        assert_eq!(shape.shape_type, ShapeType::Triangle);
    }

    #[test]
    fn test_shape_with_text() {
        let shape = Shape::rectangle("1".to_string()).with_text("Label".to_string());
        assert_eq!(shape.text, Some("Label".to_string()));
    }

    #[test]
    fn test_shape_chaining() {
        let shape = Shape::rectangle("1".to_string())
            .with_position(100.0, 200.0)
            .with_size(150.0, 100.0)
            .with_text("Test".to_string());
        assert_eq!(shape.position, (100.0, 200.0));
        assert_eq!(shape.size, (150.0, 100.0));
    }

    #[test]
    fn test_shape_serialization() {
        let shape = Shape::rectangle("1".to_string());
        let json = serde_json::to_string(&shape);
        assert!(json.is_ok());
    }
}
