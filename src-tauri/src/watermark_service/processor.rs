use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Maximum watermark text length to prevent memory issues
const MAX_WATERMARK_TEXT_LENGTH: usize = 500;

/// Valid opacity range (0.0 to 1.0)
const MIN_OPACITY: f32 = 0.0;
const MAX_OPACITY: f32 = 1.0;

/// Valid rotation range (-180 to 180 degrees)
const MIN_ROTATION: i32 = -180;
const MAX_ROTATION: i32 = 180;

/// Valid font size range (8 to 200 pixels)
const MIN_FONT_SIZE: i32 = 8;
const MAX_FONT_SIZE: i32 = 200;

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 500;

/// Configuration for document watermark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatermarkConfig {
    /// Whether the watermark is enabled
    pub enabled: bool,
    /// Watermark text content
    pub text: String,
    /// Opacity value (0.0 to 1.0)
    pub opacity: f32,
    /// Rotation angle in degrees (-180 to 180)
    pub rotation: i32,
    /// Text color in hex format (e.g., "#cccccc")
    pub color: String,
    /// Font size in pixels
    pub font_size: i32,
}

impl Default for WatermarkConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            text: String::new(),
            opacity: 0.3,
            rotation: -45,
            color: "#cccccc".to_string(),
            font_size: 48,
        }
    }
}

impl WatermarkConfig {
    /// Validates the watermark configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.text.len() > MAX_WATERMARK_TEXT_LENGTH {
            return Err(format!("Text exceeds maximum length of {} characters", MAX_WATERMARK_TEXT_LENGTH));
        }
        if self.opacity < MIN_OPACITY || self.opacity > MAX_OPACITY {
            return Err(format!("Opacity must be between {} and {}", MIN_OPACITY, MAX_OPACITY));
        }
        if self.rotation < MIN_ROTATION || self.rotation > MAX_ROTATION {
            return Err(format!("Rotation must be between {} and {} degrees", MIN_ROTATION, MAX_ROTATION));
        }
        if self.font_size < MIN_FONT_SIZE || self.font_size > MAX_FONT_SIZE {
            return Err(format!("Font size must be between {} and {} pixels", MIN_FONT_SIZE, MAX_FONT_SIZE));
        }
        if !self.color.starts_with('#') || self.color.len() != 7 {
            return Err("Color must be a valid hex color (e.g., #cccccc)".to_string());
        }
        Ok(())
    }
}

/// Service for managing document watermarks
pub struct WatermarkService;

impl WatermarkService {
    /// Creates a new watermark service instance
    pub fn new() -> Self {
        Self
    }

    /// Applies watermark to HTML document
    /// 
    /// # Arguments
    /// * `html` - The HTML content to modify
    /// * `config` - Watermark configuration
    /// 
    /// # Returns
    /// Modified HTML with watermark applied
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    pub fn apply_watermark(&self, html: &str, config: &WatermarkConfig) -> String {
        let start_time = Instant::now();
        
        // Validate configuration
        if let Err(e) = config.validate() {
            eprintln!("Watermark validation failed: {}", e);
            return html.to_string();
        }

        if !config.enabled || config.text.is_empty() {
            return html.to_string();
        }

        let mut modified_html = html.to_string();

        // Remove existing watermark
        modified_html = self.remove_watermark(&modified_html);

        // Create watermark HTML
        let watermark_html = self.format_watermark(config);

        // Insert watermark at the beginning of the document
        let result = format!("{}{}", watermark_html, modified_html);
        
        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Watermark application performance warning: took {}ms", elapsed.as_millis());
        }
        
