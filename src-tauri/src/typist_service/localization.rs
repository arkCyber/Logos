/*!
 * 航空航天级本地化系统
 * 实现 Typst 的本地化功能（语言设置、本地化字符串、区域设置）
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 支持的语言
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    English,
    German,
    French,
    Spanish,
    Chinese,
    Japanese,
    Korean,
    Russian,
    Arabic,
    Custom(String),
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::English => write!(f, "en"),
            Language::German => write!(f, "de"),
            Language::French => write!(f, "fr"),
            Language::Spanish => write!(f, "es"),
            Language::Chinese => write!(f, "zh"),
            Language::Japanese => write!(f, "ja"),
            Language::Korean => write!(f, "ko"),
            Language::Russian => write!(f, "ru"),
            Language::Arabic => write!(f, "ar"),
            Language::Custom(code) => write!(f, "{}", code),
        }
    }
}

/// 本地化配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizationConfig {
    pub language: Language,
    pub region: Option<String>,
}

impl Default for LocalizationConfig {
    fn default() -> Self {
        Self {
            language: Language::English,
            region: None,
        }
    }
}

/// 本地化系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Localization {
    pub config: LocalizationConfig,
    pub translations: HashMap<String, HashMap<String, String>>,
}

impl Localization {
    pub fn new() -> Self {
        let mut translations = HashMap::new();

        // 默认翻译
        let mut en = HashMap::new();
        en.insert("figure".to_string(), "Figure".to_string());
        en.insert("table".to_string(), "Table".to_string());
        en.insert("equation".to_string(), "Equation".to_string());
        en.insert("theorem".to_string(), "Theorem".to_string());
        en.insert("lemma".to_string(), "Lemma".to_string());
        en.insert("definition".to_string(), "Definition".to_string());
        en.insert("example".to_string(), "Example".to_string());
        en.insert("remark".to_string(), "Remark".to_string());
        en.insert("proof".to_string(), "Proof".to_string());
        en.insert("chapter".to_string(), "Chapter".to_string());
        en.insert("section".to_string(), "Section".to_string());
        en.insert("appendix".to_string(), "Appendix".to_string());
        en.insert("bibliography".to_string(), "Bibliography".to_string());
        en.insert("glossary".to_string(), "Glossary".to_string());
        en.insert("index".to_string(), "Index".to_string());
        en.insert("contents".to_string(), "Contents".to_string());
        en.insert("page".to_string(), "Page".to_string());
        translations.insert("en".to_string(), en);

        let mut de = HashMap::new();
        de.insert("figure".to_string(), "Abbildung".to_string());
        de.insert("table".to_string(), "Tabelle".to_string());
        de.insert("equation".to_string(), "Gleichung".to_string());
        de.insert("theorem".to_string(), "Satz".to_string());
        de.insert("lemma".to_string(), "Lemma".to_string());
        de.insert("definition".to_string(), "Definition".to_string());
        de.insert("example".to_string(), "Beispiel".to_string());
        de.insert("remark".to_string(), "Bemerkung".to_string());
        de.insert("proof".to_string(), "Beweis".to_string());
        de.insert("chapter".to_string(), "Kapitel".to_string());
        de.insert("section".to_string(), "Abschnitt".to_string());
        de.insert("appendix".to_string(), "Anhang".to_string());
        de.insert(
            "bibliography".to_string(),
            "Literaturverzeichnis".to_string(),
        );
        de.insert("glossary".to_string(), "Glossar".to_string());
        de.insert("index".to_string(), "Index".to_string());
        de.insert("contents".to_string(), "Inhalt".to_string());
        de.insert("page".to_string(), "Seite".to_string());
        translations.insert("de".to_string(), de);

        let mut fr = HashMap::new();
        fr.insert("figure".to_string(), "Figure".to_string());
        fr.insert("table".to_string(), "Tableau".to_string());
        fr.insert("equation".to_string(), "Équation".to_string());
        fr.insert("theorem".to_string(), "Théorème".to_string());
        fr.insert("lemma".to_string(), "Lemme".to_string());
        fr.insert("definition".to_string(), "Définition".to_string());
        fr.insert("example".to_string(), "Exemple".to_string());
        fr.insert("remark".to_string(), "Remarque".to_string());
        fr.insert("proof".to_string(), "Preuve".to_string());
        fr.insert("chapter".to_string(), "Chapitre".to_string());
        fr.insert("section".to_string(), "Section".to_string());
        fr.insert("appendix".to_string(), "Annexe".to_string());
        fr.insert("bibliography".to_string(), "Bibliographie".to_string());
        fr.insert("glossary".to_string(), "Glossaire".to_string());
        fr.insert("index".to_string(), "Index".to_string());
        fr.insert("contents".to_string(), "Table des matières".to_string());
        fr.insert("page".to_string(), "Page".to_string());
        translations.insert("fr".to_string(), fr);

        let mut zh = HashMap::new();
        zh.insert("figure".to_string(), "图".to_string());
        zh.insert("table".to_string(), "表".to_string());
        zh.insert("equation".to_string(), "方程".to_string());
        zh.insert("theorem".to_string(), "定理".to_string());
        zh.insert("lemma".to_string(), "引理".to_string());
        zh.insert("definition".to_string(), "定义".to_string());
        zh.insert("example".to_string(), "示例".to_string());
        zh.insert("remark".to_string(), "备注".to_string());
        zh.insert("proof".to_string(), "证明".to_string());
        zh.insert("chapter".to_string(), "章".to_string());
        zh.insert("section".to_string(), "节".to_string());
        zh.insert("appendix".to_string(), "附录".to_string());
        zh.insert("bibliography".to_string(), "参考文献".to_string());
        zh.insert("glossary".to_string(), "术语表".to_string());
        zh.insert("index".to_string(), "索引".to_string());
        zh.insert("contents".to_string(), "目录".to_string());
        zh.insert("page".to_string(), "页".to_string());
        translations.insert("zh".to_string(), zh);

        Self {
            config: LocalizationConfig::default(),
            translations,
        }
    }

    pub fn with_config(mut self, config: LocalizationConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_language(mut self, language: Language) -> Self {
        self.config.language = language;
        self
    }

    pub fn with_region(mut self, region: String) -> Self {
        self.config.region = Some(region);
        self
    }

    pub fn add_translation(mut self, lang: String, key: String, value: String) -> Self {
        self.translations
            .entry(lang)
            .or_default()
            .insert(key, value);
        self
    }

    /// 获取翻译
    pub fn get(&self, key: &str) -> String {
        let lang_code = self.config.language.to_string();
        self.translations
            .get(&lang_code)
            .and_then(|lang_map| lang_map.get(key))
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        typst.push_str("#set text(");
        typst.push_str(&format!("lang: \"{}\"", self.config.language));

        if let Some(region) = &self.config.region {
            typst.push_str(&format!(", region: \"{}\"", region));
        }

        typst.push_str(")\n");

        typst
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        html.push_str(&format!(
            "<html lang=\"{}\"{}>\n",
            self.config.language,
            if let Some(region) = &self.config.region {
                format!(" data-region=\"{}\"", region)
            } else {
                String::new()
            }
        ));

        html
    }
}

impl Default for Localization {
    fn default() -> Self {
        Self::new()
    }
}

/// 本地化构建器
pub struct LocalizationBuilder {
    localization: Localization,
}

impl LocalizationBuilder {
    pub fn new() -> Self {
        Self {
            localization: Localization::new(),
        }
    }

    pub fn language(mut self, language: Language) -> Self {
        self.localization = self.localization.with_language(language);
        self
    }

    pub fn region(mut self, region: String) -> Self {
        self.localization = self.localization.with_region(region);
        self
    }

    pub fn translation(mut self, lang: String, key: String, value: String) -> Self {
        self.localization = self.localization.add_translation(lang, key, value);
        self
    }

    pub fn build(self) -> Localization {
        self.localization
    }
}

impl Default for LocalizationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_localization_creation() {
        let localization = Localization::new();
        assert!(matches!(localization.config.language, Language::English));
    }

    #[test]
    fn test_localization_default() {
        let localization = Localization::default();
        assert!(matches!(localization.config.language, Language::English));
    }

    #[test]
    fn test_localization_config_default() {
        let config = LocalizationConfig::default();
        assert!(matches!(config.language, Language::English));
    }

    #[test]
    fn test_localization_with_language() {
        let localization = Localization::new().with_language(Language::German);
        assert!(matches!(localization.config.language, Language::German));
    }

    #[test]
    fn test_localization_with_region() {
        let localization = Localization::new().with_region("US".to_string());
        assert_eq!(localization.config.region, Some("US".to_string()));
    }

    #[test]
    fn test_language_to_string() {
        assert_eq!(Language::English.to_string(), "en");
        assert_eq!(Language::German.to_string(), "de");
        assert_eq!(Language::French.to_string(), "fr");
        assert_eq!(Language::Chinese.to_string(), "zh");
    }

    #[test]
    fn test_get_translation() {
        let localization = Localization::new();
        assert_eq!(localization.get("figure"), "Figure");
    }

    #[test]
    fn test_get_translation_german() {
        let localization = Localization::new().with_language(Language::German);
        assert_eq!(localization.get("figure"), "Abbildung");
    }

    #[test]
    fn test_get_translation_chinese() {
        let localization = Localization::new().with_language(Language::Chinese);
        assert_eq!(localization.get("figure"), "图");
    }

    #[test]
    fn test_get_translation_missing() {
        let localization = Localization::new();
        assert_eq!(localization.get("unknown"), "unknown");
    }

    #[test]
    fn test_add_translation() {
        let localization = Localization::new().add_translation(
            "en".to_string(),
            "custom".to_string(),
            "Custom".to_string(),
        );
        assert_eq!(localization.get("custom"), "Custom");
    }

    #[test]
    fn test_to_typst() {
        let localization = Localization::new();
        let typst = localization.to_typst();
        assert!(typst.contains("#set text("));
        assert!(typst.contains("lang: \"en\""));
    }

    #[test]
    fn test_to_typst_with_region() {
        let localization = Localization::new().with_region("US".to_string());
        let typst = localization.to_typst();
        assert!(typst.contains("region: \"US\""));
    }

    #[test]
    fn test_to_typst_german() {
        let localization = Localization::new().with_language(Language::German);
        let typst = localization.to_typst();
        assert!(typst.contains("lang: \"de\""));
    }

    #[test]
    fn test_to_html() {
        let localization = Localization::new();
        let html = localization.to_html();
        assert!(html.contains("<html lang=\"en\""));
    }

    #[test]
    fn test_to_html_with_region() {
        let localization = Localization::new().with_region("US".to_string());
        let html = localization.to_html();
        assert!(html.contains("data-region=\"US\""));
    }

    #[test]
    fn test_localization_builder() {
        let localization = LocalizationBuilder::new()
            .language(Language::German)
            .region("DE".to_string())
            .build();

        assert!(matches!(localization.config.language, Language::German));
        assert_eq!(localization.config.region, Some("DE".to_string()));
    }

    #[test]
    fn test_localization_builder_default() {
        let builder = LocalizationBuilder::default();
        let localization = builder.build();
        assert!(matches!(localization.config.language, Language::English));
    }

    #[test]
    fn test_language_variants() {
        assert!(matches!(Language::English, Language::English));
        assert!(matches!(
            Language::Custom("test".to_string()),
            Language::Custom(_)
        ));
    }

    #[test]
    fn test_get_all_english_translations() {
        let localization = Localization::new();
        assert_eq!(localization.get("table"), "Table");
        assert_eq!(localization.get("theorem"), "Theorem");
        assert_eq!(localization.get("bibliography"), "Bibliography");
    }

    #[test]
    fn test_get_all_german_translations() {
        let localization = Localization::new().with_language(Language::German);
        assert_eq!(localization.get("table"), "Tabelle");
        assert_eq!(localization.get("theorem"), "Satz");
        assert_eq!(localization.get("bibliography"), "Literaturverzeichnis");
    }

    #[test]
    fn test_get_all_chinese_translations() {
        let localization = Localization::new().with_language(Language::Chinese);
        assert_eq!(localization.get("table"), "表");
        assert_eq!(localization.get("theorem"), "定理");
        assert_eq!(localization.get("bibliography"), "参考文献");
    }
}
