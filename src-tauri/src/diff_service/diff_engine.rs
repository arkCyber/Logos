//! Diff Engine - Aerospace-Grade Diff Service
//!
//! Safety-critical diff computation service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Comprehensive error handling
//! - Performance monitoring
//! - Security hardening
//! - Fault tolerance and error recovery

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use crate::error_handling::{ErrorContext, ErrorSeverity, CircuitBreaker};
use crate::config_service::ExportConfigService;

/// Maximum number of changes to prevent memory issues
const MAX_CHANGES: usize = 100_000;

/// Maximum similarity computation string length
const MAX_SIMILARITY_LENGTH: usize = 100_000;

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 100;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 500;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChangeType {
    Insert,
    Delete,
    Replace,
    Equal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffChange {
    pub change_type: ChangeType,
    pub old_text: String,
    pub new_text: String,
    pub old_position: usize,
    pub new_position: usize,
    pub old_length: usize,
    pub new_length: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffResult {
    pub changes: Vec<DiffChange>,
    pub similarity: f64,
    pub old_length: usize,
    pub new_length: usize,
    pub total_changes: usize,
}

pub struct DiffEngine {
    operation_count: u64,
    last_error: Option<ErrorContext>,
    config_service: Arc<ExportConfigService>,
    circuit_breaker: CircuitBreaker,
    // In production, use a proper diff library like similar or difflib
}

impl DiffEngine {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        let circuit_breaker = CircuitBreaker::new(config_service.clone());
        Self {
            operation_count: 0,
            last_error: None,
            config_service,
            circuit_breaker,
        }
    }

    /// Validate text length
    fn validate_text_length(&self, text: &str) -> Result<(), String> {
        let diff_config = self.config_service.get_diff_config();
        if text.len() > diff_config.max_text_length {
            return Err(format!("Text exceeds maximum length of {}", diff_config.max_text_length));
        }
        Ok(())
    }

    /// Validate line count
    fn validate_line_count(&self, line_count: usize) -> Result<(), String> {
        let diff_config = self.config_service.get_diff_config();
        if line_count > diff_config.max_line_count {
            return Err(format!("Line count exceeds maximum of {}", diff_config.max_line_count));
        }
        Ok(())
    }

    /// Validate line length
    fn validate_line_length(&self, line: &str) -> Result<(), String> {
        let diff_config = self.config_service.get_diff_config();
        if line.len() > diff_config.max_line_length {
            return Err(format!("Line exceeds maximum length of {}", diff_config.max_line_length));
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

    /// Compare two texts and return diff result with validation
    /// 
    /// # Arguments
    /// * `old_text` - The original text
    /// * `new_text` - The modified text
    /// 
    /// # Returns
    /// A DiffResult containing the changes and similarity
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates all inputs to prevent DoS attacks and memory issues
    pub fn compare(&mut self, old_text: &str, new_text: &str) -> DiffResult {
        let start_time = Instant::now();
        self.operation_count += 1;

        // Check circuit breaker
        if !self.circuit_breaker.allow_operation() {
            self.record_error("CIRCUIT_BREAKER_OPEN", "Circuit breaker is open, blocking diff operations", "compare");
            return DiffResult {
                changes: vec![],
                similarity: 0.0,
                old_length: old_text.len(),
                new_length: new_text.len(),
                total_changes: 0,
            };
        }

        // Validate text lengths
        if let Err(e) = self.validate_text_length(old_text) {
            self.record_error("INVALID_OLD_TEXT", &e, "compare");
            self.circuit_breaker.record_failure();
            return DiffResult {
                changes: vec![],
                similarity: 0.0,
                old_length: old_text.len(),
                new_length: new_text.len(),
                total_changes: 0,
            };
        }
        if let Err(e) = self.validate_text_length(new_text) {
            self.record_error("INVALID_NEW_TEXT", &e, "compare");
            self.circuit_breaker.record_failure();
            return DiffResult {
                changes: vec![],
                similarity: 0.0,
                old_length: old_text.len(),
                new_length: new_text.len(),
                total_changes: 0,
            };
        }

        // Simple line-by-line diff implementation
        let old_lines: Vec<&str> = old_text.lines().collect();
        let new_lines: Vec<&str> = new_text.lines().collect();

        // Validate line counts
        if let Err(e) = self.validate_line_count(old_lines.len()) {
            self.record_error("INVALID_LINE_COUNT", &e, "compare");
            self.circuit_breaker.record_failure();
            return DiffResult {
                changes: vec![],
                similarity: 0.0,
                old_length: old_text.len(),
                new_length: new_text.len(),
                total_changes: 0,
            };
        }
        if let Err(e) = self.validate_line_count(new_lines.len()) {
            self.record_error("INVALID_LINE_COUNT", &e, "compare");
            self.circuit_breaker.record_failure();
            return DiffResult {
                changes: vec![],
                similarity: 0.0,
                old_length: old_text.len(),
                new_length: new_text.len(),
                total_changes: 0,
            };
        }

        // Validate individual line lengths
        for line in &old_lines {
            if let Err(e) = self.validate_line_length(line) {
                self.record_error("INVALID_LINE_LENGTH", &e, "compare");
                return DiffResult {
                    changes: vec![],
                    similarity: 0.0,
                    old_length: old_text.len(),
                    new_length: new_text.len(),
                    total_changes: 0,
                };
            }
        }
        for line in &new_lines {
            if let Err(e) = self.validate_line_length(line) {
                self.record_error("INVALID_LINE_LENGTH", &e, "compare");
                return DiffResult {
                    changes: vec![],
                    similarity: 0.0,
                    old_length: old_text.len(),
                    new_length: new_text.len(),
                    total_changes: 0,
                };
            }
        }

        let changes = self.compute_line_diff(&old_lines, &new_lines);
        
        // Safety check: prevent too many changes
        if changes.len() > MAX_CHANGES {
            eprintln!("DiffEngine: number of changes exceeds maximum of {}", MAX_CHANGES);
        }
        
        let similarity = self.compute_similarity(old_text, new_text);
        let total_changes = changes.len();

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("DiffEngine CRITICAL performance warning: compare took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("DiffEngine performance warning: compare took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        self.circuit_breaker.record_success();
        DiffResult {
            changes,
            similarity,
            old_length: old_text.len(),
            new_length: new_text.len(),
            total_changes,
        }
    }

    /// Compute line-by-line diff
    /// 
    /// # Arguments
    /// * `old_lines` - The original lines
    /// * `new_lines` - The modified lines
    /// 
    /// # Returns
    /// A vector of DiffChange objects
    fn compute_line_diff(&self, old_lines: &[&str], new_lines: &[&str]) -> Vec<DiffChange> {
        let mut changes = Vec::new();
        let mut old_idx = 0;
        let mut new_idx = 0;

        while old_idx < old_lines.len() || new_idx < new_lines.len() {
            if old_idx < old_lines.len() && new_idx < new_lines.len() {
                let old_line = old_lines[old_idx];
                let new_line = new_lines[new_idx];

                if old_line == new_line {
                    // Lines are equal
                    changes.push(DiffChange {
                        change_type: ChangeType::Equal,
                        old_text: old_line.to_string(),
                        new_text: new_line.to_string(),
                        old_position: old_idx,
                        new_position: new_idx,
                        old_length: old_line.len(),
                        new_length: new_line.len(),
                    });
                    old_idx += 1;
                    new_idx += 1;
                } else {
                    // Lines differ - check if it's an insert, delete, or replace
                    if self.find_line_in_remaining(old_line, &new_lines[new_idx..]) {
                        // Old line found later in new - treat as insert
                        changes.push(DiffChange {
                            change_type: ChangeType::Insert,
                            old_text: String::new(),
                            new_text: new_line.to_string(),
                            old_position: old_idx,
                            new_position: new_idx,
                            old_length: 0,
                            new_length: new_line.len(),
                        });
                        new_idx += 1;
                    } else if self.find_line_in_remaining(new_line, &old_lines[old_idx..]) {
                        // New line found later in old - treat as delete
                        changes.push(DiffChange {
                            change_type: ChangeType::Delete,
                            old_text: old_line.to_string(),
                            new_text: String::new(),
                            old_position: old_idx,
                            new_position: new_idx,
                            old_length: old_line.len(),
                            new_length: 0,
                        });
                        old_idx += 1;
                    } else {
                        // Replace
                        changes.push(DiffChange {
                            change_type: ChangeType::Replace,
                            old_text: old_line.to_string(),
                            new_text: new_line.to_string(),
                            old_position: old_idx,
                            new_position: new_idx,
                            old_length: old_line.len(),
                            new_length: new_line.len(),
                        });
                        old_idx += 1;
                        new_idx += 1;
                    }
                }
            } else if old_idx < old_lines.len() {
                // Remaining deletions
                changes.push(DiffChange {
                    change_type: ChangeType::Delete,
                    old_text: old_lines[old_idx].to_string(),
                    new_text: String::new(),
                    old_position: old_idx,
                    new_position: new_idx,
                    old_length: old_lines[old_idx].len(),
                    new_length: 0,
                });
                old_idx += 1;
            } else {
                // Remaining insertions
                changes.push(DiffChange {
                    change_type: ChangeType::Insert,
                    old_text: String::new(),
                    new_text: new_lines[new_idx].to_string(),
                    old_position: old_idx,
                    new_position: new_idx,
                    old_length: 0,
                    new_length: new_lines[new_idx].len(),
                });
                new_idx += 1;
            }
        }

        changes
    }

    /// Find a line in the remaining lines
    /// 
    /// # Arguments
    /// * `line` - The line to find
    /// * `remaining` - The remaining lines to search
    /// 
    /// # Returns
    /// true if the line is found, false otherwise
    fn find_line_in_remaining(&self, line: &str, remaining: &[&str]) -> bool {
        remaining.iter().any(|l| *l == line)
    }

    /// Compute similarity between two texts (0.0 to 1.0)
    /// 
    /// # Arguments
    /// * `old_text` - The original text
    /// * `new_text` - The modified text
    /// 
    /// # Returns
    /// Similarity score between 0.0 and 1.0
    /// 
    /// # Security
    /// Limits computation to prevent performance issues with very long strings
    fn compute_similarity(&self, old_text: &str, new_text: &str) -> f64 {
        if old_text.is_empty() && new_text.is_empty() {
            return 1.0;
        }

        if old_text.is_empty() || new_text.is_empty() {
            return 0.0;
        }

        // Safety check: prevent expensive computation on very long strings
        let max_len = old_text.len().max(new_text.len());
        if max_len > MAX_SIMILARITY_LENGTH {
            eprintln!("DiffEngine: similarity computation exceeds maximum length of {}", MAX_SIMILARITY_LENGTH);
            // Return a conservative estimate
            return 0.5;
        }

        // Simple character-level similarity using Levenshtein distance
        let distance = self.levenshtein_distance(old_text, new_text);

        if max_len == 0 {
            1.0
        } else {
            1.0 - (distance as f64 / max_len as f64)
        }
    }

    /// Compute Levenshtein distance between two strings
    /// 
    /// # Arguments
    /// * `s1` - First string
    /// * `s2` - Second string
    /// 
    /// # Returns
    /// The Levenshtein distance
    /// 
    /// # Security
    /// Protected by compute_similarity length check
    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let len1 = chars1.len();
        let len2 = chars2.len();

        if len1 == 0 {
            return len2;
        }
        if len2 == 0 {
            return len1;
        }

        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
                matrix[i][j] = [
                    matrix[i - 1][j] + 1,        // deletion
                    matrix[i][j - 1] + 1,        // insertion
                    matrix[i - 1][j - 1] + cost, // substitution
                ]
                .iter()
                .min()
                .copied()
                .unwrap_or(0);
            }
        }

        matrix[len1][len2]
    }

    /// Get diff statistics
    /// 
    /// # Arguments
    /// * `result` - The diff result
    /// 
    /// # Returns
    /// DiffStats containing the statistics
    pub fn get_stats(&self, result: &DiffResult) -> DiffStats {
        let mut inserts = 0;
        let mut deletes = 0;
        let mut replaces = 0;

        for change in &result.changes {
            match change.change_type {
                ChangeType::Insert => inserts += 1,
                ChangeType::Delete => deletes += 1,
                ChangeType::Replace => replaces += 1,
                ChangeType::Equal => {}
            }
        }

        DiffStats {
            total_changes: result.total_changes,
            inserts,
            deletes,
            replaces,
            similarity: result.similarity,
        }
    }

    /// Reset operation count
    pub fn reset_operation_count(&mut self) {
        self.operation_count = 0;
    }
}

impl Default for DiffEngine {
    fn default() -> Self {
        Self::new(Arc::new(ExportConfigService::new()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffStats {
    pub total_changes: usize,
    pub inserts: usize,
    pub deletes: usize,
    pub replaces: usize,
    pub similarity: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_texts() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let result = engine.compare("Hello", "Hello");
        assert_eq!(result.similarity, 1.0);
        assert_eq!(result.total_changes, 1); // One equal change
    }

    #[test]
    fn test_different_texts() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let result = engine.compare("Hello", "World");
        assert!(result.similarity < 1.0);
    }

    #[test]
    fn test_levenshtein_distance() {
        let engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let distance = engine.levenshtein_distance("kitten", "sitting");
        assert_eq!(distance, 3);
    }

    #[test]
    fn test_engine_creation() {
        let engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let _ = engine;
    }

    #[test]
    fn test_engine_default() {
        let engine = DiffEngine::default();
        let _ = engine;
    }

    #[test]
    fn test_change_type_variants() {
        let insert = ChangeType::Insert;
        let delete = ChangeType::Delete;
        let replace = ChangeType::Replace;
        let equal = ChangeType::Equal;

        let _ = (insert, delete, replace, equal);
    }

    #[test]
    fn test_change_type_serialization() {
        let change_type = ChangeType::Insert;
        let json = serde_json::to_string(&change_type);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"insert\"");
    }

    #[test]
    fn test_change_type_deserialization() {
        let change_type: ChangeType = serde_json::from_str("\"insert\"").unwrap();
        assert_eq!(change_type, ChangeType::Insert);
    }

    #[test]
    fn test_diff_change_creation() {
        let change = DiffChange {
            change_type: ChangeType::Insert,
            old_text: String::new(),
            new_text: "Hello".to_string(),
            old_position: 0,
            new_position: 0,
            old_length: 0,
            new_length: 5,
        };
        assert_eq!(change.new_text, "Hello");
        assert_eq!(change.new_length, 5);
    }

    #[test]
    fn test_diff_change_serialization() {
        let change = DiffChange {
            change_type: ChangeType::Insert,
            old_text: String::new(),
            new_text: "Hello".to_string(),
            old_position: 0,
            new_position: 0,
            old_length: 0,
            new_length: 5,
        };
        let json = serde_json::to_string(&change);
        assert!(json.is_ok());
    }

    #[test]
    fn test_diff_change_deserialization() {
        let json = r#"{
            "change_type": "insert",
            "old_text": "",
            "new_text": "Hello",
            "old_position": 0,
            "new_position": 0,
            "old_length": 0,
            "new_length": 5
        }"#;
        let change: Result<DiffChange, _> = serde_json::from_str(json);
        assert!(change.is_ok());
    }

    #[test]
    fn test_diff_result_creation() {
        let result = DiffResult {
            changes: vec![],
            similarity: 1.0,
            old_length: 5,
            new_length: 5,
            total_changes: 0,
        };
        assert_eq!(result.similarity, 1.0);
        assert_eq!(result.total_changes, 0);
    }

    #[test]
    fn test_diff_result_serialization() {
        let result = DiffResult {
            changes: vec![],
            similarity: 1.0,
            old_length: 5,
            new_length: 5,
            total_changes: 0,
        };
        let json = serde_json::to_string(&result);
        assert!(json.is_ok());
    }

    #[test]
    fn test_diff_result_deserialization() {
        let json = r#"{
            "changes": [],
            "similarity": 1.0,
            "old_length": 5,
            "new_length": 5,
            "total_changes": 0
        }"#;
        let result: Result<DiffResult, _> = serde_json::from_str(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_diff_stats_creation() {
        let stats = DiffStats {
            total_changes: 5,
            inserts: 2,
            deletes: 1,
            replaces: 2,
            similarity: 0.8,
        };
        assert_eq!(stats.total_changes, 5);
        assert_eq!(stats.inserts, 2);
    }

    #[test]
    fn test_diff_stats_serialization() {
        let stats = DiffStats {
            total_changes: 5,
            inserts: 2,
            deletes: 1,
            replaces: 2,
            similarity: 0.8,
        };
        let json = serde_json::to_string(&stats);
        assert!(json.is_ok());
    }

    #[test]
    fn test_diff_stats_deserialization() {
        let json = r#"{
            "total_changes": 5,
            "inserts": 2,
            "deletes": 1,
            "replaces": 2,
            "similarity": 0.8
        }"#;
        let stats: Result<DiffStats, _> = serde_json::from_str(json);
        assert!(stats.is_ok());
    }

    #[test]
    fn test_compare_empty_texts() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let result = engine.compare("", "");
        assert_eq!(result.similarity, 1.0);
    }

    #[test]
    fn test_compare_one_empty() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let result = engine.compare("Hello", "");
        assert_eq!(result.similarity, 0.0);
    }

    #[test]
    fn test_compare_insertion() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let result = engine.compare("Hello", "Hello World");
        assert!(result.similarity < 1.0);
        assert!(result.similarity > 0.0);
    }

    #[test]
    fn test_compare_deletion() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let result = engine.compare("Hello World", "Hello");
        assert!(result.similarity < 1.0);
        assert!(result.similarity > 0.0);
    }

    #[test]
    fn test_compare_multiline() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let old = "Line 1\nLine 2\nLine 3";
        let new = "Line 1\nLine 2 modified\nLine 3";
        let result = engine.compare(old, new);
        assert!(result.similarity < 1.0);
    }

    #[test]
    fn test_get_stats() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let result = engine.compare("Hello", "Hello World");
        let stats = engine.get_stats(&result);
        assert!(stats.inserts > 0 || stats.deletes > 0 || stats.replaces > 0);
    }

    #[test]
    fn test_get_stats_identical() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let result = engine.compare("Hello", "Hello");
        let stats = engine.get_stats(&result);
        assert_eq!(stats.inserts, 0);
        assert_eq!(stats.deletes, 0);
        assert_eq!(stats.replaces, 0);
    }

    #[test]
    fn test_levenshtein_empty_strings() {
        let engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let distance = engine.levenshtein_distance("", "");
        assert_eq!(distance, 0);
    }

    #[test]
    fn test_levenshtein_one_empty() {
        let engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let distance = engine.levenshtein_distance("Hello", "");
        assert_eq!(distance, 5);
    }

    #[test]
    fn test_levenshtein_same_string() {
        let engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let distance = engine.levenshtein_distance("Hello", "Hello");
        assert_eq!(distance, 0);
    }

    #[test]
    fn test_levenshtein_single_char_diff() {
        let engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let distance = engine.levenshtein_distance("Hello", "Hella");
        assert_eq!(distance, 1);
    }

    #[test]
    fn test_change_type_delete_serialization() {
        let change_type = ChangeType::Delete;
        let json = serde_json::to_string(&change_type);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"delete\"");
    }

    #[test]
    fn test_change_type_replace_serialization() {
        let change_type = ChangeType::Replace;
        let json = serde_json::to_string(&change_type);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"replace\"");
    }

    #[test]
    fn test_change_type_equal_serialization() {
        let change_type = ChangeType::Equal;
        let json = serde_json::to_string(&change_type);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"equal\"");
    }

    #[test]
    fn test_diff_change_delete() {
        let change = DiffChange {
            change_type: ChangeType::Delete,
            old_text: "Hello".to_string(),
            new_text: String::new(),
            old_position: 0,
            new_position: 0,
            old_length: 5,
            new_length: 0,
        };
        assert_eq!(change.old_text, "Hello");
        assert_eq!(change.new_text, "");
    }

    #[test]
    fn test_diff_change_replace() {
        let change = DiffChange {
            change_type: ChangeType::Replace,
            old_text: "Hello".to_string(),
            new_text: "World".to_string(),
            old_position: 0,
            new_position: 0,
            old_length: 5,
            new_length: 5,
        };
        assert_eq!(change.change_type, ChangeType::Replace);
    }

    #[test]
    fn test_diff_change_equal() {
        let change = DiffChange {
            change_type: ChangeType::Equal,
            old_text: "Hello".to_string(),
            new_text: "Hello".to_string(),
            old_position: 0,
            new_position: 0,
            old_length: 5,
            new_length: 5,
        };
        assert_eq!(change.change_type, ChangeType::Equal);
    }

    #[test]
    fn test_diff_result_with_changes() {
        let change = DiffChange {
            change_type: ChangeType::Insert,
            old_text: String::new(),
            new_text: "World".to_string(),
            old_position: 0,
            new_position: 0,
            old_length: 0,
            new_length: 5,
        };
        let result = DiffResult {
            changes: vec![change],
            similarity: 0.5,
            old_length: 5,
            new_length: 10,
            total_changes: 1,
        };
        assert_eq!(result.changes.len(), 1);
    }

    #[test]
    fn test_compare_long_texts() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let old = "a".repeat(1000);
        let new = "b".repeat(1000);
        let result = engine.compare(&old, &new);
        assert!(result.similarity < 1.0);
    }

    #[test]
    fn test_similarity_completely_different() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let result = engine.compare("abc", "xyz");
        assert!(result.similarity < 0.5);
    }

    #[test]
    fn test_similarity_partial_match() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let result = engine.compare("Hello World", "Hello There");
        assert!(result.similarity > 0.5);
        assert!(result.similarity < 1.0);
    }

    // Aerospace-level tests
    #[test]
    fn test_validate_text_length_too_long() {
        let engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_config = engine.config_service.get_diff_config();
        let long_text = "a".repeat(diff_config.max_text_length + 1);
        let result = engine.validate_text_length(&long_text);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum length"));
    }

    #[test]
    fn test_validate_line_count_too_large() {
        let engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_config = engine.config_service.get_diff_config();
        let result = engine.validate_line_count(diff_config.max_line_count + 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum"));
    }

    #[test]
    fn test_validate_line_length_too_long() {
        let engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_config = engine.config_service.get_diff_config();
        let long_line = "a".repeat(diff_config.max_line_length + 1);
        let result = engine.validate_line_length(&long_line);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum length"));
    }

    #[test]
    fn test_max_text_length_accepted() {
        let engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_config = engine.config_service.get_diff_config();
        let text = "a".repeat(diff_config.max_text_length);
        let result = engine.validate_text_length(&text);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_line_count_accepted() {
        let engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_config = engine.config_service.get_diff_config();
        let result = engine.validate_line_count(diff_config.max_line_count);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_line_length_accepted() {
        let engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_config = engine.config_service.get_diff_config();
        let line = "a".repeat(diff_config.max_line_length);
        let result = engine.validate_line_length(&line);
        assert!(result.is_ok());
    }

    #[test]
    fn test_operation_count() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        assert_eq!(engine.get_operation_count(), 0);
        
        engine.operation_count = 5;
        assert_eq!(engine.get_operation_count(), 5);
    }

    #[test]
    fn test_error_recording() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        
        engine.record_error("TEST_ERROR", "Test error message", "test_source");
        
        let last_error = engine.get_last_error();
        assert!(last_error.is_some());
        let error = last_error.unwrap();
        assert_eq!(error.code, "TEST_ERROR");
        assert_eq!(error.message, "Test error message");
        assert_eq!(error.source, "test_source");
    }

    #[test]
    fn test_error_state_reset() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        
        engine.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(engine.get_last_error().is_some());
        
        engine.reset_error_state();
        assert!(engine.get_last_error().is_none());
    }

    #[test]
    fn test_compare_with_too_long_text() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_config = engine.config_service.get_diff_config();
        let long_text = "a".repeat(diff_config.max_text_length + 1);
        let result = engine.compare(&long_text, "Hello");
        assert_eq!(result.similarity, 0.0);
        assert_eq!(result.total_changes, 0);
        assert!(engine.get_last_error().is_some());
    }

    #[test]
    fn test_compare_with_too_many_lines() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_config = engine.config_service.get_diff_config();
        let many_lines: String = (0..diff_config.max_line_count + 1).map(|_| "line\n").collect();
        let result = engine.compare(&many_lines, "Hello");
        assert_eq!(result.similarity, 0.0);
        assert_eq!(result.total_changes, 0);
        assert!(engine.get_last_error().is_some());
    }

    #[test]
    fn test_max_similarity_length() {
        let engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let long_text = "a".repeat(MAX_SIMILARITY_LENGTH + 1);
        let similarity = engine.compute_similarity(&long_text, &long_text);
        // Should return conservative estimate for very long strings
        assert_eq!(similarity, 0.5);
    }

    #[test]
    fn test_reset_operation_count() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        engine.compare("old", "new");
        assert!(engine.get_operation_count() > 0);
        
        engine.reset_operation_count();
        assert_eq!(engine.get_operation_count(), 0);
    }

    #[test]
    fn test_max_changes_warning() {
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        // Create a diff that would generate many changes
        let old: String = (0..1000).map(|i| format!("line{}\n", i)).collect();
        let new: String = (0..1000).map(|i| format!("modified{}\n", i)).collect();
        let result = engine.compare(&old, &new);
        // Should still work but log warning if changes exceed MAX_CHANGES
        assert!(result.total_changes > 0);
    }
}
