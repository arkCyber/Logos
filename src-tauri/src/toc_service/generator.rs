use serde::{Deserialize, Serialize};
use scraper::{Html, Selector};
use std::time::Instant;

/// Maximum allowed HTML input size in bytes to prevent DoS attacks
const MAX_HTML_SIZE: usize = 10 * 1024 * 1024; // 10MB

/// Maximum heading text length to prevent memory issues
const MAX_HEADING_TEXT_LENGTH: usize = 1000;

/// Maximum number of headings to process
const MAX_HEADINGS: usize = 1000;

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 1000;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 5000;

/// Represents a single item in the table of contents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TocItem {
    /// Unique identifier for the heading (slugified text)
    pub id: String,
    /// Heading level (1-6 for h1-h6)
    pub level: usize,
    /// The text content of the heading
    pub text: String,
    /// Child headings (for nested structure)
    pub children: Vec<TocItem>,
}

/// Result of TOC generation containing both structured data and HTML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TocResult {
    /// Structured TOC items
    pub items: Vec<TocItem>,
    /// Rendered HTML representation
    pub html: String,
}

/// Service for generating and managing table of contents
pub struct TocService;

impl TocService {
    /// Creates a new TOC service instance
    /// 
    /// # Returns
    /// A new TocService instance
    pub fn new() -> Self {
        Self
    }

    /// Get the maximum HTML size constant
    /// 
    /// # Returns
    /// The maximum HTML size in bytes
    pub fn max_html_size() -> usize {
        MAX_HTML_SIZE
    }

    /// Get the maximum heading text length constant
    /// 
    /// # Returns
    /// The maximum heading text length in characters
    pub fn max_heading_text_length() -> usize {
        MAX_HEADING_TEXT_LENGTH
    }

    /// Get the maximum headings constant
    /// 
    /// # Returns
    /// The maximum number of headings to process
    pub fn max_headings() -> usize {
        MAX_HEADINGS
    }

    /// Get the performance warning threshold
    /// 
    /// # Returns
    /// The performance warning threshold in milliseconds
    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    /// Get the performance critical threshold
    /// 
    /// # Returns
    /// The performance critical threshold in milliseconds
    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    /// Generates a table of contents from HTML content
    /// 
    /// # Arguments
    /// * `html` - The HTML content to parse
    /// 
    /// # Returns
    /// A `TocResult` containing structured TOC items and HTML representation
    /// 
    /// # Panics
    /// This function will not panic on invalid input, but may return empty results
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    pub fn generate_toc(&self, html: &str) -> TocResult {
        let start_time = Instant::now();
        
        // Input validation
        if html.is_empty() {
            return TocResult {
                items: Vec::new(),
                html: String::new(),
            };
        }

        // Security check: prevent DoS with oversized input
        if html.len() > MAX_HTML_SIZE {
            eprintln!("TOC generation: HTML input exceeds maximum size of {} bytes", MAX_HTML_SIZE);
            return TocResult {
                items: Vec::new(),
                html: String::new(),
            };
        }

        let document = Html::parse_document(html);
        let mut items = Vec::new();

        // Select all heading elements
        let heading_selector = match Selector::parse("h1, h2, h3, h4, h5, h6") {
            Ok(selector) => selector,
            Err(e) => {
                eprintln!("Failed to parse heading selector: {}", e);
                return TocResult {
                    items: Vec::new(),
                    html: String::new(),
                };
            }
        };
        
        for element in document.select(&heading_selector) {
            // Safety check: prevent processing too many headings
            if items.len() >= MAX_HEADINGS {
                eprintln!("TOC generation: reached maximum heading limit of {}", MAX_HEADINGS);
                break;
            }

            let tag_name = element.value().name();
            
            // Validate heading level
            let level = if tag_name.len() > 1 {
                tag_name[1..].parse::<usize>().unwrap_or(1)
            } else {
                1
            };
            
            // Clamp level to valid range 1-6
            let level = level.clamp(1, 6);
            
            let text = element.text().collect::<String>().trim().to_string();
            
            // Validate and truncate text length
            let text = if text.len() > MAX_HEADING_TEXT_LENGTH {
                eprintln!("TOC generation: heading text truncated from {} to {} characters", 
                    text.len(), MAX_HEADING_TEXT_LENGTH);
                text.chars().take(MAX_HEADING_TEXT_LENGTH).collect()
            } else {
                text
            };
            
            // Skip empty headings
            if text.is_empty() {
                continue;
            }
            
            // Generate ID from text with proper validation
            let id = self.generate_id(&text);
            
            // If ID generation failed, use a fallback
            let id = if id.is_empty() {
                format!("heading-{}", items.len())
            } else {
                id
            };

            let item = TocItem {
                id: id.clone(),
                level,
                text: self.escape_html(&text),
                children: Vec::new(),
            };

            items.push(item);
        }

        let html = self.generate_html(&items);

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("TOC generation CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("TOC generation performance warning: took {}ms", elapsed.as_millis());
        }

        TocResult { items, html }
    }

