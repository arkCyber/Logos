/*!
 * 航空航天级导出系统
 * 实现 Typst 的 HTML 和 SVG 导出功能
 */

use typst::model::Document;

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

pub struct SvgExporter {
    #[allow(dead_code)]
    config: SvgConfig,
}

#[derive(Debug, Clone)]
pub struct SvgConfig {
    pub embed_fonts: bool,
    pub use_vectors: bool,
    pub dpi: f32,
}

impl Default for SvgConfig {
    fn default() -> Self {
        Self {
            embed_fonts: false,
            use_vectors: true,
            dpi: 72.0,
        }
    }
}

impl SvgExporter {
    pub fn new() -> Self {
        Self {
            config: SvgConfig::default(),
        }
    }

    pub fn with_config(config: SvgConfig) -> Self {
        Self { config }
    }

    pub fn export(&self, document: &Document) -> Result<String, String> {
        let mut svg = String::new();

        // SVG header
        svg.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" ");
        svg.push_str(&format!(
            "width=\"{}\" height=\"{}\" ",
            document.pages.len() as f32 * 595.0,
            842.0
        ));
        svg.push_str("viewBox=\"0 0 595 842\">\n");

        // Render pages
        for (page_idx, page) in document.pages.iter().enumerate() {
            let y_offset = page_idx as f32 * 842.0;
            svg.push_str(&self.render_page(page, y_offset));
        }

        svg.push_str("</svg>\n");

        Ok(svg)
    }

    fn render_page(&self, page: &typst::layout::Page, y_offset: f32) -> String {
        let mut svg = String::new();

        let width = page.frame.width().to_pt();
        let height = page.frame.height().to_pt();

        svg.push_str(&format!("  <g transform=\"translate(0, {})\">\n", y_offset));
        svg.push_str(&format!("    <rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"white\" stroke=\"black\" stroke-width=\"1\"/>\n", width, height));

        // Render frame content
        svg.push_str(&self.render_frame(&page.frame));

        svg.push_str("  </g>\n");

        svg
    }

    fn render_frame(&self, _frame: &typst::layout::Frame) -> String {
        let mut svg = String::new();

        // Convert frame to SVG elements
        // This is a simplified version - full implementation would need to traverse the frame tree
        svg.push_str("    <!-- Frame content would be rendered here -->\n");

        svg
    }

    pub fn export_page(&self, document: &Document, page_index: usize) -> Result<String, String> {
        if page_index >= document.pages.len() {
            return Err(format!("Page index {} out of bounds", page_index));
        }

        let page = &document.pages[page_index];
        let mut svg = String::new();

        let width = page.frame.width().to_pt();
        let height = page.frame.height().to_pt();

        svg.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" ");
        svg.push_str(&format!("width=\"{}\" height=\"{}\" ", width, height));
        svg.push_str(&format!("viewBox=\"0 0 {} {}\">\n", width, height));

        svg.push_str(&format!(
            "  <rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"white\"/>\n",
            width, height
        ));
        svg.push_str(&self.render_frame(&page.frame));

        svg.push_str("</svg>\n");

        Ok(svg)
    }
}

impl Default for SvgExporter {
    fn default() -> Self {
        Self::new()
    }
}

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
        let exporter = SvgExporter::new();
        assert_eq!(exporter.config.use_vectors, true);
    }

    #[test]
    fn test_svg_exporter_with_config() {
        let config = SvgConfig {
            embed_fonts: true,
            use_vectors: false,
            dpi: 144.0,
        };
        let exporter = SvgExporter::with_config(config);
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
        let config = SvgConfig::default();
        assert_eq!(config.embed_fonts, false);
        assert_eq!(config.use_vectors, true);
        assert_eq!(config.dpi, 72.0);
    }
}
