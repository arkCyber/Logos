/*!
 * 航空航天级数据加载系统
 * 实现 Typst 的数据加载功能（JSON、YAML、CSV、TOML、CBOR、XML）
 */

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DataType {
    Json,
    Yaml,
    Csv,
    Toml,
    Cbor,
    Xml,
}

impl DataType {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "json" => Some(DataType::Json),
            "yaml" | "yml" => Some(DataType::Yaml),
            "csv" => Some(DataType::Csv),
            "toml" => Some(DataType::Toml),
            "cbor" => Some(DataType::Cbor),
            "xml" => Some(DataType::Xml),
            _ => None,
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            DataType::Json => "json",
            DataType::Yaml => "yaml",
            DataType::Csv => "csv",
            DataType::Toml => "toml",
            DataType::Cbor => "cbor",
            DataType::Xml => "xml",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataValue {
    pub data_type: DataType,
    pub value: JsonValue,
    pub source: Option<String>,
}

pub struct DataLoader {
    cache: HashMap<String, DataValue>,
}

impl DataLoader {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// 从文件加载数据
    pub fn load_from_file(&mut self, path: PathBuf) -> Result<DataValue, String> {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| "Invalid file extension".to_string())?;

        let data_type = DataType::from_extension(extension)
            .ok_or_else(|| format!("Unsupported file type: {}", extension))?;

