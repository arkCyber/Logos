use serde::{Deserialize, Serialize};

/// TipTap editor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TipTapConfig {
    /// Enable/disable specific extensions
    pub extensions: ExtensionConfig,
    /// Editor behavior settings
    pub editor: EditorConfig,
}

/// Extension configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionConfig {
    /// StarterKit configuration
    pub starter_kit: StarterKitConfig,
    /// Table configuration
    pub table: TableConfig,
    /// Image configuration
    pub image: ImageConfig,
    /// Link configuration
    pub link: LinkConfig,
    /// Text alignment configuration
    pub text_align: TextAlignConfig,
    /// Code block configuration
    pub code_block: CodeBlockConfig,
}

/// StarterKit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarterKitConfig {
    /// Enable/disable link extension
    pub link: bool,
    /// Enable/disable code block extension
    pub code_block: bool,
    /// Enable/disable history extension
    pub history: bool,
    /// Enable/disable heading extension
    pub heading: bool,
    /// Enable/disable horizontal rule extension
    pub horizontal_rule: bool,
}

/// Table configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableConfig {
    /// Enable resizable tables
    pub resizable: bool,
    /// Enable table header
    pub header: bool,
}

/// Image configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    /// Allow inline images
    pub inline: bool,
    /// Allow base64 encoded images
    pub allow_base64: bool,
    /// Maximum image size in bytes
    pub max_size: Option<usize>,
}

/// Link configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkConfig {
    /// Open link on click
    pub open_on_click: bool,
    /// Auto-link URLs
    pub autolink: bool,
}

/// Text alignment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextAlignConfig {
    /// Types that support text alignment
    pub types: Vec<String>,
    /// Default alignment
    pub default: Option<String>,
}

/// Code block configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlockConfig {
    /// Enable syntax highlighting
    pub syntax_highlighting: bool,
    /// Default language
    pub default_language: Option<String>,
}

/// Editor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    /// Enable auto-save
    pub auto_save: bool,
    /// Auto-save interval in milliseconds
    pub auto_save_interval: u64,
    /// Enable spell check
    pub spell_check: bool,
    /// Enable character count
    pub character_count: bool,
    /// Enable word count
    pub word_count: bool,
}

impl Default for TipTapConfig {
    fn default() -> Self {
        Self {
            extensions: ExtensionConfig {
                starter_kit: StarterKitConfig {
                    link: false,
                    code_block: false,
                    history: true,
                    heading: true,
                    horizontal_rule: true,
                },
                table: TableConfig {
                    resizable: true,
                    header: true,
                },
                image: ImageConfig {
                    inline: true,
                    allow_base64: true,
                    max_size: Some(5 * 1024 * 1024), // 5MB
                },
                link: LinkConfig {
                    open_on_click: false,
                    autolink: true,
                },
                text_align: TextAlignConfig {
                    types: vec!["heading".to_string(), "paragraph".to_string()],
                    default: None,
                },
                code_block: CodeBlockConfig {
                    syntax_highlighting: true,
                    default_language: Some("auto".to_string()),
                },
            },
            editor: EditorConfig {
                auto_save: true,
                auto_save_interval: 30000, // 30 seconds
                spell_check: false,
                character_count: true,
                word_count: true,
            },
        }
    }
}

impl TipTapConfig {
    /// Create a new TipTap configuration with custom settings
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a minimal configuration (for performance)
    pub fn minimal() -> Self {
        let mut config = Self::default();
        config.extensions.code_block.syntax_highlighting = false;
        config.extensions.table.resizable = false;
        config.editor.auto_save = false;
        config
    }

    /// Create a full-featured configuration
    pub fn full() -> Self {
        let mut config = Self::default();
        config.extensions.starter_kit.link = true;
        config.extensions.starter_kit.code_block = true;
        config.editor.spell_check = true;
        config
    }

    /// Convert to JSON for frontend consumption
    #[allow(dead_code)]
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Parse from JSON
    #[allow(dead_code)]
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiptap_config_default() {
        let config = TipTapConfig::default();
        assert!(config.extensions.starter_kit.history);
        assert!(config.extensions.starter_kit.heading);
        assert!(config.extensions.table.resizable);
        assert!(config.extensions.image.inline);
        assert!(config.editor.auto_save);
    }

    #[test]
    fn test_tiptap_config_new() {
        let config = TipTapConfig::new();
        assert!(config.extensions.starter_kit.history);
        assert!(config.editor.auto_save);
    }

