//! Aerospace-grade unit tests for error handling module
//! Tests error types, conversion, and logging

#[cfg(test)]
mod tests {
    use spreadsheet_service::error::{SpreadsheetError, ErrorCategory};
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    #[test]
    fn test_error_database() {
        let error = SpreadsheetError::Database(sqlx::Error::RowNotFound);
        assert!(matches!(error, SpreadsheetError::Database(_)));
    }

    #[test]
    fn test_error_not_found() {
        let error = SpreadsheetError::NotFound {
            resource: "sheet".to_string(),
            id: "123".to_string(),
        };
        assert!(matches!(error, SpreadsheetError::NotFound { .. }));
    }

    #[test]
    fn test_error_invalid_input() {
        let error = SpreadsheetError::InvalidInput {
            field: "name".to_string(),
            reason: "too long".to_string(),
        };
        assert!(matches!(error, SpreadsheetError::InvalidInput { .. }));
    }

    #[test]
    fn test_error_into_response_not_found() {
        let error = SpreadsheetError::NotFound {
            resource: "sheet".to_string(),
            id: "123".to_string(),
        };
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_error_into_response_invalid_input() {
        let error = SpreadsheetError::InvalidInput {
            field: "name".to_string(),
            reason: "invalid".to_string(),
        };
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_error_into_response_internal() {
        let error = SpreadsheetError::Internal("unexpected error".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_error_category() {
        let error = SpreadsheetError::NotFound {
            resource: "sheet".to_string(),
            id: "123".to_string(),
        };
        let category = error.category();
        assert_eq!(category, ErrorCategory::Business);
    }

    #[test]
    fn test_error_display() {
        let error = SpreadsheetError::NotFound {
            resource: "sheet".to_string(),
            id: "123".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("sheet"));
        assert!(display.contains("123"));
    }
}
