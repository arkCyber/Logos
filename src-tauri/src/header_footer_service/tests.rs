#[cfg(test)]
mod tests {
    use super::super::processor::{HeaderFooterService, HeaderConfig, FooterConfig, PageNumberConfig};

    #[test]
    fn test_header_footer_service_creation() {
        let service = HeaderFooterService::new();
        // Service created successfully
    }

    #[test]
    fn test_apply_header_only() {
        let service = HeaderFooterService::new();
        let html = "<p>Content</p>";
        let header = HeaderConfig {
            enabled: true,
            content: "Test Header".to_string(),
            align: "center".to_string(),
            different_first_page: false,
        };
        let footer = FooterConfig::default();
        let result = service.apply_header_footer(html, &header, &footer);
        assert!(result.contains("Test Header"));
        assert!(result.contains("document-header"));
        assert!(!result.contains("document-footer"));
    }

    #[test]
    fn test_apply_footer_only() {
        let service = HeaderFooterService::new();
        let html = "<p>Content</p>";
        let header = HeaderConfig::default();
        let footer = FooterConfig {
            enabled: true,
            content: "Test Footer".to_string(),
            align: "center".to_string(),
            different_first_page: false,
        };
        let result = service.apply_header_footer(html, &header, &footer);
        assert!(result.contains("Test Footer"));
        assert!(result.contains("document-footer"));
        assert!(!result.contains("document-header"));
    }

    #[test]
    fn test_apply_both_header_and_footer() {
        let service = HeaderFooterService::new();
        let html = "<p>Content</p>";
        let header = HeaderConfig {
            enabled: true,
            content: "Header".to_string(),
            align: "left".to_string(),
            different_first_page: false,
        };
        let footer = FooterConfig {
            enabled: true,
            content: "Footer".to_string(),
            align: "right".to_string(),
            different_first_page: false,
        };
        let result = service.apply_header_footer(html, &header, &footer);
        assert!(result.contains("Header"));
        assert!(result.contains("Footer"));
        assert!(result.contains("document-header"));
        assert!(result.contains("document-footer"));
    }

    #[test]
    fn test_remove_existing_header() {
        let service = HeaderFooterService::new();
        let html = r#"<div class="document-header">Old Header</div><p>Content</p>"#;
        let result = service.remove_header_footer(html);
        assert!(!result.contains("document-header"));
        assert!(!result.contains("Old Header"));
    }

    #[test]
    fn test_remove_existing_footer() {
        let service = HeaderFooterService::new();
        let html = r#"<p>Content</p><div class="document-footer">Old Footer</div>"#;
        let result = service.remove_header_footer(html);
        assert!(!result.contains("document-footer"));
        assert!(!result.contains("Old Footer"));
    }

    #[test]
    fn test_remove_both_header_and_footer() {
        let service = HeaderFooterService::new();
        let html = r#"<div class="document-header">Header</div><p>Content</p><div class="document-footer">Footer</div>"#;
        let result = service.remove_header_footer(html);
        assert!(!result.contains("document-header"));
        assert!(!result.contains("document-footer"));
        assert!(result.contains("Content"));
    }

    #[test]
    fn test_apply_page_numbers_footer() {
        let service = HeaderFooterService::new();
        let html = "<p>Content</p>";
        let config = PageNumberConfig {
            enabled: true,
            position: "footer".to_string(),
            align: "center".to_string(),
            format: "1".to_string(),
        };
        let result = service.apply_page_numbers(html, &config);
        assert!(result.contains("page-number"));
        assert!(result.contains("Page"));
    }

    #[test]
    fn test_apply_page_numbers_header() {
        let service = HeaderFooterService::new();
        let html = "<p>Content</p>";
        let config = PageNumberConfig {
            enabled: true,
            position: "header".to_string(),
            align: "left".to_string(),
            format: "1".to_string(),
        };
        let result = service.apply_page_numbers(html, &config);
        assert!(result.contains("page-number"));
        assert!(result.starts_with("<div class=\"page-number\""));
    }

    #[test]
    fn test_page_numbers_disabled() {
        let service = HeaderFooterService::new();
        let html = "<p>Content</p>";
        let config = PageNumberConfig {
            enabled: false,
            position: "footer".to_string(),
            align: "center".to_string(),
            format: "1".to_string(),
        };
        let result = service.apply_page_numbers(html, &config);
        assert!(!result.contains("page-number"));
    }

    #[test]
    fn test_header_alignment_left() {
        let service = HeaderFooterService::new();
        let html = "<p>Content</p>";
        let header = HeaderConfig {
            enabled: true,
            content: "Test".to_string(),
            align: "left".to_string(),
            different_first_page: false,
        };
        let result = service.apply_header_footer(html, &header, &FooterConfig::default());
        assert!(result.contains("text-align: left"));
    }

    #[test]
    fn test_header_alignment_right() {
        let service = HeaderFooterService::new();
        let html = "<p>Content</p>";
        let header = HeaderConfig {
            enabled: true,
            content: "Test".to_string(),
            align: "right".to_string(),
            different_first_page: false,
        };
        let result = service.apply_header_footer(html, &header, &FooterConfig::default());
        assert!(result.contains("text-align: right"));
    }

    #[test]
    fn test_different_first_page() {
        let service = HeaderFooterService::new();
        let html = "<p>Content</p>";
        let header = HeaderConfig {
            enabled: true,
            content: "Test".to_string(),
            align: "center".to_string(),
            different_first_page: true,
        };
        let result = service.apply_header_footer(html, &header, &FooterConfig::default());
        assert!(result.contains("data-first-page=\"true\""));
    }

    #[test]
    fn test_empty_config_no_change() {
        let service = HeaderFooterService::new();
        let html = "<p>Content</p>";
        let result = service.apply_header_footer(html, &HeaderConfig::default(), &FooterConfig::default());
        assert_eq!(result, html);
    }
}
