//! Core type definitions for the spreadsheet service
//! 
//! This module defines all fundamental data structures used throughout
//! the spreadsheet service with aerospace-grade serialization and validation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a cell reference (e.g., "A1", "Sheet1!B5")
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellReference {
    /// Optional sheet name
    pub sheet: Option<String>,
    /// Column letter (e.g., "A", "B", "AA")
    pub column: String,
    /// Row number (1-indexed)
    pub row: u32,
}

impl CellReference {
    /// Create a new cell reference
    pub fn new(column: String, row: u32) -> Self {
        Self {
            sheet: None,
            column,
            row,
        }
    }

    /// Create a cell reference with sheet name
    pub fn with_sheet(sheet: String, column: String, row: u32) -> Self {
        Self {
            sheet: Some(sheet),
            column,
            row,
        }
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match &self.sheet {
            Some(sheet) => format!("!{}{}{}", sheet, self.column, self.row),
            None => format!("{}{}", self.column, self.row),
        }
    }

    /// Parse from string representation
    pub fn from_str(s: &str) -> Result<Self, String> {
        let (sheet, rest) = if let Some(idx) = s.find('!') {
            (Some(s[..idx].to_string()), &s[idx + 1..])
        } else {
            (None, s)
        };

        let column_part: String = rest
            .chars()
            .take_while(|c| c.is_alphabetic())
            .collect();
        let row_part = &rest[column_part.len()..];

        if column_part.is_empty() || row_part.is_empty() {
            return Err("Invalid cell reference format".to_string());
        }

        let row = row_part
            .parse::<u32>()
            .map_err(|_| "Invalid row number".to_string())?;

        Ok(Self {
            sheet,
            column: column_part.to_string(),
            row,
        })
    }

    /// Convert to zero-based column index
    pub fn to_column_index(&self) -> Result<u32, String> {
        let mut index = 0u32;
        for c in self.column.chars() {
            if !c.is_ascii_alphabetic() {
                return Err("Invalid column letter".to_string());
            }
            index = index * 26 + (c.to_ascii_uppercase() as u32 - 'A' as u32 + 1);
        }
        Ok(index - 1)
    }

    /// Convert from zero-based column index to column letter
    pub fn from_column_index(index: u32) -> String {
        let mut index = index + 1;
        let mut result = String::new();
        while index > 0 {
            index -= 1;
            result.insert(0, (b'A' + (index % 26) as u8) as char);
            index /= 26;
        }
        result
    }
}

/// Represents a cell range (e.g., "A1:B10", "Sheet1!C5:D20")
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range {
    /// Starting cell reference
    pub start: CellReference,
    /// Ending cell reference
    pub end: CellReference,
}

impl Range {
    /// Create a new range
    pub fn new(start: CellReference, end: CellReference) -> Self {
        Self { start, end }
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.start.to_string(), self.end.to_string())
    }

    /// Parse from string representation
    pub fn from_str(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid range format".to_string());
        }

        let start = CellReference::from_str(parts[0])?;
        let end = CellReference::from_str(parts[1])?;

        Ok(Self { start, end })
    }

    /// Get all cell references in this range
    pub fn get_cells(&self) -> Vec<CellReference> {
        let mut cells = Vec::new();
        
        let start_col = self.start.to_column_index().unwrap_or(0);
        let end_col = self.end.to_column_index().unwrap_or(0);
        let start_row = self.start.row;
        let end_row = self.end.row;

        for row in start_row..=end_row {
            for col in start_col..=end_col {
                cells.push(CellReference {
                    sheet: self.start.sheet.clone(),
                    column: CellReference::from_column_index(col),
                    row,
                });
            }
        }

        cells
    }
}

/// Represents a workbook (collection of sheets)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workbook {
    /// Workbook name
    pub name: String,
    /// Sheets in the workbook
    pub sheets: Vec<Sheet>,
    /// Active sheet index
    pub active_sheet: usize,
    /// Workbook metadata
    pub metadata: WorkbookMetadata,
}

impl Default for Workbook {
    fn default() -> Self {
        Self {
            name: "Untitled".to_string(),
            sheets: Vec::new(),
            active_sheet: 0,
            metadata: WorkbookMetadata::default(),
        }
    }
}

/// Workbook metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbookMetadata {
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last modified timestamp
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Author
    pub author: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Custom properties
    pub properties: HashMap<String, String>,
}

