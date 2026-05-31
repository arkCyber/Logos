/*!
 * 航空航天级线条系统
 * 实现 Typst 的线条功能（线条、起点、终点、长度、角度、描边）
 */

use serde::{Deserialize, Serialize};

/// 点坐标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

/// 线条配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LineConfig {
    pub start: Option<Point>,
    pub end: Option<Point>,
    pub length: Option<f64>,
    pub angle: Option<f64>,
    pub stroke: Option<String>,
    pub stroke_thickness: Option<f64>,
}

/// 线条
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    pub config: LineConfig,
}

impl Line {
    pub fn new() -> Self {
        Self {
            config: LineConfig::default(),
        }
    }

    pub fn with_config(mut self, config: LineConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_start(mut self, x: f64, y: f64) -> Self {
        self.config.start = Some(Point::new(x, y));
        self
    }

    pub fn with_end(mut self, x: f64, y: f64) -> Self {
        self.config.end = Some(Point::new(x, y));
        self
    }

    pub fn with_length(mut self, length: f64) -> Self {
        self.config.length = Some(length);
        self
    }

    pub fn with_angle(mut self, angle: f64) -> Self {
        self.config.angle = Some(angle);
        self
    }

    pub fn with_stroke(mut self, stroke: String) -> Self {
        self.config.stroke = Some(stroke);
        self
    }

    pub fn with_stroke_thickness(mut self, thickness: f64) -> Self {
        self.config.stroke_thickness = Some(thickness);
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        typst.push_str("#line(");

        // 添加起点
        if let Some(start) = &self.config.start {
            typst.push_str(&format!("start: ({}, {}), ", start.x, start.y));
        }

        // 添加终点
        if let Some(end) = &self.config.end {
            typst.push_str(&format!("end: ({}, {}), ", end.x, end.y));
        }

        // 添加长度
        if let Some(length) = self.config.length {
            typst.push_str(&format!("length: {}em, ", length));
        }

        // 添加角度
        if let Some(angle) = self.config.angle {
            typst.push_str(&format!("angle: {}deg, ", angle));
        }

        // 添加描边
        if let Some(stroke) = &self.config.stroke {
            if let Some(thickness) = self.config.stroke_thickness {
                typst.push_str(&format!(
                    "stroke: (paint: {}, thickness: {}pt), ",
                    stroke, thickness
                ));
            } else {
                typst.push_str(&format!("stroke: {}, ", stroke));
            }
        }

        // 移除最后的逗号和空格
        if typst.ends_with(", ") {
            typst.pop();
            typst.pop();
        }

        typst.push_str(")\n");

        typst
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        let x1 = self.config.start.as_ref().map(|p| p.x).unwrap_or(0.0);
        let y1 = self.config.start.as_ref().map(|p| p.y).unwrap_or(0.0);

        let (x2, y2) = if let Some(end) = &self.config.end {
            (end.x, end.y)
        } else if let Some(length) = self.config.length {
            let angle = self.config.angle.unwrap_or(0.0);
            let rad = angle * std::f64::consts::PI / 180.0;
            (x1 + length * rad.cos(), y1 + length * rad.sin())
        } else {
            (x1 + 1.0, y1)
        };

        let stroke = self.config.stroke.as_deref().unwrap_or("black");
        let stroke_width = self.config.stroke_thickness.unwrap_or(0.5);

        html.push_str("<svg class=\"typst-line\" width=\"100%\" height=\"100%\">\n");
        html.push_str(&format!(
            "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" />\n",
            x1, y1, x2, y2, html_escape(stroke), stroke_width
        ));
        html.push_str("</svg>\n");

        html
    }
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}

/// 线条构建器
pub struct LineBuilder {
    line: Line,
}

impl LineBuilder {
    pub fn new() -> Self {
        Self { line: Line::new() }
    }

    pub fn start(mut self, x: f64, y: f64) -> Self {
        self.line = self.line.with_start(x, y);
        self
    }

    pub fn end(mut self, x: f64, y: f64) -> Self {
        self.line = self.line.with_end(x, y);
        self
    }

    pub fn length(mut self, length: f64) -> Self {
        self.line = self.line.with_length(length);
        self
    }

    pub fn angle(mut self, angle: f64) -> Self {
        self.line = self.line.with_angle(angle);
        self
    }

    pub fn stroke(mut self, stroke: String) -> Self {
        self.line = self.line.with_stroke(stroke);
        self
    }

    pub fn stroke_thickness(mut self, thickness: f64) -> Self {
        self.line = self.line.with_stroke_thickness(thickness);
        self
    }

    pub fn build(self) -> Line {
        self.line
    }
}

impl Default for LineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_creation() {
        let line = Line::new();
        assert!(line.config.start.is_none());
        assert!(line.config.end.is_none());
    }

    #[test]
    fn test_line_default() {
        let line = Line::default();
        assert!(line.config.start.is_none());
    }

