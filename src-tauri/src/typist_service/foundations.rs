/*!
 * 航空航天级 Foundations 模块
 * 实现 Typst 的 Foundations 核心功能（array、dictionary、str、calc、eval、regex、datetime）
 */

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 基础值类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum FoundationValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<FoundationValue>),
    Dictionary(HashMap<String, FoundationValue>),
    None,
    DateTime(DateTime<Utc>),
    Bytes(Vec<u8>),
    Function(String),
    Label(String),
    Symbol(String),
    Content(String),
    Module(HashMap<String, FoundationValue>),
    Version(Vec<u32>),
    Duration(i64), // seconds
    Decimal(i64),  // fixed-point with 10 decimal places
    Type(String),
    Target(String),
}

/// 数组操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayOps {
    pub data: Vec<FoundationValue>,
}

impl ArrayOps {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn with_data(data: Vec<FoundationValue>) -> Self {
        Self { data }
    }

    pub fn push(&mut self, value: FoundationValue) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<FoundationValue> {
        self.data.pop()
    }

    pub fn get(&self, index: usize) -> Option<&FoundationValue> {
        self.data.get(index)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn map<F>(&self, f: F) -> Vec<FoundationValue>
    where
        F: Fn(&FoundationValue) -> FoundationValue,
    {
        self.data.iter().map(f).collect()
    }

    pub fn filter<F>(&self, f: F) -> Vec<FoundationValue>
    where
        F: Fn(&FoundationValue) -> bool,
    {
        self.data.iter().filter(|v| f(v)).cloned().collect()
    }

    pub fn reduce<F>(&self, initial: FoundationValue, f: F) -> FoundationValue
    where
        F: Fn(FoundationValue, &FoundationValue) -> FoundationValue,
    {
        self.data.iter().fold(initial, f)
    }

    pub fn to_typst(&self) -> String {
        let items: Vec<String> = self.data.iter().map(|v| self.value_to_typst(v)).collect();
        format!("({})", items.join(", "))
    }

    fn value_to_typst(&self, value: &FoundationValue) -> String {
        match value {
            FoundationValue::String(s) => format!("\"{}\"", html_escape(s)),
            FoundationValue::Number(n) => n.to_string(),
            FoundationValue::Boolean(b) => b.to_string(),
            FoundationValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| self.value_to_typst(v)).collect();
                format!("({})", items.join(", "))
            }
            FoundationValue::Dictionary(dict) => {
                let items: Vec<String> = dict
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, self.value_to_typst(v)))
                    .collect();
                format!("({})", items.join(", "))
            }
            FoundationValue::None => "none".to_string(),
            FoundationValue::DateTime(dt) => {
                format!("datetime(\"{}\")", dt.format("%Y-%m-%d %H:%M:%S"))
            }
            FoundationValue::Bytes(bytes) => format!("bytes(\"{}\")", hex::encode(bytes)),
            FoundationValue::Function(f) => format!("<function: {}>", f),
            FoundationValue::Label(l) => format!("<{}>", l),
            FoundationValue::Symbol(s) => format!("symbol(\"{}\")", s),
            FoundationValue::Content(c) => format!("[{}]", c),
            FoundationValue::Module(_) => "<module>".to_string(),
            FoundationValue::Version(v) => format!(
                "version({})",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(".")
            ),
            FoundationValue::Duration(d) => format!("duration({}s)", d),
            FoundationValue::Decimal(d) => format!("decimal({})", d),
            FoundationValue::Type(t) => format!("<type: {}>", t),
            FoundationValue::Target(t) => format!("target(\"{}\")", t),
        }
    }
}

impl Default for ArrayOps {
    fn default() -> Self {
        Self::new()
    }
}

/// 字典操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryOps {
    pub data: HashMap<String, FoundationValue>,
}

impl DictionaryOps {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn with_data(data: HashMap<String, FoundationValue>) -> Self {
        Self { data }
    }

    pub fn insert(&mut self, key: String, value: FoundationValue) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&FoundationValue> {
        self.data.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<FoundationValue> {
        self.data.remove(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    pub fn values(&self) -> Vec<FoundationValue> {
        self.data.values().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn to_typst(&self) -> String {
        let items: Vec<String> = self
            .data
            .iter()
            .map(|(k, v)| format!("{}: {}", k, self.value_to_typst(v)))
            .collect();
        format!("({})", items.join(", "))
    }

    fn value_to_typst(&self, value: &FoundationValue) -> String {
        match value {
            FoundationValue::String(s) => format!("\"{}\"", html_escape(s)),
            FoundationValue::Number(n) => n.to_string(),
            FoundationValue::Boolean(b) => b.to_string(),
            FoundationValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| self.value_to_typst(v)).collect();
                format!("({})", items.join(", "))
            }
            FoundationValue::Dictionary(dict) => {
                let items: Vec<String> = dict
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, self.value_to_typst(v)))
                    .collect();
                format!("({})", items.join(", "))
            }
            FoundationValue::None => "none".to_string(),
            FoundationValue::DateTime(dt) => {
                format!("datetime(\"{}\")", dt.format("%Y-%m-%d %H:%M:%S"))
            }
            FoundationValue::Bytes(bytes) => format!("bytes(\"{}\")", hex::encode(bytes)),
            FoundationValue::Function(f) => format!("<function: {}>", f),
            FoundationValue::Label(l) => format!("<{}>", l),
            FoundationValue::Symbol(s) => format!("symbol(\"{}\")", s),
            FoundationValue::Content(c) => format!("[{}]", c),
            FoundationValue::Module(_) => "<module>".to_string(),
            FoundationValue::Version(v) => format!(
                "version({})",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(".")
            ),
            FoundationValue::Duration(d) => format!("duration({}s)", d),
            FoundationValue::Decimal(d) => format!("decimal({})", d),
            FoundationValue::Type(t) => format!("<type: {}>", t),
            FoundationValue::Target(t) => format!("target(\"{}\")", t),
        }
    }
}

