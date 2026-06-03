use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Maximum content length for header/footer to prevent memory issues
const MAX_CONTENT_LENGTH: usize = 5000;

/// Valid alignment options
const VALID_ALIGNMENTS: [&str; 3] = ["left", "center", "right"];

/// Valid page number positions
const VALID_POSITIONS: [&str; 2] = ["header", "footer"];

/// Valid page number formats
const VALID_FORMATS: [&str; 5] = ["1", "I", "i", "A", "a"];

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 500;

/// Configuration for document header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderConfig {
    /// Whether the header is enabled
    pub enabled: bool,
    /// Header content text
    pub content: String,
    /// Text alignment: "left", "center", or "right"
    pub align: String,
    /// Whether to use different header on first page
    pub different_first_page: bool,
}

impl Default for HeaderConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            content: String::new(),
            align: "center".to_string(),
            different_first_page: false,
        }
    }
}

impl HeaderConfig {
    /// Validates the header configuration
    pub fn validate(&self) -> Result<(), String> {
        if !VALID_ALIGNMENTS.contains(&self.align.as_str()) {
            return Err(format!("Invalid alignment: {}. Must be one of: left, center, right", self.align));
        }
        if self.content.len() > MAX_CONTENT_LENGTH {
            return Err(format!("Content exceeds maximum length of {} characters", MAX_CONTENT_LENGTH));
        }
        Ok(())
    }
}

/// Configuration for document footer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooterConfig {
    /// Whether the footer is enabled
    pub enabled: bool,
    /// Footer content text
    pub content: String,
    /// Text alignment: "left", "center", or "right"
    pub align: String,
    /// Whether to use different footer on first page
    pub different_first_page: bool,
}

impl Default for FooterConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            content: String::new(),
            align: "center".to_string(),
            different_first_page: false,
        }
    }
}

impl FooterConfig {
    /// Validates the footer configuration
    pub fn validate(&self) -> Result<(), String> {
        if !VALID_ALIGNMENTS.contains(&self.align.as_str()) {
            return Err(format!("Invalid alignment: {}. Must be one of: left, center, right", self.align));
        }
        if self.content.len() > MAX_CONTENT_LENGTH {
            return Err(format!("Content exceeds maximum length of {} characters", MAX_CONTENT_LENGTH));
        }
        Ok(())
    }
}

/// Configuration for page numbers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageNumberConfig {
    /// Whether page numbers are enabled
    pub enabled: bool,
    /// Position: "header" or "footer"
    pub position: String,
    /// Text alignment: "left", "center", or "right"
    pub align: String,
    /// Number format: "1", "I", "i", "A", "a"
    pub format: String,
}

impl Default for PageNumberConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            position: "footer".to_string(),
            align: "center".to_string(),
            format: "1".to_string(),
        }
    }
}

impl PageNumberConfig {
    /// Validates the page number configuration
    pub fn validate(&self) -> Result<(), String> {
        if !VALID_POSITIONS.contains(&self.position.as_str()) {
            return Err(format!("Invalid position: {}. Must be one of: header, footer", self.position));
        }
        if !VALID_ALIGNMENTS.contains(&self.align.as_str()) {
            return Err(format!("Invalid alignment: {}. Must be one of: left, center, right", self.align));
        }
        if !VALID_FORMATS.contains(&self.format.as_str()) {
            return Err(format!("Invalid format: {}. Must be one of: 1, I, i, A, a", self.format));
        }
        Ok(())
    }
}

/// Service for managing document headers, footers, and page numbers
pub struct HeaderFooterService;

impl HeaderFooterService {
    /// Creates a new header/footer service instance
    pub fn new() -> Self {
        Self
    }

    /// Applies header and footer to HTML document
    /// 
    /// # Arguments
    /// * `html` - The HTML content to modify
    /// * `header` - Header configuration
    /// * `footer` - Footer configuration
    /// 
    /// # Returns
    /// Modified HTML with header and footer applied
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    pub fn apply_header_footer(
        &self,
        html: &str,
        header: &HeaderConfig,
        footer: &FooterConfig,
    ) -> String {
        let start_time = Instant::now();
        
        // Validate configurations
        if let Err(e) = header.validate() {
            eprintln!("Header validation failed: {}", e);
            return html.to_string();
        }
        if let Err(e) = footer.validate() {
            eprintln!("Footer validation failed: {}", e);
            return html.to_string();
        }

        let mut modified_html = html.to_string();

        // Remove existing header/footer first
        modified_html = self.remove_header_footer(&modified_html);

        // Add header
        if header.enabled && !header.content.is_empty() {
            let header_html = self.format_header(header);
            modified_html = format!("{}{}", header_html, modified_html);
        }

        // Add footer
        if footer.enabled && !footer.content.is_empty() {
            let footer_html = self.format_footer(footer);
            modified_html = format!("{}{}", modified_html, footer_html);
        }

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Header/footer application performance warning: took {}ms", elapsed.as_millis());
        }

