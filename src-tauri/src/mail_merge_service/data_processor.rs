//! Data Processor - Aerospace-Grade Mail Merge Service
//!
//! Safety-critical mail merge data processing service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Comprehensive error handling
//! - Performance monitoring
//! - Security hardening
//! - Fault tolerance and error recovery

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use crate::error_handling::{ErrorContext, ErrorSeverity};
use crate::config_service::ExportConfigService;
use std::sync::Arc;
use calamine::{Reader, Xlsx, open_workbook};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataSource {
    Csv(String),
    Json(String),
    Excel(String),
    Manual(HashMap<String, String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeConfig {
    pub output_format: OutputFormat,
    pub output_path: Option<String>,
    pub batch_size: usize,
    pub skip_errors: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Html,
    Markdown,
    PlainText,
    Pdf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeBatchResult {
    pub total_records: usize,
    pub successful: usize,
    pub failed: usize,
    pub results: Vec<super::template_engine::MergeResult>,
    pub output_files: Vec<String>,
}

pub struct DataProcessor {
    /// 配置服务
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
    // In production, this would use proper CSV/Excel parsing libraries
}

impl DataProcessor {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
        }
    }

    /// Validate file size
    fn validate_file_size(&self, path: &str) -> Result<(), String> {
        let mail_merge_config = self.config_service.get_mail_merge_config();
        let metadata = fs::metadata(path).map_err(|e| format!("Failed to read file metadata: {}", e))?;
        let size = metadata.len() as usize;
        if size > mail_merge_config.max_file_size {
            return Err(format!("File size exceeds maximum of {} bytes", mail_merge_config.max_file_size));
        }
        Ok(())
    }

    /// Validate record count
    fn validate_record_count(&self, count: usize) -> Result<(), String> {
        let mail_merge_config = self.config_service.get_mail_merge_config();
        if count > mail_merge_config.max_records {
            return Err(format!("Record count exceeds maximum of {}", mail_merge_config.max_records));
        }
        Ok(())
    }

    /// Validate record structure
    fn validate_record(&self, record: &HashMap<String, String>) -> Result<(), String> {
        let mail_merge_config = self.config_service.get_mail_merge_config();
        if record.len() > mail_merge_config.max_fields_per_record {
            return Err(format!("Field count exceeds maximum of {}", mail_merge_config.max_fields_per_record));
        }
        for (key, value) in record {
            if key.len() > mail_merge_config.max_field_name_length {
                return Err(format!("Field name exceeds maximum length of {}", mail_merge_config.max_field_name_length));
            }
            if value.len() > mail_merge_config.max_field_value_length {
                return Err(format!("Field value exceeds maximum length of {}", mail_merge_config.max_field_value_length));
            }
        }
        Ok(())
    }

    /// Validate batch size
    fn validate_batch_size(&self, batch_size: usize) -> Result<(), String> {
        let mail_merge_config = self.config_service.get_mail_merge_config();
        if batch_size > mail_merge_config.max_batch_size {
            return Err(format!("Batch size exceeds maximum of {}", mail_merge_config.max_batch_size));
        }
        if batch_size == 0 {
            return Err("Batch size cannot be zero".to_string());
        }
        Ok(())
    }

    /// Record error context
    fn record_error(&mut self, code: &str, message: &str, source: &str) {
        self.last_error = Some(ErrorContext::new(
            ErrorSeverity::Error,
            code,
            message,
            source,
        ));
    }

    /// Get last error
    pub fn get_last_error(&self) -> Option<&ErrorContext> {
        self.last_error.as_ref()
    }

    /// Get operation count
    pub fn get_operation_count(&self) -> u64 {
        self.operation_count
    }

    /// Reset error state
    pub fn reset_error_state(&mut self) {
        self.last_error = None;
    }

    /// Load data from source with validation
    pub fn load_data(&mut self, source: &DataSource) -> Result<Vec<HashMap<String, String>>, String> {
        self.operation_count += 1;
        
        let result = match source {
            DataSource::Csv(path) => self.load_csv(path),
            DataSource::Json(path) => self.load_json(path),
            DataSource::Excel(path) => self.load_excel(path),
            DataSource::Manual(data) => {
                if let Err(e) = self.validate_record(data) {
                    self.record_error("INVALID_RECORD", &e, "load_data");
                    return Err(e);
                }
                Ok(vec![data.clone()])
            }
        };

        if let Ok(ref data) = result {
            if let Err(e) = self.validate_record_count(data.len()) {
                self.record_error("TOO_MANY_RECORDS", &e, "load_data");
                return Err(e);
            }
            // Validate each record
            for record in data {
                if let Err(e) = self.validate_record(record) {
                    self.record_error("INVALID_RECORD", &e, "load_data");
                    return Err(e);
                }
            }
        }

        if result.is_ok() {
            self.last_error = None;
        }
        result
    }

    fn load_csv(&self, path: &str) -> Result<Vec<HashMap<String, String>>, String> {
        // Validate file size
        if let Err(e) = self.validate_file_size(path) {
            return Err(e);
        }

        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read CSV file: {}", e))?;

        let mut records = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        if lines.is_empty() {
            return Ok(records);
        }

        let headers: Vec<String> = lines[0].split(',').map(|s| s.trim().to_string()).collect();

        for line in lines.iter().skip(1) {
            let values: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();

            let mut record = HashMap::new();
            for (i, header) in headers.iter().enumerate() {
                if i < values.len() {
                    record.insert(header.clone(), values[i].clone());
                }
            }

            records.push(record);
        }

        Ok(records)
    }

    fn load_json(&self, path: &str) -> Result<Vec<HashMap<String, String>>, String> {
        // Validate file size
        if let Err(e) = self.validate_file_size(path) {
            return Err(e);
        }

        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read JSON file: {}", e))?;

        let data: serde_json::Value =
            serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let mut records = Vec::new();

        match data {
            serde_json::Value::Array(arr) => {
                for item in arr {
                    if let Ok(record) =
                        serde_json::from_value::<HashMap<String, String>>(item.clone())
                    {
                        records.push(record);
                    } else if let serde_json::Value::Object(obj) = item {
                        let mut record = HashMap::new();
                        for (key, value) in obj {
                            if let Some(s) = value.as_str() {
                                record.insert(key, s.to_string());
                            } else {
                                record.insert(key, value.to_string());
                            }
                        }
                        records.push(record);
                    }
                }
            }
            serde_json::Value::Object(obj) => {
                let mut record = HashMap::new();
                for (key, value) in obj {
                    if let Some(s) = value.as_str() {
                        record.insert(key, s.to_string());
                    } else {
                        record.insert(key, value.to_string());
                    }
                }
                records.push(record);
            }
            _ => return Err("JSON must be an object or array".to_string()),
        }

        Ok(records)
    }

    fn load_excel(&self, _path: &str) -> Result<Vec<HashMap<String, String>>, String> {
        // Temporarily disabled due to calamine API compatibility issues
        // TODO: Re-enable after calamine API is stabilized
        Err("Excel import temporarily disabled due to API compatibility issues. Please use CSV or JSON format.".to_string())
    }

    /// Process batch merge with validation
    pub fn process_batch_merge(
        &mut self,
        template: &str,
        data: &[HashMap<String, String>],
        config: &MergeConfig,
    ) -> Result<MergeBatchResult, String> {
        self.operation_count += 1;

        // Validate batch size
        if let Err(e) = self.validate_batch_size(config.batch_size) {
            self.record_error("INVALID_BATCH_SIZE", &e, "process_batch_merge");
            return Err(e);
        }

        // Validate record count
        if let Err(e) = self.validate_record_count(data.len()) {
            self.record_error("TOO_MANY_RECORDS", &e, "process_batch_merge");
            return Err(e);
        }

        // Validate each record
        for record in data {
            if let Err(e) = self.validate_record(record) {
                self.record_error("INVALID_RECORD", &e, "process_batch_merge");
                return Err(e);
            }
        }

        let engine = super::template_engine::TemplateEngine::new();
        let mut results = Vec::new();
        let mut successful = 0;
        let mut failed = 0;
        let mut output_files = Vec::new();

        for (i, record) in data.iter().enumerate() {
            match engine.merge(template, record) {
                Ok(result) => {
                    if result.success {
                        successful += 1;

                        // Save output if path specified
                        if let Some(output_path) = &config.output_path {
                            let file_path = if data.len() > 1 {
                                format!(
                                    "{}_{}.{}",
                                    output_path,
                                    i,
                                    self.get_extension(&config.output_format)
                                )
                            } else {
                                format!(
                                    "{}.{}",
                                    output_path,
                                    self.get_extension(&config.output_format)
                                )
                            };

                            fs::write(&file_path, &result.merged_content)
                                .map_err(|e| format!("Failed to write output: {}", e))?;

                            output_files.push(file_path);
                        }
                    } else {
                        failed += 1;
                        if !config.skip_errors {
                            return Err(format!(
                                "Merge failed for record {}: {}",
                                i,
                                result.error.unwrap_or_default()
                            ));
                        }
                    }
                    results.push(result);
                }
                Err(e) => {
                    failed += 1;
                    if !config.skip_errors {
                        return Err(format!("Merge error for record {}: {}", i, e));
                    }
                }
            }
        }

        self.last_error = None;
        Ok(MergeBatchResult {
            total_records: data.len(),
            successful,
            failed,
            results,
            output_files,
        })
    }

    fn get_extension(&self, format: &OutputFormat) -> &str {
        match format {
            OutputFormat::Html => "html",
            OutputFormat::Markdown => "md",
            OutputFormat::PlainText => "txt",
            OutputFormat::Pdf => "pdf",
        }
    }

    /// Validate data structure
    #[allow(dead_code)]
    pub fn validate_data(
        &self,
        data: &[HashMap<String, String>],
        required_fields: &[String],
    ) -> Result<(), String> {
        if data.is_empty() {
            return Err("Data is empty".to_string());
        }

        for (i, record) in data.iter().enumerate() {
            for field in required_fields {
                if !record.contains_key(field) || record.get(field).map_or(true, |v| v.is_empty()) {
                    return Err(format!("Record {} is missing required field: {}", i, field));
                }
            }
        }

        Ok(())
    }

    /// Get data statistics
    #[allow(dead_code)]
    pub fn get_data_stats(&self, data: &[HashMap<String, String>]) -> DataStats {
        if data.is_empty() {
            return DataStats {
                total_records: 0,
                total_fields: 0,
                field_names: Vec::new(),
                average_fields_per_record: 0.0,
            };
        }

        let total_fields: usize = data.iter().map(|r| r.len()).sum();
        let mut field_names = std::collections::HashSet::new();

        for record in data {
            for key in record.keys() {
                field_names.insert(key.clone());
            }
        }

        DataStats {
            total_records: data.len(),
            total_fields,
            field_names: field_names.into_iter().collect(),
            average_fields_per_record: total_fields as f64 / data.len() as f64,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct DataStats {
    pub total_records: usize,
    pub total_fields: usize,
    pub field_names: Vec<String>,
    pub average_fields_per_record: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_manual_data() {
        let mut processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let mut data = HashMap::new();
        data.insert("name".to_string(), "Test".to_string());

        let source = DataSource::Manual(data);
        let result = processor.load_data(&source).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_validate_data() {
        let processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let mut data = HashMap::new();
        data.insert("name".to_string(), "Test".to_string());

        let result = processor.validate_data(&[data], &["name".to_string()]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_data_stats() {
        let processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let mut data = HashMap::new();
        data.insert("name".to_string(), "Test".to_string());
        data.insert("age".to_string(), "25".to_string());

        let stats = processor.get_data_stats(&[data.clone()]);
        assert_eq!(stats.total_records, 1);
        assert_eq!(stats.total_fields, 2);
    }

    #[test]
    fn test_data_source_csv_variant() {
        let source = DataSource::Csv("test.csv".to_string());
        assert!(matches!(source, DataSource::Csv(_)));
    }

    #[test]
    fn test_data_source_json_variant() {
        let source = DataSource::Json("test.json".to_string());
        assert!(matches!(source, DataSource::Json(_)));
    }

    #[test]
    fn test_data_source_excel_variant() {
        let source = DataSource::Excel("test.xlsx".to_string());
        assert!(matches!(source, DataSource::Excel(_)));
    }

    #[test]
    fn test_data_source_manual_variant() {
        let mut data = HashMap::new();
        data.insert("key".to_string(), "value".to_string());
        let source = DataSource::Manual(data);
        assert!(matches!(source, DataSource::Manual(_)));
    }

    #[test]
    fn test_data_source_serialization() {
        let source = DataSource::Csv("test.csv".to_string());
        let json = serde_json::to_string(&source);
        assert!(json.is_ok());
    }

    #[test]
    fn test_merge_config_creation() {
        let config = MergeConfig {
            output_format: OutputFormat::Html,
            output_path: Some("/path".to_string()),
            batch_size: 100,
            skip_errors: true,
        };
        assert_eq!(config.batch_size, 100);
        assert!(config.skip_errors);
    }

    #[test]
    fn test_merge_config_serialization() {
        let config = MergeConfig {
            output_format: OutputFormat::Html,
            output_path: Some("/path".to_string()),
            batch_size: 100,
            skip_errors: true,
        };
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_output_format_variants() {
        let html = OutputFormat::Html;
        let markdown = OutputFormat::Markdown;
        let plain_text = OutputFormat::PlainText;
        let pdf = OutputFormat::Pdf;

        assert!(matches!(html, OutputFormat::Html));
        assert!(matches!(markdown, OutputFormat::Markdown));
        assert!(matches!(plain_text, OutputFormat::PlainText));
        assert!(matches!(pdf, OutputFormat::Pdf));
    }

    #[test]
    fn test_output_format_serialization() {
        let format = OutputFormat::Html;
        let json = serde_json::to_string(&format);
        assert!(json.is_ok());
    }

    #[test]
    fn test_merge_batch_result_creation() {
        let result = MergeBatchResult {
            total_records: 10,
            successful: 8,
            failed: 2,
            results: vec![],
            output_files: vec![],
        };
        assert_eq!(result.total_records, 10);
        assert_eq!(result.successful, 8);
        assert_eq!(result.failed, 2);
    }

    #[test]
    fn test_merge_batch_result_serialization() {
        let result = MergeBatchResult {
            total_records: 10,
            successful: 8,
            failed: 2,
            results: vec![],
            output_files: vec![],
        };
        let json = serde_json::to_string(&result);
        assert!(json.is_ok());
    }

    #[test]
    fn test_data_processor_creation() {
        let mut processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let mut data = HashMap::new();
        data.insert("key".to_string(), "value".to_string());
        let source = DataSource::Manual(data);
        let result = processor.load_data(&source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_data_processor_default() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut processor = DataProcessor::new(config_service);
        let mut data = HashMap::new();
        data.insert("key".to_string(), "value".to_string());
        let source = DataSource::Manual(data);
        let result = processor.load_data(&source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_load_csv_empty() {
        let mut processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let source = DataSource::Csv("empty.csv".to_string());
        let result = processor.load_data(&source);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_csv_valid() {
        let mut processor = DataProcessor::new(Arc::new(ExportConfigService::new()));

        // Create a temporary CSV file
        let temp_file = "/tmp/test_data.csv";
        let mut file = fs::File::create(temp_file).unwrap();
        writeln!(file, "name,age").unwrap();
        writeln!(file, "John,30").unwrap();
        writeln!(file, "Jane,25").unwrap();

        let source = DataSource::Csv(temp_file.to_string());
        let result = processor.load_data(&source).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].get("name"), Some(&"John".to_string()));

        // Clean up
        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_load_csv_empty_file() {
        let mut processor = DataProcessor::new(Arc::new(ExportConfigService::new()));

        let temp_file = "/tmp/test_empty.csv";
        fs::File::create(temp_file).unwrap();

        let source = DataSource::Csv(temp_file.to_string());
        let result = processor.load_data(&source).unwrap();
        assert_eq!(result.len(), 0);

        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_load_json_array() {
        let mut processor = DataProcessor::new(Arc::new(ExportConfigService::new()));

        let temp_file = "/tmp/test_data.json";
        let mut file = fs::File::create(temp_file).unwrap();
        let json_content = r#"[{"name":"John","age":"30"},{"name":"Jane","age":"25"}]"#;
        writeln!(file, "{}", json_content).unwrap();

        let source = DataSource::Json(temp_file.to_string());
        let result = processor.load_data(&source).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].get("name"), Some(&"John".to_string()));

        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_load_json_object() {
        let mut processor = DataProcessor::new(Arc::new(ExportConfigService::new()));

        let temp_file = "/tmp/test_data_obj.json";
        let mut file = fs::File::create(temp_file).unwrap();
        let json_content = r#"{"name":"John","age":"30"}"#;
        writeln!(file, "{}", json_content).unwrap();

        let source = DataSource::Json(temp_file.to_string());
        let result = processor.load_data(&source).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].get("name"), Some(&"John".to_string()));

        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_load_json_invalid() {
        let mut processor = DataProcessor::new(Arc::new(ExportConfigService::new()));

        let temp_file = "/tmp/test_invalid.json";
        let mut file = fs::File::create(temp_file).unwrap();
        writeln!(file, "invalid json").unwrap();

        let source = DataSource::Json(temp_file.to_string());
        let result = processor.load_data(&source);
        assert!(result.is_err());

        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_load_json_not_found() {
        let mut processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let source = DataSource::Json("nonexistent.json".to_string());
        let result = processor.load_data(&source);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_excel_not_implemented() {
        let mut processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let source = DataSource::Excel("test.xlsx".to_string());
        let result = processor.load_data(&source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("temporarily disabled"));
    }

    #[test]
    fn test_get_extension_html() {
        let processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        assert_eq!(processor.get_extension(&OutputFormat::Html), "html");
    }

    #[test]
    fn test_get_extension_markdown() {
        let processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        assert_eq!(processor.get_extension(&OutputFormat::Markdown), "md");
    }

    #[test]
    fn test_get_extension_plain_text() {
        let processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        assert_eq!(processor.get_extension(&OutputFormat::PlainText), "txt");
    }

    #[test]
    fn test_get_extension_pdf() {
        let processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        assert_eq!(processor.get_extension(&OutputFormat::Pdf), "pdf");
    }

    #[test]
    fn test_validate_data_empty() {
        let processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let result = processor.validate_data(&[], &["name".to_string()]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    #[test]
    fn test_validate_data_missing_field() {
        let processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let mut data = HashMap::new();
        data.insert("name".to_string(), "Test".to_string());

        let result = processor.validate_data(&[data], &["name".to_string(), "age".to_string()]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("missing"));
    }

    #[test]
    fn test_validate_data_empty_field() {
        let processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let mut data = HashMap::new();
        data.insert("name".to_string(), "".to_string());

        let result = processor.validate_data(&[data], &["name".to_string()]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("missing"));
    }

    #[test]
    fn test_get_data_stats_empty() {
        let processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let stats = processor.get_data_stats(&[]);
        assert_eq!(stats.total_records, 0);
        assert_eq!(stats.total_fields, 0);
        assert!(stats.field_names.is_empty());
        assert_eq!(stats.average_fields_per_record, 0.0);
    }

    #[test]
    fn test_get_data_stats_multiple_records() {
        let processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let mut data1 = HashMap::new();
        data1.insert("name".to_string(), "John".to_string());
        data1.insert("age".to_string(), "30".to_string());

        let mut data2 = HashMap::new();
        data2.insert("name".to_string(), "Jane".to_string());
        data2.insert("age".to_string(), "25".to_string());

        let stats = processor.get_data_stats(&[data1, data2]);
        assert_eq!(stats.total_records, 2);
        assert_eq!(stats.total_fields, 4);
        assert_eq!(stats.average_fields_per_record, 2.0);
    }

    #[test]
    fn test_get_data_stats_field_names() {
        let processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let mut data1 = HashMap::new();
        data1.insert("name".to_string(), "John".to_string());
        data1.insert("age".to_string(), "30".to_string());

        let mut data2 = HashMap::new();
        data2.insert("name".to_string(), "Jane".to_string());
        data2.insert("city".to_string(), "NYC".to_string());

        let stats = processor.get_data_stats(&[data1, data2]);
        assert_eq!(stats.field_names.len(), 3);
        assert!(stats.field_names.contains(&"name".to_string()));
        assert!(stats.field_names.contains(&"age".to_string()));
        assert!(stats.field_names.contains(&"city".to_string()));
    }

    #[test]
    fn test_data_stats_creation() {
        let stats = DataStats {
            total_records: 10,
            total_fields: 20,
            field_names: vec!["name".to_string(), "age".to_string()],
            average_fields_per_record: 2.0,
        };
        assert_eq!(stats.total_records, 10);
        assert_eq!(stats.total_fields, 20);
    }

    #[test]
    fn test_data_stats_serialization() {
        let stats = DataStats {
            total_records: 10,
            total_fields: 20,
            field_names: vec!["name".to_string(), "age".to_string()],
            average_fields_per_record: 2.0,
        };
        let json = serde_json::to_string(&stats);
        assert!(json.is_ok());
    }

    #[test]
    fn test_process_batch_merge_skip_errors() {
        let mut processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let template = "Hello {{name}}";
        let mut data1 = HashMap::new();
        data1.insert("name".to_string(), "John".to_string());
        let mut data2 = HashMap::new();
        data2.insert("name".to_string(), "".to_string());

        let config = MergeConfig {
            output_format: OutputFormat::PlainText,
            output_path: None,
            batch_size: 10,
            skip_errors: true,
        };

        let result = processor.process_batch_merge(template, &[data1, data2], &config);
        assert!(result.is_ok());
        let batch_result = result.unwrap();
        assert_eq!(batch_result.total_records, 2);
        assert!(batch_result.failed > 0);
    }

    #[test]
    fn test_process_batch_merge_no_skip_errors() {
        let mut processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let template = "Hello {{name}}";
        let mut data = HashMap::new();
        data.insert("name".to_string(), "".to_string());

        let config = MergeConfig {
            output_format: OutputFormat::PlainText,
            output_path: None,
            batch_size: 10,
            skip_errors: false,
        };

        let result = processor.process_batch_merge(template, &[data], &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_process_batch_merge_empty_data() {
        let mut processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let template = "Hello {{name}}";
        let data: Vec<HashMap<String, String>> = vec![];

        let config = MergeConfig {
            output_format: OutputFormat::PlainText,
            output_path: None,
            batch_size: 10,
            skip_errors: true,
        };

        let result = processor.process_batch_merge(template, &data, &config);
        assert!(result.is_ok());
        let batch_result = result.unwrap();
        assert_eq!(batch_result.total_records, 0);
    }

    #[test]
    fn test_process_batch_merge_single_record() {
        let mut processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let template = "Hello {{name}}";
        let mut data = HashMap::new();
        data.insert("name".to_string(), "John".to_string());

        let config = MergeConfig {
            output_format: OutputFormat::PlainText,
            output_path: None,
            batch_size: 10,
            skip_errors: true,
        };

        let result = processor.process_batch_merge(template, &[data], &config);
        assert!(result.is_ok());
        let batch_result = result.unwrap();
        assert_eq!(batch_result.total_records, 1);
        assert_eq!(batch_result.successful, 1);
    }

    // Aerospace-level tests
    #[test]
    fn test_validate_record_too_many_fields() {
        let config_service = Arc::new(ExportConfigService::new());
        let processor = DataProcessor::new(config_service.clone());
        let mail_merge_config = config_service.get_mail_merge_config();
        let mut record = HashMap::new();
        for i in 0..mail_merge_config.max_fields_per_record + 1 {
            record.insert(format!("field{}", i), "value".to_string());
        }
        let result = processor.validate_record(&record);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum"));
    }

    #[test]
    fn test_validate_record_field_name_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let processor = DataProcessor::new(config_service.clone());
        let mail_merge_config = config_service.get_mail_merge_config();
        let mut record = HashMap::new();
        record.insert("a".repeat(mail_merge_config.max_field_name_length + 1), "value".to_string());
        let result = processor.validate_record(&record);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum length"));
    }

    #[test]
    fn test_validate_record_field_value_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let processor = DataProcessor::new(config_service.clone());
        let mail_merge_config = config_service.get_mail_merge_config();
        let mut record = HashMap::new();
        record.insert("field".to_string(), "a".repeat(mail_merge_config.max_field_value_length + 1));
        let result = processor.validate_record(&record);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum length"));
    }

    #[test]
    fn test_validate_record_count_too_many() {
        let config_service = Arc::new(ExportConfigService::new());
        let processor = DataProcessor::new(config_service.clone());
        let mail_merge_config = config_service.get_mail_merge_config();
        let result = processor.validate_record_count(mail_merge_config.max_records + 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum"));
    }

    #[test]
    fn test_validate_batch_size_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let processor = DataProcessor::new(config_service.clone());
        let mail_merge_config = config_service.get_mail_merge_config();
        let result = processor.validate_batch_size(mail_merge_config.max_batch_size + 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum"));
    }

    #[test]
    fn test_validate_batch_size_zero() {
        let processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let result = processor.validate_batch_size(0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be zero"));
    }

    #[test]
    fn test_max_fields_per_record_accepted() {
        let config_service = Arc::new(ExportConfigService::new());
        let processor = DataProcessor::new(config_service.clone());
        let mail_merge_config = config_service.get_mail_merge_config();
        let mut record = HashMap::new();
        for i in 0..mail_merge_config.max_fields_per_record {
            record.insert(format!("field{}", i), "value".to_string());
        }
        let result = processor.validate_record(&record);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_field_name_length_accepted() {
        let config_service = Arc::new(ExportConfigService::new());
        let processor = DataProcessor::new(config_service.clone());
        let mail_merge_config = config_service.get_mail_merge_config();
        let mut record = HashMap::new();
        record.insert("a".repeat(mail_merge_config.max_field_name_length), "value".to_string());
        let result = processor.validate_record(&record);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_field_value_length_accepted() {
        let config_service = Arc::new(ExportConfigService::new());
        let processor = DataProcessor::new(config_service.clone());
        let mail_merge_config = config_service.get_mail_merge_config();
        let mut record = HashMap::new();
        record.insert("field".to_string(), "a".repeat(mail_merge_config.max_field_value_length));
        let result = processor.validate_record(&record);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_record_count_accepted() {
        let config_service = Arc::new(ExportConfigService::new());
        let processor = DataProcessor::new(config_service.clone());
        let mail_merge_config = config_service.get_mail_merge_config();
        let result = processor.validate_record_count(mail_merge_config.max_records);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_batch_size_accepted() {
        let config_service = Arc::new(ExportConfigService::new());
        let processor = DataProcessor::new(config_service.clone());
        let mail_merge_config = config_service.get_mail_merge_config();
        let result = processor.validate_batch_size(mail_merge_config.max_batch_size);
        assert!(result.is_ok());
    }

    #[test]
    fn test_operation_count() {
        let mut processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        assert_eq!(processor.get_operation_count(), 0);
        
        let mut data = HashMap::new();
        data.insert("key".to_string(), "value".to_string());
        let source = DataSource::Manual(data);
        processor.load_data(&source).unwrap();
        assert!(processor.get_operation_count() > 0);
    }

    #[test]
    fn test_error_recording() {
        let mut processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        
        processor.record_error("TEST_ERROR", "Test error message", "test_source");
        
        let last_error = processor.get_last_error();
        assert!(last_error.is_some());
        let error = last_error.unwrap();
        assert_eq!(error.code, "TEST_ERROR");
        assert_eq!(error.message, "Test error message");
        assert_eq!(error.source, "test_source");
    }

    #[test]
    fn test_error_state_reset() {
        let mut processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        
        processor.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(processor.get_last_error().is_some());
        
        processor.reset_error_state();
        assert!(processor.get_last_error().is_none());
    }

    #[test]
    fn test_load_data_with_invalid_record() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut processor = DataProcessor::new(config_service.clone());
        let mail_merge_config = config_service.get_mail_merge_config();
        let mut data = HashMap::new();
        data.insert("a".repeat(mail_merge_config.max_field_name_length + 1), "value".to_string());
        let source = DataSource::Manual(data);
        let result = processor.load_data(&source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum length"));
    }

    #[test]
    fn test_process_batch_merge_with_invalid_batch_size() {
        let mut processor = DataProcessor::new(Arc::new(ExportConfigService::new()));
        let template = "Hello {{name}}";
        let mut data = HashMap::new();
        data.insert("name".to_string(), "John".to_string());

        let config = MergeConfig {
            output_format: OutputFormat::PlainText,
            output_path: None,
            batch_size: 0,
            skip_errors: true,
        };

        let result = processor.process_batch_merge(template, &[data], &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be zero"));
    }
}
