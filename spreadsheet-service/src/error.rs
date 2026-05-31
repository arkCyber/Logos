//! Aerospace-grade error handling module
//! Provides comprehensive error types with detailed context and tracing

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::{error, warn};

/// Custom error types for the spreadsheet service
#[derive(Error, Debug)]
pub enum SpreadsheetError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Database connection error: {0}")]
    DatabaseConnection(String),

    #[error("Database query error: {0}")]
    DatabaseQuery(String),

    #[error("Database transaction error: {0}")]
    DatabaseTransaction(String),

    #[error("Resource not found: {resource} with id: {id}")]
    NotFound { resource: String, id: String },

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Invalid input: {field} - {reason}")]
    InvalidInput { field: String, reason: String },

    #[error("Formula calculation error: {0}")]
    FormulaCalculation(String),

    #[error("Excel import error: {0}")]
    ExcelImport(String),

    #[error("Excel export error: {0}")]
    ExcelExport(String),

    #[error("File operation error: {0}")]
    FileOperation(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Configuration error: {0}")]
    Configuration(String),
}

impl SpreadsheetError {
    /// Convert error to HTTP status code with appropriate logging
    pub fn to_status_code(&self) -> StatusCode {
        match self {
            SpreadsheetError::NotFound { .. } => StatusCode::NOT_FOUND,
            SpreadsheetError::Validation(_) | SpreadsheetError::InvalidInput { .. } => {
                StatusCode::BAD_REQUEST
            }
            SpreadsheetError::Authentication(_) => StatusCode::UNAUTHORIZED,
            SpreadsheetError::Authorization(_) => StatusCode::FORBIDDEN,
            SpreadsheetError::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
            SpreadsheetError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Get error category for monitoring and alerting
    pub fn error_category(&self) -> &'static str {
        match self {
            SpreadsheetError::Database(_) | SpreadsheetError::DatabaseConnection(_) => {
                "database"
            }
            SpreadsheetError::NotFound { .. } => "not_found",
            SpreadsheetError::Validation(_) | SpreadsheetError::InvalidInput { .. } => "validation",
            SpreadsheetError::FormulaCalculation(_) => "formula",
            SpreadsheetError::ExcelImport(_) | SpreadsheetError::ExcelExport(_) => "excel",
            SpreadsheetError::FileOperation(_) => "file",
            SpreadsheetError::Authentication(_) => "authentication",
            SpreadsheetError::Authorization(_) => "authorization",
            SpreadsheetError::RateLimitExceeded => "rate_limit",
            SpreadsheetError::ServiceUnavailable(_) => "service",
            SpreadsheetError::Internal(_) => "internal",
            SpreadsheetError::Configuration(_) => "configuration",
            _ => "unknown",
        }
    }

    /// Log error with appropriate level and context
    pub fn log(&self) {
        match self {
            SpreadsheetError::NotFound { resource, id } => {
                warn!(
                    resource = %resource,
                    id = %id,
                    error_category = %self.error_category(),
                    "Resource not found"
                );
            }
            SpreadsheetError::Validation(msg) => {
                warn!(
                    error = %msg,
                    error_category = %self.error_category(),
                    "Validation error"
                );
            }
            SpreadsheetError::InvalidInput { field, reason } => {
                warn!(
                    field = %field,
                    reason = %reason,
                    error_category = %self.error_category(),
                    "Invalid input"
                );
            }
            SpreadsheetError::RateLimitExceeded => {
                warn!(
                    error_category = %self.error_category(),
                    "Rate limit exceeded"
                );
            }
            _ => {
                error!(
                    error = %self,
                    error_category = %self.error_category(),
                    status_code = %self.to_status_code().as_u16(),
                    "Service error"
                );
            }
        }
    }
}

impl IntoResponse for SpreadsheetError {
    fn into_response(self) -> Response {
        self.log();

        let status = self.to_status_code();
        let error_response = json!({
            "error": {
                "message": self.to_string(),
                "category": self.error_category(),
                "status": status.as_u16(),
            }
        });

        (status, Json(error_response)).into_response()
    }
}

/// Result type alias for spreadsheet operations
pub type SpreadsheetResult<T> = Result<T, SpreadsheetError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_to_status_code() {
        let err = SpreadsheetError::NotFound {
            resource: "sheet".to_string(),
            id: "123".to_string(),
        };
        assert_eq!(err.to_status_code(), StatusCode::NOT_FOUND);

        let err = SpreadsheetError::Validation("test".to_string());
        assert_eq!(err.to_status_code(), StatusCode::BAD_REQUEST);

        let err = SpreadsheetError::Authentication("test".to_string());
        assert_eq!(err.to_status_code(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_error_category() {
        let err = SpreadsheetError::NotFound {
            resource: "sheet".to_string(),
            id: "123".to_string(),
        };
        assert_eq!(err.error_category(), "not_found");

        let err = SpreadsheetError::Database(sqlx::Error::RowNotFound);
        assert_eq!(err.error_category(), "database");
    }
}
