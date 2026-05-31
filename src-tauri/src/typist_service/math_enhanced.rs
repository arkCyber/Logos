/*!
 * 航空航天级 Math 增强模块
 * 实现 Typst 的 Math 增强功能（frac、accent、attach、roots、op）
 */

use serde::{Deserialize, Serialize};

/// 分数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathFrac {
    pub numerator: String,
    pub denominator: String,
}

impl MathFrac {
    pub fn new() -> Self {
        Self {
            numerator: "1".to_string(),
            denominator: "1".to_string(),
        }
    }

    pub fn with_numerator(mut self, numerator: String) -> Self {
        self.numerator = numerator;
        self
    }

    pub fn with_denominator(mut self, denominator: String) -> Self {
        self.denominator = denominator;
        self
    }

    pub fn to_typst(&self) -> String {
        format!("$frac({}, {})$", self.numerator, self.denominator)
    }
}

impl Default for MathFrac {
    fn default() -> Self {
        Self::new()
    }
}

/// 重音符号类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccentType {
    Acute,
    Grave,
    Hat,
    Tilde,
    Bar,
    Dot,
    Ddot,
    Arrow,
}

/// 重音符号
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathAccent {
    pub accent: AccentType,
    pub base: String,
}

impl MathAccent {
    pub fn new() -> Self {
        Self {
            accent: AccentType::Hat,
            base: "x".to_string(),
        }
    }

    pub fn with_accent(mut self, accent: AccentType) -> Self {
        self.accent = accent;
        self
    }

    pub fn with_base(mut self, base: String) -> Self {
        self.base = base;
        self
    }

    pub fn to_typst(&self) -> String {
        let accent_sym = match self.accent {
            AccentType::Acute => "acute",
            AccentType::Grave => "grave",
            AccentType::Hat => "hat",
            AccentType::Tilde => "tilde",
            AccentType::Bar => "bar",
            AccentType::Dot => "dot",
            AccentType::Ddot => "ddot",
            AccentType::Arrow => "arrow",
        };
        format!("${}({})$", accent_sym, self.base)
    }
}

impl Default for MathAccent {
    fn default() -> Self {
        Self::new()
    }
}

/// 附件类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AttachType {
    Subscript,
    Superscript,
    Limits,
}

/// 附件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathAttach {
    pub attach_type: AttachType,
    pub base: String,
    pub sub: Option<String>,
    pub sup: Option<String>,
}

impl MathAttach {
    pub fn new() -> Self {
        Self {
            attach_type: AttachType::Subscript,
            base: "x".to_string(),
            sub: None,
            sup: None,
        }
    }

    pub fn with_type(mut self, attach_type: AttachType) -> Self {
        self.attach_type = attach_type;
        self
    }

    pub fn with_base(mut self, base: String) -> Self {
        self.base = base;
        self
    }

    pub fn with_sub(mut self, sub: String) -> Self {
        self.sub = Some(sub);
        self
    }

    pub fn with_sup(mut self, sup: String) -> Self {
        self.sup = Some(sup);
        self
    }

    pub fn to_typst(&self) -> String {
        match self.attach_type {
            AttachType::Subscript => {
                if let Some(sub) = &self.sub {
                    format!("${}_{} $", self.base, sub)
                } else {
                    format!("${}$", self.base)
                }
            }
            AttachType::Superscript => {
                if let Some(sup) = &self.sup {
                    format!("${}^{{{}}}$", self.base, sup)
                } else {
                    format!("${}$", self.base)
                }
            }
            AttachType::Limits => {
                let sub = self.sub.as_deref().unwrap_or("");
                let sup = self.sup.as_deref().unwrap_or("");
                format!("${}_{}^{{{}}}$", self.base, sub, sup)
            }
        }
    }
}

impl Default for MathAttach {
    fn default() -> Self {
        Self::new()
    }
}