impl Default for DictionaryOps {
    fn default() -> Self {
        Self::new()
    }
}

/// 字符串操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringOps;

impl StringOps {
    pub fn new() -> Self {
        Self
    }

    pub fn length(s: &str) -> usize {
        s.chars().count()
    }

    pub fn to_uppercase(s: &str) -> String {
        s.to_uppercase()
    }

    pub fn to_lowercase(s: &str) -> String {
        s.to_lowercase()
    }

    pub fn trim(s: &str) -> String {
        s.trim().to_string()
    }

    pub fn split(s: &str, delimiter: &str) -> Vec<String> {
        s.split(delimiter).map(|s| s.to_string()).collect()
    }

    pub fn replace(s: &str, from: &str, to: &str) -> String {
        s.replace(from, to)
    }

    pub fn contains(s: &str, pattern: &str) -> bool {
        s.contains(pattern)
    }

    pub fn starts_with(s: &str, pattern: &str) -> bool {
        s.starts_with(pattern)
    }

    pub fn ends_with(s: &str, pattern: &str) -> bool {
        s.ends_with(pattern)
    }

    pub fn substring(s: &str, start: usize, end: usize) -> String {
        s.chars().skip(start).take(end - start).collect()
    }

    pub fn repeat(s: &str, count: usize) -> String {
        s.repeat(count)
    }
}

impl Default for StringOps {
    fn default() -> Self {
        Self::new()
    }
}

/// 计算操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalcOps;

impl CalcOps {
    pub fn new() -> Self {
        Self
    }

    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    pub fn sub(a: f64, b: f64) -> f64 {
        a - b
    }

    pub fn mul(a: f64, b: f64) -> f64 {
        a * b
    }

    pub fn div(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("Division by zero".to_string())
        } else {
            Ok(a / b)
        }
    }

    pub fn pow(a: f64, b: f64) -> f64 {
        a.powf(b)
    }

    pub fn sqrt(a: f64) -> Result<f64, String> {
        if a < 0.0 {
            Err("Cannot calculate square root of negative number".to_string())
        } else {
            Ok(a.sqrt())
        }
    }

    pub fn abs(a: f64) -> f64 {
        a.abs()
    }

    pub fn round(a: f64) -> f64 {
        a.round()
    }

    pub fn floor(a: f64) -> f64 {
        a.floor()
    }

    pub fn ceil(a: f64) -> f64 {
        a.ceil()
    }

    pub fn sin(a: f64) -> f64 {
        a.sin()
    }

    pub fn cos(a: f64) -> f64 {
        a.cos()
    }

    pub fn tan(a: f64) -> f64 {
        a.tan()
    }

    pub fn log(a: f64) -> f64 {
        a.ln()
    }

    pub fn log10(a: f64) -> f64 {
        a.log10()
    }

    pub fn max(a: f64, b: f64) -> f64 {
        a.max(b)
    }

    pub fn min(a: f64, b: f64) -> f64 {
        a.min(b)
    }
}

impl Default for CalcOps {
    fn default() -> Self {
        Self::new()
    }
}

/// 正则表达式操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegexOps {
    pattern: String,
}

impl RegexOps {
    pub fn new(pattern: &str) -> Result<Self, String> {
        Regex::new(pattern)
            .map(|_| Self {
                pattern: pattern.to_string(),
            })
            .map_err(|e| format!("Invalid regex: {}", e))
    }

    pub fn is_match(&self, text: &str) -> bool {
        if let Ok(regex) = Regex::new(&self.pattern) {
            regex.is_match(text)
        } else {
            false
        }
    }

