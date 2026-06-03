use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Maximum text length for conversion to prevent memory issues
const MAX_TEXT_LENGTH: usize = 1 * 1024 * 1024; // 1MB

/// Maximum character count for conversion to prevent performance issues
const MAX_CHAR_COUNT: usize = 1_000_000; // 1 million characters

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 100;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 1000;

/// Full-width character range start
const FULL_WIDTH_START: u32 = 0xFF01;

/// Full-width character range end
const FULL_WIDTH_END: u32 = 0xFF5E;

/// Full-width space character
const FULL_WIDTH_SPACE: u32 = 0x3000;

/// Half-width character range start
const HALF_WIDTH_START: u32 = 0x21;

/// Half-width character range end
const HALF_WIDTH_END: u32 = 0x7E;

/// Half-width space character
const HALF_WIDTH_SPACE: u32 = 0x20;

/// Conversion offset between full-width and half-width
const CONVERSION_OFFSET: u32 = 0xFEE0;

/// Type of text conversion
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConversionType {
    /// Convert full-width to half-width characters
    FullToHalf,
    /// Convert half-width to full-width characters
    HalfToFull,
    /// Auto-detect and convert
    Auto,
}

/// Configuration for text conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionConfig {
    /// Type of conversion to perform
    pub conversion_type: ConversionType,
    /// Whether to preserve newlines
    pub preserve_newlines: bool,
    /// Whether to preserve spaces
    pub preserve_spaces: bool,
}

impl Default for ConversionConfig {
    fn default() -> Self {
        Self {
            conversion_type: ConversionType::Auto,
            preserve_newlines: true,
            preserve_spaces: true,
        }
    }
}

impl ConversionConfig {
    /// Validates the conversion configuration
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Service for text conversion operations
pub struct TextConversionService;

impl TextConversionService {
    /// Creates a new text conversion service instance
    pub fn new() -> Self {
        Self
    }

