use regex::Regex;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use crate::error_handling::CircuitBreaker;
use crate::config_service::ExportConfigService;
use std::sync::Arc;

/// Maximum text length for search/replace to prevent memory issues
const MAX_TEXT_LENGTH: usize = 10 * 1024 * 1024; // 10MB

/// Maximum pattern length to prevent DoS attacks
const MAX_PATTERN_LENGTH: usize = 10_000; // 10k characters

/// Maximum replacement length to prevent memory issues
const MAX_REPLACEMENT_LENGTH: usize = 100_000; // 100k characters

/// Maximum number of matches to prevent performance issues
const MAX_MATCHES: usize = 10_000;

/// Maximum regex complexity (estimated by pattern length)
const MAX_REGEX_LENGTH: usize = 5_000;

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 500;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 2000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOptions {
    pub case_sensitive: bool,
    pub whole_word: bool,
    pub use_regex: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            whole_word: false,
            use_regex: false,
        }
    }
}

impl SearchOptions {
    /// Validates the search options
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Currently always returns Ok, but can be extended for additional validation
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }

    /// Get the maximum pattern length constant
    /// 
    /// # Returns
    /// The maximum pattern length
    pub fn max_pattern_length() -> usize {
        MAX_PATTERN_LENGTH
    }

    /// Get the maximum regex length constant
    /// 
    /// # Returns
    /// The maximum regex length
    pub fn max_regex_length() -> usize {
        MAX_REGEX_LENGTH
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub matches: Vec<MatchInfo>,
    pub total_count: usize,
    pub current_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchInfo {
    pub position: usize,
    pub length: usize,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceOptions {
    pub case_sensitive: bool,
    pub whole_word: bool,
    pub use_regex: bool,
    pub replace_all: bool,
}

impl Default for ReplaceOptions {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            whole_word: false,
            use_regex: false,
            replace_all: false,
        }
    }
}

