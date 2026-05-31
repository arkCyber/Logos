use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Sheet {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSheetRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSheetRequest {
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Cell {
    pub id: String,
    pub sheet_id: String,
    pub row: i32,
    pub col: i32,
    pub value: Option<String>,
    pub formula: Option<String>,
    pub style: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCellRequest {
    pub row: i32,
    pub col: i32,
    pub value: Option<String>,
    pub formula: Option<String>,
    pub style: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCellRequest {
    pub value: Option<String>,
    pub formula: Option<String>,
    pub style: Option<String>,
}

// Batch operation models
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchCreateCellsRequest {
    pub cells: Vec<CreateCellRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchUpdateCellsRequest {
    pub cells: Vec<BatchUpdateCellItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchUpdateCellItem {
    pub row: i32,
    pub col: i32,
    pub value: Option<String>,
    pub formula: Option<String>,
    pub style: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchDeleteCellsRequest {
    pub cells: Vec<BatchDeleteCellItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchDeleteCellItem {
    pub row: i32,
    pub col: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOperationResponse {
    pub succeeded: i32,
    pub failed: i32,
    pub errors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormulaRequest {
    pub formula: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormulaResponse {
    pub result: String,
    pub error: Option<String>,
}

// Conditional Formatting Models
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateConditionalFormatRequest {
    pub sheet_id: String,
    pub range: String,
    pub rule_type: String,
    pub rule_data: String,
    pub format_data: String,
    pub priority: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateConditionalFormatRequest {
    pub range: Option<String>,
    pub rule_type: Option<String>,
    pub rule_data: Option<String>,
    pub format_data: Option<String>,
    pub priority: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct ConditionalFormatRule {
    pub id: String,
    pub sheet_id: String,
    pub range: String,
    pub rule_type: String,
    pub rule_data: String,
    pub format_data: String,
    pub priority: i32,
    pub created_at: DateTime<Utc>,
}

// Chart Models
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChartRequest {
    pub sheet_id: String,
    pub name: String,
    pub chart_type: String,
    pub data_range: String,
    pub title: Option<String>,
    pub x_axis_title: Option<String>,
    pub y_axis_title: Option<String>,
    pub legend_position: Option<String>,
    pub style_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateChartRequest {
    pub name: Option<String>,
    pub chart_type: Option<String>,
    pub data_range: Option<String>,
    pub title: Option<String>,
    pub x_axis_title: Option<String>,
    pub y_axis_title: Option<String>,
    pub legend_position: Option<String>,
    pub style_data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Chart {
    pub id: String,
    pub sheet_id: String,
    pub name: String,
    pub chart_type: String,
    pub data_range: String,
    pub title: Option<String>,
    pub x_axis_title: Option<String>,
    pub y_axis_title: Option<String>,
    pub legend_position: Option<String>,
    pub style_data: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Pivot Table Models
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePivotTableRequest {
    pub sheet_id: String,
    pub name: String,
    pub source_range: String,
    pub row_fields: String,
    pub column_fields: String,
    pub value_fields: String,
    pub filter_fields: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePivotTableRequest {
    pub name: Option<String>,
    pub source_range: Option<String>,
    pub row_fields: Option<String>,
    pub column_fields: Option<String>,
    pub value_fields: Option<String>,
    pub filter_fields: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct PivotTable {
    pub id: String,
    pub sheet_id: String,
    pub name: String,
    pub source_range: String,
    pub row_fields: String,
    pub column_fields: String,
    pub value_fields: String,
    pub filter_fields: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
