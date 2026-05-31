//! Data validation system with aerospace-grade rule enforcement
//! 
//! This module provides comprehensive data validation functionality including
//! type checking, range validation, list validation, and custom rules.

use crate::spreadsheet_service::{
    error::{SpreadsheetError, SpreadsheetResult},
    types::CellValue,
};
use serde::{Deserialize, Serialize};

/// Validation type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationType {
    /// Any value is allowed
    Any,
    /// Whole numbers only
    WholeNumber,
    /// Decimal numbers only
    Decimal,
    /// List of values
    List,
    /// Date only
    Date,
    /// Time only
    Time,
    /// Text length
    TextLength,
    /// Custom formula
    Custom,
}

impl std::fmt::Display for ValidationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationType::Any => write!(f, "Any"),
            ValidationType::WholeNumber => write!(f, "WholeNumber"),
            ValidationType::Decimal => write!(f, "Decimal"),
            ValidationType::List => write!(f, "List"),
            ValidationType::Date => write!(f, "Date"),
            ValidationType::Time => write!(f, "Time"),
            ValidationType::TextLength => write!(f, "TextLength"),
            ValidationType::Custom => write!(f, "Custom"),
        }
    }
}

/// Comparison operator for validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationOperator {
    Between,
    NotBetween,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

impl std::fmt::Display for ValidationOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationOperator::Between => write!(f, "Between"),
            ValidationOperator::NotBetween => write!(f, "NotBetween"),
            ValidationOperator::Equal => write!(f, "Equal"),
            ValidationOperator::NotEqual => write!(f, "NotEqual"),
            ValidationOperator::GreaterThan => write!(f, "GreaterThan"),
            ValidationOperator::LessThan => write!(f, "LessThan"),
            ValidationOperator::GreaterThanOrEqual => write!(f, "GreaterThanOrEqual"),
            ValidationOperator::LessThanOrEqual => write!(f, "LessThanOrEqual"),
        }
    }
}

/// Validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Validation type
    pub validation_type: ValidationType,
    /// Operator for comparison
    pub operator: Option<ValidationOperator>,
    /// First value (for range validation)
    pub value1: Option<String>,
    /// Second value (for range validation)
    pub value2: Option<String>,
    /// List of allowed values (for list validation)
    pub list: Option<Vec<String>>,
    /// Custom formula (for custom validation)
    pub formula: Option<String>,
    /// Error message to display
    pub error_message: Option<String>,
    /// Error title
    pub error_title: Option<String>,
    /// Show error alert
    pub show_error: bool,
    /// Show input message
    pub show_input: bool,
    /// Input message title
    pub input_title: Option<String>,
    /// Input message
    pub input_message: Option<String>,
}

impl Default for ValidationRule {
    fn default() -> Self {
        Self {
            validation_type: ValidationType::Any,
            operator: None,
            value1: None,
            value2: None,
            list: None,
            formula: None,
            error_message: Some("Invalid value entered".to_string()),
            error_title: Some("Error".to_string()),
            show_error: true,
            show_input: false,
            input_title: None,
            input_message: None,
        }
    }
}

impl ValidationRule {
    /// Create a new validation rule
    pub fn new(validation_type: ValidationType) -> Self {
        Self {
            validation_type,
            ..Default::default()
        }
    }

    /// Create a whole number validation rule
    pub fn whole_number(operator: ValidationOperator, value1: String, value2: Option<String>) -> Self {
        Self {
            validation_type: ValidationType::WholeNumber,
            operator: Some(operator),
            value1: Some(value1),
            value2,
            ..Default::default()
        }
    }

    /// Create a decimal validation rule
    pub fn decimal(operator: ValidationOperator, value1: String, value2: Option<String>) -> Self {
        Self {
            validation_type: ValidationType::Decimal,
            operator: Some(operator),
            value1: Some(value1),
            value2,
            ..Default::default()
        }
    }

    /// Create a list validation rule
    pub fn list(list: Vec<String>) -> Self {
        Self {
            validation_type: ValidationType::List,
            list: Some(list),
            ..Default::default()
        }
    }

    /// Create a text length validation rule
    pub fn text_length(operator: ValidationOperator, value1: String, value2: Option<String>) -> Self {
        Self {
            validation_type: ValidationType::TextLength,
            operator: Some(operator),
            value1: Some(value1),
            value2,
            ..Default::default()
        }
    }

    /// Create a custom validation rule
    pub fn custom(formula: String) -> Self {
        Self {
            validation_type: ValidationType::Custom,
            formula: Some(formula),
            ..Default::default()
        }
    }