impl ReplaceOptions {
    /// Validates the replace options
    /// 
    /// # Returns
    /// Result indicating success or failure
    /// 
    /// # Security
    /// Currently always returns Ok, but can be extended for additional validation
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }

    /// Get the maximum replacement length constant
    /// 
    /// # Returns
    /// The maximum replacement length
    pub fn max_replacement_length() -> usize {
        MAX_REPLACEMENT_LENGTH
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceResult {
    pub replaced_count: usize,
    pub new_text: String,
    pub success: bool,
}

pub struct SearchReplaceService {
    config_service: Arc<ExportConfigService>,
    circuit_breaker: CircuitBreaker,
}

impl SearchReplaceService {
    /// Creates a new search replace service instance
    /// 
    /// # Returns
    /// A new SearchReplaceService instance
    pub fn new() -> Self {
        let config_service = Arc::new(ExportConfigService::new());
        let circuit_breaker = CircuitBreaker::new(config_service.clone());
        Self {
            config_service,
            circuit_breaker,
        }
    }

    /// Get the maximum text length constant
    /// 
    /// # Returns
    /// The maximum text length in bytes
    pub fn max_text_length() -> usize {
        MAX_TEXT_LENGTH
    }

    /// Get the maximum matches constant
    /// 
    /// # Returns
    /// The maximum number of matches
    pub fn max_matches() -> usize {
        MAX_MATCHES
    }

    /// Get the performance warning threshold
    /// 
    /// # Returns
    /// The performance warning threshold in milliseconds
    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    /// Get the performance critical threshold
    /// 
    /// # Returns
    /// The performance critical threshold in milliseconds
    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    /// Searches for text pattern in the given text
    /// 
    /// # Arguments
    /// * `text` - The text to search in
    /// * `pattern` - The pattern to search for
    /// * `options` - Search options (case sensitivity, whole word, regex)
    /// * `start_position` - Position to start searching from
    /// 
    /// # Returns
    /// SearchResult containing matches and metadata
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates input sizes to prevent DoS attacks
    pub fn find_text(
        &self,
        text: &str,
        pattern: &str,
        options: &SearchOptions,
        start_position: usize,
    ) -> SearchResult {
        let start_time = Instant::now();
        let mut matches = Vec::new();
        
        // Check circuit breaker
        if !self.circuit_breaker.allow_operation() {
            eprintln!("Circuit breaker is open, blocking search operations");
            return SearchResult {
                matches,
                total_count: 0,
                current_index: 0,
            };
        }
        
        // Input validation
        if pattern.is_empty() {
            self.circuit_breaker.record_success();
            return SearchResult {
                matches,
                total_count: 0,
                current_index: 0,
            };
        }

        // Security check: prevent DoS with oversized input
        if text.len() > MAX_TEXT_LENGTH {
            eprintln!("Search: text exceeds maximum size of {} bytes", MAX_TEXT_LENGTH);
            self.circuit_breaker.record_failure();
            return SearchResult {
                matches,
                total_count: 0,
                current_index: 0,
            };
        }

        // Security check: prevent DoS with oversized pattern
        if pattern.len() > MAX_PATTERN_LENGTH {
            eprintln!("Search: pattern exceeds maximum length of {} characters", MAX_PATTERN_LENGTH);
            self.circuit_breaker.record_failure();
            return SearchResult {
                matches,
                total_count: 0,
                current_index: 0,
            };
        }

        // Validate options
        if let Err(e) = options.validate() {
            eprintln!("Search options validation failed: {}", e);
            self.circuit_breaker.record_failure();
            return SearchResult {
                matches,
                total_count: 0,
                current_index: 0,
            };
        }

        // Security check: validate start position
        if start_position > text.len() {
            eprintln!("Search: start_position {} exceeds text length {}", start_position, text.len());
            return SearchResult {
                matches,
                total_count: 0,
                current_index: 0,
            };
        }

        let search_text = if options.case_sensitive {
            text.to_string()
        } else {
            text.to_lowercase()
        };

        let search_pattern = if options.case_sensitive {
            pattern.to_string()
        } else {
            pattern.to_lowercase()
        };

        if options.use_regex {
            // Security check: prevent complex regex
            if pattern.len() > MAX_REGEX_LENGTH {
                eprintln!("Search: regex pattern exceeds maximum length of {} characters", MAX_REGEX_LENGTH);
                self.literal_search(&search_text, &search_pattern, options.whole_word, &mut matches);
            } else {
                match Regex::new(pattern) {
                    Ok(_re) => {
                        let flags = if options.case_sensitive {
                            regex::Regex::new(pattern).unwrap()
                        } else {
                            Regex::new(&format!("(?i){}", pattern)).unwrap()
                        };
                        
                        for mat in flags.find_iter(text) {
                            // Safety check: prevent too many matches
                            if matches.len() >= MAX_MATCHES {
                                eprintln!("Search: reached maximum match limit of {}", MAX_MATCHES);
                                break;
                            }
                            matches.push(MatchInfo {
                                position: mat.start(),
                                length: mat.end() - mat.start(),
                                text: mat.as_str().to_string(),
                            });
                        }
                    }
                    Err(e) => {
                        eprintln!("Search: invalid regex pattern '{}': {}", pattern, e);
                        // Invalid regex, fall back to literal search
                        self.literal_search(&search_text, &search_pattern, options.whole_word, &mut matches);
                    }
                }
            }
        } else {
            self.literal_search(&search_text, &search_pattern, options.whole_word, &mut matches);
        }

        let total_count = matches.len();
        let current_index = if start_position == 0 {
            0
        } else {
            matches.iter().position(|m| m.position >= start_position).unwrap_or(0)
        };

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Search CRITICAL performance warning: took {}ms for {} characters", 
                elapsed.as_millis(), text.len());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Search performance warning: took {}ms for {} characters", 
                elapsed.as_millis(), text.len());
        }

        self.circuit_breaker.record_success();

        SearchResult {
            matches,
            total_count,
            current_index,
        }
    }

    /// Performs literal string search (non-regex)
    /// 
    /// # Arguments
    /// * `text` - The text to search in
    /// * `pattern` - The pattern to search for
    /// * `whole_word` - Whether to match whole words only
    /// * `matches` - Vector to store found matches
    /// 
    /// # Performance
    /// O(n*m) time complexity where n is text length and m is pattern length
    fn literal_search(&self, text: &str, pattern: &str, whole_word: bool, matches: &mut Vec<MatchInfo>) {
        let mut start = 0;
        while let Some(pos) = text[start..].find(pattern) {
            let absolute_pos = start + pos;
            
            // Safety check: prevent too many matches
            if matches.len() >= MAX_MATCHES {
                eprintln!("Literal search: reached maximum match limit of {}", MAX_MATCHES);
                break;
            }
            
            if whole_word {
                // Check word boundaries
                let before = if absolute_pos > 0 {
                    text.chars().nth(absolute_pos - 1)
                } else {
                    None
                };
                let after = text.chars().nth(absolute_pos + pattern.len());
                
                let is_word_before = before.map(|c| c.is_alphanumeric()).unwrap_or(false);
                let is_word_after = after.map(|c| c.is_alphanumeric()).unwrap_or(false);
                
                if is_word_before || is_word_after {
                    start = absolute_pos + 1;
                    continue;
                }
            }
            
            matches.push(MatchInfo {
                position: absolute_pos,
                length: pattern.len(),
                text: pattern.to_string(),
            });
            
            start = absolute_pos + pattern.len();
        }
    }

    /// Replaces text pattern with replacement text
    /// 
    /// # Arguments
    /// * `text` - The text to modify
    /// * `pattern` - The pattern to search for
    /// * `replacement` - The replacement text
    /// * `options` - Replace options (case sensitivity, whole word, regex, replace all)
    /// 
    /// # Returns
    /// ReplaceResult containing the modified text and replacement count
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates input sizes to prevent DoS attacks and memory issues
    pub fn replace_text(
        &self,
        text: &str,
        pattern: &str,
        replacement: &str,
        options: &ReplaceOptions,
    ) -> ReplaceResult {
        let start_time = Instant::now();
        
        // Input validation
        if pattern.is_empty() {
            return ReplaceResult {
                replaced_count: 0,
                new_text: text.to_string(),
                success: true,
            };
        }

        // Security check: prevent DoS with oversized input
        if text.len() > MAX_TEXT_LENGTH {
            eprintln!("Replace: text exceeds maximum size of {} bytes", MAX_TEXT_LENGTH);
            return ReplaceResult {
                replaced_count: 0,
                new_text: text.to_string(),
                success: false,
            };
        }

        // Security check: prevent DoS with oversized pattern
        if pattern.len() > MAX_PATTERN_LENGTH {
            eprintln!("Replace: pattern exceeds maximum length of {} characters", MAX_PATTERN_LENGTH);
            return ReplaceResult {
                replaced_count: 0,
                new_text: text.to_string(),
                success: false,
            };
        }

        // Security check: prevent memory issues with oversized replacement
        if replacement.len() > MAX_REPLACEMENT_LENGTH {
            eprintln!("Replace: replacement exceeds maximum length of {} characters", MAX_REPLACEMENT_LENGTH);
            return ReplaceResult {
                replaced_count: 0,
                new_text: text.to_string(),
                success: false,
            };
        }

        // Validate options
        if let Err(e) = options.validate() {
            eprintln!("Replace options validation failed: {}", e);
            return ReplaceResult {
                replaced_count: 0,
                new_text: text.to_string(),
                success: false,
            };
        }

        let search_options = SearchOptions {
            case_sensitive: options.case_sensitive,
            whole_word: options.whole_word,
            use_regex: options.use_regex,
        };

        let search_result = self.find_text(text, pattern, &search_options, 0);
        
        if search_result.total_count == 0 {
            return ReplaceResult {
                replaced_count: 0,
                new_text: text.to_string(),
                success: true,
            };
        }

        if options.replace_all {
            // Replace all matches
            let new_text = if options.use_regex {
                // Security check: prevent complex regex
                if pattern.len() > MAX_REGEX_LENGTH {
                    eprintln!("Replace: regex pattern exceeds maximum length of {} characters", MAX_REGEX_LENGTH);
                    text.replace(pattern, replacement)
                } else {
                    match Regex::new(pattern) {
                        Ok(re) => {
                            let flags = if options.case_sensitive {
                                re
                            } else {
                                Regex::new(&format!("(?i){}", pattern)).unwrap()
                            };
                            flags.replace_all(text, replacement).to_string()
                        }
                        Err(e) => {
                            eprintln!("Replace: invalid regex pattern '{}': {}", pattern, e);
                            // Invalid regex, fall back to literal replacement
                            text.replace(pattern, replacement)
                        }
                    }
                }
            } else {
                text.replace(pattern, replacement)
            };

            // Performance monitoring
            let elapsed = start_time.elapsed();
            if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
                eprintln!("Replace CRITICAL performance warning: took {}ms for {} replacements", 
                    elapsed.as_millis(), search_result.total_count);
            } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
                eprintln!("Replace performance warning: took {}ms for {} replacements", 
                    elapsed.as_millis(), search_result.total_count);
            }

            ReplaceResult {
                replaced_count: search_result.total_count,
                new_text,
                success: true,
            }
        } else {
            // Replace first match only
            if let Some(first_match) = search_result.matches.first() {
                let new_text = format!(
                    "{}{}{}",
                    &text[..first_match.position],
                    replacement,
                    &text[first_match.position + first_match.length..]
                );

                // Performance monitoring
                let elapsed = start_time.elapsed();
                if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
                    eprintln!("Replace CRITICAL performance warning: took {}ms", elapsed.as_millis());
                } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
                    eprintln!("Replace performance warning: took {}ms", elapsed.as_millis());
                }

                ReplaceResult {
                    replaced_count: 1,
                    new_text,
                    success: true,
                }
            } else {
                ReplaceResult {
                    replaced_count: 0,
                    new_text: text.to_string(),
                    success: true,
                }
            }
        }
    }
}

