/*!
 * 航空航天级状态管理系统
 * 实现 Typst 的状态管理功能
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<StateValue>),
    Object(HashMap<String, StateValue>),
    Null,
}

impl StateValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            StateValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            StateValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            StateValue::Float(f) => Some(*f),
            StateValue::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            StateValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn type_name(&self) -> &str {
        match self {
            StateValue::String(_) => "string",
            StateValue::Integer(_) => "integer",
            StateValue::Float(_) => "float",
            StateValue::Boolean(_) => "boolean",
            StateValue::Array(_) => "array",
            StateValue::Object(_) => "object",
            StateValue::Null => "null",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StateKey {
    pub name: String,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateEntry {
    pub value: StateValue,
    pub readonly: bool,
    pub persistent: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct StateManager {
    states: Arc<Mutex<HashMap<StateKey, StateEntry>>>,
    history: Arc<Mutex<Vec<StateUpdate>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateUpdate {
    pub key: StateKey,
    pub old_value: Option<StateValue>,
    pub new_value: StateValue,
    pub timestamp: i64,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            states: Arc::new(Mutex::new(HashMap::new())),
            history: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 设置状态值
    pub fn set(
        &self,
        key: StateKey,
        value: StateValue,
        readonly: bool,
        persistent: bool,
    ) -> Result<(), String> {
        let mut states = self.states.lock().unwrap();
        let mut history = self.history.lock().unwrap();

        let old_value = states.get(&key).map(|entry| entry.value.clone());

        // Check if readonly
        if let Some(entry) = states.get(&key) {
            if entry.readonly {
                return Err(format!("State '{}' is readonly", key.name));
            }
        }

        let now = chrono::Utc::now().timestamp();
        let entry = StateEntry {
            value: value.clone(),
            readonly,
            persistent,
            created_at: states.get(&key).map(|e| e.created_at).unwrap_or(now),
            updated_at: now,
        };

        states.insert(key.clone(), entry);

        history.push(StateUpdate {
            key: key.clone(),
            old_value,
            new_value: value,
            timestamp: now,
        });

        Ok(())
    }

    /// 获取状态值
    pub fn get(&self, key: &StateKey) -> Option<StateValue> {
        let states = self.states.lock().unwrap();
        states.get(key).map(|entry| entry.value.clone())
    }

    /// 删除状态
    pub fn delete(&self, key: &StateKey) -> Result<(), String> {
        let mut states = self.states.lock().unwrap();
        let mut history = self.history.lock().unwrap();

        if let Some(entry) = states.remove(key) {
            let now = chrono::Utc::now().timestamp();
            history.push(StateUpdate {
                key: key.clone(),
                old_value: Some(entry.value),
                new_value: StateValue::Null,
                timestamp: now,
            });

            Ok(())
        } else {
            Err(format!("State '{}' not found", key.name))
        }
    }

    /// 检查状态是否存在
    pub fn exists(&self, key: &StateKey) -> bool {
        let states = self.states.lock().unwrap();
        states.contains_key(key)
    }

    /// 获取所有状态键
    pub fn keys(&self) -> Vec<StateKey> {
        let states = self.states.lock().unwrap();
        states.keys().cloned().collect()
    }

    /// 获取历史记录
    pub fn get_history(&self) -> Vec<StateUpdate> {
        let history = self.history.lock().unwrap();
        history.clone()
    }

    /// 清除历史记录
    pub fn clear_history(&self) {
        let mut history = self.history.lock().unwrap();
        history.clear();
    }

    /// 撤销最后的更新
    pub fn undo(&self) -> Result<(), String> {
        let mut states = self.states.lock().unwrap();
        let mut history = self.history.lock().unwrap();

        if let Some(update) = history.pop() {
            if let Some(old_value) = update.old_value {
                let entry = StateEntry {
                    value: old_value.clone(),
                    readonly: false,
                    persistent: false,
                    created_at: update.timestamp,
                    updated_at: chrono::Utc::now().timestamp(),
                };
                states.insert(update.key, entry);
            } else {
                states.remove(&update.key);
            }

            Ok(())
        } else {
            Err("No history to undo".to_string())
        }
    }

    /// 创建带作用域的状态键
    pub fn create_key(name: String, scope: Option<String>) -> StateKey {
        StateKey { name, scope }
    }

    /// 解析 Typst 状态语法
    pub fn parse_state_syntax(typst_code: &str) -> Vec<(StateKey, StateValue)> {
        let mut results = Vec::new();

        // Parse #set-state("name", value) syntax
        for line in typst_code.lines() {
            if let Some((key, value)) = Self::extract_state_assignment(line) {
                results.push((key, value));
            }
        }

        results
    }

    fn extract_state_assignment(line: &str) -> Option<(StateKey, StateValue)> {
        let line = line.trim();

        // Match #set-state("name", value)
        if line.starts_with("#set-state(") {
            let after_func = line.strip_prefix("#set-state(")?;
            let name_end = after_func.find(',')?;
            let name = after_func[..name_end].trim_matches('"').to_string();

            let rest = &after_func[name_end + 1..];
            let value_str = rest.trim_end_matches(')').trim();

            let value = Self::parse_value(value_str)?;

            return Some((StateKey { name, scope: None }, value));
        }

        None
    }

    fn parse_value(value_str: &str) -> Option<StateValue> {
        let value_str = value_str.trim();

        // Try to parse as boolean
        if value_str == "true" {
            return Some(StateValue::Boolean(true));
        }
        if value_str == "false" {
            return Some(StateValue::Boolean(false));
        }

        // Try to parse as integer
        if let Ok(i) = value_str.parse::<i64>() {
            return Some(StateValue::Integer(i));
        }

        // Try to parse as float
        if let Ok(f) = value_str.parse::<f64>() {
            return Some(StateValue::Float(f));
        }

        // Try to parse as string
        if value_str.starts_with('"') && value_str.ends_with('"') {
            return Some(StateValue::String(
                value_str[1..value_str.len() - 1].to_string(),
            ));
        }

        // Default to string
        Some(StateValue::String(value_str.to_string()))
    }

    /// 导出持久化状态
    pub fn export_persistent(&self) -> HashMap<StateKey, StateValue> {
        let states = self.states.lock().unwrap();
        states
            .iter()
            .filter(|(_, entry)| entry.persistent)
            .map(|(key, entry)| (key.clone(), entry.value.clone()))
            .collect()
    }

    /// 导入持久化状态
    pub fn import_persistent(&self, data: HashMap<StateKey, StateValue>) -> Result<(), String> {
        for (key, value) in data {
            self.set(key, value, false, true)?;
        }
        Ok(())
    }

    /// 清除所有状态
    pub fn clear_all(&self) {
        let mut states = self.states.lock().unwrap();
        states.clear();
    }

    /// 清除特定作用域的状态
    pub fn clear_scope(&self, scope: &str) {
        let mut states = self.states.lock().unwrap();
        states.retain(|key, _| key.scope.as_deref() != Some(scope));
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_manager_creation() {
        let manager = StateManager::new();
        assert!(manager.keys().is_empty());
    }

    #[test]
    fn test_set_and_get() {
        let manager = StateManager::new();
        let key = StateKey {
            name: "test".to_string(),
            scope: None,
        };

        manager
            .set(
                key.clone(),
                StateValue::String("value".to_string()),
                false,
                false,
            )
            .unwrap();
        let value = manager.get(&key).unwrap();

        assert_eq!(value.as_string(), Some("value"));
    }

    #[test]
    fn test_set_readonly() {
        let manager = StateManager::new();
        let key = StateKey {
            name: "test".to_string(),
            scope: None,
        };

        manager
            .set(
                key.clone(),
                StateValue::String("value".to_string()),
                true,
                false,
            )
            .unwrap();
        let result = manager.set(
            key.clone(),
            StateValue::String("new".to_string()),
            false,
            false,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_delete() {
        let manager = StateManager::new();
        let key = StateKey {
            name: "test".to_string(),
            scope: None,
        };

        manager
            .set(
                key.clone(),
                StateValue::String("value".to_string()),
                false,
                false,
            )
            .unwrap();
        manager.delete(&key).unwrap();

        assert!(!manager.exists(&key));
    }

    #[test]
    fn test_exists() {
        let manager = StateManager::new();
        let key = StateKey {
            name: "test".to_string(),
            scope: None,
        };

        assert!(!manager.exists(&key));

        manager
            .set(
                key.clone(),
                StateValue::String("value".to_string()),
                false,
                false,
            )
            .unwrap();
        assert!(manager.exists(&key));
    }

    #[test]
    fn test_history() {
        let manager = StateManager::new();
        let key = StateKey {
            name: "test".to_string(),
            scope: None,
        };

        manager
            .set(
                key.clone(),
                StateValue::String("value1".to_string()),
                false,
                false,
            )
            .unwrap();
        manager
            .set(
                key.clone(),
                StateValue::String("value2".to_string()),
                false,
                false,
            )
            .unwrap();

        let history = manager.get_history();
        assert_eq!(history.len(), 2);
    }

    #[test]
    fn test_undo() {
        let manager = StateManager::new();
        let key = StateKey {
            name: "test".to_string(),
            scope: None,
        };

        manager
            .set(
                key.clone(),
                StateValue::String("value1".to_string()),
                false,
                false,
            )
            .unwrap();
        manager
            .set(
                key.clone(),
                StateValue::String("value2".to_string()),
                false,
                false,
            )
            .unwrap();

        manager.undo().unwrap();

        let value = manager.get(&key).unwrap();
        assert_eq!(value.as_string(), Some("value1"));
    }

    #[test]
    fn test_scoped_states() {
        let manager = StateManager::new();
        let key1 = StateKey {
            name: "counter".to_string(),
            scope: Some("scope1".to_string()),
        };
        let key2 = StateKey {
            name: "counter".to_string(),
            scope: Some("scope2".to_string()),
        };

        manager
            .set(key1.clone(), StateValue::Integer(1), false, false)
            .unwrap();
        manager
            .set(key2.clone(), StateValue::Integer(2), false, false)
            .unwrap();

        let value1 = manager.get(&key1).unwrap();
        let value2 = manager.get(&key2).unwrap();

        assert_eq!(value1.as_integer(), Some(1));
        assert_eq!(value2.as_integer(), Some(2));
    }

    #[test]
    fn test_clear_scope() {
        let manager = StateManager::new();
        let key1 = StateKey {
            name: "test".to_string(),
            scope: Some("scope1".to_string()),
        };
        let key2 = StateKey {
            name: "test".to_string(),
            scope: Some("scope2".to_string()),
        };

        manager
            .set(
                key1.clone(),
                StateValue::String("value1".to_string()),
                false,
                false,
            )
            .unwrap();
        manager
            .set(
                key2.clone(),
                StateValue::String("value2".to_string()),
                false,
                false,
            )
            .unwrap();

        manager.clear_scope("scope1");

        assert!(!manager.exists(&key1));
        assert!(manager.exists(&key2));
    }

    #[test]
    fn test_parse_state_syntax() {
        let code = r#"
#set-state("counter", 42)
#set-state("name", "Alice")
"#;

        let results = StateManager::parse_state_syntax(code);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0.name, "counter");
        assert_eq!(results[1].0.name, "name");
    }

    #[test]
    fn test_persistent_export_import() {
        let manager = StateManager::new();
        let key = StateKey {
            name: "test".to_string(),
            scope: None,
        };

        manager
            .set(
                key.clone(),
                StateValue::String("value".to_string()),
                false,
                true,
            )
            .unwrap();

        let exported = manager.export_persistent();
        assert_eq!(exported.len(), 1);

        let manager2 = StateManager::new();
        manager2.import_persistent(exported).unwrap();

        let value = manager2.get(&key).unwrap();
        assert_eq!(value.as_string(), Some("value"));
    }

    #[test]
    fn test_state_value_type_name() {
        assert_eq!(StateValue::String("test".to_string()).type_name(), "string");
        assert_eq!(StateValue::Integer(42).type_name(), "integer");
        assert_eq!(StateValue::Float(3.14).type_name(), "float");
        assert_eq!(StateValue::Boolean(true).type_name(), "boolean");
        assert_eq!(StateValue::Array(vec![]).type_name(), "array");
        assert_eq!(StateValue::Object(HashMap::new()).type_name(), "object");
        assert_eq!(StateValue::Null.type_name(), "null");
    }
}
