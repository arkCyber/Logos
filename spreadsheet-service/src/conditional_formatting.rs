//! Conditional formatting module
//! Provides rules-based cell formatting based on cell values

use serde::{Deserialize, Serialize};
use crate::error::{SpreadsheetError, SpreadsheetResult};

/// Conditional formatting rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalFormatRule {
    pub id: String,
    pub sheet_id: String,
    pub range: String, // e.g., "A1:B10"
    pub rule_type: RuleType,
    pub format: CellFormat,
    pub priority: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Rule type for conditional formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RuleType {
    /// Cell value is greater than
    GreaterThan { value: String },
    /// Cell value is less than
    LessThan { value: String },
    /// Cell value equals
    EqualTo { value: String },
    /// Cell value is between
    Between { min: String, max: String },
    /// Cell value contains text
    ContainsText { text: String },
    /// Cell value is a duplicate
    Duplicate,
    /// Cell value is unique
    Unique,
    /// Formula-based rule
    Formula { formula: String },
    /// Top N values
    TopN { n: i32 },
    /// Bottom N values
    BottomN { n: i32 },
    /// Above average
    AboveAverage,
    /// Below average
    BelowAverage,
}

/// Cell format to apply when rule matches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellFormat {
    pub background_color: Option<String>, // Hex color
    pub font_color: Option<String>, // Hex color
    pub font_bold: Option<bool>,
    pub font_italic: Option<bool>,
    pub font_underline: Option<bool>,
    pub border_style: Option<String>,
    pub border_color: Option<String>,
}

impl Default for CellFormat {
    fn default() -> Self {
        Self {
            background_color: None,
            font_color: None,
            font_bold: None,
            font_italic: None,
            font_underline: None,
            border_style: None,
            border_color: None,
        }
    }
}

/// Conditional formatting service
pub struct ConditionalFormattingService;

impl ConditionalFormattingService {
    /// Evaluate if a cell value matches a rule
    pub fn evaluate_rule(rule: &ConditionalFormatRule, cell_value: &str) -> bool {
        match &rule.rule_type {
            RuleType::GreaterThan { value } => {
                Self::compare_numbers(cell_value, value, |a, b| a > b)
            }
            RuleType::LessThan { value } => {
                Self::compare_numbers(cell_value, value, |a, b| a < b)
            }
            RuleType::EqualTo { value } => {
                Self::compare_numbers(cell_value, value, |a, b| (a - b).abs() < 0.0001)
                    || cell_value == *value
            }
            RuleType::Between { min, max } => {
                if let (Ok(num), Ok(min_val), Ok(max_val)) = (
                    cell_value.parse::<f64>(),
                    min.parse::<f64>(),
                    max.parse::<f64>(),
                ) {
                    num >= min_val && num <= max_val
                } else {
                    false
                }
            }
            RuleType::ContainsText { text } => {
                cell_value.to_lowercase().contains(&text.to_lowercase())
            }
            RuleType::Duplicate => {
                // This requires context of other cells - handled at higher level
                false
            }
            RuleType::Unique => {
                // This requires context of other cells - handled at higher level
                false
            }
            RuleType::Formula { formula: _ } => {
                // Formula evaluation handled separately
                false
            }
            RuleType::TopN { n: _ } => {
                // Requires context of all values - handled at higher level
                false
            }
            RuleType::BottomN { n: _ } => {
                // Requires context of all values - handled at higher level
                false
            }
            RuleType::AboveAverage => {
                // Requires context of all values - handled at higher level
                false
            }
            RuleType::BelowAverage => {
                // Requires context of all values - handled at higher level
                false
            }
        }
    }

    /// Compare two numeric values
    fn compare_numbers(a: &str, b: &str, comparator: impl Fn(f64, f64) -> bool) -> bool {
        if let (Ok(num_a), Ok(num_b)) = (a.parse::<f64>(), b.parse::<f64>()) {
            comparator(num_a, num_b)
        } else {
            false
        }
    }

