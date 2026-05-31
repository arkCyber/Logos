/*!
 * 航空航天级框系统
 * 实现 Typst 的框功能（内容容器、边框、背景、内边距、外边距）
 */

use serde::{Deserialize, Serialize};

/// 框尺寸
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoxSize {
    Auto,
    Relative(f64),
    Fraction(f64),
}

/// 边框样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxStroke {
    pub thickness: f64,
    pub color: String,
}

impl Default for BoxStroke {
    fn default() -> Self {
        Self {
            thickness: 0.5,
            color: "black".to_string(),
        }
    }
}

/// 圆角
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoxRadius {
    Uniform(f64),
    Corners {
        top_left: f64,
        top_right: f64,
        bottom_right: f64,
        bottom_left: f64,
    },
}

impl Default for BoxRadius {
    fn default() -> Self {
        Self::Uniform(0.0)
    }
}

/// 内边距/外边距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoxPadding {
    Uniform(f64),
    Sides {
        top: f64,
        right: f64,
        bottom: f64,
        left: f64,
    },
}

impl Default for BoxPadding {
    fn default() -> Self {
        Self::Uniform(0.0)
    }
}

/// 框配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxConfig {
    pub width: BoxSize,
    pub height: BoxSize,
    pub baseline: f64,
    pub fill: Option<String>,
    pub stroke: Option<BoxStroke>,
    pub radius: BoxRadius,
    pub inset: BoxPadding,
    pub outset: BoxPadding,
    pub clip: bool,
}

impl Default for BoxConfig {
    fn default() -> Self {
        Self {
            width: BoxSize::Auto,
            height: BoxSize::Auto,
            baseline: 0.0,
            fill: None,
            stroke: None,
            radius: BoxRadius::default(),
            inset: BoxPadding::default(),
            outset: BoxPadding::default(),
            clip: false,
        }
    }
}

/// 框
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Box {
    pub content: String,
    pub config: BoxConfig,
}

impl Box {
    pub fn new(content: String) -> Self {
        Self {
            content,
            config: BoxConfig::default(),
        }
    }

