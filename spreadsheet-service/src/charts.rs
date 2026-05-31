//! Charts module
//! Provides data models and services for spreadsheet charts

use serde::{Deserialize, Serialize};
use crate::error::{SpreadsheetError, SpreadsheetResult};

/// Chart definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    pub id: String,
    pub sheet_id: String,
    pub name: String,
    pub chart_type: ChartType,
    pub data_range: String, // e.g., "A1:B10"
    pub title: Option<String>,
    pub x_axis_title: Option<String>,
    pub y_axis_title: Option<String>,
    pub legend_position: Option<LegendPosition>,
    pub style: ChartStyle,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Chart type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ChartType {
    /// Line chart
    Line,
    /// Bar chart
    Bar { orientation: BarOrientation },
    /// Column chart
    Column,
    /// Pie chart
    Pie,
    /// Scatter plot
    Scatter,
    /// Area chart
    Area,
    /// Doughnut chart
    Doughnut,
    /// Radar chart
    Radar,
}

/// Bar orientation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BarOrientation {
    Vertical,
    Horizontal,
}

/// Legend position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegendPosition {
    Top,
    Bottom,
    Left,
    Right,
    None,
}

/// Chart style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartStyle {
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub background_color: Option<String>,
    pub border_color: Option<String>,
    pub border_width: Option<i32>,
    pub show_gridlines: Option<bool>,
    pub show_data_labels: Option<bool>,
}

impl Default for ChartStyle {
    fn default() -> Self {
        Self {
            width: Some(400),
            height: Some(300),
            background_color: None,
            border_color: None,
            border_width: None,
            show_gridlines: Some(true),
            show_data_labels: Some(false),
        }
    }
}

/// Chart data series
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartSeries {
    pub name: String,
    pub values: Vec<f64>,
    pub color: Option<String>,
}

/// Chart data for rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartData {
    pub chart: Chart,
    pub series: Vec<ChartSeries>,
    pub categories: Vec<String>,
}

/// Chart service
pub struct ChartService;

impl ChartService {
    /// Parse data range and extract chart data
    pub fn extract_chart_data(
        chart: &Chart,
        cell_values: &[(i32, i32, String)], // (row, col, value)
    ) -> SpreadsheetResult<ChartData> {
        let (start_col, start_row, end_col, end_row) = Self::parse_range(&chart.data_range)?;

        // Extract categories (first column) and series (remaining columns)
        let mut categories = Vec::new();
        let mut series_data: Vec<Vec<f64>> = Vec::new();

        for row in start_row..=end_row {
            // Get category from first column
            if let Some((_, _, value)) = cell_values.iter().find(|(r, c, _)| *r == row && *c == start_col) {
                categories.push(value.clone());
            } else {
                categories.push(format!("Row {}", row + 1));
            }

            // Get series values from remaining columns
            for col in (start_col + 1)..=end_col {
                let series_index = (col - start_col - 1) as usize;
                if series_index >= series_data.len() {
                    series_data.push(Vec::new());
                }

                if let Some((_, _, value)) = cell_values.iter().find(|(r, c, _)| *r == row && *c == col) {
                    if let Ok(num) = value.parse::<f64>() {
                        series_data[series_index].push(num);
                    } else {
                        series_data[series_index].push(0.0);
                    }
                } else {
                    series_data[series_index].push(0.0);
                }
            }
        }

        // Create series
        let series: Vec<ChartSeries> = series_data
            .iter()
            .enumerate()
            .map(|(i, values)| ChartSeries {
                name: format!("Series {}", i + 1),
                values: values.clone(),
                color: Self::get_default_color(i),
            })
            .collect();

        Ok(ChartData {
            chart: chart.clone(),
            series,
            categories,
        })
    }

    /// Parse cell range (e.g., "A1:B10")
    fn parse_range(range: &str) -> SpreadsheetResult<(i32, i32, i32, i32)> {
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

    /// Get default color for series index
    fn get_default_color(index: usize) -> Option<String> {
        let colors = vec![
            "#4472C4", "#ED7D31", "#A5A5A5", "#FFC000", "#5B9BD5",
            "#70AD47", "#FF0000", "#800080", "#00B0F0", "#008000",
        ];
        colors.get(index % colors.len()).map(|s| s.to_string())
    }

    /// Validate chart configuration
    pub fn validate_chart(chart: &Chart) -> SpreadsheetResult<()> {
        if chart.name.is_empty() {
            return Err(SpreadsheetError::Validation("Chart name cannot be empty".to_string()));
        }

        // Validate data range
        Self::parse_range(&chart.data_range)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cell_reference() {
        assert_eq!(ChartService::parse_cell_reference("A1").unwrap(), (0, 0));
        assert_eq!(ChartService::parse_cell_reference("B2").unwrap(), (1, 1));
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(
            ChartService::parse_range("A1:B10").unwrap(),
            (0, 0, 1, 9)
        );
    }

    #[test]
    fn test_extract_chart_data() {
        let chart = Chart {
            id: "1".to_string(),
            sheet_id: "sheet1".to_string(),
            name: "Test Chart".to_string(),
            chart_type: ChartType::Line,
            data_range: "A1:B3".to_string(),
            title: Some("Test".to_string()),
            x_axis_title: None,
            y_axis_title: None,
            legend_position: None,
            style: ChartStyle::default(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let cell_values = vec![
            (0, 0, "Category 1".to_string()),
            (0, 1, "10".to_string()),
            (1, 0, "Category 2".to_string()),
            (1, 1, "20".to_string()),
            (2, 0, "Category 3".to_string()),
            (2, 1, "30".to_string()),
        ];

        let chart_data = ChartService::extract_chart_data(&chart, &cell_values).unwrap();
        assert_eq!(chart_data.categories.len(), 3);
        assert_eq!(chart_data.series.len(), 1);
        assert_eq!(chart_data.series[0].values, vec![10.0, 20.0, 30.0]);
    }

    #[test]
    fn test_validate_chart() {
        let chart = Chart {
            id: "1".to_string(),
            sheet_id: "sheet1".to_string(),
            name: "Test Chart".to_string(),
            chart_type: ChartType::Line,
            data_range: "A1:B10".to_string(),
            title: None,
            x_axis_title: None,
            y_axis_title: None,
            legend_position: None,
            style: ChartStyle::default(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        assert!(ChartService::validate_chart(&chart).is_ok());
    }
}
