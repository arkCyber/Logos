//! Pivot tables module
//! Provides data aggregation and summarization for spreadsheet data

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error::{SpreadsheetError, SpreadsheetResult};

/// Pivot table definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotTable {
    pub id: String,
    pub sheet_id: String,
    pub name: String,
    pub source_range: String, // e.g., "A1:D100"
    pub row_fields: Vec<PivotField>,
    pub column_fields: Vec<PivotField>,
    pub value_fields: Vec<PivotValueField>,
    pub filter_fields: Vec<PivotField>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Pivot field (row, column, or filter)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotField {
    pub column_index: i32,
    pub name: String,
    pub sort_order: Option<SortOrder>,
    pub custom_sort: Option<Vec<String>>,
}

/// Pivot value field (aggregation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotValueField {
    pub column_index: i32,
    pub name: String,
    pub aggregation: AggregationType,
    pub custom_name: Option<String>,
}

/// Sort order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
    Custom,
}

/// Aggregation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AggregationType {
    Sum,
    Count,
    Average,
    Min,
    Max,
    CountA, // Count non-empty
    CountDistinct,
    Product,
    StdDev,
    StdDevP,
    Var,
    VarP,
}

/// Pivot table data for rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotData {
    pub pivot_table: PivotTable,
    pub data: Vec<PivotCell>,
    pub row_headers: Vec<String>,
    pub column_headers: Vec<String>,
}

/// Pivot cell data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotCell {
    pub row_index: i32,
    pub col_index: i32,
    pub value: PivotValue,
    pub is_total: bool,
    pub is_grand_total: bool,
}

/// Pivot cell value
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PivotValue {
    Number(f64),
    Text(String),
    Error(String),
}

/// Pivot table service
pub struct PivotTableService;

impl PivotTableService {
    /// Generate pivot table data from source data
    pub fn generate_pivot_data(
        pivot: &PivotTable,
        source_data: &Vec<Vec<String>>, // 2D array of cell values
    ) -> SpreadsheetResult<PivotData> {
        if source_data.is_empty() {
            return Err(SpreadsheetError::Validation("Source data is empty".to_string()));
        }

        // Extract column headers from first row
        let _headers = &source_data[0];
        let data_rows = &source_data[1..];

        // Build data map for aggregation
        let mut data_map: HashMap<String, Vec<f64>> = HashMap::new();

        for row in data_rows {
            // Build key from row and column fields
            let mut key_parts = Vec::new();

            for field in &pivot.row_fields {
                if field.column_index < row.len() as i32 {
                    key_parts.push(row[field.column_index as usize].clone());
                }
            }

            for field in &pivot.column_fields {
                if field.column_index < row.len() as i32 {
                    key_parts.push(row[field.column_index as usize].clone());
                }
            }

            let key = key_parts.join("|");

            // Aggregate value fields
            for value_field in &pivot.value_fields {
                if value_field.column_index < row.len() as i32 {
                    let cell_value = &row[value_field.column_index as usize];
                    if let Ok(num) = cell_value.parse::<f64>() {
                        data_map.entry(key.clone())
                            .or_insert_with(Vec::new)
                            .push(num);
                    }
                }
            }
        }

        // Generate pivot cells
        let mut pivot_cells = Vec::new();
        let mut row_headers = Vec::new();
        let mut column_headers = Vec::new();

        // Extract unique row headers
        for field in &pivot.row_fields {
            let mut unique_values: Vec<String> = data_rows
                .iter()
                .filter_map(|row| {
                    if field.column_index < row.len() as i32 {
                        Some(row[field.column_index as usize].clone())
                    } else {
                        None
                    }
                })
                .collect();
            unique_values.sort();
            unique_values.dedup();
            row_headers.extend(unique_values);
        }

        // Extract unique column headers
        for field in &pivot.column_fields {
            let mut unique_values: Vec<String> = data_rows
                .iter()
                .filter_map(|row| {
                    if field.column_index < row.len() as i32 {
                        Some(row[field.column_index as usize].clone())
                    } else {
                        None
                    }
                })
                .collect();
            unique_values.sort();
            unique_values.dedup();
            column_headers.extend(unique_values);
        }

        // Generate cells
        let mut row_idx = 0;
        for row_header in &row_headers {
            let mut col_idx = 0;
            for col_header in &column_headers {
                let key = format!("{}|{}", row_header, col_header);
                
                let value = if let Some(values) = data_map.get(&key) {
                    if let Some(value_field) = pivot.value_fields.first() {
                        Self::aggregate_values(values, &value_field.aggregation)
                    } else {
                        PivotValue::Number(0.0)
                    }
                } else {
                    PivotValue::Number(0.0)
                };

                pivot_cells.push(PivotCell {
                    row_index: row_idx,
                    col_index: col_idx,
                    value,
                    is_total: false,
                    is_grand_total: false,
                });

                col_idx += 1;
            }
            row_idx += 1;
        }

        Ok(PivotData {
            pivot_table: pivot.clone(),
            data: pivot_cells,
            row_headers,
            column_headers,
        })
    }

