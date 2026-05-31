//! Excel import/export module using calamine and umya-spreadsheet

use crate::config::AppConfig;
use crate::error::{SpreadsheetError, SpreadsheetResult};
use crate::models::{Sheet, Cell};
use sqlx::SqlitePool;
use tracing::{info, error};
use uuid::Uuid;
use chrono::Utc;
use calamine::{Reader, Xlsx, open_workbook, Data};
use umya_spreadsheet::{Spreadsheet, writer};
use std::path::Path;

/// Excel import service
pub struct ExcelImporter {
    config: AppConfig,
}

impl ExcelImporter {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    /// Import Excel file and create sheet with cells
    pub async fn import_excel(
        &self,
        file_path: &str,
        pool: &SqlitePool,
    ) -> SpreadsheetResult<Sheet> {
        info!(file_path = %file_path, "Starting Excel import");

        // Validate file exists
        if !Path::new(file_path).exists() {
            return Err(SpreadsheetError::Validation(format!("File not found: {}", file_path)));
        }

        // Open Excel file using calamine
        let mut workbook: Xlsx<_> = open_workbook(file_path)
            .map_err(|e| SpreadsheetError::FileOperation(format!("Failed to open Excel file: {}", e)))?;

        // Get first sheet name
        let sheet_names = workbook.sheet_names();
        if sheet_names.is_empty() {
            return Err(SpreadsheetError::Validation("Excel file has no sheets".to_string()));
        }

        let first_sheet_name = &sheet_names[0];
        let range = workbook.worksheet_range(first_sheet_name)
            .map_err(|e| SpreadsheetError::FileOperation(format!("Failed to read sheet: {}", e)))?;

        // Create sheet in database
        let sheet_id = Uuid::new_v4().to_string();
        let sheet_name = first_sheet_name.clone();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO sheets (id, name, created_at, updated_at) VALUES (?, ?, ?, ?)"
        )
        .bind(&sheet_id)
        .bind(&sheet_name)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await
        .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create sheet: {}", e)))?;

        // Import cells
        let mut cell_count = 0;
        for (row_idx, row) in range.rows().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                let cell_value = match cell {
                    Data::String(s) => Some(s.clone()),
                    Data::Float(f) => Some(f.to_string()),
                    Data::Int(i) => Some(i.to_string()),
                    Data::Bool(b) => Some(b.to_string()),
                    Data::DateTime(dt) => Some(dt.to_string()),
                    Data::Error(e) => {
                        error!("Cell error at ({}, {}): {:?}", row_idx, col_idx, e);
                        continue;
                    }
                    Data::Empty => continue,
                    _ => continue,
                };

                if let Some(value) = cell_value {
                    let cell_id = Uuid::new_v4().to_string();
                    let row = row_idx as i32;
                    let col = col_idx as i32;

                    sqlx::query(
                        "INSERT INTO cells (id, sheet_id, row, col, value, formula, style, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
                    )
                    .bind(&cell_id)
                    .bind(&sheet_id)
                    .bind(row)
                    .bind(col)
                    .bind(&value)
                    .bind::<Option<String>>(None)
                    .bind::<Option<String>>(None)
                    .bind(&now)
                    .bind(&now)
                    .execute(pool)
                    .await
                    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create cell: {}", e)))?;

                    cell_count += 1;
                }
            }
        }

        info!(
            sheet_id = %sheet_id,
            sheet_name = %sheet_name,
            cells_imported = cell_count,
            "Excel import completed successfully"
        );

        Ok(Sheet {
            id: sheet_id,
            name: sheet_name,
            created_at: now,
            updated_at: now,
        })
    }
}

/// Excel export service
pub struct ExcelExporter {
    config: AppConfig,
}

impl ExcelExporter {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    /// Export sheet to Excel file
    pub async fn export_excel(
        &self,
        sheet_id: &str,
        pool: &SqlitePool,
    ) -> SpreadsheetResult<Vec<u8>> {
        info!(sheet_id = %sheet_id, "Starting Excel export");

        // Get sheet info
        let sheet = sqlx::query_as::<_, Sheet>(
            "SELECT id, name, created_at, updated_at FROM sheets WHERE id = ?"
        )
        .bind(sheet_id)
        .fetch_one(pool)
        .await
        .map_err(|_| SpreadsheetError::NotFound {
            resource: "sheet".to_string(),
            id: sheet_id.to_string(),
        })?;

        // Get all cells for the sheet
        let cells = sqlx::query_as::<_, Cell>(
            "SELECT id, sheet_id, row, col, value, formula, style, created_at, updated_at FROM cells WHERE sheet_id = ? ORDER BY row, col"
        )
        .bind(sheet_id)
        .fetch_all(pool)
        .await
        .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to fetch cells: {}", e)))?;

        // Create new spreadsheet using umya-spreadsheet
        let mut spreadsheet = Spreadsheet::default();
        let worksheet = spreadsheet.get_sheet_mut(&0).expect("Failed to get worksheet");
        worksheet.set_name(&sheet.name);

        // Populate cells
        let cells_count = cells.len();
        for cell in &cells {
            if let Some(value) = &cell.value {
                let row_idx = (cell.row + 1) as u32;
                let col_idx = (cell.col + 1) as u32;
                
                worksheet.get_cell_mut((col_idx, row_idx))
                    .set_value_string(value);
            }
        }

        // Export to bytes using temporary file
        let temp_dir = std::path::Path::new(&self.config.excel.temp_dir);
        std::fs::create_dir_all(temp_dir)
            .map_err(|e| SpreadsheetError::FileOperation(format!("Failed to create temp dir: {}", e)))?;
        
        let temp_path = temp_dir.join(format!("temp_export_{}.xlsx", Uuid::new_v4()));
        
        // Use umya-spreadsheet writer to save to file
        writer::xlsx::write(&spreadsheet, &temp_path)
            .map_err(|e| SpreadsheetError::FileOperation(format!("Failed to export Excel: {}", e)))?;
        
        let bytes = std::fs::read(&temp_path)
            .map_err(|e| SpreadsheetError::FileOperation(format!("Failed to read exported file: {}", e)))?;
        
        // Clean up temp file
        let _ = std::fs::remove_file(&temp_path);

        info!(
            sheet_id = %sheet_id,
            sheet_name = %sheet.name,
            cells_exported = cells_count,
            "Excel export completed successfully"
        );

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_excel_importer_creation() {
        let config = AppConfig::default();
        let importer = ExcelImporter::new(config);
        assert_eq!(importer.config.excel.max_file_size, 10 * 1024 * 1024);
    }

    #[test]
    fn test_excel_exporter_creation() {
        let config = AppConfig::default();
        let exporter = ExcelExporter::new(config);
        assert_eq!(exporter.config.excel.temp_dir, "./temp");
    }
}
