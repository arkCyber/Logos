//! Pivot table generation with aerospace-grade aggregation
//! 
//! This module provides comprehensive pivot table functionality including
//! multiple aggregation types, filtering, and data grouping.

use crate::spreadsheet_service::{
    error::{SpreadsheetError, SpreadsheetResult},
    types::CellValue,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pivot aggregation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PivotAggregation {
    Sum,
    Average,
    Count,
    CountNumbers,
    Min,
    Max,
    Product,
    StdDev,
    StdDevP,
    Var,
    VarP,
}

/// Pivot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotConfig {
    /// Row fields
    pub rows: Vec<String>,
    /// Column fields
    pub columns: Vec<String>,
    /// Value fields with aggregation
    pub values: Vec<PivotValue>,
    /// Filter fields
    pub filters: Vec<PivotFilter>,
}

/// Pivot value field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotValue {
    /// Field name
    pub field: String,
    /// Aggregation type
    pub aggregation: PivotAggregation,
    /// Custom name for the value
    pub name: Option<String>,
}

/// Pivot filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotFilter {
    /// Field name
    pub field: String,
    /// Filter operator
    pub operator: FilterOperator,
    /// Filter value
    pub value: String,
}

/// Filter operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
}

/// Pivot table result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotTable {
    /// Pivot configuration
    pub config: PivotConfig,
    /// Pivot data
    pub data: PivotData,
}

/// Pivot data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotData {
    /// Headers
    pub headers: Vec<String>,
    /// Rows
    pub rows: Vec<PivotRow>,
    /// Grand total
    pub grand_total: Option<HashMap<String, f64>>,
}

/// Pivot row
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotRow {
    /// Row labels
    pub labels: Vec<String>,
    /// Values
    pub values: HashMap<String, f64>,
    /// Row total
    pub total: f64,
}

/// Pivot table generator
pub struct PivotGenerator;

impl PivotGenerator {
    /// Create a new pivot generator
    pub fn new() -> Self {
        Self
    }

    /// Generate a pivot table from data
    pub fn generate(
        &self,
        data: &[HashMap<String, CellValue>],
        config: PivotConfig,
    ) -> SpreadsheetResult<PivotTable> {
        // Validate configuration
        if config.rows.is_empty() && config.columns.is_empty() {
            return Err(SpreadsheetError::pivot_error(
                "generate",
                "Pivot table must have at least one row or column field",
            ));
        }

        if config.values.is_empty() {
            return Err(SpreadsheetError::pivot_error(
                "generate",
                "Pivot table must have at least one value field",
            ));
        }

        // Apply filters
        let filtered_data = self.apply_filters(data, &config.filters)?;

        // Group data
        let grouped = self.group_data(&filtered_data, &config);

        // Calculate aggregations
        let pivot_data = self.calculate_aggregations(&grouped, &config);

        Ok(PivotTable {
            config,
            data: pivot_data,
        })
    }

    /// Apply filters to data
    fn apply_filters(
        &self,
        data: &[HashMap<String, CellValue>],
        filters: &[PivotFilter],
    ) -> SpreadsheetResult<Vec<HashMap<String, CellValue>>> {
        let mut filtered = Vec::new();

        for row in data {
            let mut include = true;

            for filter in filters {
                if let Some(value) = row.get(&filter.field) {
                    let value_str = match value {
                        CellValue::Text(s) => s.clone(),
                        CellValue::Number(n) => n.to_string(),
                        CellValue::Boolean(b) => b.to_string(),
                        _ => continue,
                    };

                    let matches = match filter.operator {
                        FilterOperator::Equals => value_str == filter.value,
                        FilterOperator::NotEquals => value_str != filter.value,
                        FilterOperator::GreaterThan => {
                            let row_val = value_str.parse::<f64>().unwrap_or(f64::NEG_INFINITY);
                            let filter_val = filter.value.parse::<f64>().unwrap_or(f64::NEG_INFINITY);
                            row_val > filter_val
                        }
                        FilterOperator::LessThan => {
                            let row_val = value_str.parse::<f64>().unwrap_or(f64::INFINITY);
                            let filter_val = filter.value.parse::<f64>().unwrap_or(f64::INFINITY);
                            row_val < filter_val
                        }
                        FilterOperator::GreaterThanOrEqual => {
                            let row_val = value_str.parse::<f64>().unwrap_or(f64::NEG_INFINITY);
                            let filter_val = filter.value.parse::<f64>().unwrap_or(f64::NEG_INFINITY);
                            row_val >= filter_val
                        }
                        FilterOperator::LessThanOrEqual => {
                            let row_val = value_str.parse::<f64>().unwrap_or(f64::INFINITY);
                            let filter_val = filter.value.parse::<f64>().unwrap_or(f64::INFINITY);
                            row_val <= filter_val
                        }
                        FilterOperator::Contains => value_str.contains(&filter.value),
                        FilterOperator::NotContains => !value_str.contains(&filter.value),
                        FilterOperator::StartsWith => value_str.starts_with(&filter.value),
                        FilterOperator::EndsWith => value_str.ends_with(&filter.value),
                    };

                    if !matches {
                        include = false;
                        break;
                    }
                } else {
                    include = false;
                    break;
                }
            }

            if include {
                filtered.push(row.clone());
            }
        }

        Ok(filtered)
    }