    pub fn with_config(mut self, config: BoxConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_width(mut self, width: BoxSize) -> Self {
        self.config.width = width;
        self
    }

    pub fn with_height(mut self, height: BoxSize) -> Self {
        self.config.height = height;
        self
    }

    pub fn with_fill(mut self, fill: String) -> Self {
        self.config.fill = Some(fill);
        self
    }

    pub fn with_stroke(mut self, stroke: BoxStroke) -> Self {
        self.config.stroke = Some(stroke);
        self
    }

    pub fn with_radius(mut self, radius: BoxRadius) -> Self {
        self.config.radius = radius;
        self
    }

    pub fn with_inset(mut self, inset: BoxPadding) -> Self {
        self.config.inset = inset;
        self
    }

    pub fn with_outset(mut self, outset: BoxPadding) -> Self {
        self.config.outset = outset;
        self
    }

    pub fn with_clip(mut self, clip: bool) -> Self {
        self.config.clip = clip;
        self
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        typst.push_str("#box(");

        // 添加宽度
        match &self.config.width {
            BoxSize::Auto => {}
            BoxSize::Relative(size) => typst.push_str(&format!("width: {}em, ", size)),
            BoxSize::Fraction(size) => typst.push_str(&format!("width: {}%, ", size * 100.0)),
        }

        // 添加高度
        match &self.config.height {
            BoxSize::Auto => {}
            BoxSize::Relative(size) => typst.push_str(&format!("height: {}em, ", size)),
            BoxSize::Fraction(size) => typst.push_str(&format!("height: {}%, ", size * 100.0)),
        }

        // 添加填充
        if let Some(fill) = &self.config.fill {
            typst.push_str(&format!("fill: \"{}\", ", fill));
        }

        // 添加边框
        if let Some(stroke) = &self.config.stroke {
            typst.push_str(&format!(
                "stroke: {}pt + {}, ",
                stroke.thickness, stroke.color
            ));
        }

        // 添加圆角
        match &self.config.radius {
            BoxRadius::Uniform(0.0) => {}
            BoxRadius::Uniform(size) => typst.push_str(&format!("radius: {}em, ", size)),
            BoxRadius::Corners {
                top_left,
                top_right,
                bottom_right,
                bottom_left,
            } => {
                typst.push_str(&format!(
                    "radius: (top-left: {}em, top-right: {}em, bottom-right: {}em, bottom-left: {}em), ",
                    top_left, top_right, bottom_right, bottom_left
                ));
            }
        }

        // 添加内边距
        match &self.config.inset {
            BoxPadding::Uniform(0.0) => {}
            BoxPadding::Uniform(size) => typst.push_str(&format!("inset: {}em, ", size)),
            BoxPadding::Sides {
                top,
                right,
                bottom,
                left,
            } => {
                typst.push_str(&format!(
                    "inset: (top: {}em, right: {}em, bottom: {}em, left: {}em), ",
                    top, right, bottom, left
                ));
            }
        }

        // 添加外边距
        match &self.config.outset {
            BoxPadding::Uniform(0.0) => {}
            BoxPadding::Uniform(size) => typst.push_str(&format!("outset: {}em, ", size)),
            BoxPadding::Sides {
                top,
                right,
                bottom,
                left,
            } => {
                typst.push_str(&format!(
                    "outset: (top: {}em, right: {}em, bottom: {}em, left: {}em), ",
                    top, right, bottom, left
                ));
            }
        }

        // 添加裁剪
        if self.config.clip {
            typst.push_str("clip: true, ");
        }

        typst.push_str(&format!("[{}])\n", html_escape(&self.content)));

        typst
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        let width_attr = match &self.config.width {
            BoxSize::Auto => String::new(),
            BoxSize::Relative(size) => format!(" width: {}em;", size),
            BoxSize::Fraction(size) => format!(" width: {}%;", size * 100.0),
        };

        let height_attr = match &self.config.height {
            BoxSize::Auto => String::new(),
            BoxSize::Relative(size) => format!(" height: {}em;", size),
            BoxSize::Fraction(size) => format!(" height: {}%;", size * 100.0),
        };

        let fill_attr = if let Some(fill) = &self.config.fill {
            format!(" background-color: {};", fill)
        } else {
            String::new()
        };

        let stroke_attr = if let Some(stroke) = &self.config.stroke {
            format!(" border: {}px solid {};", stroke.thickness, stroke.color)
        } else {
            String::new()
        };

        let radius_attr = match &self.config.radius {
            BoxRadius::Uniform(0.0) => String::new(),
            BoxRadius::Uniform(size) => format!(" border-radius: {}em;", size),
            BoxRadius::Corners {
                top_left,
                top_right,
                bottom_right,
                bottom_left,
            } => {
                format!(
                    " border-radius: {}em {}em {}em {}em;",
                    top_left, top_right, bottom_right, bottom_left
                )
            }
        };

        let padding_attr = match &self.config.inset {
            BoxPadding::Uniform(0.0) => String::new(),
            BoxPadding::Uniform(size) => format!(" padding: {}em;", size),
            BoxPadding::Sides {
                top,
                right,
                bottom,
                left,
            } => {
                format!(" padding: {}em {}em {}em {}em;", top, right, bottom, left)
            }
        };

        let margin_attr = match &self.config.outset {
            BoxPadding::Uniform(0.0) => String::new(),
            BoxPadding::Uniform(size) => format!(" margin: {}em;", size),
            BoxPadding::Sides {
                top,
                right,
                bottom,
                left,
            } => {
                format!(" margin: {}em {}em {}em {}em;", top, right, bottom, left)
            }
        };

        let clip_attr = if self.config.clip {
            " overflow: hidden;"
        } else {
            ""
        };

        html.push_str(&format!(
            "<div class=\"typst-box\" style=\"display: inline-block;{}{}{}{}{}{}{}{}\">\n",
            width_attr,
            height_attr,
            fill_attr,
            stroke_attr,
            radius_attr,
            padding_attr,
            margin_attr,
            clip_attr
        ));
        html.push_str(&format!("  {}\n", html_escape(&self.content)));
        html.push_str("</div>\n");

        html
    }
}

impl Default for Box {
    fn default() -> Self {
        Self::new("".to_string())
    }
}

/// 框构建器
pub struct BoxBuilder {
    inner_box: Box,
}

impl BoxBuilder {
    pub fn new(content: String) -> Self {
        Self {
            inner_box: Box::new(content),
        }
    }