    /// Parse cell range (e.g., "A1:B10")
    pub fn parse_range(range: &str) -> SpreadsheetResult<(i32, i32, i32, i32)> {
        if let Some((start, end)) = range.split_once(':') {
            let (start_col, start_row) = Self::parse_cell_reference(start)?;
            let (end_col, end_row) = Self::parse_cell_reference(end)?;
            Ok((start_col, start_row, end_col, end_row))
        } else {
            Err(SpreadsheetError::Validation("Invalid range format".to_string()))
        }
    }

    /// Parse cell reference (e.g., "A1" -> (0, 0))
    fn parse_cell_reference(cell_ref: &str) -> SpreadsheetResult<(i32, i32)> {
        let cell_ref = cell_ref.to_uppercase();
        let (col_part, row_part) = cell_ref.split_at(
            cell_ref.chars().take_while(|c| c.is_alphabetic()).count()
        );

        let col = col_part
            .chars()
            .fold(0i32, |acc, c| acc * 26 + (c as i32 - 'A' as i32 + 1)) - 1;

        let row = row_part
            .parse::<i32>()
            .map_err(|_| SpreadsheetError::Validation("Invalid row number".to_string()))?
            - 1;

        Ok((col, row))
    }

    /// Check if a cell is within a range
    pub fn is_cell_in_range(cell_col: i32, cell_row: i32, range: &str) -> bool {
        if let Ok((start_col, start_row, end_col, end_row)) = Self::parse_range(range) {
            cell_col >= start_col && cell_col <= end_col
                && cell_row >= start_row && cell_row <= end_row
        } else {
            false
        }
    }

    /// Convert column number to letter (0 -> A, 1 -> B, etc.)
    pub fn col_to_letter(col: i32) -> String {
        let mut col = col + 1;
        let mut result = String::new();
        
        while col > 0 {
            col -= 1;
            result.insert(0, (b'A' + (col % 26) as u8) as char);
            col /= 26;
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cell_reference() {
        assert_eq!(ConditionalFormattingService::parse_cell_reference("A1").unwrap(), (0, 0));
        assert_eq!(ConditionalFormattingService::parse_cell_reference("B2").unwrap(), (1, 1));
        assert_eq!(ConditionalFormattingService::parse_cell_reference("AA1").unwrap(), (26, 0));
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(
            ConditionalFormattingService::parse_range("A1:B10").unwrap(),
            (0, 0, 1, 9)
        );
    }

    #[test]
    fn test_is_cell_in_range() {
        assert!(ConditionalFormattingService::is_cell_in_range(0, 0, "A1:B10"));
        assert!(ConditionalFormattingService::is_cell_in_range(1, 5, "A1:B10"));
        assert!(!ConditionalFormattingService::is_cell_in_range(2, 0, "A1:B10"));
    }

    #[test]
    fn test_evaluate_rule_greater_than() {
        let rule = ConditionalFormatRule {
            id: "1".to_string(),
            sheet_id: "sheet1".to_string(),
            range: "A1:A10".to_string(),
            rule_type: RuleType::GreaterThan { value: "10".to_string() },
            format: CellFormat::default(),
            priority: 1,
            created_at: chrono::Utc::now(),
        };

        assert!(ConditionalFormattingService::evaluate_rule(&rule, "15"));
        assert!(!ConditionalFormattingService::evaluate_rule(&rule, "5"));
    }

    #[test]
    fn test_evaluate_rule_contains_text() {
        let rule = ConditionalFormatRule {
            id: "1".to_string(),
            sheet_id: "sheet1".to_string(),
            range: "A1:A10".to_string(),
            rule_type: RuleType::ContainsText { text: "error".to_string() },
            format: CellFormat::default(),
            priority: 1,
            created_at: chrono::Utc::now(),
        };

        assert!(ConditionalFormattingService::evaluate_rule(&rule, "Error occurred"));
        assert!(ConditionalFormattingService::evaluate_rule(&rule, "ERROR"));
        assert!(!ConditionalFormattingService::evaluate_rule(&rule, "Success"));
    }
}