    pub fn find(&self, text: &str) -> Vec<String> {
        if let Ok(regex) = Regex::new(&self.pattern) {
            regex
                .find_iter(text)
                .map(|m| m.as_str().to_string())
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn replace(&self, text: &str, replacement: &str) -> String {
        if let Ok(regex) = Regex::new(&self.pattern) {
            regex.replace_all(text, replacement).to_string()
        } else {
            text.to_string()
        }
    }

    pub fn captures(&self, text: &str) -> Vec<Vec<String>> {
        if let Ok(regex) = Regex::new(&self.pattern) {
            regex
                .captures_iter(text)
                .map(|caps| {
                    caps.iter()
                        .skip(1)
                        .filter_map(|m| m.map(|s| s.as_str().to_string()))
                        .collect()
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

/// 日期时间操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTimeOps;

impl DateTimeOps {
    pub fn new() -> Self {
        Self
    }

    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }

    pub fn from_string(s: &str) -> Result<DateTime<Utc>, String> {
        // Try multiple formats
        let formats = [
            "%Y-%m-%d %H:%M:%S",
            "%Y-%m-%d",
            "%Y/%m/%d %H:%M:%S",
            "%Y/%m/%d",
        ];

        for format in &formats {
            if let Ok(naive) = NaiveDateTime::parse_from_str(s, format) {
                return Ok(DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc));
            }
            if let Ok(naive) = NaiveDate::parse_from_str(s, format) {
                return Ok(DateTime::<Utc>::from_naive_utc_and_offset(
                    naive.and_hms_opt(0, 0, 0).unwrap(),
                    Utc,
                ));
            }
        }

        Err("Invalid datetime format".to_string())
    }

    pub fn to_string(dt: &DateTime<Utc>, format: &str) -> String {
        dt.format(format).to_string()
    }

    pub fn add_days(dt: &DateTime<Utc>, days: i64) -> DateTime<Utc> {
        *dt + chrono::Duration::days(days)
    }

    pub fn add_hours(dt: &DateTime<Utc>, hours: i64) -> DateTime<Utc> {
        *dt + chrono::Duration::hours(hours)
    }

    pub fn add_minutes(dt: &DateTime<Utc>, minutes: i64) -> DateTime<Utc> {
        *dt + chrono::Duration::minutes(minutes)
    }

    pub fn diff(dt1: &DateTime<Utc>, dt2: &DateTime<Utc>) -> chrono::Duration {
        *dt1 - *dt2
    }

    pub fn format_iso8601(dt: &DateTime<Utc>) -> String {
        dt.to_rfc3339()
    }
}

impl Default for DateTimeOps {
    fn default() -> Self {
        Self::new()
    }
}

/// 代码评估
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalOps;

impl EvalOps {
    pub fn new() -> Self {
        Self
    }

    /// 简单的数学表达式评估
    pub fn eval_math(expr: &str) -> Result<f64, String> {
        // 这是一个简化的实现，实际应用中应该使用更安全的解析器
        let expr = expr.replace(" ", "");

        // 简单的加减乘除
        if let Some((left, right)) = expr.split_once('+') {
            let left_val = Self::eval_math(left)?;
            let right_val = Self::eval_math(right)?;
            return Ok(left_val + right_val);
        }

        if let Some((left, right)) = expr.split_once('-') {
            let left_val = Self::eval_math(left)?;
            let right_val = Self::eval_math(right)?;
            return Ok(left_val - right_val);
        }

        if let Some((left, right)) = expr.split_once('*') {
            let left_val = Self::eval_math(left)?;
            let right_val = Self::eval_math(right)?;
            return Ok(left_val * right_val);
        }

        if let Some((left, right)) = expr.split_once('/') {
            let left_val = Self::eval_math(left)?;
            let right_val = Self::eval_math(right)?;
            if right_val == 0.0 {
                return Err("Division by zero".to_string());
            }
            return Ok(left_val / right_val);
        }

        // 尝试解析为数字
        expr.parse::<f64>()
            .map_err(|_| format!("Invalid expression: {}", expr))
    }
}

impl Default for EvalOps {
    fn default() -> Self {
        Self::new()
    }
}

/// Arguments - 捕获的函数参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arguments {
    pub positional: Vec<FoundationValue>,
    pub named: HashMap<String, FoundationValue>,
}

impl Arguments {
    pub fn new() -> Self {
        Self {
            positional: Vec::new(),
            named: HashMap::new(),
        }
    }

    pub fn with_positional(mut self, value: FoundationValue) -> Self {
        self.positional.push(value);
        self
    }

    pub fn with_named(mut self, key: String, value: FoundationValue) -> Self {
        self.named.insert(key, value);
        self
    }

    pub fn get_positional(&self, index: usize) -> Option<&FoundationValue> {
        self.positional.get(index)
    }

    pub fn get_named(&self, key: &str) -> Option<&FoundationValue> {
        self.named.get(key)
    }
}

impl Default for Arguments {
    fn default() -> Self {
        Self::new()
    }
}

/// Assert - 确保条件满足
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assert;

impl Assert {
    pub fn new() -> Self {
        Self
    }

    pub fn assert(condition: bool, message: &str) -> Result<(), String> {
        if condition {
            Ok(())
        } else {
            Err(message.to_string())
        }
    }

    pub fn assert_eq<T: PartialEq + std::fmt::Debug>(a: T, b: T) -> Result<(), String> {
        if a == b {
            Ok(())
        } else {
            Err(format!("Assertion failed: {:?} != {:?}", a, b))
        }
    }
}

impl Default for Assert {
    fn default() -> Self {
        Self::new()
    }
}

/// Auto - 智能默认值
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Auto;

impl Auto {
    pub fn new() -> Self {
        Self
    }

    pub fn to_typst(&self) -> String {
        "auto".to_string()
    }
}

impl Default for Auto {
    fn default() -> Self {
        Self::new()
    }
}

/// Bool - 布尔类型操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoolOps;

impl BoolOps {
    pub fn new() -> Self {
        Self
    }

    pub fn not(value: bool) -> bool {
        !value
    }

    pub fn and(a: bool, b: bool) -> bool {
        a && b
    }

    pub fn or(a: bool, b: bool) -> bool {
        a || b
    }

    pub fn xor(a: bool, b: bool) -> bool {
        a ^ b
    }

    pub fn to_typst(value: bool) -> String {
        if value {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }
}

impl Default for BoolOps {
    fn default() -> Self {
        Self::new()
    }
}

/// Bytes - 字节序列操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BytesOps {
    pub data: Vec<u8>,
}

impl BytesOps {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn with_data(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn to_hex(&self) -> String {
        self.data.iter().map(|b| format!("{:02x}", b)).collect()
    }

    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let mut data = Vec::new();
        for i in (0..hex.len()).step_by(2) {
            let byte_str = &hex[i..i + 2];
            let byte = u8::from_str_radix(byte_str, 16)
                .map_err(|_| format!("Invalid hex: {}", byte_str))?;
            data.push(byte);
        }
        Ok(Self { data })
    }
}

impl Default for BytesOps {
    fn default() -> Self {
        Self::new()
    }
}

/// Content - 文档内容片段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub value: String,
}

impl Content {
    pub fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    pub fn with_value(value: String) -> Self {
        Self { value }
    }

    pub fn to_typst(&self) -> String {
        format!("[{}]", self.value)
    }
}

impl Default for Content {
    fn default() -> Self {
        Self::new()
    }
}

/// Decimal - 定点十进制数操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecimalOps;

impl DecimalOps {
    pub fn new() -> Self {
        Self
    }

    /// Convert from i64 (fixed-point with 10 decimal places)
    pub fn from_i64(value: i64) -> f64 {
        value as f64 / 1e10
    }

    /// Convert to i64 (fixed-point with 10 decimal places)
    pub fn to_i64(value: f64) -> i64 {
        (value * 1e10) as i64
    }

    pub fn add(a: i64, b: i64) -> i64 {
        a + b
    }

    pub fn sub(a: i64, b: i64) -> i64 {
        a - b
    }

    pub fn mul(a: i64, b: i64) -> i64 {
        (a as f64 * b as f64 / 1e10) as i64
    }

    pub fn div(a: i64, b: i64) -> Result<i64, String> {
        if b == 0 {
            Err("Division by zero".to_string())
        } else {
            Ok((a as f64 / b as f64 * 1e10) as i64)
        }
    }
}

impl Default for DecimalOps {
    fn default() -> Self {
        Self::new()
    }
}

/// Duration - 时间跨度操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationOps;

impl DurationOps {
    pub fn new() -> Self {
        Self
    }

    pub fn from_seconds(seconds: i64) -> i64 {
        seconds
    }

    pub fn from_minutes(minutes: i64) -> i64 {
        minutes * 60
    }

    pub fn from_hours(hours: i64) -> i64 {
        hours * 3600
    }

    pub fn from_days(days: i64) -> i64 {
        days * 86400
    }

    pub fn to_seconds(duration: i64) -> i64 {
        duration
    }

    pub fn to_minutes(duration: i64) -> i64 {
        duration / 60
    }

    pub fn to_hours(duration: i64) -> i64 {
        duration / 3600
    }

    pub fn to_days(duration: i64) -> i64 {
        duration / 86400
    }

    pub fn add(a: i64, b: i64) -> i64 {
        a + b
    }

    pub fn sub(a: i64, b: i64) -> i64 {
        a - b
    }
}

impl Default for DurationOps {
    fn default() -> Self {
        Self::new()
    }
}

/// Float - 浮点数操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatOps;

impl FloatOps {
    pub fn new() -> Self {
        Self
    }

    pub fn is_nan(value: f64) -> bool {
        value.is_nan()
    }

    pub fn is_infinite(value: f64) -> bool {
        value.is_infinite()
    }

    pub fn is_finite(value: f64) -> bool {
        value.is_finite()
    }

    pub fn floor(value: f64) -> f64 {
        value.floor()
    }

    pub fn ceil(value: f64) -> f64 {
        value.ceil()
    }

    pub fn round(value: f64) -> f64 {
        value.round()
    }

    pub fn trunc(value: f64) -> f64 {
        value.trunc()
    }

    pub fn fract(value: f64) -> f64 {
        value.fract()
    }
}

impl Default for FloatOps {
    fn default() -> Self {
        Self::new()
    }
}

/// Function - 函数类型操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionOps {
    pub name: String,
    pub params: Vec<String>,
}

impl FunctionOps {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: Vec::new(),
        }
    }

