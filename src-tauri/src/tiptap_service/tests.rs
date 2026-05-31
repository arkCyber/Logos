#[cfg(test)]
mod tests {
    use super::super::{TipTapConfig, TipTapPresets};

    #[test]
    fn test_default_config() {
        let config = TipTapConfig::default();
        assert!(config.extensions.starter_kit.history);
        assert!(!config.extensions.starter_kit.link);
        assert!(config.extensions.table.resizable);
    }

    #[test]
    fn test_minimal_config() {
        let config = TipTapConfig::minimal();
        assert!(!config.extensions.code_block.syntax_highlighting);
        assert!(!config.extensions.table.resizable);
        assert!(!config.editor.auto_save);
    }

    #[test]
    fn test_full_config() {
        let config = TipTapConfig::full();
        assert!(config.extensions.starter_kit.link);
        assert!(config.extensions.starter_kit.code_block);
        assert!(config.editor.spell_check);
    }

    #[test]
    fn test_config_serialization() {
        let config = TipTapConfig::default();
        let json = config.to_json();
        assert!(json.is_ok());

        if let Ok(json_str) = json {
            let deserialized = TipTapConfig::from_json(&json_str);
            assert!(deserialized.is_ok());
        }
    }

    #[test]
    fn test_presentation_preset() {
        let config = TipTapPresets::get_preset("presentation");
        assert!(config.is_some());

        if let Some(preset) = config {
            assert!(!preset.editor.auto_save);
            assert_eq!(preset.extensions.text_align.types.len(), 1);
        }
    }

    #[test]
    fn test_document_preset() {
        let config = TipTapPresets::get_preset("document");
        assert!(config.is_some());

        if let Some(preset) = config {
            assert!(preset.editor.spell_check);
            assert_eq!(preset.editor.auto_save_interval, 60000);
        }
    }

    #[test]
    fn test_code_preset() {
        let config = TipTapPresets::get_preset("code");
        assert!(config.is_some());

        if let Some(preset) = config {
            assert!(!preset.extensions.starter_kit.heading);
            assert!(preset.extensions.code_block.syntax_highlighting);
        }
    }

    #[test]
    fn test_invalid_preset() {
        let config = TipTapPresets::get_preset("invalid");
        assert!(config.is_none());
    }

    #[test]
    fn test_list_presets() {
        let presets = TipTapPresets::list_presets();
        assert!(presets.len() > 0);
        assert!(presets.contains(&"default".to_string()));
        assert!(presets.contains(&"minimal".to_string()));
    }
}
