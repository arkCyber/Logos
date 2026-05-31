use super::config::TipTapConfig;

/// TipTap configuration presets for different use cases
pub struct TipTapPresets;

impl TipTapPresets {
    /// Get preset configuration by name
    pub fn get_preset(name: &str) -> Option<TipTapConfig> {
        match name {
            "minimal" => Some(TipTapConfig::minimal()),
            "full" => Some(TipTapConfig::full()),
            "default" => Some(TipTapConfig::default()),
            "presentation" => Some(Self::presentation()),
            "document" => Some(Self::document()),
            "code" => Some(Self::code()),
            _ => None,
        }
    }

    /// Get all available preset names
    pub fn list_presets() -> Vec<String> {
        vec![
            "minimal".to_string(),
            "default".to_string(),
            "full".to_string(),
            "presentation".to_string(),
            "document".to_string(),
            "code".to_string(),
        ]
    }

    /// Presentation mode preset (optimized for slides)
    fn presentation() -> TipTapConfig {
        let mut config = TipTapConfig::default();
        config.extensions.text_align.types = vec!["heading".to_string()];
        config.extensions.starter_kit.horizontal_rule = true;
        config.editor.auto_save = false;
        config
    }

    /// Document mode preset (optimized for long-form writing)
    fn document() -> TipTapConfig {
        let mut config = TipTapConfig::full();
        config.editor.auto_save_interval = 60000; // 1 minute
        config.editor.spell_check = true;
        config
    }