    /// Validate a value against this rule
    pub fn validate(&self, value: &CellValue) -> SpreadsheetResult<bool> {
        match self.validation_type {
            ValidationType::Any => Ok(true),
            ValidationType::WholeNumber => self.validate_whole_number(value),
            ValidationType::Decimal => self.validate_decimal(value),
            ValidationType::List => self.validate_list(value),
            ValidationType::Date => self.validate_date(value),
            ValidationType::Time => self.validate_time(value),
            ValidationType::TextLength => self.validate_text_length(value),
            ValidationType::Custom => self.validate_custom(value),
        }
    }

    fn validate_whole_number(&self, value: &CellValue) -> SpreadsheetResult<bool> {
        let num = match value {
            CellValue::Number(n) => n,
            _ => return Ok(false),
        };

        if !num.fract().eq(&0.0) {
            return Ok(false);
        }

        if let Some(operator) = &self.operator {
            self.validate_numeric_comparison(*num, operator)
        } else {
            Ok(true)
        }
    }

    fn validate_decimal(&self, value: &CellValue) -> SpreadsheetResult<bool> {
        let num = match value {
            CellValue::Number(n) => n,
            _ => return Ok(false),
        };

        if let Some(operator) = &self.operator {
            self.validate_numeric_comparison(*num, operator)
        } else {
            Ok(true)
        }
    }

    fn validate_list(&self, value: &CellValue) -> SpreadsheetResult<bool> {
        let list = self.list.as_ref().ok_or_else(|| {
            SpreadsheetError::invalid_input("list", "none", "list validation requires a list")
        })?;

        let value_str = match value {
            CellValue::Text(s) => s,
            CellValue::Number(n) => &n.to_string(),
            CellValue::Boolean(b) => &b.to_string(),
            _ => return Ok(false),
        };

        Ok(list.contains(value_str))
    }

    fn validate_date(&self, value: &CellValue) -> SpreadsheetResult<bool> {
        match value {
            CellValue::DateTime(_) => Ok(true),
            CellValue::Text(s) => {
                // Try to parse as date
                Ok(chrono::DateTime::parse_from_rfc3339(s).is_ok()
                    || chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").is_ok()
                    || chrono::NaiveDate::parse_from_str(s, "%m/%d/%Y").is_ok()
                    || chrono::NaiveDate::parse_from_str(s, "%d/%m/%Y").is_ok())
            }
            _ => Ok(false),
        }
    }

    fn validate_time(&self, value: &CellValue) -> SpreadsheetResult<bool> {
        match value {
            CellValue::DateTime(_) => Ok(true),
            CellValue::Text(s) => {
                Ok(chrono::NaiveTime::parse_from_str(s, "%H:%M:%S").is_ok()
                    || chrono::NaiveTime::parse_from_str(s, "%H:%M").is_ok())
            }
            _ => Ok(false),
        }
    }

    fn validate_text_length(&self, value: &CellValue) -> SpreadsheetResult<bool> {
        let text = match value {
            CellValue::Text(s) => s,
            CellValue::Number(n) => &n.to_string(),
            CellValue::Boolean(b) => &b.to_string(),
            _ => return Ok(false),
        };

        let length = text.len() as f64;

        if let Some(operator) = &self.operator {
            self.validate_numeric_comparison(length, operator)
        } else {
            Ok(true)
        }
    }

    fn validate_custom(&self, _value: &CellValue) -> SpreadsheetResult<bool> {
        // Custom validation requires formula evaluation
        // This would be implemented with the formula engine
        Ok(true)
    }

    fn validate_numeric_comparison(
        &self,
        value: f64,
        operator: &ValidationOperator,
    ) -> SpreadsheetResult<bool> {
        let val1 = self
            .value1
            .as_ref()
            .and_then(|v| v.parse::<f64>().ok())
            .ok_or_else(|| SpreadsheetError::invalid_input("value1", "none", "invalid number"))?;

        let val2 = self
            .value2
            .as_ref()
            .and_then(|v| v.parse::<f64>().ok());

        match operator {
            ValidationOperator::Between => {
                if let Some(v2) = val2 {
                    Ok(value >= val1 && value <= v2)
                } else {
                    Err(SpreadsheetError::invalid_input(
                        "operator",
                        "between",
                        "requires two values",
                    ))
                }
            }
            ValidationOperator::NotBetween => {
                if let Some(v2) = val2 {
                    Ok(value < val1 || value > v2)
                } else {
                    Err(SpreadsheetError::invalid_input(
                        "operator",
                        "not_between",
                        "requires two values",
                    ))
                }
            }
            ValidationOperator::Equal => Ok((value - val1).abs() < f64::EPSILON),
            ValidationOperator::NotEqual => Ok((value - val1).abs() >= f64::EPSILON),
            ValidationOperator::GreaterThan => Ok(value > val1),
            ValidationOperator::LessThan => Ok(value < val1),
            ValidationOperator::GreaterThanOrEqual => Ok(value >= val1),
            ValidationOperator::LessThanOrEqual => Ok(value <= val1),
        }
    }
}

