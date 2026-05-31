use axum::{
    extract::{Path, State, Multipart},
    http::StatusCode,
    response::{Json, IntoResponse},
};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::*;
use crate::services::*;
use crate::error::SpreadsheetError;
use crate::validation::RequestValidator;
use crate::excel::{ExcelImporter, ExcelExporter};
use crate::config::AppConfig;
use crate::auth::{AuthService, LoginRequest, RegisterRequest, AuthResponse};

pub async fn list_sheets(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Sheet>>, SpreadsheetError> {
    let sheets = sqlx::query_as::<_, Sheet>(
        "SELECT id, name, created_at, updated_at FROM sheets ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to list sheets: {}", e)))?;

    Ok(Json(sheets))
}

pub async fn create_sheet(
    State(pool): State<SqlitePool>,
    Json(req): Json<CreateSheetRequest>,
) -> Result<Json<Sheet>, SpreadsheetError> {
    let validated_name = RequestValidator::validate_create_sheet_request(&req.name)?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    sqlx::query(
        "INSERT INTO sheets (id, name, created_at, updated_at) VALUES (?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&validated_name)
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create sheet: {}", e)))?;
    
    let sheet = Sheet {
        id: id.clone(),
        name: validated_name,
        created_at: now,
        updated_at: now,
    };
    
    Ok(Json(sheet))
}

pub async fn get_sheet(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<Sheet>, SpreadsheetError> {
    let validated_id = RequestValidator::validate_uuid(&id)?;
    let sheet = sqlx::query_as::<_, Sheet>(
        "SELECT id, name, created_at, updated_at FROM sheets WHERE id = ?"
    )
    .bind(&validated_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        if matches!(e, sqlx::Error::RowNotFound) {
            SpreadsheetError::NotFound {
                resource: "sheet".to_string(),
                id: validated_id,
            }
        } else {
            SpreadsheetError::DatabaseQuery(format!("Failed to get sheet: {}", e))
        }
    })?;
    
    Ok(Json(sheet))
}

pub async fn update_sheet(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(req): Json<UpdateSheetRequest>,
) -> Result<Json<Sheet>, SpreadsheetError> {
    let validated_id = RequestValidator::validate_uuid(&id)?;
    let validated_name = RequestValidator::validate_update_sheet_request(&req.name)?;
    let now = Utc::now();
    
    if let Some(name) = validated_name {
        sqlx::query(
            "UPDATE sheets SET name = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&name)
        .bind(&now)
        .bind(&validated_id)
        .execute(&pool)
        .await
        .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to update sheet: {}", e)))?;
    }
    
    let sheet = sqlx::query_as::<_, Sheet>(
        "SELECT id, name, created_at, updated_at FROM sheets WHERE id = ?"
    )
    .bind(&validated_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        if matches!(e, sqlx::Error::RowNotFound) {
            SpreadsheetError::NotFound {
                resource: "sheet".to_string(),
                id: validated_id,
            }
        } else {
            SpreadsheetError::DatabaseQuery(format!("Failed to get sheet: {}", e))
        }
    })?;
    
    Ok(Json(sheet))
}

pub async fn delete_sheet(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<StatusCode, SpreadsheetError> {
    let validated_id = RequestValidator::validate_uuid(&id)?;
    let result = sqlx::query("DELETE FROM sheets WHERE id = ?")
        .bind(&validated_id)
        .execute(&pool)
        .await
        .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to delete sheet: {}", e)))?;
    
    if result.rows_affected() == 0 {
        return Err(SpreadsheetError::NotFound {
            resource: "sheet".to_string(),
            id: validated_id,
        });
    }
    
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_cells(
    State(pool): State<SqlitePool>,
    Path(sheet_id): Path<String>,
) -> Result<Json<Vec<Cell>>, SpreadsheetError> {
    let cells = sqlx::query_as::<_, Cell>(
        "SELECT id, sheet_id, row, col, value, formula, style, created_at, updated_at FROM cells WHERE sheet_id = ? ORDER BY row, col"
    )
    .bind(&sheet_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to list cells: {}", e)))?;
    
    Ok(Json(cells))
}

pub async fn create_cell(
    State(pool): State<SqlitePool>,
    Path(sheet_id): Path<String>,
    Json(req): Json<CreateCellRequest>,
) -> Result<Json<Cell>, SpreadsheetError> {
    // Validate request
    let validated_sheet_id = RequestValidator::validate_uuid(&sheet_id)?;
    let _validated_request = RequestValidator::validate_create_cell_request(req.row, req.col, &req.value, &req.formula, &req.style)?;
    
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    sqlx::query(
        "INSERT INTO cells (id, sheet_id, row, col, value, formula, style, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&validated_sheet_id)
    .bind(req.row)
    .bind(req.col)
    .bind(&req.value)
    .bind(&req.formula)
    .bind(&req.style)
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create cell: {}", e)))?;
    
    let cell = Cell {
        id: id.clone(),
        sheet_id: sheet_id.clone(),
        row: req.row,
        col: req.col,
        value: req.value,
        formula: req.formula,
        style: req.style,
        created_at: now,
        updated_at: now,
    };
    
    Ok(Json(cell))
}

pub async fn get_cell(
    State(pool): State<SqlitePool>,
    Path((sheet_id, row, col)): Path<(String, i32, i32)>,
) -> Result<Json<Cell>, SpreadsheetError> {
    let cell = sqlx::query_as::<_, Cell>(
        "SELECT id, sheet_id, row, col, value, formula, style, created_at, updated_at FROM cells WHERE sheet_id = ? AND row = ? AND col = ?"
    )
    .bind(&sheet_id)
    .bind(row)
    .bind(col)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        if matches!(e, sqlx::Error::RowNotFound) {
            SpreadsheetError::NotFound {
                resource: "cell".to_string(),
                id: format!("{}:{}:{}", sheet_id, row, col),
            }
        } else {
            SpreadsheetError::DatabaseQuery(format!("Failed to get cell: {}", e))
        }
    })?;
    
    Ok(Json(cell))
}

pub async fn update_cell(
    State(pool): State<SqlitePool>,
    Path((sheet_id, row, col)): Path<(String, i32, i32)>,
    Json(req): Json<UpdateCellRequest>,
) -> Result<Json<Cell>, SpreadsheetError> {
    // Validate request
    let validated_sheet_id = RequestValidator::validate_uuid(&sheet_id)?;
    let _validated_coordinates = RequestValidator::validate_cell_coordinates(row, col)?;
    let _validated_request = RequestValidator::validate_update_cell_request(&req.value, &req.formula, &req.style)?;
    
    let now = Utc::now();
    
    sqlx::query(
        "UPDATE cells SET value = ?, formula = ?, style = ?, updated_at = ? WHERE sheet_id = ? AND row = ? AND col = ?"
    )
    .bind(&req.value)
    .bind(&req.formula)
    .bind(&req.style)
    .bind(&now)
    .bind(&validated_sheet_id)
    .bind(row)
    .bind(col)
    .execute(&pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to update cell: {}", e)))?;
    
    let cell = sqlx::query_as::<_, Cell>(
        "SELECT id, sheet_id, row, col, value, formula, style, created_at, updated_at FROM cells WHERE sheet_id = ? AND row = ? AND col = ?"
    )
    .bind(&validated_sheet_id)
    .bind(row)
    .bind(col)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        if matches!(e, sqlx::Error::RowNotFound) {
            SpreadsheetError::NotFound {
                resource: "cell".to_string(),
                id: format!("{}:{}:{}", validated_sheet_id, row, col),
            }
        } else {
            SpreadsheetError::DatabaseQuery(format!("Failed to get cell: {}", e))
        }
    })?;
    
    Ok(Json(cell))
}

pub async fn delete_cell(
    State(pool): State<SqlitePool>,
    Path((sheet_id, row, col)): Path<(String, i32, i32)>,
) -> Result<StatusCode, SpreadsheetError> {
    // Validate request
    let validated_sheet_id = RequestValidator::validate_uuid(&sheet_id)?;
    let _validated_coordinates = RequestValidator::validate_cell_coordinates(row, col)?;
    
    let result = sqlx::query("DELETE FROM cells WHERE sheet_id = ? AND row = ? AND col = ?")
        .bind(&validated_sheet_id)
        .bind(row)
        .bind(col)
        .execute(&pool)
        .await
        .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to delete cell: {}", e)))?;
    
    if result.rows_affected() == 0 {
        return Err(SpreadsheetError::NotFound {
            resource: "cell".to_string(),
            id: format!("{}:{}:{}", validated_sheet_id, row, col),
        });
    }
    
    Ok(StatusCode::NO_CONTENT)
}

pub async fn calculate_formula(
    State(pool): State<SqlitePool>,
    Path(sheet_id): Path<String>,
    Json(req): Json<FormulaRequest>,
) -> Result<Json<FormulaResponse>, SpreadsheetError> {
    let validated_sheet_id = RequestValidator::validate_uuid(&sheet_id)?;
    let validated_formula = RequestValidator::validate_formula_request(&req.formula)?;
    let result = formula_service::calculate(&validated_formula, &pool, &validated_sheet_id).await;
    
    Ok(Json(result))
}

pub async fn import_excel(
    State(pool): State<SqlitePool>,
    _multipart: Multipart,
) -> Result<Json<Sheet>, SpreadsheetError> {
    let config = AppConfig::load()?;
    let importer = ExcelImporter::new(config);
    
    // Placeholder implementation - create a sample sheet
    let sheet = importer.import_excel("placeholder.xlsx", &pool).await?;
    
    Ok(Json(sheet))
}

pub async fn export_excel(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<axum::response::Response, SpreadsheetError> {
    let config = AppConfig::load()?;
    let exporter = ExcelExporter::new(config);
    
    let excel_data = exporter.export_excel(&id, &pool).await?;
    
    let filename = format!("attachment; filename=\"{}.xlsx\"", id);
    let headers = [
        ("content-type", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
        ("content-disposition", filename.as_str()),
    ];
    
    Ok((headers, excel_data).into_response())
}

// Authentication handlers

pub async fn register(
    State(pool): State<SqlitePool>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, SpreadsheetError> {
    let config = AppConfig::load()?;
    let auth_service = AuthService::new(
        config.security.jwt_secret,
        config.security.jwt_expiration as i64,
    );
    
    let response = auth_service.register(req, &pool).await?;
    Ok(Json(response))
}

pub async fn login(
    State(pool): State<SqlitePool>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, SpreadsheetError> {
    let config = AppConfig::load()?;
    let auth_service = AuthService::new(
        config.security.jwt_secret,
        config.security.jwt_expiration as i64,
    );
    
    let response = auth_service.login(req, &pool).await?;
    Ok(Json(response))
}

// Conditional Formatting Handlers
pub async fn list_conditional_formats(
    State(pool): State<SqlitePool>,
    Path(sheet_id): Path<String>,
) -> Result<Json<Vec<ConditionalFormatRule>>, SpreadsheetError> {
    let rules = sqlx::query_as::<_, ConditionalFormatRule>(
        "SELECT id, sheet_id, range, rule_type, rule_data, format_data, priority, created_at 
         FROM conditional_formatting_rules 
         WHERE sheet_id = ? 
         ORDER BY priority ASC, created_at DESC"
    )
    .bind(&sheet_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to list conditional formats: {}", e)))?;

    Ok(Json(rules))
}

pub async fn create_conditional_format(
    State(pool): State<SqlitePool>,
    Json(req): Json<CreateConditionalFormatRequest>,
) -> Result<Json<ConditionalFormatRule>, SpreadsheetError> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let priority = req.priority.unwrap_or(0);
    
    sqlx::query(
        "INSERT INTO conditional_formatting_rules (id, sheet_id, range, rule_type, rule_data, format_data, priority, created_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&req.sheet_id)
    .bind(&req.range)
    .bind(&req.rule_type)
    .bind(&req.rule_data)
    .bind(&req.format_data)
    .bind(priority)
    .bind(&now)
    .execute(&pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create conditional format: {}", e)))?;
    
    let rule = ConditionalFormatRule {
        id: id.clone(),
        sheet_id: req.sheet_id,
        range: req.range,
        rule_type: req.rule_type,
        rule_data: req.rule_data,
        format_data: req.format_data,
        priority,
        created_at: now,
    };
    
    Ok(Json(rule))
}

pub async fn update_conditional_format(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(req): Json<UpdateConditionalFormatRequest>,
) -> Result<Json<ConditionalFormatRule>, SpreadsheetError> {
    let mut query = "UPDATE conditional_formatting_rules SET updated_at = ?".to_string();
    let mut params = vec![Utc::now().to_rfc3339()];
    
    if let Some(range) = &req.range {
        query.push_str(", range = ?");
        params.push(range.clone());
    }
    if let Some(rule_type) = &req.rule_type {
        query.push_str(", rule_type = ?");
        params.push(rule_type.clone());
    }
    if let Some(rule_data) = &req.rule_data {
        query.push_str(", rule_data = ?");
        params.push(rule_data.clone());
    }
    if let Some(format_data) = &req.format_data {
        query.push_str(", format_data = ?");
        params.push(format_data.clone());
    }
    if let Some(priority) = req.priority {
        query.push_str(", priority = ?");
        params.push(priority.to_string());
    }
    
    query.push_str(" WHERE id = ?");
    params.push(id.clone());
    
    let mut sql_query = sqlx::query(&query);
    for param in params {
        sql_query = sql_query.bind(param);
    }
    
    sql_query.execute(&pool)
        .await
        .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to update conditional format: {}", e)))?;
    
    let rule = sqlx::query_as::<_, ConditionalFormatRule>(
        "SELECT id, sheet_id, range, rule_type, rule_data, format_data, priority, created_at 
         FROM conditional_formatting_rules WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to fetch updated conditional format: {}", e)))?;
    
    Ok(Json(rule))
}

pub async fn delete_conditional_format(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<StatusCode, SpreadsheetError> {
    sqlx::query("DELETE FROM conditional_formatting_rules WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to delete conditional format: {}", e)))?;
    
    Ok(StatusCode::NO_CONTENT)
}

// Chart Handlers
pub async fn list_charts(
    State(pool): State<SqlitePool>,
    Path(sheet_id): Path<String>,
) -> Result<Json<Vec<Chart>>, SpreadsheetError> {
    let charts = sqlx::query_as::<_, Chart>(
        "SELECT id, sheet_id, name, chart_type, data_range, title, x_axis_title, y_axis_title, legend_position, style_data, created_at, updated_at 
         FROM charts 
         WHERE sheet_id = ? 
         ORDER BY created_at DESC"
    )
    .bind(&sheet_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to list charts: {}", e)))?;

    Ok(Json(charts))
}

pub async fn create_chart(
    State(pool): State<SqlitePool>,
    Json(req): Json<CreateChartRequest>,
) -> Result<Json<Chart>, SpreadsheetError> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    sqlx::query(
        "INSERT INTO charts (id, sheet_id, name, chart_type, data_range, title, x_axis_title, y_axis_title, legend_position, style_data, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&req.sheet_id)
    .bind(&req.name)
    .bind(&req.chart_type)
    .bind(&req.data_range)
    .bind(&req.title)
    .bind(&req.x_axis_title)
    .bind(&req.y_axis_title)
    .bind(&req.legend_position)
    .bind(&req.style_data)
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create chart: {}", e)))?;
    
    let chart = Chart {
        id: id.clone(),
        sheet_id: req.sheet_id,
        name: req.name,
        chart_type: req.chart_type,
        data_range: req.data_range,
        title: req.title,
        x_axis_title: req.x_axis_title,
        y_axis_title: req.y_axis_title,
        legend_position: req.legend_position,
        style_data: req.style_data,
        created_at: now,
        updated_at: now,
    };
    
    Ok(Json(chart))
}

pub async fn update_chart(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(req): Json<UpdateChartRequest>,
) -> Result<Json<Chart>, SpreadsheetError> {
    let now = Utc::now();
    
    let mut query = "UPDATE charts SET updated_at = ?".to_string();
    let mut params: Vec<String> = vec![now.to_rfc3339()];
    
    if let Some(name) = &req.name {
        query.push_str(", name = ?");
        params.push(name.clone());
    }
    if let Some(chart_type) = &req.chart_type {
        query.push_str(", chart_type = ?");
        params.push(chart_type.clone());
    }
    if let Some(data_range) = &req.data_range {
        query.push_str(", data_range = ?");
        params.push(data_range.clone());
    }
    if let Some(title) = &req.title {
        query.push_str(", title = ?");
        params.push(title.clone());
    }
    if let Some(x_axis_title) = &req.x_axis_title {
        query.push_str(", x_axis_title = ?");
        params.push(x_axis_title.clone());
    }
    if let Some(y_axis_title) = &req.y_axis_title {
        query.push_str(", y_axis_title = ?");
        params.push(y_axis_title.clone());
    }
    if let Some(legend_position) = &req.legend_position {
        query.push_str(", legend_position = ?");
        params.push(legend_position.clone());
    }
    if let Some(style_data) = &req.style_data {
        query.push_str(", style_data = ?");
        params.push(style_data.clone());
    }
    
    query.push_str(" WHERE id = ?");
    params.push(id.clone());
    
    let mut sql_query = sqlx::query(&query);
    for param in params {
        sql_query = sql_query.bind(param);
    }
    
    sql_query.execute(&pool)
        .await
        .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to update chart: {}", e)))?;
    
    let chart = sqlx::query_as::<_, Chart>(
        "SELECT id, sheet_id, name, chart_type, data_range, title, x_axis_title, y_axis_title, legend_position, style_data, created_at, updated_at 
         FROM charts WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to fetch updated chart: {}", e)))?;
    
    Ok(Json(chart))
}

pub async fn delete_chart(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<StatusCode, SpreadsheetError> {
    sqlx::query("DELETE FROM charts WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to delete chart: {}", e)))?;
    
    Ok(StatusCode::NO_CONTENT)
}

// Pivot Table Handlers
pub async fn list_pivot_tables(
    State(pool): State<SqlitePool>,
    Path(sheet_id): Path<String>,
) -> Result<Json<Vec<PivotTable>>, SpreadsheetError> {
    let pivots = sqlx::query_as::<_, PivotTable>(
        "SELECT id, sheet_id, name, source_range, row_fields, column_fields, value_fields, filter_fields, created_at, updated_at 
         FROM pivot_tables 
         WHERE sheet_id = ? 
         ORDER BY created_at DESC"
    )
    .bind(&sheet_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to list pivot tables: {}", e)))?;

    Ok(Json(pivots))
}

pub async fn create_pivot_table(
    State(pool): State<SqlitePool>,
    Json(req): Json<CreatePivotTableRequest>,
) -> Result<Json<PivotTable>, SpreadsheetError> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    sqlx::query(
        "INSERT INTO pivot_tables (id, sheet_id, name, source_range, row_fields, column_fields, value_fields, filter_fields, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&req.sheet_id)
    .bind(&req.name)
    .bind(&req.source_range)
    .bind(&req.row_fields)
    .bind(&req.column_fields)
    .bind(&req.value_fields)
    .bind(&req.filter_fields)
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create pivot table: {}", e)))?;
    
    let pivot = PivotTable {
        id: id.clone(),
        sheet_id: req.sheet_id,
        name: req.name,
        source_range: req.source_range,
        row_fields: req.row_fields,
        column_fields: req.column_fields,
        value_fields: req.value_fields,
        filter_fields: req.filter_fields,
        created_at: now,
        updated_at: now,
    };
    
    Ok(Json(pivot))
}

pub async fn update_pivot_table(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(req): Json<UpdatePivotTableRequest>,
) -> Result<Json<PivotTable>, SpreadsheetError> {
    let now = Utc::now();
    
    let mut query = "UPDATE pivot_tables SET updated_at = ?".to_string();
    let mut params: Vec<String> = vec![now.to_rfc3339()];
    
    if let Some(name) = &req.name {
        query.push_str(", name = ?");
        params.push(name.clone());
    }
    if let Some(source_range) = &req.source_range {
        query.push_str(", source_range = ?");
        params.push(source_range.clone());
    }
    if let Some(row_fields) = &req.row_fields {
        query.push_str(", row_fields = ?");
        params.push(row_fields.clone());
    }
    if let Some(column_fields) = &req.column_fields {
        query.push_str(", column_fields = ?");
        params.push(column_fields.clone());
    }
    if let Some(value_fields) = &req.value_fields {
        query.push_str(", value_fields = ?");
        params.push(value_fields.clone());
    }
    if let Some(filter_fields) = &req.filter_fields {
        query.push_str(", filter_fields = ?");
        params.push(filter_fields.clone());
    }
    
    query.push_str(" WHERE id = ?");
    params.push(id.clone());
    
    let mut sql_query = sqlx::query(&query);
    for param in params {
        sql_query = sql_query.bind(param);
    }
    
    sql_query.execute(&pool)
        .await
        .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to update pivot table: {}", e)))?;
    
    let pivot = sqlx::query_as::<_, PivotTable>(
        "SELECT id, sheet_id, name, source_range, row_fields, column_fields, value_fields, filter_fields, created_at, updated_at 
         FROM pivot_tables WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to fetch updated pivot table: {}", e)))?;
    
    Ok(Json(pivot))
}

pub async fn delete_pivot_table(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<StatusCode, SpreadsheetError> {
    sqlx::query("DELETE FROM pivot_tables WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to delete pivot table: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}

// Batch operation handlers

pub async fn batch_create_cells(
    State(pool): State<SqlitePool>,
    Path(sheet_id): Path<String>,
    Json(req): Json<BatchCreateCellsRequest>,
) -> Result<Json<BatchOperationResponse>, SpreadsheetError> {
    let validated_sheet_id = RequestValidator::validate_uuid(&sheet_id)?;
    let now = Utc::now();
    let mut succeeded = 0;
    let mut failed = 0;
    let mut errors = Vec::new();

    // Use simple transaction with sqlx
    let mut tx = pool.begin().await
        .map_err(|e| SpreadsheetError::DatabaseTransaction(format!("Failed to begin transaction: {}", e)))?;

    for cell_req in &req.cells {
        match RequestValidator::validate_create_cell_request(
            cell_req.row,
            cell_req.col,
            &cell_req.value,
            &cell_req.formula,
            &cell_req.style,
        ) {
            Ok(_) => {
                let id = Uuid::new_v4().to_string();

                match sqlx::query(
                    "INSERT INTO cells (id, sheet_id, row, col, value, formula, style, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
                )
                .bind(&id)
                .bind(&validated_sheet_id)
                .bind(cell_req.row)
                .bind(cell_req.col)
                .bind(&cell_req.value)
                .bind(&cell_req.formula)
                .bind(&cell_req.style)
                .bind(&now)
                .bind(&now)
                .execute(&mut *tx)
                .await
                {
                    Ok(_) => succeeded += 1,
                    Err(e) => {
                        failed += 1;
                        errors.push(format!("Cell ({},{}): {}", cell_req.row, cell_req.col, e));
                    }
                }
            }
            Err(e) => {
                failed += 1;
                errors.push(format!("Cell ({},{}): {}", cell_req.row, cell_req.col, e));
            }
        }
    }

    // Commit or rollback based on results
    if failed == 0 {
        tx.commit().await
            .map_err(|e| SpreadsheetError::DatabaseTransaction(format!("Failed to commit transaction: {}", e)))?;
    } else {
        tx.rollback().await
            .map_err(|e| SpreadsheetError::DatabaseTransaction(format!("Failed to rollback transaction: {}", e)))?;
    }

    Ok(Json(BatchOperationResponse {
        succeeded,
        failed,
        errors,
    }))
}

pub async fn batch_update_cells(
    State(pool): State<SqlitePool>,
    Path(sheet_id): Path<String>,
    Json(req): Json<BatchUpdateCellsRequest>,
) -> Result<Json<BatchOperationResponse>, SpreadsheetError> {
    let validated_sheet_id = RequestValidator::validate_uuid(&sheet_id)?;
    let now = Utc::now();
    let mut succeeded = 0;
    let mut failed = 0;
    let mut errors = Vec::new();

    let mut tx = pool.begin().await
        .map_err(|e| SpreadsheetError::DatabaseTransaction(format!("Failed to begin transaction: {}", e)))?;

    for cell_item in &req.cells {
        match RequestValidator::validate_update_cell_request(
            &cell_item.value,
            &cell_item.formula,
            &cell_item.style,
        ) {
            Ok(_) => {
                match sqlx::query(
                    "UPDATE cells SET value = ?, formula = ?, style = ?, updated_at = ? WHERE sheet_id = ? AND row = ? AND col = ?"
                )
                .bind(&cell_item.value)
                .bind(&cell_item.formula)
                .bind(&cell_item.style)
                .bind(&now)
                .bind(&validated_sheet_id)
                .bind(cell_item.row)
                .bind(cell_item.col)
                .execute(&mut *tx)
                .await
                {
                    Ok(_) => succeeded += 1,
                    Err(e) => {
                        failed += 1;
                        errors.push(format!("Cell ({},{}): {}", cell_item.row, cell_item.col, e));
                    }
                }
            }
            Err(e) => {
                failed += 1;
                errors.push(format!("Cell ({},{}): {}", cell_item.row, cell_item.col, e));
            }
        }
    }

    if failed == 0 {
        tx.commit().await
            .map_err(|e| SpreadsheetError::DatabaseTransaction(format!("Failed to commit transaction: {}", e)))?;
    } else {
        tx.rollback().await
            .map_err(|e| SpreadsheetError::DatabaseTransaction(format!("Failed to rollback transaction: {}", e)))?;
    }

    Ok(Json(BatchOperationResponse {
        succeeded,
        failed,
        errors,
    }))
}

pub async fn batch_delete_cells(
    State(pool): State<SqlitePool>,
    Path(sheet_id): Path<String>,
    Json(req): Json<BatchDeleteCellsRequest>,
) -> Result<Json<BatchOperationResponse>, SpreadsheetError> {
    let validated_sheet_id = RequestValidator::validate_uuid(&sheet_id)?;
    let mut succeeded = 0;
    let mut failed = 0;
    let mut errors = Vec::new();

    let mut tx = pool.begin().await
        .map_err(|e| SpreadsheetError::DatabaseTransaction(format!("Failed to begin transaction: {}", e)))?;

    for cell_item in &req.cells {
        match RequestValidator::validate_cell_coordinates(
            cell_item.row,
            cell_item.col,
        ) {
            Ok(_) => {
                match sqlx::query("DELETE FROM cells WHERE sheet_id = ? AND row = ? AND col = ?")
                    .bind(&validated_sheet_id)
                    .bind(cell_item.row)
                    .bind(cell_item.col)
                    .execute(&mut *tx)
                    .await
                {
                    Ok(_) => succeeded += 1,
                    Err(e) => {
                        failed += 1;
                        errors.push(format!("Cell ({},{}): {}", cell_item.row, cell_item.col, e));
                    }
                }
            }
            Err(e) => {
                failed += 1;
                errors.push(format!("Cell ({},{}): {}", cell_item.row, cell_item.col, e));
            }
        }
    }

    if failed == 0 {
        tx.commit().await
            .map_err(|e| SpreadsheetError::DatabaseTransaction(format!("Failed to commit transaction: {}", e)))?;
    } else {
        tx.rollback().await
            .map_err(|e| SpreadsheetError::DatabaseTransaction(format!("Failed to rollback transaction: {}", e)))?;
    }

    Ok(Json(BatchOperationResponse {
        succeeded,
        failed,
        errors,
    }))
}