/// 根号类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RootType {
    Square,
    Cube,
    Nth(u32),
}

/// 根号
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathRoot {
    pub root_type: RootType,
    pub radicand: String,
}

impl MathRoot {
    pub fn new() -> Self {
        Self {
            root_type: RootType::Square,
            radicand: "x".to_string(),
        }
    }

    pub fn with_type(mut self, root_type: RootType) -> Self {
        self.root_type = root_type;
        self
    }

    pub fn with_radicand(mut self, radicand: String) -> Self {
        self.radicand = radicand;
        self
    }

    pub fn to_typst(&self) -> String {
        match self.root_type {
            RootType::Square => format!("$sqrt({})$", self.radicand),
            RootType::Cube => format!("$root(3, {})$", self.radicand),
            RootType::Nth(n) => format!("$root({}, {})$", n, self.radicand),
        }
    }
}

impl Default for MathRoot {
    fn default() -> Self {
        Self::new()
    }
}

/// 运算符
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathOp {
    pub symbol: String,
    pub name: String,
}

impl MathOp {
    pub fn new() -> Self {
        Self {
            symbol: "+".to_string(),
            name: "plus".to_string(),
        }
    }

    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = symbol;
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn to_typst(&self) -> String {
        format!("$op(\"{}\")$", self.symbol)
    }
}

impl Default for MathOp {
    fn default() -> Self {
        Self::new()
    }
}

/// Lr - 分隔符匹配
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathLr {
    pub left: String,
    pub right: String,
    pub body: String,
}

impl MathLr {
    pub fn new(left: String, right: String, body: String) -> Self {
        Self { left, right, body }
    }

    pub fn to_typst(&self) -> String {
        format!("$lr({} {} {})$", self.left, self.body, self.right)
    }
}

/// Sizes - 强制表达式大小样式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MathSize {
    Tiny,
    Script,
    Footnote,
    Small,
    Normal,
    Large,
    Huge,
    Display,
}

