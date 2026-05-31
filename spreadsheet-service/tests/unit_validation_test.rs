//! Aerospace-grade unit tests for validation module
//! Tests input validation and sanitization functions

#[cfg(test)]
mod tests {
    use spreadsheet_service::validation::{
        validate_sheet_name, validate_uuid, validate_cell_coordinates,
        validate_cell_value, validate_formula, validate_cell_style,
        validate_pagination, validate_sort
    };

    #[test]
    fn test_validate_sheet_name_valid() {
        let result = validate_sheet_name("My Sheet");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "My Sheet");
    }

    #[test]
    fn test_validate_sheet_name_empty() {
        let result = validate_sheet_name("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_sheet_name_too_long() {
        let long_name = "a".repeat(300);
        let result = validate_sheet_name(&long_name);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_sheet_name_path_traversal() {
        let result = validate_sheet_name("../../../etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_uuid_valid() {
        let uuid = "550e8400-e29b-41d4-a716-446655440000";
        let result = validate_uuid(uuid);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), uuid);
    }

    #[test]
    fn test_validate_uuid_invalid() {
        let result = validate_uuid("invalid-uuid");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_cell_coordinates_valid() {
        let result = validate_cell_coordinates("A1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_cell_coordinates_invalid() {
        let result = validate_cell_coordinates("INVALID");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_cell_coordinates_out_of_range() {
        let result = validate_cell_coordinates("XFD1048577"); // Beyond Excel limit
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_cell_value_text() {
        let result = validate_cell_value(Some("Hello World"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_cell_value_number() {
        let result = validate_cell_value(Some("123.45"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_cell_value_too_long() {
        let long_value = "a".repeat(50000);
        let result = validate_cell_value(Some(&long_value));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_formula_valid() {
        let result = validate_formula(Some("=SUM(A1:A10)"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_formula_dangerous_eval() {
        let result = validate_formula(Some("=eval('malicious')"));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_formula_dangerous_system() {
        let result = validate_formula(Some("=system('rm -rf /')"));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_formula_http_url() {
        let result = validate_formula(Some("=http://evil.com"));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_cell_style_valid_json() {
        let style = r#"{"bold": true, "color": "#FF0000"}"#;
        let result = validate_cell_style(Some(style));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_cell_style_invalid_json() {
        let result = validate_cell_style(Some("not json"));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_pagination_valid() {
        let result = validate_pagination(1, 10);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_pagination_zero_page() {
        let result = validate_pagination(0, 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_pagination_negative_limit() {
        let result = validate_pagination(1, -10);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_pagination_too_large_limit() {
        let result = validate_pagination(1, 10000);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_sort_valid_field() {
        let result = validate_sort("created_at", "desc");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_sort_invalid_field() {
        let result = validate_sort("invalid_field", "asc");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_sort_invalid_direction() {
        let result = validate_sort("created_at", "invalid");
        assert!(result.is_err());
    }
}