        let content =
            fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))?;

        self.load_from_string(
            &content,
            data_type,
            Some(path.to_string_lossy().to_string()),
        )
    }

    /// 从字符串加载数据
    pub fn load_from_string(
        &mut self,
        content: &str,
        data_type: DataType,
        source: Option<String>,
    ) -> Result<DataValue, String> {
        let value = match data_type {
            DataType::Json => {
                serde_json::from_str(content).map_err(|e| format!("JSON parse error: {}", e))?
            }
            DataType::Yaml => {
                let yaml_value: serde_yaml::Value = serde_yaml::from_str(content)
                    .map_err(|e| format!("YAML parse error: {}", e))?;
                // Convert YAML to JSON
                serde_json::to_value(yaml_value)
                    .map_err(|e| format!("YAML to JSON conversion error: {}", e))?
            }
            DataType::Toml => {
                let toml_value: toml::Value =
                    toml::from_str(content).map_err(|e| format!("TOML parse error: {}", e))?;
                // Convert TOML to JSON
                serde_json::to_value(toml_value)
                    .map_err(|e| format!("TOML to JSON conversion error: {}", e))?
            }
            DataType::Csv => self.parse_csv(content)?,
            DataType::Cbor => self.parse_cbor(content)?,
            DataType::Xml => self.parse_xml(content)?,
        };

        let data_value = DataValue {
            data_type,
            value,
            source: source.clone(),
        };

        // Cache the result
        if let Some(ref source) = source {
            self.cache.insert(source.clone(), data_value.clone());
        }

        Ok(data_value)
    }

    /// 解析 CSV 数据
    fn parse_csv(&self, content: &str) -> Result<JsonValue, String> {
        let mut reader = csv::Reader::from_reader(content.as_bytes());
        let mut records = Vec::new();
        let headers = reader
            .headers()
            .map_err(|e| format!("CSV header error: {}", e))?
            .iter()
            .map(|h| h.to_string())
            .collect::<Vec<_>>();

        for result in reader.records() {
            let record = result.map_err(|e| format!("CSV record error: {}", e))?;
            let mut obj = serde_json::Map::new();

            for (i, header) in headers.iter().enumerate() {
                if let Some(value) = record.get(i) {
                    obj.insert(header.clone(), JsonValue::String(value.to_string()));
                }
            }

            records.push(JsonValue::Object(obj));
        }

        Ok(JsonValue::Array(records))
    }

    /// 解析 CBOR 数据
    fn parse_cbor(&self, content: &str) -> Result<JsonValue, String> {
        // CBOR is binary, so we need to decode from bytes
        // For string input, we'll assume base64 encoding
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        let bytes = STANDARD
            .decode(content)
            .map_err(|e| format!("CBOR base64 decode error: {}", e))?;

        let value: serde_cbor::Value =
            serde_cbor::from_slice(&bytes).map_err(|e| format!("CBOR decode error: {}", e))?;

        // Convert CBOR to JSON
        self.cbor_to_json(&value)
    }

    fn cbor_to_json(&self, value: &serde_cbor::Value) -> Result<JsonValue, String> {
        match value {
            serde_cbor::Value::Null => Ok(JsonValue::Null),
            serde_cbor::Value::Bool(b) => Ok(JsonValue::Bool(*b)),
            serde_cbor::Value::Integer(i) => {
                // Handle i128 to i64 conversion with overflow check
                if *i >= i64::MIN as i128 && *i <= i64::MAX as i128 {
                    Ok(JsonValue::Number((*i as i64).into()))
                } else {
                    // For values outside i64 range, use string representation
                    Ok(JsonValue::String(i.to_string()))
                }
            }
            serde_cbor::Value::Float(f) => Ok(JsonValue::Number(
                serde_json::Number::from_f64(*f).unwrap_or(0.into()),
            )),
            serde_cbor::Value::Text(s) => Ok(JsonValue::String(s.clone())),
            serde_cbor::Value::Array(arr) => {
                let mut json_arr = Vec::new();
                for item in arr {
                    json_arr.push(self.cbor_to_json(item)?);
                }
                Ok(JsonValue::Array(json_arr))
            }
            serde_cbor::Value::Map(map) => {
                let mut json_obj = serde_json::Map::new();
                for (key, val) in map {
                    if let serde_cbor::Value::Text(key_str) = key {
                        json_obj.insert(key_str.clone(), self.cbor_to_json(val)?);
                    }
                }
                Ok(JsonValue::Object(json_obj))
            }
            _ => Err("Unsupported CBOR value type".to_string()),
        }
    }

    /// 解析 XML 数据
    fn parse_xml(&self, content: &str) -> Result<JsonValue, String> {
        use xml::reader::{EventReader, XmlEvent};

        let reader = EventReader::from_str(content);
        let mut stack = Vec::new();
        let mut root = serde_json::Map::new();

        for e in reader {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    let mut obj = serde_json::Map::new();
                    obj.insert("_name".to_string(), JsonValue::String(name.to_string()));
                    obj.insert("_children".to_string(), JsonValue::Array(Vec::new()));
                    stack.push((name.to_string(), obj));
                }
                Ok(XmlEvent::Characters(text)) => {
                    if let Some((_, ref mut obj)) = stack.last_mut() {
                        if let Some(JsonValue::Array(ref mut children)) = obj.get_mut("_children") {
                            children.push(JsonValue::String(text));
                        }
                    }
                }
                Ok(XmlEvent::EndElement { .. }) => {
                    if let Some((name, obj)) = stack.pop() {
                        if stack.is_empty() {
                            root.insert(name, JsonValue::Object(obj));
                        } else {
                            if let Some((_, ref mut parent)) = stack.last_mut() {
                                if let Some(JsonValue::Array(ref mut children)) =
                                    parent.get_mut("_children")
                                {
                                    children.push(JsonValue::Object(obj));
                                }
                            }
                        }
                    }
                }
                Err(e) => return Err(format!("XML parse error: {}", e)),
                _ => {}
            }
        }

        Ok(JsonValue::Object(root))
    }

    /// 获取缓存的数据
    pub fn get_cached(&self, source: &str) -> Option<&DataValue> {
        self.cache.get(source)
    }

    /// 清除缓存
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// 将数据转换为 Typst 格式
    pub fn to_typst(&self, data: &DataValue) -> String {
        match &data.value {
            JsonValue::Object(obj) => {
                let mut output = String::new();
                output.push_str("( ");

                for (key, value) in obj {
                    output.push_str(&format!("{}: {}, ", key, self.value_to_typst(value)));
                }

                output.push(')');
                output
            }
            JsonValue::Array(arr) => {
                let mut output = String::new();
                output.push_str("( ");

                for value in arr {
                    output.push_str(&self.value_to_typst(value));
                    output.push_str(", ");
                }

                output.push(')');
                output
            }
            _ => self.value_to_typst(&data.value),
        }
    }

    fn value_to_typst(&self, value: &JsonValue) -> String {
        match value {
            JsonValue::Null => "none".to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => format!("\"{}\"", s),
            JsonValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| self.value_to_typst(v)).collect();
                format!("({})", items.join(", "))
            }
            JsonValue::Object(obj) => {
                let pairs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, self.value_to_typst(v)))
                    .collect();
                format!("({})", pairs.join(", "))
            }
        }
    }

    /// 查询数据路径
    pub fn query_path(&self, data: &DataValue, path: &str) -> Result<JsonValue, String> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = &data.value;

        for part in parts {
            current = match current {
                JsonValue::Object(obj) => obj
                    .get(part)
                    .ok_or_else(|| format!("Key '{}' not found", part))?,
                JsonValue::Array(arr) => {
                    let index: usize = part
                        .parse()
                        .map_err(|_| format!("Invalid array index: {}", part))?;
                    arr.get(index)
                        .ok_or_else(|| format!("Array index {} out of bounds", index))?
                }
                _ => return Err(format!("Cannot access path '{}' on non-object/array", part)),
            };
        }

        Ok(current.clone())
    }
}

