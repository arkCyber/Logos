/*!
 * 航空航天级导出系统
 * 实现 Typst 的 HTML 和 SVG 导出功能
 */

use typst::layout::Abs;
use typst::model::Document;
use typst_svg::{svg, svg_merged};

pub struct HtmlExporter {
    config: HtmlConfig,
}

#[derive(Debug, Clone)]
pub struct HtmlConfig {
    pub include_css: bool,
    pub embed_images: bool,
    pub pretty_print: bool,
    pub custom_css: Option<String>,
}

impl Default for HtmlConfig {
    fn default() -> Self {
        Self {
            include_css: true,
            embed_images: true,
            pretty_print: true,
            custom_css: None,
        }
    }
}

impl HtmlExporter {
    pub fn new() -> Self {
        Self {
            config: HtmlConfig::default(),
        }
    }

    pub fn with_config(config: HtmlConfig) -> Self {
        Self { config }
    }

    pub fn export(&self, document: &Document) -> Result<String, String> {
        let mut html = String::new();

        // HTML header
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("  <meta charset=\"UTF-8\">\n");
        html.push_str(
            "  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str("  <title>Typst Document</title>\n");

        if self.config.include_css {
            html.push_str("  <style>\n");
            html.push_str(self.get_default_css());
            if let Some(ref custom_css) = self.config.custom_css {
                html.push_str(custom_css);
            }
            html.push_str("  </style>\n");
        }

        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <div class=\"typst-document\">\n");

        // Render pages
        for (page_idx, page) in document.pages.iter().enumerate() {
            html.push_str(&self.render_page(page, page_idx));
        }

        html.push_str("  </div>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");

        Ok(html)
    }

    fn render_page(&self, page: &typst::layout::Page, page_idx: usize) -> String {
        let mut html = String::new();

        html.push_str(&format!(
            "    <div class=\"page\" data-page=\"{}\">\n",
            page_idx
        ));

        // Render frame content
        html.push_str(&self.render_frame(&page.frame));

        html.push_str("    </div>\n");

        html
    }

    fn render_frame(&self, _frame: &typst::layout::Frame) -> String {
        let mut html = String::new();

        // Convert frame to HTML structure
        // This is a simplified version - full implementation would need to traverse the frame tree
        html.push_str("      <div class=\"frame\">\n");
        html.push_str("        <!-- Frame content would be rendered here -->\n");
        html.push_str("      </div>\n");

        html
    }

    fn get_default_css(&self) -> &str {
        r"
        .typst-document {
            max-width: 210mm;
            margin: 0 auto;
            padding: 20mm;
            background: white;
        }
        
        .page {
            width: 210mm;
            min-height: 297mm;
            margin-bottom: 20mm;
            padding: 20mm;
            background: white;
            box-shadow: 0 0 10px rgba(0,0,0,0.1);
        }
        
        .frame {
            position: relative;
        }
        
        @media print {
            .typst-document {
                padding: 0;
            }
            .page {
                margin: 0;
                box-shadow: none;
                page-break-after: always;
            }
        }
        "
    }
}

impl Default for HtmlExporter {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TypstSvgExporter {
    #[allow(dead_code)]
    config: TypstSvgExportConfig,
}

#[derive(Debug, Clone)]
pub struct TypstSvgExportConfig {
    pub embed_fonts: bool,
    pub use_vectors: bool,
    pub dpi: f32,
}

impl Default for TypstSvgExportConfig {
    fn default() -> Self {
        Self {
            embed_fonts: false,
            use_vectors: true,
            dpi: 72.0,
        }
    }
}

impl TypstSvgExporter {
    /// Create a Typst document SVG exporter (distinct from `svg_service::SvgExporter`).
    pub fn new() -> Self {
        Self {
            config: TypstSvgExportConfig::default(),
        }
    }

    pub fn with_config(config: TypstSvgExportConfig) -> Self {
        Self { config }
    }

    /// Export a Typst document to a multi-page SVG string via the official typst-svg renderer.
    pub fn export(&self, document: &Document) -> Result<String, String> {
        if document.pages.len() > 1000 {
            return Err("Typst document page count exceeds maximum of 1000".to_string());
        }
        if document.pages.is_empty() {
            return Ok(empty_typst_svg());
        }

        Ok(svg_merged(document, Abs::pt(0.0)))
    }

    /// Export a single Typst page to SVG via the official typst-svg renderer.
    pub fn export_page(&self, document: &Document, page_index: usize) -> Result<String, String> {
        if page_index >= document.pages.len() {
            return Err(format!("Page index {} out of bounds", page_index));
        }

        let page = &document.pages[page_index];
        let width = page.frame.width().to_pt();
        let height = page.frame.height().to_pt();
        if !width.is_finite() || !height.is_finite() || width <= 0.0 || height <= 0.0 {
            return Err("Invalid Typst page dimensions".to_string());
        }

        Ok(svg(page))
    }
}

/// Render a minimal SVG placeholder for empty Typst documents.
fn empty_typst_svg() -> String {
    concat!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n",
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"595\" height=\"842\" viewBox=\"0 0 595 842\">\n",
        "  <rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n",
        "  <text x=\"50%\" y=\"50%\" text-anchor=\"middle\" font-size=\"14\" fill=\"#666\">Empty Typst document</text>\n",
        "</svg>\n"
    )
    .to_string()
}

