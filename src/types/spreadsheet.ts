/**
 * Aerospace-grade Spreadsheet Service Type Definitions
 * 
 * This file contains TypeScript type definitions for the spreadsheet service
 * that mirror the Rust backend types for type-safe frontend-backend communication.
 */

/**
 * Cell value types
 */
export type CellValue = 
  | { type: 'Empty' }
  | { type: 'Text'; value: string }
  | { type: 'Number'; value: number }
  | { type: 'Boolean'; value: boolean }
  | { type: 'Error'; value: string }
  | { type: 'DateTime'; value: string }
  | { type: 'Array'; value: CellValue[] };

/**
 * Cell reference (e.g., "A1", "Sheet1!B2")
 */
export interface CellReference {
  sheet?: string;
  column: string;
  row: number;
}

/**
 * Cell with all properties
 */
export interface Cell {
  reference: CellReference;
  value: CellValue;
  formula?: string;
  style?: CellStyle;
  validation?: DataValidation;
  comment?: string;
  hyperlink?: string;
  merged: boolean;
}

/**
 * Cell style
 */
export interface CellStyle {
  font?: FontStyle;
  border?: Border;
  fill?: Fill;
  alignment?: Alignment;
  numberFormat?: string;
  protection?: Protection;
}

/**
 * Font style
 */
export interface FontStyle {
  name?: string;
  size?: number;
  bold?: boolean;
  italic?: boolean;
  underline?: boolean;
  color?: string;
}

/**
 * Border style
 */
export interface Border {
  left?: BorderSide;
  right?: BorderSide;
  top?: BorderSide;
  bottom?: BorderSide;
  diagonal?: BorderSide;
}

/**
 * Border side
 */
export interface BorderSide {
  style: BorderStyle;
  color?: string;
}

/**
 * Border styles
 */
export type BorderStyle = 
  | 'None' | 'Thin' | 'Medium' | 'Thick' | 'Double' | 'Hair'
  | 'Dotted' | 'Dashed' | 'DashDot' | 'DashDotDot' | 'SlantDashDot'
  | 'MediumDashed' | 'MediumDashDotDot';

/**
 * Fill pattern
 */
export interface Fill {
  pattern: FillPattern;
  foregroundColor?: string;
  backgroundColor?: string;
}

/**
 * Fill patterns
 */
export type FillPattern = 
  | 'None' | 'Solid' | 'MediumGray' | 'DarkGray' | 'LightGray'
  | 'DarkHorizontal' | 'DarkVertical' | 'DarkDown' | 'DarkUp' | 'DarkGrid' | 'DarkTrellis'
  | 'LightHorizontal' | 'LightVertical' | 'LightDown' | 'LightUp' | 'LightGrid' | 'LightTrellis'
  | 'Gray125' | 'Gray0625';

/**
 * Alignment
 */
export interface Alignment {
  horizontal?: HorizontalAlignment;
  vertical?: VerticalAlignment;
  wrapText?: boolean;
  rotation?: number;
}

/**
 * Horizontal alignment
 */
export type HorizontalAlignment = 
  | 'General' | 'Left' | 'Center' | 'Right' | 'Fill' | 'Justify' | 'CenterContinuous' | 'Distributed';

/**
 * Vertical alignment
 */
export type VerticalAlignment = 
  | 'Top' | 'Center' | 'Bottom' | 'Justify' | 'Distributed';

/**
 * Protection
 */
export interface Protection {
  locked?: boolean;
  hidden?: boolean;
}

/**
 * Data validation
 */
export interface DataValidation {
  rule: ValidationRule;
}

/**
 * Validation rule
 */
export interface ValidationRule {
  validationType: ValidationType;
  operator?: ValidationOperator;
  value1?: string;
  value2?: string;
  inputMessage?: string;
  errorMessage?: string;
  errorTitle?: string;
  ignoreBlank: boolean;
  inCellDropdown: boolean;
}

/**
 * Validation types
 */
export type ValidationType = 
  | 'Any' | 'WholeNumber' | 'Decimal' | 'List' | 'Date' | 'Time' | 'TextLength' | 'Custom';

/**
 * Validation operators
 */
export type ValidationOperator = 
  | 'Between' | 'NotBetween' | 'Equal' | 'NotEqual'
  | 'GreaterThan' | 'LessThan' | 'GreaterThanOrEqual' | 'LessThanOrEqual';

/**
 * Sheet dimensions
 */
export interface SheetDimensions {
  rows: number;
  columns: number;
}

/**
 * Sheet visibility
 */
export type SheetVisibility = 'Visible' | 'Hidden' | 'VeryHidden';

/**
 * Sheet
 */
export interface Sheet {
  name: string;
  cells: Record<string, Cell>;
  dimensions: SheetDimensions;
  visibility: SheetVisibility;
  color?: string;
}

/**
 * Workbook
 */
export interface Workbook {
  name: string;
  sheets: Sheet[];
  activeSheet: number;
  metadata: WorkbookMetadata;
}

/**
 * Workbook metadata
 */
export interface WorkbookMetadata {
  createdAt: string;
  modifiedAt: string;
  author?: string;
  description?: string;
  properties: Record<string, string>;
}

