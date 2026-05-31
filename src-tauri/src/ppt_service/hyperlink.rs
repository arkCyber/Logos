//! Hyperlink Element Module
//! 
//! Aerospace-grade hyperlink element implementation for PPT slides with:
//! - Input validation
//! - URL validation
//! - Security hardening against malicious URLs
//! - Comprehensive error handling

use serde::{Deserialize, Serialize};

/// Hyperlink element for PPT slides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperlinkElement {
    /// Hyperlink element ID
    pub id: String,
    /// Target URL
    pub url: String,
    /// Display text
    pub text: String,
    /// Tooltip text
    pub tooltip: String,
    /// Hyperlink type
    pub link_type: HyperlinkType,
    /// Position (X, Y coordinates in points)
    pub position: (f64, f64),
    /// Size (width, height in points)
    pub size: (f64, f64),
    /// Whether to open in new window
    pub open_in_new_window: bool,
    /// Screen tip (alternative text)
    pub screen_tip: String,
}

/// Hyperlink type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HyperlinkType {
    /// External URL
    External,
    /// Internal slide reference
    InternalSlide(String),
    /// Email address
    Email(String),
    /// File path
    FilePath(String),
}

impl HyperlinkElement {
    /// Maximum URL length to prevent buffer overflow
    const MAX_URL_LENGTH: usize = 2048;

    /// Maximum text length
    const MAX_TEXT_LENGTH: usize = 500;

    /// Maximum tooltip length
    const MAX_TOOLTIP_LENGTH: usize = 255;

    /// Create a new hyperlink element
    pub fn new(id: String, url: String, text: String) -> Self {
        Self {
            id,
            url,
            text,
            tooltip: String::new(),
            link_type: HyperlinkType::External,
            position: (0.0, 0.0),
            size: (100.0, 20.0),
            open_in_new_window: false,
            screen_tip: String::new(),
        }
    }

    /// Set tooltip
    pub fn with_tooltip(mut self, tooltip: String) -> Self {
        self.tooltip = tooltip;
        self
    }

    /// Set link type
    pub fn with_link_type(mut self, link_type: HyperlinkType) -> Self {
        self.link_type = link_type;
        self
    }