    /// Aggregate values based on aggregation type
    fn aggregate_values(values: &[f64], aggregation: &AggregationType) -> PivotValue {
        if values.is_empty() {
            return PivotValue::Number(0.0);
        }

        match aggregation {
            AggregationType::Sum => {
                PivotValue::Number(values.iter().sum())
            }
            AggregationType::Count => {
                PivotValue::Number(values.len() as f64)
            }
            AggregationType::Average => {
                let sum: f64 = values.iter().sum();
                PivotValue::Number(sum / values.len() as f64)
            }
            AggregationType::Min => {
                PivotValue::Number(values.iter().cloned().fold(f64::INFINITY, f64::min))
            }
            AggregationType::Max => {
                PivotValue::Number(values.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
            }
            AggregationType::CountA => {
                PivotValue::Number(values.iter().filter(|&&v| v != 0.0).count() as f64)
            }
            AggregationType::CountDistinct => {
                let unique: std::collections::HashSet<_> = values.iter()
                    .map(|&v| v as i64)
                    .collect();
                PivotValue::Number(unique.len() as f64)
            }
            AggregationType::Product => {
                PivotValue::Number(values.iter().product())
            }
            AggregationType::StdDev => {
                let avg = values.iter().sum::<f64>() / values.len() as f64;
                let variance = values.iter()
                    .map(|&x| (x - avg).powi(2))
                    .sum::<f64>() / (values.len() - 1) as f64;
                PivotValue::Number(variance.sqrt())
            }
            AggregationType::StdDevP => {
                let avg = values.iter().sum::<f64>() / values.len() as f64;
                let variance = values.iter()
                    .map(|&x| (x - avg).powi(2))
                    .sum::<f64>() / values.len() as f64;
                PivotValue::Number(variance.sqrt())
            }
            AggregationType::Var => {
                let avg = values.iter().sum::<f64>() / values.len() as f64;
                let variance = values.iter()
                    .map(|&x| (x - avg).powi(2))
                    .sum::<f64>() / (values.len() - 1) as f64;
                PivotValue::Number(variance)
            }
            AggregationType::VarP => {
                let avg = values.iter().sum::<f64>() / values.len() as f64;
                let variance = values.iter()
                    .map(|&x| (x - avg).powi(2))
                    .sum::<f64>() / values.len() as f64;
                PivotValue::Number(variance)
            }
        }
    }

    /// Parse cell range (e.g., "A1:D100")
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

    /// Validate pivot table configuration
    pub fn validate_pivot(pivot: &PivotTable) -> SpreadsheetResult<()> {
        if pivot.name.is_empty() {
            return Err(SpreadsheetError::Validation("Pivot table name cannot be empty".to_string()));
        }

        if pivot.value_fields.is_empty() {
            return Err(SpreadsheetError::Validation("At least one value field is required".to_string()));
        }

        if pivot.row_fields.is_empty() && pivot.column_fields.is_empty() {
            return Err(SpreadsheetError::Validation("At least one row or column field is required".to_string()));
        }

        // Validate source range
        Self::parse_range(&pivot.source_range)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cell_reference() {
        assert_eq!(PivotTableService::parse_cell_reference("A1").unwrap(), (0, 0));
        assert_eq!(PivotTableService::parse_cell_reference("D10").unwrap(), (3, 9));
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(
            PivotTableService::parse_range("A1:D10").unwrap(),
            (0, 0, 3, 9)
        );
    }

    #[test]
    fn test_aggregate_values_sum() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = PivotTableService::aggregate_values(&values, &AggregationType::Sum);
        assert_eq!(result, PivotValue::Number(15.0));
    }

    #[test]
    fn test_aggregate_values_average() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = PivotTableService::aggregate_values(&values, &AggregationType::Average);
        assert_eq!(result, PivotValue::Number(3.0));
    }

    #[test]
    fn test_aggregate_values_count() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = PivotTableService::aggregate_values(&values, &AggregationType::Count);
        assert_eq!(result, PivotValue::Number(5.0));
    }

    #[test]
    fn test_generate_pivot_data() {
        let pivot = PivotTable {
            id: "1".to_string(),
            sheet_id: "sheet1".to_string(),
            name: "Test Pivot".to_string(),
            source_range: "A1:C5".to_string(),
            row_fields: vec![PivotField {
                column_index: 0,
                name: "Category".to_string(),
                sort_order: None,
                custom_sort: None,
            }],
            column_fields: vec![],
            value_fields: vec![PivotValueField {
                column_index: 1,
                name: "Value".to_string(),
                aggregation: AggregationType::Sum,
                custom_name: None,
            }],
            filter_fields: vec![],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let source_data = vec![
            vec!["Category".to_string(), "Value".to_string(), "Other".to_string()],
            vec!["A".to_string(), "10".to_string(), "X".to_string()],
            vec!["A".to_string(), "20".to_string(), "Y".to_string()],
            vec!["B".to_string(), "30".to_string(), "Z".to_string()],
            vec!["B".to_string(), "40".to_string(), "W".to_string()],
        ];

        let pivot_data = PivotTableService::generate_pivot_data(&pivot, &source_data).unwrap();
        assert_eq!(pivot_data.row_headers.len(), 2);
        assert_eq!(pivot_data.data.len(), 2);
    }

    #[test]
    fn test_validate_pivot() {
        let pivot = PivotTable {
            id: "1".to_string(),
            sheet_id: "sheet1".to_string(),
            name: "Test Pivot".to_string(),
            source_range: "A1:D10".to_string(),
            row_fields: vec![PivotField {
                column_index: 0,
                name: "Category".to_string(),
                sort_order: None,
                custom_sort: None,
            }],
            column_fields: vec![],
            value_fields: vec![PivotValueField {
                column_index: 1,
                name: "Value".to_string(),
                aggregation: AggregationType::Sum,
                custom_name: None,
            }],
            filter_fields: vec![],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        assert!(PivotTableService::validate_pivot(&pivot).is_ok());
    }
}
