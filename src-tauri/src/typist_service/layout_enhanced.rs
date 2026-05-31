/*!
 * 航空航天级 Layout 增强模块
 * 实现 Typst 的 Layout 增强功能（align、block、measure、place、fraction）
 */

use serde::{Deserialize, Serialize};

/// 对齐方向
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Alignment {
    Auto,
    Left,
    Right,
    Center,
    Top,
    Bottom,
    Start,
    End,
}

/// 对齐配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlignConfig {
    pub horizontal: Alignment,
    pub vertical: Alignment,
}

impl Default for AlignConfig {
    fn default() -> Self {
        Self {
            horizontal: Alignment::Auto,
            vertical: Alignment::Auto,
        }
    }
}

/// 对齐操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Align {
    pub config: AlignConfig,
}

impl Align {
    pub fn new() -> Self {
        Self {
            config: AlignConfig::default(),
        }
    }

    pub fn with_horizontal(mut self, alignment: Alignment) -> Self {
        self.config.horizontal = alignment;
        self
    }

    pub fn with_vertical(mut self, alignment: Alignment) -> Self {
        self.config.vertical = alignment;
        self
    }

    pub fn to_typst(&self) -> String {
        let mut parts = Vec::new();

        if self.config.horizontal != Alignment::Auto {
            parts.push(format!(
                "horizontal: {}",
                self.alignment_to_typst(&self.config.horizontal)
            ));
        }

        if self.config.vertical != Alignment::Auto {
            parts.push(format!(
                "vertical: {}",
                self.alignment_to_typst(&self.config.vertical)
            ));
        }

        if parts.is_empty() {
            "#align()".to_string()
        } else {
            format!("#align({})", parts.join(", "))
        }
    }

    fn alignment_to_typst(&self, alignment: &Alignment) -> String {
        match alignment {
            Alignment::Auto => "auto".to_string(),
            Alignment::Left => "left".to_string(),
            Alignment::Right => "right".to_string(),
            Alignment::Center => "center".to_string(),
            Alignment::Top => "top".to_string(),
            Alignment::Bottom => "bottom".to_string(),
            Alignment::Start => "start".to_string(),
            Alignment::End => "end".to_string(),
        }
    }
}

impl Default for Align {
    fn default() -> Self {
        Self::new()
    }
}

/// 块级容器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub radius: Option<f64>,
    pub inset: Option<f64>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            fill: None,
            stroke: None,
            radius: None,
            inset: None,
        }
    }

    pub fn with_width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: f64) -> Self {
        self.height = Some(height);
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

    pub fn with_radius(mut self, radius: f64) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn with_inset(mut self, inset: f64) -> Self {
        self.inset = Some(inset);
        self
    }

    pub fn to_typst(&self) -> String {
        let mut parts = Vec::new();

        if let Some(width) = self.width {
            parts.push(format!("width: {}pt", width));
        }

        if let Some(height) = self.height {
            parts.push(format!("height: {}pt", height));
        }

        if let Some(fill) = &self.fill {
            parts.push(format!("fill: \"{}\"", fill));
        }

        if let Some(stroke) = &self.stroke {
            parts.push(format!("stroke: \"{}\"", stroke));
        }

        if let Some(radius) = self.radius {
            parts.push(format!("radius: {}pt", radius));
        }

        if let Some(inset) = self.inset {
            parts.push(format!("inset: {}pt", inset));
        }

        if parts.is_empty() {
            "#block()".to_string()
        } else {
            format!("#block({})", parts.join(", "))
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

/// 测量结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureResult {
    pub width: f64,
    pub height: f64,
    pub ascent: f64,
    pub descent: f64,
}

/// 测量操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measure;

impl Measure {
    pub fn new() -> Self {
        Self
    }

    /// 模拟测量内容尺寸
    pub fn measure_content(content: &str) -> MeasureResult {
        // 这是一个简化的实现，实际应用中应该使用真实的排版引擎
        let char_count = content.chars().count() as f64;
        let line_count = content.lines().count() as f64;

        MeasureResult {
            width: char_count * 6.0,   // 假设每个字符 6pt 宽
            height: line_count * 12.0, // 假设每行 12pt 高
            ascent: 10.0,
            descent: 2.0,
        }
    }

    pub fn to_typst(&self, content: &str) -> String {
        format!("#measure({})", html_escape(content))
    }
}

impl Default for Measure {
    fn default() -> Self {
        Self::new()
    }
}

/// 放置位置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlacePosition {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

/// 放置配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceConfig {
    pub position: PlacePosition,
    pub dx: Option<f64>,
    pub dy: Option<f64>,
}

impl Default for PlaceConfig {
    fn default() -> Self {
        Self {
            position: PlacePosition::Center,
            dx: None,
            dy: None,
        }
    }
}

/// 放置操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Place {
    pub config: PlaceConfig,
}