    #[test]
    fn test_tiptap_config_minimal() {
        let config = TipTapConfig::minimal();
        assert!(!config.extensions.code_block.syntax_highlighting);
        assert!(!config.extensions.table.resizable);
        assert!(!config.editor.auto_save);
    }

    #[test]
    fn test_tiptap_config_full() {
        let config = TipTapConfig::full();
        assert!(config.extensions.starter_kit.link);
        assert!(config.extensions.starter_kit.code_block);
        assert!(config.editor.spell_check);
    }

    #[test]
    fn test_tiptap_config_to_json() {
        let config = TipTapConfig::default();
        let json = config.to_json();
        assert!(json.is_ok());
    }

    #[test]
    fn test_tiptap_config_from_json() {
        let config = TipTapConfig::default();
        let json = config.to_json().unwrap();
        let parsed = TipTapConfig::from_json(&json);
        assert!(parsed.is_ok());
    }

    #[test]
    fn test_extension_config_creation() {
        let extensions = ExtensionConfig {
            starter_kit: StarterKitConfig {
                link: true,
                code_block: true,
                history: true,
                heading: true,
                horizontal_rule: true,
            },
            table: TableConfig {
                resizable: true,
                header: true,
            },
            image: ImageConfig {
                inline: true,
                allow_base64: true,
                max_size: Some(1024),
            },
            link: LinkConfig {
                open_on_click: true,
                autolink: true,
            },
            text_align: TextAlignConfig {
                types: vec!["heading".to_string()],
                default: Some("left".to_string()),
            },
            code_block: CodeBlockConfig {
                syntax_highlighting: true,
                default_language: Some("rust".to_string()),
            },
        };
        assert!(extensions.starter_kit.link);
        assert!(extensions.table.resizable);
    }

    #[test]
    fn test_starter_kit_config() {
        let config = StarterKitConfig {
            link: true,
            code_block: false,
            history: true,
            heading: true,
            horizontal_rule: false,
        };
        assert!(config.link);
        assert!(!config.code_block);
    }

    #[test]
    fn test_table_config() {
        let config = TableConfig {
            resizable: true,
            header: false,
        };
        assert!(config.resizable);
        assert!(!config.header);
    }

    #[test]
    fn test_image_config() {
        let config = ImageConfig {
            inline: true,
            allow_base64: false,
            max_size: Some(2048),
        };
        assert!(config.inline);
        assert!(!config.allow_base64);
        assert_eq!(config.max_size, Some(2048));
    }

    #[test]
    fn test_image_config_no_max_size() {
        let config = ImageConfig {
            inline: true,
            allow_base64: true,
            max_size: None,
        };
        assert!(config.max_size.is_none());
    }

    #[test]
    fn test_link_config() {
        let config = LinkConfig {
            open_on_click: true,
            autolink: false,
        };
        assert!(config.open_on_click);
        assert!(!config.autolink);
    }

    #[test]
    fn test_text_align_config() {
        let config = TextAlignConfig {
            types: vec!["heading".to_string(), "paragraph".to_string()],
            default: Some("center".to_string()),
        };
        assert_eq!(config.types.len(), 2);
        assert_eq!(config.default, Some("center".to_string()));
    }

    #[test]
    fn test_text_align_config_no_default() {
        let config = TextAlignConfig {
            types: vec!["heading".to_string()],
            default: None,
        };
        assert!(config.default.is_none());
    }

    #[test]
    fn test_code_block_config() {
        let config = CodeBlockConfig {
            syntax_highlighting: true,
            default_language: Some("javascript".to_string()),
        };
        assert!(config.syntax_highlighting);
        assert_eq!(config.default_language, Some("javascript".to_string()));
    }

    #[test]
    fn test_code_block_config_no_language() {
        let config = CodeBlockConfig {
            syntax_highlighting: false,
            default_language: None,
        };
        assert!(!config.syntax_highlighting);
        assert!(config.default_language.is_none());
    }

    #[test]
    fn test_editor_config() {
        let config = EditorConfig {
            auto_save: true,
            auto_save_interval: 60000,
            spell_check: true,
            character_count: true,
            word_count: false,
        };
        assert!(config.auto_save);
        assert_eq!(config.auto_save_interval, 60000);
        assert!(config.spell_check);
        assert!(!config.word_count);
    }