        result
    }

    /// Removes existing watermark from HTML
    /// 
    /// # Arguments
    /// * `html` - The HTML content to modify
    /// 
    /// # Returns
    /// HTML with watermark removed
    pub fn remove_watermark(&self, html: &str) -> String {
        match regex::Regex::new(r#"<div class="watermark"[^>]*>.*?</div>"#) {
            Ok(re) => re.replace_all(html, "").to_string(),
            Err(e) => {
                eprintln!("Failed to create watermark regex: {}", e);
                html.to_string()
            }
        }
    }

    /// Formats watermark HTML with proper escaping and validation
    fn format_watermark(&self, config: &WatermarkConfig) -> String {
        // Clamp values to valid ranges
        let opacity = config.opacity.clamp(MIN_OPACITY, MAX_OPACITY);
        let rotation = config.rotation.clamp(MIN_ROTATION, MAX_ROTATION);
        let font_size = config.font_size.clamp(MIN_FONT_SIZE, MAX_FONT_SIZE);
        
        // Escape text to prevent HTML injection
        let escaped_text = self.escape_html(&config.text);
        
        // Validate and sanitize color
        let color = if self.is_valid_hex_color(&config.color) {
            &config.color
        } else {
            eprintln!("Invalid color format: {}, using default", config.color);
            "#cccccc"
        };

        format!(
            r#"<div class="watermark" style="position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%) rotate({}deg); opacity: {}; color: {}; font-size: {}px; font-weight: bold; pointer-events: none; z-index: 1000; white-space: nowrap;">{}</div>"#,
            rotation, opacity, color, font_size, escaped_text
        )
    }

    /// Validates hex color format
    fn is_valid_hex_color(&self, color: &str) -> bool {
        color.starts_with('#') && color.len() == 7 && color[1..].chars().all(|c| c.is_ascii_hexdigit())
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

impl Default for WatermarkService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_watermark() {
        let service = WatermarkService::new();
        let html = "<p>Content</p>";
        let config = WatermarkConfig {
            enabled: true,
            text: "CONFIDENTIAL".to_string(),
            opacity: 0.5,
            rotation: -45,
            color: "#ff0000".to_string(),
            font_size: 48,
        };
        let result = service.apply_watermark(html, &config);
        assert!(result.contains("CONFIDENTIAL"));
        assert!(result.contains("watermark"));
    }

    #[test]
    fn test_remove_watermark() {
        let service = WatermarkService::new();
        let html = r#"<div class="watermark">Test</div><p>Content</p>"#;
        let result = service.remove_watermark(html);
        assert!(!result.contains("watermark"));
        assert!(result.contains("Content"));
    }

    #[test]
    fn test_disabled_watermark() {
        let service = WatermarkService::new();
        let html = "<p>Content</p>";
        let config = WatermarkConfig {
            enabled: false,
            text: "Test".to_string(),
            ..Default::default()
        };
        let result = service.apply_watermark(html, &config);
        assert_eq!(result, html);
    }

    #[test]
    fn test_empty_text_watermark() {
        let service = WatermarkService::new();
        let html = "<p>Content</p>";
        let config = WatermarkConfig {
            enabled: true,
            text: String::new(),
            ..Default::default()
        };
        let result = service.apply_watermark(html, &config);
        assert_eq!(result, html);
    }

    #[test]
    fn test_watermark_opacity() {
        let service = WatermarkService::new();
        let html = "<p>Content</p>";
        let config = WatermarkConfig {
            enabled: true,
            text: "Test".to_string(),
            opacity: 0.7,
            ..Default::default()
        };
        let result = service.apply_watermark(html, &config);
        assert!(result.contains("opacity: 0.7"));
    }

    #[test]
    fn test_watermark_rotation() {
        let service = WatermarkService::new();
        let html = "<p>Content</p>";
        let config = WatermarkConfig {
            enabled: true,
            text: "Test".to_string(),
            rotation: 30,
            ..Default::default()
        };
        let result = service.apply_watermark(html, &config);
        assert!(result.contains("rotate(30deg)"));
    }

    #[test]
    fn test_watermark_color() {
        let service = WatermarkService::new();
        let html = "<p>Content</p>";
        let config = WatermarkConfig {
            enabled: true,
            text: "Test".to_string(),
            color: "#00ff00".to_string(),
            ..Default::default()
        };
        let result = service.apply_watermark(html, &config);
        assert!(result.contains("#00ff00"));
    }

    #[test]
    fn test_watermark_font_size() {
        let service = WatermarkService::new();
        let html = "<p>Content</p>";
        let config = WatermarkConfig {
            enabled: true,
            text: "Test".to_string(),
            font_size: 72,
            ..Default::default()
        };
        let result = service.apply_watermark(html, &config);
        assert!(result.contains("font-size: 72px"));
    }
}
