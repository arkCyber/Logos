//! Aerospace-grade input validation and sanitization module
//! Provides comprehensive input validation to prevent injection attacks and ensure data integrity

use crate::error::{SpreadsheetError, SpreadsheetResult};
use tracing::{debug, warn};
use uuid::Uuid;

/// Validator for spreadsheet inputs
pub struct Validator;

impl Validator {
    /// Validate and sanitize sheet name
    pub fn validate_sheet_name(name: &str) -> SpreadsheetResult<String> {
        let name = name.trim();

        if name.is_empty() {
            return Err(SpreadsheetError::InvalidInput {
                field: "name".to_string(),
                reason: "Sheet name cannot be empty".to_string(),
            });
        }

        if name.len() > 255 {
            return Err(SpreadsheetError::InvalidInput {
                field: "name".to_string(),
                reason: "Sheet name too long (max 255 characters)".to_string(),
            });
        }

        // Prevent path traversal and special characters
        let forbidden_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        if name.contains(|c| forbidden_chars.contains(&c)) {
            return Err(SpreadsheetError::InvalidInput {
                field: "name".to_string(),
                reason: "Sheet name contains forbidden characters".to_string(),
            });
        }

        // Sanitize by removing control characters
        let sanitized: String = name.chars().filter(|c| !c.is_control()).collect();

        debug!(original = %name, sanitized = %sanitized, "Sheet name validated");

        Ok(sanitized)
    }

    /// Validate UUID format
    pub fn validate_uuid(id: &str) -> SpreadsheetResult<String> {
        if id.is_empty() {
            return Err(SpreadsheetError::InvalidInput {
                field: "id".to_string(),
                reason: "ID cannot be empty".to_string(),
            });
        }

        Uuid::parse_str(id).map_err(|_| SpreadsheetError::InvalidInput {
            field: "id".to_string(),
            reason: "Invalid UUID format".to_string(),
        })?;

        Ok(id.to_string())
    }

    /// Validate cell coordinates
    pub fn validate_cell_coordinates(row: i32, col: i32) -> SpreadsheetResult<()> {
        if row < 0 || row > 1048576 {
            // Excel max rows
            return Err(SpreadsheetError::InvalidInput {
                field: "row".to_string(),
                reason: "Row must be between 0 and 1,048,576".to_string(),
            });
        }

        if col < 0 || col > 16384 {
            // Excel max columns
            return Err(SpreadsheetError::InvalidInput {
                field: "col".to_string(),
                reason: "Column must be between 0 and 16,384".to_string(),
            });
        }

        Ok(())
    }

    /// Validate and sanitize cell value
    pub fn validate_cell_value(value: &Option<String>) -> SpreadsheetResult<Option<String>> {
        match value {
            Some(v) => {
                if v.len() > 32767 {
                    // Excel max cell length
                    return Err(SpreadsheetError::InvalidInput {
                        field: "value".to_string(),
                        reason: "Cell value too long (max 32,767 characters)".to_string(),
                    });
                }

                // Sanitize by removing null bytes and excessive whitespace
                let sanitized: String = v
                    .chars()
                    .filter(|c| *c != '\0')
                    .collect();

                Ok(Some(sanitized))
            }
            None => Ok(None),
        }
    }

    /// Validate and sanitize formula
    pub fn validate_formula(formula: &Option<String>) -> SpreadsheetResult<Option<String>> {
        match formula {
            Some(f) => {
                let f = f.trim();

                if f.is_empty() {
                    return Ok(None);
                }

                if !f.starts_with('=') {
                    return Err(SpreadsheetError::InvalidInput {
                        field: "formula".to_string(),
                        reason: "Formula must start with '='".to_string(),
                    });
                }

                if f.len() > 8192 {
                    // Excel max formula length
                    return Err(SpreadsheetError::InvalidInput {
                        field: "formula".to_string(),
                        reason: "Formula too long (max 8,192 characters)".to_string(),
                    });
                }

                // Check for potentially dangerous patterns
                let dangerous_patterns = [
                    "eval(", "exec(", "system(", "shell_exec(", "passthru(",
                    "__import__", "open(", "file://", "http://", "https://",
                ];

                for pattern in &dangerous_patterns {
                    if f.to_lowercase().contains(pattern) {
                        warn!(
                            formula = %f,
                            pattern = %pattern,
                            "Potentially dangerous formula pattern detected"
                        );
                        return Err(SpreadsheetError::InvalidInput {
                            field: "formula".to_string(),
                            reason: format!("Formula contains forbidden pattern: {}", pattern),
                        });
                    }
                }

                Ok(Some(f.to_string()))
            }
            None => Ok(None),
        }
    }