    #[test]
    fn test_tiptap_config_serialization() {
        let config = TipTapConfig::default();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_tiptap_config_deserialization() {
        let json = r#"{
            "extensions": {
                "starter_kit": {
                    "link": false,
                    "code_block": false,
                    "history": true,
                    "heading": true,
                    "horizontal_rule": true
                },
                "table": {
                    "resizable": true,
                    "header": true
                },
                "image": {
                    "inline": true,
                    "allow_base64": true,
                    "max_size": 5242880
                },
                "link": {
                    "open_on_click": false,
                    "autolink": true
                },
                "text_align": {
                    "types": ["heading", "paragraph"],
                    "default": null
                },
                "code_block": {
                    "syntax_highlighting": true,
                    "default_language": "auto"
                }
            },
            "editor": {
                "auto_save": true,
                "auto_save_interval": 30000,
                "spell_check": false,
                "character_count": true,
                "word_count": true
            }
        }"#;
        let config: Result<TipTapConfig, _> = serde_json::from_str(json);
        assert!(config.is_ok());
    }

    #[test]
    fn test_tiptap_config_clone() {
        let config = TipTapConfig::default();
        let cloned = config.clone();
        assert_eq!(config.editor.auto_save, cloned.editor.auto_save);
    }

    #[test]
    fn test_extension_config_serialization() {
        let extensions = ExtensionConfig {
            starter_kit: StarterKitConfig {
                link: true,
                code_block: true,
                history: true,
                heading: true,
                horizontal_rule: true,
            },
            table: TableConfig {
                resizable: true,
                header: true,
            },
            image: ImageConfig {
                inline: true,
                allow_base64: true,
                max_size: Some(1024),
            },
            link: LinkConfig {
                open_on_click: true,
                autolink: true,
            },
            text_align: TextAlignConfig {
                types: vec!["heading".to_string()],
                default: Some("left".to_string()),
            },
            code_block: CodeBlockConfig {
                syntax_highlighting: true,
                default_language: Some("rust".to_string()),
            },
        };
        let json = serde_json::to_string(&extensions);
        assert!(json.is_ok());
    }

    #[test]
    fn test_starter_kit_config_serialization() {
        let config = StarterKitConfig {
            link: true,
            code_block: true,
            history: true,
            heading: true,
            horizontal_rule: true,
        };
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_table_config_serialization() {
        let config = TableConfig {
            resizable: true,
            header: true,
        };
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_image_config_serialization() {
        let config = ImageConfig {
            inline: true,
            allow_base64: true,
            max_size: Some(1024),
        };
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_link_config_serialization() {
        let config = LinkConfig {
            open_on_click: true,
            autolink: true,
        };
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_text_align_config_serialization() {
        let config = TextAlignConfig {
            types: vec!["heading".to_string()],
            default: Some("left".to_string()),
        };
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_code_block_config_serialization() {
        let config = CodeBlockConfig {
            syntax_highlighting: true,
            default_language: Some("rust".to_string()),
        };
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_editor_config_serialization() {
        let config = EditorConfig {
            auto_save: true,
            auto_save_interval: 30000,
            spell_check: false,
            character_count: true,
            word_count: true,
        };
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_tiptap_config_invalid_json() {
        let json = "{ invalid }";
        let result = TipTapConfig::from_json(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_tiptap_config_minimal_vs_full() {
        let minimal = TipTapConfig::minimal();
        let full = TipTapConfig::full();

        assert!(!minimal.extensions.code_block.syntax_highlighting);
        assert!(full.extensions.code_block.syntax_highlighting);

        assert!(!minimal.editor.spell_check);
        assert!(full.editor.spell_check);
    }

    #[test]
    fn test_image_config_large_max_size() {
        let config = ImageConfig {
            inline: true,
            allow_base64: true,
            max_size: Some(100 * 1024 * 1024), // 100MB
        };
        assert_eq!(config.max_size, Some(104857600));
    }

    #[test]
    fn test_editor_config_zero_interval() {
        let config = EditorConfig {
            auto_save: true,
            auto_save_interval: 0,
            spell_check: false,
            character_count: false,
            word_count: false,
        };
        assert_eq!(config.auto_save_interval, 0);
    }

    #[test]
    fn test_text_align_config_empty_types() {
        let config = TextAlignConfig {
            types: vec![],
            default: None,
        };
        assert!(config.types.is_empty());
    }

    #[test]
    fn test_text_align_config_multiple_types() {
        let config = TextAlignConfig {
            types: vec![
                "heading".to_string(),
                "paragraph".to_string(),
                "listItem".to_string(),
            ],
            default: Some("left".to_string()),
        };
        assert_eq!(config.types.len(), 3);
    }
}