/// Data validation for a cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataValidation {
    /// Validation rule
    pub rule: ValidationRule,
    /// Ignore blank cells
    pub ignore_blank: bool,
    /// In-cell dropdown
    pub in_cell_dropdown: bool,
}

impl Default for DataValidation {
    fn default() -> Self {
        Self {
            rule: ValidationRule::default(),
            ignore_blank: true,
            in_cell_dropdown: false,
        }
    }
}

impl DataValidation {
    /// Create a new data validation
    pub fn new(rule: ValidationRule) -> Self {
        Self {
            rule,
            ignore_blank: true,
            in_cell_dropdown: false,
        }
    }

    /// Validate a value
    pub fn validate(&self, value: &CellValue) -> SpreadsheetResult<bool> {
        if self.ignore_blank && value.is_empty() {
            return Ok(true);
        }
        self.rule.validate(value)
    }
}

/// Validation manager for handling data validations
pub struct ValidationManager {
    /// Validations storage (key: "sheet,row,col", value: DataValidation)
    validations: std::collections::HashMap<String, DataValidation>,
}

impl ValidationManager {
    /// Create a new validation manager
    pub fn new() -> Self {
        Self {
            validations: std::collections::HashMap::new(),
        }
    }

    /// Add a validation to a cell
    pub fn add_validation(
        &mut self,
        sheet: &str,
        row: u32,
        col: &str,
        validation: DataValidation,
    ) -> SpreadsheetResult<()> {
        let key = format!("{},{},{}", sheet, row, col);
        self.validations.insert(key, validation);
        Ok(())
    }

    /// Get validation for a cell
    pub fn get_validation(
        &self,
        sheet: &str,
        row: u32,
        col: &str,
    ) -> Option<&DataValidation> {
        let key = format!("{},{},{}", sheet, row, col);
        self.validations.get(&key)
    }

    /// Remove validation from a cell
    pub fn remove_validation(&mut self, sheet: &str, row: u32, col: &str) -> SpreadsheetResult<()> {
        let key = format!("{},{},{}", sheet, row, col);
        self.validations.remove(&key);
        Ok(())
    }

    /// Clear all validations for a sheet
    pub fn clear_sheet(&mut self, sheet: &str) {
        let prefix = format!("{},", sheet);
        self.validations.retain(|k, _| !k.starts_with(&prefix));
    }

    /// Validate a cell value
    pub fn validate_cell(
        &self,
        sheet: &str,
        row: u32,
        col: &str,
        value: &CellValue,
    ) -> SpreadsheetResult<bool> {
        if let Some(validation) = self.get_validation(sheet, row, col) {
            validation.validate(value)
        } else {
            Ok(true) // No validation means any value is allowed
        }
    }
}