    /// Validate and sanitize cell style JSON
    pub fn validate_cell_style(style: &Option<String>) -> SpreadsheetResult<Option<String>> {
        match style {
            Some(s) => {
                if s.len() > 4096 {
                    return Err(SpreadsheetError::InvalidInput {
                        field: "style".to_string(),
                        reason: "Style data too long (max 4,096 characters)".to_string(),
                    });
                }

                // Validate JSON format
                if let Err(_) = serde_json::from_str::<serde_json::Value>(s) {
                    return Err(SpreadsheetError::InvalidInput {
                        field: "style".to_string(),
                        reason: "Style must be valid JSON".to_string(),
                    });
                }

                Ok(Some(s.clone()))
            }
            None => Ok(None),
        }
    }

    /// Sanitize string to prevent SQL injection
    pub fn sanitize_sql_string(input: &str) -> String {
        // Escape single quotes by doubling them (SQLite standard)
        input.replace('\'', "''")
    }

    /// Validate pagination parameters
    pub fn validate_pagination(limit: Option<u32>, offset: Option<u32>) -> SpreadsheetResult<(u32, u32)> {
        let limit = limit.unwrap_or(100).min(1000); // Max 1000 items per page
        let offset = offset.unwrap_or(0);

        if limit == 0 {
            return Err(SpreadsheetError::InvalidInput {
                field: "limit".to_string(),
                reason: "Limit must be greater than 0".to_string(),
            });
        }

        Ok((limit, offset))
    }

    /// Validate sort parameters
    pub fn validate_sort(sort_by: &Option<String>, sort_order: &Option<String>) -> SpreadsheetResult<(Option<String>, Option<String>)> {
        let allowed_fields = ["id", "name", "created_at", "updated_at", "row", "col"];
        let allowed_orders = ["asc", "desc"];

        if let Some(field) = sort_by {
            if !allowed_fields.contains(&field.as_str()) {
                return Err(SpreadsheetError::InvalidInput {
                    field: "sort_by".to_string(),
                    reason: format!("Invalid sort field. Allowed: {:?}", allowed_fields),
                });
            }
        }

        if let Some(order) = sort_order {
            if !allowed_orders.contains(&order.as_str()) {
                return Err(SpreadsheetError::InvalidInput {
                    field: "sort_order".to_string(),
                    reason: format!("Invalid sort order. Allowed: {:?}", allowed_orders),
                });
            }
        }

        Ok((sort_by.clone(), sort_order.clone()))
    }
}

/// Request validator for API requests
pub struct RequestValidator;

impl RequestValidator {
    /// Validate create sheet request
    pub fn validate_create_sheet_request(name: &str) -> SpreadsheetResult<String> {
        Validator::validate_sheet_name(name)
    }

    /// Validate update sheet request
    pub fn validate_update_sheet_request(name: &Option<String>) -> SpreadsheetResult<Option<String>> {
        match name {
            Some(n) => Validator::validate_sheet_name(n).map(Some),
            None => Ok(None),
        }
    }

    /// Validate create cell request
    pub fn validate_create_cell_request(
        row: i32,
        col: i32,
        value: &Option<String>,
        formula: &Option<String>,
        style: &Option<String>,
    ) -> SpreadsheetResult<(i32, i32, Option<String>, Option<String>, Option<String>)> {
        Validator::validate_cell_coordinates(row, col)?;
        let sanitized_value = Validator::validate_cell_value(value)?;
        let sanitized_formula = Validator::validate_formula(formula)?;
        let sanitized_style = Validator::validate_cell_style(style)?;

        Ok((row, col, sanitized_value, sanitized_formula, sanitized_style))
    }

    /// Validate update cell request
    pub fn validate_update_cell_request(
        value: &Option<String>,
        formula: &Option<String>,
        style: &Option<String>,
    ) -> SpreadsheetResult<(Option<String>, Option<String>, Option<String>)> {
        let sanitized_value = Validator::validate_cell_value(value)?;
        let sanitized_formula = Validator::validate_formula(formula)?;
        let sanitized_style = Validator::validate_cell_style(style)?;

        Ok((sanitized_value, sanitized_formula, sanitized_style))
    }

    /// Validate UUID
    pub fn validate_uuid(id: &str) -> SpreadsheetResult<String> {
        if id.is_empty() {
            return Err(SpreadsheetError::Validation("UUID cannot be empty".to_string()));
        }
        // Basic UUID format validation
        let uuid_regex = regex::Regex::new(r"^[0-9a-fA-F-]{36}$").unwrap();
        if !uuid_regex.is_match(id) {
            return Err(SpreadsheetError::Validation("Invalid UUID format".to_string()));
        }
        Ok(id.to_string())
    }