    pub fn width(mut self, width: BoxSize) -> Self {
        self.inner_box = self.inner_box.with_width(width);
        self
    }

    pub fn height(mut self, height: BoxSize) -> Self {
        self.inner_box = self.inner_box.with_height(height);
        self
    }

    pub fn fill(mut self, fill: String) -> Self {
        self.inner_box = self.inner_box.with_fill(fill);
        self
    }

    pub fn stroke(mut self, stroke: BoxStroke) -> Self {
        self.inner_box = self.inner_box.with_stroke(stroke);
        self
    }

    pub fn radius(mut self, radius: BoxRadius) -> Self {
        self.inner_box = self.inner_box.with_radius(radius);
        self
    }

    pub fn inset(mut self, inset: BoxPadding) -> Self {
        self.inner_box = self.inner_box.with_inset(inset);
        self
    }

    pub fn outset(mut self, outset: BoxPadding) -> Self {
        self.inner_box = self.inner_box.with_outset(outset);
        self
    }

    pub fn clip(mut self, clip: bool) -> Self {
        self.inner_box = self.inner_box.with_clip(clip);
        self
    }

    pub fn build(self) -> Box {
        self.inner_box
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
    fn test_box_creation() {
        let test_box = Box::new("Test".to_string());
        assert_eq!(test_box.content, "Test");
    }

    #[test]
    fn test_box_default() {
        let test_box = Box::default();
        assert_eq!(test_box.content, "");
    }

    #[test]
    fn test_box_config_default() {
        let config = BoxConfig::default();
        assert!(matches!(config.width, BoxSize::Auto));
        assert!(!config.clip);
    }

    #[test]
    fn test_box_with_width() {
        let test_box = Box::new("Test".to_string()).with_width(BoxSize::Relative(10.0));
        assert!(matches!(test_box.config.width, BoxSize::Relative(10.0)));
    }

    #[test]
    fn test_box_with_height() {
        let test_box = Box::new("Test".to_string()).with_height(BoxSize::Fraction(0.5));
        assert!(matches!(test_box.config.height, BoxSize::Fraction(0.5)));
    }

    #[test]
    fn test_box_with_fill() {
        let test_box = Box::new("Test".to_string()).with_fill("red".to_string());
        assert_eq!(test_box.config.fill, Some("red".to_string()));
    }

    #[test]
    fn test_box_with_stroke() {
        let stroke = BoxStroke {
            thickness: 2.0,
            color: "blue".to_string(),
        };
        let test_box = Box::new("Test".to_string()).with_stroke(stroke);
        assert!(test_box.config.stroke.is_some());
    }

    #[test]
    fn test_box_with_radius() {
        let test_box = Box::new("Test".to_string()).with_radius(BoxRadius::Uniform(5.0));
        assert!(matches!(test_box.config.radius, BoxRadius::Uniform(5.0)));
    }

    #[test]
    fn test_box_with_inset() {
        let test_box = Box::new("Test".to_string()).with_inset(BoxPadding::Uniform(1.0));
        assert!(matches!(test_box.config.inset, BoxPadding::Uniform(1.0)));
    }

    #[test]
    fn test_box_with_outset() {
        let test_box = Box::new("Test".to_string()).with_outset(BoxPadding::Uniform(1.0));
        assert!(matches!(test_box.config.outset, BoxPadding::Uniform(1.0)));
    }

    #[test]
    fn test_box_with_clip() {
        let test_box = Box::new("Test".to_string()).with_clip(true);
        assert!(test_box.config.clip);
    }

    #[test]
    fn test_to_typst() {
        let test_box = Box::new("Test".to_string());
        let typst = test_box.to_typst();
        assert!(typst.contains("#box("));
        assert!(typst.contains("[Test]"));
    }

    #[test]
    fn test_to_typst_with_width() {
        let test_box = Box::new("Test".to_string()).with_width(BoxSize::Relative(10.0));
        let typst = test_box.to_typst();
        assert!(typst.contains("width: 10em"));
    }

    #[test]
    fn test_to_typst_with_fill() {
        let test_box = Box::new("Test".to_string()).with_fill("red".to_string());
        let typst = test_box.to_typst();
        assert!(typst.contains("fill: \"red\""));
    }

    #[test]
    fn test_to_html() {
        let test_box = Box::new("Test".to_string());
        let html = test_box.to_html();
        assert!(html.contains("<div class=\"typst-box\""));
        assert!(html.contains("Test"));
    }

    #[test]
    fn test_to_html_with_fill() {
        let test_box = Box::new("Test".to_string()).with_fill("red".to_string());
        let html = test_box.to_html();
        assert!(html.contains("background-color: red"));
    }

    #[test]
    fn test_box_stroke_default() {
        let stroke = BoxStroke::default();
        assert_eq!(stroke.thickness, 0.5);
        assert_eq!(stroke.color, "black");
    }

    #[test]
    fn test_box_radius_default() {
        let radius = BoxRadius::default();
        assert!(matches!(radius, BoxRadius::Uniform(0.0)));
    }

    #[test]
    fn test_box_padding_default() {
        let padding = BoxPadding::default();
        assert!(matches!(padding, BoxPadding::Uniform(0.0)));
    }

    #[test]
    fn test_box_builder() {
        let test_box = BoxBuilder::new("Test".to_string())
            .width(BoxSize::Relative(10.0))
            .fill("red".to_string())
            .build();

        assert_eq!(test_box.content, "Test");
        assert!(matches!(test_box.config.width, BoxSize::Relative(10.0)));
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_box_size_variants() {
        let auto = BoxSize::Auto;
        let relative = BoxSize::Relative(10.0);
        let fraction = BoxSize::Fraction(0.5);

        assert!(matches!(auto, BoxSize::Auto));
        assert!(matches!(relative, BoxSize::Relative(_)));
        assert!(matches!(fraction, BoxSize::Fraction(_)));
    }

    #[test]
    fn test_box_radius_corners() {
        let radius = BoxRadius::Corners {
            top_left: 1.0,
            top_right: 2.0,
            bottom_right: 3.0,
            bottom_left: 4.0,
        };
        assert!(matches!(radius, BoxRadius::Corners { .. }));
    }

    #[test]
    fn test_box_padding_sides() {
        let padding = BoxPadding::Sides {
            top: 1.0,
            right: 2.0,
            bottom: 3.0,
            left: 4.0,
        };
        assert!(matches!(padding, BoxPadding::Sides { .. }));
    }

    #[test]
    fn test_to_typst_with_corners_radius() {
        let test_box = Box::new("Test".to_string()).with_radius(BoxRadius::Corners {
            top_left: 1.0,
            top_right: 2.0,
            bottom_right: 3.0,
            bottom_left: 4.0,
        });
        let typst = test_box.to_typst();
        assert!(typst.contains("top-left: 1em"));
    }

    #[test]
    fn test_to_typst_with_sides_inset() {
        let test_box = Box::new("Test".to_string()).with_inset(BoxPadding::Sides {
            top: 1.0,
            right: 2.0,
            bottom: 3.0,
            left: 4.0,
        });
        let typst = test_box.to_typst();
        assert!(typst.contains("top: 1em"));
    }

    #[test]
    fn test_to_html_with_corners_radius() {
        let test_box = Box::new("Test".to_string()).with_radius(BoxRadius::Corners {
            top_left: 1.0,
            top_right: 2.0,
            bottom_right: 3.0,
            bottom_left: 4.0,
        });
        let html = test_box.to_html();
        assert!(html.contains("border-radius: 1em 2em 3em 4em"));
    }

    #[test]
    fn test_to_html_with_clip() {
        let test_box = Box::new("Test".to_string()).with_clip(true);
        let html = test_box.to_html();
        assert!(html.contains("overflow: hidden"));
    }
}