impl MathSize {
    pub fn to_typst(&self) -> String {
        match self {
            MathSize::Tiny => "tiny".to_string(),
            MathSize::Script => "script".to_string(),
            MathSize::Footnote => "footnote".to_string(),
            MathSize::Small => "small".to_string(),
            MathSize::Normal => "normal".to_string(),
            MathSize::Large => "large".to_string(),
            MathSize::Huge => "huge".to_string(),
            MathSize::Display => "display".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathSizes {
    pub size: MathSize,
    pub content: String,
}

impl MathSizes {
    pub fn new(size: MathSize, content: String) -> Self {
        Self { size, content }
    }

    pub fn to_typst(&self) -> String {
        format!("${}({})$", self.size.to_typst(), self.content)
    }
}

/// Stretch - 拉伸字形
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathStretch {
    pub glyph: String,
    pub height: Option<f64>,
}

impl MathStretch {
    pub fn new(glyph: String) -> Self {
        Self {
            glyph,
            height: None,
        }
    }

    pub fn with_height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    pub fn to_typst(&self) -> String {
        if let Some(height) = self.height {
            format!("$stretch({}, height: {}pt)$", self.glyph, height)
        } else {
            format!("$stretch({})$", self.glyph)
        }
    }
}

/// Styles - 公式中的替代字母形式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MathStyle {
    Normal,
    Italic,
    Bold,
    BoldItalic,
    Calligraphic,
    Fraktur,
    Sans,
    Monospace,
}

impl MathStyle {
    pub fn to_typst(&self) -> String {
        match self {
            MathStyle::Normal => "normal".to_string(),
            MathStyle::Italic => "italic".to_string(),
            MathStyle::Bold => "bold".to_string(),
            MathStyle::BoldItalic => "bold-italic".to_string(),
            MathStyle::Calligraphic => "cal".to_string(),
            MathStyle::Fraktur => "frak".to_string(),
            MathStyle::Sans => "sans".to_string(),
            MathStyle::Monospace => "mono".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathStyles {
    pub style: MathStyle,
    pub content: String,
}

impl MathStyles {
    pub fn new(style: MathStyle, content: String) -> Self {
        Self { style, content }
    }

    pub fn to_typst(&self) -> String {
        format!("${}({})$", self.style.to_typst(), self.content)
    }
}

/// Underover - 分隔符在方程部分上方或下方
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathUnderover {
    pub under: Option<String>,
    pub over: Option<String>,
    pub body: String,
}

impl MathUnderover {
    pub fn new(body: String) -> Self {
        Self {
            under: None,
            over: None,
            body,
        }
    }

    pub fn with_under(mut self, under: String) -> Self {
        self.under = Some(under);
        self
    }

    pub fn with_over(mut self, over: String) -> Self {
        self.over = Some(over);
        self
    }

    pub fn to_typst(&self) -> String {
        let mut parts = Vec::new();

        if let Some(under) = &self.under {
            parts.push(format!("under: {}", under));
        }

        if let Some(over) = &self.over {
            parts.push(format!("over: {}", over));
        }

        if parts.is_empty() {
            format!("$underover({})$", self.body)
        } else {
            format!("$underover({}, {})$", self.body, parts.join(", "))
        }
    }
}

/// Variants - 公式中的替代字体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MathVariant {
    Serif,
    Sans,
    Mono,
    Caligraphic,
    Fraktur,
}

impl MathVariant {
    pub fn to_typst(&self) -> String {
        match self {
            MathVariant::Serif => "serif".to_string(),
            MathVariant::Sans => "sans".to_string(),
            MathVariant::Mono => "mono".to_string(),
            MathVariant::Caligraphic => "cal".to_string(),
            MathVariant::Fraktur => "frak".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathVariants {
    pub variant: MathVariant,
    pub content: String,
}

impl MathVariants {
    pub fn new(variant: MathVariant, content: String) -> Self {
        Self { variant, content }
    }

    pub fn to_typst(&self) -> String {
        format!("${}({})$", self.variant.to_typst(), self.content)
    }
}

/// Class - 强制使用特定数学类
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MathClass {
    Normal,
    Operator,
    Binary,
    Relation,
    Punctuation,
    Opening,
    Closing,
}

impl MathClass {
    pub fn to_typst(&self) -> String {
        match self {
            MathClass::Normal => "normal".to_string(),
            MathClass::Operator => "operator".to_string(),
            MathClass::Binary => "binary".to_string(),
            MathClass::Relation => "relation".to_string(),
            MathClass::Punctuation => "punctuation".to_string(),
            MathClass::Opening => "opening".to_string(),
            MathClass::Closing => "closing".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathClassOp {
    pub class: MathClass,
    pub content: String,
}

impl MathClassOp {
    pub fn new(class: MathClass, content: String) -> Self {
        Self { class, content }
    }

    pub fn to_typst(&self) -> String {
        format!("$class({}, {})$", self.class.to_typst(), self.content)
    }
}

/// Vec - 列向量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathVec {
    pub elements: Vec<String>,
}

impl MathVec {
    pub fn new(elements: Vec<String>) -> Self {
        Self { elements }
    }

    pub fn to_typst(&self) -> String {
        let elements = self.elements.join(", ");
        format!("$vec(({}))$", elements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_frac_creation() {
        let frac = MathFrac::new();
        assert_eq!(frac.numerator, "1");
    }

    #[test]
    fn test_math_frac_to_typst() {
        let frac = MathFrac::new()
            .with_numerator("a".to_string())
            .with_denominator("b".to_string());
        let typst = frac.to_typst();
        assert!(typst.contains("frac(a, b)"));
    }

    #[test]
    fn test_math_accent_creation() {
        let accent = MathAccent::new();
        assert_eq!(accent.base, "x");
    }

    #[test]
    fn test_math_accent_to_typst() {
        let accent = MathAccent::new()
            .with_accent(AccentType::Hat)
            .with_base("x".to_string());
        let typst = accent.to_typst();
        assert!(typst.contains("hat(x)"));
    }

    #[test]
    fn test_math_attach_creation() {
        let attach = MathAttach::new();
        assert_eq!(attach.base, "x");
    }

    #[test]
    fn test_math_attach_subscript() {
        let attach = MathAttach::new()
            .with_type(AttachType::Subscript)
            .with_sub("i".to_string());
        let typst = attach.to_typst();
        assert!(typst.contains("_i"));
    }

    #[test]
    fn test_math_attach_superscript() {
        let attach = MathAttach::new()
            .with_type(AttachType::Superscript)
            .with_sup("2".to_string());
        let typst = attach.to_typst();
        assert!(typst.contains("^{2}"));
    }

    #[test]
    fn test_math_attach_limits() {
        let attach = MathAttach::new()
            .with_type(AttachType::Limits)
            .with_sub("i=0".to_string())
            .with_sup("n".to_string());
        let typst = attach.to_typst();
        assert!(typst.contains("_i=0") && typst.contains("^{n}"));
    }

    #[test]
    fn test_math_root_creation() {
        let root = MathRoot::new();
        assert_eq!(root.radicand, "x");
    }

    #[test]
    fn test_math_root_square() {
        let root = MathRoot::new()
            .with_type(RootType::Square)
            .with_radicand("x".to_string());
        let typst = root.to_typst();
        assert!(typst.contains("sqrt(x)"));
    }

    #[test]
    fn test_math_root_cube() {
        let root = MathRoot::new()
            .with_type(RootType::Cube)
            .with_radicand("x".to_string());
        let typst = root.to_typst();
        assert!(typst.contains("root(3, x)"));
    }

    #[test]
    fn test_math_root_nth() {
        let root = MathRoot::new()
            .with_type(RootType::Nth(5))
            .with_radicand("x".to_string());
        let typst = root.to_typst();
        assert!(typst.contains("root(5, x)"));
    }

    #[test]
    fn test_math_op_creation() {
        let op = MathOp::new();
        assert_eq!(op.symbol, "+");
    }

    #[test]
    fn test_math_op_to_typst() {
        let op = MathOp::new().with_symbol("∫".to_string());
        let typst = op.to_typst();
        assert!(typst.contains("op"));
    }

    #[test]
    fn test_accent_type_variants() {
        assert_eq!(AccentType::Hat, AccentType::Hat);
        assert_eq!(AccentType::Bar, AccentType::Bar);
    }

    #[test]
    fn test_attach_type_variants() {
        assert_eq!(AttachType::Subscript, AttachType::Subscript);
        assert_eq!(AttachType::Superscript, AttachType::Superscript);
    }

    #[test]
    fn test_root_type_variants() {
        assert_eq!(RootType::Square, RootType::Square);
        assert_eq!(RootType::Cube, RootType::Cube);
        assert_eq!(RootType::Nth(3), RootType::Nth(3));
    }

    #[test]
    fn test_math_lr_creation() {
        let lr = MathLr::new("(".to_string(), ")".to_string(), "x".to_string());
        assert_eq!(lr.left, "(");
        assert_eq!(lr.right, ")");
    }

    #[test]
    fn test_math_lr_to_typst() {
        let lr = MathLr::new("(".to_string(), ")".to_string(), "x".to_string());
        assert!(lr.to_typst().contains("lr"));
    }

    #[test]
    fn test_math_size_to_typst() {
        assert_eq!(MathSize::Tiny.to_typst(), "tiny");
        assert_eq!(MathSize::Normal.to_typst(), "normal");
        assert_eq!(MathSize::Display.to_typst(), "display");
    }

    #[test]
    fn test_math_sizes_creation() {
        let sizes = MathSizes::new(MathSize::Large, "x".to_string());
        assert_eq!(sizes.size, MathSize::Large);
    }

    #[test]
    fn test_math_sizes_to_typst() {
        let sizes = MathSizes::new(MathSize::Large, "x".to_string());
        assert!(sizes.to_typst().contains("large"));
    }

    #[test]
    fn test_math_stretch_creation() {
        let stretch = MathStretch::new("(".to_string());
        assert_eq!(stretch.glyph, "(");
    }

    #[test]
    fn test_math_stretch_with_height() {
        let stretch = MathStretch::new("(".to_string()).with_height(10.0);
        assert_eq!(stretch.height, Some(10.0));
    }

    #[test]
    fn test_math_stretch_to_typst() {
        let stretch = MathStretch::new("(".to_string());
        assert!(stretch.to_typst().contains("stretch"));
    }

    #[test]
    fn test_math_style_to_typst() {
        assert_eq!(MathStyle::Normal.to_typst(), "normal");
        assert_eq!(MathStyle::Italic.to_typst(), "italic");
        assert_eq!(MathStyle::Bold.to_typst(), "bold");
    }

    #[test]
    fn test_math_styles_creation() {
        let styles = MathStyles::new(MathStyle::Bold, "x".to_string());
        assert_eq!(styles.style, MathStyle::Bold);
    }

    #[test]
    fn test_math_styles_to_typst() {
        let styles = MathStyles::new(MathStyle::Bold, "x".to_string());
        assert!(styles.to_typst().contains("bold"));
    }

    #[test]
    fn test_math_underover_creation() {
        let underover = MathUnderover::new("x".to_string());
        assert_eq!(underover.body, "x");
    }

    #[test]
    fn test_math_underover_with_under() {
        let underover = MathUnderover::new("x".to_string()).with_under("lim".to_string());
        assert_eq!(underover.under, Some("lim".to_string()));
    }

    #[test]
    fn test_math_underover_to_typst() {
        let underover = MathUnderover::new("x".to_string());
        assert!(underover.to_typst().contains("underover"));
    }

    #[test]
    fn test_math_variant_to_typst() {
        assert_eq!(MathVariant::Serif.to_typst(), "serif");
        assert_eq!(MathVariant::Sans.to_typst(), "sans");
        assert_eq!(MathVariant::Mono.to_typst(), "mono");
    }

    #[test]
    fn test_math_variants_creation() {
        let variants = MathVariants::new(MathVariant::Caligraphic, "x".to_string());
        assert_eq!(variants.variant, MathVariant::Caligraphic);
    }

    #[test]
    fn test_math_variants_to_typst() {
        let variants = MathVariants::new(MathVariant::Caligraphic, "x".to_string());
        assert!(variants.to_typst().contains("cal"));
    }

    #[test]
    fn test_math_class_to_typst() {
        assert_eq!(MathClass::Normal.to_typst(), "normal");
        assert_eq!(MathClass::Operator.to_typst(), "operator");
        assert_eq!(MathClass::Binary.to_typst(), "binary");
    }

    #[test]
    fn test_math_class_op_creation() {
        let class_op = MathClassOp::new(MathClass::Operator, "+".to_string());
        assert_eq!(class_op.class, MathClass::Operator);
    }

    #[test]
    fn test_math_class_op_to_typst() {
        let class_op = MathClassOp::new(MathClass::Operator, "+".to_string());
        assert!(class_op.to_typst().contains("operator"));
    }

    #[test]
    fn test_math_vec_creation() {
        let vec = MathVec::new(vec!["1".to_string(), "2".to_string(), "3".to_string()]);
        assert_eq!(vec.elements.len(), 3);
    }

    #[test]
    fn test_math_vec_to_typst() {
        let vec = MathVec::new(vec!["1".to_string(), "2".to_string(), "3".to_string()]);
        assert!(vec.to_typst().contains("vec"));
    }
}
