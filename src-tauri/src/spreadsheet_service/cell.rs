//! Cell management with aerospace-grade validation and operations
//! 
//! This module provides comprehensive cell management functionality including
//! CRUD operations, value validation, and dependency tracking.

use crate::spreadsheet_service::{
    error::{SpreadsheetError, SpreadsheetResult},
    types::{Cell, CellReference, CellValue},
};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Cell manager for handling cell operations
pub struct CellManager {
    /// Cells storage (key: "sheet,row,col", value: Cell)
    cells: Arc<RwLock<HashMap<String, Cell>>>,
    /// Cell dependencies (key: cell reference, value: cells that depend on it)
    dependencies: Arc<RwLock<HashMap<String, HashSet<String>>>>,
    /// Cell value cache for performance
    value_cache: Arc<RwLock<HashMap<String, CellValue>>>,
}

impl CellManager {
    /// Create a new cell manager
    pub fn new() -> Self {
        Self {
            cells: Arc::new(RwLock::new(HashMap::new())),
            dependencies: Arc::new(RwLock::new(HashMap::new())),
            value_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Check if the manager is initialized
    pub fn is_initialized(&self) -> bool {
        true // Always initialized after creation
    }

    /// Get a cell by reference
    pub async fn get_cell(&self, sheet: &str, reference: &CellReference) -> SpreadsheetResult<Cell> {
        let key = self.cell_key(sheet, reference);
        let cells = self.cells.read().await;
        
        cells.get(&key)
            .cloned()
            .ok_or_else(|| SpreadsheetError::cell_reference_error(
                &reference.to_string(),
                "cell not found"
            ))
    }

    /// Set a cell value
    pub async fn set_cell(
        &self,
        sheet: &str,
        reference: CellReference,
        value: CellValue,
    ) -> SpreadsheetResult<()> {
        let key = self.cell_key(sheet, &reference);
        
        // Validate the value
        self.validate_value(&value)?;
        
        let mut cells = self.cells.write().await;
        let mut value_cache = self.value_cache.write().await;
        
        // Check if cell exists
        if let Some(cell) = cells.get_mut(&key) {
            cell.value = value.clone();
            value_cache.insert(key, value);
        } else {
            let cell = Cell::new(reference.clone(), value.clone());
            cells.insert(key.clone(), cell);
            value_cache.insert(key, value);
        }
        
        Ok(())
    }

    /// Delete a cell
    pub async fn delete_cell(&self, sheet: &str, reference: &CellReference) -> SpreadsheetResult<()> {
        let key = self.cell_key(sheet, reference);
        
        let mut cells = self.cells.write().await;
        let mut value_cache = self.value_cache.write().await;
        let mut dependencies = self.dependencies.write().await;
        
        cells.remove(&key);
        value_cache.remove(&key);
        dependencies.remove(&key);
        
        Ok(())
    }

    /// Get a cell value (cached)
    pub async fn get_value(&self, sheet: &str, reference: &CellReference) -> SpreadsheetResult<CellValue> {
        let key = self.cell_key(sheet, reference);
        
        // Try cache first
        {
            let cache = self.value_cache.read().await;
            if let Some(value) = cache.get(&key) {
                return Ok(value.clone());
            }
        }
        
        // Fall back to cell storage
        let cell = self.get_cell(sheet, reference).await?;
        let mut cache = self.value_cache.write().await;
        cache.insert(key, cell.value.clone());
        Ok(cell.value)
    }

    /// Get multiple cells in a range
    pub async fn get_range(
        &self,
        sheet: &str,
        start: &CellReference,
        end: &CellReference,
    ) -> SpreadsheetResult<Vec<Cell>> {
        let mut result = Vec::new();
        
        let start_col = start.to_column_index().map_err(|e| 
            SpreadsheetError::cell_reference_error(&start.to_string(), &e))?;
        let end_col = end.to_column_index().map_err(|e| 
            SpreadsheetError::cell_reference_error(&end.to_string(), &e))?;
        
        for row in start.row..=end.row {
            for col in start_col..=end_col {
                let ref_str = crate::spreadsheet_service::types::CellReference::from_column_index(col);
                let cell_ref = CellReference {
                    sheet: Some(sheet.to_string()),
                    column: ref_str,
                    row,
                };
                
                if let Ok(cell) = self.get_cell(sheet, &cell_ref).await {
                    result.push(cell);
                }
            }
        }
        
        Ok(result)
    }

    /// Clear all cells in a sheet
    pub async fn clear_sheet(&self, sheet: &str) -> SpreadsheetResult<()> {
        let mut cells = self.cells.write().await;
        let mut value_cache = self.value_cache.write().await;
        let mut dependencies = self.dependencies.write().await;
        
        let prefix = format!("{},", sheet);
        
        cells.retain(|k, _| !k.starts_with(&prefix));
        value_cache.retain(|k, _| !k.starts_with(&prefix));
        dependencies.retain(|k, _| !k.starts_with(&prefix));
        
        Ok(())
    }

    /// Add a dependency between cells
    pub async fn add_dependency(
        &self,
        sheet: &str,
        dependent: &CellReference,
        depends_on: &CellReference,
    ) -> SpreadsheetResult<()> {
        let dependent_key = self.cell_key(sheet, dependent);
        let depends_on_key = self.cell_key(sheet, depends_on);
        
        let mut dependencies = self.dependencies.write().await;
        dependencies
            .entry(depends_on_key)
            .or_insert_with(HashSet::new)
            .insert(dependent_key);
        
        Ok(())
    }

    /// Get cells that depend on a given cell
    pub async fn get_dependents(
        &self,
        sheet: &str,
        reference: &CellReference,
    ) -> SpreadsheetResult<Vec<CellReference>> {
        let key = self.cell_key(sheet, reference);
        let dependencies = self.dependencies.read().await;
        
        if let Some(dependents) = dependencies.get(&key) {
            let result = dependents
                .iter()
                .filter_map(|k| self.parse_cell_key(k))
                .collect();
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }

    /// Check for circular references (iterative approach to avoid async recursion)
    pub async fn check_circular_reference(
        &self,
        sheet: &str,
        reference: &CellReference,
        visited: &mut HashSet<String>,
    ) -> bool {
        let start_key = self.cell_key(sheet, reference);
        
        let mut stack = vec![start_key.clone()];
        let mut local_visited = HashSet::new();
        
        while let Some(current_key) = stack.pop() {
            if local_visited.contains(&current_key) {
                return true; // Circular reference detected
            }
            
            if visited.contains(&current_key) {
                continue; // Already checked in outer context
            }
            
            local_visited.insert(current_key.clone());
            
            let dependents = {
                let dependencies = self.dependencies.read().await;
                dependencies.get(&current_key).cloned()
            };

            if let Some(dependents) = dependents {
                for dep_key in dependents {
                    stack.push(dep_key);
                }
            }
        }
        
        false
    }

    /// Get all cells in a sheet
    pub async fn get_sheet_cells(&self, sheet: &str) -> SpreadsheetResult<Vec<Cell>> {
        let cells = self.cells.read().await;
        let prefix = format!("{},", sheet);
        
        let result = cells
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix))
            .map(|(_, cell)| cell.clone())
            .collect();
        
        Ok(result)
    }

    /// Get cell count for a sheet
    pub async fn get_cell_count(&self, sheet: &str) -> usize {
        let cells = self.cells.read().await;
        let prefix = format!("{},", sheet);
        cells.iter().filter(|(k, _)| k.starts_with(&prefix)).count()
    }

    /// Validate a cell value
    fn validate_value(&self, value: &CellValue) -> SpreadsheetResult<()> {
        match value {
            CellValue::Number(n) => {
                if n.is_nan() || n.is_infinite() {
                    return Err(SpreadsheetError::invalid_input(
                        "value",
                        &n.to_string(),
                        "NaN and infinity are not allowed"
                    ));
                }
            }
            CellValue::Text(s) => {
                if s.len() > 32767 {
                    return Err(SpreadsheetError::invalid_input(
                        "value",
                        &format!("{} chars", s.len()),
                        "text too long (max 32767 characters)"
                    ));
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Generate a cell key for storage
    fn cell_key(&self, sheet: &str, reference: &CellReference) -> String {
        format!("{},{},{}", sheet, reference.row, reference.column)
    }

    /// Parse a cell key back to CellReference
    fn parse_cell_key(&self, key: &str) -> Option<CellReference> {
        let parts: Vec<&str> = key.split(',').collect();
        if parts.len() == 3 {
            let row = parts[1].parse::<u32>().ok()?;
            Some(CellReference {
                sheet: Some(parts[0].to_string()),
                column: parts[2].to_string(),
                row,
            })
        } else {
            None
        }
    }
}

impl Default for CellManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cell_manager_creation() {
        let manager = CellManager::new();
        assert!(manager.is_initialized());
    }

    #[tokio::test]
    async fn test_set_and_get_cell() {
        let manager = CellManager::new();
        let reference = CellReference::new("A".to_string(), 1);
        let value = CellValue::Number(42.0);
        
        manager.set_cell("Sheet1", reference.clone(), value.clone()).await.unwrap();
        let retrieved = manager.get_cell("Sheet1", &reference).await.unwrap();
        
        assert_eq!(retrieved.value, value);
    }

    #[tokio::test]
    async fn test_get_nonexistent_cell() {
        let manager = CellManager::new();
        let reference = CellReference::new("A".to_string(), 1);
        
        let result = manager.get_cell("Sheet1", &reference).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_cell() {
        let manager = CellManager::new();
        let reference = CellReference::new("A".to_string(), 1);
        let value = CellValue::Number(42.0);
        
        manager.set_cell("Sheet1", reference.clone(), value).await.unwrap();
        manager.delete_cell("Sheet1", &reference).await.unwrap();
        
        let result = manager.get_cell("Sheet1", &reference).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_value_cached() {
        let manager = CellManager::new();
        let reference = CellReference::new("A".to_string(), 1);
        let value = CellValue::Number(42.0);
        
        manager.set_cell("Sheet1", reference.clone(), value.clone()).await.unwrap();
        let retrieved = manager.get_value("Sheet1", &reference).await.unwrap();
        
        assert_eq!(retrieved, value);
    }

    #[tokio::test]
    async fn test_clear_sheet() {
        let manager = CellManager::new();
        
        manager.set_cell("Sheet1", CellReference::new("A".to_string(), 1), CellValue::Number(1.0)).await.unwrap();
        manager.set_cell("Sheet1", CellReference::new("B".to_string(), 1), CellValue::Number(2.0)).await.unwrap();
        manager.set_cell("Sheet2", CellReference::new("A".to_string(), 1), CellValue::Number(3.0)).await.unwrap();
        
        manager.clear_sheet("Sheet1").await.unwrap();
        
        assert_eq!(manager.get_cell_count("Sheet1").await, 0);
        assert_eq!(manager.get_cell_count("Sheet2").await, 1);
    }

    #[tokio::test]
    async fn test_add_dependency() {
        let manager = CellManager::new();
        let dependent = CellReference::with_sheet("Sheet1".to_string(), "A".to_string(), 1);
        let depends_on = CellReference::with_sheet("Sheet1".to_string(), "B".to_string(), 1);
        
        manager.add_dependency("Sheet1", &dependent, &depends_on).await.unwrap();
        let dependents = manager.get_dependents("Sheet1", &depends_on).await.unwrap();
        
        assert_eq!(dependents.len(), 1);
        assert_eq!(dependents[0], dependent);
    }

    #[tokio::test]
    async fn test_validate_number_value() {
        let manager = CellManager::new();
        
        // Valid number
        assert!(manager.validate_value(&CellValue::Number(42.0)).is_ok());
        
        // Invalid: NaN
        assert!(manager.validate_value(&CellValue::Number(f64::NAN)).is_err());
        
        // Invalid: Infinity
        assert!(manager.validate_value(&CellValue::Number(f64::INFINITY)).is_err());
    }

    #[tokio::test]
    async fn test_validate_text_value() {
        let manager = CellManager::new();
        
        // Valid text
        assert!(manager.validate_value(&CellValue::Text("Hello".to_string())).is_ok());
        
        // Invalid: too long
        let long_text = "a".repeat(32768);
        assert!(manager.validate_value(&CellValue::Text(long_text)).is_err());
    }

    #[tokio::test]
    async fn test_get_range() {
        let manager = CellManager::new();
        
        manager.set_cell("Sheet1", CellReference::new("A".to_string(), 1), CellValue::Number(1.0)).await.unwrap();
        manager.set_cell("Sheet1", CellReference::new("B".to_string(), 1), CellValue::Number(2.0)).await.unwrap();
        manager.set_cell("Sheet1", CellReference::new("A".to_string(), 2), CellValue::Number(3.0)).await.unwrap();
        
        let start = CellReference::new("A".to_string(), 1);
        let end = CellReference::new("B".to_string(), 2);
        let range = manager.get_range("Sheet1", &start, &end).await.unwrap();
        
        assert_eq!(range.len(), 3);
    }
}