    /// Converts text between full-width and half-width characters
    /// 
    /// # Arguments
    /// * `text` - The text to convert
    /// * `config` - Conversion configuration
    /// 
    /// # Returns
    /// Converted text string
    /// 
    /// # Performance
    /// Logs a warning if processing takes longer than PERFORMANCE_WARNING_THRESHOLD_MS
    /// Logs a critical warning if processing takes longer than PERFORMANCE_CRITICAL_THRESHOLD_MS
    /// 
    /// # Security
    /// Validates input size and character count to prevent DoS attacks
    pub fn convert_text(&self, text: &str, config: &ConversionConfig) -> String {
        let start_time = Instant::now();
        
        // Input validation
        if text.is_empty() {
            return String::new();
        }

        // Security check: prevent DoS with oversized input
        if text.len() > MAX_TEXT_LENGTH {
            eprintln!("Text conversion: input exceeds maximum size of {} bytes", MAX_TEXT_LENGTH);
            return text.to_string();
        }

        // Security check: prevent performance issues with too many characters
        let char_count = text.chars().count();
        if char_count > MAX_CHAR_COUNT {
            eprintln!("Text conversion: input exceeds maximum character count of {}", MAX_CHAR_COUNT);
            return text.to_string();
        }

        // Validate configuration
        if let Err(e) = config.validate() {
            eprintln!("Conversion config validation failed: {}", e);
            return text.to_string();
        }

        let result = match config.conversion_type {
            ConversionType::FullToHalf => self.full_to_half(text, config),
            ConversionType::HalfToFull => self.half_to_full(text, config),
            ConversionType::Auto => self.auto_convert(text, config),
        };

        // Performance monitoring
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Text conversion CRITICAL performance warning: took {}ms for {} characters", 
                elapsed.as_millis(), char_count);
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Text conversion performance warning: took {}ms for {} characters", 
                elapsed.as_millis(), char_count);
        }

        result
    }

    /// Converts full-width characters to half-width
    /// 
    /// Full-width range: 0xFF01-0xFF5E -> Half-width: 0x21-0x7E
    /// Full-width space: 0x3000 -> Half-width space: 0x20
    /// 
    /// # Arguments
    /// * `text` - The text to convert
    /// * `config` - Conversion configuration
    /// 
    /// # Returns
    /// Converted text string with full-width characters converted to half-width
    fn full_to_half(&self, text: &str, config: &ConversionConfig) -> String {
        let mut result = String::with_capacity(text.len());
        
        for c in text.chars() {
            let code = c as u32;
            
            if code >= FULL_WIDTH_START && code <= FULL_WIDTH_END {
                // Full-width to half-width
                if let Some(char) = char::from_u32(code - CONVERSION_OFFSET) {
                    result.push(char);
                }
            } else if code == FULL_WIDTH_SPACE {
                // Full-width space to half-width space
                if config.preserve_spaces {
                    result.push(' ');
                }
            } else if c == '\n' {
                // Handle newlines based on config
                if config.preserve_newlines {
                    result.push('\n');
                }
            } else if c == ' ' {
                // Handle spaces based on config
                if config.preserve_spaces {
                    result.push(' ');
                }
            } else {
                result.push(c);
            }
        }
        
        result
    }

    /// Converts half-width characters to full-width
    /// 
    /// Half-width range: 0x21-0x7E -> Full-width: 0xFF01-0xFF5E
    /// Half-width space: 0x20 -> Full-width space: 0x3000
    /// 
    /// # Arguments
    /// * `text` - The text to convert
    /// * `config` - Conversion configuration
    /// 
    /// # Returns
    /// Converted text string with half-width characters converted to full-width
    fn half_to_full(&self, text: &str, config: &ConversionConfig) -> String {
        let mut result = String::with_capacity(text.len() * 2);
        
        for c in text.chars() {
            let code = c as u32;
            
            if code >= HALF_WIDTH_START && code <= HALF_WIDTH_END {
                // Half-width to full-width
                if let Some(char) = char::from_u32(code + CONVERSION_OFFSET) {
                    result.push(char);
                }
            } else if code == HALF_WIDTH_SPACE {
                // Half-width space to full-width space
                if config.preserve_spaces {
                    result.push('\u{3000}');
                }
            } else if c == '\n' {
                // Handle newlines based on config
                if config.preserve_newlines {
                    result.push('\n');
                }
            } else if c == ' ' {
                // Handle spaces based on config
                if config.preserve_spaces {
                    result.push(' ');
                }
            } else {
                result.push(c);
            }
        }
        
        result
    }

    /// Auto-detects and converts between full-width and half-width
    /// 
    /// Analyzes the text to determine the dominant character type and converts
    /// to the opposite type. If full-width characters dominate, converts to half-width.
    /// If half-width characters dominate, converts to full-width.
    /// 
    /// # Arguments
    /// * `text` - The text to convert
    /// * `config` - Conversion configuration
    /// 
    /// # Returns
    /// Converted text string based on auto-detection
    /// 
    /// # Algorithm
    /// Counts full-width and half-width characters and converts to the less common type
    fn auto_convert(&self, text: &str, config: &ConversionConfig) -> String {
        // Detect if text has more full-width or half-width characters
        let full_width_count = text.chars()
            .filter(|c| {
                let code = *c as u32;
                (code >= FULL_WIDTH_START && code <= FULL_WIDTH_END) || code == FULL_WIDTH_SPACE
            })
            .count();
        
        let half_width_count = text.chars()
            .filter(|c| {
                let code = *c as u32;
                (code >= HALF_WIDTH_START && code <= HALF_WIDTH_END) || code == HALF_WIDTH_SPACE
            })
            .count();
        
        if full_width_count > half_width_count {
            self.full_to_half(text, config)
        } else {
            self.half_to_full(text, config)
        }
    }

    /// Detects if text contains full-width characters
    /// 
    /// # Arguments
    /// * `text` - The text to check
    /// 
    /// # Returns
    /// true if text contains full-width characters, false otherwise
    pub fn has_full_width(&self, text: &str) -> bool {
        text.chars().any(|c| {
            let code = c as u32;
            (code >= FULL_WIDTH_START && code <= FULL_WIDTH_END) || code == FULL_WIDTH_SPACE
        })
    }

    /// Detects if text contains half-width characters
    /// 
    /// # Arguments
    /// * `text` - The text to check
    /// 
    /// # Returns
    /// true if text contains half-width characters, false otherwise
    pub fn has_half_width(&self, text: &str) -> bool {
        text.chars().any(|c| {
            let code = c as u32;
            (code >= HALF_WIDTH_START && code <= HALF_WIDTH_END) || code == HALF_WIDTH_SPACE
        })
    }

    /// Gets statistics about character types in text
    /// 
    /// # Arguments
    /// * `text` - The text to analyze
    /// 
    /// # Returns
    /// CharStats containing counts of full-width, half-width, and other characters
    /// 
    /// # Performance
    /// O(n) time complexity where n is the number of characters
    pub fn get_char_stats(&self, text: &str) -> CharStats {
        let mut full_width = 0;
        let mut half_width = 0;
        let mut other = 0;
        
        for c in text.chars() {
            let code = c as u32;
            if (code >= FULL_WIDTH_START && code <= FULL_WIDTH_END) || code == FULL_WIDTH_SPACE {
                full_width += 1;
            } else if (code >= HALF_WIDTH_START && code <= HALF_WIDTH_END) || code == HALF_WIDTH_SPACE {
                half_width += 1;
            } else {
                other += 1;
            }
        }
        
        CharStats {
            full_width,
            half_width,
            other,
            total: text.chars().count(),
        }
    }
}