    /// Generates a URL-safe ID from heading text
    fn generate_id(&self, text: &str) -> String {
        text.to_lowercase()
            .chars()
            .filter(|c| c.is_ascii())
            .map(|c| {
                if c.is_ascii_alphanumeric() {
                    c
                } else {
                    '-'
                }
            })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("-")
    }

    /// Escapes HTML special characters to prevent injection
    fn escape_html(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }

    /// Generates HTML representation of TOC items
    /// 
    /// # Arguments
    /// * `items` - Slice of TOC items to render
    /// 
    /// # Returns
    /// HTML string containing the TOC structure
    pub fn generate_html(&self, items: &[TocItem]) -> String {
        if items.is_empty() {
            return String::new();
        }

        let mut html = String::from("<ul class=\"toc\">");
        
        for item in items {
            html.push_str(&format!(
                "<li class=\"toc-item toc-level-{}\"><a href=\"#{}\">{}</a>",
                item.level, item.id, item.text
            ));
            
            if !item.children.is_empty() {
                html.push_str(&self.generate_html(&item.children));
            }
            
            html.push_str("</li>");
        }
        
        html.push_str("</ul>");
        html
    }

    /// Inserts TOC into HTML at specified position
    /// 
    /// # Arguments
    /// * `html` - The original HTML content
    /// * `toc` - The TOC result to insert
    /// * `position` - Where to insert the TOC
    /// 
    /// # Returns
    /// Modified HTML with TOC inserted
    pub fn insert_toc(&self, html: &str, toc: &TocResult, position: InsertPosition) -> String {
        if toc.html.is_empty() {
            return html.to_string();
        }

        let toc_html = format!(
            "<div class=\"table-of-contents\">{}</div>",
            toc.html
        );

        match position {
            InsertPosition::Beginning => format!("{}{}", toc_html, html),
            InsertPosition::AfterFirstHeading => {
                if let Some(pos) = html.find("<h1") {
                    if let Some(end_pos) = html[pos..].find('>') {
                        let insert_pos = pos + end_pos + 1;
                        if insert_pos <= html.len() {
                            format!("{}{}{}", &html[..insert_pos], toc_html, &html[insert_pos..])
                        } else {
                            format!("{}{}", toc_html, html)
                        }
                    } else {
                        format!("{}{}", toc_html, html)
                    }
                } else {
                    format!("{}{}", toc_html, html)
                }
            }
            InsertPosition::End => format!("{}{}", html, toc_html),
        }
    }
}

impl Default for TocService {
    fn default() -> Self {
        Self::new()
    }
}

/// Position where TOC should be inserted in the document
#[derive(Debug, Clone, Copy)]
pub enum InsertPosition {
    /// Insert at the very beginning of the document
    Beginning,
    /// Insert after the first h1 heading
    AfterFirstHeading,
    /// Insert at the end of the document
    End,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_toc_basic() {
        let service = TocService::new();
        let html = "<h1>Title</h1><p>Content</p><h2>Subtitle</h2>";
        let result = service.generate_toc(html);
        assert_eq!(result.items.len(), 2);
        assert_eq!(result.items[0].level, 1);
        assert_eq!(result.items[1].level, 2);
    }