    pub fn with_params(mut self, params: Vec<String>) -> Self {
        self.params = params;
        self
    }

    pub fn to_typst(&self) -> String {
        if self.params.is_empty() {
            format!("({})", self.name)
        } else {
            format!("({}: {})", self.name, self.params.join(", "))
        }
    }
}

/// Int - 整数操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntOps;

impl IntOps {
    pub fn new() -> Self {
        Self
    }

    pub fn abs(value: i64) -> i64 {
        value.abs()
    }

    pub fn signum(value: i64) -> i64 {
        value.signum()
    }

    pub fn clamp(value: i64, min: i64, max: i64) -> i64 {
        value.max(min).min(max)
    }

    pub fn to_float(value: i64) -> f64 {
        value as f64
    }
}

impl Default for IntOps {
    fn default() -> Self {
        Self::new()
    }
}

/// Label - 元素标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundationLabel {
    pub name: String,
}

impl FoundationLabel {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn to_typst(&self) -> String {
        format!("<{}>", self.name)
    }
}

/// Module - 模块操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleOps {
    pub variables: HashMap<String, FoundationValue>,
    pub functions: HashMap<String, String>,
}

impl ModuleOps {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn with_variable(mut self, key: String, value: FoundationValue) -> Self {
        self.variables.insert(key, value);
        self
    }