impl Default for WorkbookMetadata {
    fn default() -> Self {
        let now = chrono::Utc::now();
        Self {
            created_at: now,
            modified_at: now,
            author: None,
            description: None,
            properties: HashMap::new(),
        }
    }
}

impl Workbook {
    /// Create a new workbook
    pub fn new(name: String) -> Self {
        Self {
            name,
            sheets: Vec::new(),
            active_sheet: 0,
            metadata: WorkbookMetadata::default(),
        }
    }

    /// Add a sheet to the workbook
    pub fn add_sheet(&mut self, sheet: Sheet) {
        self.sheets.push(sheet);
    }

    /// Get the active sheet
    pub fn get_active_sheet(&self) -> Option<&Sheet> {
        self.sheets.get(self.active_sheet)
    }

    /// Get a sheet by index
    pub fn get_sheet(&self, index: usize) -> Option<&Sheet> {
        self.sheets.get(index)
    }

    /// Get a sheet by name
    pub fn get_sheet_by_name(&self, name: &str) -> Option<&Sheet> {
        self.sheets.iter().find(|s| s.name == name)
    }
}

/// Represents a single sheet in a workbook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sheet {
    /// Sheet name
    pub name: String,
    /// Cells in the sheet (key: "row,col", value: Cell)
    pub cells: HashMap<String, Cell>,
    /// Sheet dimensions
    pub dimensions: SheetDimensions,
    /// Sheet visibility
    pub visibility: SheetVisibility,
    /// Sheet color (for tab)
    pub color: Option<String>,
}

/// Sheet dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetDimensions {
    /// Number of rows
    pub rows: u32,
    /// Number of columns
    pub columns: u32,
}

impl Default for SheetDimensions {
    fn default() -> Self {
        Self {
            rows: 1048576, // Excel max rows
            columns: 16384, // Excel max columns
        }
    }
}

/// Sheet visibility
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SheetVisibility {
    Visible,
    Hidden,
    VeryHidden,
}

impl Default for SheetVisibility {
    fn default() -> Self {
        Self::Visible
    }
}

/// Represents a cell with all its properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    /// Cell reference
    pub reference: CellReference,
    /// Cell value
    pub value: CellValue,
    /// Cell formula (if any)
    pub formula: Option<String>,
    /// Cell style
    pub style: Option<CellStyle>,
    /// Data validation (if any)
    pub validation: Option<DataValidation>,
    /// Cell comment
    pub comment: Option<String>,
    /// Cell hyperlink
    pub hyperlink: Option<String>,
    /// Whether the cell is part of a merged range
    pub merged: bool,
}

impl Cell {
    /// Create a new cell
    pub fn new(reference: CellReference, value: CellValue) -> Self {
        Self {
            reference,
            value,
            formula: None,
            style: None,
            validation: None,
            comment: None,
            hyperlink: None,
            merged: false,
        }
    }

    /// Get the cell key for HashMap storage
    pub fn key(&self) -> String {
        format!("{},{}", self.reference.row, self.reference.column)
    }
}

/// Cell value types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CellValue {
    /// Numeric value
    Number(f64),
    /// Text value
    Text(String),
    /// Boolean value
    Boolean(bool),
    /// Error value
    Error(String),
    /// Date/Time value
    DateTime(chrono::DateTime<chrono::Utc>),
    /// Empty cell
    Empty,
    /// Array of values
    Array(Vec<CellValue>),
}

impl CellValue {
    /// Check if the value is numeric
    pub fn is_number(&self) -> bool {
        matches!(self, CellValue::Number(_))
    }

    /// Check if the value is text
    pub fn is_text(&self) -> bool {
        matches!(self, CellValue::Text(_))
    }

    /// Check if the value is boolean
    pub fn is_boolean(&self) -> bool {
        matches!(self, CellValue::Boolean(_))
    }

    /// Check if the value is an error
    pub fn is_error(&self) -> bool {
        matches!(self, CellValue::Error(_))
    }

    /// Check if the value is empty
    pub fn is_empty(&self) -> bool {
        matches!(self, CellValue::Empty)
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            CellValue::Number(n) => n.to_string(),
            CellValue::Text(s) => s.clone(),
            CellValue::Boolean(b) => b.to_string(),
            CellValue::Error(e) => format!("#{}", e),
            CellValue::DateTime(dt) => dt.to_rfc3339(),
            CellValue::Empty => String::new(),
            CellValue::Array(arr) => format!("{:?}", arr),
        }
    }
}

