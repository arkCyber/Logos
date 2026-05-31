/**
 * Aerospace-grade Spreadsheet Service
 * 
 * This service provides type-safe access to the Rust backend spreadsheet functionality
 * through Tauri commands with comprehensive error handling and validation.
 */

import { invoke } from '@tauri-apps/api/core';
import type {
  CellValue,
  FormulaResult,
  PivotConfig,
  PivotTable,
  ChartConfig,
  Chart,
  ValidationRule,
  ValidationResult,
  CellStyle,
  SpreadsheetServiceStatus,
  SpreadsheetError,
  Workbook,
  ExcelImportOptions,
  ExcelExportOptions
} from '../types/spreadsheet';

/**
 * Spreadsheet service class
 */
export class SpreadsheetService {
  /**
   * Evaluate a formula
   * @param formula - The formula to evaluate (e.g., "=SUM(A1,A2)")
   * @param cellValues - Map of cell references to values
   * @returns Formula result
   */
  static async evaluateFormula(
    formula: string,
    cellValues: Record<string, CellValue>
  ): Promise<FormulaResult> {
    try {
      const result = await invoke<FormulaResult>('evaluate_formula', {
        formula,
        cellValuesJson: JSON.stringify(cellValues)
      });
      return result;
    } catch (error) {
      throw this.handleError(error, 'Formula evaluation failed');
    }
  }

  /**
   * Generate a pivot table
   * @param data - Source data as array of objects
   * @param config - Pivot table configuration
   * @returns Generated pivot table
   */
  static async generatePivotTable(
    data: Record<string, unknown>[],
    config: PivotConfig
  ): Promise<PivotTable> {
    try {
      const result = await invoke<PivotTable>('generate_pivot_table', {
        dataJson: JSON.stringify(data),
        configJson: JSON.stringify(config)
      });
      return result;
    } catch (error) {
      throw this.handleError(error, 'Pivot table generation failed');
    }
  }

  /**
   * Generate a chart
   * @param data - Source data as array of objects
   * @param config - Chart configuration
   * @returns Generated chart
   */
  static async generateChart(
    data: Record<string, unknown>[],
    config: ChartConfig
  ): Promise<Chart> {
    try {
      const result = await invoke<Chart>('generate_spreadsheet_chart', {
        dataJson: JSON.stringify(data),
        configJson: JSON.stringify(config)
      });
      return result;
    } catch (error) {
      throw this.handleError(error, 'Chart generation failed');
    }
  }

  /**
   * Validate cell data
   * @param value - Cell value to validate
   * @param rule - Validation rule
   * @returns Validation result
   */
  static async validateCellData(
    value: CellValue,
    rule: ValidationRule
  ): Promise<ValidationResult> {
    try {
      const result = await invoke<ValidationResult>('validate_cell_data', {
        valueJson: JSON.stringify(value),
        ruleJson: JSON.stringify(rule)
      });
      return result;
    } catch (error) {
      throw this.handleError(error, 'Cell data validation failed');
    }
  }

  /**
   * Apply cell style
   * @param style - Cell style to apply
   * @returns Style ID
   */
  static async applyCellStyle(style: CellStyle): Promise<string> {
    try {
      const result = await invoke<string>('apply_cell_style', {
        styleJson: JSON.stringify(style)
      });
      return result;
    } catch (error) {
      throw this.handleError(error, 'Cell style application failed');
    }
  }

  /**
   * Get spreadsheet service status
   * @returns Service status
   */
  static async getServiceStatus(): Promise<SpreadsheetServiceStatus> {
    try {
      const result = await invoke<SpreadsheetServiceStatus>('get_spreadsheet_service_status');
      return result;
    } catch (error) {
      throw this.handleError(error, 'Failed to get service status');
    }
  }

  /**
   * Import Excel file from bytes
   * @param data - Excel file bytes
   * @param options - Import options
   * @returns Imported workbook
   */
  static async importExcelFromBytes(
    data: Uint8Array,
    options: ExcelImportOptions
  ): Promise<Workbook> {
    try {
      const result = await invoke<Workbook>('import_excel_from_bytes', {
        data: Array.from(data),
        optionsJson: JSON.stringify(options)
      });
      return result;
    } catch (error) {
      throw this.handleError(error, 'Excel import failed');
    }
  }

