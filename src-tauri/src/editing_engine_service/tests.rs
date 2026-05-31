#[cfg(test)]
mod tests {
    use super::super::{DocumentOperations, FileManager, FormatConverter};
    use std::fs;

    #[test]
    fn test_word_count() {
        let text = "Hello world this is a test";
        assert_eq!(DocumentOperations::get_word_count(text), 6);
    }

    #[test]
    fn test_word_count_empty() {
        assert_eq!(DocumentOperations::get_word_count(""), 0);
    }

    #[test]
    fn test_char_count() {
        let text = "Hello";
        assert_eq!(DocumentOperations::get_char_count(text), 5);
    }

    #[test]
    fn test_paragraph_count() {
        let text = "Para 1\n\nPara 2\n\nPara 3";
        assert_eq!(DocumentOperations::get_paragraph_count(text), 3);
    }

    #[test]
    fn test_line_count() {
        let text = "Line 1\nLine 2\nLine 3";
        assert_eq!(DocumentOperations::get_line_count(text), 3);
    }

    #[test]
    fn test_reading_time() {
        let text = "word ".repeat(400);
        let time = DocumentOperations::get_reading_time(&text, 200);
        assert_eq!(time, 2);
    }

    #[test]
    fn test_extract_text_from_html() {
        let html = "<p>Hello <strong>world</strong></p>";
        let text = DocumentOperations::extract_text_from_html(html);
        assert!(text.contains("Hello"));
        assert!(text.contains("world"));
    }

    #[test]
    fn test_html_to_markdown() {
        let html = "<h1>Title</h1><p>Content</p>";
        let markdown = FormatConverter::html_to_markdown(html).unwrap();
        assert!(markdown.contains("# Title"));
        assert!(markdown.contains("Content"));
    }

    #[test]
    fn test_markdown_to_html() {
        let markdown = "# Title\n\nContent";
        let html = FormatConverter::markdown_to_html(markdown).unwrap();
        assert!(html.contains("<h1>") || html.contains("Title"));
    }

    #[test]
    fn test_plain_text_to_html() {
        let text = "Hello world";
        let html = FormatConverter::plain_text_to_html(text).unwrap();
        assert!(html.contains("Hello"));
        assert!(html.contains("<p>"));
    }

    #[test]
    fn test_html_validation_empty() {
        let result = DocumentOperations::validate_html("");
        assert!(result.is_err());
    }

    #[test]
    fn test_html_validation_valid() {
        let html = "<p>Hello</p>";
        let result = DocumentOperations::validate_html(html);
        assert!(result.is_ok());
    }

    #[test]
    fn test_file_save_and_load() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_logos_file.txt");
        let content = "Test content for file operations";

        let save_result = FileManager::save_file(test_file.to_str().unwrap(), content);
        assert!(save_result.is_ok());

        let load_result = FileManager::load_file(test_file.to_str().unwrap());
        assert!(load_result.is_ok());
        assert_eq!(load_result.unwrap(), content);

        // Cleanup
        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_file_exists() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_exists.txt");

        FileManager::save_file(test_file.to_str().unwrap(), "test").unwrap();
        assert!(FileManager::file_exists(test_file.to_str().unwrap()));

        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_file_size() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_size.txt");
        let content = "Hello world";

        FileManager::save_file(test_file.to_str().unwrap(), content).unwrap();
        let size = FileManager::get_file_size(test_file.to_str().unwrap()).unwrap();
        assert_eq!(size, 11);

        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_delete_file() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_delete.txt");

        FileManager::save_file(test_file.to_str().unwrap(), "test").unwrap();
        assert!(FileManager::file_exists(test_file.to_str().unwrap()));

        let delete_result = FileManager::delete_file(test_file.to_str().unwrap());
        assert!(delete_result.is_ok());
        assert!(!FileManager::file_exists(test_file.to_str().unwrap()));
    }
}
