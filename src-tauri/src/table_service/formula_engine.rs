use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error_handling::CircuitBreaker;
use crate::config_service::ExportConfigService;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct CellReference {
    pub sheet: Option<String>,
    pub column: String,
    pub row: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FormulaResult {
    Number(f64),
    String(String),
    Boolean(bool),
    Error(String),
    Array(Vec<FormulaResult>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaError {
    pub message: String,
    pub error_type: ErrorType,
}

impl std::fmt::Display for FormulaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error_type, self.message)
    }
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::Value => write!(f, "#VALUE!"),
            ErrorType::Ref => write!(f, "#REF!"),
            ErrorType::Name => write!(f, "#NAME!"),
            ErrorType::Div0 => write!(f, "#DIV/0!"),
            ErrorType::NA => write!(f, "#N/A"),
            ErrorType::Num => write!(f, "#NUM!"),
            ErrorType::Null => write!(f, "#NULL!"),
            ErrorType::Calc => write!(f, "#CALC!"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ErrorType {
    Value,
    Ref,
    Name,
    Div0,
    NA,
    Num,
    Null,
    Calc,
}

pub struct FormulaEngine {
    config_service: Arc<ExportConfigService>,
    circuit_breaker: CircuitBreaker,
    // In production, use a proper formula engine library like calamine or similar
}

impl FormulaEngine {
    pub fn new() -> Self {
        let config_service = Arc::new(ExportConfigService::new());
        let circuit_breaker = CircuitBreaker::new(config_service.clone());
        Self {
            config_service,
            circuit_breaker,
        }
    }

    /// Evaluate a formula string
    pub fn evaluate(
        &self,
        formula: &str,
        cell_values: &HashMap<String, FormulaResult>,
    ) -> Result<FormulaResult, FormulaError> {
        // Check circuit breaker
        if !self.circuit_breaker.allow_operation() {
            return Err(FormulaError {
                message: "Circuit breaker is open, blocking formula evaluation".to_string(),
                error_type: ErrorType::Calc,
            });
        }

        if !formula.starts_with('=') {
            // It's a literal value
            let result = self.parse_literal(formula);
            if result.is_ok() {
                self.circuit_breaker.record_success();
            } else {
                self.circuit_breaker.record_failure();
            }
            return result;
        }

        let formula_expr = &formula[1..]; // Remove '='
        let result = self.evaluate_expression(formula_expr, cell_values);
        if result.is_ok() {
            self.circuit_breaker.record_success();
        } else {
            self.circuit_breaker.record_failure();
        }
        result
    }

    fn parse_literal(&self, value: &str) -> Result<FormulaResult, FormulaError> {
        let trimmed = value.trim();

        // Try to parse as number
        if let Ok(num) = trimmed.parse::<f64>() {
            return Ok(FormulaResult::Number(num));
        }

        // Try to parse as boolean
        match trimmed.to_lowercase().as_str() {
            "true" => return Ok(FormulaResult::Boolean(true)),
            "false" => return Ok(FormulaResult::Boolean(false)),
            _ => {}
        }

        // Treat as string
        Ok(FormulaResult::String(trimmed.to_string()))
    }

    fn evaluate_expression(
        &self,
        expr: &str,
        cell_values: &HashMap<String, FormulaResult>,
    ) -> Result<FormulaResult, FormulaError> {
        let expr = expr.trim();

        // Handle basic arithmetic operations
        if let Some(result) = self.try_arithmetic(expr, cell_values) {
            return result;
        }

        // Handle function calls
        if let Some(result) = self.try_function(expr, cell_values) {
            return result;
        }

        // Handle cell references
        if let Some(result) = self.try_cell_reference(expr, cell_values) {
            return result;
        }

        Err(FormulaError {
            message: format!("Unable to evaluate expression: {}", expr),
            error_type: ErrorType::Value,
        })
    }

    fn try_arithmetic(
        &self,
        expr: &str,
        cell_values: &HashMap<String, FormulaResult>,
    ) -> Option<Result<FormulaResult, FormulaError>> {
        // Simple arithmetic: try to find operators
        for op in ['+', '-', '*', '/'] {
            if let Some(idx) = expr.find(op) {
                if idx == 0 {
                    continue; // Skip if operator is at start
                }

                let left = &expr[..idx];
                let right = &expr[idx + 1..];

                let left_val = match self.evaluate_expression(left, cell_values) {
                    Ok(FormulaResult::Number(n)) => n,
                    Ok(_) => {
                        return Some(Err(FormulaError {
                            message: "Left operand must be a number".to_string(),
                            error_type: ErrorType::Value,
                        }))
                    }
                    Err(e) => return Some(Err(e)),
                };

                let right_val = match self.evaluate_expression(right, cell_values) {
                    Ok(FormulaResult::Number(n)) => n,
                    Ok(_) => {
                        return Some(Err(FormulaError {
                            message: "Right operand must be a number".to_string(),
                            error_type: ErrorType::Value,
                        }))
                    }
                    Err(e) => return Some(Err(e)),
                };

                let result = match op {
                    '+' => left_val + right_val,
                    '-' => left_val - right_val,
                    '*' => left_val * right_val,
                    '/' => {
                        if right_val == 0.0 {
                            return Some(Err(FormulaError {
                                message: "Division by zero".to_string(),
                                error_type: ErrorType::Div0,
                            }));
                        }
                        left_val / right_val
                    }
                    _ => {
                        return Some(Err(FormulaError {
                            message: format!("Unknown operator: {}", op),
                            error_type: ErrorType::Value,
                        }))
                    }
                };

                return Some(Ok(FormulaResult::Number(result)));
            }
        }

        None
    }

    fn try_function(
        &self,
        expr: &str,
        cell_values: &HashMap<String, FormulaResult>,
    ) -> Option<Result<FormulaResult, FormulaError>> {
        if !expr.contains('(') || !expr.contains(')') {
            return None;
        }

        let paren_idx = expr.find('(')?;
        let func_name = &expr[..paren_idx];
        let args_str = &expr[paren_idx + 1..expr.len() - 1];

        let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();

        match func_name.to_uppercase().as_str() {
            "SUM" => Some(self.function_sum(&args, cell_values)),
            "AVERAGE" => Some(self.function_average(&args, cell_values)),
            "MIN" => Some(self.function_min(&args, cell_values)),
            "MAX" => Some(self.function_max(&args, cell_values)),
            "COUNT" => Some(self.function_count(&args, cell_values)),
            "IF" => Some(self.function_if(&args, cell_values)),
            "CONCAT" => Some(self.function_concat(&args, cell_values)),
            _ => None,
        }
    }

    fn function_sum(
        &self,
        args: &[&str],
        cell_values: &HashMap<String, FormulaResult>,
    ) -> Result<FormulaResult, FormulaError> {
        let mut sum = 0.0;
        for arg in args {
            match self.evaluate_expression(arg, cell_values)? {
                FormulaResult::Number(n) => sum += n,
                _ => {
                    return Err(FormulaError {
                        message: "SUM requires numeric arguments".to_string(),
                        error_type: ErrorType::Value,
                    })
                }
            }
        }
        Ok(FormulaResult::Number(sum))
    }

    fn function_average(
        &self,
        args: &[&str],
        cell_values: &HashMap<String, FormulaResult>,
    ) -> Result<FormulaResult, FormulaError> {
        let sum_result = self.function_sum(args, cell_values)?;
        if let FormulaResult::Number(sum) = sum_result {
            Ok(FormulaResult::Number(sum / args.len() as f64))
        } else {
            Err(FormulaError {
                message: "AVERAGE failed".to_string(),
                error_type: ErrorType::Calc,
            })
        }
    }

    fn function_min(
        &self,
        args: &[&str],
        cell_values: &HashMap<String, FormulaResult>,
    ) -> Result<FormulaResult, FormulaError> {
        let mut min = f64::INFINITY;
        for arg in args {
            match self.evaluate_expression(arg, cell_values)? {
                FormulaResult::Number(n) => min = min.min(n),
                _ => {
                    return Err(FormulaError {
                        message: "MIN requires numeric arguments".to_string(),
                        error_type: ErrorType::Value,
                    })
                }
            }
        }
        Ok(FormulaResult::Number(min))
    }

    fn function_max(
        &self,
        args: &[&str],
        cell_values: &HashMap<String, FormulaResult>,
    ) -> Result<FormulaResult, FormulaError> {
        let mut max = f64::NEG_INFINITY;
        for arg in args {
            match self.evaluate_expression(arg, cell_values)? {
                FormulaResult::Number(n) => max = max.max(n),
                _ => {
                    return Err(FormulaError {
                        message: "MAX requires numeric arguments".to_string(),
                        error_type: ErrorType::Value,
                    })
                }
            }
        }
        Ok(FormulaResult::Number(max))
    }

    fn function_count(
        &self,
        args: &[&str],
        cell_values: &HashMap<String, FormulaResult>,
    ) -> Result<FormulaResult, FormulaError> {
        let mut count = 0;
        for arg in args {
            match self.evaluate_expression(arg, cell_values)? {
                FormulaResult::Number(_) => count += 1,
                FormulaResult::String(s) if !s.is_empty() => count += 1,
                FormulaResult::Boolean(_) => count += 1,
                _ => {}
            }
        }
        Ok(FormulaResult::Number(count as f64))
    }

    fn function_if(
        &self,
        args: &[&str],
        cell_values: &HashMap<String, FormulaResult>,
    ) -> Result<FormulaResult, FormulaError> {
        if args.len() != 3 {
            return Err(FormulaError {
                message: "IF requires exactly 3 arguments".to_string(),
                error_type: ErrorType::Value,
            });
        }

        let condition = self.evaluate_expression(args[0], cell_values)?;
        let is_true = match condition {
            FormulaResult::Boolean(b) => b,
            FormulaResult::Number(n) => n != 0.0,
            FormulaResult::String(s) => !s.is_empty(),
            _ => false,
        };

        if is_true {
            self.evaluate_expression(args[1], cell_values)
        } else {
            self.evaluate_expression(args[2], cell_values)
        }
    }

    fn function_concat(
        &self,
        args: &[&str],
        cell_values: &HashMap<String, FormulaResult>,
    ) -> Result<FormulaResult, FormulaError> {
        let mut result = String::new();
        for arg in args {
            match self.evaluate_expression(arg, cell_values)? {
                FormulaResult::String(s) => result.push_str(&s),
                FormulaResult::Number(n) => result.push_str(&n.to_string()),
                FormulaResult::Boolean(b) => result.push_str(&b.to_string()),
                _ => {}
            }
        }
        Ok(FormulaResult::String(result))
    }

    fn try_cell_reference(
        &self,
        expr: &str,
        cell_values: &HashMap<String, FormulaResult>,
    ) -> Option<Result<FormulaResult, FormulaError>> {
        // Check if it looks like a cell reference (e.g., A1, B2, Sheet1!A1)
        if is_valid_cell_reference(expr) {
            let value = cell_values.get(expr).cloned();
            Some(value.ok_or_else(|| FormulaError {
                message: format!("Cell reference not found: {}", expr),
                error_type: ErrorType::Ref,
            }))
        } else {
            None
        }
    }
}

impl Default for FormulaEngine {
    fn default() -> Self {
        Self::new()
    }
}

fn is_valid_cell_reference(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let chars: Vec<char> = s.chars().collect();
    let mut has_alpha = false;
    let mut has_digit = false;

    for c in &chars {
        if c.is_alphabetic() {
            has_alpha = true;
        } else if c.is_digit(10) {
            has_digit = true;
        } else if *c == '!' {
            // Sheet separator, valid
        } else {
            return false;
        }
    }

    has_alpha && has_digit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_literal_number() {
        let engine = FormulaEngine::new();
        let result = engine.parse_literal("42").unwrap();
        assert_eq!(result, FormulaResult::Number(42.0));
    }

    #[test]
    fn test_parse_literal_string() {
        let engine = FormulaEngine::new();
        let result = engine.parse_literal("hello").unwrap();
        assert_eq!(result, FormulaResult::String("hello".to_string()));
    }

    #[test]
    fn test_parse_literal_boolean_true() {
        let engine = FormulaEngine::new();
        let result = engine.parse_literal("true").unwrap();
        assert_eq!(result, FormulaResult::Boolean(true));
    }

    #[test]
    fn test_parse_literal_boolean_false() {
        let engine = FormulaEngine::new();
        let result = engine.parse_literal("false").unwrap();
        assert_eq!(result, FormulaResult::Boolean(false));
    }

    #[test]
    fn test_parse_literal_boolean_case_insensitive() {
        let engine = FormulaEngine::new();
        let result = engine.parse_literal("TRUE").unwrap();
        assert_eq!(result, FormulaResult::Boolean(true));
    }

    #[test]
    fn test_arithmetic() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Number(10.0));
        values.insert("B1".to_string(), FormulaResult::Number(5.0));

        let result = engine.evaluate("=A1+B1", &values).unwrap();
        assert_eq!(result, FormulaResult::Number(15.0));
    }

    #[test]
    fn test_arithmetic_subtraction() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Number(10.0));
        values.insert("B1".to_string(), FormulaResult::Number(3.0));

        let result = engine.evaluate("=A1-B1", &values).unwrap();
        assert_eq!(result, FormulaResult::Number(7.0));
    }

    #[test]
    fn test_arithmetic_multiplication() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Number(4.0));
        values.insert("B1".to_string(), FormulaResult::Number(5.0));

        let result = engine.evaluate("=A1*B1", &values).unwrap();
        assert_eq!(result, FormulaResult::Number(20.0));
    }

    #[test]
    fn test_arithmetic_division() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Number(20.0));
        values.insert("B1".to_string(), FormulaResult::Number(4.0));

        let result = engine.evaluate("=A1/B1", &values).unwrap();
        assert_eq!(result, FormulaResult::Number(5.0));
    }

    #[test]
    fn test_arithmetic_division_by_zero() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Number(10.0));
        values.insert("B1".to_string(), FormulaResult::Number(0.0));

        let result = engine.evaluate("=A1/B1", &values);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.error_type, ErrorType::Div0);
        }
    }

    #[test]
    fn test_function_sum() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Number(10.0));
        values.insert("B1".to_string(), FormulaResult::Number(20.0));

        let result = engine.evaluate("=SUM(A1,B1)", &values).unwrap();
        assert_eq!(result, FormulaResult::Number(30.0));
    }

    #[test]
    fn test_function_average() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Number(10.0));
        values.insert("B1".to_string(), FormulaResult::Number(20.0));

        let result = engine.evaluate("=AVERAGE(A1,B1)", &values).unwrap();
        assert_eq!(result, FormulaResult::Number(15.0));
    }

    #[test]
    fn test_function_min() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Number(10.0));
        values.insert("B1".to_string(), FormulaResult::Number(5.0));

        let result = engine.evaluate("=MIN(A1,B1)", &values).unwrap();
        assert_eq!(result, FormulaResult::Number(5.0));
    }

    #[test]
    fn test_function_max() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Number(10.0));
        values.insert("B1".to_string(), FormulaResult::Number(20.0));

        let result = engine.evaluate("=MAX(A1,B1)", &values).unwrap();
        assert_eq!(result, FormulaResult::Number(20.0));
    }

    #[test]
    fn test_function_count() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Number(10.0));
        values.insert("B1".to_string(), FormulaResult::String("test".to_string()));
        values.insert("C1".to_string(), FormulaResult::Boolean(true));

        let result = engine.evaluate("=COUNT(A1,B1,C1)", &values).unwrap();
        assert_eq!(result, FormulaResult::Number(3.0));
    }

    #[test]
    fn test_function_if_true() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Boolean(true));
        values.insert("B1".to_string(), FormulaResult::Number(10.0));
        values.insert("C1".to_string(), FormulaResult::Number(20.0));

        let result = engine.evaluate("=IF(A1,B1,C1)", &values).unwrap();
        assert_eq!(result, FormulaResult::Number(10.0));
    }

    #[test]
    fn test_function_if_false() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Boolean(false));
        values.insert("B1".to_string(), FormulaResult::Number(10.0));
        values.insert("C1".to_string(), FormulaResult::Number(20.0));

        let result = engine.evaluate("=IF(A1,B1,C1)", &values).unwrap();
        assert_eq!(result, FormulaResult::Number(20.0));
    }

    #[test]
    fn test_function_if_with_number_condition() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Number(5.0));
        values.insert("B1".to_string(), FormulaResult::Number(10.0));
        values.insert("C1".to_string(), FormulaResult::Number(20.0));

        let result = engine.evaluate("=IF(A1,B1,C1)", &values).unwrap();
        assert_eq!(result, FormulaResult::Number(10.0));
    }

    #[test]
    fn test_function_concat() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::String("Hello".to_string()));
        values.insert("B1".to_string(), FormulaResult::String("World".to_string()));

        let result = engine.evaluate("=CONCAT(A1,B1)", &values).unwrap();
        assert_eq!(result, FormulaResult::String("HelloWorld".to_string()));
    }

    #[test]
    fn test_function_concat_mixed_types() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert(
            "A1".to_string(),
            FormulaResult::String("Value: ".to_string()),
        );
        values.insert("B1".to_string(), FormulaResult::Number(42.0));

        let result = engine.evaluate("=CONCAT(A1,B1)", &values).unwrap();
        assert_eq!(result, FormulaResult::String("Value: 42".to_string()));
    }

    #[test]
    fn test_cell_reference() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Number(42.0));

        let result = engine.evaluate("=A1", &values).unwrap();
        assert_eq!(result, FormulaResult::Number(42.0));
    }

    #[test]
    fn test_cell_reference_not_found() {
        let engine = FormulaEngine::new();
        let values = HashMap::new();

        let result = engine.evaluate("=A1", &values);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.error_type, ErrorType::Ref);
        }
    }

    #[test]
    fn test_evaluate_without_equals() {
        let engine = FormulaEngine::new();
        let values = HashMap::new();

        let result = engine.evaluate("42", &values).unwrap();
        assert_eq!(result, FormulaResult::Number(42.0));
    }

    #[test]
    fn test_evaluate_string_literal() {
        let engine = FormulaEngine::new();
        let values = HashMap::new();

        let result = engine.evaluate("hello", &values).unwrap();
        assert_eq!(result, FormulaResult::String("hello".to_string()));
    }

    #[test]
    fn test_formula_result_number() {
        let result = FormulaResult::Number(42.0);
        assert_eq!(result, FormulaResult::Number(42.0));
    }

    #[test]
    fn test_formula_result_string() {
        let result = FormulaResult::String("test".to_string());
        assert_eq!(result, FormulaResult::String("test".to_string()));
    }

    #[test]
    fn test_formula_result_boolean() {
        let result = FormulaResult::Boolean(true);
        assert_eq!(result, FormulaResult::Boolean(true));
    }

    #[test]
    fn test_formula_result_error() {
        let result = FormulaResult::Error("test error".to_string());
        assert_eq!(result, FormulaResult::Error("test error".to_string()));
    }

    #[test]
    fn test_formula_result_array() {
        let result =
            FormulaResult::Array(vec![FormulaResult::Number(1.0), FormulaResult::Number(2.0)]);
        assert!(matches!(result, FormulaResult::Array(_)));
    }

    #[test]
    fn test_formula_error_creation() {
        let error = FormulaError {
            message: "Test error".to_string(),
            error_type: ErrorType::Value,
        };
        assert_eq!(error.message, "Test error");
        assert_eq!(error.error_type, ErrorType::Value);
    }

    #[test]
    fn test_error_type_variants() {
        let value = ErrorType::Value;
        let ref_err = ErrorType::Ref;
        let name = ErrorType::Name;
        let div0 = ErrorType::Div0;
        let na = ErrorType::NA;
        let num = ErrorType::Num;
        let null = ErrorType::Null;
        let calc = ErrorType::Calc;

        assert!(matches!(value, ErrorType::Value));
        assert!(matches!(ref_err, ErrorType::Ref));
        assert!(matches!(name, ErrorType::Name));
        assert!(matches!(div0, ErrorType::Div0));
        assert!(matches!(na, ErrorType::NA));
        assert!(matches!(num, ErrorType::Num));
        assert!(matches!(null, ErrorType::Null));
        assert!(matches!(calc, ErrorType::Calc));
    }

    #[test]
    fn test_error_type_display() {
        assert_eq!(format!("{}", ErrorType::Value), "#VALUE!");
        assert_eq!(format!("{}", ErrorType::Ref), "#REF!");
        assert_eq!(format!("{}", ErrorType::Name), "#NAME!");
        assert_eq!(format!("{}", ErrorType::Div0), "#DIV/0!");
        assert_eq!(format!("{}", ErrorType::NA), "#N/A");
        assert_eq!(format!("{}", ErrorType::Num), "#NUM!");
        assert_eq!(format!("{}", ErrorType::Null), "#NULL!");
        assert_eq!(format!("{}", ErrorType::Calc), "#CALC!");
    }

    #[test]
    fn test_formula_error_display() {
        let error = FormulaError {
            message: "Test error".to_string(),
            error_type: ErrorType::Value,
        };
        let display = format!("{}", error);
        assert!(display.contains("#VALUE!"));
        assert!(display.contains("Test error"));
    }

    #[test]
    fn test_cell_reference_creation() {
        let cell_ref = CellReference {
            sheet: Some("Sheet1".to_string()),
            column: "A".to_string(),
            row: 1,
        };
        assert_eq!(cell_ref.sheet, Some("Sheet1".to_string()));
        assert_eq!(cell_ref.column, "A");
        assert_eq!(cell_ref.row, 1);
    }

    #[test]
    fn test_cell_reference_without_sheet() {
        let cell_ref = CellReference {
            sheet: None,
            column: "B".to_string(),
            row: 2,
        };
        assert!(cell_ref.sheet.is_none());
        assert_eq!(cell_ref.column, "B");
        assert_eq!(cell_ref.row, 2);
    }

    #[test]
    fn test_is_valid_cell_reference() {
        assert!(is_valid_cell_reference("A1"));
        assert!(is_valid_cell_reference("B2"));
        assert!(is_valid_cell_reference("Sheet1!A1"));
        assert!(is_valid_cell_reference("Z999"));
        assert!(!is_valid_cell_reference(""));
        assert!(!is_valid_cell_reference("123"));
        assert!(!is_valid_cell_reference("ABC"));
        assert!(!is_valid_cell_reference("A"));
    }

    #[test]
    fn test_engine_creation() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Number(1.0));
        values.insert("B1".to_string(), FormulaResult::Number(1.0));
        let result = engine.evaluate("=A1+B1", &values);
        assert!(result.is_ok());
    }

    #[test]
    fn test_engine_default() {
        let engine = FormulaEngine::default();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Number(1.0));
        values.insert("B1".to_string(), FormulaResult::Number(1.0));
        let result = engine.evaluate("=A1+B1", &values);
        assert!(result.is_ok());
    }

    #[test]
    fn test_function_if_wrong_arg_count() {
        let engine = FormulaEngine::new();
        let values = HashMap::new();

        let result = engine.evaluate("=IF(A1,B1)", &values);
        assert!(result.is_err());
    }

    #[test]
    fn test_function_sum_with_non_numeric() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert(
            "A1".to_string(),
            FormulaResult::String("not a number".to_string()),
        );

        let result = engine.evaluate("=SUM(A1)", &values);
        assert!(result.is_err());
    }

    #[test]
    fn test_arithmetic_with_non_numeric_left() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::String("text".to_string()));
        values.insert("B1".to_string(), FormulaResult::Number(5.0));

        let result = engine.evaluate("=A1+B1", &values);
        assert!(result.is_err());
    }

    #[test]
    fn test_formula_result_serialization() {
        let result = FormulaResult::Number(42.0);
        let json = serde_json::to_string(&result);
        assert!(json.is_ok());
    }

    #[test]
    fn test_formula_error_serialization() {
        let error = FormulaError {
            message: "Test error".to_string(),
            error_type: ErrorType::Value,
        };
        let json = serde_json::to_string(&error);
        assert!(json.is_ok());
    }

    #[test]
    fn test_error_type_serialization() {
        let error_type = ErrorType::Value;
        let json = serde_json::to_string(&error_type);
        assert!(json.is_ok());
    }

    #[test]
    fn test_cell_reference_serialization() {
        let cell_ref = CellReference {
            sheet: Some("Sheet1".to_string()),
            column: "A".to_string(),
            row: 1,
        };
        let json = serde_json::to_string(&cell_ref);
        assert!(json.is_ok());
    }

    #[test]
    fn test_function_sum_single_arg() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Number(42.0));

        let result = engine.evaluate("=SUM(A1)", &values).unwrap();
        assert_eq!(result, FormulaResult::Number(42.0));
    }

    #[test]
    fn test_function_count_empty_string() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::String("".to_string()));
        values.insert("B1".to_string(), FormulaResult::Number(10.0));

        let result = engine.evaluate("=COUNT(A1,B1)", &values).unwrap();
        assert_eq!(result, FormulaResult::Number(1.0));
    }

    #[test]
    fn test_function_count_error() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Error("test".to_string()));
        values.insert("B1".to_string(), FormulaResult::Number(10.0));

        let result = engine.evaluate("=COUNT(A1,B1)", &values).unwrap();
        assert_eq!(result, FormulaResult::Number(1.0));
    }

    #[test]
    fn test_function_concat_with_boolean() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert(
            "A1".to_string(),
            FormulaResult::String("Value: ".to_string()),
        );
        values.insert("B1".to_string(), FormulaResult::Boolean(true));

        let result = engine.evaluate("=CONCAT(A1,B1)", &values).unwrap();
        assert_eq!(result, FormulaResult::String("Value: true".to_string()));
    }

    #[test]
    fn test_nested_arithmetic() {
        let engine = FormulaEngine::new();
        let mut values = HashMap::new();
        values.insert("A1".to_string(), FormulaResult::Number(2.0));
        values.insert("B1".to_string(), FormulaResult::Number(3.0));
        values.insert("C1".to_string(), FormulaResult::Number(4.0));

        let result = engine.evaluate("=A1+B1*C1", &values).unwrap();
        assert_eq!(result, FormulaResult::Number(14.0));
    }

    #[test]
    fn test_invalid_expression() {
        let engine = FormulaEngine::new();
        let values = HashMap::new();

        let result = engine.evaluate("=INVALID_FUNCTION()", &values);
        assert!(result.is_err());
    }
}