  /**
   * Import Excel file from path
   * @param path - File path
   * @param options - Import options
   * @returns Imported workbook
   */
  static async importExcelFromPath(
    path: string,
    options: ExcelImportOptions
  ): Promise<Workbook> {
    try {
      const result = await invoke<Workbook>('import_excel_from_path', {
        path,
        optionsJson: JSON.stringify(options)
      });
      return result;
    } catch (error) {
      throw this.handleError(error, 'Excel import failed');
    }
  }

  /**
   * Export workbook to bytes
   * @param workbook - Workbook to export
   * @param options - Export options
   * @returns Excel file bytes
   */
  static async exportExcelToBytes(
    workbook: Workbook,
    options: ExcelExportOptions
  ): Promise<Uint8Array> {
    try {
      const result = await invoke<number[]>('export_excel_to_bytes', {
        workbookJson: JSON.stringify(workbook),
        optionsJson: JSON.stringify(options)
      });
      return new Uint8Array(result);
    } catch (error) {
      throw this.handleError(error, 'Excel export failed');
    }
  }

  /**
   * Export workbook to path
   * @param workbook - Workbook to export
   * @param path - Output file path
   * @param options - Export options
   */
  static async exportExcelToPath(
    workbook: Workbook,
    path: string,
    options: ExcelExportOptions
  ): Promise<void> {
    try {
      await invoke('export_excel_to_path', {
        workbookJson: JSON.stringify(workbook),
        path,
        optionsJson: JSON.stringify(options)
      });
    } catch (error) {
      throw this.handleError(error, 'Excel export failed');
    }
  }

  /**
   * Handle errors with aerospace-grade error processing
   * @param error - Error object
   * @param context - Error context
   * @returns Formatted error
   */
  private static handleError(error: unknown, context: string): Error {
    if (error instanceof Error) {
      // Check if it's a spreadsheet error from the backend
      if ('errorType' in error && 'severity' in error) {
        const spreadsheetError = error as unknown as SpreadsheetError;
        return new Error(
          `${context}: ${spreadsheetError.message} (Severity: ${spreadsheetError.severity})`
        );
      }
      return new Error(`${context}: ${error.message}`);
    }
    return new Error(`${context}: Unknown error occurred`);
  }

  /**
   * Validate service health
   * @returns True if service is healthy
   */
  static async isHealthy(): Promise<boolean> {
    try {
      const status = await this.getServiceStatus();
      return status.initialized;
    } catch {
      return false;
    }
  }

  /**
   * Batch evaluate formulas
   * @param formulas - Array of formula evaluations
   * @returns Array of formula results
   */
  static async batchEvaluateFormulas(
    formulas: Array<{ formula: string; cellValues: Record<string, CellValue> }>
  ): Promise<FormulaResult[]> {
    const results = await Promise.allSettled(
      formulas.map(({ formula, cellValues }) =>
        this.evaluateFormula(formula, cellValues)
      )
    );

    return results.map((result, index) => {
      if (result.status === 'fulfilled') {
        return result.value;
      }
      throw new Error(`Formula ${index + 1} failed: ${result.reason}`);
    });
  }

  /**
   * Create a simple pivot table with common defaults
   * @param data - Source data
   * @param rowField - Field for rows
   * @param columnField - Field for columns
   * @param valueField - Field for values
   * @param aggregation - Aggregation type
   * @returns Generated pivot table
   */
  static async createSimplePivotTable(
    data: Record<string, unknown>[],
    rowField: string,
    columnField: string,
    valueField: string,
    aggregation: 'Sum' | 'Average' | 'Count' = 'Sum'
  ): Promise<PivotTable> {
    const config: PivotConfig = {
      rows: [rowField],
      columns: [columnField],
      values: [{ field: valueField, aggregation, name: valueField }],
      filters: []
    };

    return this.generatePivotTable(data, config);
  }

  /**
   * Create a simple chart with common defaults
   * @param data - Source data
   * @param categoryField - Field for categories
   * @param valueFields - Fields for values
   * @param chartType - Chart type
   * @returns Generated chart
   */
  static async createSimpleChart(
    data: Record<string, unknown>[],
    categoryField: string,
    valueFields: string[],
    chartType: ChartConfig['chartType'] = 'Bar'
  ): Promise<Chart> {
    const config: ChartConfig = {
      chartType,
      title: `${valueFields.join(' & ')} by ${categoryField}`,
      dataRange: 'A1:Z100',
      categoryField,
      valueFields,
      legendPosition: 'Bottom',
      showDataLabels: true,
      showGridlines: true
    };

    return this.generateChart(data, config);
  }
}

/**
 * Spreadsheet service instance for convenience
 */
export const spreadsheetService = SpreadsheetService;
