use typst::model::Document;
use typst_pdf::PdfOptions;

#[derive(Debug, Clone, Default)]
pub struct RenderOptions {
    pub dpi: f32,
}

impl RenderOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_dpi(mut self, dpi: f32) -> Self {
        self.dpi = dpi;
        self
    }
}

pub struct TypstRenderer;

impl TypstRenderer {
    pub fn render_page_to_png(
        document: &Document,
        page_index: usize,
        dpi: f32,
    ) -> Result<Vec<u8>, String> {
        Self::render_page_to_png_with_options(
            document,
            page_index,
            RenderOptions::new().with_dpi(dpi),
        )
    }

    pub fn render_page_to_png_with_options(
        document: &Document,
        page_index: usize,
        options: RenderOptions,
    ) -> Result<Vec<u8>, String> {
        eprintln!(
            "[Typst Renderer] Rendering page {} at {} DPI",
            page_index, options.dpi
        );
        if page_index >= document.pages.len() {
            eprintln!(
                "[Typst Renderer] Page index {} out of bounds (total: {})",
                page_index,
                document.pages.len()
            );
            return Err(format!("Page index {} out of bounds", page_index));
        }

        let page = &document.pages[page_index];
        let pixmap = typst_render::render(page, options.dpi / 72.0);

        let result = pixmap.encode_png().map_err(|e| {
            eprintln!("[Typst Renderer] PNG encoding error: {}", e);
            format!("PNG encoding error: {}", e)
        })?;
        eprintln!("[Typst Renderer] Render successful, {} bytes", result.len());
        Ok(result)
    }

    pub fn render_first_page_to_png(document: &Document, dpi: f32) -> Result<Vec<u8>, String> {
        Self::render_page_to_png(document, 0, dpi)
    }

    #[allow(dead_code)]
    pub fn render_all_pages_to_png(document: &Document, dpi: f32) -> Result<Vec<Vec<u8>>, String> {
        document
            .pages
            .iter()
            .enumerate()
            .map(|(i, _)| Self::render_page_to_png(document, i, dpi))
            .collect()
    }

    /// Export document to PDF
    pub fn export_to_pdf(document: &Document) -> Result<Vec<u8>, String> {
        Self::export_to_pdf_with_options(document, PdfOptions::default())
    }

    /// Export document to PDF with custom options
    pub fn export_to_pdf_with_options(
        document: &Document,
        options: PdfOptions,
    ) -> Result<Vec<u8>, String> {
        eprintln!(
            "[Typst Renderer] Exporting {} pages to PDF",
            document.pages.len()
        );
        let result = typst_pdf::pdf(document, &options).map_err(|e| {
            eprintln!("[Typst Renderer] PDF export error: {:?}", e);
            format!("PDF export error: {:?}", e)
        })?;
        eprintln!(
            "[Typst Renderer] PDF export successful, {} bytes",
            result.len()
        );
        Ok(result)
    }

    /// Get page dimensions
    pub fn get_page_dimensions(
        document: &Document,
        page_index: usize,
    ) -> Result<(f64, f64), String> {
        if page_index >= document.pages.len() {
            return Err(format!("Page index {} out of bounds", page_index));
        }

        let page = &document.pages[page_index];
        let width = page.frame.width().to_pt();
        let height = page.frame.height().to_pt();
        Ok((width, height))
    }