// Forward declarations for types used in Cell
use super::{CellStyle, DataValidation};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_reference_creation() {
        let ref1 = CellReference::new("A".to_string(), 1);
        assert_eq!(ref1.column, "A");
        assert_eq!(ref1.row, 1);
    }

    #[test]
    fn test_cell_reference_with_sheet() {
        let ref1 = CellReference::with_sheet("Sheet1".to_string(), "B".to_string(), 5);
        assert_eq!(ref1.sheet, Some("Sheet1".to_string()));
        assert_eq!(ref1.column, "B");
        assert_eq!(ref1.row, 5);
    }

    #[test]
    fn test_cell_reference_to_string() {
        let ref1 = CellReference::new("A".to_string(), 1);
        assert_eq!(ref1.to_string(), "A1");
    }

    #[test]
    fn test_cell_reference_from_string() {
        let ref1 = CellReference::from_str("A1").unwrap();
        assert_eq!(ref1.column, "A");
        assert_eq!(ref1.row, 1);
    }

    #[test]
    fn test_cell_reference_from_string_with_sheet() {
        let ref1 = CellReference::from_str("Sheet1!B5").unwrap();
        assert_eq!(ref1.sheet, Some("Sheet1".to_string()));
        assert_eq!(ref1.column, "B");
        assert_eq!(ref1.row, 5);
    }

    #[test]
    fn test_column_index_conversion() {
        let ref1 = CellReference::new("A".to_string(), 1);
        assert_eq!(ref1.to_column_index().unwrap(), 0);
        
        let ref2 = CellReference::new("B".to_string(), 1);
        assert_eq!(ref2.to_column_index().unwrap(), 1);
        
        let ref3 = CellReference::new("AA".to_string(), 1);
        assert_eq!(ref3.to_column_index().unwrap(), 26);
    }

    #[test]
    fn test_column_index_to_letter() {
        assert_eq!(CellReference::from_column_index(0), "A");
        assert_eq!(CellReference::from_column_index(1), "B");
        assert_eq!(CellReference::from_column_index(26), "AA");
    }

    #[test]
    fn test_range_creation() {
        let start = CellReference::new("A".to_string(), 1);
        let end = CellReference::new("B".to_string(), 10);
        let range = Range::new(start, end);
        assert_eq!(range.to_string(), "A1:B10");
    }

    #[test]
    fn test_range_from_string() {
        let range = Range::from_str("A1:B10").unwrap();
        assert_eq!(range.start.column, "A");
        assert_eq!(range.start.row, 1);
        assert_eq!(range.end.column, "B");
        assert_eq!(range.end.row, 10);
    }

    #[test]
    fn test_range_get_cells() {
        let range = Range::from_str("A1:B2").unwrap();
        let cells = range.get_cells();
        assert_eq!(cells.len(), 4);
    }

    #[test]
    fn test_workbook_creation() {
        let workbook = Workbook::new("Test".to_string());
        assert_eq!(workbook.name, "Test");
        assert!(workbook.sheets.is_empty());
    }

    #[test]
    fn test_sheet_creation() {
        let sheet = Sheet {
            name: "Sheet1".to_string(),
            cells: HashMap::new(),
            dimensions: SheetDimensions::default(),
            visibility: SheetVisibility::default(),
            color: None,
        };
        assert_eq!(sheet.name, "Sheet1");
    }

    #[test]
    fn test_cell_value_number() {
        let value = CellValue::Number(42.0);
        assert!(value.is_number());
        assert_eq!(value.to_string(), "42");
    }

    #[test]
    fn test_cell_value_text() {
        let value = CellValue::Text("Hello".to_string());
        assert!(value.is_text());
        assert_eq!(value.to_string(), "Hello");
    }

    #[test]
    fn test_cell_value_boolean() {
        let value = CellValue::Boolean(true);
        assert!(value.is_boolean());
        assert_eq!(value.to_string(), "true");
    }

    #[test]
    fn test_cell_value_error() {
        let value = CellValue::Error("VALUE".to_string());
        assert!(value.is_error());
        assert_eq!(value.to_string(), "#VALUE");
    }

    #[test]
    fn test_cell_value_empty() {
        let value = CellValue::Empty;
        assert!(value.is_empty());
        assert_eq!(value.to_string(), "");
    }
}
