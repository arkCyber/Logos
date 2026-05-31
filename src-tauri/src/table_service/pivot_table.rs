use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PivotAggregation {
    Sum,
    Average,
    Count,
    Min,
    Max,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotConfig {
    pub rows: Vec<String>,
    pub columns: Vec<String>,
    pub values: Vec<PivotValue>,
    pub filters: Option<Vec<PivotFilter>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotValue {
    pub field: String,
    pub aggregation: PivotAggregation,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotFilter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    StartsWith,
    EndsWith,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PivotTable {
    pub data: Vec<HashMap<String, String>>,
    pub config: PivotConfig,
    pub result: PivotResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PivotResult {
    pub headers: Vec<String>,
    pub rows: Vec<PivotRow>,
    pub grand_total: Option<HashMap<String, f64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PivotRow {
    pub row_labels: Vec<String>,
    pub values: HashMap<String, f64>,
    pub row_total: f64,
}

pub struct PivotTableGenerator {
    // In production, this would use a proper data analysis library
}

impl PivotTableGenerator {
    pub fn new() -> Self {
        Self {}
    }

    /// Generate a pivot table from data
    pub fn generate(
        &self,
        data: Vec<HashMap<String, String>>,
        config: PivotConfig,
    ) -> Result<PivotTable, String> {
        // Validate configuration
        if config.rows.is_empty() && config.columns.is_empty() {
            return Err("Pivot table must have at least one row or column".to_string());
        }

        if config.values.is_empty() {
            return Err("Pivot table must have at least one value field".to_string());
        }

        // Apply filters
        let filtered_data = self.apply_filters(&data, &config.filters)?;

        // Group data by row and column dimensions
        let grouped = self.group_data(&filtered_data, &config);

        // Calculate aggregations
        let result = self.calculate_aggregations(&grouped, &config);

        Ok(PivotTable {
            data,
            config,
            result,
        })
    }

    fn apply_filters(
        &self,
        data: &[HashMap<String, String>],
        filters: &Option<Vec<PivotFilter>>,
    ) -> Result<Vec<HashMap<String, String>>, String> {
        let filters = filters.as_ref().map(|f| f.as_slice()).unwrap_or(&[]);

        let mut filtered = Vec::new();

        for row in data {
            let mut include = true;

            for filter in filters {
                if let Some(value) = row.get(&filter.field) {
                    let matches = match filter.operator {
                        FilterOperator::Equals => value == &filter.value,
                        FilterOperator::NotEquals => value != &filter.value,
                        FilterOperator::GreaterThan => {
                            let row_val = value.parse::<f64>().unwrap_or(f64::NEG_INFINITY);
                            let filter_val =
                                filter.value.parse::<f64>().unwrap_or(f64::NEG_INFINITY);
                            row_val > filter_val
                        }
                        FilterOperator::LessThan => {
                            let row_val = value.parse::<f64>().unwrap_or(f64::INFINITY);
                            let filter_val = filter.value.parse::<f64>().unwrap_or(f64::INFINITY);
                            row_val < filter_val
                        }
                        FilterOperator::Contains => value.contains(&filter.value),
                        FilterOperator::StartsWith => value.starts_with(&filter.value),
                        FilterOperator::EndsWith => value.ends_with(&filter.value),
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

    fn group_data<'a>(
        &self,
        data: &'a [HashMap<String, String>],
        config: &PivotConfig,
    ) -> HashMap<Vec<String>, Vec<&'a HashMap<String, String>>> {
        let mut grouped: HashMap<Vec<String>, Vec<&'a HashMap<String, String>>> = HashMap::new();

        for row in data {
            let mut key = Vec::new();

            // Group by row fields
            for field in &config.rows {
                key.push(row.get(field).cloned().unwrap_or_else(|| String::new()));
            }

            // Also group by column fields if present
            for field in &config.columns {
                key.push(row.get(field).cloned().unwrap_or_else(|| String::new()));
            }

            grouped.entry(key).or_insert_with(Vec::new).push(row);
        }

        grouped
    }

    fn calculate_aggregations(
        &self,
        grouped: &HashMap<Vec<String>, Vec<&HashMap<String, String>>>,
        config: &PivotConfig,
    ) -> PivotResult {
        let mut headers = Vec::new();
        let mut rows = Vec::new();
        let mut grand_total: HashMap<String, f64> = HashMap::new();

        // Build headers from row fields
        for field in &config.rows {
            headers.push(field.clone());
        }

        // Build headers from column fields
        for field in &config.columns {
            headers.push(field.clone());
        }

        // Build headers from value fields
        for value in &config.values {
            headers.push(value.name.as_ref().unwrap_or(&value.field).clone());
        }

        headers.push("Total".to_string());

        // Calculate row aggregations
        for (key_labels, group_data) in grouped {
            let mut values = HashMap::new();
            let mut row_total = 0.0;

            // Separate row labels from column labels
            let row_labels: Vec<String> =
                key_labels.iter().take(config.rows.len()).cloned().collect();

            let column_labels: Vec<String> =
                key_labels.iter().skip(config.rows.len()).cloned().collect();

            for pivot_value in &config.values {
                let aggregated =
                    self.aggregate_field(group_data, &pivot_value.field, &pivot_value.aggregation);

                // Create a unique key for this value based on column labels
                let value_key = if column_labels.is_empty() {
                    pivot_value
                        .name
                        .as_ref()
                        .unwrap_or(&pivot_value.field)
                        .clone()
                } else {
                    format!(
                        "{}-{}",
                        column_labels.join("-"),
                        pivot_value.name.as_ref().unwrap_or(&pivot_value.field)
                    )
                };

                values.insert(value_key.clone(), aggregated);
                row_total += aggregated;

                // Add to grand total
                *grand_total
                    .entry(
                        pivot_value
                            .name
                            .as_ref()
                            .unwrap_or(&pivot_value.field)
                            .clone(),
                    )
                    .or_insert(0.0) += aggregated;
            }

            rows.push(PivotRow {
                row_labels,
                values,
                row_total,
            });
        }

        // Sort rows by row labels for consistent output
        rows.sort_by(|a, b| a.row_labels.cmp(&b.row_labels));

        PivotResult {
            headers,
            rows,
            grand_total: Some(grand_total),
        }
    }

    fn aggregate_field(
        &self,
        data: &[&HashMap<String, String>],
        field: &str,
        aggregation: &PivotAggregation,
    ) -> f64 {
        let values: Vec<f64> = data
            .iter()
            .filter_map(|row| row.get(field))
            .filter_map(|v| v.parse::<f64>().ok())
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
            PivotAggregation::Count => data.len() as f64, // Count all rows, not just numeric values
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
        }
    }
}

impl Default for PivotTableGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pivot_generation() {
        let generator = PivotTableGenerator::new();

        let data = vec![
            {
                let mut row = HashMap::new();
                row.insert("category".to_string(), "A".to_string());
                row.insert("value".to_string(), "10".to_string());
                row
            },
            {
                let mut row = HashMap::new();
                row.insert("category".to_string(), "B".to_string());
                row.insert("value".to_string(), "20".to_string());
                row
            },
        ];

        let config = PivotConfig {
            rows: vec!["category".to_string()],
            columns: vec![],
            values: vec![PivotValue {
                field: "value".to_string(),
                aggregation: PivotAggregation::Sum,
                name: None,
            }],
            filters: None,
        };

        let result = generator.generate(data, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generator_creation() {
        let generator = PivotTableGenerator::new();
        let data = vec![{
            let mut row = HashMap::new();
            row.insert("test".to_string(), "value".to_string());
            row
        }];
        let result = generator.generate(
            data,
            PivotConfig {
                rows: vec!["test".to_string()],
                columns: vec![],
                values: vec![PivotValue {
                    field: "value".to_string(),
                    aggregation: PivotAggregation::Sum,
                    name: None,
                }],
                filters: None,
            },
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_generator_default() {
        let generator = PivotTableGenerator::default();
        let data = vec![{
            let mut row = HashMap::new();
            row.insert("test".to_string(), "value".to_string());
            row
        }];
        let result = generator.generate(
            data,
            PivotConfig {
                rows: vec!["test".to_string()],
                columns: vec![],
                values: vec![PivotValue {
                    field: "value".to_string(),
                    aggregation: PivotAggregation::Sum,
                    name: None,
                }],
                filters: None,
            },
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_pivot_aggregation_variants() {
        let sum = PivotAggregation::Sum;
        let average = PivotAggregation::Average;
        let count = PivotAggregation::Count;
        let min = PivotAggregation::Min;
        let max = PivotAggregation::Max;

        let _ = (sum, average, count, min, max);
    }

    #[test]
    fn test_pivot_aggregation_serialization() {
        let agg = PivotAggregation::Sum;
        let json = serde_json::to_string(&agg);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"sum\"");
    }

    #[test]
    fn test_pivot_aggregation_deserialization() {
        let agg: PivotAggregation = serde_json::from_str("\"sum\"").unwrap();
        assert!(matches!(agg, PivotAggregation::Sum));
    }

    #[test]
    fn test_filter_operator_variants() {
        let equals = FilterOperator::Equals;
        let not_equals = FilterOperator::NotEquals;
        let greater_than = FilterOperator::GreaterThan;
        let less_than = FilterOperator::LessThan;
        let contains = FilterOperator::Contains;
        let starts_with = FilterOperator::StartsWith;
        let ends_with = FilterOperator::EndsWith;

        let _ = (
            equals,
            not_equals,
            greater_than,
            less_than,
            contains,
            starts_with,
            ends_with,
        );
    }

    #[test]
    fn test_filter_operator_serialization() {
        let op = FilterOperator::Equals;
        let json = serde_json::to_string(&op);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"equals\"");
    }

    #[test]
    fn test_filter_operator_deserialization() {
        let op: FilterOperator = serde_json::from_str("\"equals\"").unwrap();
        assert!(matches!(op, FilterOperator::Equals));
    }

    #[test]
    fn test_pivot_config_creation() {
        let config = PivotConfig {
            rows: vec!["category".to_string()],
            columns: vec!["date".to_string()],
            values: vec![PivotValue {
                field: "value".to_string(),
                aggregation: PivotAggregation::Sum,
                name: Some("Total".to_string()),
            }],
            filters: Some(vec![]),
        };
        assert_eq!(config.rows.len(), 1);
        assert_eq!(config.columns.len(), 1);
    }

    #[test]
    fn test_pivot_config_serialization() {
        let config = PivotConfig {
            rows: vec!["category".to_string()],
            columns: vec![],
            values: vec![PivotValue {
                field: "value".to_string(),
                aggregation: PivotAggregation::Sum,
                name: None,
            }],
            filters: None,
        };
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_pivot_config_deserialization() {
        let json = r#"{
            "rows": ["category"],
            "columns": [],
            "values": [{"field": "value", "aggregation": "sum", "name": null}],
            "filters": null
        }"#;
        let config: Result<PivotConfig, _> = serde_json::from_str(json);
        assert!(config.is_ok());
    }

    #[test]
    fn test_pivot_value_creation() {
        let value = PivotValue {
            field: "amount".to_string(),
            aggregation: PivotAggregation::Average,
            name: Some("Average Amount".to_string()),
        };
        assert_eq!(value.field, "amount");
        assert!(value.name.is_some());
    }

    #[test]
    fn test_pivot_value_serialization() {
        let value = PivotValue {
            field: "amount".to_string(),
            aggregation: PivotAggregation::Average,
            name: None,
        };
        let json = serde_json::to_string(&value);
        assert!(json.is_ok());
    }

    #[test]
    fn test_pivot_value_deserialization() {
        let json = r#"{
            "field": "amount",
            "aggregation": "average",
            "name": null
        }"#;
        let value: Result<PivotValue, _> = serde_json::from_str(json);
        assert!(value.is_ok());
    }

    #[test]
    fn test_pivot_filter_creation() {
        let filter = PivotFilter {
            field: "category".to_string(),
            operator: FilterOperator::Equals,
            value: "A".to_string(),
        };
        assert_eq!(filter.field, "category");
        assert_eq!(filter.value, "A");
    }

    #[test]
    fn test_pivot_filter_serialization() {
        let filter = PivotFilter {
            field: "category".to_string(),
            operator: FilterOperator::Equals,
            value: "A".to_string(),
        };
        let json = serde_json::to_string(&filter);
        assert!(json.is_ok());
    }

    #[test]
    fn test_pivot_filter_deserialization() {
        let json = r#"{
            "field": "category",
            "operator": "equals",
            "value": "A"
        }"#;
        let filter: Result<PivotFilter, _> = serde_json::from_str(json);
        assert!(filter.is_ok());
    }

    #[test]
    fn test_pivot_result_creation() {
        let result = PivotResult {
            headers: vec!["Category".to_string(), "Total".to_string()],
            rows: vec![],
            grand_total: Some(HashMap::new()),
        };
        assert_eq!(result.headers.len(), 2);
    }

    #[test]
    fn test_pivot_result_serialization() {
        let result = PivotResult {
            headers: vec!["Category".to_string()],
            rows: vec![],
            grand_total: None,
        };
        let json = serde_json::to_string(&result);
        assert!(json.is_ok());
    }

    #[test]
    fn test_pivot_result_deserialization() {
        let json = r#"{
            "headers": ["Category"],
            "rows": [],
            "grand_total": null
        }"#;
        let result: Result<PivotResult, _> = serde_json::from_str(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pivot_row_creation() {
        let mut values = HashMap::new();
        values.insert("Total".to_string(), 100.0);

        let row = PivotRow {
            row_labels: vec!["A".to_string()],
            values,
            row_total: 100.0,
        };
        assert_eq!(row.row_labels.len(), 1);
        assert_eq!(row.row_total, 100.0);
    }

    #[test]
    fn test_pivot_row_serialization() {
        let mut values = HashMap::new();
        values.insert("Total".to_string(), 100.0);

        let row = PivotRow {
            row_labels: vec!["A".to_string()],
            values,
            row_total: 100.0,
        };
        let json = serde_json::to_string(&row);
        assert!(json.is_ok());
    }

    #[test]
    fn test_pivot_row_deserialization() {
        let json = r#"{
            "row_labels": ["A"],
            "values": {"Total": 100.0},
            "row_total": 100.0
        }"#;
        let row: Result<PivotRow, _> = serde_json::from_str(json);
        assert!(row.is_ok());
    }

    #[test]
    fn test_generate_empty_rows_and_columns() {
        let generator = PivotTableGenerator::new();
        let data = vec![];
        let config = PivotConfig {
            rows: vec![],
            columns: vec![],
            values: vec![PivotValue {
                field: "value".to_string(),
                aggregation: PivotAggregation::Sum,
                name: None,
            }],
            filters: None,
        };
        let result = generator.generate(data, config);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_empty_values() {
        let generator = PivotTableGenerator::new();
        let data = vec![];
        let config = PivotConfig {
            rows: vec!["category".to_string()],
            columns: vec![],
            values: vec![],
            filters: None,
        };
        let result = generator.generate(data, config);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_with_average_aggregation() {
        let generator = PivotTableGenerator::new();

        let data = vec![
            {
                let mut row = HashMap::new();
                row.insert("category".to_string(), "A".to_string());
                row.insert("value".to_string(), "10".to_string());
                row
            },
            {
                let mut row = HashMap::new();
                row.insert("category".to_string(), "A".to_string());
                row.insert("value".to_string(), "20".to_string());
                row
            },
        ];

        let config = PivotConfig {
            rows: vec!["category".to_string()],
            columns: vec![],
            values: vec![PivotValue {
                field: "value".to_string(),
                aggregation: PivotAggregation::Average,
                name: None,
            }],
            filters: None,
        };

        let result = generator.generate(data, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_with_count_aggregation() {
        let generator = PivotTableGenerator::new();

        let data = vec![
            {
                let mut row = HashMap::new();
                row.insert("category".to_string(), "A".to_string());
                row.insert("value".to_string(), "10".to_string());
                row
            },
            {
                let mut row = HashMap::new();
                row.insert("category".to_string(), "A".to_string());
                row.insert("value".to_string(), "20".to_string());
                row
            },
        ];

        let config = PivotConfig {
            rows: vec!["category".to_string()],
            columns: vec![],
            values: vec![PivotValue {
                field: "value".to_string(),
                aggregation: PivotAggregation::Count,
                name: None,
            }],
            filters: None,
        };

        let result = generator.generate(data, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_with_min_aggregation() {
        let generator = PivotTableGenerator::new();

        let data = vec![
            {
                let mut row = HashMap::new();
                row.insert("category".to_string(), "A".to_string());
                row.insert("value".to_string(), "10".to_string());
                row
            },
            {
                let mut row = HashMap::new();
                row.insert("category".to_string(), "A".to_string());
                row.insert("value".to_string(), "20".to_string());
                row
            },
        ];

        let config = PivotConfig {
            rows: vec!["category".to_string()],
            columns: vec![],
            values: vec![PivotValue {
                field: "value".to_string(),
                aggregation: PivotAggregation::Min,
                name: None,
            }],
            filters: None,
        };

        let result = generator.generate(data, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_with_max_aggregation() {
        let generator = PivotTableGenerator::new();

        let data = vec![
            {
                let mut row = HashMap::new();
                row.insert("category".to_string(), "A".to_string());
                row.insert("value".to_string(), "10".to_string());
                row
            },
            {
                let mut row = HashMap::new();
                row.insert("category".to_string(), "A".to_string());
                row.insert("value".to_string(), "20".to_string());
                row
            },
        ];

        let config = PivotConfig {
            rows: vec!["category".to_string()],
            columns: vec![],
            values: vec![PivotValue {
                field: "value".to_string(),
                aggregation: PivotAggregation::Max,
                name: None,
            }],
            filters: None,
        };

        let result = generator.generate(data, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_with_filters() {
        let generator = PivotTableGenerator::new();

        let data = vec![
            {
                let mut row = HashMap::new();
                row.insert("category".to_string(), "A".to_string());
                row.insert("value".to_string(), "10".to_string());
                row
            },
            {
                let mut row = HashMap::new();
                row.insert("category".to_string(), "B".to_string());
                row.insert("value".to_string(), "20".to_string());
                row
            },
        ];

        let config = PivotConfig {
            rows: vec!["category".to_string()],
            columns: vec![],
            values: vec![PivotValue {
                field: "value".to_string(),
                aggregation: PivotAggregation::Sum,
                name: None,
            }],
            filters: Some(vec![PivotFilter {
                field: "category".to_string(),
                operator: FilterOperator::Equals,
                value: "A".to_string(),
            }]),
        };

        let result = generator.generate(data, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pivot_table_creation() {
        let data = vec![];
        let config = PivotConfig {
            rows: vec!["category".to_string()],
            columns: vec![],
            values: vec![PivotValue {
                field: "value".to_string(),
                aggregation: PivotAggregation::Sum,
                name: None,
            }],
            filters: None,
        };
        let result = PivotResult {
            headers: vec![],
            rows: vec![],
            grand_total: None,
        };

        let table = PivotTable {
            data,
            config,
            result,
        };
        assert!(table.data.is_empty());
    }

    #[test]
    fn test_pivot_table_serialization() {
        let data = vec![];
        let config = PivotConfig {
            rows: vec!["category".to_string()],
            columns: vec![],
            values: vec![PivotValue {
                field: "value".to_string(),
                aggregation: PivotAggregation::Sum,
                name: None,
            }],
            filters: None,
        };
        let result = PivotResult {
            headers: vec![],
            rows: vec![],
            grand_total: None,
        };

        let table = PivotTable {
            data,
            config,
            result,
        };
        let json = serde_json::to_string(&table);
        assert!(json.is_ok());
    }

    #[test]
    fn test_pivot_config_no_filters() {
        let config = PivotConfig {
            rows: vec!["category".to_string()],
            columns: vec![],
            values: vec![PivotValue {
                field: "value".to_string(),
                aggregation: PivotAggregation::Sum,
                name: None,
            }],
            filters: None,
        };
        assert!(config.filters.is_none());
    }

    #[test]
    fn test_pivot_config_with_filters() {
        let config = PivotConfig {
            rows: vec!["category".to_string()],
            columns: vec![],
            values: vec![PivotValue {
                field: "value".to_string(),
                aggregation: PivotAggregation::Sum,
                name: None,
            }],
            filters: Some(vec![PivotFilter {
                field: "category".to_string(),
                operator: FilterOperator::Equals,
                value: "A".to_string(),
            }]),
        };
        assert!(config.filters.is_some());
    }

    #[test]
    fn test_pivot_value_with_name() {
        let value = PivotValue {
            field: "amount".to_string(),
            aggregation: PivotAggregation::Sum,
            name: Some("Total Amount".to_string()),
        };
        assert_eq!(value.name.unwrap(), "Total Amount");
    }

    #[test]
    fn test_pivot_value_without_name() {
        let value = PivotValue {
            field: "amount".to_string(),
            aggregation: PivotAggregation::Sum,
            name: None,
        };
        assert!(value.name.is_none());
    }

    #[test]
    fn test_pivot_result_with_grand_total() {
        let mut grand_total = HashMap::new();
        grand_total.insert("Total".to_string(), 100.0);

        let result = PivotResult {
            headers: vec![],
            rows: vec![],
            grand_total: Some(grand_total),
        };
        assert!(result.grand_total.is_some());
    }

    #[test]
    fn test_pivot_result_without_grand_total() {
        let result = PivotResult {
            headers: vec![],
            rows: vec![],
            grand_total: None,
        };
        assert!(result.grand_total.is_none());
    }

    #[test]
    fn test_pivot_row_empty_labels() {
        let row = PivotRow {
            row_labels: vec![],
            values: HashMap::new(),
            row_total: 0.0,
        };
        assert!(row.row_labels.is_empty());
    }

    #[test]
    fn test_pivot_row_multiple_labels() {
        let row = PivotRow {
            row_labels: vec!["A".to_string(), "2024".to_string()],
            values: HashMap::new(),
            row_total: 0.0,
        };
        assert_eq!(row.row_labels.len(), 2);
    }

    #[test]
    fn test_generate_with_multiple_values() {
        let generator = PivotTableGenerator::new();

        let data = vec![{
            let mut row = HashMap::new();
            row.insert("category".to_string(), "A".to_string());
            row.insert("value1".to_string(), "10".to_string());
            row.insert("value2".to_string(), "20".to_string());
            row
        }];

        let config = PivotConfig {
            rows: vec!["category".to_string()],
            columns: vec![],
            values: vec![
                PivotValue {
                    field: "value1".to_string(),
                    aggregation: PivotAggregation::Sum,
                    name: None,
                },
                PivotValue {
                    field: "value2".to_string(),
                    aggregation: PivotAggregation::Sum,
                    name: None,
                },
            ],
            filters: None,
        };

        let result = generator.generate(data, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_with_columns() {
        let generator = PivotTableGenerator::new();

        let data = vec![{
            let mut row = HashMap::new();
            row.insert("category".to_string(), "A".to_string());
            row.insert("date".to_string(), "2024".to_string());
            row.insert("value".to_string(), "10".to_string());
            row
        }];

        let config = PivotConfig {
            rows: vec!["category".to_string()],
            columns: vec!["date".to_string()],
            values: vec![PivotValue {
                field: "value".to_string(),
                aggregation: PivotAggregation::Sum,
                name: None,
            }],
            filters: None,
        };

        let result = generator.generate(data, config);
        assert!(result.is_ok());
    }
}