impl Default for TextConversionService {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about character types in text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharStats {
    /// Number of full-width characters
    pub full_width: usize,
    /// Number of half-width characters
    pub half_width: usize,
    /// Number of other characters
    pub other: usize,
    /// Total character count
    pub total: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_to_half_basic() {
        let service = TextConversionService::new();
        let config = ConversionConfig::default();
        let result = service.full_to_half("ＡＢＣ", &config);
        assert_eq!(result, "ABC");
    }

    #[test]
    fn test_half_to_full_basic() {
        let service = TextConversionService::new();
        let config = ConversionConfig::default();
        let result = service.half_to_full("ABC", &config);
        assert_eq!(result, "ＡＢＣ");
    }

    #[test]
    fn test_full_to_half_space() {
        let service = TextConversionService::new();
        let config = ConversionConfig::default();
        let result = service.full_to_half("Ａ　Ｂ", &config);
        assert_eq!(result, "A B");
    }

    #[test]
    fn test_half_to_full_space() {
        let service = TextConversionService::new();
        let config = ConversionConfig::default();
        let result = service.half_to_full("A B", &config);
        assert_eq!(result, "Ａ　Ｂ");
    }

    #[test]
    fn test_auto_convert_full_to_half() {
        let service = TextConversionService::new();
        let config = ConversionConfig {
            conversion_type: ConversionType::Auto,
            ..Default::default()
        };
        let result = service.convert_text("ＡＢＣ", &config);
        assert_eq!(result, "ABC");
    }

    #[test]
    fn test_auto_convert_half_to_full() {
        let service = TextConversionService::new();
        let config = ConversionConfig {
            conversion_type: ConversionType::Auto,
            ..Default::default()
        };
        let result = service.convert_text("ABC", &config);
        assert_eq!(result, "ＡＢＣ");
    }

    #[test]
    fn test_has_full_width() {
        let service = TextConversionService::new();
        assert!(service.has_full_width("ＡＢＣ"));
        assert!(!service.has_full_width("ABC"));
    }

    #[test]
    fn test_has_half_width() {
        let service = TextConversionService::new();
        assert!(service.has_half_width("ABC"));
        assert!(!service.has_half_width("ＡＢＣ"));
    }

    #[test]
    fn test_get_char_stats() {
        let service = TextConversionService::new();
        let stats = service.get_char_stats("ＡＢＣ123");
        assert_eq!(stats.full_width, 3);
        assert_eq!(stats.half_width, 3);
        assert_eq!(stats.total, 6);
    }

    #[test]
    fn test_empty_text() {
        let service = TextConversionService::new();
        let config = ConversionConfig::default();
        let result = service.convert_text("", &config);
        assert_eq!(result, "");
    }

    #[test]
    fn test_preserve_newlines() {
        let service = TextConversionService::new();
        let config = ConversionConfig {
            preserve_newlines: true,
            ..Default::default()
        };
        let result = service.full_to_half("Ａ\nＢ", &config);
        assert!(result.contains('\n'));
    }

    #[test]
    fn test_config_validation() {
        let config = ConversionConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_max_text_length() {
        let service = TextConversionService::new();
        let config = ConversionConfig::default();
        let large_text = "Ａ".repeat(MAX_TEXT_LENGTH + 1);
        let result = service.convert_text(&large_text, &config);
        // Should return original text when exceeding max size
        assert_eq!(result.len(), large_text.len());
    }

    #[test]
    fn test_special_characters() {
        let service = TextConversionService::new();
        let config = ConversionConfig::default();
        let result = service.full_to_half("＠＃＄％＆", &config);
        assert_eq!(result, "@#$%&");
    }

    #[test]
    fn test_mixed_characters() {
        let service = TextConversionService::new();
        let config = ConversionConfig::default();
        let result = service.full_to_half("ＡＢＣ123", &config);
        assert_eq!(result, "ABC123");
    }

    #[test]
    fn test_unicode_characters() {
        let service = TextConversionService::new();
        let config = ConversionConfig::default();
        let result = service.full_to_half("ＡＢＣ中文", &config);
        assert!(result.contains("ABC"));
        assert!(result.contains("中文"));
    }

    #[test]
    fn test_performance_large_text() {
        let service = TextConversionService::new();
        let config = ConversionConfig::default();
        let large_text = "Ａ".repeat(100_000);
        let result = service.convert_text(&large_text, &config);
        assert_eq!(result.len(), 100_000);
    }

    #[test]
    fn test_no_preserve_spaces() {
        let service = TextConversionService::new();
        let config = ConversionConfig {
            preserve_spaces: false,
            ..Default::default()
        };
        let result = service.full_to_half("Ａ　Ｂ", &config);
        assert_eq!(result, "AB");
    }

    #[test]
    fn test_no_preserve_newlines() {
        let service = TextConversionService::new();
        let config = ConversionConfig {
            preserve_newlines: false,
            ..Default::default()
        };
        let result = service.full_to_half("Ａ\nＢ", &config);
        assert!(!result.contains('\n'));
    }
}