    /// Group data by row and column fields
    fn group_data<'a>(
        &self,
        data: &'a [HashMap<String, CellValue>],
        config: &PivotConfig,
    ) -> HashMap<Vec<String>, Vec<&'a HashMap<String, CellValue>>> {
        let mut grouped: HashMap<Vec<String>, Vec<&'a HashMap<String, CellValue>>> = HashMap::new();

        for row in data {
            let mut key = Vec::new();

            // Group by row fields
            for field in &config.rows {
                key.push(
                    row.get(field)
                        .and_then(|v| match v {
                            CellValue::Text(s) => Some(s.clone()),
                            CellValue::Number(n) => Some(n.to_string()),
                            CellValue::Boolean(b) => Some(b.to_string()),
                            _ => None,
                        })
                        .unwrap_or_else(String::new),
                );
            }

            // Also group by column fields if present
            for field in &config.columns {
                key.push(
                    row.get(field)
                        .and_then(|v| match v {
                            CellValue::Text(s) => Some(s.clone()),
                            CellValue::Number(n) => Some(n.to_string()),
                            CellValue::Boolean(b) => Some(b.to_string()),
                            _ => None,
                        })
                        .unwrap_or_else(String::new),
                );
            }

            grouped.entry(key).or_insert_with(Vec::new).push(row);
        }

        grouped
    }

    /// Calculate aggregations for grouped data
    fn calculate_aggregations(
        &self,
        grouped: &HashMap<Vec<String>, Vec<&HashMap<String, CellValue>>>,
        config: &PivotConfig,
    ) -> PivotData {
        let mut headers = Vec::new();
        let mut rows = Vec::new();
        let mut grand_total: HashMap<String, f64> = HashMap::new();

        // Build headers
        for field in &config.rows {
            headers.push(field.clone());
        }
        for field in &config.columns {
            headers.push(field.clone());
        }
        for value in &config.values {
            headers.push(value.name.as_ref().unwrap_or(&value.field).clone());
        }
        headers.push("Total".to_string());

        // Calculate row aggregations
        for (key_labels, group_data) in grouped {
            let mut values = HashMap::new();
            let mut row_total = 0.0;

            // Separate row labels from column labels
            let row_labels: Vec<String> = key_labels.iter().take(config.rows.len()).cloned().collect();
            let column_labels: Vec<String> = key_labels.iter().skip(config.rows.len()).cloned().collect();

            for pivot_value in &config.values {
                let aggregated = self.aggregate_field(group_data, &pivot_value.field, &pivot_value.aggregation);

                // Create a unique key for this value
                let value_key = if column_labels.is_empty() {
                    pivot_value.name.as_ref().unwrap_or(&pivot_value.field).clone()
                } else {
                    format!("{}-{}", column_labels.join("-"), pivot_value.name.as_ref().unwrap_or(&pivot_value.field))
                };

                values.insert(value_key.clone(), aggregated);
                row_total += aggregated;

                // Add to grand total
                *grand_total
                    .entry(pivot_value.name.as_ref().unwrap_or(&pivot_value.field).clone())
                    .or_insert(0.0) += aggregated;
            }

            rows.push(PivotRow {
                labels: row_labels,
                values,
                total: row_total,
            });
        }

        // Sort rows by labels for consistent output
        rows.sort_by(|a, b| a.labels.cmp(&b.labels));

        PivotData {
            headers,
            rows,
            grand_total: Some(grand_total),
        }
    }

    /// Aggregate a field using the specified aggregation type
    fn aggregate_field(
        &self,
        data: &[&HashMap<String, CellValue>],
        field: &str,
        aggregation: &PivotAggregation,
    ) -> f64 {
        let values: Vec<f64> = data
            .iter()
            .filter_map(|row| row.get(field))
            .filter_map(|v| match v {
                CellValue::Number(n) => Some(*n),
                _ => None,
            })
            .collect();

        match aggregation {
            PivotAggregation::Sum => values.iter().sum(),
            PivotAggregation::Average => {
                if values.is_empty() {
                    0.0
                } else {
                    values.iter().sum::<f64>() / values.len() as f64
                }
            }
            PivotAggregation::Count => data.len() as f64,
            PivotAggregation::CountNumbers => values.len() as f64,
            PivotAggregation::Min => {
                if values.is_empty() {
                    0.0
                } else {
                    values.iter().cloned().fold(f64::INFINITY, f64::min)
                }
            }
            PivotAggregation::Max => {
                if values.is_empty() {
                    0.0
                } else {
                    values.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                }
            }
            PivotAggregation::Product => values.iter().product(),
            PivotAggregation::StdDev => {
                if values.len() < 2 {
                    0.0
                } else {
                    let mean = values.iter().sum::<f64>() / values.len() as f64;
                    let variance = values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (values.len() - 1) as f64;
                    variance.sqrt()
                }
            }
            PivotAggregation::StdDevP => {
                if values.is_empty() {
                    0.0
                } else {
                    let mean = values.iter().sum::<f64>() / values.len() as f64;
                    let variance = values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
                    variance.sqrt()
                }
            }
            PivotAggregation::Var => {
                if values.len() < 2 {
                    0.0
                } else {
                    let mean = values.iter().sum::<f64>() / values.len() as f64;
                    values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (values.len() - 1) as f64
                }
            }
            PivotAggregation::VarP => {
                if values.is_empty() {
                    0.0
                } else {
                    let mean = values.iter().sum::<f64>() / values.len() as f64;
                    values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64
                }
            }
        }
    }
}