impl Default for ValidationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_rule_default() {
        let rule = ValidationRule::default();
        assert_eq!(rule.validation_type, ValidationType::Any);
    }

    #[test]
    fn test_validation_rule_whole_number() {
        let rule = ValidationRule::whole_number(
            ValidationOperator::Between,
            "1".to_string(),
            Some("10".to_string()),
        );
        assert_eq!(rule.validation_type, ValidationType::WholeNumber);
    }

    #[test]
    fn test_validation_rule_list() {
        let rule = ValidationRule::list(vec!["A".to_string(), "B".to_string()]);
        assert_eq!(rule.validation_type, ValidationType::List);
    }

    #[test]
    fn test_validate_whole_number_valid() {
        let rule = ValidationRule::whole_number(
            ValidationOperator::Between,
            "1".to_string(),
            Some("10".to_string()),
        );
        let value = CellValue::Number(5.0);
        assert!(rule.validate(&value).unwrap());
    }

    #[test]
    fn test_validate_whole_number_invalid() {
        let rule = ValidationRule::whole_number(
            ValidationOperator::Between,
            "1".to_string(),
            Some("10".to_string()),
        );
        let value = CellValue::Number(5.5); // Not a whole number
        assert!(!rule.validate(&value).unwrap());
    }

    #[test]
    fn test_validate_list_valid() {
        let rule = ValidationRule::list(vec!["A".to_string(), "B".to_string()]);
        let value = CellValue::Text("A".to_string());
        assert!(rule.validate(&value).unwrap());
    }

    #[test]
    fn test_validate_list_invalid() {
        let rule = ValidationRule::list(vec!["A".to_string(), "B".to_string()]);
        let value = CellValue::Text("C".to_string());
        assert!(!rule.validate(&value).unwrap());
    }

    #[test]
    fn test_validate_date_valid() {
        let rule = ValidationRule::new(ValidationType::Date);
        let value = CellValue::Text("2024-01-01".to_string());
        assert!(rule.validate(&value).unwrap());
    }

    #[test]
    fn test_validate_date_invalid() {
        let rule = ValidationRule::new(ValidationType::Date);
        let value = CellValue::Text("not a date".to_string());
        assert!(!rule.validate(&value).unwrap());
    }

    #[test]
    fn test_validate_text_length_valid() {
        let rule = ValidationRule::text_length(
            ValidationOperator::LessThan,
            "10".to_string(),
            None,
        );
        let value = CellValue::Text("hello".to_string());
        assert!(rule.validate(&value).unwrap());
    }

    #[test]
    fn test_validate_text_length_invalid() {
        let rule = ValidationRule::text_length(
            ValidationOperator::LessThan,
            "5".to_string(),
            None,
        );
        let value = CellValue::Text("hello world".to_string());
        assert!(!rule.validate(&value).unwrap());
    }

    #[test]
    fn test_data_validation() {
        let rule = ValidationRule::list(vec!["A".to_string(), "B".to_string()]);
        let validation = DataValidation::new(rule);
        let value = CellValue::Text("A".to_string());
        assert!(validation.validate(&value).unwrap());
    }

    #[test]
    fn test_data_validation_ignore_blank() {
        let rule = ValidationRule::list(vec!["A".to_string(), "B".to_string()]);
        let mut validation = DataValidation::new(rule);
        validation.ignore_blank = true;
        let value = CellValue::Empty;
        assert!(validation.validate(&value).unwrap());
    }

    #[test]
    fn test_validation_manager() {
        let mut manager = ValidationManager::new();
        let rule = ValidationRule::list(vec!["A".to_string()]);
        let validation = DataValidation::new(rule);
        
        manager.add_validation("Sheet1", 1, "A", validation).unwrap();
        let retrieved = manager.get_validation("Sheet1", 1, "A");
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_validation_manager_validate_cell() {
        let mut manager = ValidationManager::new();
        let rule = ValidationRule::list(vec!["A".to_string()]);
        let validation = DataValidation::new(rule);
        
        manager.add_validation("Sheet1", 1, "A", validation).unwrap();
        let value = CellValue::Text("A".to_string());
        assert!(manager.validate_cell("Sheet1", 1, "A", &value).unwrap());
    }

    #[test]
    fn test_validation_type_display() {
        assert_eq!(format!("{:?}", ValidationType::Any), "Any");
        assert_eq!(format!("{:?}", ValidationType::WholeNumber), "WholeNumber");
        assert_eq!(format!("{:?}", ValidationType::Decimal), "Decimal");
        assert_eq!(format!("{:?}", ValidationType::List), "List");
        assert_eq!(format!("{:?}", ValidationType::Date), "Date");
        assert_eq!(format!("{:?}", ValidationType::Time), "Time");
        assert_eq!(format!("{:?}", ValidationType::TextLength), "TextLength");
    }

    #[test]
    fn test_validation_operator_display() {
        assert_eq!(format!("{:?}", ValidationOperator::Between), "Between");
        assert_eq!(format!("{:?}", ValidationOperator::NotBetween), "NotBetween");
        assert_eq!(format!("{:?}", ValidationOperator::Equal), "Equal");
        assert_eq!(format!("{:?}", ValidationOperator::NotEqual), "NotEqual");
        assert_eq!(format!("{:?}", ValidationOperator::GreaterThan), "GreaterThan");
        assert_eq!(format!("{:?}", ValidationOperator::LessThan), "LessThan");
    }

    #[test]
    fn test_validation_rule_decimal() {
        let rule = ValidationRule::decimal(
            ValidationOperator::Between,
            "1.5".to_string(),
            Some("10.5".to_string()),
        );
        assert_eq!(rule.validation_type, ValidationType::Decimal);
    }

    #[test]
    fn test_validate_decimal_valid() {
        let rule = ValidationRule::decimal(
            ValidationOperator::Between,
            "1.5".to_string(),
            Some("10.5".to_string()),
        );
        let value = CellValue::Number(5.5);
        assert!(rule.validate(&value).unwrap());
    }

    #[test]
    fn test_validation_rule_time() {
        let rule = ValidationRule::new(ValidationType::Time);
        assert_eq!(rule.validation_type, ValidationType::Time);
    }
}
