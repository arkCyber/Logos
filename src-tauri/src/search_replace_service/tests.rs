#[cfg(test)]
mod tests {
    use super::super::searcher::{SearchReplaceService, SearchOptions, ReplaceOptions};

    #[test]
    fn test_search_replace_service_creation() {
        let service = SearchReplaceService::new();
        // Service created successfully
    }

    #[test]
    fn test_find_text_basic() {
        let service = SearchReplaceService::new();
        let options = SearchOptions::default();
        let result = service.find_text("hello world", "hello", &options, 0);
        assert_eq!(result.total_count, 1);
        assert_eq!(result.matches.len(), 1);
    }

    #[test]
    fn test_find_text_multiple() {
        let service = SearchReplaceService::new();
        let options = SearchOptions::default();
        let result = service.find_text("hello hello hello", "hello", &options, 0);
        assert_eq!(result.total_count, 3);
    }

    #[test]
    fn test_find_text_case_sensitive() {
        let service = SearchReplaceService::new();
        let options = SearchOptions {
            case_sensitive: true,
            ..Default::default()
        };
        let result = service.find_text("Hello hello", "hello", &options, 0);
        assert_eq!(result.total_count, 1);
    }

    #[test]
    fn test_find_text_case_insensitive() {
        let service = SearchReplaceService::new();
        let options = SearchOptions {
            case_sensitive: false,
            ..Default::default()
        };
        let result = service.find_text("Hello hello", "hello", &options, 0);
        assert_eq!(result.total_count, 2);
    }

    #[test]
    fn test_find_text_whole_word() {
        let service = SearchReplaceService::new();
        let options = SearchOptions {
            whole_word: true,
            ..Default::default()
        };
        let result = service.find_text("hello helloworld", "hello", &options, 0);
        assert_eq!(result.total_count, 1);
    }

    #[test]
    fn test_find_text_regex() {
        let service = SearchReplaceService::new();
        let options = SearchOptions {
            use_regex: true,
            ..Default::default()
        };
        let result = service.find_text("abc123 def456", r"\d+", &options, 0);
        assert_eq!(result.total_count, 2);
    }

    #[test]
    fn test_replace_text_all() {
        let service = SearchReplaceService::new();
        let options = ReplaceOptions {
            replace_all: true,
            ..Default::default()
        };
        let result = service.replace_text("hello world hello", "hello", "hi", &options);
        assert_eq!(result.replaced_count, 2);
        assert_eq!(result.new_text, "hi world hi");
        assert!(result.success);
    }

    #[test]
    fn test_replace_text_first() {
        let service = SearchReplaceService::new();
        let options = ReplaceOptions {
            replace_all: false,
            ..Default::default()
        };
        let result = service.replace_text("hello world hello", "hello", "hi", &options);
        assert_eq!(result.replaced_count, 1);
        assert_eq!(result.new_text, "hi world hello");
        assert!(result.success);
    }

    #[test]
    fn test_replace_text_regex() {
        let service = SearchReplaceService::new();
        let options = ReplaceOptions {
            use_regex: true,
            replace_all: true,
            ..Default::default()
        };
        let result = service.replace_text("abc123 def456", r"\d+", "X", &options);
        assert_eq!(result.replaced_count, 2);
        assert_eq!(result.new_text, "abcX defX");
    }

    #[test]
    fn test_find_text_empty_pattern() {
        let service = SearchReplaceService::new();
        let options = SearchOptions::default();
        let result = service.find_text("hello world", "", &options, 0);
        assert_eq!(result.total_count, 0);
    }

    #[test]
    fn test_find_text_no_matches() {
        let service = SearchReplaceService::new();
        let options = SearchOptions::default();
        let result = service.find_text("hello world", "xyz", &options, 0);
        assert_eq!(result.total_count, 0);
    }

    #[test]
    fn test_replace_text_empty_pattern() {
        let service = SearchReplaceService::new();
        let options = ReplaceOptions::default();
        let result = service.replace_text("hello world", "", "hi", &options);
        assert_eq!(result.replaced_count, 0);
        assert_eq!(result.new_text, "hello world");
    }

    #[test]
    fn test_find_text_with_start_position() {
        let service = SearchReplaceService::new();
        let options = SearchOptions::default();
        let result = service.find_text("hello world hello", "hello", &options, 6);
        assert_eq!(result.total_count, 2);
        assert_eq!(result.current_index, 1);
    }
}