    pub fn with_function(mut self, key: String, value: String) -> Self {
        self.functions.insert(key, value);
        self
    }

    pub fn get_variable(&self, key: &str) -> Option<&FoundationValue> {
        self.variables.get(key)
    }

    pub fn get_function(&self, key: &str) -> Option<&String> {
        self.functions.get(key)
    }
}

impl Default for ModuleOps {
    fn default() -> Self {
        Self::new()
    }
}

/// None - 无值操作
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NoneValue;

impl NoneValue {
    pub fn new() -> Self {
        Self
    }

    pub fn to_typst(&self) -> String {
        "none".to_string()
    }
}

impl Default for NoneValue {
    fn default() -> Self {
        Self::new()
    }
}

/// Panic - 失败并报错
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panic;

impl Panic {
    pub fn new() -> Self {
        Self
    }

    pub fn panic(message: &str) -> String {
        format!("panic: {}", message)
    }
}

impl Default for Panic {
    fn default() -> Self {
        Self::new()
    }
}

/// Repr - 返回值的字符串表示
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repr;

impl Repr {
    pub fn new() -> Self {
        Self
    }

    pub fn repr(value: &FoundationValue) -> String {
        match value {
            FoundationValue::String(s) => format!("\"{}\"", s),
            FoundationValue::Number(n) => n.to_string(),
            FoundationValue::Boolean(b) => b.to_string(),
            FoundationValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(Self::repr).collect();
                format!("({})", items.join(", "))
            }
            FoundationValue::Dictionary(dict) => {
                let items: Vec<String> = dict
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, Self::repr(v)))
                    .collect();
                format!("({})", items.join(", "))
            }
            FoundationValue::None => "none".to_string(),
            FoundationValue::DateTime(dt) => {
                format!("datetime(\"{}\")", dt.format("%Y-%m-%d %H:%M:%S"))
            }
            FoundationValue::Bytes(bytes) => format!("bytes(\"{}\")", hex::encode(bytes)),
            FoundationValue::Function(f) => format!("<function: {}>", f),
            FoundationValue::Label(l) => format!("<{}>", l),
            FoundationValue::Symbol(s) => format!("symbol(\"{}\")", s),
            FoundationValue::Content(c) => format!("[{}]", c),
            FoundationValue::Module(_) => "<module>".to_string(),
            FoundationValue::Version(v) => format!(
                "version({})",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(".")
            ),
            FoundationValue::Duration(d) => format!("duration({}s)", d),
            FoundationValue::Decimal(d) => format!("decimal({})", d),
            FoundationValue::Type(t) => format!("<type: {}>", t),
            FoundationValue::Target(t) => format!("target(\"{}\")", t),
        }
    }
}

impl Default for Repr {
    fn default() -> Self {
        Self::new()
    }
}

/// Selector - 元素选择器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Selector {
    pub pattern: String,
    pub selector_type: SelectorType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SelectorType {
    Element,
    Label,
    Class,
    Function,
}

impl Selector {
    pub fn new(pattern: String) -> Self {
        Self {
            pattern,
            selector_type: SelectorType::Element,
        }
    }

    pub fn with_type(mut self, selector_type: SelectorType) -> Self {
        self.selector_type = selector_type;
        self
    }

    pub fn to_typst(&self) -> String {
        match self.selector_type {
            SelectorType::Element => format!("selector(\"{}\")", self.pattern),
            SelectorType::Label => format!("selector(<{}>)", self.pattern),
            SelectorType::Class => format!("selector(.{})", self.pattern),
            SelectorType::Function => format!("selector({})", self.pattern),
        }
    }
}

/// Std - 标准模块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Std;

impl Std {
    pub fn new() -> Self {
        Self
    }

    pub fn get_module() -> ModuleOps {
        ModuleOps::new()
    }
}

impl Default for Std {
    fn default() -> Self {
        Self::new()
    }
}

/// Symbol - Unicode 符号
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundationSymbol {
    pub code: String,
}

impl FoundationSymbol {
    pub fn new(code: String) -> Self {
        Self { code }
    }

    pub fn to_typst(&self) -> String {
        format!("symbol(\"{}\")", self.code)
    }
}

/// Sys - 系统交互模块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sys;

impl Sys {
    pub fn new() -> Self {
        Self
    }

    pub fn get_version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    pub fn get_os() -> String {
        std::env::consts::OS.to_string()
    }

    pub fn get_arch() -> String {
        std::env::consts::ARCH.to_string()
    }
}

impl Default for Sys {
    fn default() -> Self {
        Self::new()
    }
}

/// Target - 导出目标
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExportTarget {
    Pdf,
    Html,
    Svg,
    Png,
}

impl ExportTarget {
    pub fn new() -> Self {
        Self::Pdf
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "pdf" => Ok(Self::Pdf),
            "html" => Ok(Self::Html),
            "svg" => Ok(Self::Svg),
            "png" => Ok(Self::Png),
            _ => Err(format!("Unknown target: {}", s)),
        }
    }
}