        modified_html
    }

    /// Removes existing header and footer from HTML
    /// 
    /// # Arguments
    /// * `html` - The HTML content to modify
    /// 
    /// # Returns
    /// HTML with header and footer removed
    pub fn remove_header_footer(&self, html: &str) -> String {
        let mut modified = html.to_string();
        
        // Remove existing header with error handling
        match regex::Regex::new(r#"<div class="document-header"[^>]*>.*?</div>"#) {
            Ok(re) => {
                modified = re.replace_all(&modified, "").to_string();
            }
            Err(e) => {
                eprintln!("Failed to create header regex: {}", e);
            }
        }

        // Remove existing footer with error handling
        match regex::Regex::new(r#"<div class="document-footer"[^>]*>.*?</div>"#) {
            Ok(re) => {
                modified = re.replace_all(&modified, "").to_string();
            }
            Err(e) => {
                eprintln!("Failed to create footer regex: {}", e);
            }
        }

        modified
    }

    /// Applies page numbers to HTML document
    /// 
    /// # Arguments
    /// * `html` - The HTML content to modify
    /// * `config` - Page number configuration
    /// 
    /// # Returns
    /// Modified HTML with page numbers applied
    pub fn apply_page_numbers(&self, html: &str, config: &PageNumberConfig) -> String {
        // Validate configuration
        if let Err(e) = config.validate() {
            eprintln!("Page number validation failed: {}", e);
            return html.to_string();
        }

        let mut modified_html = html.to_string();

        // Remove existing page numbers with error handling
        match regex::Regex::new(r#"<div class="page-number"[^>]*>.*?</div>"#) {
            Ok(re) => {
                modified_html = re.replace_all(&modified_html, "").to_string();
            }
            Err(e) => {
                eprintln!("Failed to create page number regex: {}", e);
            }
        }

        if !config.enabled {
            return modified_html;
        }

        let page_number_html = self.format_page_number(config);

        match config.position.as_str() {
            "header" => {
                // Insert at the beginning
                format!("{}{}", page_number_html, modified_html)
            }
            "footer" => {
                // Insert at the end
                format!("{}{}", modified_html, page_number_html)
            }
            _ => {
                eprintln!("Unknown page number position: {}, defaulting to footer", config.position);
                format!("{}{}", modified_html, page_number_html)
            }
        }
    }

    /// Formats header HTML with proper escaping
    fn format_header(&self, config: &HeaderConfig) -> String {
        let align_style = format!("text-align: {};", config.align);
        let first_page_attr = if config.different_first_page {
            " data-first-page=\"true\""
        } else {
            ""
        };
        let escaped_content = self.escape_html(&config.content);

        format!(
            r#"<div class="document-header" style="padding: 20px; border-bottom: 1px solid #ccc; margin-bottom: 20px;"{}><div style="{}">{}</div></div>"#,
            first_page_attr, align_style, escaped_content
        )
    }

    /// Formats footer HTML with proper escaping
    fn format_footer(&self, config: &FooterConfig) -> String {
        let align_style = format!("text-align: {};", config.align);
        let first_page_attr = if config.different_first_page {
            " data-first-page=\"true\""
        } else {
            ""
        };
        let escaped_content = self.escape_html(&config.content);

        format!(
            r#"<div class="document-footer" style="padding: 20px; border-top: 1px solid #ccc; margin-top: 20px;"{}><div style="{}">{}</div></div>"#,
            first_page_attr, align_style, escaped_content
        )
    }

    /// Formats page number HTML
    fn format_page_number(&self, config: &PageNumberConfig) -> String {
        let align_style = format!("text-align: {};", config.align);
        
        format!(
            r#"<div class="page-number" style="padding: 10px; margin: 20px 0;"><div style="{}">Page <span class="page-num">1</span></div></div>"#,
            align_style
        )
    }

    /// Escapes HTML special characters to prevent injection
    fn escape_html(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }
}

impl Default for HeaderFooterService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_header() {
        let service = HeaderFooterService::new();
        let html = "<p>Content</p>";
        let header = HeaderConfig {
            enabled: true,
            content: "Header Text".to_string(),
            align: "center".to_string(),
            different_first_page: false,
        };
        let footer = FooterConfig::default();
        let result = service.apply_header_footer(html, &header, &footer);
        assert!(result.contains("Header Text"));
        assert!(result.contains("document-header"));
    }

    #[test]
    fn test_apply_footer() {
        let service = HeaderFooterService::new();
        let html = "<p>Content</p>";
        let header = HeaderConfig::default();
        let footer = FooterConfig {
            enabled: true,
            content: "Footer Text".to_string(),
            align: "center".to_string(),
            different_first_page: false,
        };
        let result = service.apply_header_footer(html, &header, &footer);
        assert!(result.contains("Footer Text"));
        assert!(result.contains("document-footer"));
    }

    #[test]
    fn test_remove_header_footer() {
        let service = HeaderFooterService::new();
        let html = r#"<div class="document-header">Header</div><p>Content</p><div class="document-footer">Footer</div>"#;
        let result = service.remove_header_footer(html);
        assert!(!result.contains("document-header"));
        assert!(!result.contains("document-footer"));
    }

    #[test]
    fn test_apply_page_numbers() {
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
    fn test_header_alignment() {
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
    fn test_empty_header_footer() {
        let service = HeaderFooterService::new();
        let html = "<p>Content</p>";
        let result = service.apply_header_footer(html, &HeaderConfig::default(), &FooterConfig::default());
        assert_eq!(result, html);
    }
}