/**
 * Formula result
 */
export type FormulaResult = 
  | { type: 'Number'; value: number }
  | { type: 'String'; value: string }
  | { type: 'Boolean'; value: boolean }
  | { type: 'Error'; errorType: FormulaErrorType }
  | { type: 'Array'; value: FormulaResult[] };

/**
 * Formula error types
 */
export type FormulaErrorType = 
  | 'Value' | 'Ref' | 'Name' | 'Div0' | 'NA' | 'Num' | 'Null' | 'Calc';

/**
 * Pivot aggregation types
 */
export type PivotAggregation = 
  | 'Sum' | 'Average' | 'Count' | 'CountNumbers' | 'Min' | 'Max' | 'Product'
  | 'StdDev' | 'StdDevP' | 'Var' | 'VarP';

/**
 * Pivot value
 */
export interface PivotValue {
  field: string;
  aggregation: PivotAggregation;
  name?: string;
}

/**
 * Pivot filter
 */
export interface PivotFilter {
  field: string;
  operator: FilterOperator;
  value: string;
}

/**
 * Filter operators
 */
export type FilterOperator = 
  | 'Equals' | 'NotEquals' | 'GreaterThan' | 'LessThan'
  | 'GreaterThanOrEqual' | 'LessThanOrEqual' | 'Contains' | 'NotContains'
  | 'StartsWith' | 'EndsWith';

/**
 * Pivot table configuration
 */
export interface PivotConfig {
  rows: string[];
  columns: string[];
  values: PivotValue[];
  filters: PivotFilter[];
}

/**
 * Pivot table result
 */
export interface PivotTable {
  data: PivotData;
  config: PivotConfig;
}

/**
 * Pivot data
 */
export interface PivotData {
  headers: string[];
  rows: PivotRow[];
  grandTotal?: number;
}

/**
 * Pivot row
 */
export interface PivotRow {
  values: (string | number)[];
  total?: number;
}

/**
 * Chart types
 */
export type ChartType = 
  | 'Line' | 'Bar' | 'Column' | 'Pie' | 'Scatter' | 'Area' | 'Doughnut' | 'Radar';

/**
 * Legend position
 */
export type LegendPosition = 'Top' | 'Bottom' | 'Left' | 'Right' | 'TopRight' | 'TopLeft';

/**
 * Chart configuration
 */
export interface ChartConfig {
  chartType: ChartType;
  title?: string;
  dataRange: string;
  categoryField?: string;
  valueFields: string[];
  legendPosition?: LegendPosition;
  showDataLabels: boolean;
  showGridlines: boolean;
  colors?: string[];
}

/**
 * Chart data
 */
export interface ChartData {
  categories: string[];
  series: ChartSeries[];
}

/**
 * Chart series
 */
export interface ChartSeries {
  name: string;
  values: number[];
  color?: string;
}

/**
 * Chart
 */
export interface Chart {
  config: ChartConfig;
  data: ChartData;
}

/**
 * Conditional format rule types
 */
export type ConditionalFormatType = 
  | 'CellIs' | 'Expression' | 'ColorScale' | 'DataBar' | 'IconSet'
  | 'Top10' | 'Bottom10' | 'AboveAverage' | 'BelowAverage'
  | 'DuplicateValues' | 'UniqueValues'
  | 'ContainsText' | 'NotContainsText' | 'BeginsWith' | 'EndsWith';

/**
 * Comparison operators
 */
export type ComparisonOperator = 
  | 'Equal' | 'NotEqual' | 'GreaterThan' | 'LessThan'
  | 'GreaterThanOrEqual' | 'LessThanOrEqual' | 'Between' | 'NotBetween';

/**
 * Conditional format rule
 */
export interface ConditionalFormatRule {
  ruleType: ConditionalFormatType;
  operator?: ComparisonOperator;
  value1?: string;
  value2?: string;
  formula?: string;
  style: CellStyle;
  priority: number;
  stopIfTrue: boolean;
}

/**
 * Conditional format
 */
export interface ConditionalFormat {
  range: string;
  rules: ConditionalFormatRule[];
}

/**
 * Validation result
 */
export interface ValidationResult {
  valid: boolean;
  errorMessage?: string;
}

/**
 * Service status
 */
export interface SpreadsheetServiceStatus {
  initialized: boolean;
  cellManager: string;
  formulaEngine: string;
  styleManager: string;
  validationManager: string;
  pivotGenerator: string;
  chartGenerator: string;
  conditionalFormatManager: string;
}

/**
 * Excel import options
 */
export interface ExcelImportOptions {
  includeFormulas: boolean;
  includeStyles: boolean;
  includeMergedCells: boolean;
  firstSheetOnly: boolean;
}

/**
 * Excel export options
 */
export interface ExcelExportOptions {
  includeFormulas: boolean;
  includeStyles: boolean;
  includeMergedCells: boolean;
}

/**
 * Spreadsheet error
 */
export interface SpreadsheetError {
  errorType: string;
  message: string;
  severity: 'Critical' | 'High' | 'Medium' | 'Low' | 'Info';
  recoverySuggestion?: string;
}
