use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub default_value: Option<String>,
    pub required: bool,
    pub data_type: VariableType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VariableType {
    String,
    Number,
    Date,
    Boolean,
    Image,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeResult {
    pub merged_content: String,
    pub variables_used: Vec<String>,
    pub missing_variables: Vec<String>,
    pub success: bool,
    pub error: Option<String>,
}

pub struct TemplateEngine {
    // In production, use a proper template engine like handlebars or tera
}

impl TemplateEngine {
    pub fn new() -> Self {
        Self {}
    }

    /// Parse template and extract variables
    pub fn parse_template(&self, template: &str) -> Result<Vec<TemplateVariable>, String> {
        let mut variables = Vec::new();
        let mut seen_names = std::collections::HashSet::new();

        // Find variables in format {{variable_name}}
        let re =
            Regex::new(r"\{\{([^}]+)\}\}").map_err(|e| format!("Failed to create regex: {}", e))?;

        for cap in re.captures_iter(template) {
            if let Some(var_name) = cap.get(1) {
                let name = var_name.as_str().trim();

                if !seen_names.contains(name) {
                    seen_names.insert(name.to_string());

                    variables.push(TemplateVariable {
                        name: name.to_string(),
                        default_value: None,
                        required: true,
                        data_type: VariableType::String,
                    });
                }
            }
        }

        Ok(variables)
    }

    /// Merge template with data
    pub fn merge(
        &self,
        template: &str,
        data: &HashMap<String, String>,
    ) -> Result<MergeResult, String> {
        let variables = self.parse_template(template)?;
        let mut variables_used = Vec::new();
        let mut missing_variables = Vec::new();
        let mut merged_content = template.to_string();

        for variable in &variables {
            if let Some(value) = data.get(&variable.name) {
                if !value.is_empty() {
                    variables_used.push(variable.name.clone());
                    let placeholder = format!("{{{{{}}}}}", variable.name);
                    merged_content = merged_content.replace(&placeholder, value);
                } else if let Some(default) = &variable.default_value {
                    variables_used.push(variable.name.clone());
                    let placeholder = format!("{{{{{}}}}}", variable.name);
                    merged_content = merged_content.replace(&placeholder, default);
                } else if variable.required {
                    missing_variables.push(variable.name.clone());
                }
            } else if let Some(default) = &variable.default_value {
                variables_used.push(variable.name.clone());
                let placeholder = format!("{{{{{}}}}}", variable.name);
                merged_content = merged_content.replace(&placeholder, default);
            } else if variable.required {
                missing_variables.push(variable.name.clone());
            }
        }

        let success = missing_variables.is_empty();
        let missing_vars_clone = missing_variables.clone();

        Ok(MergeResult {
            merged_content,
            variables_used,
            missing_variables,
            success,
            error: if !success {
                Some(format!(
                    "Missing required variables: {}",
                    missing_vars_clone.join(", ")
                ))
            } else {
                None
            },
        })
    }

    /// Validate template syntax
    pub fn validate_template(&self, template: &str) -> Result<(), String> {
        let re =
            Regex::new(r"\{\{([^}]+)\}\}").map_err(|e| format!("Failed to create regex: {}", e))?;

        let mut _open_count = 0;
        let _close_count = 0;

        for cap in re.captures_iter(template) {
            if let Some(_) = cap.get(0) {
                _open_count += 1;
            }
        }

        // Check for balanced braces
        let mut brace_count = 0;
        for c in template.chars() {
            match c {
                '{' => brace_count += 1,
                '}' => brace_count -= 1,
                _ => {}
            }
        }

        if brace_count != 0 {
            return Err("Unbalanced braces in template".to_string());
        }

        Ok(())
    }

    /// Create a template from content
    #[allow(dead_code)]
    pub fn create_template(&self, content: String, _name: String) -> Result<String, String> {
        self.validate_template(&content)?;

        let template_id = format!(
            "template-{}-{}",
            chrono::Utc::now().timestamp_millis(),
            rand::random::<u32>()
        );

        // In production, this would save to a database
        Ok(template_id)
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_template() {
        let engine = TemplateEngine::new();
        let template = "Hello {{name}}, your order {{order_id}} is ready.";
        let variables = engine.parse_template(template).unwrap();
        assert_eq!(variables.len(), 2);
    }

    #[test]
    fn test_merge() {
        let engine = TemplateEngine::new();
        let template = "Hello {{name}}";
        let mut data = HashMap::new();
        data.insert("name".to_string(), "World".to_string());

        let result = engine.merge(template, &data).unwrap();
        assert_eq!(result.merged_content, "Hello World");
        assert!(result.success);
    }

    #[test]
    fn test_merge_missing_variable() {
        let engine = TemplateEngine::new();
        let template = "Hello {{name}}";
        let data = HashMap::new();

        let result = engine.merge(template, &data).unwrap();
        assert!(!result.success);
        assert!(result.missing_variables.contains(&"name".to_string()));
    }

    #[test]
    fn test_engine_creation() {
        let engine = TemplateEngine::new();
        assert!(engine.parse_template("").is_ok());
    }

    #[test]
    fn test_engine_default() {
        let engine = TemplateEngine::default();
        assert!(engine.parse_template("").is_ok());
    }

    #[test]
    fn test_parse_template_empty() {
        let engine = TemplateEngine::new();
        let template = "";
        let variables = engine.parse_template(template).unwrap();
        assert!(variables.is_empty());
    }

    #[test]
    fn test_parse_template_no_variables() {
        let engine = TemplateEngine::new();
        let template = "Hello World";
        let variables = engine.parse_template(template).unwrap();
        assert!(variables.is_empty());
    }

    #[test]
    fn test_parse_template_single_variable() {
        let engine = TemplateEngine::new();
        let template = "Hello {{name}}";
        let variables = engine.parse_template(template).unwrap();
        assert_eq!(variables.len(), 1);
        assert_eq!(variables[0].name, "name");
    }

    #[test]
    fn test_parse_template_duplicate_variables() {
        let engine = TemplateEngine::new();
        let template = "Hello {{name}}, {{name}} is here";
        let variables = engine.parse_template(template).unwrap();
        assert_eq!(variables.len(), 1);
    }

    #[test]
    fn test_parse_template_whitespace() {
        let engine = TemplateEngine::new();
        let template = "Hello {{  name  }}";
        let variables = engine.parse_template(template).unwrap();
        assert_eq!(variables.len(), 1);
        assert_eq!(variables[0].name, "name");
    }

    #[test]
    fn test_merge_multiple_variables() {
        let engine = TemplateEngine::new();
        let template = "Hello {{name}}, your order {{order_id}} is ready.";
        let mut data = HashMap::new();
        data.insert("name".to_string(), "John".to_string());
        data.insert("order_id".to_string(), "12345".to_string());

        let result = engine.merge(template, &data).unwrap();
        assert!(result.success);
        assert_eq!(result.variables_used.len(), 2);
    }

    #[test]
    fn test_merge_partial_data() {
        let engine = TemplateEngine::new();
        let template = "Hello {{name}}, your order {{order_id}} is ready.";
        let mut data = HashMap::new();
        data.insert("name".to_string(), "John".to_string());

        let result = engine.merge(template, &data).unwrap();
        assert!(!result.success);
        assert!(result.missing_variables.contains(&"order_id".to_string()));
    }

    #[test]
    fn test_merge_empty_value() {
        let engine = TemplateEngine::new();
        let template = "Hello {{name}}";
        let mut data = HashMap::new();
        data.insert("name".to_string(), "".to_string());

        let result = engine.merge(template, &data).unwrap();
        assert!(!result.success);
    }

    #[test]
    fn test_validate_template_valid() {
        let engine = TemplateEngine::new();
        let template = "Hello {{name}}";
        let result = engine.validate_template(template);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_template_unbalanced_braces() {
        let engine = TemplateEngine::new();
        let template = "Hello {{name";
        let result = engine.validate_template(template);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_template_unbalanced_braces_close() {
        let engine = TemplateEngine::new();
        let template = "Hello name}}";
        let result = engine.validate_template(template);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_template() {
        let engine = TemplateEngine::new();
        let result = engine.create_template("Hello {{name}}".to_string(), "test".to_string());
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_create_template_invalid() {
        let engine = TemplateEngine::new();
        let result = engine.create_template("Hello {{name".to_string(), "test".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_template_variable_creation() {
        let variable = TemplateVariable {
            name: "test".to_string(),
            default_value: Some("default".to_string()),
            required: true,
            data_type: VariableType::String,
        };
        assert_eq!(variable.name, "test");
        assert!(variable.required);
    }

    #[test]
    fn test_template_variable_serialization() {
        let variable = TemplateVariable {
            name: "test".to_string(),
            default_value: Some("default".to_string()),
            required: true,
            data_type: VariableType::String,
        };
        let json = serde_json::to_string(&variable);
        assert!(json.is_ok());
    }

    #[test]
    fn test_template_variable_deserialization() {
        let json = r#"{
            "name": "test",
            "default_value": "default",
            "required": true,
            "data_type": "string"
        }"#;
        let variable: Result<TemplateVariable, _> = serde_json::from_str(json);
        assert!(variable.is_ok());
    }

    #[test]
    fn test_variable_type_variants() {
        let string_type = VariableType::String;
        let number_type = VariableType::Number;
        let date_type = VariableType::Date;
        let boolean_type = VariableType::Boolean;
        let image_type = VariableType::Image;

        let _ = (
            string_type,
            number_type,
            date_type,
            boolean_type,
            image_type,
        );
    }

    #[test]
    fn test_variable_type_serialization() {
        let var_type = VariableType::String;
        let json = serde_json::to_string(&var_type);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"string\"");
    }

    #[test]
    fn test_variable_type_deserialization() {
        let var_type: VariableType = serde_json::from_str("\"string\"").unwrap();
        assert!(matches!(var_type, VariableType::String));
    }

    #[test]
    fn test_merge_result_creation() {
        let result = MergeResult {
            merged_content: "Hello World".to_string(),
            variables_used: vec!["name".to_string()],
            missing_variables: vec![],
            success: true,
            error: None,
        };
        assert!(result.success);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_merge_result_with_error() {
        let result = MergeResult {
            merged_content: "Hello {{name}}".to_string(),
            variables_used: vec![],
            missing_variables: vec!["name".to_string()],
            success: false,
            error: Some("Missing required variables: name".to_string()),
        };
        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_merge_result_serialization() {
        let result = MergeResult {
            merged_content: "Hello World".to_string(),
            variables_used: vec!["name".to_string()],
            missing_variables: vec![],
            success: true,
            error: None,
        };
        let json = serde_json::to_string(&result);
        assert!(json.is_ok());
    }

    #[test]
    fn test_merge_result_deserialization() {
        let json = r#"{
            "merged_content": "Hello World",
            "variables_used": ["name"],
            "missing_variables": [],
            "success": true,
            "error": null
        }"#;
        let result: Result<MergeResult, _> = serde_json::from_str(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_template_with_special_chars() {
        let engine = TemplateEngine::new();
        let template = "Hello {{first_name}} {{last_name}}";
        let variables = engine.parse_template(template).unwrap();
        assert_eq!(variables.len(), 2);
    }

    #[test]
    fn test_merge_with_default_value() {
        let engine = TemplateEngine::new();
        let template = "Hello {{name}}";
        let data = HashMap::new();

        // This test shows the limitation - we can't directly set default values in the current API
        // But we can test the merge with empty data
        let result = engine.merge(template, &data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_template_no_variables() {
        let engine = TemplateEngine::new();
        let template = "Hello World";
        let result = engine.validate_template(template);
        assert!(result.is_ok());
    }

    #[test]
    fn test_merge_preserves_non_variable_text() {
        let engine = TemplateEngine::new();
        let template = "Hello {{name}}, welcome to our service!";
        let mut data = HashMap::new();
        data.insert("name".to_string(), "John".to_string());

        let result = engine.merge(template, &data).unwrap();
        assert!(result.merged_content.contains("welcome to our service"));
    }

    #[test]
    fn test_parse_template_nested_braces() {
        let engine = TemplateEngine::new();
        let template = "Hello {{name}} {{order_id}}";
        let variables = engine.parse_template(template).unwrap();
        assert_eq!(variables.len(), 2);
    }

    #[test]
    fn test_merge_case_sensitive() {
        let engine = TemplateEngine::new();
        let template = "Hello {{Name}}";
        let mut data = HashMap::new();
        data.insert("name".to_string(), "John".to_string());

        let result = engine.merge(template, &data).unwrap();
        assert!(!result.success);
    }

    #[test]
    fn test_template_variable_without_default() {
        let variable = TemplateVariable {
            name: "test".to_string(),
            default_value: None,
            required: true,
            data_type: VariableType::String,
        };
        assert!(variable.default_value.is_none());
    }

    #[test]
    fn test_template_variable_not_required() {
        let variable = TemplateVariable {
            name: "test".to_string(),
            default_value: Some("default".to_string()),
            required: false,
            data_type: VariableType::String,
        };
        assert!(!variable.required);
    }

    #[test]
    fn test_variable_type_number() {
        let var_type = VariableType::Number;
        let json = serde_json::to_string(&var_type).unwrap();
        assert_eq!(json, "\"number\"");
    }

    #[test]
    fn test_variable_type_date() {
        let var_type = VariableType::Date;
        let json = serde_json::to_string(&var_type).unwrap();
        assert_eq!(json, "\"date\"");
    }

    #[test]
    fn test_variable_type_boolean() {
        let var_type = VariableType::Boolean;
        let json = serde_json::to_string(&var_type).unwrap();
        assert_eq!(json, "\"boolean\"");
    }

    #[test]
    fn test_variable_type_image() {
        let var_type = VariableType::Image;
        let json = serde_json::to_string(&var_type).unwrap();
        assert_eq!(json, "\"image\"");
    }

    #[test]
    fn test_merge_result_variables_used() {
        let engine = TemplateEngine::new();
        let template = "Hello {{name}}";
        let mut data = HashMap::new();
        data.insert("name".to_string(), "John".to_string());

        let result = engine.merge(template, &data).unwrap();
        assert_eq!(result.variables_used.len(), 1);
        assert_eq!(result.variables_used[0], "name");
    }

    #[test]
    fn test_merge_result_missing_variables_list() {
        let engine = TemplateEngine::new();
        let template = "Hello {{name}}, your order {{order_id}} is ready";
        let mut data = HashMap::new();
        data.insert("name".to_string(), "John".to_string());

        let result = engine.merge(template, &data).unwrap();
        assert_eq!(result.missing_variables.len(), 1);
        assert_eq!(result.missing_variables[0], "order_id");
    }

    #[test]
    fn test_merge_error_message() {
        let engine = TemplateEngine::new();
        let template = "Hello {{name}}";
        let data = HashMap::new();

        let result = engine.merge(template, &data).unwrap();
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("name"));
    }
}
