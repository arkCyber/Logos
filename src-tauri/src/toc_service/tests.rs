#[cfg(test)]
mod tests {
    use super::super::generator::{TocService, TocItem, InsertPosition, TocResult};

    #[test]
    fn test_toc_service_creation() {
        let service = TocService::new();
        // Service created successfully
    }

    #[test]
    fn test_generate_toc_single_heading() {
        let service = TocService::new();
        let html = "<h1>Main Title</h1>";
        let result = service.generate_toc(html);
        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].text, "Main Title");
        assert_eq!(result.items[0].level, 1);
    }

    #[test]
    fn test_generate_toc_multiple_headings() {
        let service = TocService::new();
        let html = "<h1>Title 1</h1><h2>Subtitle 1</h2><h1>Title 2</h1>";
        let result = service.generate_toc(html);
        assert_eq!(result.items.len(), 3); // Flat structure: all headings
    }

    #[test]
    fn test_generate_toc_nested_structure() {
        let service = TocService::new();
        let html = "<h1>Main</h1><h2>Section</h2><h3>Subsection</h3><h2>Another Section</h2>";
        let result = service.generate_toc(html);
        assert_eq!(result.items.len(), 4); // Flat structure: all headings
    }

    #[test]
    fn test_generate_toc_id_generation() {
        let service = TocService::new();
        let html = "<h1>Hello World!</h1>";
        let result = service.generate_toc(html);
        assert_eq!(result.items[0].id, "hello-world");
    }

    #[test]
    fn test_generate_toc_html_output() {
        let service = TocService::new();
        let html = "<h1>Title</h1>";
        let result = service.generate_toc(html);
        assert!(result.html.contains("<ul class=\"toc\">"));
        assert!(result.html.contains("<a href=\"#title\">Title</a>"));
    }

    #[test]
    fn test_insert_toc_at_beginning() {
        let service = TocService::new();
        let html = "<p>Content</p>";
        let toc = TocResult {
            items: vec![],
            html: "<div>TOC</div>".to_string(),
        };
        let result = service.insert_toc(html, &toc, InsertPosition::Beginning);
        assert!(result.starts_with("<div class=\"table-of-contents\">"));
    }

    #[test]
    fn test_insert_toc_at_end() {
        let service = TocService::new();
        let html = "<p>Content</p>";
        let toc = TocResult {
            items: vec![],
            html: "<div>TOC</div>".to_string(),
        };
        let result = service.insert_toc(html, &toc, InsertPosition::End);
        assert!(result.contains("<div class=\"table-of-contents\">"));
    }

    #[test]
    fn test_insert_toc_after_first_heading() {
        let service = TocService::new();
        let html = "<h1>Title</h1><p>Content</p>";
        let toc = TocResult {
            items: vec![],
            html: "<div>TOC</div>".to_string(),
        };
        let result = service.insert_toc(html, &toc, InsertPosition::AfterFirstHeading);
        assert!(result.contains("<div class=\"table-of-contents\">"));
    }

    #[test]
    fn test_generate_toc_empty_html() {
        let service = TocService::new();
        let html = "";
        let result = service.generate_toc(html);
        assert_eq!(result.items.len(), 0);
    }

    #[test]
    fn test_generate_toc_no_headings() {
        let service = TocService::new();
        let html = "<p>Just a paragraph</p><div>Some div</div>";
        let result = service.generate_toc(html);
        assert_eq!(result.items.len(), 0);
    }

    #[test]
    fn test_generate_toc_all_heading_levels() {
        let service = TocService::new();
        let html = "<h1>H1</h1><h2>H2</h2><h3>H3</h3><h4>H4</h4><h5>H5</h5><h6>H6</h6>";
        let result = service.generate_toc(html);
        assert_eq!(result.items.len(), 6); // Flat structure: all headings
    }

    #[test]
    fn test_generate_toc_special_characters() {
        let service = TocService::new();
        let html = "<h1>Title with @#$ special chars</h1>";
        let result = service.generate_toc(html);
        assert_eq!(result.items[0].text, "Title with @#$ special chars");
        assert!(!result.items[0].id.contains('@'));
        assert!(!result.items[0].id.contains('#'));
    }
}
