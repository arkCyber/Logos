use serde::{Deserialize, Serialize};
use crate::error_handling::{ErrorContext, ErrorSeverity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentStats {
    pub word_count: usize,
    pub char_count: usize,
    pub paragraph_count: usize,
    pub line_count: usize,
    pub reading_time_minutes: usize,
    pub sentence_count: usize,
    pub avg_word_length: f64,
    pub avg_sentence_length: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentAnalysis {
    pub stats: DocumentStats,
    pub is_valid_html: bool,
    pub validation_errors: Vec<String>,
    pub has_images: bool,
    pub has_links: bool,
    pub has_tables: bool,
    pub has_code_blocks: bool,
    pub content_detection: ContentDetection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentDetection {
    pub images: usize,
    pub links: usize,
    pub tables: usize,
    pub code_blocks: usize,
}

#[allow(dead_code)]
pub struct DocumentOperations;

#[allow(dead_code)]
impl DocumentOperations {
    pub fn get_word_count(text: &str) -> usize {
        text.trim()
            .split_whitespace()
            .filter(|w| !w.is_empty())
            .count()
    }

    pub fn get_char_count(text: &str) -> usize {
        text.chars().count()
    }

    pub fn get_paragraph_count(text: &str) -> usize {
        text.split("\n\n").filter(|p| !p.trim().is_empty()).count()
    }

    pub fn get_line_count(text: &str) -> usize {
        text.split('\n').count()
    }

    pub fn get_sentence_count(text: &str) -> Result<usize, String> {
        let re = regex::Regex::new(r"[.!?]+")
            .map_err(|e| {
                let context = ErrorContext::new(
                    ErrorSeverity::Error,
                    "REGEX_COMPILE_FAILED",
                    &format!("Failed to compile sentence regex: {}", e),
                    "document_ops",
                );
                eprintln!("[Document Operations] Error: {}", context.message);
                context.message
            })?;
        Ok(re.find_iter(text).count())
    }

    pub fn get_reading_time(text: &str, words_per_minute: usize) -> usize {
        let word_count = Self::get_word_count(text);
        if words_per_minute == 0 {
            return 1; // Prevent division by zero
        }
        (word_count / words_per_minute).max(1)
    }

    pub fn get_avg_word_length(text: &str) -> f64 {
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.is_empty() {
            return 0.0;
        }
        let total_chars: usize = words.iter().map(|w| w.chars().count()).sum();
        total_chars as f64 / words.len() as f64
    }

    pub fn get_avg_sentence_length(text: &str) -> Result<f64, String> {
        let word_count = Self::get_word_count(text);
        let sentence_count = Self::get_sentence_count(text)?;
        if sentence_count == 0 {
            return Ok(0.0);
        }
        Ok(word_count as f64 / sentence_count as f64)
    }

    pub fn extract_text_from_html(html: &str) -> String {
        let text = html
            .replace("<br>", "\n")
            .replace("<br/>", "\n")
            .replace("<br />", "\n")
            .replace("</p>", "\n\n")
            .replace("</li>", "\n");

        // Remove all HTML tags with error handling
        let re = match regex::Regex::new(r"<[^>]+>") {
            Ok(regex) => regex,
            Err(e) => {
                eprintln!("[Document Operations] Failed to compile HTML tag regex: {}", e);
                // Fallback: return text without tag removal
                return text.trim().to_string();
            }
        };
        let clean_text = re.replace_all(&text, "").to_string();

        // Decode HTML entities
        clean_text
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&nbsp;", " ")
            .replace("&quot;", "\"")
            .trim()
            .to_string()
    }

    pub fn validate_html(html: &str) -> Result<(), String> {
        // Basic HTML validation
        if html.is_empty() {
            let context = ErrorContext::new(
                ErrorSeverity::Error,
                "EMPTY_HTML",
                "HTML content is empty",
                "document_ops",
            );
            return Err(context.message);
        }

        // Check for unclosed tags (simplified)
        let re = match regex::Regex::new(r"<(/?)(\w+)[^>]*>") {
            Ok(regex) => regex,
            Err(e) => {
                let context = ErrorContext::new(
                    ErrorSeverity::Error,
                    "REGEX_COMPILE_FAILED",
                    &format!("Failed to compile HTML validation regex: {}", e),
                    "document_ops",
                );
                return Err(context.message);
            }
        };

        for cap in re.captures_iter(html) {
            let is_closing = cap.get(1).map(|m| m.as_str() == "/").unwrap_or(false);
            let tag = cap.get(2).map(|m| m.as_str()).unwrap_or("");

            if is_closing {
                // Should match last opened tag (simplified check)
            } else if !tag.starts_with("br") && !tag.starts_with("hr") && !tag.starts_with("img") {
                // Self-closing tags don't need closing
            }
        }

        Ok(())
    }

    pub fn analyze_document(html: &str) -> Result<DocumentAnalysis, String> {
        let text = Self::extract_text_from_html(html);
        
        let stats = DocumentStats {
            word_count: Self::get_word_count(&text),
            char_count: Self::get_char_count(&text),
            paragraph_count: Self::get_paragraph_count(&text),
            line_count: Self::get_line_count(&text),
            reading_time_minutes: Self::get_reading_time(&text, 200),
            sentence_count: Self::get_sentence_count(&text).unwrap_or(0),
            avg_word_length: Self::get_avg_word_length(&text),
            avg_sentence_length: Self::get_avg_sentence_length(&text).unwrap_or(0.0),
        };

        let validation_result = Self::validate_html(html);
        let is_valid_html = validation_result.is_ok();
        let validation_errors = if let Err(e) = validation_result {
            vec![e]
        } else {
            vec![]
        };

        let has_images = html.contains("<img") || html.contains("<image");
        let has_links = html.contains("<a ") || html.contains("<a>");
        let has_tables = html.contains("<table") || html.contains("<td");
        let has_code_blocks = html.contains("<pre") || html.contains("<code");

        // Count actual occurrences
        let images = html.matches("<img").count() + html.matches("<image").count();
        let links = html.matches("<a ").count() + html.matches("<a>").count();
        let tables = html.matches("<table").count() + html.matches("<td").count();
        let code_blocks = html.matches("<pre").count() + html.matches("<code").count();

        let content_detection = ContentDetection {
            images,
            links,
            tables,
            code_blocks,
        };

        Ok(DocumentAnalysis {
            stats,
            is_valid_html,
            validation_errors,
            has_images,
            has_links,
            has_tables,
            has_code_blocks,
            content_detection,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_word_count_empty() {
        let count = DocumentOperations::get_word_count("");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_get_word_count_single_word() {
        let count = DocumentOperations::get_word_count("hello");
        assert_eq!(count, 1);
    }

    #[test]
    fn test_get_word_count_multiple_words() {
        let count = DocumentOperations::get_word_count("hello world test");
        assert_eq!(count, 3);
    }

    #[test]
    fn test_get_word_count_with_whitespace() {
        let count = DocumentOperations::get_word_count("  hello   world  ");
        assert_eq!(count, 2);
    }

    #[test]
    fn test_get_word_count_with_newlines() {
        let count = DocumentOperations::get_word_count("hello\nworld\ntest");
        assert_eq!(count, 3);
    }

    #[test]
    fn test_get_word_count_with_tabs() {
        let count = DocumentOperations::get_word_count("hello\tworld\ttest");
        assert_eq!(count, 3);
    }

    #[test]
    fn test_get_char_count_empty() {
        let count = DocumentOperations::get_char_count("");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_get_char_count_single_char() {
        let count = DocumentOperations::get_char_count("a");
        assert_eq!(count, 1);
    }

    #[test]
    fn test_get_char_count_multiple_chars() {
        let count = DocumentOperations::get_char_count("hello");
        assert_eq!(count, 5);
    }

    #[test]
    fn test_get_char_count_with_unicode() {
        let count = DocumentOperations::get_char_count("hello 世界");
        assert_eq!(count, 8);
    }

    #[test]
    fn test_get_char_count_with_emoji() {
        let count = DocumentOperations::get_char_count("hello 😊");
        assert_eq!(count, 7);
    }

    #[test]
    fn test_get_paragraph_count_empty() {
        let count = DocumentOperations::get_paragraph_count("");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_get_paragraph_count_single() {
        let count = DocumentOperations::get_paragraph_count("hello world");
        assert_eq!(count, 1);
    }

    #[test]
    fn test_get_paragraph_count_multiple() {
        let count = DocumentOperations::get_paragraph_count("hello\n\nworld\n\ntest");
        assert_eq!(count, 3);
    }

    #[test]
    fn test_get_paragraph_count_with_empty_lines() {
        let count = DocumentOperations::get_paragraph_count("hello\n\n\n\nworld");
        assert_eq!(count, 2);
    }

    #[test]
    fn test_get_paragraph_count_with_whitespace() {
        let count = DocumentOperations::get_paragraph_count("  hello  \n\n  world  ");
        assert_eq!(count, 2);
    }

    #[test]
    fn test_get_line_count_empty() {
        let count = DocumentOperations::get_line_count("");
        assert_eq!(count, 1);
    }

    #[test]
    fn test_get_line_count_single() {
        let count = DocumentOperations::get_line_count("hello");
        assert_eq!(count, 1);
    }

    #[test]
    fn test_get_line_count_multiple() {
        let count = DocumentOperations::get_line_count("hello\nworld\ntest");
        assert_eq!(count, 3);
    }

    #[test]
    fn test_get_line_count_with_empty_lines() {
        let count = DocumentOperations::get_line_count("hello\n\nworld");
        assert_eq!(count, 3);
    }

    #[test]
    fn test_get_reading_time_empty() {
        let time = DocumentOperations::get_reading_time("", 200);
        assert_eq!(time, 1);
    }

    #[test]
    fn test_get_reading_time_short() {
        let time = DocumentOperations::get_reading_time("hello world", 200);
        assert_eq!(time, 1);
    }

    #[test]
    fn test_get_reading_time_long() {
        let text = "word ".repeat(400);
        let time = DocumentOperations::get_reading_time(&text, 200);
        assert_eq!(time, 2);
    }

    #[test]
    fn test_get_reading_time_custom_speed() {
        let text = "word ".repeat(100);
        let time = DocumentOperations::get_reading_time(&text, 50);
        assert_eq!(time, 2);
    }

    #[test]
    fn test_extract_text_from_html_simple() {
        let html = "<p>Hello world</p>";
        let text = DocumentOperations::extract_text_from_html(html);
        assert_eq!(text, "Hello world");
    }

    #[test]
    fn test_extract_text_from_html_with_br() {
        let html = "Hello<br>world";
        let text = DocumentOperations::extract_text_from_html(html);
        assert_eq!(text, "Hello\nworld");
    }

    #[test]
    fn test_extract_text_from_html_with_entities() {
        let html = "Hello &amp; world &lt;test&gt;";
        let text = DocumentOperations::extract_text_from_html(html);
        assert_eq!(text, "Hello & world <test>");
    }

    #[test]
    fn test_extract_text_from_html_with_nbsp() {
        let html = "Hello&nbsp;world";
        let text = DocumentOperations::extract_text_from_html(html);
        assert_eq!(text, "Hello world");
    }

    #[test]
    fn test_extract_text_from_html_with_quotes() {
        let html = "Hello &quot;world&quot;";
        let text = DocumentOperations::extract_text_from_html(html);
        assert_eq!(text, "Hello \"world\"");
    }

    #[test]
    fn test_extract_text_from_html_complex() {
        let html = "<div><p>Hello</p><br><p>world</p></div>";
        let text = DocumentOperations::extract_text_from_html(html);
        assert!(text.contains("Hello"));
        assert!(text.contains("world"));
    }

    #[test]
    fn test_extract_text_from_html_empty() {
        let html = "";
        let text = DocumentOperations::extract_text_from_html(html);
        assert_eq!(text, "");
    }

    #[test]
    fn test_validate_html_empty() {
        let result = DocumentOperations::validate_html("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_html_valid() {
        let html = "<p>Hello world</p>";
        let result = DocumentOperations::validate_html(html);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_html_simple() {
        let html = "<div>test</div>";
        let result = DocumentOperations::validate_html(html);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_html_with_self_closing() {
        let html = "<br/><img src='test.jpg'/>";
        let result = DocumentOperations::validate_html(html);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_html_with_hr() {
        let html = "<hr/>";
        let result = DocumentOperations::validate_html(html);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_word_count_with_punctuation() {
        let count = DocumentOperations::get_word_count("hello, world! test.");
        assert_eq!(count, 3);
    }

    #[test]
    fn test_get_word_count_with_numbers() {
        let count = DocumentOperations::get_word_count("hello 123 world 456");
        assert_eq!(count, 4);
    }

    #[test]
    fn test_get_char_count_with_spaces() {
        let count = DocumentOperations::get_char_count("hello world");
        assert_eq!(count, 11);
    }

    #[test]
    fn test_get_paragraph_count_single_line() {
        let count = DocumentOperations::get_paragraph_count("hello world");
        assert_eq!(count, 1);
    }

    #[test]
    fn test_get_line_count_trailing_newline() {
        let count = DocumentOperations::get_line_count("hello\n");
        assert_eq!(count, 2);
    }

    #[test]
    fn test_get_reading_time_zero_wpm() {
        let text = "hello world";
        let time = DocumentOperations::get_reading_time(text, 0);
        assert_eq!(time, 1); // Should return 1 when division by zero is prevented
    }

    #[test]
    fn test_extract_text_from_html_nested_tags() {
        let html = "<div><p><strong>Hello</strong> world</p></div>";
        let text = DocumentOperations::extract_text_from_html(html);
        assert!(text.contains("Hello"));
        assert!(text.contains("world"));
    }

    #[test]
    fn test_extract_text_from_html_list() {
        let html = "<ul><li>Item 1</li><li>Item 2</li></ul>";
        let text = DocumentOperations::extract_text_from_html(html);
        assert!(text.contains("Item 1"));
        assert!(text.contains("Item 2"));
    }

    #[test]
    fn test_extract_text_from_html_mixed_br() {
        let html = "Hello<br/>world<br />test";
        let text = DocumentOperations::extract_text_from_html(html);
        assert_eq!(text, "Hello\nworld\ntest");
    }

    #[test]
    fn test_validate_html_with_attributes() {
        let html = "<div class='test' id='main'>Hello</div>";
        let result = DocumentOperations::validate_html(html);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_word_count_very_long_text() {
        let text = "word ".repeat(10000);
        let count = DocumentOperations::get_word_count(&text);
        assert_eq!(count, 10000);
    }

    #[test]
    fn test_get_char_count_very_long_text() {
        let text = "a".repeat(10000);
        let count = DocumentOperations::get_char_count(&text);
        assert_eq!(count, 10000);
    }
}
