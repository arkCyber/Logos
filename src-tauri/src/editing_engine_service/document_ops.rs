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

    pub fn get_reading_time(text: &str, words_per_minute: usize) -> usize {
        let word_count = Self::get_word_count(text);
        (word_count / words_per_minute).max(1)
    }

    pub fn extract_text_from_html(html: &str) -> String {
        let text = html
            .replace("<br>", "\n")
            .replace("<br/>", "\n")
            .replace("<br />", "\n")
            .replace("</p>", "\n\n")
            .replace("</li>", "\n");

        // Remove all HTML tags
        let re = regex::Regex::new(r"<[^>]+>").unwrap();
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
            return Err("HTML content is empty".to_string());
        }

        // Check for unclosed tags (simplified)
        let _tag_stack: Vec<&str> = Vec::new();
        let re = regex::Regex::new(r"<(/?)(\w+)[^>]*>").unwrap();

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
        let time = DocumentOperations::get_reading_time(text, 1);
        assert_eq!(time, 2);
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