    /// Code mode preset (optimized for code editing)
    fn code() -> TipTapConfig {
        let mut config = TipTapConfig::default();
        config.extensions.code_block.syntax_highlighting = true;
        config.extensions.code_block.default_language = Some("rust".to_string());
        config.extensions.starter_kit.heading = false;
        config.extensions.table.resizable = false;
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_preset_minimal() {
        let preset = TipTapPresets::get_preset("minimal");
        assert!(preset.is_some());
        let config = preset.unwrap();
        assert!(!config.extensions.code_block.syntax_highlighting);
    }

    #[test]
    fn test_get_preset_full() {
        let preset = TipTapPresets::get_preset("full");
        assert!(preset.is_some());
        let config = preset.unwrap();
        assert!(config.extensions.starter_kit.link);
    }

    #[test]
    fn test_get_preset_default() {
        let preset = TipTapPresets::get_preset("default");
        assert!(preset.is_some());
        let config = preset.unwrap();
        assert!(config.editor.auto_save);
    }

    #[test]
    fn test_get_preset_presentation() {
        let preset = TipTapPresets::get_preset("presentation");
        assert!(preset.is_some());
        let config = preset.unwrap();
        assert!(!config.editor.auto_save);
        assert_eq!(config.extensions.text_align.types.len(), 1);
    }

    #[test]
    fn test_get_preset_document() {
        let preset = TipTapPresets::get_preset("document");
        assert!(preset.is_some());
        let config = preset.unwrap();
        assert_eq!(config.editor.auto_save_interval, 60000);
        assert!(config.editor.spell_check);
    }

    #[test]
    fn test_get_preset_code() {
        let preset = TipTapPresets::get_preset("code");
        assert!(preset.is_some());
        let config = preset.unwrap();
        assert!(config.extensions.code_block.syntax_highlighting);
        assert_eq!(
            config.extensions.code_block.default_language,
            Some("rust".to_string())
        );
        assert!(!config.extensions.starter_kit.heading);
    }

    #[test]
    fn test_get_preset_invalid() {
        let preset = TipTapPresets::get_preset("invalid_preset");
        assert!(preset.is_none());
    }

    #[test]
    fn test_get_preset_empty_string() {
        let preset = TipTapPresets::get_preset("");
        assert!(preset.is_none());
    }

    #[test]
    fn test_list_presets() {
        let presets = TipTapPresets::list_presets();
        assert_eq!(presets.len(), 6);
        assert!(presets.contains(&"minimal".to_string()));
        assert!(presets.contains(&"default".to_string()));
        assert!(presets.contains(&"full".to_string()));
        assert!(presets.contains(&"presentation".to_string()));
        assert!(presets.contains(&"document".to_string()));
        assert!(presets.contains(&"code".to_string()));
    }

    #[test]
    fn test_list_presets_order() {
        let presets = TipTapPresets::list_presets();
        assert_eq!(presets[0], "minimal");
        assert_eq!(presets[1], "default");
        assert_eq!(presets[2], "full");
    }

    #[test]
    fn test_presentation_preset_config() {
        let config = TipTapPresets::presentation();
        assert_eq!(config.extensions.text_align.types, vec!["heading"]);
        assert!(config.extensions.starter_kit.horizontal_rule);
        assert!(!config.editor.auto_save);
    }

    #[test]
    fn test_document_preset_config() {
        let config = TipTapPresets::document();
        assert_eq!(config.editor.auto_save_interval, 60000);
        assert!(config.editor.spell_check);
        assert!(config.extensions.starter_kit.link);
    }

    #[test]
    fn test_code_preset_config() {
        let config = TipTapPresets::code();
        assert!(config.extensions.code_block.syntax_highlighting);
        assert_eq!(
            config.extensions.code_block.default_language,
            Some("rust".to_string())
        );
        assert!(!config.extensions.starter_kit.heading);
        assert!(!config.extensions.table.resizable);
    }

    #[test]
    fn test_all_presets_return_some() {
        let presets = TipTapPresets::list_presets();
        for preset_name in presets {
            let preset = TipTapPresets::get_preset(&preset_name);
            assert!(
                preset.is_some(),
                "Preset {} should return Some",
                preset_name
            );
        }
    }

    #[test]
    fn test_preset_case_sensitivity() {
        let preset = TipTapPresets::get_preset("Minimal");
        assert!(preset.is_none());

        let preset = TipTapPresets::get_preset("MINIMAL");
        assert!(preset.is_none());
    }

    #[test]
    fn test_presentation_preset_no_auto_save() {
        let config = TipTapPresets::presentation();
        assert!(!config.editor.auto_save);
    }

    #[test]
    fn test_document_preset_longer_interval() {
        let config = TipTapPresets::document();
        assert!(
            config.editor.auto_save_interval > TipTapConfig::default().editor.auto_save_interval
        );
    }

    #[test]
    fn test_code_preset_rust_language() {
        let config = TipTapPresets::code();
        assert_eq!(
            config.extensions.code_block.default_language,
            Some("rust".to_string())
        );
    }

    #[test]
    fn test_code_preset_no_heading() {
        let config = TipTapPresets::code();
        assert!(!config.extensions.starter_kit.heading);
    }

    #[test]
    fn test_code_preset_non_resizable_table() {
        let config = TipTapPresets::code();
        assert!(!config.extensions.table.resizable);
    }

    #[test]
    fn test_presentation_preset_single_align_type() {
        let config = TipTapPresets::presentation();
        assert_eq!(config.extensions.text_align.types.len(), 1);
    }

    #[test]
    fn test_document_preset_spell_check_enabled() {
        let config = TipTapPresets::document();
        assert!(config.editor.spell_check);
    }

    #[test]
    fn test_minimal_vs_presentation() {
        let minimal = TipTapPresets::get_preset("minimal").unwrap();
        let presentation = TipTapPresets::get_preset("presentation").unwrap();

        assert!(!minimal.editor.auto_save);
        assert!(!presentation.editor.auto_save);

        assert_ne!(
            minimal.extensions.text_align.types,
            presentation.extensions.text_align.types
        );
    }

    #[test]
    fn test_full_vs_document() {
        let full = TipTapPresets::get_preset("full").unwrap();
        let document = TipTapPresets::get_preset("document").unwrap();

        assert!(full.editor.spell_check);
        assert!(document.editor.spell_check);

        assert_ne!(
            full.editor.auto_save_interval,
            document.editor.auto_save_interval
        );
    }

    #[test]
    fn test_default_vs_code() {
        let default = TipTapPresets::get_preset("default").unwrap();
        let code = TipTapPresets::get_preset("code").unwrap();

        assert!(default.extensions.starter_kit.heading);
        assert!(!code.extensions.starter_kit.heading);
    }
}
