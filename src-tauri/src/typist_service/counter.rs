/*!
 * 航空航天级 Counter 系统
 * 实现 Typst 的自动编号和计数功能
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CounterKey {
    pub name: String,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterValue {
    pub value: i64,
    pub display: String,
    pub numbering_style: NumberingStyle,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NumberingStyle {
    Arabic,      // 1, 2, 3
    RomanLower,  // i, ii, iii
    RomanUpper,  // I, II, III
    LetterLower, // a, b, c
    LetterUpper, // A, B, C
    Symbol,      // *, †, ‡
}

impl NumberingStyle {
    pub fn format(&self, value: i64) -> String {
        match self {
            NumberingStyle::Arabic => value.to_string(),
            NumberingStyle::RomanLower => Self::to_roman(value).to_lowercase(),
            NumberingStyle::RomanUpper => Self::to_roman(value),
            NumberingStyle::LetterLower => Self::to_letter(value).to_lowercase(),
            NumberingStyle::LetterUpper => Self::to_letter(value),
            NumberingStyle::Symbol => Self::to_symbol(value),
        }
    }

    fn to_roman(mut num: i64) -> String {
        if num <= 0 {
            return "0".to_string();
        }

        let values = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut result = String::new();
        for (val, sym) in values.iter() {
            while num >= *val {
                result.push_str(sym);
                num -= *val;
            }
        }
        result
    }

    fn to_letter(value: i64) -> String {
        if value <= 0 {
            return "?".to_string();
        }

        let mut num = value - 1;
        let mut result = String::new();

        while num >= 0 {
            let remainder = (num % 26) as u8;
            result.insert(0, (b'A' + remainder) as char);
            num = num / 26 - 1;
        }

        result
    }

    fn to_symbol(value: i64) -> String {
        let symbols = ["*", "†", "‡", "§", "¶", "‖"];
        let index = ((value - 1) as usize) % symbols.len();
        let repeats = ((value - 1) as usize) / symbols.len() + 1;
        symbols[index].repeat(repeats)
    }
}

#[derive(Debug, Clone)]
pub struct CounterUpdate {
    pub key: CounterKey,
    pub delta: i64,
    pub new_value: i64,
}

pub struct CounterSystem {
    counters: Arc<Mutex<HashMap<CounterKey, CounterValue>>>,
    updates: Arc<Mutex<Vec<CounterUpdate>>>,
}

impl CounterSystem {
    pub fn new() -> Self {
        Self {
            counters: Arc::new(Mutex::new(HashMap::new())),
            updates: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 获取或创建计数器
    pub fn get_or_create(&self, key: CounterKey, style: NumberingStyle) -> CounterValue {
        let mut counters = self.counters.lock().unwrap();

        if !counters.contains_key(&key) {
            let value = CounterValue {
                value: 0,
                display: style.format(0),
                numbering_style: style,
            };
            counters.insert(key.clone(), value);
        }

        counters.get(&key).cloned().unwrap()
    }

    /// 增加计数器
    pub fn increment(&self, key: CounterKey, delta: i64) -> CounterValue {
        let mut counters = self.counters.lock().unwrap();
        let mut updates = self.updates.lock().unwrap();

        let entry = counters.entry(key.clone()).or_insert_with(|| CounterValue {
            value: 0,
            display: "0".to_string(),
            numbering_style: NumberingStyle::Arabic,
        });

        entry.value += delta;
        entry.display = entry.numbering_style.format(entry.value);

        let new_value = entry.value;
        updates.push(CounterUpdate {
            key: key.clone(),
            delta,
            new_value,
        });

        entry.clone()
    }

    /// 设置计数器值
    pub fn set(&self, key: CounterKey, value: i64) -> CounterValue {
        let mut counters = self.counters.lock().unwrap();
        let mut updates = self.updates.lock().unwrap();

        let entry = counters.entry(key.clone()).or_insert_with(|| CounterValue {
            value: 0,
            display: "0".to_string(),
            numbering_style: NumberingStyle::Arabic,
        });

        let old_value = entry.value;
        entry.value = value;
        entry.display = entry.numbering_style.format(entry.value);

        updates.push(CounterUpdate {
            key: key.clone(),
            delta: value - old_value,
            new_value: value,
        });

        entry.clone()
    }

    /// 获取计数器当前值
    pub fn get(&self, key: &CounterKey) -> Option<CounterValue> {
        let counters = self.counters.lock().unwrap();
        counters.get(key).cloned()
    }

    /// 重置计数器
    pub fn reset(&self, key: &CounterKey) -> Option<CounterValue> {
        let mut counters = self.counters.lock().unwrap();
        let mut updates = self.updates.lock().unwrap();

        if let Some(entry) = counters.get_mut(key) {
            let old_value = entry.value;
            entry.value = 0;
            entry.display = entry.numbering_style.format(0);

            updates.push(CounterUpdate {
                key: key.clone(),
                delta: -old_value,
                new_value: 0,
            });

            return Some(entry.clone());
        }

        None
    }

    /// 获取所有更新记录
    pub fn get_updates(&self) -> Vec<CounterUpdate> {
        let updates = self.updates.lock().unwrap();
        updates.clone()
    }

    /// 清除更新记录
    pub fn clear_updates(&self) {
        let mut updates = self.updates.lock().unwrap();
        updates.clear();
    }

    /// 获取所有计数器
    pub fn get_all_counters(&self) -> HashMap<CounterKey, CounterValue> {
        let counters = self.counters.lock().unwrap();
        counters.clone()
    }

    /// 创建带作用域的计数器键
    pub fn create_key(name: String, scope: Option<String>) -> CounterKey {
        CounterKey { name, scope }
    }

    /// 解析 Typst 计数器语法
    pub fn parse_counter_syntax(typst_code: &str) -> Vec<(CounterKey, i64)> {
        let mut results = Vec::new();

        // 解析 #counter(name).update(delta) 语法
        for line in typst_code.lines() {
            if let Some(caps) = Self::extract_counter_update(line) {
                results.push(caps);
            }
        }

        results
    }

    fn extract_counter_update(line: &str) -> Option<(CounterKey, i64)> {
        let line = line.trim();

        // 匹配 #counter("name").update(delta)
        if line.starts_with("#counter(") {
            let after_counter = line.strip_prefix("#counter(")?;
            let name_end = after_counter.find(')')?;
            let name = after_counter[..name_end].trim_matches('"').to_string();

            let rest = &after_counter[name_end + 1..];
            if rest.starts_with(".update(") {
                let after_update = rest.strip_prefix(".update(")?;
                let delta_end = after_update.find(')')?;
                let delta_str = &after_update[..delta_end];
                let delta: i64 = delta_str.trim().parse().ok()?;

                return Some((CounterKey { name, scope: None }, delta));
            }
        }

        None
    }
}

impl Default for CounterSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_creation() {
        let system = CounterSystem::new();
        let key = CounterKey {
            name: "test".to_string(),
            scope: None,
        };
        let value = system.get_or_create(key, NumberingStyle::Arabic);
        assert_eq!(value.value, 0);
    }

    #[test]
    fn test_counter_increment() {
        let system = CounterSystem::new();
        let key = CounterKey {
            name: "test".to_string(),
            scope: None,
        };

        system.increment(key.clone(), 1);
        let value = system.get(&key).unwrap();
        assert_eq!(value.value, 1);

        system.increment(key.clone(), 2);
        let value = system.get(&key).unwrap();
        assert_eq!(value.value, 3);
    }

    #[test]
    fn test_counter_set() {
        let system = CounterSystem::new();
        let key = CounterKey {
            name: "test".to_string(),
            scope: None,
        };

        system.set(key.clone(), 10);
        let value = system.get(&key).unwrap();
        assert_eq!(value.value, 10);
    }

    #[test]
    fn test_counter_reset() {
        let system = CounterSystem::new();
        let key = CounterKey {
            name: "test".to_string(),
            scope: None,
        };

        system.increment(key.clone(), 5);
        system.reset(&key);
        let value = system.get(&key).unwrap();
        assert_eq!(value.value, 0);
    }

    #[test]
    fn test_numbering_style_arabic() {
        assert_eq!(NumberingStyle::Arabic.format(1), "1");
        assert_eq!(NumberingStyle::Arabic.format(10), "10");
    }

    #[test]
    fn test_numbering_style_roman_lower() {
        assert_eq!(NumberingStyle::RomanLower.format(1), "i");
        assert_eq!(NumberingStyle::RomanLower.format(3), "iii");
        assert_eq!(NumberingStyle::RomanLower.format(4), "iv");
    }

    #[test]
    fn test_numbering_style_roman_upper() {
        assert_eq!(NumberingStyle::RomanUpper.format(1), "I");
        assert_eq!(NumberingStyle::RomanUpper.format(3), "III");
        assert_eq!(NumberingStyle::RomanUpper.format(4), "IV");
    }

    #[test]
    fn test_numbering_style_letter_lower() {
        assert_eq!(NumberingStyle::LetterLower.format(1), "a");
        assert_eq!(NumberingStyle::LetterLower.format(26), "z");
        assert_eq!(NumberingStyle::LetterLower.format(27), "aa");
    }

    #[test]
    fn test_numbering_style_letter_upper() {
        assert_eq!(NumberingStyle::LetterUpper.format(1), "A");
        assert_eq!(NumberingStyle::LetterUpper.format(26), "Z");
        assert_eq!(NumberingStyle::LetterUpper.format(27), "AA");
    }

    #[test]
    fn test_numbering_style_symbol() {
        assert_eq!(NumberingStyle::Symbol.format(1), "*");
        assert_eq!(NumberingStyle::Symbol.format(2), "†");
        assert_eq!(NumberingStyle::Symbol.format(3), "‡");
        assert_eq!(NumberingStyle::Symbol.format(4), "§");
        assert_eq!(NumberingStyle::Symbol.format(7), "**");
    }

    #[test]
    fn test_counter_updates() {
        let system = CounterSystem::new();
        let key = CounterKey {
            name: "test".to_string(),
            scope: None,
        };

        system.increment(key.clone(), 1);
        system.increment(key.clone(), 2);

        let updates = system.get_updates();
        assert_eq!(updates.len(), 2);
        assert_eq!(updates[0].delta, 1);
        assert_eq!(updates[1].delta, 2);
    }

    #[test]
    fn test_counter_scoped() {
        let system = CounterSystem::new();
        let key1 = CounterKey {
            name: "section".to_string(),
            scope: Some("chapter1".to_string()),
        };
        let key2 = CounterKey {
            name: "section".to_string(),
            scope: Some("chapter2".to_string()),
        };

        system.increment(key1.clone(), 1);
        system.increment(key2.clone(), 1);

        let value1 = system.get(&key1).unwrap();
        let value2 = system.get(&key2).unwrap();

        assert_eq!(value1.value, 1);
        assert_eq!(value2.value, 1);
    }

    #[test]
    fn test_parse_counter_syntax() {
        let code = r#"
#counter("figure").update(1)
#counter("table").update(1)
"#;

        let results = CounterSystem::parse_counter_syntax(code);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0.name, "figure");
        assert_eq!(results[0].1, 1);
        assert_eq!(results[1].0.name, "table");
        assert_eq!(results[1].1, 1);
    }

    #[test]
    fn test_create_key() {
        let key = CounterSystem::create_key("test".to_string(), Some("scope".to_string()));
        assert_eq!(key.name, "test");
        assert_eq!(key.scope, Some("scope".to_string()));
    }
}