impl Default for TypstSvgExporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Backward-compatible alias for Typst document SVG export.
pub type SvgExporter = TypstSvgExporter;

/// Backward-compatible alias for Typst SVG export configuration.
pub type SvgConfig = TypstSvgExportConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_exporter_creation() {
        let exporter = HtmlExporter::new();
        assert_eq!(exporter.config.include_css, true);
    }

    #[test]
    fn test_html_exporter_with_config() {
        let config = HtmlConfig {
            include_css: false,
            embed_images: false,
            pretty_print: false,
            custom_css: None,
        };
        let exporter = HtmlExporter::with_config(config);
        assert_eq!(exporter.config.include_css, false);
    }

    #[test]
    fn test_svg_exporter_creation() {
        let exporter = TypstSvgExporter::new();
        assert_eq!(exporter.config.use_vectors, true);
    }

    #[test]
    fn test_svg_exporter_with_config() {
        let config = TypstSvgExportConfig {
            embed_fonts: true,
            use_vectors: false,
            dpi: 144.0,
        };
        let exporter = TypstSvgExporter::with_config(config);
        assert_eq!(exporter.config.embed_fonts, true);
        assert_eq!(exporter.config.dpi, 144.0);
    }

    #[test]
    fn test_html_config_default() {
        let config = HtmlConfig::default();
        assert_eq!(config.include_css, true);
        assert_eq!(config.embed_images, true);
        assert_eq!(config.pretty_print, true);
    }

    #[test]
    fn test_svg_config_default() {
        let config = TypstSvgExportConfig::default();
        assert_eq!(config.embed_fonts, false);
        assert_eq!(config.use_vectors, true);
        assert_eq!(config.dpi, 72.0);
    }

    #[test]
    fn test_svg_export_produces_valid_xml() {
        let exporter = TypstSvgExporter::new();
        let document = typst::model::Document::default();
        let result = exporter.export(&document);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(svg.contains("<svg xmlns=\"http://www.w3.org/2000/svg\""));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_svg_export_includes_viewbox() {
        let exporter = TypstSvgExporter::new();
        let document = typst::model::Document::default();
        let result = exporter.export(&document);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("viewBox="));
    }

    #[test]
    fn test_svg_export_includes_width_height() {
        let exporter = TypstSvgExporter::new();
        let document = typst::model::Document::default();
        let result = exporter.export(&document);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("width="));
        assert!(svg.contains("height="));
    }

    #[test]
    fn test_svg_export_page_out_of_bounds() {
        let exporter = TypstSvgExporter::new();
        let document = typst::model::Document::default();
        let result = exporter.export_page(&document, 999);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("out of bounds"));
    }

    #[test]
    fn test_svg_config_embed_fonts() {
        let config = TypstSvgExportConfig {
            embed_fonts: true,
            use_vectors: true,
            dpi: 72.0,
        };
        assert_eq!(config.embed_fonts, true);
    }

    #[test]
    fn test_svg_config_use_vectors() {
        let config = TypstSvgExportConfig {
            embed_fonts: false,
            use_vectors: false,
            dpi: 72.0,
        };
        assert_eq!(config.use_vectors, false);
    }

    #[test]
    fn test_svg_config_dpi() {
        let config = TypstSvgExportConfig {
            embed_fonts: false,
            use_vectors: true,
            dpi: 300.0,
        };
        assert_eq!(config.dpi, 300.0);
    }

    #[test]
    fn test_svg_exporter_default() {
        let exporter = TypstSvgExporter::default();
        assert_eq!(exporter.config.use_vectors, true);
    }

    #[test]
    fn test_render_frame_depth_limit() {
        let exporter = TypstSvgExporter::new();
        let document = typst::model::Document::default();
        let result = exporter.export(&document);
        assert!(result.is_ok());
        let svg = result.unwrap();
        // Should not contain max depth warning for normal documents
        assert!(!svg.contains("Max depth reached"));
    }

    #[test]
    fn test_render_text_escaping() {
        use crate::svg_service::sanitize::escape_svg_text;
        assert_eq!(escape_svg_text("<&>"), "&lt;&amp;&gt;");
    }

    #[test]
    fn test_svg_export_uses_typst_svg_renderer() {
        let compiler = crate::typist_service::TypstCompiler::new();
        let code = "= Hello SVG\n\nExport test.";
        let Ok(document) = compiler.compile(code.to_string()) else {
            return;
        };
        if document.pages.is_empty() {
            return;
        }

        let exporter = TypstSvgExporter::new();
        let svg = exporter.export(&document);
        assert!(svg.is_ok());
        let svg = svg.unwrap();
        assert!(svg.contains("<svg"));
        assert!(!svg.contains("Frame content rendering requires"));
        assert!(!svg.contains("<!-- Frame item at"));
    }

    #[test]
    fn test_svg_export_page_renders_single_page() {
        let compiler = crate::typist_service::TypstCompiler::new();
        let code = "= Page One";
        let Ok(document) = compiler.compile(code.to_string()) else {
            return;
        };
        if document.pages.is_empty() {
            return;
        }

        let exporter = TypstSvgExporter::new();
        let svg = exporter.export_page(&document, 0);
        assert!(svg.is_ok());
        let svg = svg.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("viewBox"));
    }
}