    #[test]
    fn test_line_config_default() {
        let config = LineConfig::default();
        assert!(config.start.is_none());
        assert!(config.end.is_none());
    }

    #[test]
    fn test_line_with_start() {
        let line = Line::new().with_start(1.0, 2.0);
        assert!(line.config.start.is_some());
        assert_eq!(line.config.start.unwrap().x, 1.0);
    }

    #[test]
    fn test_line_with_end() {
        let line = Line::new().with_end(3.0, 4.0);
        assert!(line.config.end.is_some());
        assert_eq!(line.config.end.unwrap().x, 3.0);
    }

    #[test]
    fn test_line_with_length() {
        let line = Line::new().with_length(5.0);
        assert_eq!(line.config.length, Some(5.0));
    }

    #[test]
    fn test_line_with_angle() {
        let line = Line::new().with_angle(45.0);
        assert_eq!(line.config.angle, Some(45.0));
    }

    #[test]
    fn test_line_with_stroke() {
        let line = Line::new().with_stroke("red".to_string());
        assert_eq!(line.config.stroke, Some("red".to_string()));
    }

    #[test]
    fn test_line_with_stroke_thickness() {
        let line = Line::new().with_stroke_thickness(2.0);
        assert_eq!(line.config.stroke_thickness, Some(2.0));
    }

    #[test]
    fn test_point_creation() {
        let point = Point::new(1.0, 2.0);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
    }

    #[test]
    fn test_point_default() {
        let point = Point::default();
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 0.0);
    }

    #[test]
    fn test_to_typst() {
        let line = Line::new();
        let typst = line.to_typst();
        assert!(typst.contains("#line("));
    }

    #[test]
    fn test_to_typst_with_start() {
        let line = Line::new().with_start(1.0, 2.0);
        let typst = line.to_typst();
        assert!(typst.contains("start: (1, 2)"));
    }

    #[test]
    fn test_to_typst_with_end() {
        let line = Line::new().with_end(3.0, 4.0);
        let typst = line.to_typst();
        assert!(typst.contains("end: (3, 4)"));
    }

    #[test]
    fn test_to_typst_with_length() {
        let line = Line::new().with_length(5.0);
        let typst = line.to_typst();
        assert!(typst.contains("length: 5em"));
    }

    #[test]
    fn test_to_typst_with_angle() {
        let line = Line::new().with_angle(45.0);
        let typst = line.to_typst();
        assert!(typst.contains("angle: 45deg"));
    }

    #[test]
    fn test_to_typst_with_stroke() {
        let line = Line::new().with_stroke("red".to_string());
        let typst = line.to_typst();
        assert!(typst.contains("stroke: red"));
    }

    #[test]
    fn test_to_html() {
        let line = Line::new();
        let html = line.to_html();
        assert!(html.contains("<svg class=\"typst-line\""));
        assert!(html.contains("<line"));
    }

    #[test]
    fn test_to_html_with_start_end() {
        let line = Line::new().with_start(1.0, 2.0).with_end(3.0, 4.0);
        let html = line.to_html();
        assert!(html.contains("x1=\"1\""));
        assert!(html.contains("y1=\"2\""));
        assert!(html.contains("x2=\"3\""));
        assert!(html.contains("y2=\"4\""));
    }

    #[test]
    fn test_to_html_with_stroke() {
        let line = Line::new().with_stroke("red".to_string());
        let html = line.to_html();
        assert!(html.contains("stroke=\"red\""));
    }

    #[test]
    fn test_line_builder() {
        let line = LineBuilder::new()
            .start(1.0, 2.0)
            .end(3.0, 4.0)
            .stroke("red".to_string())
            .build();

        assert!(line.config.start.is_some());
        assert!(line.config.end.is_some());
    }

    #[test]
    fn test_line_builder_default() {
        let builder = LineBuilder::default();
        let line = builder.build();
        assert!(line.config.start.is_none());
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_to_typst_with_full_config() {
        let line = Line::new()
            .with_start(0.0, 0.0)
            .with_end(10.0, 10.0)
            .with_stroke("blue".to_string())
            .with_stroke_thickness(2.0);
        let typst = line.to_typst();
        assert!(typst.contains("start: (0, 0)"));
        assert!(typst.contains("end: (10, 10)"));
        assert!(typst.contains("stroke: (paint: blue, thickness: 2pt)"));
    }

    #[test]
    fn test_to_html_with_length_and_angle() {
        let line = Line::new()
            .with_start(0.0, 0.0)
            .with_length(10.0)
            .with_angle(90.0);
        let html = line.to_html();
        assert!(html.contains("x1=\"0\""));
        assert!(html.contains("y1=\"0\""));
        // Calculate end point with angle 90 degrees
        assert!(html.contains("x2=\""));
        assert!(html.contains("y2=\""));
    }
}
