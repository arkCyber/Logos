use sqlx::SqlitePool;
use crate::models::FormulaResponse;
use crate::error::SpreadsheetError;
use tracing::debug;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod formula_service {
    use super::*;

    /// Dependency graph for detecting circular references
    type DependencyGraph = Arc<RwLock<HashMap<String, HashSet<String>>>>;

    /// Enhanced formula calculation with aerospace-grade error handling
    pub async fn calculate(formula: &str, pool: &SqlitePool, sheet_id: &str) -> FormulaResponse {
        debug!(formula = %formula, sheet_id = %sheet_id, "Calculating formula");

        let formula = formula.trim();

        // Return literal values if not a formula
        if !formula.starts_with('=') {
            return FormulaResponse {
                result: formula.to_string(),
                error: None,
            };
        }

        let expr = &formula[1..]; // Remove '='
        let dep_graph = Arc::new(RwLock::new(HashMap::new()));

        // Try to evaluate the expression with circular reference detection
        match evaluate_expression_with_deps(expr, pool, sheet_id, "RESULT", &dep_graph).await {
            Ok(result) => FormulaResponse {
                result,
                error: None,
            },
            Err(e) => FormulaResponse {
                result: String::new(),
                error: Some(e.to_string()),
            },
        }
    }

    /// Evaluate expression with circular reference detection
    async fn evaluate_expression_with_deps(
        expr: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> Result<String, SpreadsheetError> {
        // Use a boxed future to avoid recursive async issues
        let future = evaluate_expression_with_deps_inner(expr, pool, sheet_id, current_cell, dep_graph);
        Box::pin(future).await
    }

    /// Inner evaluation function (boxed to avoid recursion)
    async fn evaluate_expression_with_deps_inner(
        expr: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> Result<String, SpreadsheetError> {
        let expr = expr.trim();

        // Handle cell references (e.g., A1, B2)
        if is_cell_reference(expr) {
            return resolve_cell_reference_with_deps(expr, pool, sheet_id, current_cell, dep_graph).await;
        }

        // Handle basic arithmetic
        if let Some(result) = evaluate_arithmetic(expr) {
            return Ok(result);
        }

        // Handle built-in functions
        if let Some(result) = evaluate_function_with_deps(expr, pool, sheet_id, current_cell, dep_graph).await {
            return Ok(result);
        }

        // Try to parse as number
        if let Ok(num) = expr.parse::<f64>() {
            return Ok(num.to_string());
        }

        // Try to parse as string literal
        if expr.starts_with('"') && expr.ends_with('"') {
            return Ok(expr[1..expr.len()-1].to_string());
        }

        Err(SpreadsheetError::FormulaCalculation(format!(
            "Unable to evaluate expression: {}",
            expr
        )))
    }

    /// Evaluate expression without dependency tracking (for internal use)
    async fn evaluate_expression(expr: &str, pool: &SqlitePool, sheet_id: &str) -> Result<String, SpreadsheetError> {
        let dep_graph = Arc::new(RwLock::new(HashMap::new()));
        evaluate_expression_with_deps(expr, pool, sheet_id, "INTERNAL", &dep_graph).await
    }

    /// Check if string is a cell reference
    fn is_cell_reference(s: &str) -> bool {
        let s = s.to_uppercase();
        let regex = regex::Regex::new(r"^[A-Z]+[0-9]+$").unwrap();
        regex.is_match(&s)
    }

    /// Resolve cell reference with circular reference detection
    async fn resolve_cell_reference_with_deps(
        cell_ref: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> Result<String, SpreadsheetError> {
        let (col, row) = parse_cell_reference(cell_ref)?;
        let cell_key = format!("{}:{}:{}", sheet_id, row, col);

        // Check for circular reference
        {
            let deps = dep_graph.read().await;
            if let Some(current_deps) = deps.get(current_cell) {
                if current_deps.contains(&cell_key) {
                    return Err(SpreadsheetError::FormulaCalculation(
                        format!("Circular reference detected: {} -> {}", current_cell, cell_ref)
                    ));
                }
            }
        }

        // Add dependency
        {
            let mut deps = dep_graph.write().await;
            deps.entry(current_cell.to_string())
                .or_insert_with(HashSet::new)
                .insert(cell_key.clone());
        }

        let cell = sqlx::query_as::<_, crate::models::Cell>(
            "SELECT id, sheet_id, row, col, value, formula, style, created_at, updated_at FROM cells WHERE sheet_id = ? AND row = ? AND col = ?"
        )
        .bind(sheet_id)
        .bind(row)
        .bind(col)
        .fetch_optional(pool)
        .await
        .map_err(|e| SpreadsheetError::FormulaCalculation(format!("Failed to resolve cell reference: {}", e)))?;

        match cell {
            Some(c) => {
                if let Some(formula) = c.formula {
                    // Recursively calculate formula
                    let expr = &formula[1..]; // Remove '='
                    evaluate_expression_with_deps(expr, pool, sheet_id, &cell_key, dep_graph).await
                } else {
                    Ok(c.value.unwrap_or_else(|| "0".to_string()))
                }
            }
            None => Ok("0".to_string()), // Return 0 for empty cells
        }
    }

    /// Parse cell reference (e.g., A1 -> col=0, row=0)
    fn parse_cell_reference(cell_ref: &str) -> Result<(i32, i32), SpreadsheetError> {
        let cell_ref = cell_ref.to_uppercase();
        let (col_part, row_part) = cell_ref.split_at(
            cell_ref.chars().take_while(|c| c.is_alphabetic()).count()
        );

        let col = col_part
            .chars()
            .fold(0i32, |acc, c| acc * 26 + (c as i32 - 'A' as i32 + 1)) - 1;

        let row = row_part
            .parse::<i32>()
            .map_err(|_| SpreadsheetError::FormulaCalculation("Invalid row number".to_string()))?
            - 1;

        Ok((col, row))
    }

    /// Evaluate basic arithmetic expressions
    fn evaluate_arithmetic(expr: &str) -> Option<String> {
        let expr = expr.replace(" ", "");

        // Handle parentheses first
        if let Some(inner) = extract_parentheses(&expr) {
            let inner_result = evaluate_arithmetic(&inner)?;
            let new_expr = expr.replace(&format!("({})", inner), &inner_result);
            return evaluate_arithmetic(&new_expr);
        }

        // Handle multiplication and division
        if let Some((left, right)) = find_operator(&expr, &['*', '/']) {
            let left_val = evaluate_arithmetic(left)?.parse::<f64>().ok()?;
            let right_val = evaluate_arithmetic(right)?.parse::<f64>().ok()?;
            let result = if expr.contains('*') {
                left_val * right_val
            } else {
                left_val / right_val
            };
            return Some(result.to_string());
        }

        // Handle addition and subtraction
        if let Some((left, right)) = find_operator(&expr, &['+', '-']) {
            let left_val = evaluate_arithmetic(left)?.parse::<f64>().ok()?;
            let right_val = evaluate_arithmetic(right)?.parse::<f64>().ok()?;
            let result = if expr.contains('+') {
                left_val + right_val
            } else {
                left_val - right_val
            };
            return Some(result.to_string());
        }

        // Try to parse as number
        expr.parse::<f64>().ok().map(|n| n.to_string())
    }

    /// Extract content from innermost parentheses
    fn extract_parentheses(expr: &str) -> Option<String> {
        let mut depth = 0;
        let mut start = 0;

        for (i, c) in expr.chars().enumerate() {
            match c {
                '(' => {
                    if depth == 0 {
                        start = i;
                    }
                    depth += 1;
                }
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(expr[start + 1..i].to_string());
                    }
                }
                _ => {}
            }
        }

        None
    }

    /// Find operator at the lowest depth
    fn find_operator<'a>(expr: &'a str, operators: &[char]) -> Option<(&'a str, &'a str)> {
        let mut depth = 0;
        let mut best_pos = None;

        for (i, c) in expr.chars().enumerate() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ if depth == 0 && operators.contains(&c) => {
                    best_pos = Some(i);
                }
                _ => {}
            }
        }

        best_pos.map(|pos| expr.split_at(pos))
    }

    /// Evaluate built-in functions with dependency tracking
    async fn evaluate_function_with_deps(
        expr: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> Option<String> {
        let expr_upper = expr.to_uppercase();

        // SUM function: SUM(A1:A10)
        if expr_upper.starts_with("SUM(") && expr_upper.ends_with(')') {
            let range = &expr[4..expr.len() - 1];
            return Some(evaluate_sum_with_deps(range, pool, sheet_id, current_cell, dep_graph).await);
        }

        // AVERAGE function: AVERAGE(A1:A10)
        if expr_upper.starts_with("AVERAGE(") && expr_upper.ends_with(')') {
            let range = &expr[8..expr.len() - 1];
            return Some(evaluate_average_with_deps(range, pool, sheet_id, current_cell, dep_graph).await);
        }

        // COUNT function: COUNT(A1:A10)
        if expr_upper.starts_with("COUNT(") && expr_upper.ends_with(')') {
            let range = &expr[6..expr.len() - 1];
            return Some(evaluate_count_with_deps(range, pool, sheet_id, current_cell, dep_graph).await);
        }

        // MAX function: MAX(A1:A10)
        if expr_upper.starts_with("MAX(") && expr_upper.ends_with(')') {
            let range = &expr[4..expr.len() - 1];
            return Some(evaluate_max_with_deps(range, pool, sheet_id, current_cell, dep_graph).await);
        }

        // MIN function: MIN(A1:A10)
        if expr_upper.starts_with("MIN(") && expr_upper.ends_with(')') {
            let range = &expr[4..expr.len() - 1];
            return Some(evaluate_min_with_deps(range, pool, sheet_id, current_cell, dep_graph).await);
        }

        // IF function: IF(condition, true_value, false_value)
        if expr_upper.starts_with("IF(") && expr_upper.ends_with(')') {
            let args = &expr[3..expr.len() - 1];
            return Some(evaluate_if(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // VLOOKUP function: VLOOKUP(lookup_value, table_range, col_index, [range_lookup])
        if expr_upper.starts_with("VLOOKUP(") && expr_upper.ends_with(')') {
            let args = &expr[8..expr.len() - 1];
            return Some(evaluate_vlookup(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // HLOOKUP function: HLOOKUP(lookup_value, table_range, row_index, [range_lookup])
        if expr_upper.starts_with("HLOOKUP(") && expr_upper.ends_with(')') {
            let args = &expr[8..expr.len() - 1];
            return Some(evaluate_hlookup(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // INDEX function: INDEX(array, row_num, [column_num])
        if expr_upper.starts_with("INDEX(") && expr_upper.ends_with(')') {
            let args = &expr[6..expr.len() - 1];
            return Some(evaluate_index(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // MATCH function: MATCH(lookup_value, lookup_array, [match_type])
        if expr_upper.starts_with("MATCH(") && expr_upper.ends_with(')') {
            let args = &expr[6..expr.len() - 1];
            return Some(evaluate_match(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // CONCAT function: CONCAT(text1, text2, ...)
        if expr_upper.starts_with("CONCAT(") && expr_upper.ends_with(')') {
            let args = &expr[7..expr.len() - 1];
            return Some(evaluate_concat(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // LEFT function: LEFT(text, [num_chars])
        if expr_upper.starts_with("LEFT(") && expr_upper.ends_with(')') {
            let args = &expr[5..expr.len() - 1];
            return Some(evaluate_left(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // RIGHT function: RIGHT(text, [num_chars])
        if expr_upper.starts_with("RIGHT(") && expr_upper.ends_with(')') {
            let args = &expr[6..expr.len() - 1];
            return Some(evaluate_right(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // MID function: MID(text, start_num, num_chars)
        if expr_upper.starts_with("MID(") && expr_upper.ends_with(')') {
            let args = &expr[4..expr.len() - 1];
            return Some(evaluate_mid(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // LEN function: LEN(text)
        if expr_upper.starts_with("LEN(") && expr_upper.ends_with(')') {
            let args = &expr[4..expr.len() - 1];
            return Some(evaluate_len(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // UPPER function: UPPER(text)
        if expr_upper.starts_with("UPPER(") && expr_upper.ends_with(')') {
            let args = &expr[6..expr.len() - 1];
            return Some(evaluate_upper(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // LOWER function: LOWER(text)
        if expr_upper.starts_with("LOWER(") && expr_upper.ends_with(')') {
            let args = &expr[6..expr.len() - 1];
            return Some(evaluate_lower(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // TRIM function: TRIM(text)
        if expr_upper.starts_with("TRIM(") && expr_upper.ends_with(')') {
            let args = &expr[5..expr.len() - 1];
            return Some(evaluate_trim(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // ROUND function: ROUND(number, num_digits)
        if expr_upper.starts_with("ROUND(") && expr_upper.ends_with(')') {
            let args = &expr[6..expr.len() - 1];
            return Some(evaluate_round(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // ABS function: ABS(number)
        if expr_upper.starts_with("ABS(") && expr_upper.ends_with(')') {
            let args = &expr[4..expr.len() - 1];
            return Some(evaluate_abs(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // POWER function: POWER(number, power)
        if expr_upper.starts_with("POWER(") && expr_upper.ends_with(')') {
            let args = &expr[6..expr.len() - 1];
            return Some(evaluate_power(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // SQRT function: SQRT(number)
        if expr_upper.starts_with("SQRT(") && expr_upper.ends_with(')') {
            let args = &expr[5..expr.len() - 1];
            return Some(evaluate_sqrt(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // MOD function: MOD(number, divisor)
        if expr_upper.starts_with("MOD(") && expr_upper.ends_with(')') {
            let args = &expr[4..expr.len() - 1];
            return Some(evaluate_mod(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // AND function: AND(logical1, logical2, ...)
        if expr_upper.starts_with("AND(") && expr_upper.ends_with(')') {
            let args = &expr[4..expr.len() - 1];
            return Some(evaluate_and(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // OR function: OR(logical1, logical2, ...)
        if expr_upper.starts_with("OR(") && expr_upper.ends_with(')') {
            let args = &expr[3..expr.len() - 1];
            return Some(evaluate_or(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // NOT function: NOT(logical)
        if expr_upper.starts_with("NOT(") && expr_upper.ends_with(')') {
            let args = &expr[4..expr.len() - 1];
            return Some(evaluate_not(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // SUMPRODUCT function: SUMPRODUCT(array1, array2, ...)
        if expr_upper.starts_with("SUMPRODUCT(") && expr_upper.ends_with(')') {
            let args = &expr[11..expr.len() - 1];
            return Some(evaluate_sumproduct(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        // TRANSPOSE function: TRANSPOSE(array)
        if expr_upper.starts_with("TRANSPOSE(") && expr_upper.ends_with(')') {
            let args = &expr[10..expr.len() - 1];
            return Some(evaluate_transpose(args, pool, sheet_id, current_cell, dep_graph).await);
        }

        None
    }

    /// Evaluate SUM function with dependency tracking
    async fn evaluate_sum_with_deps(
        range: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let values = get_range_values_with_deps(range, pool, sheet_id, current_cell, dep_graph).await;
        let sum: f64 = values.iter().filter_map(|v| v.parse::<f64>().ok()).sum();
        sum.to_string()
    }

    /// Evaluate AVERAGE function with dependency tracking
    async fn evaluate_average_with_deps(
        range: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let values = get_range_values_with_deps(range, pool, sheet_id, current_cell, dep_graph).await;
        let numeric_values: Vec<f64> = values.iter().filter_map(|v| v.parse::<f64>().ok()).collect();
        
        if numeric_values.is_empty() {
            return "0".to_string();
        }
        
        let sum: f64 = numeric_values.iter().sum();
        let avg = sum / numeric_values.len() as f64;
        avg.to_string()
    }

    /// Evaluate COUNT function with dependency tracking
    async fn evaluate_count_with_deps(
        range: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let values = get_range_values_with_deps(range, pool, sheet_id, current_cell, dep_graph).await;
        let count = values.iter().filter(|v| !v.is_empty()).count();
        count.to_string()
    }

    /// Evaluate MAX function with dependency tracking
    async fn evaluate_max_with_deps(
        range: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let values = get_range_values_with_deps(range, pool, sheet_id, current_cell, dep_graph).await;
        let numeric_values: Vec<f64> = values.iter().filter_map(|v| v.parse::<f64>().ok()).collect();
        
        numeric_values.iter().cloned().fold(f64::NAN, f64::max).to_string()
    }

    /// Evaluate MIN function with dependency tracking
    async fn evaluate_min_with_deps(
        range: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let values = get_range_values_with_deps(range, pool, sheet_id, current_cell, dep_graph).await;
        let numeric_values: Vec<f64> = values.iter().filter_map(|v| v.parse::<f64>().ok()).collect();
        
        numeric_values.iter().cloned().fold(f64::NAN, f64::min).to_string()
    }

    /// Get values from a cell range with dependency tracking
    async fn get_range_values_with_deps(
        range: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> Vec<String> {
        // Parse range (e.g., A1:B10)
        if let Some((start, end)) = range.split_once(':') {
            if let (Ok((start_col, start_row)), Ok((end_col, end_row))) = 
                (parse_cell_reference(start), parse_cell_reference(end)) {
                
                let mut values = Vec::new();
                
                for row in start_row..=end_row {
                    for col in start_col..=end_col {
                        let cell_ref = format!("{}{}", col_to_letter(col), row + 1);
                        if let Ok(value) = resolve_cell_reference_with_deps(&cell_ref, pool, sheet_id, current_cell, dep_graph).await {
                            values.push(value);
                        }
                    }
                }
                
                return values;
            }
        }
        
        // Single cell reference
        if let Ok(value) = resolve_cell_reference_with_deps(range, pool, sheet_id, current_cell, dep_graph).await {
            vec![value]
        } else {
            vec![]
        }
    }

    /// Get values from a cell range (without dependency tracking)
    async fn get_range_values(range: &str, pool: &SqlitePool, sheet_id: &str) -> Vec<String> {
        let dep_graph = Arc::new(RwLock::new(HashMap::new()));
        get_range_values_with_deps(range, pool, sheet_id, "INTERNAL", &dep_graph).await
    }

    /// Convert column number to letter (0 -> A, 1 -> B, etc.)
    fn col_to_letter(col: i32) -> String {
        let mut col = col + 1;
        let mut result = String::new();
        
        while col > 0 {
            col -= 1;
            result.insert(0, (b'A' + (col % 26) as u8) as char);
            col /= 26;
        }
        
        result
    }

    /// Parse function arguments (handles nested commas)
    fn parse_function_args(args: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut depth = 0;
        let mut in_string = false;

        for c in args.chars() {
            match c {
                '"' => {
                    in_string = !in_string;
                    current.push(c);
                }
                '(' if !in_string => {
                    depth += 1;
                    current.push(c);
                }
                ')' if !in_string => {
                    depth -= 1;
                    current.push(c);
                }
                ',' if depth == 0 && !in_string => {
                    result.push(current.trim().to_string());
                    current = String::new();
                }
                _ => current.push(c),
            }
        }
        
        if !current.trim().is_empty() {
            result.push(current.trim().to_string());
        }
        
        result
    }

    /// Evaluate IF function: IF(condition, true_value, false_value)
    async fn evaluate_if(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.len() < 2 {
            return "#ERROR".to_string();
        }

        let condition = parsed_args[0].trim();
        let true_value = parsed_args[1].trim();
        let false_value = if parsed_args.len() > 2 {
            parsed_args[2].trim()
        } else {
            ""
        };

        // Evaluate condition
        let condition_result = match evaluate_expression_with_deps(condition, pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => v,
            Err(_) => return "#ERROR".to_string(),
        };

        // Check if condition is true
        let is_true = if let Ok(num) = condition_result.parse::<f64>() {
            num != 0.0
        } else {
            !condition_result.is_empty() && condition_result.to_uppercase() != "FALSE"
        };

        if is_true {
            match evaluate_expression_with_deps(true_value, pool, sheet_id, current_cell, dep_graph).await {
                Ok(v) => v,
                Err(_) => true_value.to_string(),
            }
        } else {
            match evaluate_expression_with_deps(false_value, pool, sheet_id, current_cell, dep_graph).await {
                Ok(v) => v,
                Err(_) => false_value.to_string(),
            }
        }
    }

    /// Evaluate VLOOKUP function
    async fn evaluate_vlookup(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.len() < 3 {
            return "#ERROR".to_string();
        }

        let lookup_value = parsed_args[0].trim();
        let table_range = parsed_args[1].trim();
        let col_index = parsed_args[2].trim();
        let range_lookup = if parsed_args.len() > 3 {
            parsed_args[3].trim()
        } else {
            "TRUE"
        };

        // Evaluate lookup value
        let lookup_value = match evaluate_expression_with_deps(lookup_value, pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => v,
            Err(_) => return "#ERROR".to_string(),
        };

        // Parse column index
        let col_index: i32 = match evaluate_expression_with_deps(col_index, pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => match v.parse() {
                Ok(n) => n,
                Err(_) => return "#ERROR".to_string(),
            },
            Err(_) => return "#ERROR".to_string(),
        };

        if col_index < 1 {
            return "#ERROR".to_string();
        }

        // Parse table range
        if let Some((start, end)) = table_range.split_once(':') {
            if let (Ok((start_col, start_row)), Ok((end_col, end_row))) = 
                (parse_cell_reference(start), parse_cell_reference(end)) {
                
                // Search in first column
                for row in start_row..=end_row {
                    let cell_ref = format!("{}{}", col_to_letter(start_col), row + 1);
                    if let Ok(cell_value) = resolve_cell_reference_with_deps(&cell_ref, pool, sheet_id, current_cell, dep_graph).await {
                        let match_found = if range_lookup.to_uppercase() == "FALSE" {
                            cell_value == lookup_value
                        } else {
                            // Approximate match (for sorted data)
                            if let (Ok(cell_num), Ok(lookup_num)) = (cell_value.parse::<f64>(), lookup_value.parse::<f64>()) {
                                cell_num <= lookup_num
                            } else {
                                false
                            }
                        };

                        if match_found {
                            // Return value from col_index column
                            let target_col = start_col + col_index - 1;
                            if target_col <= end_col {
                                let target_ref = format!("{}{}", col_to_letter(target_col), row + 1);
                                if let Ok(result) = resolve_cell_reference_with_deps(&target_ref, pool, sheet_id, current_cell, dep_graph).await {
                                    return result;
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }

        "#N/A".to_string()
    }

    /// Evaluate HLOOKUP function (horizontal lookup)
    async fn evaluate_hlookup(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.len() < 3 {
            return "#ERROR".to_string();
        }

        let lookup_value = parsed_args[0].trim();
        let table_range = parsed_args[1].trim();
        let row_index = parsed_args[2].trim();
        let range_lookup = if parsed_args.len() > 3 {
            parsed_args[3].trim()
        } else {
            "TRUE"
        };

        // Parse row index
        let row_num: i32 = match evaluate_expression_with_deps(row_index, pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => match v.parse() {
                Ok(n) => n,
                Err(_) => return "#ERROR".to_string(),
            },
            Err(_) => return "#ERROR".to_string(),
        };

        if row_num < 1 {
            return "#ERROR".to_string();
        }

        // Parse table range
        if let Some((start, end)) = table_range.split_once(':') {
            if let (Ok((start_col, start_row)), Ok((end_col, end_row))) = 
                (parse_cell_reference(start), parse_cell_reference(end)) {
                
                // Search in first row
                for col in start_col..=end_col {
                    let cell_ref = format!("{}{}", col_to_letter(col), start_row + 1);
                    if let Ok(cell_value) = resolve_cell_reference_with_deps(&cell_ref, pool, sheet_id, current_cell, dep_graph).await {
                        let match_found = if range_lookup.to_uppercase() == "FALSE" {
                            cell_value == lookup_value
                        } else {
                            // Approximate match (for sorted data)
                            if let (Ok(cell_num), Ok(lookup_num)) = (cell_value.parse::<f64>(), lookup_value.parse::<f64>()) {
                                cell_num <= lookup_num
                            } else {
                                false
                            }
                        };

                        if match_found {
                            // Return value from row_num row
                            let target_row = start_row + row_num - 1;
                            if target_row <= end_row {
                                let target_ref = format!("{}{}", col_to_letter(col), target_row + 1);
                                if let Ok(result) = resolve_cell_reference_with_deps(&target_ref, pool, sheet_id, current_cell, dep_graph).await {
                                    return result;
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }

        "#N/A".to_string()
    }

    /// Evaluate INDEX function
    async fn evaluate_index(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.is_empty() {
            return "#ERROR".to_string();
        }

        let array_range = parsed_args[0].trim();
        let row_num = if parsed_args.len() > 1 {
            parsed_args[1].trim()
        } else {
            "1"
        };
        let col_num = if parsed_args.len() > 2 {
            parsed_args[2].trim()
        } else {
            "1"
        };

        // Parse row and column numbers
        let row: i32 = match evaluate_expression_with_deps(row_num, pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => match v.parse() {
                Ok(n) => n,
                Err(_) => return "#ERROR".to_string(),
            },
            Err(_) => return "#ERROR".to_string(),
        };

        let col: i32 = match evaluate_expression_with_deps(col_num, pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => match v.parse() {
                Ok(n) => n,
                Err(_) => return "#ERROR".to_string(),
            },
            Err(_) => return "#ERROR".to_string(),
        };

        if row < 1 || col < 1 {
            return "#ERROR".to_string();
        }

        // Parse array range
        if let Some((start, end)) = array_range.split_once(':') {
            if let (Ok((start_col, start_row)), Ok((end_col, end_row))) = 
                (parse_cell_reference(start), parse_cell_reference(end)) {
                
                let target_row = start_row + row - 1;
                let target_col = start_col + col - 1;

                if target_row <= end_row && target_col <= end_col {
                    let cell_ref = format!("{}{}", col_to_letter(target_col), target_row + 1);
                    if let Ok(result) = resolve_cell_reference_with_deps(&cell_ref, pool, sheet_id, current_cell, dep_graph).await {
                        return result;
                    }
                }
            }
        }

        "#REF!".to_string()
    }

    /// Evaluate MATCH function
    async fn evaluate_match(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.len() < 2 {
            return "#ERROR".to_string();
        }

        let lookup_value = parsed_args[0].trim();
        let lookup_array = parsed_args[1].trim();
        let match_type = if parsed_args.len() > 2 {
            parsed_args[2].trim()
        } else {
            "1"
        };

        // Parse match type
        let mt: i32 = match evaluate_expression_with_deps(match_type, pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => match v.parse() {
                Ok(n) => n,
                Err(_) => 1,
            },
            Err(_) => 1,
        };

        // Get values from lookup array
        let values = get_range_values_with_deps(lookup_array, pool, sheet_id, current_cell, dep_graph).await;

        // Find match
        match mt {
            0 => {
                // Exact match
                for (i, val) in values.iter().enumerate() {
                    if val == lookup_value {
                        return (i + 1).to_string();
                    }
                }
            }
            1 => {
                // Less than or equal (for ascending sorted data)
                let mut best_match = None;
                let lookup_num = lookup_value.parse::<f64>().ok();
                
                for (i, val) in values.iter().enumerate() {
                    if let Ok(val_num) = val.parse::<f64>() {
                        if let Some(ln) = lookup_num {
                            if val_num <= ln {
                                best_match = Some(i);
                            }
                        } else if val == lookup_value {
                            return (i + 1).to_string();
                        }
                    } else if val == lookup_value {
                        return (i + 1).to_string();
                    }
                }
                
                if let Some(idx) = best_match {
                    return (idx + 1).to_string();
                }
            }
            -1 => {
                // Greater than or equal (for descending sorted data)
                let mut best_match = None;
                let lookup_num = lookup_value.parse::<f64>().ok();
                
                for (i, val) in values.iter().enumerate() {
                    if let Ok(val_num) = val.parse::<f64>() {
                        if let Some(ln) = lookup_num {
                            if val_num >= ln {
                                best_match = Some(i);
                            }
                        } else if val == lookup_value {
                            return (i + 1).to_string();
                        }
                    } else if val == lookup_value {
                        return (i + 1).to_string();
                    }
                }
                
                if let Some(idx) = best_match {
                    return (idx + 1).to_string();
                }
            }
            _ => {}
        }

        "#N/A".to_string()
    }

    /// Evaluate CONCAT function
    async fn evaluate_concat(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        let mut result = String::new();

        for arg in parsed_args {
            match evaluate_expression_with_deps(arg.trim(), pool, sheet_id, current_cell, dep_graph).await {
                Ok(v) => result.push_str(&v),
                Err(_) => result.push_str(arg.trim()),
            }
        }

        result
    }

    /// Evaluate LEFT function
    async fn evaluate_left(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.is_empty() {
            return "#ERROR".to_string();
        }

        let text = match evaluate_expression_with_deps(parsed_args[0].trim(), pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => v,
            Err(_) => return "#ERROR".to_string(),
        };

        let num_chars = if parsed_args.len() > 1 {
            match evaluate_expression_with_deps(parsed_args[1].trim(), pool, sheet_id, current_cell, dep_graph).await {
                Ok(v) => v.parse::<usize>().unwrap_or(1),
                Err(_) => 1,
            }
        } else {
            1
        };

        text.chars().take(num_chars).collect()
    }

    /// Evaluate RIGHT function
    async fn evaluate_right(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.is_empty() {
            return "#ERROR".to_string();
        }

        let text = match evaluate_expression_with_deps(parsed_args[0].trim(), pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => v,
            Err(_) => return "#ERROR".to_string(),
        };

        let num_chars = if parsed_args.len() > 1 {
            match evaluate_expression_with_deps(parsed_args[1].trim(), pool, sheet_id, current_cell, dep_graph).await {
                Ok(v) => v.parse::<usize>().unwrap_or(1),
                Err(_) => 1,
            }
        } else {
            1
        };

        let chars: Vec<char> = text.chars().collect();
        let start = if chars.len() > num_chars { chars.len() - num_chars } else { 0 };
        chars[start..].iter().collect()
    }

    /// Evaluate MID function
    async fn evaluate_mid(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.len() < 2 {
            return "#ERROR".to_string();
        }

        let text = match evaluate_expression_with_deps(parsed_args[0].trim(), pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => v,
            Err(_) => return "#ERROR".to_string(),
        };

        let start_num = match evaluate_expression_with_deps(parsed_args[1].trim(), pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => v.parse::<usize>().unwrap_or(1),
            Err(_) => return "#ERROR".to_string(),
        };

        let num_chars = if parsed_args.len() > 2 {
            match evaluate_expression_with_deps(parsed_args[2].trim(), pool, sheet_id, current_cell, dep_graph).await {
                Ok(v) => v.parse::<usize>().unwrap_or(1),
                Err(_) => 1,
            }
        } else {
            1
        };

        let chars: Vec<char> = text.chars().collect();
        let start = if start_num > 0 { start_num - 1 } else { 0 };
        chars.iter().skip(start).take(num_chars).collect()
    }

    /// Evaluate LEN function
    async fn evaluate_len(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.is_empty() {
            return "#ERROR".to_string();
        }

        let text = match evaluate_expression_with_deps(parsed_args[0].trim(), pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => v,
            Err(_) => return "#ERROR".to_string(),
        };

        text.chars().count().to_string()
    }

    /// Evaluate UPPER function
    async fn evaluate_upper(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.is_empty() {
            return "#ERROR".to_string();
        }

        let text = match evaluate_expression_with_deps(parsed_args[0].trim(), pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => v,
            Err(_) => return "#ERROR".to_string(),
        };

        text.to_uppercase()
    }

    /// Evaluate LOWER function
    async fn evaluate_lower(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.is_empty() {
            return "#ERROR".to_string();
        }

        let text = match evaluate_expression_with_deps(parsed_args[0].trim(), pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => v,
            Err(_) => return "#ERROR".to_string(),
        };

        text.to_lowercase()
    }

    /// Evaluate TRIM function
    async fn evaluate_trim(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.is_empty() {
            return "#ERROR".to_string();
        }

        let text = match evaluate_expression_with_deps(parsed_args[0].trim(), pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => v,
            Err(_) => return "#ERROR".to_string(),
        };

        text.trim().to_string()
    }

    /// Evaluate ROUND function
    async fn evaluate_round(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.is_empty() {
            return "#ERROR".to_string();
        }

        let number = match evaluate_expression_with_deps(parsed_args[0].trim(), pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => v.parse::<f64>().unwrap_or(0.0),
            Err(_) => return "#ERROR".to_string(),
        };

        let num_digits = if parsed_args.len() > 1 {
            match evaluate_expression_with_deps(parsed_args[1].trim(), pool, sheet_id, current_cell, dep_graph).await {
                Ok(v) => v.parse::<i32>().unwrap_or(0),
                Err(_) => 0,
            }
        } else {
            0
        };

        let multiplier = 10_f64.powi(num_digits);
        ((number * multiplier).round() / multiplier).to_string()
    }

    /// Evaluate ABS function
    async fn evaluate_abs(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.is_empty() {
            return "#ERROR".to_string();
        }

        let number = match evaluate_expression_with_deps(parsed_args[0].trim(), pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => v.parse::<f64>().unwrap_or(0.0),
            Err(_) => return "#ERROR".to_string(),
        };

        number.abs().to_string()
    }

    /// Evaluate POWER function
    async fn evaluate_power(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.len() < 2 {
            return "#ERROR".to_string();
        }

        let number = match evaluate_expression_with_deps(parsed_args[0].trim(), pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => v.parse::<f64>().unwrap_or(0.0),
            Err(_) => return "#ERROR".to_string(),
        };

        let power = match evaluate_expression_with_deps(parsed_args[1].trim(), pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => v.parse::<f64>().unwrap_or(0.0),
            Err(_) => return "#ERROR".to_string(),
        };

        number.powf(power).to_string()
    }

    /// Evaluate SQRT function
    async fn evaluate_sqrt(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.is_empty() {
            return "#ERROR".to_string();
        }

        let number = match evaluate_expression_with_deps(parsed_args[0].trim(), pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => v.parse::<f64>().unwrap_or(0.0),
            Err(_) => return "#ERROR".to_string(),
        };

        if number < 0.0 {
            return "#NUM!".to_string();
        }

        number.sqrt().to_string()
    }

    /// Evaluate MOD function
    async fn evaluate_mod(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.len() < 2 {
            return "#ERROR".to_string();
        }

        let number = match evaluate_expression_with_deps(parsed_args[0].trim(), pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => v.parse::<f64>().unwrap_or(0.0),
            Err(_) => return "#ERROR".to_string(),
        };

        let divisor = match evaluate_expression_with_deps(parsed_args[1].trim(), pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => v.parse::<f64>().unwrap_or(1.0),
            Err(_) => return "#ERROR".to_string(),
        };

        if divisor == 0.0 {
            return "#DIV/0!".to_string();
        }

        (number % divisor).to_string()
    }

    /// Evaluate AND function
    async fn evaluate_and(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.is_empty() {
            return "TRUE".to_string();
        }

        for arg in parsed_args {
            let value = match evaluate_expression_with_deps(arg.trim(), pool, sheet_id, current_cell, dep_graph).await {
                Ok(v) => v,
                Err(_) => return "FALSE".to_string(),
            };

            let is_true = if let Ok(num) = value.parse::<f64>() {
                num != 0.0
            } else {
                !value.is_empty() && value.to_uppercase() != "FALSE"
            };

            if !is_true {
                return "FALSE".to_string();
            }
        }

        "TRUE".to_string()
    }

    /// Evaluate OR function
    async fn evaluate_or(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.is_empty() {
            return "FALSE".to_string();
        }

        for arg in parsed_args {
            let value = match evaluate_expression_with_deps(arg.trim(), pool, sheet_id, current_cell, dep_graph).await {
                Ok(v) => v,
                Err(_) => continue,
            };

            let is_true = if let Ok(num) = value.parse::<f64>() {
                num != 0.0
            } else {
                !value.is_empty() && value.to_uppercase() != "FALSE"
            };

            if is_true {
                return "TRUE".to_string();
            }
        }

        "FALSE".to_string()
    }

    /// Evaluate NOT function
    async fn evaluate_not(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.is_empty() {
            return "TRUE".to_string();
        }

        let value = match evaluate_expression_with_deps(parsed_args[0].trim(), pool, sheet_id, current_cell, dep_graph).await {
            Ok(v) => v,
            Err(_) => return "TRUE".to_string(),
        };

        let is_true = if let Ok(num) = value.parse::<f64>() {
            num != 0.0
        } else {
            !value.is_empty() && value.to_uppercase() != "FALSE"
        };

        if is_true {
            "FALSE".to_string()
        } else {
            "TRUE".to_string()
        }
    }

    /// Evaluate SUMPRODUCT function (array formula)
    async fn evaluate_sumproduct(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.len() < 2 {
            return "#ERROR".to_string();
        }

        // Get values from each array/range
        let mut arrays: Vec<Vec<f64>> = Vec::new();
        for arg in parsed_args {
            let values = get_range_values_with_deps(arg.trim(), pool, sheet_id, current_cell, dep_graph).await;
            let nums: Vec<f64> = values.iter()
                .filter_map(|v| v.parse::<f64>().ok())
                .collect();
            arrays.push(nums);
        }

        // Check if all arrays have the same length
        let first_len = arrays[0].len();
        if !arrays.iter().all(|arr| arr.len() == first_len) {
            return "#ERROR".to_string();
        }

        // Calculate sum of products
        let mut sum = 0.0;
        for i in 0..first_len {
            let mut product = 1.0;
            for arr in &arrays {
                product *= arr[i];
            }
            sum += product;
        }

        sum.to_string()
    }

    /// Evaluate TRANSPOSE function (array formula)
    async fn evaluate_transpose(
        args: &str,
        pool: &SqlitePool,
        sheet_id: &str,
        current_cell: &str,
        dep_graph: &DependencyGraph,
    ) -> String {
        let parsed_args = parse_function_args(args);
        if parsed_args.is_empty() {
            return "#ERROR".to_string();
        }

        // Get values from the range
        let values = get_range_values_with_deps(parsed_args[0].trim(), pool, sheet_id, current_cell, dep_graph).await;
        
        // For simplicity, return comma-separated transposed values
        // In a full implementation, this would return a 2D array
        values.join(",")
    }
}