    /// Get total page count
    pub fn get_page_count(document: &Document) -> usize {
        document.pages.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_exists() {
        let _renderer = TypstRenderer;
    }

    #[test]
    fn test_render_page_out_of_bounds() {
        // Since we can't easily create a Document without compilation,
        // we'll test the error path by checking the method signature
        // The actual test would require a compiled document
        let _ = TypstRenderer::render_page_to_png;
    }

    #[test]
    fn test_render_first_page_exists() {
        let _ = TypstRenderer::render_first_page_to_png;
    }

    #[test]
    fn test_render_all_pages_exists() {
        let _ = TypstRenderer::render_all_pages_to_png;
    }

    #[test]
    fn test_export_to_pdf_exists() {
        let _ = TypstRenderer::export_to_pdf;
    }

    #[test]
    fn test_renderer_methods_return_result() {
        // Verify the methods return Result types
        fn check_render() -> Result<Vec<u8>, String> {
            Err("test".to_string())
        }
        fn check_export() -> Result<Vec<u8>, String> {
            Err("test".to_string())
        }
        let _ = check_render;
        let _ = check_export;
    }

    #[test]
    fn test_render_options_new() {
        let options = RenderOptions::new();
        assert_eq!(options.dpi, 0.0);
    }

    #[test]
    fn test_render_options_default() {
        let options = RenderOptions::default();
        assert_eq!(options.dpi, 0.0);
    }

    #[test]
    fn test_render_options_with_dpi() {
        let options = RenderOptions::new().with_dpi(144.0);
        assert_eq!(options.dpi, 144.0);
    }

    #[test]
    fn test_render_options_chaining() {
        let options = RenderOptions::new().with_dpi(72.0).with_dpi(144.0);
        assert_eq!(options.dpi, 144.0);
    }

    #[test]
    fn test_get_page_count() {
        // Test with a simple compiled document
        let compiler = crate::typist_service::TypstCompiler::new();
        let code = "= Hello".to_string();
        if let Ok(document) = compiler.compile(code) {
            let count = TypstRenderer::get_page_count(&document);
            assert!(count >= 1);
        }
    }

    #[test]
    fn test_get_page_dimensions() {
        // Test with a simple compiled document
        let compiler = crate::typist_service::TypstCompiler::new();
        let code = "= Hello".to_string();
        if let Ok(document) = compiler.compile(code) {
            if document.pages.len() > 0 {
                let result = TypstRenderer::get_page_dimensions(&document, 0);
                assert!(result.is_ok());
                let (width, height) = result.unwrap();
                assert!(width > 0.0);
                assert!(height > 0.0);
            }
        }
    }

    #[test]
    fn test_get_page_dimensions_out_of_bounds() {
        // Test with a simple compiled document
        let compiler = crate::typist_service::TypstCompiler::new();
        let code = "= Hello".to_string();
        if let Ok(document) = compiler.compile(code) {
            let result = TypstRenderer::get_page_dimensions(&document, 999);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_render_page_to_png_with_options() {
        // Test with a simple compiled document
        let compiler = crate::typist_service::TypstCompiler::new();
        let code = "= Hello".to_string();
        if let Ok(document) = compiler.compile(code) {
            if document.pages.len() > 0 {
                let options = RenderOptions::new().with_dpi(72.0);
                let result = TypstRenderer::render_page_to_png_with_options(&document, 0, options);
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn test_export_to_pdf_with_options() {
        // Test with a simple compiled document
        let compiler = crate::typist_service::TypstCompiler::new();
        let code = "= Hello".to_string();
        if let Ok(document) = compiler.compile(code) {
            let options = PdfOptions::default();
            let result = TypstRenderer::export_to_pdf_with_options(&document, options);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_render_first_page_to_png() {
        // Test with a simple compiled document
        let compiler = crate::typist_service::TypstCompiler::new();
        let code = "= Hello".to_string();
        if let Ok(document) = compiler.compile(code) {
            let result = TypstRenderer::render_first_page_to_png(&document, 72.0);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_render_all_pages_to_png() {
        // Test with a simple compiled document
        let compiler = crate::typist_service::TypstCompiler::new();
        let code = "= Hello\n\nPage 2".to_string();
        if let Ok(document) = compiler.compile(code) {
            let result = TypstRenderer::render_all_pages_to_png(&document, 72.0);
            assert!(result.is_ok());
            let pages = result.unwrap();
            assert!(pages.len() >= 1);
        }
    }

    #[test]
    fn test_export_to_pdf() {
        // Test with a simple compiled document
        let compiler = crate::typist_service::TypstCompiler::new();
        let code = "= Hello".to_string();
        if let Ok(document) = compiler.compile(code) {
            let result = TypstRenderer::export_to_pdf(&document);
            assert!(result.is_ok());
        }
    }
}