impl Default for SearchReplaceService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_search() {
        let service = SearchReplaceService::new();
        let options = SearchOptions::default();
        let result = service.find_text("hello world hello", "hello", &options, 0);
        assert_eq!(result.total_count, 2);
    }

    #[test]
    fn test_case_sensitive_search() {
        let service = SearchReplaceService::new();
        let options = SearchOptions {
            case_sensitive: true,
            ..Default::default()
        };
        let result = service.find_text("Hello hello", "hello", &options, 0);
        assert_eq!(result.total_count, 1);
    }

    #[test]
    fn test_case_insensitive_search() {
        let service = SearchReplaceService::new();
        let options = SearchOptions {
            case_sensitive: false,
            ..Default::default()
        };
        let result = service.find_text("Hello hello", "hello", &options, 0);
        assert_eq!(result.total_count, 2);
    }

    #[test]
    fn test_whole_word_search() {
        let service = SearchReplaceService::new();
        let options = SearchOptions {
            whole_word: true,
            ..Default::default()
        };
        let result = service.find_text("hello helloworld", "hello", &options, 0);
        assert_eq!(result.total_count, 1);
    }

    #[test]
    fn test_regex_search() {
        let service = SearchReplaceService::new();
        let options = SearchOptions {
            use_regex: true,
            ..Default::default()
        };
        let result = service.find_text("hello123 world456", r"\d+", &options, 0);
        assert_eq!(result.total_count, 2);
    }

    #[test]
    fn test_replace_all() {
        let service = SearchReplaceService::new();
        let options = ReplaceOptions {
            replace_all: true,
            ..Default::default()
        };
        let result = service.replace_text("hello world hello", "hello", "hi", &options);
        assert_eq!(result.replaced_count, 2);
        assert_eq!(result.new_text, "hi world hi");
    }

    #[test]
    fn test_replace_first() {
        let service = SearchReplaceService::new();
        let options = ReplaceOptions {
            replace_all: false,
            ..Default::default()
        };
        let result = service.replace_text("hello world hello", "hello", "hi", &options);
        assert_eq!(result.replaced_count, 1);
        assert_eq!(result.new_text, "hi world hello");
    }

    #[test]
    fn test_empty_pattern() {
        let service = SearchReplaceService::new();
        let options = SearchOptions::default();
        let result = service.find_text("hello world", "", &options, 0);
        assert_eq!(result.total_count, 0);
    }

    #[test]
    fn test_no_matches() {
        let service = SearchReplaceService::new();
        let options = SearchOptions::default();
        let result = service.find_text("hello world", "xyz", &options, 0);
        assert_eq!(result.total_count, 0);
    }

    #[test]
    fn test_max_text_length() {
        let service = SearchReplaceService::new();
        let options = SearchOptions::default();
        let large_text = "a".repeat(MAX_TEXT_LENGTH + 1);
        let result = service.find_text(&large_text, "a", &options, 0);
        assert_eq!(result.total_count, 0);
    }

    #[test]
    fn test_max_pattern_length() {
        let service = SearchReplaceService::new();
        let options = SearchOptions::default();
        let large_pattern = "a".repeat(MAX_PATTERN_LENGTH + 1);
        let result = service.find_text("hello world", &large_pattern, &options, 0);
        assert_eq!(result.total_count, 0);
    }

    #[test]
    fn test_invalid_start_position() {
        let service = SearchReplaceService::new();
        let options = SearchOptions::default();
        let result = service.find_text("hello world", "hello", &options, 100);
        assert_eq!(result.total_count, 0);
    }

    #[test]
    fn test_regex_invalid_pattern() {
        let service = SearchReplaceService::new();
        let options = SearchOptions {
            use_regex: true,
            ..Default::default()
        };
        let result = service.find_text("hello world", "[invalid", &options, 0);
        // Should fall back to literal search
        assert_eq!(result.total_count, 0);
    }

    #[test]
    fn test_regex_case_insensitive() {
        let service = SearchReplaceService::new();
        let options = SearchOptions {
            use_regex: true,
            case_sensitive: false,
            ..Default::default()
        };
        let result = service.find_text("Hello HELLO hello", "hello", &options, 0);
        assert_eq!(result.total_count, 3);
    }

    #[test]
    fn test_max_replacement_length() {
        let service = SearchReplaceService::new();
        let options = ReplaceOptions::default();
        let large_replacement = "a".repeat(MAX_REPLACEMENT_LENGTH + 1);
        let result = service.replace_text("hello world", "hello", &large_replacement, &options);
        assert!(!result.success);
    }

    #[test]
    fn test_replace_with_regex() {
        let service = SearchReplaceService::new();
        let options = ReplaceOptions {
            use_regex: true,
            replace_all: true,
            ..Default::default()
        };
        let result = service.replace_text("hello123 world456", r"\d+", "X", &options);
        assert_eq!(result.replaced_count, 2);
        assert_eq!(result.new_text, "helloX worldX");
    }

    #[test]
    fn test_replace_regex_invalid() {
        let service = SearchReplaceService::new();
        let options = ReplaceOptions {
            use_regex: true,
            replace_all: true,
            ..Default::default()
        };
        let result = service.replace_text("hello world", "[invalid", "X", &options);
        // Should fall back to literal replacement
        assert!(result.success);
    }

    #[test]
    fn test_search_with_start_position() {
        let service = SearchReplaceService::new();
        let options = SearchOptions::default();
        let result = service.find_text("hello world hello", "hello", &options, 6);
        assert_eq!(result.total_count, 2);
        assert_eq!(result.current_index, 1);
    }

    #[test]
    fn test_unicode_search() {
        let service = SearchReplaceService::new();
        let options = SearchOptions::default();
        let result = service.find_text("你好世界", "你好", &options, 0);
        assert_eq!(result.total_count, 1);
    }

    #[test]
    fn test_special_characters() {
        let service = SearchReplaceService::new();
        let options = SearchOptions::default();
        let result = service.find_text("hello@world.com", "@", &options, 0);
        assert_eq!(result.total_count, 1);
    }

    #[test]
    fn test_performance_large_text() {
        let service = SearchReplaceService::new();
        let options = SearchOptions::default();
        let large_text = "hello ".repeat(10_000);
        let result = service.find_text(&large_text, "hello", &options, 0);
        assert_eq!(result.total_count, 10_000);
    }

    #[test]
    fn test_max_matches_limit() {
        let service = SearchReplaceService::new();
        let options = SearchOptions::default();
        let large_text = "a ".repeat(MAX_MATCHES + 100);
        let result = service.find_text(&large_text, "a", &options, 0);
        // Should stop at MAX_MATCHES
        assert!(result.total_count <= MAX_MATCHES);
    }

    #[test]
    fn test_max_regex_length() {
        let service = SearchReplaceService::new();
        let options = SearchOptions {
            use_regex: true,
            ..Default::default()
        };
        let large_pattern = "a".repeat(MAX_REGEX_LENGTH + 1);
        let result = service.find_text("hello world", &large_pattern, &options, 0);
        // Should fall back to literal search
        assert_eq!(result.total_count, 0);
    }

    #[test]
    fn test_search_options_getters() {
        assert_eq!(SearchOptions::max_pattern_length(), MAX_PATTERN_LENGTH);
        assert_eq!(SearchOptions::max_regex_length(), MAX_REGEX_LENGTH);
    }

    #[test]
    fn test_replace_options_getters() {
        assert_eq!(ReplaceOptions::max_replacement_length(), MAX_REPLACEMENT_LENGTH);
    }

    #[test]
    fn test_service_getters() {
        assert_eq!(SearchReplaceService::max_text_length(), MAX_TEXT_LENGTH);
        assert_eq!(SearchReplaceService::max_matches(), MAX_MATCHES);
        assert_eq!(SearchReplaceService::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(SearchReplaceService::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_whole_word_boundaries() {
        let service = SearchReplaceService::new();
        let options = SearchOptions {
            whole_word: true,
            ..Default::default()
        };
        let result = service.find_text("hello,world", "hello", &options, 0);
        // Should match because comma is not alphanumeric
        assert_eq!(result.total_count, 1);
    }

    #[test]
    fn test_whole_word_start() {
        let service = SearchReplaceService::new();
        let options = SearchOptions {
            whole_word: true,
            ..Default::default()
        };
        let result = service.find_text("hello world", "hello", &options, 0);
        // Should match at start
        assert_eq!(result.total_count, 1);
    }

    #[test]
    fn test_whole_word_end() {
        let service = SearchReplaceService::new();
        let options = SearchOptions {
            whole_word: true,
            ..Default::default()
        };
        let result = service.find_text("world hello", "hello", &options, 0);
        // Should match at end
        assert_eq!(result.total_count, 1);
    }
}