impl Default for DataLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_type_from_extension() {
        assert_eq!(DataType::from_extension("json"), Some(DataType::Json));
        assert_eq!(DataType::from_extension("yaml"), Some(DataType::Yaml));
        assert_eq!(DataType::from_extension("yml"), Some(DataType::Yaml));
        assert_eq!(DataType::from_extension("csv"), Some(DataType::Csv));
        assert_eq!(DataType::from_extension("toml"), Some(DataType::Toml));
        assert_eq!(DataType::from_extension("cbor"), Some(DataType::Cbor));
        assert_eq!(DataType::from_extension("xml"), Some(DataType::Xml));
        assert_eq!(DataType::from_extension("txt"), None);
    }

    #[test]
    fn test_load_json() {
        let mut loader = DataLoader::new();
        let json = r#"{"name": "test", "value": 42}"#;
        let result = loader.load_from_string(json, DataType::Json, None);
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.data_type, DataType::Json);
        assert_eq!(data.value["name"], "test");
        assert_eq!(data.value["value"], 42);
    }

    #[test]
    fn test_load_yaml() {
        let mut loader = DataLoader::new();
        let yaml = r#"name: test
value: 42"#;
        let result = loader.load_from_string(yaml, DataType::Yaml, None);
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.data_type, DataType::Yaml);
        assert_eq!(data.value["name"], "test");
        assert_eq!(data.value["value"], 42);
    }

    #[test]
    fn test_load_toml() {
        let mut loader = DataLoader::new();
        let toml = r#"[section]
name = "test"
value = 42"#;
        let result = loader.load_from_string(toml, DataType::Toml, None);
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.data_type, DataType::Toml);
    }

    #[test]
    fn test_load_csv() {
        let mut loader = DataLoader::new();
        let csv = "name,value\nAlice,30\nBob,25";
        let result = loader.load_from_string(csv, DataType::Csv, None);
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.data_type, DataType::Csv);
        assert!(data.value.is_array());
    }

    #[test]
    fn test_query_path() {
        let mut loader = DataLoader::new();
        let json = r#"{"user": {"name": "Alice", "age": 30}}"#;
        let data = loader.load_from_string(json, DataType::Json, None).unwrap();

        let result = loader.query_path(&data, "user.name");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Alice");
    }

    #[test]
    fn test_to_typst() {
        let mut loader = DataLoader::new();
        let json = r#"{"name": "test", "value": 42}"#;
        let data = loader.load_from_string(json, DataType::Json, None).unwrap();

        let typst = loader.to_typst(&data);
        assert!(typst.contains("name:"));
        assert!(typst.contains("value:"));
    }

    #[test]
    fn test_cache() {
        let mut loader = DataLoader::new();
        let json = r#"{"test": true}"#;

        loader
            .load_from_string(json, DataType::Json, Some("test.json".to_string()))
            .unwrap();
        let cached = loader.get_cached("test.json");

        assert!(cached.is_some());
        assert_eq!(cached.unwrap().value["test"], true);
    }

    #[test]
    fn test_clear_cache() {
        let mut loader = DataLoader::new();
        let json = r#"{"test": true}"#;

        loader
            .load_from_string(json, DataType::Json, Some("test.json".to_string()))
            .unwrap();
        loader.clear_cache();

        let cached = loader.get_cached("test.json");
        assert!(cached.is_none());
    }
}
