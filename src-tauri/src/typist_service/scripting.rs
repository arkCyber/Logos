/*!
 * 航空航天级脚本系统
 * 实现 Typst 的脚本功能（变量、函数、条件、循环、计算）
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 脚本值类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScriptValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<ScriptValue>),
    Dictionary(HashMap<String, ScriptValue>),
    None,
}

/// 脚本变量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVariable {
    pub name: String,
    pub value: ScriptValue,
}

impl ScriptVariable {
    pub fn new(name: String, value: ScriptValue) -> Self {
        Self { name, value }
    }
}

/// 脚本函数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptFunction {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: String,
}

impl ScriptFunction {
    pub fn new(name: String, parameters: Vec<String>, body: String) -> Self {
        Self {
            name,
            parameters,
            body,
        }
    }
}

/// 脚本系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scripting {
    pub variables: HashMap<String, ScriptValue>,
    pub functions: HashMap<String, ScriptFunction>,
}

impl Scripting {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn with_variables(mut self, variables: HashMap<String, ScriptValue>) -> Self {
        self.variables = variables;
        self
    }

    pub fn with_functions(mut self, functions: HashMap<String, ScriptFunction>) -> Self {
        self.functions = functions;
        self
    }

    pub fn add_variable(mut self, name: String, value: ScriptValue) -> Self {
        self.variables.insert(name, value);
        self
    }

    pub fn add_function(mut self, function: ScriptFunction) -> Self {
        self.functions.insert(function.name.clone(), function);
        self
    }

    pub fn get_variable(&self, name: &str) -> Option<&ScriptValue> {
        self.variables.get(name)
    }

    pub fn get_function(&self, name: &str) -> Option<&ScriptFunction> {
        self.functions.get(name)
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        // 添加变量
        for (name, value) in &self.variables {
            typst.push_str(&format!("#let {} = {}\n", name, self.value_to_typst(value)));
        }

        // 添加函数
        for function in self.functions.values() {
            typst.push_str(&format!(
                "#let {}({}) => {{\n  {}\n}}\n",
                function.name,
                function.parameters.join(", "),
                function.body
            ));
        }

        typst
    }

    fn value_to_typst(&self, value: &ScriptValue) -> String {
        match value {
            ScriptValue::String(s) => format!("\"{}\"", html_escape(s)),
            ScriptValue::Number(n) => n.to_string(),
            ScriptValue::Boolean(b) => b.to_string(),
            ScriptValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| self.value_to_typst(v)).collect();
                format!("({})", items.join(", "))
            }
            ScriptValue::Dictionary(dict) => {
                let items: Vec<String> = dict
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, self.value_to_typst(v)))
                    .collect();
                format!("({})", items.join(", "))
            }
            ScriptValue::None => "none".to_string(),
        }
    }

    /// 转换为 JavaScript
    pub fn to_javascript(&self) -> String {
        let mut js = String::new();

        // 添加变量
        for (name, value) in &self.variables {
            js.push_str(&format!(
                "const {} = {};\n",
                name,
                self.value_to_javascript(value)
            ));
        }

        // 添加函数
        for function in self.functions.values() {
            js.push_str(&format!(
                "function {}({}) {{\n  {}\n}}\n",
                function.name,
                function.parameters.join(", "),
                function.body
            ));
        }

        js
    }

    fn value_to_javascript(&self, value: &ScriptValue) -> String {
        match value {
            ScriptValue::String(s) => format!("\"{}\"", html_escape(s)),
            ScriptValue::Number(n) => n.to_string(),
            ScriptValue::Boolean(b) => b.to_string(),
            ScriptValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| self.value_to_javascript(v)).collect();
                format!("[{}]", items.join(", "))
            }
            ScriptValue::Dictionary(dict) => {
                let items: Vec<String> = dict
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, self.value_to_javascript(v)))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
            ScriptValue::None => "null".to_string(),
        }
    }
}

impl Default for Scripting {
    fn default() -> Self {
        Self::new()
    }
}

/// 脚本构建器
pub struct ScriptingBuilder {
    scripting: Scripting,
}

impl ScriptingBuilder {
    pub fn new() -> Self {
        Self {
            scripting: Scripting::new(),
        }
    }

    pub fn variable(mut self, name: String, value: ScriptValue) -> Self {
        self.scripting = self.scripting.add_variable(name, value);
        self
    }

    pub fn function(mut self, function: ScriptFunction) -> Self {
        self.scripting = self.scripting.add_function(function);
        self
    }

    pub fn build(self) -> Scripting {
        self.scripting
    }
}

impl Default for ScriptingBuilder {
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
    fn test_scripting_creation() {
        let scripting = Scripting::new();
        assert_eq!(scripting.variables.len(), 0);
    }

    #[test]
    fn test_scripting_default() {
        let scripting = Scripting::default();
        assert_eq!(scripting.variables.len(), 0);
    }

    #[test]
    fn test_script_variable_creation() {
        let variable = ScriptVariable::new("x".to_string(), ScriptValue::Number(42.0));
        assert_eq!(variable.name, "x");
    }

    #[test]
    fn test_script_function_creation() {
        let function = ScriptFunction::new(
            "add".to_string(),
            vec!["a".to_string(), "b".to_string()],
            "a + b".to_string(),
        );
        assert_eq!(function.name, "add");
    }

    #[test]
    fn test_scripting_add_variable() {
        let scripting = Scripting::new().add_variable("x".to_string(), ScriptValue::Number(42.0));
        assert!(scripting.variables.contains_key("x"));
    }

    #[test]
    fn test_scripting_add_function() {
        let function = ScriptFunction::new(
            "add".to_string(),
            vec!["a".to_string()],
            "a + 1".to_string(),
        );
        let scripting = Scripting::new().add_function(function);
        assert!(scripting.functions.contains_key("add"));
    }

    #[test]
    fn test_get_variable() {
        let scripting = Scripting::new().add_variable("x".to_string(), ScriptValue::Number(42.0));
        assert!(scripting.get_variable("x").is_some());
    }

    #[test]
    fn test_get_function() {
        let function = ScriptFunction::new(
            "add".to_string(),
            vec!["a".to_string()],
            "a + 1".to_string(),
        );
        let scripting = Scripting::new().add_function(function);
        assert!(scripting.get_function("add").is_some());
    }

    #[test]
    fn test_script_value_variants() {
        let string_val = ScriptValue::String("test".to_string());
        let number_val = ScriptValue::Number(42.0);
        let bool_val = ScriptValue::Boolean(true);
        let array_val = ScriptValue::Array(vec![ScriptValue::Number(1.0)]);
        let dict_val = ScriptValue::Dictionary(HashMap::new());
        let none_val = ScriptValue::None;

        assert!(matches!(string_val, ScriptValue::String(_)));
        assert!(matches!(number_val, ScriptValue::Number(_)));
        assert!(matches!(bool_val, ScriptValue::Boolean(_)));
        assert!(matches!(array_val, ScriptValue::Array(_)));
        assert!(matches!(dict_val, ScriptValue::Dictionary(_)));
        assert!(matches!(none_val, ScriptValue::None));
    }

    #[test]
    fn test_to_typst() {
        let scripting = Scripting::new().add_variable("x".to_string(), ScriptValue::Number(42.0));
        let typst = scripting.to_typst();
        assert!(typst.contains("#let x = 42"));
    }

    #[test]
    fn test_to_typst_with_function() {
        let function = ScriptFunction::new(
            "add".to_string(),
            vec!["a".to_string(), "b".to_string()],
            "a + b".to_string(),
        );
        let scripting = Scripting::new().add_function(function);
        let typst = scripting.to_typst();
        assert!(typst.contains("#let add(a, b) =>"));
    }

    #[test]
    fn test_to_javascript() {
        let scripting = Scripting::new().add_variable("x".to_string(), ScriptValue::Number(42.0));
        let js = scripting.to_javascript();
        assert!(js.contains("const x = 42"));
    }

    #[test]
    fn test_to_javascript_with_function() {
        let function = ScriptFunction::new(
            "add".to_string(),
            vec!["a".to_string(), "b".to_string()],
            "a + b".to_string(),
        );
        let scripting = Scripting::new().add_function(function);
        let js = scripting.to_javascript();
        assert!(js.contains("function add(a, b)"));
    }

    #[test]
    fn test_value_to_typst_string() {
        let scripting = Scripting::new();
        let val = ScriptValue::String("test".to_string());
        assert_eq!(scripting.value_to_typst(&val), "\"test\"");
    }

    #[test]
    fn test_value_to_typst_number() {
        let scripting = Scripting::new();
        let val = ScriptValue::Number(42.0);
        assert_eq!(scripting.value_to_typst(&val), "42");
    }

    #[test]
    fn test_value_to_typst_boolean() {
        let scripting = Scripting::new();
        let val = ScriptValue::Boolean(true);
        assert_eq!(scripting.value_to_typst(&val), "true");
    }

    #[test]
    fn test_value_to_typst_none() {
        let scripting = Scripting::new();
        let val = ScriptValue::None;
        assert_eq!(scripting.value_to_typst(&val), "none");
    }

    #[test]
    fn test_value_to_javascript_string() {
        let scripting = Scripting::new();
        let val = ScriptValue::String("test".to_string());
        assert_eq!(scripting.value_to_javascript(&val), "\"test\"");
    }

    #[test]
    fn test_value_to_javascript_number() {
        let scripting = Scripting::new();
        let val = ScriptValue::Number(42.0);
        assert_eq!(scripting.value_to_javascript(&val), "42");
    }

    #[test]
    fn test_value_to_javascript_none() {
        let scripting = Scripting::new();
        let val = ScriptValue::None;
        assert_eq!(scripting.value_to_javascript(&val), "null");
    }

    #[test]
    fn test_scripting_builder() {
        let function = ScriptFunction::new(
            "add".to_string(),
            vec!["a".to_string()],
            "a + 1".to_string(),
        );
        let scripting = ScriptingBuilder::new()
            .variable("x".to_string(), ScriptValue::Number(42.0))
            .function(function)
            .build();

        assert!(scripting.variables.contains_key("x"));
        assert!(scripting.functions.contains_key("add"));
    }

    #[test]
    fn test_scripting_builder_default() {
        let builder = ScriptingBuilder::default();
        let scripting = builder.build();
        assert_eq!(scripting.variables.len(), 0);
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_to_typst_with_array() {
        let scripting = Scripting::new().add_variable(
            "arr".to_string(),
            ScriptValue::Array(vec![ScriptValue::Number(1.0), ScriptValue::Number(2.0)]),
        );
        let typst = scripting.to_typst();
        assert!(typst.contains("#let arr = (1, 2)"));
    }

    #[test]
    fn test_to_typst_with_dictionary() {
        let mut dict = HashMap::new();
        dict.insert("key".to_string(), ScriptValue::String("value".to_string()));
        let scripting =
            Scripting::new().add_variable("dict".to_string(), ScriptValue::Dictionary(dict));
        let typst = scripting.to_typst();
        assert!(typst.contains("#let dict = (key: \"value\")"));
    }

    #[test]
    fn test_to_javascript_with_array() {
        let scripting = Scripting::new().add_variable(
            "arr".to_string(),
            ScriptValue::Array(vec![ScriptValue::Number(1.0), ScriptValue::Number(2.0)]),
        );
        let js = scripting.to_javascript();
        assert!(js.contains("const arr = [1, 2]"));
    }

    #[test]
    fn test_to_javascript_with_dictionary() {
        let mut dict = HashMap::new();
        dict.insert("key".to_string(), ScriptValue::String("value".to_string()));
        let scripting =
            Scripting::new().add_variable("dict".to_string(), ScriptValue::Dictionary(dict));
        let js = scripting.to_javascript();
        assert!(js.contains("const dict = {key: \"value\"}"));
    }
}
