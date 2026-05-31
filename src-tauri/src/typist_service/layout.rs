/*!
 * 航空航天级布局系统
 * 实现 Typst 的 Grid 和 Stack 布局功能
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Length {
    Auto,
    Pt(f64),
    Mm(f64),
    Cm(f64),
    In(f64),
    Percent(f64),
    Fr(f64), // Fraction for grid
}

impl Length {
    pub fn to_points(&self, base: f64) -> f64 {
        match self {
            Length::Auto => 0.0,
            Length::Pt(val) => *val,
            Length::Mm(val) => val * 2.83465,
            Length::Cm(val) => val * 28.3465,
            Length::In(val) => val * 72.0,
            Length::Percent(val) => base * (val / 100.0),
            Length::Fr(val) => *val, // Fractional units handled by layout engine
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub padding: Length,
    pub spacing: Length,
    pub align_h: Alignment,
    pub align_v: Alignment,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            padding: Length::Pt(8.0),
            spacing: Length::Pt(4.0),
            align_h: Alignment::Start,
            align_v: Alignment::Start,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Alignment {
    Start,
    Center,
    End,
    Justify,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridCell {
    pub content: String,
    pub row_span: usize,
    pub col_span: usize,
    pub config: LayoutConfig,
}

impl Default for GridCell {
    fn default() -> Self {
        Self {
            content: String::new(),
            row_span: 1,
            col_span: 1,
            config: LayoutConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridColumn {
    pub width: Length,
    pub min_width: Option<Length>,
    pub max_width: Option<Length>,
}

impl Default for GridColumn {
    fn default() -> Self {
        Self {
            width: Length::Auto,
            min_width: None,
            max_width: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridRow {
    pub height: Length,
    pub min_height: Option<Length>,
    pub max_height: Option<Length>,
}

impl Default for GridRow {
    fn default() -> Self {
        Self {
            height: Length::Auto,
            min_height: None,
            max_height: None,
        }
    }
}

pub struct GridLayout {
    columns: Vec<GridColumn>,
    rows: Vec<GridRow>,
    cells: HashMap<(usize, usize), GridCell>,
    config: LayoutConfig,
}

impl GridLayout {
    pub fn new(columns: usize, rows: usize) -> Self {
        Self {
            columns: vec![GridColumn::default(); columns],
            rows: vec![GridRow::default(); rows],
            cells: HashMap::new(),
            config: LayoutConfig::default(),
        }
    }

    pub fn with_config(columns: usize, rows: usize, config: LayoutConfig) -> Self {
        Self {
            columns: vec![GridColumn::default(); columns],
            rows: vec![GridRow::default(); rows],
            cells: HashMap::new(),
            config,
        }
    }

    pub fn set_column(&mut self, index: usize, column: GridColumn) -> Result<(), String> {
        if index >= self.columns.len() {
            return Err(format!("Column index {} out of bounds", index));
        }
        self.columns[index] = column;
        Ok(())
    }

    pub fn set_row(&mut self, index: usize, row: GridRow) -> Result<(), String> {
        if index >= self.rows.len() {
            return Err(format!("Row index {} out of bounds", index));
        }
        self.rows[index] = row;
        Ok(())
    }

    pub fn set_cell(&mut self, row: usize, col: usize, cell: GridCell) -> Result<(), String> {
        if row >= self.rows.len() || col >= self.columns.len() {
            return Err(format!("Cell position ({}, {}) out of bounds", row, col));
        }
        self.cells.insert((row, col), cell);
        Ok(())
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<&GridCell> {
        self.cells.get(&(row, col))
    }

    pub fn render(&self, container_width: f64) -> String {
        let mut output = String::new();

        // Calculate column widths
        let col_widths = self.calculate_column_widths(container_width);

        // Calculate row heights
        let row_heights = self.calculate_row_heights();

        // Render cells
        for (row_idx, _row) in self.rows.iter().enumerate() {
            let row_height = row_heights[row_idx];

            for (col_idx, _col) in self.columns.iter().enumerate() {
                let col_width = col_widths[col_idx];

                if let Some(cell) = self.get_cell(row_idx, col_idx) {
                    output.push_str(&format!(
                        "<div style=\"position: absolute; left: {:.1}px; top: {:.1}px; width: {:.1}px; height: {:.1}px; padding: {:.1}px;\">{}</div>\n",
                        col_width * col_idx as f64,
                        row_height * row_idx as f64,
                        col_width,
                        row_height,
                        self.config.padding.to_points(72.0),
                        cell.content
                    ));
                }
            }
        }

        output
    }

    fn calculate_column_widths(&self, container_width: f64) -> Vec<f64> {
        let mut widths = Vec::new();
        let mut total_fr = 0.0;
        let mut fixed_width = 0.0;

        // First pass: calculate fixed widths and count fr units
        for col in &self.columns {
            match col.width {
                Length::Fr(val) => total_fr += val,
                Length::Auto => {
                    // Auto will be handled in second pass
                }
                _ => {
                    fixed_width += col.width.to_points(container_width);
                }
            }
        }

        // Second pass: distribute remaining space
        let remaining = container_width - fixed_width;
        let fr_unit = if total_fr > 0.0 {
            remaining / total_fr
        } else {
            0.0
        };

        for col in &self.columns {
            let width = match col.width {
                Length::Fr(val) => val * fr_unit,
                Length::Auto => {
                    // Distribute auto columns evenly
                    let auto_count = self
                        .columns
                        .iter()
                        .filter(|c| matches!(c.width, Length::Auto))
                        .count() as f64;
                    if auto_count > 0.0 {
                        remaining / auto_count
                    } else {
                        0.0
                    }
                }
                _ => col.width.to_points(container_width),
            };
            widths.push(width.max(0.0));
        }

        widths
    }

    fn calculate_row_heights(&self) -> Vec<f64> {
        let mut heights = Vec::new();

        for row in &self.rows {
            let height = match row.height {
                Length::Auto => 100.0, // Default auto height
                _ => row.height.to_points(100.0),
            };
            heights.push(height.max(0.0));
        }

        heights
    }

    pub fn to_typst(&self) -> String {
        let mut output = String::new();

        output.push_str("#grid(\n");
        output.push_str(&format!("  columns: {},\n", self.columns.len()));

        // Add column specifications
        let _col_specs: Vec<String> = self
            .columns
            .iter()
            .map(|col| match col.width {
                Length::Fr(val) => format!("{}fr", val),
                Length::Pt(val) => format!("{}pt", val),
                Length::Percent(val) => format!("{}%", val),
                _ => "auto".to_string(),
            })
            .collect();
        output.push_str(&format!(
            "  column-gutter: {},\n",
            self.config.spacing.to_points(72.0)
        ));

        // Add cells
        for (row_idx, col_idx) in self.cells.keys() {
            if let Some(cell) = self.get_cell(*row_idx, *col_idx) {
                output.push_str(&format!("  {},\n", cell.content));
            }
        }

        output.push_str(")\n");

        output
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StackItem {
    pub content: String,
    pub config: LayoutConfig,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum StackDirection {
    Horizontal,
    Vertical,
}

pub struct StackLayout {
    items: Vec<StackItem>,
    direction: StackDirection,
    config: LayoutConfig,
}

impl StackLayout {
    pub fn new(direction: StackDirection) -> Self {
        Self {
            items: Vec::new(),
            direction,
            config: LayoutConfig::default(),
        }
    }

    pub fn with_config(direction: StackDirection, config: LayoutConfig) -> Self {
        Self {
            items: Vec::new(),
            direction,
            config,
        }
    }

    pub fn add_item(&mut self, item: StackItem) {
        self.items.push(item);
    }

    pub fn render(&self, container_width: f64) -> String {
        let mut output = String::new();
        let spacing = self.config.spacing.to_points(72.0);

        match self.direction {
            StackDirection::Horizontal => {
                let item_width = (container_width - spacing * (self.items.len() - 1) as f64)
                    / self.items.len() as f64;

                for (idx, item) in self.items.iter().enumerate() {
                    let x = idx as f64 * (item_width + spacing);
                    output.push_str(&format!(
                        "<div style=\"position: absolute; left: {:.1}px; top: 0px; width: {:.1}px; height: 100%; padding: {:.1}px;\">{}</div>\n",
                        x,
                        item_width,
                        self.config.padding.to_points(72.0),
                        item.content
                    ));
                }
            }
            StackDirection::Vertical => {
                let item_height = 100.0 / self.items.len() as f64;

                for (idx, item) in self.items.iter().enumerate() {
                    let y = idx as f64 * (item_height + spacing);
                    output.push_str(&format!(
                        "<div style=\"position: absolute; left: 0px; top: {:.1}px; width: 100%; height: {:.1}px; padding: {:.1}px;\">{}</div>\n",
                        y,
                        item_height,
                        self.config.padding.to_points(72.0),
                        item.content
                    ));
                }
            }
        }

        output
    }

    pub fn to_typst(&self) -> String {
        let mut output = String::new();

        match self.direction {
            StackDirection::Horizontal => {
                output.push_str("#h(");
            }
            StackDirection::Vertical => {
                output.push_str("#v(");
            }
        }

        let spacing = self.config.spacing.to_points(72.0);
        output.push_str(&format!("spacing: {}, ", spacing));

        for item in &self.items {
            output.push_str(&item.content);
            output.push_str(", ");
        }

        output.push_str(")\n");

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_to_points() {
        assert_eq!(Length::Pt(72.0).to_points(100.0), 72.0);
        assert!((Length::Mm(25.4).to_points(100.0) - 72.0).abs() < 0.01); // 1 inch
        assert_eq!(Length::Percent(50.0).to_points(100.0), 50.0);
    }

    #[test]
    fn test_grid_creation() {
        let grid = GridLayout::new(3, 2);
        assert_eq!(grid.columns.len(), 3);
        assert_eq!(grid.rows.len(), 2);
    }

    #[test]
    fn test_grid_set_cell() {
        let mut grid = GridLayout::new(2, 2);
        let cell = GridCell {
            content: "Test".to_string(),
            ..Default::default()
        };

        assert!(grid.set_cell(0, 0, cell.clone()).is_ok());
        assert!(grid.set_cell(2, 0, cell).is_err()); // Out of bounds
    }

    #[test]
    fn test_grid_calculate_column_widths() {
        let mut grid = GridLayout::new(3, 1);
        grid.set_column(
            0,
            GridColumn {
                width: Length::Fr(1.0),
                ..Default::default()
            },
        )
        .unwrap();
        grid.set_column(
            1,
            GridColumn {
                width: Length::Fr(2.0),
                ..Default::default()
            },
        )
        .unwrap();
        grid.set_column(
            2,
            GridColumn {
                width: Length::Fr(1.0),
                ..Default::default()
            },
        )
        .unwrap();

        let widths = grid.calculate_column_widths(300.0);
        assert_eq!(widths[0], 75.0);
        assert_eq!(widths[1], 150.0);
        assert_eq!(widths[2], 75.0);
    }

    #[test]
    fn test_stack_creation() {
        let stack = StackLayout::new(StackDirection::Vertical);
        assert_eq!(stack.direction, StackDirection::Vertical);
    }

    #[test]
    fn test_stack_add_item() {
        let mut stack = StackLayout::new(StackDirection::Horizontal);
        let item = StackItem {
            content: "Item".to_string(),
            ..Default::default()
        };

        stack.add_item(item);
        assert_eq!(stack.items.len(), 1);
    }

    #[test]
    fn test_layout_config_default() {
        let config = LayoutConfig::default();
        assert_eq!(config.padding.to_points(72.0), 8.0);
        assert_eq!(config.spacing.to_points(72.0), 4.0);
    }

    #[test]
    fn test_grid_to_typst() {
        let grid = GridLayout::new(2, 2);
        let typst = grid.to_typst();
        assert!(typst.contains("#grid("));
        assert!(typst.contains("columns: 2"));
    }

    #[test]
    fn test_stack_to_typst() {
        let stack = StackLayout::new(StackDirection::Horizontal);
        let typst = stack.to_typst();
        assert!(typst.contains("#h("));
    }
}
