use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::time::Instant;
use crate::error_handling::CircuitBreaker;
use crate::config_service::ExportConfigService;
use std::sync::Arc;

/// Maximum text length for spell check to prevent memory issues
const MAX_TEXT_LENGTH: usize = 10 * 1024 * 1024; // 10MB

/// Maximum word length to prevent performance issues
const MAX_WORD_LENGTH: usize = 100;

/// Maximum number of errors to report to prevent performance issues
const MAX_ERRORS: usize = 10_000;

/// Maximum number of suggestions per word
const MAX_SUGGESTIONS: usize = 10;

/// Maximum edit distance for suggestions
const MAX_EDIT_DISTANCE: usize = 3;

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 1000;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 5000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellingError {
    pub word: String,
    pub position: usize,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellCheckResult {
    pub errors: Vec<SpellingError>,
    pub total_words: usize,
    pub error_count: usize,
}

pub struct SpellChecker {
    dictionary: HashSet<String>,
    config_service: Arc<ExportConfigService>,
    circuit_breaker: CircuitBreaker,
}

impl SpellChecker {
    /// Creates a new spell checker instance with default dictionary
    /// 
    /// # Returns
    /// A new SpellChecker instance
    /// 
    /// # Note
    /// Loads a simplified English dictionary. In production, load from file or use a proper spellcheck library.
    pub fn new() -> Self {
        // Basic English dictionary (simplified for demo)
        // In production, load from file or use a proper spellcheck library
        let dictionary = Self::load_default_dictionary();
        let config_service = Arc::new(ExportConfigService::new());
        let circuit_breaker = CircuitBreaker::new(config_service.clone());
        Self { 
            dictionary,
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

    /// Get the maximum word length constant
    /// 
    /// # Returns
    /// The maximum word length in characters
    pub fn max_word_length() -> usize {
        MAX_WORD_LENGTH
    }

    /// Get the maximum errors constant
    /// 
    /// # Returns
    /// The maximum number of errors to report
    pub fn max_errors() -> usize {
        MAX_ERRORS
    }

    /// Get the maximum suggestions constant
    /// 
    /// # Returns
    /// The maximum number of suggestions per word
    pub fn max_suggestions() -> usize {
        MAX_SUGGESTIONS
    }

    /// Get the maximum edit distance constant
    /// 
    /// # Returns
    /// The maximum edit distance for suggestions
    pub fn max_edit_distance() -> usize {
        MAX_EDIT_DISTANCE
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

    /// Loads the default dictionary with common English words
    /// 
    /// # Returns
    /// A HashSet containing lowercase dictionary words
    /// 
    /// # Note
    /// This is a simplified dictionary for demonstration.
    /// In production, load from file or use a proper spellcheck library.
    fn load_default_dictionary() -> HashSet<String> {
        // This is a simplified dictionary for demonstration
        // In production, use a proper spellcheck library like `spellcheck` crate
        let words = vec![
            // Common words
            "the", "be", "to", "of", "and", "a", "in", "that", "have", "i",
            "it", "for", "not", "on", "with", "he", "as", "you", "do", "at",
            "this", "but", "his", "by", "from", "they", "we", "say", "her", "she",
            "or", "an", "will", "my", "one", "all", "would", "there", "their", "what",
            "so", "up", "out", "if", "about", "who", "get", "which", "go", "me",
            // Document-related words
            "document", "editor", "text", "content", "paragraph", "sentence",
            "word", "character", "page", "section", "chapter", "title", "heading",
            "format", "style", "font", "size", "color", "alignment", "spacing",
            "margin", "indent", "bullet", "list", "table", "image", "link", "reference",
            "citation", "footnote", "endnote", "index", "glossary", "appendix",
            "introduction", "conclusion", "summary", "abstract", "keywords",
            "author", "date", "version", "revision", "draft", "final", "published",
            "save", "open", "close", "edit", "delete", "insert", "replace", "find",
            "search", "copy", "paste", "cut", "undo", "redo", "print", "export",
            "import", "file", "folder", "directory", "path", "name", "extension",
            "typst", "latex", "markdown", "html", "pdf", "docx", "odt", "rtf",
            "rust", "javascript", "typescript", "vue", "react", "tauri", "electron",
        ];
        
        words.into_iter().map(|w| w.to_lowercase()).collect()
    }

    /// Checks text for spelling errors
    /// 
    /// # Arguments
    /// * `text` - The text to check for spelling errors
    /// 
    /// # Returns
    /// SpellCheckResult containing errors and statistics
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates input size to prevent DoS attacks
    pub fn check_text(&self, text: &str) -> SpellCheckResult {
        let start_time = Instant::now();
        
        // Check circuit breaker
        if !self.circuit_breaker.allow_operation() {
            eprintln!("Circuit breaker is open, blocking spell check operations");
            return SpellCheckResult {
                errors: Vec::new(),
                total_words: 0,
                error_count: 0,
            };
        }
        
        // Input validation
        if text.is_empty() {
            self.circuit_breaker.record_success();
            return SpellCheckResult {
                errors: Vec::new(),
                total_words: 0,
                error_count: 0,
            };
        }

        // Security check: prevent DoS with oversized input
        if text.len() > MAX_TEXT_LENGTH {
            eprintln!("Spell check: text exceeds maximum size of {} bytes", MAX_TEXT_LENGTH);
            self.circuit_breaker.record_failure();
            return SpellCheckResult {
                errors: Vec::new(),
                total_words: 0,
                error_count: 0,
            };
        }

        let words: Vec<&str> = text.split_whitespace().collect();
        let total_words = words.len();
        let mut errors = Vec::new();
        let mut position = 0;

        for word in words {
            // Safety check: prevent too many errors
            if errors.len() >= MAX_ERRORS {
                eprintln!("Spell check: reached maximum error limit of {}", MAX_ERRORS);
                break;
            }

            let clean_word = word
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase();
            
            // Skip empty words and words that are too long
            if clean_word.is_empty() || clean_word.len() > MAX_WORD_LENGTH {
                position += word.len() + 1;
                continue;
            }
            
            if !self.dictionary.contains(&clean_word) {
                // Check if it's a number or has numbers (might be valid)
                if !clean_word.chars().any(|c| c.is_numeric()) {
                    let suggestions = self.get_suggestions(&clean_word);
                    errors.push(SpellingError {
                        word: word.to_string(),
                        position,
                        suggestions,
                    });
                }
            }
            position += word.len() + 1; // +1 for space
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Spell check CRITICAL performance warning: took {}ms for {} words", 
                elapsed.as_millis(), total_words);
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Spell check performance warning: took {}ms for {} words", 
                elapsed.as_millis(), total_words);
        }

        self.circuit_breaker.record_success();

        SpellCheckResult {
            errors: errors.clone(),
            total_words,
            error_count: errors.len(),
        }
    }

    /// Gets spelling suggestions for a misspelled word
    /// 
    /// # Arguments
    /// * `word` - The misspelled word to get suggestions for
    /// 
    /// # Returns
    /// A vector of suggested words sorted by edit distance
    /// 
    /// # Algorithm
    /// Uses Levenshtein edit distance to find similar words in dictionary
    /// 
    /// # Performance
    /// O(n*m*k) where n is dictionary size, m is word length, k is max edit distance
    fn get_suggestions(&self, word: &str) -> Vec<String> {
        // Simple suggestion algorithm based on edit distance
        // In production, use a proper spellcheck library
        let mut suggestions: Vec<String> = self.dictionary
            .iter()
            .filter(|dict_word| {
                let distance = Self::edit_distance(word, dict_word);
                distance <= MAX_EDIT_DISTANCE && distance > 0
            })
            .cloned()
            .take(MAX_SUGGESTIONS)
            .collect();

        suggestions.sort_by_key(|s| Self::edit_distance(word, s));
        suggestions
    }

    /// Calculates Levenshtein edit distance between two strings
    /// 
    /// # Arguments
    /// * `a` - First string
    /// * `b` - Second string
    /// 
    /// # Returns
    /// The minimum number of single-character edits to transform a into b
    /// 
    /// # Algorithm
    /// Dynamic programming approach with O(m*n) time and space complexity
    /// where m and n are the lengths of the input strings
    /// 
    /// # Performance
    /// O(m*n) time complexity, O(m*n) space complexity
    fn edit_distance(a: &str, b: &str) -> usize {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let m = a_chars.len();
        let n = b_chars.len();

        if m == 0 {
            return n;
        }
        if n == 0 {
            return m;
        }

        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 0..=m {
            dp[i][0] = i;
        }
        for j in 0..=n {
            dp[0][j] = j;
        }

        for i in 1..=m {
            for j in 1..=n {
                if a_chars[i - 1] == b_chars[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + std::cmp::min(
                        std::cmp::min(dp[i - 1][j], dp[i][j - 1]),
                        dp[i - 1][j - 1],
                    );
                }
            }
        }

        dp[m][n]
    }

    /// Adds a custom word to the dictionary
    /// 
    /// # Arguments
    /// * `word` - The word to add to the dictionary
    /// 
    /// # Note
    /// The word is converted to lowercase before adding
    /// 
    /// # Security
    /// Validates word length to prevent memory issues
    pub fn add_word(&mut self, word: String) {
        // Validate word length
        if word.len() > MAX_WORD_LENGTH {
            eprintln!("Add word: word exceeds maximum length of {} characters", MAX_WORD_LENGTH);
            return;
        }
        self.dictionary.insert(word.to_lowercase());
    }

    /// Checks if a word exists in the dictionary
    /// 
    /// # Arguments
    /// * `word` - The word to check
    /// 
    /// # Returns
    /// true if the word exists in the dictionary, false otherwise
    pub fn has_word(&self, word: &str) -> bool {
        let clean_word = word.to_lowercase();
        self.dictionary.contains(&clean_word)
    }

    /// Gets the size of the dictionary
    /// 
    /// # Returns
    /// The number of words in the dictionary
    pub fn dictionary_size(&self) -> usize {
        self.dictionary.len()
    }

    /// Clears the dictionary
    /// 
    /// # Warning
    /// This will remove all words from the dictionary
    pub fn clear_dictionary(&mut self) {
        self.dictionary.clear();
    }

    /// Removes a word from the dictionary
    /// 
    /// # Arguments
    /// * `word` - The word to remove
    /// 
    /// # Returns
    /// true if the word was removed, false if it wasn't in the dictionary
    pub fn remove_word(&mut self, word: &str) -> bool {
        let clean_word = word.to_lowercase();
        self.dictionary.remove(&clean_word)
    }
}

impl Default for SpellChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spell_checker_creation() {
        let checker = SpellChecker::new();
        assert!(!checker.dictionary.is_empty());
    }

    #[test]
    fn test_check_correct_text() {
        let checker = SpellChecker::new();
        let result = checker.check_text("the document text");
        assert_eq!(result.error_count, 0);
    }

    #[test]
    fn test_check_incorrect_text() {
        let checker = SpellChecker::new();
        let result = checker.check_text("the documnt is savd");
        assert!(result.error_count > 0);
    }

    #[test]
    fn test_edit_distance() {
        assert_eq!(SpellChecker::edit_distance("kitten", "sitting"), 3);
        assert_eq!(SpellChecker::edit_distance("book", "back"), 2);
        assert_eq!(SpellChecker::edit_distance("same", "same"), 0);
    }

    #[test]
    fn test_add_word() {
        let mut checker = SpellChecker::new();
        checker.add_word("customword".to_string());
        let result = checker.check_text("customword");
        assert_eq!(result.error_count, 0);
    }

    #[test]
    fn test_empty_text() {
        let checker = SpellChecker::new();
        let result = checker.check_text("");
        assert_eq!(result.total_words, 0);
        assert_eq!(result.error_count, 0);
    }

    #[test]
    fn test_numbers_ignored() {
        let checker = SpellChecker::new();
        let result = checker.check_text("123 456 789");
        assert_eq!(result.error_count, 0);
    }

    #[test]
    fn test_max_text_length() {
        let checker = SpellChecker::new();
        let large_text = "a ".repeat(MAX_TEXT_LENGTH + 1);
        let result = checker.check_text(&large_text);
        assert_eq!(result.total_words, 0);
    }

    #[test]
    fn test_max_word_length() {
        let checker = SpellChecker::new();
        let long_word = "a".repeat(MAX_WORD_LENGTH + 1);
        let result = checker.check_text(&long_word);
        assert_eq!(result.error_count, 0);
    }

    #[test]
    fn test_max_errors_limit() {
        let checker = SpellChecker::new();
        let text = "xyz ".repeat(MAX_ERRORS + 10);
        let result = checker.check_text(&text);
        assert!(result.error_count <= MAX_ERRORS);
    }

    #[test]
    fn test_has_word() {
        let checker = SpellChecker::new();
        assert!(checker.has_word("document"));
        assert!(!checker.has_word("nonexistent"));
    }

    #[test]
    fn test_dictionary_size() {
        let checker = SpellChecker::new();
        assert!(checker.dictionary_size() > 0);
    }

    #[test]
    fn test_add_word_validation() {
        let mut checker = SpellChecker::new();
        let long_word = "a".repeat(MAX_WORD_LENGTH + 1);
        checker.add_word(long_word);
        assert!(!checker.has_word(&"a".repeat(MAX_WORD_LENGTH + 1)));
    }

    #[test]
    fn test_case_insensitive() {
        let checker = SpellChecker::new();
        assert!(checker.has_word("DOCUMENT"));
        assert!(checker.has_word("Document"));
        assert!(checker.has_word("document"));
    }

    #[test]
    fn test_suggestions_limit() {
        let checker = SpellChecker::new();
        let result = checker.check_text("xyz");
        if result.error_count > 0 {
            assert!(result.errors[0].suggestions.len() <= MAX_SUGGESTIONS);
        }
    }

    #[test]
    fn test_edit_distance_empty() {
        assert_eq!(SpellChecker::edit_distance("", "test"), 4);
        assert_eq!(SpellChecker::edit_distance("test", ""), 4);
        assert_eq!(SpellChecker::edit_distance("", ""), 0);
    }

    #[test]
    fn test_performance_large_text() {
        let checker = SpellChecker::new();
        let large_text = "the document text ".repeat(10_000);
        let result = checker.check_text(&large_text);
        assert_eq!(result.error_count, 0);
    }

    #[test]
    fn test_unicode_text() {
        let checker = SpellChecker::new();
        let result = checker.check_text("你好世界");
        // Unicode characters not in dictionary should be flagged
        assert!(result.total_words > 0);
    }

    #[test]
    fn test_mixed_case() {
        let checker = SpellChecker::new();
        let result = checker.check_text("The Document Text");
        assert_eq!(result.error_count, 0);
    }

    #[test]
    fn test_punctuation_ignored() {
        let checker = SpellChecker::new();
        let result = checker.check_text("the, document. text!");
        assert_eq!(result.error_count, 0);
    }

    #[test]
    fn test_service_getters() {
        assert_eq!(SpellChecker::max_text_length(), MAX_TEXT_LENGTH);
        assert_eq!(SpellChecker::max_word_length(), MAX_WORD_LENGTH);
        assert_eq!(SpellChecker::max_errors(), MAX_ERRORS);
        assert_eq!(SpellChecker::max_suggestions(), MAX_SUGGESTIONS);
        assert_eq!(SpellChecker::max_edit_distance(), MAX_EDIT_DISTANCE);
        assert_eq!(SpellChecker::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(SpellChecker::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_clear_dictionary() {
        let mut checker = SpellChecker::new();
        assert!(checker.dictionary_size() > 0);
        
        checker.clear_dictionary();
        assert_eq!(checker.dictionary_size(), 0);
    }

    #[test]
    fn test_remove_word() {
        let mut checker = SpellChecker::new();
        assert!(checker.has_word("document"));
        
        let removed = checker.remove_word("document");
        assert!(removed);
        assert!(!checker.has_word("document"));
    }

    #[test]
    fn test_remove_nonexistent_word() {
        let mut checker = SpellChecker::new();
        let removed = checker.remove_word("nonexistentword");
        assert!(!removed);
    }

    #[test]
    fn test_add_and_remove_word() {
        let mut checker = SpellChecker::new();
        checker.add_word("testword".to_string());
        assert!(checker.has_word("testword"));
        
        checker.remove_word("testword");
        assert!(!checker.has_word("testword"));
    }
}