impl std::fmt::Display for ExportTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pdf => write!(f, "pdf"),
            Self::Html => write!(f, "html"),
            Self::Svg => write!(f, "svg"),
            Self::Png => write!(f, "png"),
        }
    }
}

impl Default for ExportTarget {
    fn default() -> Self {
        Self::new()
    }
}

/// Type - 类型描述
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeOps;

impl TypeOps {
    pub fn new() -> Self {
        Self
    }

    pub fn get_type(value: &FoundationValue) -> String {
        match value {
            FoundationValue::String(_) => "string".to_string(),
            FoundationValue::Number(_) => "number".to_string(),
            FoundationValue::Boolean(_) => "boolean".to_string(),
            FoundationValue::Array(_) => "array".to_string(),
            FoundationValue::Dictionary(_) => "dictionary".to_string(),
            FoundationValue::None => "none".to_string(),
            FoundationValue::DateTime(_) => "datetime".to_string(),
            FoundationValue::Bytes(_) => "bytes".to_string(),
            FoundationValue::Function(_) => "function".to_string(),
            FoundationValue::Label(_) => "label".to_string(),
            FoundationValue::Symbol(_) => "symbol".to_string(),
            FoundationValue::Content(_) => "content".to_string(),
            FoundationValue::Module(_) => "module".to_string(),
            FoundationValue::Version(_) => "version".to_string(),
            FoundationValue::Duration(_) => "duration".to_string(),
            FoundationValue::Decimal(_) => "decimal".to_string(),
            FoundationValue::Type(_) => "type".to_string(),
            FoundationValue::Target(_) => "target".to_string(),
        }
    }
}

impl Default for TypeOps {
    fn default() -> Self {
        Self::new()
    }
}

/// Version - 版本操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionOps {
    pub components: Vec<u32>,
}

impl VersionOps {
    pub fn new(components: Vec<u32>) -> Self {
        Self { components }
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        let components: Result<Vec<u32>, _> = s
            .split('.')
            .map(|part| part.parse::<u32>().map_err(|e| e.to_string()))
            .collect();
        components.map(|comps| Self { components: comps })
    }

    pub fn compare(&self, other: &Self) -> std::cmp::Ordering {
        for (a, b) in self.components.iter().zip(other.components.iter()) {
            match a.cmp(b) {
                std::cmp::Ordering::Equal => continue,
                other => return other,
            }
        }
        self.components.len().cmp(&other.components.len())
    }
}