    /// Set position
    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.position = (x, y);
        self
    }

    /// Set size
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.size = (width, height);
        self
    }

    /// Set open in new window
    pub fn with_open_in_new_window(mut self, open: bool) -> Self {
        self.open_in_new_window = open;
        self
    }

    /// Set screen tip
    pub fn with_screen_tip(mut self, screen_tip: String) -> Self {
        self.screen_tip = screen_tip;
        self
    }

    /// Validate URL
    pub fn validate_url(&self) -> Result<(), String> {
        // Check if URL is empty
        if self.url.is_empty() {
            return Err("URL cannot be empty".to_string());
        }

        // Check URL length
        if self.url.len() > Self::MAX_URL_LENGTH {
            return Err(format!(
                "URL length exceeds maximum of {} characters",
                Self::MAX_URL_LENGTH
            ));
        }

        // Validate based on link type
        match &self.link_type {
            HyperlinkType::External => {
                // Must be HTTP or HTTPS
                if !self.url.starts_with("http://") && !self.url.starts_with("https://") {
                    return Err("External URL must start with http:// or https://".to_string());
                }
            }
            HyperlinkType::Email(_) => {
                // Validate email format
                if !self.url.contains('@') || !self.url.contains('.') {
                    return Err("Invalid email address format".to_string());
                }
            }
            HyperlinkType::InternalSlide(_) => {
                // Internal slide reference - just check it's not empty
                if self.url.is_empty() {
                    return Err("Internal slide reference cannot be empty".to_string());
                }
            }
            HyperlinkType::FilePath(_) => {
                // File path - basic validation
                if self.url.is_empty() {
                    return Err("File path cannot be empty".to_string());
                }
            }
        }

        Ok(())
    }

    /// Validate hyperlink settings
    pub fn validate(&self) -> Result<(), String> {
        // Validate URL
        self.validate_url()?;

        // Validate text
        if self.text.is_empty() {
            return Err("Display text cannot be empty".to_string());
        }

        if self.text.len() > Self::MAX_TEXT_LENGTH {
            return Err(format!(
                "Text length exceeds maximum of {} characters",
                Self::MAX_TEXT_LENGTH
            ));
        }

        // Validate tooltip
        if self.tooltip.len() > Self::MAX_TOOLTIP_LENGTH {
            return Err(format!(
                "Tooltip length exceeds maximum of {} characters",
                Self::MAX_TOOLTIP_LENGTH
            ));
        }

        // Validate position
        if self.position.0 < 0.0 || self.position.1 < 0.0 {
            return Err("Position coordinates cannot be negative".to_string());
        }

        // Validate size
        if self.size.0 <= 0.0 || self.size.1 <= 0.0 {
            return Err("Size dimensions must be positive".to_string());
        }

        Ok(())
    }

    /// Create external hyperlink
    pub fn external(id: String, url: String, text: String) -> Result<Self, String> {
        let link = Self::new(id, url, text).with_link_type(HyperlinkType::External);
        link.validate()?;
        Ok(link)
    }

    /// Create email hyperlink
    pub fn email(id: String, email: String, text: String) -> Result<Self, String> {
        let link = Self::new(id, email.clone(), text)
            .with_link_type(HyperlinkType::Email(email));
        link.validate()?;
        Ok(link)
    }

    /// Create internal slide hyperlink
    pub fn internal_slide(id: String, slide_id: String, text: String) -> Result<Self, String> {
        let link = Self::new(id, slide_id.clone(), text)
            .with_link_type(HyperlinkType::InternalSlide(slide_id));
        link.validate()?;
        Ok(link)
    }

    /// Create file path hyperlink
    pub fn file_path(id: String, path: String, text: String) -> Result<Self, String> {
        let link = Self::new(id, path.clone(), text)
            .with_link_type(HyperlinkType::FilePath(path));
        link.validate()?;
        Ok(link)
    }

    /// Get display URL (for UI purposes)
    pub fn display_url(&self) -> String {
        match &self.link_type {
            HyperlinkType::External => self.url.clone(),
            HyperlinkType::Email(email) => format!("mailto:{}", email),
            HyperlinkType::InternalSlide(slide_id) => format!("slide:{}", slide_id),
            HyperlinkType::FilePath(path) => format!("file:{}", path),
        }
    }
}