    #[test]
    fn test_generate_toc_nested() {
        let service = TocService::new();
        let html = "<h1>Title</h1><h2>Subtitle</h2><h3>Sub-subtitle</h3>";
        let result = service.generate_toc(html);
        assert_eq!(result.items.len(), 3); // Flat structure: all headings
    }

    #[test]
    fn test_generate_html() {
        let service = TocService::new();
        let items = vec![
            TocItem {
                id: "title".to_string(),
                level: 1,
                text: "Title".to_string(),
                children: vec![],
            }
        ];
        let html = service.generate_html(&items);
        assert!(html.contains("<ul class=\"toc\">"));
        assert!(html.contains("<a href=\"#title\">Title</a>"));
    }

    #[test]
    fn test_insert_toc_beginning() {
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
    fn test_insert_toc_end() {
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
    fn test_empty_html() {
        let service = TocService::new();
        let html = "";
        let result = service.generate_toc(html);
        assert_eq!(result.items.len(), 0);
    }

    #[test]
    fn test_no_headings() {
        let service = TocService::new();
        let html = "<p>Just content</p>";
        let result = service.generate_toc(html);
        assert_eq!(result.items.len(), 0);
    }

    #[test]
    fn test_max_html_size() {
        let service = TocService::new();
        let large_html = "a".repeat(MAX_HTML_SIZE + 1);
        let result = service.generate_toc(&large_html);
        assert_eq!(result.items.len(), 0);
    }

    #[test]
    fn test_max_headings_limit() {
        let service = TocService::new();
        let html = "<h1>Heading</h1>".repeat(MAX_HEADINGS + 10);
        let result = service.generate_toc(&html);
        assert!(result.items.len() <= MAX_HEADINGS);
    }

    #[test]
    fn test_max_heading_text_length() {
        let service = TocService::new();
        let long_text = "a".repeat(MAX_HEADING_TEXT_LENGTH + 100);
        let html = format!("<h1>{}</h1>", long_text);
        let result = service.generate_toc(&html);
        assert!(result.items[0].text.len() <= MAX_HEADING_TEXT_LENGTH);
    }

    #[test]
    fn test_service_getters() {
        assert_eq!(TocService::max_html_size(), MAX_HTML_SIZE);
        assert_eq!(TocService::max_heading_text_length(), MAX_HEADING_TEXT_LENGTH);
        assert_eq!(TocService::max_headings(), MAX_HEADINGS);
        assert_eq!(TocService::performance_warning_threshold_ms(), PERFORMANCE_WARNING_THRESHOLD_MS);
        assert_eq!(TocService::performance_critical_threshold_ms(), PERFORMANCE_CRITICAL_THRESHOLD_MS);
    }

    #[test]
    fn test_escape_html() {
        let service = TocService::new();
        let escaped = service.escape_html("<script>alert('xss')</script>");
        assert!(escaped.contains("&lt;"));
        assert!(escaped.contains("&gt;"));
        assert!(!escaped.contains("<script>"));
    }

    #[test]
    fn test_generate_id_special_chars() {
        let service = TocService::new();
        let id = service.generate_id("Hello World! @#$");
        assert!(!id.contains(' '));
        assert!(!id.contains('@'));
        assert!(!id.contains('#'));
        assert!(!id.contains('$'));
    }

    #[test]
    fn test_empty_heading_skipped() {
        let service = TocService::new();
        let html = "<h1></h1><h2>Valid</h2>";
        let result = service.generate_toc(html);
        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].text, "Valid");
    }

    #[test]
    fn test_heading_level_clamping() {
        let service = TocService::new();
        let html = "<h1>Level 1</h1><h6>Level 6</h6>";
        let result = service.generate_toc(html);
        assert_eq!(result.items[0].level, 1);
        assert_eq!(result.items[1].level, 6);
    }
}
