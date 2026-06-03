#[cfg(test)]
mod integration_tests {
    use super::*;
    use super::super::converter::{TextConversionService, ConversionConfig, ConversionType};

    #[test]
    fn test_conversion_service_creation() {
        let service = TextConversionService::new();
        let _ = service;
    }

    #[test]
    fn test_default_service() {
        let service = TextConversionService::default();
        let _ = service;
    }

    #[test]
    fn test_full_to_half_conversion() {
        let service = TextConversionService::new();
        let config = ConversionConfig {
            conversion_type: ConversionType::FullToHalf,
            ..Default::default()
        };
        let result = service.convert_text("ＡＢＣ", &config);
        assert_eq!(result, "ABC");
    }

    #[test]
    fn test_half_to_full_conversion() {
        let service = TextConversionService::new();
        let config = ConversionConfig {
            conversion_type: ConversionType::HalfToFull,
            ..Default::default()
        };
        let result = service.convert_text("ABC", &config);
        assert_eq!(result, "ＡＢＣ");
    }

    #[test]
    fn test_mixed_text_conversion() {
        let service = TextConversionService::new();
        let config = ConversionConfig {
            conversion_type: ConversionType::FullToHalf,
            ..Default::default()
        };
        let result = service.convert_text("ＡＢＣ123", &config);
        assert_eq!(result, "ABC123");
    }

    #[test]
    fn test_large_text_conversion() {
        let service = TextConversionService::new();
        let config = ConversionConfig::default();
        let large_text = "Ａ".repeat(1000);
        let result = service.convert_text(&large_text, &config);
        assert_eq!(result.len(), 1000);
    }

    #[test]
    fn test_special_characters() {
        let service = TextConversionService::new();
        let config = ConversionConfig::default();
        let result = service.convert_text("＠＃＄", &config);
        assert_eq!(result, "@#$");
    }

    #[test]
    fn test_char_stats_accuracy() {
        let service = TextConversionService::new();
        let stats = service.get_char_stats("ＡＢＣ123!@#");
        assert_eq!(stats.full_width, 3);
        assert_eq!(stats.half_width, 6);
        assert_eq!(stats.other, 0);
        assert_eq!(stats.total, 9);
    }
}