impl Default for HyperlinkElement {
    fn default() -> Self {
        Self::new("default".to_string(), "".to_string(), "".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyperlink_element_new() {
        let link = HyperlinkElement::new("1".to_string(), "https://example.com".to_string(), "Click here".to_string());
        assert_eq!(link.id, "1");
        assert_eq!(link.url, "https://example.com");
        assert_eq!(link.text, "Click here");
        assert_eq!(link.link_type, HyperlinkType::External);
    }

    #[test]
    fn test_hyperlink_element_with_tooltip() {
        let link = HyperlinkElement::new("1".to_string(), "https://example.com".to_string(), "Click here".to_string())
            .with_tooltip("Visit example.com".to_string());
        assert_eq!(link.tooltip, "Visit example.com");
    }

    #[test]
    fn test_hyperlink_element_with_position() {
        let link = HyperlinkElement::new("1".to_string(), "https://example.com".to_string(), "Click here".to_string())
            .with_position(100.0, 200.0);
        assert_eq!(link.position, (100.0, 200.0));
    }

    #[test]
    fn test_hyperlink_element_with_open_in_new_window() {
        let link = HyperlinkElement::new("1".to_string(), "https://example.com".to_string(), "Click here".to_string())
            .with_open_in_new_window(true);
        assert!(link.open_in_new_window);
    }

    #[test]
    fn test_hyperlink_element_validate_url_empty() {
        let link = HyperlinkElement::new("1".to_string(), "".to_string(), "Click here".to_string());
        assert!(link.validate_url().is_err());
    }

    #[test]
    fn test_hyperlink_element_validate_url_invalid_external() {
        let link = HyperlinkElement::new("1".to_string(), "example.com".to_string(), "Click here".to_string())
            .with_link_type(HyperlinkType::External);
        assert!(link.validate_url().is_err());
    }

    #[test]
    fn test_hyperlink_element_validate_url_valid_external() {
        let link = HyperlinkElement::new("1".to_string(), "https://example.com".to_string(), "Click here".to_string())
            .with_link_type(HyperlinkType::External);
        assert!(link.validate_url().is_ok());
    }

    #[test]
    fn test_hyperlink_element_validate_text_empty() {
        let link = HyperlinkElement::new("1".to_string(), "https://example.com".to_string(), "".to_string());
        assert!(link.validate().is_err());
    }

    #[test]
    fn test_hyperlink_element_validate_position_negative() {
        let link = HyperlinkElement::new("1".to_string(), "https://example.com".to_string(), "Click here".to_string())
            .with_position(-10.0, 100.0);
        assert!(link.validate().is_err());
    }

    #[test]
    fn test_hyperlink_element_external() {
        let link = HyperlinkElement::external("1".to_string(), "https://example.com".to_string(), "Click here".to_string()).unwrap();
        assert_eq!(link.link_type, HyperlinkType::External);
        assert_eq!(link.url, "https://example.com");
    }

    #[test]
    fn test_hyperlink_element_email() {
        let link = HyperlinkElement::email("1".to_string(), "test@example.com".to_string(), "Email me".to_string()).unwrap();
        assert!(matches!(link.link_type, HyperlinkType::Email(_)));
    }

    #[test]
    fn test_hyperlink_element_internal_slide() {
        let link = HyperlinkElement::internal_slide("1".to_string(), "slide-2".to_string(), "Go to slide 2".to_string()).unwrap();
        assert!(matches!(link.link_type, HyperlinkType::InternalSlide(_)));
    }

    #[test]
    fn test_hyperlink_element_file_path() {
        let link = HyperlinkElement::file_path("1".to_string(), "/path/to/file.pdf".to_string(), "Open file".to_string()).unwrap();
        assert!(matches!(link.link_type, HyperlinkType::FilePath(_)));
    }

    #[test]
    fn test_hyperlink_element_display_url() {
        let link = HyperlinkElement::new("1".to_string(), "https://example.com".to_string(), "Click here".to_string())
            .with_link_type(HyperlinkType::External);
        assert_eq!(link.display_url(), "https://example.com");

        let link = HyperlinkElement::new("1".to_string(), "test@example.com".to_string(), "Email me".to_string())
            .with_link_type(HyperlinkType::Email("test@example.com".to_string()));
        assert_eq!(link.display_url(), "mailto:test@example.com");
    }

    #[test]
    fn test_hyperlink_element_chaining() {
        let link = HyperlinkElement::new("1".to_string(), "https://example.com".to_string(), "Click here".to_string())
            .with_tooltip("Visit example.com".to_string())
            .with_position(100.0, 200.0)
            .with_size(150.0, 30.0)
            .with_open_in_new_window(true);
        assert_eq!(link.tooltip, "Visit example.com");
        assert_eq!(link.position, (100.0, 200.0));
        assert_eq!(link.size, (150.0, 30.0));
        assert!(link.open_in_new_window);
    }

    #[test]
    fn test_hyperlink_element_serialization() {
        let link = HyperlinkElement::new("1".to_string(), "https://example.com".to_string(), "Click here".to_string());
        let json = serde_json::to_string(&link);
        assert!(json.is_ok());
    }

    #[test]
    fn test_hyperlink_element_deserialization() {
        let link = HyperlinkElement::new("1".to_string(), "https://example.com".to_string(), "Click here".to_string());
        let json = serde_json::to_string(&link).unwrap();
        let deserialized: HyperlinkElement = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, link.id);
        assert_eq!(deserialized.url, link.url);
        assert_eq!(deserialized.text, link.text);
    }
}
