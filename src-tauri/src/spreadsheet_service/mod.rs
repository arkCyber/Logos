//! Aerospace-grade Spreadsheet Service
//! 
//! This module provides high-performance, fault-tolerant spreadsheet functionality
//! with comprehensive error handling, logging, and validation suitable for critical applications.
//! 
//! ## Architecture
//! 
//! - **Cell Management**: CRUD operations with validation
//! - **Formula Engine**: Advanced formula evaluation with circular reference detection
//! - **Style System**: Comprehensive cell styling with serialization
//! - **Data Validation**: Input validation with custom rules
//! - **Excel I/O**: Import/export with error recovery
//! - **Pivot Tables**: Data aggregation with multiple aggregation types
//! - **Charts**: Data visualization with multiple chart types
//! - **Conditional Formatting**: Rule-based cell formatting
//! 
//! ## Safety Guarantees
//! 
//! - All operations are validated before execution
//! - Comprehensive error handling with detailed error types
//! - Structured logging for audit trails
//! - Memory-safe operations with Rust's type system
//! - Thread-safe operations where applicable

pub mod cell;
pub mod formula;
pub mod style;
pub mod validation;
pub mod excel_io;
pub mod pivot;
pub mod charts;
pub mod conditional_formatting;
pub mod error;
pub mod types;

#[cfg(test)]
mod tests;

pub use cell::{CellManager};
pub use formula::{FormulaEngine, FormulaResult};
pub use style::{CellStyle, StyleManager, FontStyle, HorizontalAlignment, VerticalAlignment};
pub use validation::{DataValidation, ValidationRule, ValidationType, ValidationManager};
pub use excel_io::{ExcelImporter, ExcelExporter, ExcelImportOptions, ExcelExportOptions};
pub use pivot::{PivotTable, PivotConfig, PivotGenerator, PivotAggregation, PivotValue};
pub use charts::{Chart, ChartType, ChartGenerator, ChartConfig};
pub use conditional_formatting::{ConditionalFormat, ConditionalFormatRule, ConditionalFormatManager};
pub use error::{SpreadsheetError, SpreadsheetResult, FormulaErrorType};
pub use types::{Sheet, Workbook, Range, CellReference, Cell, CellValue};

use std::sync::Arc;
use tokio::sync::RwLock;

/// Global spreadsheet service instance with aerospace-grade thread safety
static SPREADSHEET_SERVICE: once_cell::sync::Lazy<
    Arc<RwLock<SpreadsheetService>>
> = once_cell::sync::Lazy::new(|| {
    Arc::new(RwLock::new(SpreadsheetService::new()))
});

/// Main spreadsheet service providing all spreadsheet functionality
pub struct SpreadsheetService {
    cell_manager: CellManager,
    formula_engine: FormulaEngine,
    style_manager: StyleManager,
    validation_manager: ValidationManager,
    pivot_generator: PivotGenerator,
    chart_generator: ChartGenerator,
    conditional_format_manager: ConditionalFormatManager,
}

impl SpreadsheetService {
    /// Create a new spreadsheet service instance
    pub fn new() -> Self {
        Self {
            cell_manager: CellManager::new(),
            formula_engine: FormulaEngine::new(),
            style_manager: StyleManager::new(),
            validation_manager: ValidationManager::new(),
            pivot_generator: PivotGenerator::new(),
            chart_generator: ChartGenerator::new(),
            conditional_format_manager: ConditionalFormatManager::new(),
        }
    }

    /// Get the global spreadsheet service instance
    pub fn global() -> Arc<RwLock<SpreadsheetService>> {
        SPREADSHEET_SERVICE.clone()
    }

    /// Get the cell manager
    pub fn cell_manager(&self) -> &CellManager {
        &self.cell_manager
    }

    /// Get the formula engine
    pub fn formula_engine(&self) -> &FormulaEngine {
        &self.formula_engine
    }

    /// Get the style manager
    pub fn style_manager(&self) -> &StyleManager {
        &self.style_manager
    }

    /// Get the validation manager
    pub fn validation_manager(&self) -> &ValidationManager {
        &self.validation_manager
    }

    /// Get the pivot generator
    pub fn pivot_generator(&self) -> &PivotGenerator {
        &self.pivot_generator
    }

    /// Get the chart generator
    pub fn chart_generator(&self) -> &ChartGenerator {
        &self.chart_generator
    }

    /// Get the conditional format manager
    pub fn conditional_format_manager(&self) -> &ConditionalFormatManager {
        &self.conditional_format_manager
    }
}

impl Default for SpreadsheetService {
    fn default() -> Self {
        Self::new()
    }
}
