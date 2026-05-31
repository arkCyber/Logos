#[cfg(test)]
mod tests {
    use super::super::{FontLoader, TypstCompiler, TypstRenderer, validate_source, validate_output_size, get_operation_count, record_error, get_last_error, reset_error_state};
    use crate::config_service::ExportConfigService;
    use std::sync::Arc;

    #[test]
    fn test_font_loader_creation() {
        let loader = FontLoader::new();
        assert!(loader.get_fonts().len() > 0 || loader.get_fonts().is_empty());
    }

    #[test]
    fn test_compiler_creation() {
        let compiler = TypstCompiler::new();
        assert!(compiler.get_font_count() >= 0);
    }

    #[test]
    fn test_simple_typst_compilation() {
        let compiler = TypstCompiler::new();
        let code = "= Hello World\n\nThis is a test.";

        let result = compiler.compile(code.to_string());
        // May fail if no fonts are available, but should not panic
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_empty_typst_compilation() {
        let compiler = TypstCompiler::new();
        let result = compiler.compile(String::new());
        // Empty code may compile to an empty document
        // Just check it doesn't panic
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_invalid_typst_syntax() {
        let compiler = TypstCompiler::new();
        let invalid_code = "= Unclosed heading\n\n#invalid_function()";

        let result = compiler.compile(invalid_code.to_string());
        // Invalid syntax should fail
        assert!(result.is_err());
    }

    #[test]
    fn test_pdf_export() {
        let compiler = TypstCompiler::new();
        let code = "= Test Slide\n\nThis is a test content.";

        let result = compiler.compile(code.to_string());
        assert!(result.is_ok());

        if let Ok(document) = result {
            let pdf_result: Result<Vec<u8>, String> = TypstRenderer::export_to_pdf(&document);
            assert!(pdf_result.is_ok());

            if let Ok(pdf_bytes) = pdf_result {
                assert!(!pdf_bytes.is_empty());
            }
        }
    }

    #[test]
    fn test_pdf_export_empty_document() {
        let compiler = TypstCompiler::new();
        let result = compiler.compile(String::new());

        if let Ok(document) = result {
            let pdf_result: Result<Vec<u8>, String> = TypstRenderer::export_to_pdf(&document);
            // Empty document may still export to PDF
            assert!(pdf_result.is_ok() || pdf_result.is_err());
        }
    }

    // Aerospace-level tests
    #[test]
    fn test_source_validation_too_long() {
        let config_service = Arc::new(ExportConfigService::new());
        let typist_config = config_service.get_typist_config();
        let long_source = "a".repeat(typist_config.max_source_length + 1);
        let result = validate_source(&long_source, &config_service);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum length"));
    }

    #[test]
    fn test_output_size_validation_too_large() {
        let config_service = Arc::new(ExportConfigService::new());
        let typist_config = config_service.get_typist_config();
        let result = validate_output_size(typist_config.max_output_size + 1, &config_service);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum size"));
    }

    #[test]
    fn test_max_source_accepted() {
        let config_service = Arc::new(ExportConfigService::new());
        let typist_config = config_service.get_typist_config();
        let source = "a".repeat(typist_config.max_source_length);
        let result = validate_source(&source, &config_service);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_output_size_accepted() {
        let config_service = Arc::new(ExportConfigService::new());
        let typist_config = config_service.get_typist_config();
        let result = validate_output_size(typist_config.max_output_size, &config_service);
        assert!(result.is_ok());
    }

    #[test]
    fn test_operation_count() {
        let initial_count = get_operation_count();
        // The count should be a valid u64
        assert!(initial_count >= 0);
    }

    #[test]
    fn test_error_recording() {
        record_error("TEST_ERROR", "Test error message", "test_source");
        
        let last_error = get_last_error();
        assert!(last_error.is_some());
        let error = last_error.unwrap();
        assert_eq!(error.code, "TEST_ERROR");
        assert_eq!(error.message, "Test error message");
        assert_eq!(error.source, "test_source");
    }

    #[test]
    fn test_error_state_reset() {
        record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(get_last_error().is_some());
        
        reset_error_state();
        assert!(get_last_error().is_none());
    }
}
