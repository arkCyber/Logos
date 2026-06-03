#[cfg(test)]
mod tests {
    use super::super::processor::{WatermarkService, WatermarkConfig};

    #[test]
    fn test_watermark_service_creation() {
        let service = WatermarkService::new();
        // Service created successfully
    }

    #[test]
    fn test_apply_watermark_basic() {
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
        assert!(result.contains("Content"));
    }

    #[test]
    fn test_remove_watermark() {
        let service = WatermarkService::new();
        let html = r#"<div class="watermark">Test</div><p>Content</p>"#;
        let result = service.remove_watermark(html);
        assert!(!result.contains("watermark"));
        assert!(!result.contains("Test"));
        assert!(result.contains("Content"));
    }

    #[test]
    fn test_remove_multiple_watermarks() {
        let service = WatermarkService::new();
        let html = r#"<div class="watermark">Test1</div><p>Content</p><div class="watermark">Test2</div>"#;
        let result = service.remove_watermark(html);
        assert!(!result.contains("watermark"));
        assert!(result.contains("Content"));
    }

    #[test]
    fn test_disabled_watermark_no_change() {
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
    fn test_empty_text_watermark_no_change() {
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
    fn test_watermark_opacity_value() {
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
    fn test_watermark_rotation_positive() {
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
    fn test_watermark_rotation_negative() {
        let service = WatermarkService::new();
        let html = "<p>Content</p>";
        let config = WatermarkConfig {
            enabled: true,
            text: "Test".to_string(),
            rotation: -45,
            ..Default::default()
        };
        let result = service.apply_watermark(html, &config);
        assert!(result.contains("rotate(-45deg)"));
    }

    #[test]
    fn test_watermark_color_hex() {
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

    #[test]
    fn test_watermark_fixed_position() {
        let service = WatermarkService::new();
        let html = "<p>Content</p>";
        let config = WatermarkConfig {
            enabled: true,
            text: "Test".to_string(),
            ..Default::default()
        };
        let result = service.apply_watermark(html, &config);
        assert!(result.contains("position: fixed"));
        assert!(result.contains("top: 50%"));
        assert!(result.contains("left: 50%"));
    }

    #[test]
    fn test_watermark_pointer_events_none() {
        let service = WatermarkService::new();
        let html = "<p>Content</p>";
        let config = WatermarkConfig {
            enabled: true,
            text: "Test".to_string(),
            ..Default::default()
        };
        let result = service.apply_watermark(html, &config);
        assert!(result.contains("pointer-events: none"));
    }

    #[test]
    fn test_watermark_z_index() {
        let service = WatermarkService::new();
        let html = "<p>Content</p>";
        let config = WatermarkConfig {
            enabled: true,
            text: "Test".to_string(),
            ..Default::default()
        };
        let result = service.apply_watermark(html, &config);
        assert!(result.contains("z-index: 1000"));
    }

    #[test]
    fn test_default_config() {
        let config = WatermarkConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.text, "");
        assert_eq!(config.opacity, 0.3);
        assert_eq!(config.rotation, -45);
        assert_eq!(config.color, "#cccccc");
        assert_eq!(config.font_size, 48);
    }
}
