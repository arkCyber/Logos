/*!
 * 航空航天级符号系统
 * 实现 Typst 的符号和表情符号支持
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SymbolCategory {
    Math,
    Greek,
    Arrow,
    Operator,
    Relation,
    Punctuation,
    Currency,
    Misc,
    Emoji,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub unicode: String,
    pub latex: String,
    pub typst: String,
    pub category: SymbolCategory,
    pub description: Option<String>,
}

pub struct SymbolRegistry {
    symbols: HashMap<String, Symbol>,
    emoji_map: HashMap<String, String>,
}

impl SymbolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            symbols: HashMap::new(),
            emoji_map: HashMap::new(),
        };

        registry.initialize_math_symbols();
        registry.initialize_greek_symbols();
        registry.initialize_arrow_symbols();
        registry.initialize_operator_symbols();
        registry.initialize_relation_symbols();
        registry.initialize_currency_symbols();
        registry.initialize_emoji_map();

        registry
    }

    fn initialize_math_symbols(&mut self) {
        // 数学符号
        let math_symbols = vec![
            ("infinity", "∞", "\\infty", "infinity", "Infinity symbol"),
            ("partial", "∂", "\\partial", "partial", "Partial derivative"),
            ("nabla", "∇", "\\nabla", "nabla", "Nabla operator"),
            ("sum", "∑", "\\sum", "sum", "Summation"),
            ("product", "∏", "\\prod", "product", "Product"),
            ("integral", "∫", "\\int", "integral", "Integral"),
            ("sqrt", "√", "\\sqrt", "sqrt", "Square root"),
            ("plus-minus", "±", "\\pm", "plus-minus", "Plus-minus sign"),
            ("minus-plus", "∓", "\\mp", "minus-plus", "Minus-plus sign"),
            ("times", "×", "\\times", "times", "Multiplication sign"),
            ("divide", "÷", "\\div", "divide", "Division sign"),
            ("approx", "≈", "\\approx", "approx", "Approximately equal"),
            ("not-equal", "≠", "\\neq", "not-equal", "Not equal"),
            (
                "less-equal",
                "≤",
                "\\leq",
                "less-equal",
                "Less than or equal",
            ),
            (
                "greater-equal",
                "≥",
                "\\geq",
                "greater-equal",
                "Greater than or equal",
            ),
            ("degree", "°", "^\\circ", "degree", "Degree symbol"),
            ("pi", "π", "\\pi", "pi", "Pi symbol"),
            ("alpha", "α", "\\alpha", "alpha", "Alpha symbol"),
            ("beta", "β", "\\beta", "beta", "Beta symbol"),
            ("gamma", "γ", "\\gamma", "gamma", "Gamma symbol"),
            ("delta", "δ", "\\delta", "delta", "Delta symbol"),
            ("epsilon", "ε", "\\epsilon", "epsilon", "Epsilon symbol"),
            ("theta", "θ", "\\theta", "theta", "Theta symbol"),
            ("lambda", "λ", "\\lambda", "lambda", "Lambda symbol"),
            ("mu", "μ", "\\mu", "mu", "Mu symbol"),
            ("sigma", "σ", "\\sigma", "sigma", "Sigma symbol"),
            ("phi", "φ", "\\phi", "phi", "Phi symbol"),
            ("omega", "ω", "\\omega", "omega", "Omega symbol"),
        ];

        for (name, unicode, latex, typst, description) in math_symbols {
            self.symbols.insert(
                name.to_string(),
                Symbol {
                    name: name.to_string(),
                    unicode: unicode.to_string(),
                    latex: latex.to_string(),
                    typst: typst.to_string(),
                    category: SymbolCategory::Math,
                    description: Some(description.to_string()),
                },
            );
        }
    }

    fn initialize_greek_symbols(&mut self) {
        // 希腊字母（大写）
        let greek_upper = vec![
            ("Gamma", "Γ", "\\Gamma", "Gamma", "Capital Gamma"),
            ("Delta", "Δ", "\\Delta", "Delta", "Capital Delta"),
            ("Theta", "Θ", "\\Theta", "Theta", "Capital Theta"),
            ("Lambda", "Λ", "\\Lambda", "Lambda", "Capital Lambda"),
            ("Xi", "Ξ", "\\Xi", "Xi", "Capital Xi"),
            ("Pi", "Π", "\\Pi", "Pi", "Capital Pi"),
            ("Sigma", "Σ", "\\Sigma", "Sigma", "Capital Sigma"),
            ("Phi", "Φ", "\\Phi", "Phi", "Capital Phi"),
            ("Psi", "Ψ", "\\Psi", "Psi", "Capital Psi"),
            ("Omega", "Ω", "\\Omega", "Omega", "Capital Omega"),
        ];

        for (name, unicode, latex, typst, description) in greek_upper {
            self.symbols.insert(
                name.to_string(),
                Symbol {
                    name: name.to_string(),
                    unicode: unicode.to_string(),
                    latex: latex.to_string(),
                    typst: typst.to_string(),
                    category: SymbolCategory::Greek,
                    description: Some(description.to_string()),
                },
            );
        }
    }

    fn initialize_arrow_symbols(&mut self) {
        // 箭头符号
        let arrows = vec![
            ("arrow-right", "→", "\\rightarrow", "arrow", "Right arrow"),
            ("arrow-left", "←", "\\leftarrow", "arrow.l", "Left arrow"),
            ("arrow-up", "↑", "\\uparrow", "arrow.u", "Up arrow"),
            ("arrow-down", "↓", "\\downarrow", "arrow.d", "Down arrow"),
            (
                "arrow-double-right",
                "⇒",
                "\\Rightarrow",
                "arrow.r.double",
                "Double right arrow",
            ),
            (
                "arrow-double-left",
                "⇐",
                "\\Leftarrow",
                "arrow.l.double",
                "Double left arrow",
            ),
            (
                "arrow-up-down",
                "↕",
                "\\updownarrow",
                "arrow.ud",
                "Up-down arrow",
            ),
            (
                "arrow-left-right",
                "↔",
                "\\leftrightarrow",
                "arrow.lr",
                "Left-right arrow",
            ),
        ];

        for (name, unicode, latex, typst, description) in arrows {
            self.symbols.insert(
                name.to_string(),
                Symbol {
                    name: name.to_string(),
                    unicode: unicode.to_string(),
                    latex: latex.to_string(),
                    typst: typst.to_string(),
                    category: SymbolCategory::Arrow,
                    description: Some(description.to_string()),
                },
            );
        }
    }

    fn initialize_operator_symbols(&mut self) {
        // 运算符符号
        let operators = vec![
            ("union", "∪", "\\cup", "union", "Union"),
            ("intersection", "∩", "\\cap", "intersection", "Intersection"),
            ("subset", "⊂", "\\subset", "subset", "Subset"),
            ("superset", "⊃", "\\supset", "superset", "Superset"),
            ("element", "∈", "\\in", "in", "Element of"),
            ("not-element", "∉", "\\notin", "not-in", "Not element of"),
            ("empty-set", "∅", "\\emptyset", "empty-set", "Empty set"),
            ("forall", "∀", "\\forall", "forall", "For all"),
            ("exists", "∃", "\\exists", "exists", "There exists"),
            ("therefore", "∴", "\\therefore", "therefore", "Therefore"),
            ("because", "∵", "\\because", "because", "Because"),
        ];

        for (name, unicode, latex, typst, description) in operators {
            self.symbols.insert(
                name.to_string(),
                Symbol {
                    name: name.to_string(),
                    unicode: unicode.to_string(),
                    latex: latex.to_string(),
                    typst: typst.to_string(),
                    category: SymbolCategory::Operator,
                    description: Some(description.to_string()),
                },
            );
        }
    }

    fn initialize_relation_symbols(&mut self) {
        // 关系符号
        let relations = vec![
            ("equivalent", "≡", "\\equiv", "equiv", "Equivalent to"),
            ("congruent", "≅", "\\cong", "congruent", "Congruent to"),
            ("similar", "∼", "\\sim", "similar", "Similar to"),
            ("parallel", "∥", "\\parallel", "parallel", "Parallel to"),
            (
                "perpendicular",
                "⊥",
                "\\perp",
                "perpendicular",
                "Perpendicular to",
            ),
            ("angle", "∠", "\\angle", "angle", "Angle"),
        ];

        for (name, unicode, latex, typst, description) in relations {
            self.symbols.insert(
                name.to_string(),
                Symbol {
                    name: name.to_string(),
                    unicode: unicode.to_string(),
                    latex: latex.to_string(),
                    typst: typst.to_string(),
                    category: SymbolCategory::Relation,
                    description: Some(description.to_string()),
                },
            );
        }
    }

    fn initialize_currency_symbols(&mut self) {
        // 货币符号
        let currencies = vec![
            ("dollar", "$", "\\$", "dollar", "Dollar sign"),
            ("euro", "€", "\\euro", "euro", "Euro sign"),
            ("pound", "£", "\\pound", "pound", "Pound sign"),
            ("yen", "¥", "\\yen", "yen", "Yen sign"),
            ("cent", "¢", "\\cent", "cent", "Cent sign"),
        ];

        for (name, unicode, latex, typst, description) in currencies {
            self.symbols.insert(
                name.to_string(),
                Symbol {
                    name: name.to_string(),
                    unicode: unicode.to_string(),
                    latex: latex.to_string(),
                    typst: typst.to_string(),
                    category: SymbolCategory::Currency,
                    description: Some(description.to_string()),
                },
            );
        }
    }

    fn initialize_emoji_map(&mut self) {
        // 表情符号映射
        let emojis = vec![
            ("smile", "😊"),
            ("laugh", "😂"),
            ("heart", "❤️"),
            ("thumbs-up", "👍"),
            ("thumbs-down", "👎"),
            ("star", "⭐"),
            ("fire", "🔥"),
            ("check", "✅"),
            ("cross", "❌"),
            ("warning", "⚠️"),
            ("info", "ℹ️"),
            ("question", "❓"),
            ("exclamation", "❗"),
            ("rocket", "🚀"),
            ("bulb", "💡"),
            ("gear", "⚙️"),
            ("document", "📄"),
            ("folder", "📁"),
            ("link", "🔗"),
            ("search", "🔍"),
        ];

        for (name, unicode) in emojis {
            self.emoji_map.insert(name.to_string(), unicode.to_string());
            self.symbols.insert(
                name.to_string(),
                Symbol {
                    name: name.to_string(),
                    unicode: unicode.to_string(),
                    latex: name.to_string(),
                    typst: name.to_string(),
                    category: SymbolCategory::Emoji,
                    description: None,
                },
            );
        }
    }

    /// 通过名称获取符号
    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    /// 通过 Unicode 获取符号
    pub fn find_by_unicode(&self, unicode: &str) -> Option<&Symbol> {
        self.symbols.values().find(|s| s.unicode == unicode)
    }

    /// 按类别获取符号
    pub fn get_by_category(&self, category: SymbolCategory) -> Vec<&Symbol> {
        self.symbols
            .values()
            .filter(|s| s.category == category)
            .collect()
    }

    /// 搜索符号
    pub fn search(&self, query: &str) -> Vec<&Symbol> {
        let query_lower = query.to_lowercase();
        self.symbols
            .values()
            .filter(|s| {
                s.name.to_lowercase().contains(&query_lower)
                    || s.unicode.contains(query)
                    || s.description
                        .as_ref()
                        .is_none_or(|d| d.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// 获取表情符号
    pub fn get_emoji(&self, name: &str) -> Option<&str> {
        self.emoji_map.get(name).map(|s| s.as_str())
    }

    /// 将文本中的符号名称替换为 Unicode
    pub fn replace_symbols(&self, text: &str) -> String {
        let mut result = text.to_string();

        // 按名称长度降序排序，避免部分匹配
        let mut names: Vec<_> = self.symbols.keys().collect();
        names.sort_by_key(|b| std::cmp::Reverse(b.len()));

        for name in names {
            let symbol = self.symbols.get(name).unwrap();
            result = result.replace(&format!(":{}:", name), &symbol.unicode);
        }

        result
    }

    /// 将 LaTeX 符号转换为 Typst
    pub fn latex_to_typst(&self, latex: &str) -> String {
        let mut result = latex.to_string();

        for symbol in self.symbols.values() {
            result = result.replace(
                &format!("\\{}", &symbol.latex.trim_start_matches('\\')),
                &symbol.typst,
            );
        }

        result
    }
}

impl Default for SymbolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_registry_creation() {
        let registry = SymbolRegistry::new();
        assert!(!registry.symbols.is_empty());
    }

    #[test]
    fn test_get_symbol() {
        let registry = SymbolRegistry::new();
        let symbol = registry.get_symbol("infinity");
        assert!(symbol.is_some());
        assert_eq!(symbol.unwrap().unicode, "∞");
    }

    #[test]
    fn test_get_emoji() {
        let registry = SymbolRegistry::new();
        let emoji = registry.get_emoji("smile");
        assert!(emoji.is_some());
        assert_eq!(emoji.unwrap(), "😊");
    }

    #[test]
    fn test_get_by_category() {
        let registry = SymbolRegistry::new();
        let math_symbols = registry.get_by_category(SymbolCategory::Math);
        assert!(!math_symbols.is_empty());
    }

    #[test]
    fn test_search_symbols() {
        let registry = SymbolRegistry::new();
        let results = registry.search("arrow");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_replace_symbols() {
        let registry = SymbolRegistry::new();
        let text = "The value is :infinity:";
        let result = registry.replace_symbols(text);
        assert!(result.contains("∞"));
    }

    #[test]
    fn test_latex_to_typst() {
        let registry = SymbolRegistry::new();
        let latex = r"The sum is $\sum$";
        let result = registry.latex_to_typst(latex);
        assert!(result.contains("sum"));
    }

    #[test]
    fn test_find_by_unicode() {
        let registry = SymbolRegistry::new();
        let symbol = registry.find_by_unicode("∞");
        assert!(symbol.is_some());
        assert_eq!(symbol.unwrap().name, "infinity");
    }

    #[test]
    fn test_currency_symbols() {
        let registry = SymbolRegistry::new();
        let dollar = registry.get_symbol("dollar");
        assert!(dollar.is_some());
        assert_eq!(dollar.unwrap().unicode, "$");
    }

    #[test]
    fn test_greek_symbols() {
        let registry = SymbolRegistry::new();
        let alpha = registry.get_symbol("alpha");
        assert!(alpha.is_some());
        assert_eq!(alpha.unwrap().unicode, "α");
    }
}
