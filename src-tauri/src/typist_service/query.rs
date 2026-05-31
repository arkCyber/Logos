/*!
 * 航空航天级查询系统
 * 实现 Typst 的内容查询功能
 */

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySelector {
    pub element_type: Option<String>,
    pub attribute_filter: Option<AttributeFilter>,
    pub position_filter: Option<PositionFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeFilter {
    pub attribute: String,
    pub operator: FilterOperator,
    pub value: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    Contains,
    StartsWith,
    EndsWith,
    Matches,
    GreaterThan,
    LessThan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionFilter {
    pub position: QueryPosition,
    pub index: Option<usize>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum QueryPosition {
    First,
    Last,
    Nth,
    Even,
    Odd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub matches: Vec<QueryMatch>,
    pub total_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryMatch {
    pub element_id: String,
    pub element_type: String,
    pub content: String,
    pub position: usize,
    pub attributes: HashMap<String, String>,
}

pub struct QueryEngine {
    cache: HashMap<String, QueryResult>,
}

impl QueryEngine {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// 执行查询
    pub fn query(
        &mut self,
        document: &str,
        selector: QuerySelector,
    ) -> Result<QueryResult, String> {
        let matches = self.find_matches(document, &selector)?;
        let total_count = matches.len();

        let result = QueryResult {
            matches,
            total_count,
        };

        Ok(result)
    }

    fn find_matches(
        &self,
        document: &str,
        selector: &QuerySelector,
    ) -> Result<Vec<QueryMatch>, String> {
        let mut matches = Vec::new();

        // Parse document into elements (simplified - real implementation would use Typst's AST)
        let elements = self.parse_document(document)?;

        for (idx, element) in elements.iter().enumerate() {
            if self.matches_selector(element, selector, idx) {
                matches.push(QueryMatch {
                    element_id: format!("element-{}", idx),
                    element_type: element.element_type.clone(),
                    content: element.content.clone(),
                    position: idx,
                    attributes: element.attributes.clone(),
                });
            }
        }

        Ok(matches)
    }

    fn parse_document(&self, document: &str) -> Result<Vec<DocumentElement>, String> {
        let mut elements = Vec::new();

        // Simple parsing for headings, paragraphs, etc.
        for line in document.lines() {
            let line = line.trim();

            if let Some(content) = line.strip_prefix("= ") {
                elements.push(DocumentElement {
                    element_type: "heading".to_string(),
                    content: content.to_string(),
                    attributes: {
                        let mut attrs = HashMap::new();
                        attrs.insert("level".to_string(), "1".to_string());
                        attrs
                    },
                });
            } else if let Some(content) = line.strip_prefix("== ") {
                elements.push(DocumentElement {
                    element_type: "heading".to_string(),
                    content: content.to_string(),
                    attributes: {
                        let mut attrs = HashMap::new();
                        attrs.insert("level".to_string(), "2".to_string());
                        attrs
                    },
                });
            } else if let Some(content) = line.strip_prefix("=== ") {
                elements.push(DocumentElement {
                    element_type: "heading".to_string(),
                    content: content.to_string(),
                    attributes: {
                        let mut attrs = HashMap::new();
                        attrs.insert("level".to_string(), "3".to_string());
                        attrs
                    },
                });
            } else if !line.is_empty() {
                elements.push(DocumentElement {
                    element_type: "paragraph".to_string(),
                    content: line.to_string(),
                    attributes: HashMap::new(),
                });
            }
        }

        Ok(elements)
    }

    fn matches_selector(
        &self,
        element: &DocumentElement,
        selector: &QuerySelector,
        position: usize,
    ) -> bool {
        // Check element type
        if let Some(ref element_type) = selector.element_type {
            if element.element_type != *element_type {
                return false;
            }
        }

        // Check attribute filter
        if let Some(ref attr_filter) = selector.attribute_filter {
            if !self.matches_attribute_filter(element, attr_filter) {
                return false;
            }
        }

        // Check position filter
        if let Some(ref pos_filter) = selector.position_filter {
            if !self.matches_position_filter(position, pos_filter) {
                return false;
            }
        }

        true
    }

    fn matches_attribute_filter(
        &self,
        element: &DocumentElement,
        filter: &AttributeFilter,
    ) -> bool {
        let attr_value = element.attributes.get(&filter.attribute);

        match attr_value {
            Some(value) => match filter.operator {
                FilterOperator::Equals => value == &filter.value,
                FilterOperator::NotEquals => value != &filter.value,
                FilterOperator::Contains => value.contains(&filter.value),
                FilterOperator::StartsWith => value.starts_with(&filter.value),
                FilterOperator::EndsWith => value.ends_with(&filter.value),
                FilterOperator::Matches => Regex::new(&filter.value)
                    .map(|re| re.is_match(value))
                    .unwrap_or(false),
                FilterOperator::GreaterThan => value
                    .parse::<i64>()
                    .map(|v| v > filter.value.parse::<i64>().unwrap_or(0))
                    .unwrap_or(false),
                FilterOperator::LessThan => value
                    .parse::<i64>()
                    .map(|v| v < filter.value.parse::<i64>().unwrap_or(0))
                    .unwrap_or(false),
            },
            None => false,
        }
    }

    fn matches_position_filter(&self, position: usize, filter: &PositionFilter) -> bool {
        match filter.position {
            QueryPosition::First => position == 0,
            QueryPosition::Last => false, // Need total count to determine
            QueryPosition::Nth => {
                if let Some(index) = filter.index {
                    position == index
                } else {
                    false
                }
            }
            QueryPosition::Even => position.is_multiple_of(2),
            QueryPosition::Odd => position % 2 == 1,
        }
    }

    /// 解析 Typst 查询语法
    pub fn parse_query_syntax(typst_code: &str) -> Result<QuerySelector, String> {
        // Parse #query(selector) syntax
        let code = typst_code.trim();

        if code.starts_with("#query(") {
            let after_query = code.strip_prefix("#query(").ok_or("Invalid query syntax")?;
            let selector_str = after_query
                .strip_suffix(")")
                .ok_or("Invalid query syntax")?;

            Self::parse_selector_string(selector_str)
        } else {
            Err("Not a query expression".to_string())
        }
    }

    fn parse_selector_string(selector_str: &str) -> Result<QuerySelector, String> {
        let mut selector = QuerySelector {
            element_type: None,
            attribute_filter: None,
            position_filter: None,
        };

        // Simple parsing: "heading[level=1]" or "paragraph:nth(0)"
        if selector_str.contains('[') {
            let parts: Vec<&str> = selector_str.split('[').collect();
            if parts.len() >= 2 {
                selector.element_type = Some(parts[0].to_string());

                let filter_str = parts[1].trim_end_matches(']');
                if filter_str.contains('=') {
                    let filter_parts: Vec<&str> = filter_str.split('=').collect();
                    if filter_parts.len() >= 2 {
                        selector.attribute_filter = Some(AttributeFilter {
                            attribute: filter_parts[0].to_string(),
                            operator: FilterOperator::Equals,
                            value: filter_parts[1].to_string(),
                        });
                    }
                }
            }
        } else if selector_str.contains(':') {
            let parts: Vec<&str> = selector_str.split(':').collect();
            if parts.len() >= 2 {
                selector.element_type = Some(parts[0].to_string());

                let pos_str = parts[1];
                if pos_str.starts_with("nth(") {
                    let index_str = pos_str
                        .strip_prefix("nth(")
                        .and_then(|s| s.strip_suffix(')'))
                        .ok_or("Invalid nth syntax")?;
                    let index: usize = index_str.parse().map_err(|_| "Invalid index")?;
                    selector.position_filter = Some(PositionFilter {
                        position: QueryPosition::Nth,
                        index: Some(index),
                    });
                }
            }
        } else {
            selector.element_type = Some(selector_str.to_string());
        }

        Ok(selector)
    }

    /// 获取缓存结果
    pub fn get_cached(&self, query_key: &str) -> Option<&QueryResult> {
        self.cache.get(query_key)
    }

    /// 缓存结果
    pub fn cache_result(&mut self, query_key: String, result: QueryResult) {
        self.cache.insert(query_key, result);
    }

    /// 清除缓存
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

#[derive(Debug, Clone)]
struct DocumentElement {
    element_type: String,
    content: String,
    attributes: HashMap<String, String>,
}

impl Default for QueryEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_engine_creation() {
        let engine = QueryEngine::new();
        assert!(engine.cache.is_empty());
    }

    #[test]
    fn test_parse_document() {
        let engine = QueryEngine::new();
        let document = "= Heading 1\n\nParagraph text";
        let elements = engine.parse_document(document).unwrap();
        assert_eq!(elements.len(), 2);
        assert_eq!(elements[0].element_type, "heading");
        assert_eq!(elements[1].element_type, "paragraph");
    }

    #[test]
    fn test_query_by_element_type() {
        let mut engine = QueryEngine::new();
        let document = "= Heading 1\n\nParagraph text\n== Heading 2";
        let selector = QuerySelector {
            element_type: Some("heading".to_string()),
            attribute_filter: None,
            position_filter: None,
        };

        let result = engine.query(document, selector).unwrap();
        assert_eq!(result.total_count, 2);
    }

    #[test]
    fn test_query_with_attribute_filter() {
        let mut engine = QueryEngine::new();
        let document = "= Heading 1\n== Heading 2\n=== Heading 3";
        let selector = QuerySelector {
            element_type: Some("heading".to_string()),
            attribute_filter: Some(AttributeFilter {
                attribute: "level".to_string(),
                operator: FilterOperator::Equals,
                value: "2".to_string(),
            }),
            position_filter: None,
        };

        let result = engine.query(document, selector).unwrap();
        assert_eq!(result.total_count, 1);
    }

    #[test]
    fn test_query_with_position_filter() {
        let mut engine = QueryEngine::new();
        let document = "= Heading 1\n== Heading 2\n=== Heading 3";
        let selector = QuerySelector {
            element_type: Some("heading".to_string()),
            attribute_filter: None,
            position_filter: Some(PositionFilter {
                position: QueryPosition::First,
                index: None,
            }),
        };

        let result = engine.query(document, selector).unwrap();
        assert_eq!(result.total_count, 1);
        assert_eq!(result.matches[0].position, 0);
    }

    #[test]
    fn test_parse_query_syntax() {
        let code = "#query(heading[level=1])";
        let selector = QueryEngine::parse_query_syntax(code).unwrap();
        assert_eq!(selector.element_type, Some("heading".to_string()));
        assert!(selector.attribute_filter.is_some());
    }

    #[test]
    fn test_filter_operator_contains() {
        let engine = QueryEngine::new();
        let element = DocumentElement {
            element_type: "paragraph".to_string(),
            content: "Hello world".to_string(),
            attributes: {
                let mut attrs = HashMap::new();
                attrs.insert("text".to_string(), "Hello world".to_string());
                attrs
            },
        };

        let filter = AttributeFilter {
            attribute: "text".to_string(),
            operator: FilterOperator::Contains,
            value: "Hello".to_string(),
        };

        assert!(engine.matches_attribute_filter(&element, &filter));
    }

    #[test]
    fn test_cache_operations() {
        let mut engine = QueryEngine::new();
        let result = QueryResult {
            matches: Vec::new(),
            total_count: 0,
        };

        engine.cache_result("test".to_string(), result.clone());
        let cached = engine.get_cached("test");
        assert!(cached.is_some());

        engine.clear_cache();
        let cached = engine.get_cached("test");
        assert!(cached.is_none());
    }
}
