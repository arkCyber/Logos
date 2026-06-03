#[cfg(test)]
mod tests {
    use super::super::checker::{SpellChecker, SpellCheckResult};

    #[test]
    fn test_spell_checker_basic() {
        let checker = SpellChecker::new();
        let result = checker.check_text("the document text");
        assert_eq!(result.error_count, 0);
        assert_eq!(result.total_words, 3);
    }

    #[test]
    fn test_spell_checker_with_errors() {
        let checker = SpellChecker::new();
        let result = checker.check_text("the documnt is savd");
        assert!(result.error_count > 0);
    }

    #[test]
    fn test_spell_checker_empty() {
        let checker = SpellChecker::new();
        let result = checker.check_text("");
        assert_eq!(result.total_words, 0);
        assert_eq!(result.error_count, 0);
    }

    #[test]
    fn test_spell_checker_numbers() {
        let checker = SpellChecker::new();
        let result = checker.check_text("123 456 789");
        assert_eq!(result.error_count, 0);
    }
}
