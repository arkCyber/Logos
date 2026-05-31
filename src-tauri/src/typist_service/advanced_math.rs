/*!
 * 航空航天级高级数学系统
 * 实现 Typst 的高级数学功能（Binomial、Primes、Cancel、Cases）
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Binomial {
    pub n: i64,
    pub k: i64,
}

impl Binomial {
    pub fn new(n: i64, k: i64) -> Self {
        Self { n, k }
    }

    /// 计算二项式系数 C(n,k) = n! / (k! * (n-k)!)
    pub fn calculate(&self) -> i64 {
        if self.k < 0 || self.k > self.n {
            return 0;
        }

        // 优化：使用较小的 k 值
        let k = if self.k > self.n - self.k {
            self.n - self.k
        } else {
            self.k
        };

        let mut result = 1i64;
        for i in 0..k {
            result = result * (self.n - i) / (i + 1);
        }

        result
    }

    /// 生成 LaTeX 格式
    pub fn to_latex(&self) -> String {
        format!("\\binom{{{}}}{{{}}}", self.n, self.k)
    }

    /// 生成 Typst 格式
    pub fn to_typst(&self) -> String {
        format!("binomial({}, {})", self.n, self.k)
    }

    /// 生成 HTML 格式
    pub fn to_html(&self) -> String {
        format!(
            "<span class=\"binomial\"><span class=\"binomial-top\">{}</span><span class=\"binomial-bottom\">{}</span></span>",
            self.n, self.k
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Primes {
    pub limit: i64,
}

impl Primes {
    pub fn new(limit: i64) -> Self {
        Self { limit }
    }

    /// 生成质数列表
    pub fn generate(&self) -> Vec<i64> {
        if self.limit < 2 {
            return Vec::new();
        }

        let mut sieve = vec![true; (self.limit + 1) as usize];
        sieve[0] = false;
        sieve[1] = false;

        let mut i = 2;
        while i * i <= self.limit {
            if sieve[i as usize] {
                let mut j = i * i;
                while j <= self.limit {
                    sieve[j as usize] = false;
                    j += i;
                }
            }
            i += 1;
        }

        (2..=self.limit).filter(|&x| sieve[x as usize]).collect()
    }

    /// 检查是否为质数
    pub fn is_prime(&self, n: i64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }

        let sqrt_n = (n as f64).sqrt() as i64;
        for i in (3..=sqrt_n).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }

        true
    }

    /// 获取第 n 个质数
    pub fn nth_prime(&self, n: usize) -> Option<i64> {
        let primes = self.generate();
        primes.get(n).copied()
    }

    /// 质因数分解
    pub fn prime_factors(&self, n: i64) -> Vec<(i64, i64)> {
        if n < 2 {
            return Vec::new();
        }

        let mut factors = Vec::new();
        let mut num = n;
        let mut divisor = 2;

        while divisor * divisor <= num {
            let mut count = 0;
            while num % divisor == 0 {
                count += 1;
                num /= divisor;
            }
            if count > 0 {
                factors.push((divisor, count));
            }
            divisor += 1;
        }

        if num > 1 {
            factors.push((num, 1));
        }

        factors
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cancel {
    pub numerator: String,
    pub denominator: String,
    pub cancelled: bool,
}

impl Cancel {
    pub fn new(numerator: String, denominator: String) -> Self {
        Self {
            numerator,
            denominator,
            cancelled: false,
        }
    }

    pub fn with_cancelled(mut self, cancelled: bool) -> Self {
        self.cancelled = cancelled;
        self
    }

    /// 生成 LaTeX 格式
    pub fn to_latex(&self) -> String {
        if self.cancelled {
            format!(
                "\\cancel{{{}}} / \\cancel{{{}}}",
                self.numerator, self.denominator
            )
        } else {
            format!("{} / {}", self.numerator, self.denominator)
        }
    }

    /// 生成 Typst 格式
    pub fn to_typst(&self) -> String {
        if self.cancelled {
            format!("cancel({}) / cancel({})", self.numerator, self.denominator)
        } else {
            format!("{} / {}", self.numerator, self.denominator)
        }
    }

    /// 生成 HTML 格式
    pub fn to_html(&self) -> String {
        let cancel_class = if self.cancelled { "cancel" } else { "" };
        format!(
            "<span class=\"fraction\"><span class=\"numerator {}\">{}</span><span class=\"denominator {}\">{}</span></span>",
            cancel_class, self.numerator, cancel_class, self.denominator
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Case {
    pub condition: String,
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cases {
    pub cases: Vec<Case>,
    pub bracket_style: BracketStyle,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BracketStyle {
    Curly,
    Square,
    Parentheses,
}

impl Cases {
    pub fn new() -> Self {
        Self {
            cases: Vec::new(),
            bracket_style: BracketStyle::Curly,
        }
    }

    pub fn with_bracket_style(mut self, style: BracketStyle) -> Self {
        self.bracket_style = style;
        self
    }

    pub fn add_case(&mut self, condition: String, result: String) {
        self.cases.push(Case { condition, result });
    }

    /// 生成 LaTeX 格式
    pub fn to_latex(&self) -> String {
        let _bracket = match self.bracket_style {
            BracketStyle::Curly => "\\{",
            BracketStyle::Square => "[",
            BracketStyle::Parentheses => "(",
        };

        let mut latex = "\\begin{cases}\n".to_string();
        for case in &self.cases {
            latex.push_str(&format!(
                "{} & \\text{{if }} {} \\\\\n",
                case.result, case.condition
            ));
        }
        latex.push_str("\\end{cases}");

        latex
    }

    /// 生成 Typst 格式
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();
        typst.push_str("cases(\n");

        for case in &self.cases {
            typst.push_str(&format!("  ({}, {}),\n", case.result, case.condition));
        }

        typst.push_str(")\n");
        typst
    }

    /// 生成 HTML 格式
    pub fn to_html(&self) -> String {
        let mut html = String::new();
        html.push_str("<div class=\"cases\">\n");
        html.push_str("  <div class=\"cases-content\">\n");

        for case in &self.cases {
            html.push_str(&format!(
                "    <div class=\"case\"><span class=\"result\">{}</span> <span class=\"condition\">if {}</span></div>\n",
                case.result, case.condition
            ));
        }

        html.push_str("  </div>\n");
        html.push_str("</div>\n");
        html
    }
}

impl Default for Cases {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Matrix {
    pub rows: Vec<Vec<String>>,
    pub bracket_style: BracketStyle,
    pub alignment: MatrixAlignment,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MatrixAlignment {
    Left,
    Center,
    Right,
}

impl Matrix {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            bracket_style: BracketStyle::Square,
            alignment: MatrixAlignment::Center,
        }
    }

    pub fn with_bracket_style(mut self, style: BracketStyle) -> Self {
        self.bracket_style = style;
        self
    }

    pub fn with_alignment(mut self, alignment: MatrixAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    /// 生成 LaTeX 格式
    pub fn to_latex(&self) -> String {
        let env = match self.bracket_style {
            BracketStyle::Parentheses => "pmatrix",
            BracketStyle::Square => "bmatrix",
            BracketStyle::Curly => "vmatrix",
        };

        let mut latex = format!("\\begin{{{}}}\n", env);

        for row in &self.rows {
            let row_str = row.join(" & ");
            latex.push_str(&format!("{} \\\\\n", row_str));
        }

        latex.push_str(&format!("\\end{{{}}}", env));
        latex
    }

    /// 生成 Typst 格式
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();
        typst.push_str("matrix(\n");

        for row in &self.rows {
            let row_str = row.join(", ");
            typst.push_str(&format!("  ({}),\n", row_str));
        }

        typst.push_str(")\n");
        typst
    }

    /// 生成 HTML 格式
    pub fn to_html(&self) -> String {
        let mut html = String::new();
        html.push_str("<div class=\"matrix\">\n");

        for row in &self.rows {
            html.push_str("  <div class=\"matrix-row\">\n");
            for cell in row {
                html.push_str(&format!("    <div class=\"matrix-cell\">{}</div>\n", cell));
            }
            html.push_str("  </div>\n");
        }

        html.push_str("</div>\n");
        html
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binomial_calculation() {
        let binomial = Binomial::new(5, 2);
        assert_eq!(binomial.calculate(), 10);

        let binomial = Binomial::new(10, 3);
        assert_eq!(binomial.calculate(), 120);
    }

    #[test]
    fn test_binomial_edge_cases() {
        let binomial = Binomial::new(5, 0);
        assert_eq!(binomial.calculate(), 1);

        let binomial = Binomial::new(5, 5);
        assert_eq!(binomial.calculate(), 1);

        let binomial = Binomial::new(5, 6);
        assert_eq!(binomial.calculate(), 0);
    }

    #[test]
    fn test_binomial_to_typst() {
        let binomial = Binomial::new(5, 2);
        let typst = binomial.to_typst();
        assert_eq!(typst, "binomial(5, 2)");
    }

    #[test]
    fn test_primes_generation() {
        let primes = Primes::new(10);
        let result = primes.generate();
        assert_eq!(result, vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_primes_is_prime() {
        let primes = Primes::new(100);
        assert!(primes.is_prime(2));
        assert!(primes.is_prime(7));
        assert!(primes.is_prime(13));
        assert!(!primes.is_prime(4));
        assert!(!primes.is_prime(9));
    }

    #[test]
    fn test_primes_nth_prime() {
        let primes = Primes::new(100);
        assert_eq!(primes.nth_prime(0), Some(2));
        assert_eq!(primes.nth_prime(1), Some(3));
        assert_eq!(primes.nth_prime(4), Some(11));
    }

    #[test]
    fn test_primes_prime_factors() {
        let primes = Primes::new(100);
        let factors = primes.prime_factors(12);
        assert_eq!(factors, vec![(2, 2), (3, 1)]);

        let factors = primes.prime_factors(17);
        assert_eq!(factors, vec![(17, 1)]);
    }

    #[test]
    fn test_cancel_creation() {
        let cancel = Cancel::new("x".to_string(), "y".to_string());
        assert_eq!(cancel.numerator, "x");
        assert_eq!(cancel.denominator, "y");
        assert!(!cancel.cancelled);
    }

    #[test]
    fn test_cancel_with_cancelled() {
        let cancel = Cancel::new("x".to_string(), "y".to_string()).with_cancelled(true);
        assert!(cancel.cancelled);
    }

    #[test]
    fn test_cancel_to_typst() {
        let cancel = Cancel::new("x".to_string(), "y".to_string());
        let typst = cancel.to_typst();
        assert_eq!(typst, "x / y");
    }

    #[test]
    fn test_cases_creation() {
        let cases = Cases::new();
        assert!(cases.cases.is_empty());
    }

    #[test]
    fn test_cases_add_case() {
        let mut cases = Cases::new();
        cases.add_case("x > 0".to_string(), "1".to_string());
        cases.add_case("x < 0".to_string(), "-1".to_string());
        assert_eq!(cases.cases.len(), 2);
    }

    #[test]
    fn test_cases_to_typst() {
        let mut cases = Cases::new();
        cases.add_case("x > 0".to_string(), "1".to_string());
        let typst = cases.to_typst();
        assert!(typst.contains("cases("));
        assert!(typst.contains("x > 0"));
    }

    #[test]
    fn test_matrix_creation() {
        let matrix = Matrix::new();
        assert!(matrix.rows.is_empty());
    }

    #[test]
    fn test_matrix_add_row() {
        let mut matrix = Matrix::new();
        matrix.add_row(vec!["1".to_string(), "2".to_string()]);
        matrix.add_row(vec!["3".to_string(), "4".to_string()]);
        assert_eq!(matrix.rows.len(), 2);
    }

    #[test]
    fn test_matrix_to_typst() {
        let mut matrix = Matrix::new();
        matrix.add_row(vec!["1".to_string(), "2".to_string()]);
        let typst = matrix.to_typst();
        assert!(typst.contains("matrix("));
    }

    #[test]
    fn test_matrix_alignment() {
        let matrix = Matrix::new().with_alignment(MatrixAlignment::Left);
        assert_eq!(matrix.alignment, MatrixAlignment::Left);
    }

    #[test]
    fn test_bracket_styles() {
        let cases = Cases::new().with_bracket_style(BracketStyle::Square);
        assert_eq!(cases.bracket_style, BracketStyle::Square);
    }
}