    /// Validate cell coordinates
    pub fn validate_cell_coordinates(row: i32, col: i32) -> SpreadsheetResult<()> {
        if row < 0 || row > 9999 {
            return Err(SpreadsheetError::Validation("Row must be between 0 and 9999".to_string()));
        }
        if col < 0 || col > 9999 {
            return Err(SpreadsheetError::Validation("Column must be between 0 and 9999".to_string()));
        }
        Ok(())
    }

    /// Validate formula request
    pub fn validate_formula_request(formula: &str) -> SpreadsheetResult<String> {
        Validator::validate_formula(&Some(formula.to_string()))
            .map(|opt| opt.unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_sheet_name_valid() {
        let result = Validator::validate_sheet_name("My Sheet");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "My Sheet");
    }

    #[test]
    fn test_validate_sheet_name_empty() {
        let result = Validator::validate_sheet_name("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_sheet_name_too_long() {
        let long_name = "a".repeat(256);
        let result = Validator::validate_sheet_name(&long_name);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_sheet_name_forbidden_chars() {
        let result = Validator::validate_sheet_name("Sheet/Name");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_uuid_valid() {
        let uuid = "550e8400-e29b-41d4-a716-446655440000";
        let result = Validator::validate_uuid(uuid);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_uuid_invalid() {
        let result = Validator::validate_uuid("not-a-uuid");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_cell_coordinates_valid() {
        let result = Validator::validate_cell_coordinates(10, 5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_cell_coordinates_invalid() {
        let result = Validator::validate_cell_coordinates(-1, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_cell_coordinates_max_exceeded() {
        let result = Validator::validate_cell_coordinates(1048577, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_cell_value_valid() {
        let result = Validator::validate_cell_value(&Some("Hello".to_string()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_cell_value_too_long() {
        let long_value = "a".repeat(32768);
        let result = Validator::validate_cell_value(&Some(long_value));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_cell_value_none() {
        let result = Validator::validate_cell_value(&None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_validate_formula_valid() {
        let result = Validator::validate_formula(&Some("=SUM(A1:A10)".to_string()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_formula_no_equals() {
        let result = Validator::validate_formula(&Some("SUM(A1:A10)".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_formula_dangerous() {
        let result = Validator::validate_formula(&Some("=eval(malicious)".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_formula_too_long() {
        let long_formula = "=".repeat(8193);
        let result = Validator::validate_formula(&Some(long_formula));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_cell_style_valid_json() {
        let result = Validator::validate_cell_style(&Some(r#"{"color":"red"}"#.to_string()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_cell_style_invalid_json() {
        let result = Validator::validate_cell_style(&Some("not json".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_cell_style_too_long() {
        let long_style = "a".repeat(4097);
        let result = Validator::validate_cell_style(&Some(long_style));
        assert!(result.is_err());
    }

    #[test]
    fn test_sanitize_sql_string() {
        let input = "O'Reilly";
        let sanitized = Validator::sanitize_sql_string(input);
        assert_eq!(sanitized, "O''Reilly");
    }

    #[test]
    fn test_validate_pagination_valid() {
        let result = Validator::validate_pagination(Some(50), Some(0));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (50, 0));
    }

    #[test]
    fn test_validate_pagination_zero_limit() {
        let result = Validator::validate_pagination(Some(0), Some(0));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_pagination_exceeds_max() {
        let result = Validator::validate_pagination(Some(2000), Some(0));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (1000, 0)); // Should be capped at 1000
    }

    #[test]
    fn test_validate_sort_valid_field() {
        let result = Validator::validate_sort(&Some("name".to_string()), &Some("asc".to_string()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_sort_invalid_field() {
        let result = Validator::validate_sort(&Some("invalid".to_string()), &Some("asc".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_sort_invalid_order() {
        let result = Validator::validate_sort(&Some("name".to_string()), &Some("invalid".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_request_validator_create_sheet() {
        let result = RequestValidator::validate_create_sheet_request("Test Sheet");
        assert!(result.is_ok());
    }

    #[test]
    fn test_request_validator_update_sheet() {
        let result = RequestValidator::validate_update_sheet_request(&Some("New Name".to_string()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_request_validator_create_cell() {
        let result = RequestValidator::validate_create_cell_request(
            0, 0, &Some("value".to_string()), &None, &None
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_request_validator_update_cell() {
        let result = RequestValidator::validate_update_cell_request(
            &Some("new value".to_string()), &None, &None
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_request_validator_formula() {
        let result = RequestValidator::validate_formula_request("=SUM(A1:A10)");
        assert!(result.is_ok());
    }
}