impl std::fmt::Display for VersionOps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.components.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("."))
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
    use chrono::Datelike;

    #[test]
    fn test_array_ops_creation() {
        let array = ArrayOps::new();
        assert_eq!(array.len(), 0);
    }

    #[test]
    fn test_array_ops_push() {
        let mut array = ArrayOps::new();
        array.push(FoundationValue::Number(42.0));
        assert_eq!(array.len(), 1);
    }

    #[test]
    fn test_array_ops_get() {
        let mut array = ArrayOps::new();
        array.push(FoundationValue::Number(42.0));
        assert_eq!(array.get(0), Some(&FoundationValue::Number(42.0)));
    }

    #[test]
    fn test_array_ops_to_typst() {
        let mut array = ArrayOps::new();
        array.push(FoundationValue::Number(1.0));
        array.push(FoundationValue::Number(2.0));
        let typst = array.to_typst();
        assert_eq!(typst, "(1, 2)");
    }

    #[test]
    fn test_dictionary_ops_creation() {
        let dict = DictionaryOps::new();
        assert_eq!(dict.len(), 0);
    }

    #[test]
    fn test_dictionary_ops_insert() {
        let mut dict = DictionaryOps::new();
        dict.insert("key".to_string(), FoundationValue::Number(42.0));
        assert_eq!(dict.len(), 1);
    }

    #[test]
    fn test_dictionary_ops_get() {
        let mut dict = DictionaryOps::new();
        dict.insert("key".to_string(), FoundationValue::Number(42.0));
        assert_eq!(dict.get("key"), Some(&FoundationValue::Number(42.0)));
    }

    #[test]
    fn test_string_ops_length() {
        assert_eq!(StringOps::length("hello"), 5);
    }

    #[test]
    fn test_string_ops_to_uppercase() {
        assert_eq!(StringOps::to_uppercase("hello"), "HELLO");
    }

    #[test]
    fn test_string_ops_to_lowercase() {
        assert_eq!(StringOps::to_lowercase("HELLO"), "hello");
    }

    #[test]
    fn test_string_ops_trim() {
        assert_eq!(StringOps::trim("  hello  "), "hello");
    }

    #[test]
    fn test_string_ops_split() {
        let result = StringOps::split("a,b,c", ",");
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_string_ops_replace() {
        assert_eq!(
            StringOps::replace("hello world", "world", "typst"),
            "hello typst"
        );
    }

    #[test]
    fn test_calc_ops_add() {
        assert_eq!(CalcOps::add(2.0, 3.0), 5.0);
    }

    #[test]
    fn test_calc_ops_sub() {
        assert_eq!(CalcOps::sub(5.0, 3.0), 2.0);
    }

    #[test]
    fn test_calc_ops_mul() {
        assert_eq!(CalcOps::mul(2.0, 3.0), 6.0);
    }

    #[test]
    fn test_calc_ops_div() {
        assert_eq!(CalcOps::div(6.0, 2.0), Ok(3.0));
    }

    #[test]
    fn test_calc_ops_div_by_zero() {
        assert!(CalcOps::div(6.0, 0.0).is_err());
    }

    #[test]
    fn test_calc_ops_pow() {
        assert_eq!(CalcOps::pow(2.0, 3.0), 8.0);
    }

    #[test]
    fn test_calc_ops_sqrt() {
        assert_eq!(CalcOps::sqrt(4.0), Ok(2.0));
    }

    #[test]
    fn test_calc_ops_sqrt_negative() {
        assert!(CalcOps::sqrt(-4.0).is_err());
    }

    #[test]
    fn test_regex_ops_creation() {
        let regex = RegexOps::new(r"\d+").unwrap();
        assert!(regex.is_match("123"));
    }

    #[test]
    fn test_regex_ops_is_match() {
        let regex = RegexOps::new(r"\d+").unwrap();
        assert!(regex.is_match("123"));
        assert!(!regex.is_match("abc"));
    }

    #[test]
    fn test_regex_ops_find() {
        let regex = RegexOps::new(r"\d+").unwrap();
        let result = regex.find("abc123def456");
        assert_eq!(result, vec!["123", "456"]);
    }

    #[test]
    fn test_regex_ops_replace() {
        let regex = RegexOps::new(r"\d+").unwrap();
        let result = regex.replace("abc123def", "X");
        assert_eq!(result, "abcXdef");
    }

    #[test]
    fn test_datetime_ops_now() {
        let now = DateTimeOps::now();
        assert!(now.timestamp() > 0);
    }

    #[test]
    fn test_datetime_ops_from_string() {
        let dt = DateTimeOps::from_string("2024-01-01 00:00:00").unwrap();
        assert_eq!(dt.year(), 2024);
    }

    #[test]
    fn test_datetime_ops_to_string() {
        let dt = DateTimeOps::from_string("2024-01-01 00:00:00").unwrap();
        let formatted = DateTimeOps::to_string(&dt, "%Y-%m-%d");
        assert_eq!(formatted, "2024-01-01");
    }

    #[test]
    fn test_datetime_ops_add_days() {
        let dt = DateTimeOps::from_string("2024-01-01 00:00:00").unwrap();
        let result = DateTimeOps::add_days(&dt, 1);
        assert_eq!(result.day(), 2);
    }

    #[test]
    fn test_eval_ops_eval_math() {
        assert_eq!(EvalOps::eval_math("2+3"), Ok(5.0));
        assert_eq!(EvalOps::eval_math("2*3"), Ok(6.0));
        assert_eq!(EvalOps::eval_math("6/2"), Ok(3.0));
    }

    #[test]
    fn test_eval_ops_eval_math_complex() {
        assert_eq!(EvalOps::eval_math("2+3*4"), Ok(14.0));
    }

    #[test]
    fn test_eval_ops_eval_math_error() {
        assert!(EvalOps::eval_math("invalid").is_err());
    }

    #[test]
    fn test_arguments_creation() {
        let args = Arguments::new();
        assert_eq!(args.positional.len(), 0);
        assert_eq!(args.named.len(), 0);
    }

    #[test]
    fn test_arguments_with_positional() {
        let args = Arguments::new().with_positional(FoundationValue::Number(42.0));
        assert_eq!(args.positional.len(), 1);
    }

    #[test]
    fn test_arguments_with_named() {
        let args = Arguments::new().with_named("key".to_string(), FoundationValue::Number(42.0));
        assert_eq!(args.named.len(), 1);
    }

    #[test]
    fn test_assert_success() {
        assert!(Assert::assert(true, "test").is_ok());
    }

    #[test]
    fn test_assert_failure() {
        assert!(Assert::assert(false, "test").is_err());
    }

    #[test]
    fn test_assert_eq_success() {
        assert!(Assert::assert_eq(1, 1).is_ok());
    }

    #[test]
    fn test_assert_eq_failure() {
        assert!(Assert::assert_eq(1, 2).is_err());
    }

    #[test]
    fn test_auto_to_typst() {
        let auto = Auto::new();
        assert_eq!(auto.to_typst(), "auto");
    }

    #[test]
    fn test_bool_ops_not() {
        assert_eq!(BoolOps::not(true), false);
        assert_eq!(BoolOps::not(false), true);
    }

    #[test]
    fn test_bool_ops_and() {
        assert_eq!(BoolOps::and(true, true), true);
        assert_eq!(BoolOps::and(true, false), false);
    }

    #[test]
    fn test_bool_ops_or() {
        assert_eq!(BoolOps::or(true, false), true);
        assert_eq!(BoolOps::or(false, false), false);
    }

    #[test]
    fn test_bool_ops_xor() {
        assert_eq!(BoolOps::xor(true, false), true);
        assert_eq!(BoolOps::xor(true, true), false);
    }

    #[test]
    fn test_bytes_ops_creation() {
        let bytes = BytesOps::new();
        assert_eq!(bytes.len(), 0);
    }

    #[test]
    fn test_bytes_ops_to_hex() {
        let bytes = BytesOps::with_data(vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]);
        assert_eq!(bytes.to_hex(), "48656c6c6f");
    }

    #[test]
    fn test_bytes_ops_from_hex() {
        let bytes = BytesOps::from_hex("48656c6c6f").unwrap();
        assert_eq!(bytes.data, vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]);
    }

    #[test]
    fn test_content_creation() {
        let content = Content::new();
        assert_eq!(content.value, "");
    }

    #[test]
    fn test_content_to_typst() {
        let content = Content::with_value("Hello".to_string());
        assert_eq!(content.to_typst(), "[Hello]");
    }

    #[test]
    fn test_decimal_ops_conversion() {
        let value = 123.456789;
        let decimal = DecimalOps::to_i64(value);
        let back = DecimalOps::from_i64(decimal);
        assert!((back - value).abs() < 0.0001);
    }

    #[test]
    fn test_decimal_ops_add() {
        assert_eq!(DecimalOps::add(10000000000, 20000000000), 30000000000);
    }

    #[test]
    fn test_duration_ops_from_seconds() {
        assert_eq!(DurationOps::from_seconds(60), 60);
    }

    #[test]
    fn test_duration_ops_from_minutes() {
        assert_eq!(DurationOps::from_minutes(1), 60);
    }

    #[test]
    fn test_duration_ops_to_minutes() {
        assert_eq!(DurationOps::to_minutes(120), 2);
    }

    #[test]
    fn test_float_ops_is_nan() {
        assert!(FloatOps::is_nan(f64::NAN));
        assert!(!FloatOps::is_nan(1.0));
    }

    #[test]
    fn test_float_ops_is_infinite() {
        assert!(FloatOps::is_infinite(f64::INFINITY));
        assert!(!FloatOps::is_infinite(1.0));
    }

    #[test]
    fn test_float_ops_floor() {
        assert_eq!(FloatOps::floor(3.7), 3.0);
    }

    #[test]
    fn test_float_ops_ceil() {
        assert_eq!(FloatOps::ceil(3.2), 4.0);
    }

    #[test]
    fn test_int_ops_abs() {
        assert_eq!(IntOps::abs(-5), 5);
        assert_eq!(IntOps::abs(5), 5);
    }

    #[test]
    fn test_int_ops_clamp() {
        assert_eq!(IntOps::clamp(5, 1, 10), 5);
        assert_eq!(IntOps::clamp(0, 1, 10), 1);
        assert_eq!(IntOps::clamp(15, 1, 10), 10);
    }

    #[test]
    fn test_label_to_typst() {
        let label = FoundationLabel::new("test".to_string());
        assert_eq!(label.to_typst(), "<test>");
    }

    #[test]
    fn test_module_ops_creation() {
        let module = ModuleOps::new();
        assert_eq!(module.variables.len(), 0);
    }

    #[test]
    fn test_module_ops_with_variable() {
        let module =
            ModuleOps::new().with_variable("key".to_string(), FoundationValue::Number(42.0));
        assert_eq!(module.variables.len(), 1);
    }

    #[test]
    fn test_none_value_to_typst() {
        let none = NoneValue::new();
        assert_eq!(none.to_typst(), "none");
    }

    #[test]
    fn test_panic() {
        let result = Panic::panic("test error");
        assert!(result.contains("panic"));
    }

    #[test]
    fn test_repr_string() {
        let value = FoundationValue::String("hello".to_string());
        assert_eq!(Repr::repr(&value), "\"hello\"");
    }

    #[test]
    fn test_repr_number() {
        let value = FoundationValue::Number(42.0);
        assert_eq!(Repr::repr(&value), "42");
    }

    #[test]
    fn test_selector_creation() {
        let selector = Selector::new("heading".to_string());
        assert_eq!(selector.pattern, "heading");
    }

    #[test]
    fn test_selector_to_typst() {
        let selector = Selector::new("heading".to_string()).with_type(SelectorType::Element);
        assert!(selector.to_typst().contains("heading"));
    }

    #[test]
    fn test_sys_get_version() {
        let version = Sys::get_version();
        assert!(!version.is_empty());
    }

    #[test]
    fn test_sys_get_os() {
        let os = Sys::get_os();
        assert!(!os.is_empty());
    }

    #[test]
    fn test_target_from_string() {
        assert!(ExportTarget::from_string("pdf").is_ok());
        assert!(ExportTarget::from_string("html").is_ok());
        assert!(ExportTarget::from_string("invalid").is_err());
    }

    #[test]
    fn test_target_to_string() {
        let target = ExportTarget::new();
        assert_eq!(target.to_string(), "pdf");
    }

    #[test]
    fn test_type_ops_get_type() {
        let value = FoundationValue::String("hello".to_string());
        assert_eq!(TypeOps::get_type(&value), "string");
    }

    #[test]
    fn test_version_ops_from_string() {
        let version = VersionOps::from_string("1.2.3").unwrap();
        assert_eq!(version.components, vec![1, 2, 3]);
    }

    #[test]
    fn test_version_ops_to_string() {
        let version = VersionOps::new(vec![1, 2, 3]);
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn test_version_ops_compare() {
        let v1 = VersionOps::new(vec![1, 2, 3]);
        let v2 = VersionOps::new(vec![1, 2, 4]);
        assert_eq!(v1.compare(&v2), std::cmp::Ordering::Less);
    }
}