impl Place {
    pub fn new() -> Self {
        Self {
            config: PlaceConfig::default(),
        }
    }

    pub fn with_position(mut self, position: PlacePosition) -> Self {
        self.config.position = position;
        self
    }

    pub fn with_dx(mut self, dx: f64) -> Self {
        self.config.dx = Some(dx);
        self
    }

    pub fn with_dy(mut self, dy: f64) -> Self {
        self.config.dy = Some(dy);
        self
    }

    pub fn to_typst(&self) -> String {
        let mut parts = vec![format!(
            "position: {}",
            self.position_to_typst(&self.config.position)
        )];

        if let Some(dx) = self.config.dx {
            parts.push(format!("dx: {}pt", dx));
        }

        if let Some(dy) = self.config.dy {
            parts.push(format!("dy: {}pt", dy));
        }

        format!("#place({})", parts.join(", "))
    }

    fn position_to_typst(&self, position: &PlacePosition) -> String {
        match position {
            PlacePosition::TopLeft => "top + left".to_string(),
            PlacePosition::TopCenter => "top".to_string(),
            PlacePosition::TopRight => "top + right".to_string(),
            PlacePosition::CenterLeft => "left".to_string(),
            PlacePosition::Center => "center".to_string(),
            PlacePosition::CenterRight => "right".to_string(),
            PlacePosition::BottomLeft => "bottom + left".to_string(),
            PlacePosition::BottomCenter => "bottom".to_string(),
            PlacePosition::BottomRight => "bottom + right".to_string(),
        }
    }
}

impl Default for Place {
    fn default() -> Self {
        Self::new()
    }
}

/// 分数配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractionConfig {
    pub numerator: f64,
    pub denominator: f64,
}

impl Default for FractionConfig {
    fn default() -> Self {
        Self {
            numerator: 1.0,
            denominator: 1.0,
        }
    }
}

/// 分数操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fraction {
    pub config: FractionConfig,
}

impl Fraction {
    pub fn new() -> Self {
        Self {
            config: FractionConfig::default(),
        }
    }

    pub fn with_numerator(mut self, numerator: f64) -> Self {
        self.config.numerator = numerator;
        self
    }

    pub fn with_denominator(mut self, denominator: f64) -> Self {
        self.config.denominator = denominator;
        self
    }

    pub fn value(&self) -> f64 {
        if self.config.denominator == 0.0 {
            f64::NAN
        } else {
            self.config.numerator / self.config.denominator
        }
    }

    pub fn to_typst(&self) -> String {
        format!("{} / {}", self.config.numerator, self.config.denominator)
    }
}

impl Default for Fraction {
    fn default() -> Self {
        Self::new()
    }
}

/// Angle - 旋转角度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Angle {
    pub degrees: f64,
}

impl Angle {
    pub fn new(degrees: f64) -> Self {
        Self { degrees }
    }

    pub fn from_radians(radians: f64) -> Self {
        Self {
            degrees: radians * 180.0 / std::f64::consts::PI,
        }
    }

    pub fn to_radians(&self) -> f64 {
        self.degrees * std::f64::consts::PI / 180.0
    }

    pub fn to_degrees(&self) -> f64 {
        self.degrees
    }

    pub fn to_typst(&self) -> String {
        format!("{}deg", self.degrees)
    }
}

/// Colbreak - 强制分栏符
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Colbreak;

impl Colbreak {
    pub fn new() -> Self {
        Self
    }

    pub fn to_typst(&self) -> String {
        "#colbreak()".to_string()
    }
}

impl Default for Colbreak {
    fn default() -> Self {
        Self::new()
    }
}

/// Direction - 内容布局方向
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Direction {
    LTR, // Left to Right
    RTL, // Right to Left
    TTB, // Top to Bottom
    BTT, // Bottom to Top
}

impl Direction {
    pub fn to_typst(&self) -> String {
        match self {
            Direction::LTR => "ltr".to_string(),
            Direction::RTL => "rtl".to_string(),
            Direction::TTB => "ttb".to_string(),
            Direction::BTT => "btt".to_string(),
        }
    }
}