impl Default for PivotGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pivot_generator_creation() {
        let generator = PivotGenerator::new();
        // Just test creation
        assert!(true);
    }

    #[test]
    fn test_pivot_config_validation() {
        let generator = PivotGenerator::new();
        let data = vec![];
        let config = PivotConfig {
            rows: vec![],
            columns: vec![],
            values: vec![],
            filters: vec![],
        };
        let result = generator.generate(&data, config);
        assert!(result.is_err());
    }

    #[test]
    fn test_aggregate_sum() {
        let generator = PivotGenerator::new();
        let mut row1 = HashMap::new();
        row1.insert("value".to_string(), CellValue::Number(10.0));
        let mut row2 = HashMap::new();
        row2.insert("value".to_string(), CellValue::Number(20.0));
        
        let data = vec![&row1, &row2];
        let result = generator.aggregate_field(&data, "value", &PivotAggregation::Sum);
        assert_eq!(result, 30.0);
    }

    #[test]
    fn test_aggregate_average() {
        let generator = PivotGenerator::new();
        let mut row1 = HashMap::new();
        row1.insert("value".to_string(), CellValue::Number(10.0));
        let mut row2 = HashMap::new();
        row2.insert("value".to_string(), CellValue::Number(20.0));
        
        let data = vec![&row1, &row2];
        let result = generator.aggregate_field(&data, "value", &PivotAggregation::Average);
        assert_eq!(result, 15.0);
    }

    #[test]
    fn test_aggregate_count() {
        let generator = PivotGenerator::new();
        let mut row1 = HashMap::new();
        row1.insert("value".to_string(), CellValue::Number(10.0));
        let mut row2 = HashMap::new();
        row2.insert("value".to_string(), CellValue::Number(20.0));
        
        let data = vec![&row1, &row2];
        let result = generator.aggregate_field(&data, "value", &PivotAggregation::Count);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_aggregate_max() {
        let generator = PivotGenerator::new();
        let mut row1 = HashMap::new();
        row1.insert("value".to_string(), CellValue::Number(10.0));
        let mut row2 = HashMap::new();
        row2.insert("value".to_string(), CellValue::Number(20.0));
        
        let data = vec![&row1, &row2];
        let result = generator.aggregate_field(&data, "value", &PivotAggregation::Max);
        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_aggregate_min() {
        let generator = PivotGenerator::new();
        let mut row1 = HashMap::new();
        row1.insert("value".to_string(), CellValue::Number(10.0));
        let mut row2 = HashMap::new();
        row2.insert("value".to_string(), CellValue::Number(20.0));
        
        let data = vec![&row1, &row2];
        let result = generator.aggregate_field(&data, "value", &PivotAggregation::Min);
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_aggregate_empty() {
        let generator = PivotGenerator::new();
        let data: Vec<&HashMap<String, CellValue>> = vec![];
        let result = generator.aggregate_field(&data, "value", &PivotAggregation::Sum);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_pivot_generator_default() {
        let generator = PivotGenerator::default();
        // PivotGenerator is a unit struct, just verify it exists
        assert!(true);
    }
}
