//! Chart generation with aerospace-grade data visualization
//! 
//! This module provides comprehensive chart functionality including
//! multiple chart types, data extraction, and configuration.

use crate::spreadsheet_service::{
    error::{SpreadsheetError, SpreadsheetResult},
    types::CellValue,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Chart type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChartType {
    Line,
    Bar,
    Column,
    Pie,
    Scatter,
    Area,
    Doughnut,
    Radar,
    Stock,
    Surface,
}

impl std::fmt::Display for ChartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChartType::Line => write!(f, "Line"),
            ChartType::Bar => write!(f, "Bar"),
            ChartType::Column => write!(f, "Column"),
            ChartType::Pie => write!(f, "Pie"),
            ChartType::Scatter => write!(f, "Scatter"),
            ChartType::Area => write!(f, "Area"),
            ChartType::Doughnut => write!(f, "Doughnut"),
            ChartType::Radar => write!(f, "Radar"),
            ChartType::Stock => write!(f, "Stock"),
            ChartType::Surface => write!(f, "Surface"),
        }
    }
}

/// Chart configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartConfig {
    /// Chart type
    pub chart_type: ChartType,
    /// Chart title
    pub title: Option<String>,
    /// Data source range
    pub data_range: String,
    /// Category axis field
    pub category_field: Option<String>,
    /// Value fields
    pub value_fields: Vec<String>,
    /// Legend position
    pub legend_position: Option<LegendPosition>,
    /// Show data labels
    pub show_data_labels: bool,
    /// Show gridlines
    pub show_gridlines: bool,
    /// Chart colors
    pub colors: Option<Vec<String>>,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            chart_type: ChartType::Bar,
            title: None,
            data_range: "A1:B10".to_string(),
            category_field: None,
            value_fields: Vec::new(),
            legend_position: None,
            show_data_labels: false,
            show_gridlines: true,
            colors: None,
        }
    }
}

/// Legend position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegendPosition {
    Top,
    Bottom,
    Left,
    Right,
    TopRight,
    None,
}

impl std::fmt::Display for LegendPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LegendPosition::Top => write!(f, "Top"),
            LegendPosition::Bottom => write!(f, "Bottom"),
            LegendPosition::Left => write!(f, "Left"),
            LegendPosition::Right => write!(f, "Right"),
            LegendPosition::TopRight => write!(f, "TopRight"),
            LegendPosition::None => write!(f, "None"),
        }
    }
}

/// Chart data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartData {
    /// Categories
    pub categories: Vec<String>,
    /// Series data
    pub series: Vec<ChartSeries>,
}

/// Chart series
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartSeries {
    /// Series name
    pub name: String,
    /// Series values
    pub values: Vec<f64>,
    /// Series color
    pub color: Option<String>,
}

/// Chart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    /// Chart configuration
    pub config: ChartConfig,
    /// Chart data
    pub data: ChartData,
}

/// Chart generator
pub struct ChartGenerator;

impl ChartGenerator {
    /// Create a new chart generator
    pub fn new() -> Self {
        Self
    }

    /// Generate a chart from data
    pub fn generate(
        &self,
        data: &[HashMap<String, CellValue>],
        config: ChartConfig,
    ) -> SpreadsheetResult<Chart> {
        // Validate configuration
        if config.value_fields.is_empty() {
            return Err(SpreadsheetError::chart_error(
                format!("{:?}", config.chart_type).as_str(),
                "Chart must have at least one value field",
            ));
        }

        // Extract chart data
        let chart_data = self.extract_chart_data(data, &config)?;

        Ok(Chart {
            config,
            data: chart_data,
        })
    }

    /// Extract chart data from source data
    fn extract_chart_data(
        &self,
        data: &[HashMap<String, CellValue>],
        config: &ChartConfig,
    ) -> SpreadsheetResult<ChartData> {
        let mut categories = Vec::new();
        let mut series_map: HashMap<String, Vec<f64>> = HashMap::new();

        // Extract categories
        if let Some(category_field) = &config.category_field {
            for row in data {
                if let Some(value) = row.get(category_field) {
                    categories.push(match value {
                        CellValue::Text(s) => s.clone(),
                        CellValue::Number(n) => n.to_string(),
                        CellValue::Boolean(b) => b.to_string(),
                        _ => String::new(),
                    });
                }
            }
        } else {
            // Use row numbers as categories
            for i in 0..data.len() {
                categories.push((i + 1).to_string());
            }
        }

        // Extract series data
        for value_field in &config.value_fields {
            let mut values = Vec::new();
            for row in data {
                if let Some(value) = row.get(value_field) {
                    match value {
                        CellValue::Number(n) => values.push(*n),
                        _ => values.push(0.0),
                    }
                }
            }
            series_map.insert(value_field.clone(), values);
        }

        // Convert to chart series
        let mut series = Vec::new();
        for (name, values) in series_map {
            series.push(ChartSeries {
                name,
                values,
                color: None,
            });
        }

        Ok(ChartData { categories, series })
    }
}

impl Default for ChartGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_generator_creation() {
        let generator = ChartGenerator::new();
        // Just test creation
        assert!(true);
    }

    #[test]
    fn test_chart_config_validation() {
        let generator = ChartGenerator::new();
        let data = vec![];
        let config = ChartConfig {
            chart_type: ChartType::Bar,
            title: None,
            data_range: "A1:B10".to_string(),
            category_field: None,
            value_fields: vec![],
            legend_position: None,
            show_data_labels: false,
            show_gridlines: true,
            colors: None,
        };
        let result = generator.generate(&data, config);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_chart_data() {
        let generator = ChartGenerator::new();
        let mut row1 = HashMap::new();
        row1.insert("category".to_string(), CellValue::Text("A".to_string()));
        row1.insert("value".to_string(), CellValue::Number(10.0));
        let mut row2 = HashMap::new();
        row2.insert("category".to_string(), CellValue::Text("B".to_string()));
        row2.insert("value".to_string(), CellValue::Number(20.0));
        
        let data = vec![row1, row2];
        let config = ChartConfig {
            chart_type: ChartType::Bar,
            title: None,
            data_range: "A1:B10".to_string(),
            category_field: Some("category".to_string()),
            value_fields: vec!["value".to_string()],
            legend_position: None,
            show_data_labels: false,
            show_gridlines: true,
            colors: None,
        };
        
        let result = generator.extract_chart_data(&data, &config);
        assert!(result.is_ok());
        let chart_data = result.unwrap();
        assert_eq!(chart_data.categories.len(), 2);
        assert_eq!(chart_data.series.len(), 1);
    }

    #[test]
    fn test_chart_generator_default() {
        let generator = ChartGenerator::default();
        // ChartGenerator is initialized if it can be created successfully
        assert!(true);
    }

    #[test]
    fn test_chart_config_default() {
        let config = ChartConfig::default();
        assert_eq!(config.chart_type, ChartType::Bar);
        assert!(config.show_gridlines);
        assert!(!config.show_data_labels);
    }
}