/// H - 水平间距插入
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct H {
    pub amount: f64,
    pub weak: bool,
}

impl H {
    pub fn new(amount: f64) -> Self {
        Self {
            amount,
            weak: false,
        }
    }

    pub fn with_weak(mut self, weak: bool) -> Self {
        self.weak = weak;
        self
    }

    pub fn to_typst(&self) -> String {
        if self.weak {
            format!("#h(weak: {}pt)", self.amount)
        } else {
            format!("#h({}pt)", self.amount)
        }
    }
}

/// Hide - 隐藏内容不影响布局
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hide {
    pub content: String,
}

impl Hide {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn to_typst(&self) -> String {
        format!("#hide({})", self.content)
    }
}

/// Layout - 访问当前容器尺寸
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutInfo {
    pub width: f64,
    pub height: f64,
}

impl LayoutInfo {
    pub fn new() -> Self {
        // 这是一个简化的实现，实际应用中应该从排版引擎获取真实尺寸
        Self {
            width: 595.0,  // A4 width in points
            height: 842.0, // A4 height in points
        }
    }

    pub fn to_typst(&self) -> String {
        "#layout()".to_string()
    }
}

impl Default for LayoutInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Length - 长度或距离
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Length {
    pub value: f64,
    pub unit: LengthUnit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LengthUnit {
    Pt,      // Points
    Mm,      // Millimeters
    Cm,      // Centimeters
    In,      // Inches
    Em,      // Em
    Rem,     // Rem
    Percent, // Percentage
}

impl Length {
    pub fn new(value: f64, unit: LengthUnit) -> Self {
        Self { value, unit }
    }

    pub fn to_points(&self) -> f64 {
        match self.unit {
            LengthUnit::Pt => self.value,
            LengthUnit::Mm => self.value * 2.83465,
            LengthUnit::Cm => self.value * 28.3465,
            LengthUnit::In => self.value * 72.0,
            LengthUnit::Em => self.value * 12.0, // Assuming 12pt font
            LengthUnit::Rem => self.value * 16.0, // Assuming 16px base
            LengthUnit::Percent => self.value,   // Keep as is for now
        }
    }

    pub fn to_typst(&self) -> String {
        match self.unit {
            LengthUnit::Pt => format!("{}pt", self.value),
            LengthUnit::Mm => format!("{}mm", self.value),
            LengthUnit::Cm => format!("{}cm", self.value),
            LengthUnit::In => format!("{}in", self.value),
            LengthUnit::Em => format!("{}em", self.value),
            LengthUnit::Rem => format!("{}rem", self.value),
            LengthUnit::Percent => format!("{}%", self.value),
        }
    }
}

/// Move - 移动内容不影响布局
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub dx: f64,
    pub dy: f64,
}

impl Move {
    pub fn new(dx: f64, dy: f64) -> Self {
        Self { dx, dy }
    }

    pub fn to_typst(&self) -> String {
        format!("#move(dx: {}pt, dy: {}pt)", self.dx, self.dy)
    }
}

/// Pad - 内容周围间距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pad {
    pub left: Option<f64>,
    pub right: Option<f64>,
    pub top: Option<f64>,
    pub bottom: Option<f64>,
}

impl Pad {
    pub fn new() -> Self {
        Self {
            left: None,
            right: None,
            top: None,
            bottom: None,
        }
    }

    pub fn with_all(mut self, padding: f64) -> Self {
        self.left = Some(padding);
        self.right = Some(padding);
        self.top = Some(padding);
        self.bottom = Some(padding);
        self
    }

    pub fn with_left(mut self, left: f64) -> Self {
        self.left = Some(left);
        self
    }

    pub fn with_right(mut self, right: f64) -> Self {
        self.right = Some(right);
        self
    }

    pub fn with_top(mut self, top: f64) -> Self {
        self.top = Some(top);
        self
    }

    pub fn with_bottom(mut self, bottom: f64) -> Self {
        self.bottom = Some(bottom);
        self
    }

    pub fn to_typst(&self) -> String {
        let mut parts = Vec::new();

        if let Some(left) = self.left {
            parts.push(format!("left: {}pt", left));
        }

        if let Some(right) = self.right {
            parts.push(format!("right: {}pt", right));
        }

        if let Some(top) = self.top {
            parts.push(format!("top: {}pt", top));
        }

        if let Some(bottom) = self.bottom {
            parts.push(format!("bottom: {}pt", bottom));
        }

        if parts.is_empty() {
            "#pad()".to_string()
        } else {
            format!("#pad({})", parts.join(", "))
        }
    }
}

