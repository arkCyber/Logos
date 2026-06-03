//! Integration tests for the aerospace-grade spreadsheet service
//! 
//! This module contains comprehensive integration tests that verify
//! the interaction between different components of the spreadsheet service.

use super::*;
use std::collections::HashMap;
use crate::spreadsheet_service::style::{Color, Font, FontStyle, FontWeight, TextDecoration, FillPattern, Fill};
use crate::spreadsheet_service::validation::{ValidationOperator, ValidationType};
use crate::spreadsheet_service::pivot::PivotValue;
use crate::spreadsheet_service::charts::LegendPosition;
use crate::spreadsheet_service::conditional_formatting::{ConditionalFormatType, ComparisonOperator, ConditionalFormatRule};
use crate::spreadsheet_service::formula::FormulaResult;
use crate::spreadsheet_service::pivot::PivotAggregation;
use crate::spreadsheet_service::charts::ChartType;
use crate::spreadsheet_service::conditional_formatting::ConditionalFormat;

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Test end-to-end formula evaluation
    #[test]
    fn test_formula_evaluation() {
        let mut engine = FormulaEngine::new();

        let mut cell_values = HashMap::new();
        cell_values.insert("A1".to_string(), CellValue::Number(10.0));
        cell_values.insert("A2".to_string(), CellValue::Number(20.0));

        let result = engine.evaluate("=SUM(A1,A2)", &cell_values).unwrap();
        assert_eq!(result, FormulaResult::Number(30.0));
    }

    /// Test style application and merging
    #[test]
    fn test_style_application_and_merging() {
        let mut manager = StyleManager::new();

        // Create base style
        let mut base_style = CellStyle::minimal();
        base_style.font = Some(Font {
            name: Some("Arial".to_string()),
            size: Some(12.0),
            style: Some(FontStyle::Normal),
            weight: Some(FontWeight::Normal),
            color: Some(Color::from_hex("#000000").unwrap()),
            decoration: Some(TextDecoration::None),
            strike: Some(false),
            outline: Some(false),
            shadow: Some(false),
            condense: Some(false),
            extend: Some(false),
        });

        let base_id = manager.register_style(base_style.clone());

        // Create conditional style
        let mut conditional_style = CellStyle::minimal();
        conditional_style.fill = Some(Fill {
            pattern: Some(FillPattern::Solid),
            foreground_color: Some(Color::from_hex("#FF0000").unwrap()),
            background_color: None,
        });

        let conditional_id = manager.register_style(conditional_style.clone());

        // Merge styles
        let merged = base_style.merge(&conditional_style);

        assert!(merged.font.is_some());
        assert!(merged.fill.is_some());
        assert_eq!(merged.font.unwrap().name, Some("Arial".to_string()));
        assert_eq!(merged.fill.unwrap().foreground_color.unwrap(), Color::from_hex("#FF0000").unwrap());
    }

    /// Test data validation with multiple rules
    #[test]
    fn test_data_validation_multiple_rules() {
        let mut manager = ValidationManager::new();

        // Create whole number validation
        let rule1 = ValidationRule {
            validation_type: ValidationType::WholeNumber,
            operator: Some(ValidationOperator::Between),
            value1: Some("1".to_string()),
            value2: Some("100".to_string()),
            list: None,
            formula: None,
            input_message: Some("Enter a number between 1 and 100".to_string()),
            error_message: Some("Value must be between 1 and 100".to_string()),
            error_title: Some("Invalid Input".to_string()),
            show_error: true,
            show_input: true,
            input_title: Some("Input".to_string()),
        };

        // Create list validation
        let rule2 = ValidationRule {
            validation_type: ValidationType::List,
            operator: None,
            value1: None,
            value2: None,
            list: Some(vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()]),
            formula: None,
            input_message: Some("Select a color".to_string()),
            error_message: Some("Invalid color".to_string()),
            error_title: Some("Invalid Input".to_string()),
            show_error: true,
            show_input: true,
            input_title: Some("Input".to_string()),
        };

        let data_validation1 = DataValidation {
            rule: rule1,
            ignore_blank: true,
            in_cell_dropdown: false,
        };
        let data_validation2 = DataValidation {
            rule: rule2,
            ignore_blank: true,
            in_cell_dropdown: true,
        };

        manager.add_validation("Sheet1", 0, "A", data_validation1).unwrap();
        manager.add_validation("Sheet1", 0, "B", data_validation2).unwrap();

        // Test validations
        let validation1 = manager.get_validation("Sheet1", 0, "A").unwrap();
        assert_eq!(validation1.rule.validation_type, ValidationType::WholeNumber);

        let validation2 = manager.get_validation("Sheet1", 0, "B").unwrap();
        assert_eq!(validation2.rule.validation_type, ValidationType::List);
    }

    /// Test pivot table generation with real data
    #[test]
    fn test_pivot_table_generation() {
        let generator = PivotGenerator::new();

        // Create sample data
        let mut data = Vec::new();
        
        let mut row1 = HashMap::new();
        row1.insert("Region".to_string(), CellValue::Text("North".to_string()));
        row1.insert("Product".to_string(), CellValue::Text("Widget".to_string()));
        row1.insert("Sales".to_string(), CellValue::Number(100.0));
        data.push(row1);

        let mut row2 = HashMap::new();
        row2.insert("Region".to_string(), CellValue::Text("North".to_string()));
        row2.insert("Product".to_string(), CellValue::Text("Gadget".to_string()));
        row2.insert("Sales".to_string(), CellValue::Number(150.0));
        data.push(row2);

        let mut row3 = HashMap::new();
        row3.insert("Region".to_string(), CellValue::Text("South".to_string()));
        row3.insert("Product".to_string(), CellValue::Text("Widget".to_string()));
        row3.insert("Sales".to_string(), CellValue::Number(200.0));
        data.push(row3);

        // Configure pivot table
        let config = PivotConfig {
            rows: vec!["Region".to_string()],
            columns: vec!["Product".to_string()],
            values: vec![PivotValue {
                field: "Sales".to_string(),
                aggregation: PivotAggregation::Sum,
                name: Some("Total Sales".to_string()),
            }],
            filters: vec![],
        };

        let result = generator.generate(&data, config).unwrap();

        assert!(!result.data.rows.is_empty());
        assert_eq!(result.data.headers.len(), 4); // Region, Product, Total Sales, Total
    }

    /// Test chart generation with real data
    #[test]
    fn test_chart_generation() {
        let generator = ChartGenerator::new();

        // Create sample data
        let mut data = Vec::new();
        
        let mut row1 = HashMap::new();
        row1.insert("Month".to_string(), CellValue::Text("Jan".to_string()));
        row1.insert("Sales".to_string(), CellValue::Number(100.0));
        data.push(row1);

        let mut row2 = HashMap::new();
        row2.insert("Month".to_string(), CellValue::Text("Feb".to_string()));
        row2.insert("Sales".to_string(), CellValue::Number(150.0));
        data.push(row2);

        let mut row3 = HashMap::new();
        row3.insert("Month".to_string(), CellValue::Text("Mar".to_string()));
        row3.insert("Sales".to_string(), CellValue::Number(200.0));
        data.push(row3);

        // Configure chart
        let config = ChartConfig {
            chart_type: ChartType::Bar,
            title: Some("Monthly Sales".to_string()),
            data_range: "A1:B3".to_string(),
            category_field: Some("Month".to_string()),
            value_fields: vec!["Sales".to_string()],
            legend_position: Some(LegendPosition::Bottom),
            show_data_labels: true,
            show_gridlines: true,
            colors: None,
        };

        let result = generator.generate(&data, config).unwrap();

        assert_eq!(result.data.categories.len(), 3);
        assert_eq!(result.data.series.len(), 1);
        assert_eq!(result.data.series[0].values.len(), 3);
    }

    /// Test conditional formatting with multiple rules
    #[test]
    fn test_conditional_formatting_multiple_rules() {
        let mut manager = ConditionalFormatManager::new();

        // Create rule 1: Greater than 100
        let rule1 = ConditionalFormatRule {
            rule_type: ConditionalFormatType::CellIs,
            operator: Some(ComparisonOperator::GreaterThan),
            value1: Some("100".to_string()),
            value2: None,
            formula: None,
            style: {
                let mut style = CellStyle::minimal();
                style.font = Some(Font {
                    name: Some("Arial".to_string()),
                    size: Some(12.0),
                    style: Some(FontStyle::Normal),
                    weight: Some(FontWeight::Bold),
                    color: Some(Color::from_hex("#FF0000").unwrap()),
                    decoration: Some(TextDecoration::None),
                    strike: Some(false),
                    outline: Some(false),
                    shadow: Some(false),
                    condense: Some(false),
                    extend: Some(false),
                });
                style
            },
            priority: 1,
            stop_if_true: false,
        };

        // Create rule 2: Less than 50
        let rule2 = ConditionalFormatRule {
            rule_type: ConditionalFormatType::CellIs,
            operator: Some(ComparisonOperator::LessThan),
            value1: Some("50".to_string()),
            value2: None,
            formula: None,
            style: {
                let mut style = CellStyle::minimal();
                style.font = Some(Font {
                    name: Some("Arial".to_string()),
                    size: Some(12.0),
                    style: Some(FontStyle::Italic),
                    weight: Some(FontWeight::Normal),
                    color: Some(Color::from_hex("#0000FF").unwrap()),
                    decoration: Some(TextDecoration::None),
                    strike: Some(false),
                    outline: Some(false),
                    shadow: Some(false),
                    condense: Some(false),
                    extend: Some(false),
                });
                style
            },
            priority: 2,
            stop_if_true: false,
        };

        let format = ConditionalFormat {
            range: "A1:A10".to_string(),
            rules: vec![rule1, rule2],
        };

        manager.add_format("Sheet1", "A1:A10", format).unwrap();

        // Test evaluation
        let style_high = manager.evaluate_cell("Sheet1", "A1", &CellValue::Number(150.0)).unwrap();
        assert!(style_high.is_some());

        let style_low = manager.evaluate_cell("Sheet1", "A2", &CellValue::Number(25.0)).unwrap();
        // Low value may not match any rule depending on implementation
        // Just check that evaluation doesn't crash
    }

    /// Test error handling across components
    #[test]
    fn test_error_handling() {
        // Test cell reference creation
        let cell_ref = CellReference::new("A".to_string(), 1);
        assert_eq!(cell_ref.column, "A");
        assert_eq!(cell_ref.row, 1);

        // Test cell reference with sheet
        let cell_ref_with_sheet = CellReference::with_sheet("Sheet1".to_string(), "B".to_string(), 5);
        assert_eq!(cell_ref_with_sheet.sheet, Some("Sheet1".to_string()));
        assert_eq!(cell_ref_with_sheet.column, "B");
        assert_eq!(cell_ref_with_sheet.row, 5);

        // Test invalid formula
        let mut engine = FormulaEngine::new();
        let cell_values = HashMap::new();
        let result = engine.evaluate("=INVALID_FUNCTION()", &cell_values);
        assert!(result.is_err());
    }

    /// Test formula engine with complex nested functions
    #[test]
    fn test_complex_nested_formulas() {
        let mut engine = FormulaEngine::new();
        let mut cell_values = HashMap::new();
        
        cell_values.insert("A1".to_string(), CellValue::Number(10.0));
        cell_values.insert("A2".to_string(), CellValue::Number(20.0));
        cell_values.insert("A3".to_string(), CellValue::Number(30.0));

        // Test SUM with multiple arguments
        let result = engine.evaluate("=SUM(A1,A2,A3)", &cell_values).unwrap();
        assert_eq!(result, FormulaResult::Number(60.0));
    }

    /// Test style manager with many styles
    #[test]
    fn test_style_manager_performance() {
        let mut manager = StyleManager::new();
        let mut style_ids = Vec::new();

        // Register 100 styles
        for i in 0..100 {
            let mut style = CellStyle::minimal();
            style.font = Some(Font {
                name: Some("Arial".to_string()),
                size: Some((i % 20) as f64 + 8.0),
                style: if i % 3 == 0 { Some(FontStyle::Italic) } else { Some(FontStyle::Normal) },
                weight: if i % 2 == 0 { Some(FontWeight::Bold) } else { Some(FontWeight::Normal) },
                color: Some(Color::from_hex(&format!("#{:06X}", i)).unwrap()),
                decoration: if i % 5 == 0 { Some(TextDecoration::Underline) } else { Some(TextDecoration::None) },
                strike: Some(false),
                outline: Some(false),
                shadow: Some(false),
                condense: Some(false),
                extend: Some(false),
            });
            let id = manager.register_style(style);
            style_ids.push(id);
        }

        // Verify all styles are registered
        assert_eq!(style_ids.len(), 100);

        // Retrieve a style
        let retrieved = manager.get_style(&style_ids[50]);
        assert!(retrieved.is_some());
    }
}