impl Default for Pad {
    fn default() -> Self {
        Self::new()
    }
}

/// Repeat - 重复内容填充空间
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repeat {
    pub content: String,
    pub count: Option<usize>,
}

impl Repeat {
    pub fn new(content: String) -> Self {
        Self {
            content,
            count: None,
        }
    }

    pub fn with_count(mut self, count: usize) -> Self {
        self.count = Some(count);
        self
    }

    pub fn to_typst(&self) -> String {
        if let Some(count) = self.count {
            format!("#repeat({}, {})", self.content, count)
        } else {
            format!("#repeat({})", self.content)
        }
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
    fn test_align_creation() {
        let align = Align::new();
        assert_eq!(align.config.horizontal, Alignment::Auto);
    }

    #[test]
    fn test_align_with_horizontal() {
        let align = Align::new().with_horizontal(Alignment::Center);
        assert_eq!(align.config.horizontal, Alignment::Center);
    }

    #[test]
    fn test_align_to_typst() {
        let align = Align::new().with_horizontal(Alignment::Center);
        let typst = align.to_typst();
        assert!(typst.contains("horizontal: center"));
    }

    #[test]
    fn test_block_creation() {
        let block = Block::new();
        assert!(block.width.is_none());
    }

    #[test]
    fn test_block_with_width() {
        let block = Block::new().with_width(100.0);
        assert_eq!(block.width, Some(100.0));
    }

    #[test]
    fn test_block_to_typst() {
        let block = Block::new().with_width(100.0).with_height(50.0);
        let typst = block.to_typst();
        assert!(typst.contains("width: 100pt"));
        assert!(typst.contains("height: 50pt"));
    }

    #[test]
    fn test_measure_content() {
        let result = Measure::measure_content("hello");
        assert_eq!(result.width, 30.0); // 5 chars * 6pt
        assert_eq!(result.height, 12.0); // 1 line * 12pt
    }

    #[test]
    fn test_measure_multiline() {
        let result = Measure::measure_content("hello\nworld");
        assert_eq!(result.height, 24.0); // 2 lines * 12pt
    }

    #[test]
    fn test_place_creation() {
        let place = Place::new();
        assert_eq!(place.config.position, PlacePosition::Center);
    }

    #[test]
    fn test_place_with_position() {
        let place = Place::new().with_position(PlacePosition::TopLeft);
        assert_eq!(place.config.position, PlacePosition::TopLeft);
    }

    #[test]
    fn test_place_to_typst() {
        let place = Place::new().with_position(PlacePosition::Center);
        let typst = place.to_typst();
        assert!(typst.contains("position: center"));
    }

    #[test]
    fn test_fraction_creation() {
        let fraction = Fraction::new();
        assert_eq!(fraction.config.numerator, 1.0);
    }

    #[test]
    fn test_fraction_value() {
        let fraction = Fraction::new().with_numerator(1.0).with_denominator(2.0);
        assert_eq!(fraction.value(), 0.5);
    }

    #[test]
    fn test_fraction_division_by_zero() {
        let fraction = Fraction::new().with_numerator(1.0).with_denominator(0.0);
        assert!(fraction.value().is_nan());
    }

    #[test]
    fn test_fraction_to_typst() {
        let fraction = Fraction::new().with_numerator(1.0).with_denominator(2.0);
        let typst = fraction.to_typst();
        assert_eq!(typst, "1 / 2");
    }

    #[test]
    fn test_alignment_variants() {
        assert_eq!(Alignment::Auto, Alignment::Auto);
        assert_eq!(Alignment::Left, Alignment::Left);
        assert_eq!(Alignment::Center, Alignment::Center);
    }

    #[test]
    fn test_place_position_variants() {
        assert_eq!(PlacePosition::Center, PlacePosition::Center);
        assert_eq!(PlacePosition::TopLeft, PlacePosition::TopLeft);
    }

    #[test]
    fn test_angle_creation() {
        let angle = Angle::new(45.0);
        assert_eq!(angle.degrees, 45.0);
    }

    #[test]
    fn test_angle_from_radians() {
        let angle = Angle::from_radians(std::f64::consts::PI);
        assert!((angle.degrees - 180.0).abs() < 0.01);
    }

    #[test]
    fn test_angle_to_radians() {
        let angle = Angle::new(180.0);
        assert!((angle.to_radians() - std::f64::consts::PI).abs() < 0.01);
    }

    #[test]
    fn test_angle_to_typst() {
        let angle = Angle::new(45.0);
        assert_eq!(angle.to_typst(), "45deg");
    }

    #[test]
    fn test_colbreak_to_typst() {
        let colbreak = Colbreak::new();
        assert_eq!(colbreak.to_typst(), "#colbreak()");
    }

    #[test]
    fn test_direction_to_typst() {
        assert_eq!(Direction::LTR.to_typst(), "ltr");
        assert_eq!(Direction::RTL.to_typst(), "rtl");
        assert_eq!(Direction::TTB.to_typst(), "ttb");
        assert_eq!(Direction::BTT.to_typst(), "btt");
    }

    #[test]
    fn test_h_creation() {
        let h = H::new(10.0);
        assert_eq!(h.amount, 10.0);
        assert!(!h.weak);
    }

    #[test]
    fn test_h_with_weak() {
        let h = H::new(10.0).with_weak(true);
        assert!(h.weak);
    }

    #[test]
    fn test_h_to_typst() {
        let h = H::new(10.0);
        assert_eq!(h.to_typst(), "#h(10pt)");
    }

    #[test]
    fn test_h_to_typst_weak() {
        let h = H::new(10.0).with_weak(true);
        assert_eq!(h.to_typst(), "#h(weak: 10pt)");
    }

    #[test]
    fn test_hide_creation() {
        let hide = Hide::new("content".to_string());
        assert_eq!(hide.content, "content");
    }

    #[test]
    fn test_hide_to_typst() {
        let hide = Hide::new("content".to_string());
        assert_eq!(hide.to_typst(), "#hide(content)");
    }

    #[test]
    fn test_layout_get_info() {
        let info = LayoutInfo::new();
        assert_eq!(info.width, 595.0);
        assert_eq!(info.height, 842.0);
    }

    #[test]
    fn test_length_creation() {
        let length = Length::new(10.0, LengthUnit::Pt);
        assert_eq!(length.value, 10.0);
        assert_eq!(length.unit, LengthUnit::Pt);
    }

    #[test]
    fn test_length_to_points() {
        let length = Length::new(10.0, LengthUnit::Mm);
        assert!((length.to_points() - 28.3465).abs() < 0.01);
    }

    #[test]
    fn test_length_to_typst() {
        let length = Length::new(10.0, LengthUnit::Pt);
        assert_eq!(length.to_typst(), "10pt");
    }

    #[test]
    fn test_move_creation() {
        let move_op = Move::new(10.0, 20.0);
        assert_eq!(move_op.dx, 10.0);
        assert_eq!(move_op.dy, 20.0);
    }

    #[test]
    fn test_move_to_typst() {
        let move_op = Move::new(10.0, 20.0);
        assert_eq!(move_op.to_typst(), "#move(dx: 10pt, dy: 20pt)");
    }

    #[test]
    fn test_pad_creation() {
        let pad = Pad::new();
        assert!(pad.left.is_none());
    }

    #[test]
    fn test_pad_with_all() {
        let pad = Pad::new().with_all(10.0);
        assert_eq!(pad.left, Some(10.0));
        assert_eq!(pad.right, Some(10.0));
        assert_eq!(pad.top, Some(10.0));
        assert_eq!(pad.bottom, Some(10.0));
    }

    #[test]
    fn test_pad_to_typst() {
        let pad = Pad::new().with_all(10.0);
        let typst = pad.to_typst();
        assert!(typst.contains("left: 10pt"));
        assert!(typst.contains("right: 10pt"));
    }

    #[test]
    fn test_repeat_creation() {
        let repeat = Repeat::new("content".to_string());
        assert_eq!(repeat.content, "content");
        assert!(repeat.count.is_none());
    }

    #[test]
    fn test_repeat_with_count() {
        let repeat = Repeat::new("content".to_string()).with_count(5);
        assert_eq!(repeat.count, Some(5));
    }

    #[test]
    fn test_repeat_to_typst() {
        let repeat = Repeat::new("content".to_string());
        assert_eq!(repeat.to_typst(), "#repeat(content)");
    }

    #[test]
    fn test_repeat_to_typst_with_count() {
        let repeat = Repeat::new("content".to_string()).with_count(5);
        assert_eq!(repeat.to_typst(), "#repeat(content, 5)");
    }
}
