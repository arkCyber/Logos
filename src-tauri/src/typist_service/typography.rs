/*!
 * 航空航天级高级字体排版系统
 * 实现专业排版美学功能：字体微调、OpenType特性、字体配对
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 字距调整
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KerningPair {
    pub left_char: char,
    pub right_char: char,
    pub adjustment: f64, // in em units
}

/// 字距调整表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KerningTable {
    pub pairs: Vec<KerningPair>,
}

impl KerningTable {
    pub fn new() -> Self {
        Self {
            pairs: Vec::new(),
        }
    }

    pub fn add_pair(&mut self, pair: KerningPair) {
        self.pairs.push(pair);
    }

    pub fn get_adjustment(&self, left: char, right: char) -> f64 {
        self.pairs
            .iter()
            .find(|p| p.left_char == left && p.right_char == right)
            .map(|p| p.adjustment)
            .unwrap_or(0.0)
    }
}

impl Default for KerningTable {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenType 特性类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OpenTypeFeature {
    /// 连字
    Liga,
    /// 历史连字
    Hlig,
    /// 上下文连字
    Clig,
    /// 小型大写字母
    Smcp,
    /// 标题小型大写字母
    C2sc,
    /// 数字样式
    Onum,
    /// 等宽数字
    Tnum,
    /// 表格数字
    Pnum,
    /// 分数
    Frac,
    /// 替代字形
    Salt,
    /// 样式集
    Ss01,
    Ss02,
    Ss03,
    Ss04,
    Ss05,
    Ss06,
    Ss07,
    Ss08,
    Ss09,
    Ss10,
    Ss11,
    Ss12,
    Ss13,
    Ss14,
    Ss15,
    Ss16,
    Ss17,
    Ss18,
    Ss19,
    Ss20,
    /// 上下文替代
    Calt,
    /// 字符变体
    Cv01,
    Cv02,
    Cv03,
    Cv04,
    Cv05,
    Cv06,
    Cv07,
    Cv08,
    Cv09,
    Cv10,
    Cv11,
    Cv12,
    Cv13,
    Cv14,
    Cv15,
    Cv16,
    Cv17,
    Cv18,
    Cv19,
    Cv20,
    /// 装饰
    Ornm,
    /// 花体字
    Swsh,
    /// 标题替代
    Titl,
}

impl OpenTypeFeature {
    pub fn to_string(&self) -> String {
        match self {
            OpenTypeFeature::Liga => "liga".to_string(),
            OpenTypeFeature::Hlig => "hlig".to_string(),
            OpenTypeFeature::Clig => "clig".to_string(),
            OpenTypeFeature::Smcp => "smcp".to_string(),
            OpenTypeFeature::C2sc => "c2sc".to_string(),
            OpenTypeFeature::Onum => "onum".to_string(),
            OpenTypeFeature::Tnum => "tnum".to_string(),
            OpenTypeFeature::Pnum => "pnum".to_string(),
            OpenTypeFeature::Frac => "frac".to_string(),
            OpenTypeFeature::Salt => "salt".to_string(),
            OpenTypeFeature::Ss01 => "ss01".to_string(),
            OpenTypeFeature::Ss02 => "ss02".to_string(),
            OpenTypeFeature::Ss03 => "ss03".to_string(),
            OpenTypeFeature::Ss04 => "ss04".to_string(),
            OpenTypeFeature::Ss05 => "ss05".to_string(),
            OpenTypeFeature::Ss06 => "ss06".to_string(),
            OpenTypeFeature::Ss07 => "ss07".to_string(),
            OpenTypeFeature::Ss08 => "ss08".to_string(),
            OpenTypeFeature::Ss09 => "ss09".to_string(),
            OpenTypeFeature::Ss10 => "ss10".to_string(),
            OpenTypeFeature::Ss11 => "ss11".to_string(),
            OpenTypeFeature::Ss12 => "ss12".to_string(),
            OpenTypeFeature::Ss13 => "ss13".to_string(),
            OpenTypeFeature::Ss14 => "ss14".to_string(),
            OpenTypeFeature::Ss15 => "ss15".to_string(),
            OpenTypeFeature::Ss16 => "ss16".to_string(),
            OpenTypeFeature::Ss17 => "ss17".to_string(),
            OpenTypeFeature::Ss18 => "ss18".to_string(),
            OpenTypeFeature::Ss19 => "ss19".to_string(),
            OpenTypeFeature::Ss20 => "ss20".to_string(),
            OpenTypeFeature::Calt => "calt".to_string(),
            OpenTypeFeature::Cv01 => "cv01".to_string(),
            OpenTypeFeature::Cv02 => "cv02".to_string(),
            OpenTypeFeature::Cv03 => "cv03".to_string(),
            OpenTypeFeature::Cv04 => "cv04".to_string(),
            OpenTypeFeature::Cv05 => "cv05".to_string(),
            OpenTypeFeature::Cv06 => "cv06".to_string(),
            OpenTypeFeature::Cv07 => "cv07".to_string(),
            OpenTypeFeature::Cv08 => "cv08".to_string(),
            OpenTypeFeature::Cv09 => "cv09".to_string(),
            OpenTypeFeature::Cv10 => "cv10".to_string(),
            OpenTypeFeature::Cv11 => "cv11".to_string(),
            OpenTypeFeature::Cv12 => "cv12".to_string(),
            OpenTypeFeature::Cv13 => "cv13".to_string(),
            OpenTypeFeature::Cv14 => "cv14".to_string(),
            OpenTypeFeature::Cv15 => "cv15".to_string(),
            OpenTypeFeature::Cv16 => "cv16".to_string(),
            OpenTypeFeature::Cv17 => "cv17".to_string(),
            OpenTypeFeature::Cv18 => "cv18".to_string(),
            OpenTypeFeature::Cv19 => "cv19".to_string(),
            OpenTypeFeature::Cv20 => "cv20".to_string(),
            OpenTypeFeature::Ornm => "ornm".to_string(),
            OpenTypeFeature::Swsh => "swsh".to_string(),
            OpenTypeFeature::Titl => "titl".to_string(),
        }
    }
}

/// OpenType 特性配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenTypeFeatures {
    pub enabled_features: HashMap<OpenTypeFeature, bool>,
}

impl OpenTypeFeatures {
    pub fn new() -> Self {
        let mut enabled_features = HashMap::new();
        
        // 默认启用常用特性
        enabled_features.insert(OpenTypeFeature::Liga, true);
        enabled_features.insert(OpenTypeFeature::Clig, true);
        enabled_features.insert(OpenTypeFeature::Calt, true);
        
        Self { enabled_features }
    }

    pub fn enable_feature(&mut self, feature: OpenTypeFeature) {
        self.enabled_features.insert(feature, true);
    }

    pub fn disable_feature(&mut self, feature: OpenTypeFeature) {
        self.enabled_features.insert(feature, false);
    }

    pub fn is_enabled(&self, feature: &OpenTypeFeature) -> bool {
        *self.enabled_features.get(feature).unwrap_or(&false)
    }

    pub fn to_typst(&self) -> String {
        let features: Vec<String> = self
            .enabled_features
            .iter()
            .filter(|(_, enabled)| **enabled)
            .map(|(feature, _)| format!("\"{}\"", feature.to_string()))
            .collect();

        if features.is_empty() {
            String::new()
        } else {
            format!("features: ({})", features.join(", "))
        }
    }
}

impl Default for OpenTypeFeatures {
    fn default() -> Self {
        Self::new()
    }
}

/// 字体配对建议
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontPairing {
    pub title_font: String,
    pub body_font: String,
    pub mono_font: String,
    pub description: String,
}

/// 字体配对系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontPairingSystem {
    pub pairings: Vec<FontPairing>,
}

impl FontPairingSystem {
    pub fn new() -> Self {
        let mut pairings = Vec::new();
        
        // 经典配对
        pairings.push(FontPairing {
            title_font: "New Computer Modern Serif".to_string(),
            body_font: "New Computer Modern Sans".to_string(),
            mono_font: "New Computer Modern Mono".to_string(),
            description: "Modern academic pairing".to_string(),
        });
        
        // 优雅配对
        pairings.push(FontPairing {
            title_font: "Georgia".to_string(),
            body_font: "Helvetica".to_string(),
            mono_font: "Courier New".to_string(),
            description: "Elegant and readable".to_string(),
        });
        
        // 专业配对
        pairings.push(FontPairing {
            title_font: "Times New Roman".to_string(),
            body_font: "Arial".to_string(),
            mono_font: "Consolas".to_string(),
            description: "Professional business".to_string(),
        });
        
        Self { pairings }
    }

    pub fn get_pairing(&self, index: usize) -> Option<&FontPairing> {
        self.pairings.get(index)
    }

    pub fn get_all_pairings(&self) -> &[FontPairing] {
        &self.pairings
    }

    pub fn add_pairing(&mut self, pairing: FontPairing) {
        self.pairings.push(pairing);
    }
}

impl Default for FontPairingSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// 高级字体排版配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyConfig {
    pub kerning_table: KerningTable,
    pub opentype_features: OpenTypeFeatures,
    pub font_pairing: FontPairingSystem,
    pub optical_size: Option<f64>, // 光学尺寸
    pub letter_spacing: f64,
    pub word_spacing: f64,
}

impl TypographyConfig {
    pub fn new() -> Self {
        Self {
            kerning_table: KerningTable::new(),
            opentype_features: OpenTypeFeatures::new(),
            font_pairing: FontPairingSystem::new(),
            optical_size: None,
            letter_spacing: 0.0,
            word_spacing: 0.0,
        }
    }

    pub fn with_kerning(mut self, kerning_table: KerningTable) -> Self {
        self.kerning_table = kerning_table;
        self
    }

    pub fn with_opentype_features(mut self, features: OpenTypeFeatures) -> Self {
        self.opentype_features = features;
        self
    }

    pub fn with_optical_size(mut self, size: f64) -> Self {
        self.optical_size = Some(size);
        self
    }

    pub fn with_letter_spacing(mut self, spacing: f64) -> Self {
        self.letter_spacing = spacing;
        self
    }

    pub fn with_word_spacing(mut self, spacing: f64) -> Self {
        self.word_spacing = spacing;
        self
    }

    pub fn to_typst(&self) -> String {
        let mut typst = String::new();
        
        typst.push_str("#set text(");
        
        // 字距调整
        if self.letter_spacing != 0.0 {
            typst.push_str(&format!("tracking: {}em, ", self.letter_spacing));
        }
        
        // 词间距
        if self.word_spacing != 0.0 {
            typst.push_str(&format!("spacing: {}em, ", self.word_spacing));
        }
        
        // OpenType 特性
        let features = self.opentype_features.to_typst();
        if !features.is_empty() {
            typst.push_str(&format!("{}, ", features));
        }
        
        // 光学尺寸
        if let Some(size) = self.optical_size {
            typst.push_str(&format!("optical-size: {}, ", size));
        }
        
        // 移除最后的逗号和空格
        if typst.ends_with(", ") {
            typst.pop();
            typst.pop();
        }
        
        typst.push_str(")\n");
        
        typst
    }
}

impl Default for TypographyConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// 字体微调器
pub struct TypographyEngine {
    config: TypographyConfig,
}

impl TypographyEngine {
    pub fn new() -> Self {
        Self {
            config: TypographyConfig::new(),
        }
    }

    pub fn with_config(config: TypographyConfig) -> Self {
        Self { config }
    }

    /// 应用字距调整
    pub fn apply_kerning(&self, text: &str) -> String {
        let chars: Vec<char> = text.chars().collect();
        let mut result = String::new();
        
        for (i, &ch) in chars.iter().enumerate() {
            result.push(ch);
            
            if i < chars.len() - 1 {
                let next_char = chars[i + 1];
                let adjustment = self.config.kerning_table.get_adjustment(ch, next_char);
                
                if adjustment != 0.0 {
                    // 在 Typst 中使用 #h() 调整间距
                    result.push_str(&format!("#h({}em)", adjustment));
                }
            }
        }
        
        result
    }

    /// 获取字体配对建议
    pub fn get_font_pairing_suggestion(&self, index: usize) -> Option<&FontPairing> {
        self.config.font_pairing.get_pairing(index)
    }

    /// 应用 OpenType 特性
    pub fn apply_opentype_features(&self, text: &str) -> String {
        let features = self.config.opentype_features.to_typst();
        
        if features.is_empty() {
            text.to_string()
        } else {
            format!("#set text({})\n{}", features, text)
        }
    }

    /// 生成完整的排版配置
    pub fn generate_typography_config(&self) -> String {
        self.config.to_typst()
    }
}

impl Default for TypographyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kerning_table_creation() {
        let table = KerningTable::new();
        assert!(table.pairs.is_empty());
    }

    #[test]
    fn test_kerning_table_add_pair() {
        let mut table = KerningTable::new();
        table.add_pair(KerningPair {
            left_char: 'A',
            right_char: 'V',
            adjustment: -0.1,
        });
        assert_eq!(table.pairs.len(), 1);
    }

    #[test]
    fn test_kerning_table_get_adjustment() {
        let mut table = KerningTable::new();
        table.add_pair(KerningPair {
            left_char: 'A',
            right_char: 'V',
            adjustment: -0.1,
        });
        
        let adj = table.get_adjustment('A', 'V');
        assert_eq!(adj, -0.1);
        
        let adj_none = table.get_adjustment('A', 'B');
        assert_eq!(adj_none, 0.0);
    }

    #[test]
    fn test_opentype_feature_to_string() {
        assert_eq!(OpenTypeFeature::Liga.to_string(), "liga");
        assert_eq!(OpenTypeFeature::Smcp.to_string(), "smcp");
        assert_eq!(OpenTypeFeature::Ss01.to_string(), "ss01");
    }

    #[test]
    fn test_opentype_features_creation() {
        let features = OpenTypeFeatures::new();
        assert!(features.is_enabled(&OpenTypeFeature::Liga));
    }

    #[test]
    fn test_opentype_features_enable_disable() {
        let mut features = OpenTypeFeatures::new();
        features.disable_feature(OpenTypeFeature::Liga);
        assert!(!features.is_enabled(&OpenTypeFeature::Liga));
        
        features.enable_feature(OpenTypeFeature::Liga);
        assert!(features.is_enabled(&OpenTypeFeature::Liga));
    }

    #[test]
    fn test_opentype_features_to_typst() {
        let mut features = OpenTypeFeatures::new();
        features.disable_feature(OpenTypeFeature::Liga);
        let typst = features.to_typst();
        assert!(!typst.contains("liga"));
    }

    #[test]
    fn test_font_pairing_system_creation() {
        let system = FontPairingSystem::new();
        assert!(!system.pairings.is_empty());
    }

    #[test]
    fn test_font_pairing_system_get_pairing() {
        let system = FontPairingSystem::new();
        let pairing = system.get_pairing(0);
        assert!(pairing.is_some());
    }

    #[test]
    fn test_typography_config_creation() {
        let config = TypographyConfig::new();
        assert_eq!(config.letter_spacing, 0.0);
    }

    #[test]
    fn test_typography_config_with_optical_size() {
        let config = TypographyConfig::new().with_optical_size(12.0);
        assert_eq!(config.optical_size, Some(12.0));
    }

    #[test]
    fn test_typography_config_to_typst() {
        let config = TypographyConfig::new().with_letter_spacing(0.1);
        let typst = config.to_typst();
        assert!(typst.contains("tracking: 0.1em"));
    }

    #[test]
    fn test_typography_engine_creation() {
        let engine = TypographyEngine::new();
        assert_eq!(engine.config.letter_spacing, 0.0);
    }

    #[test]
    fn test_typography_engine_apply_kerning() {
        let mut table = KerningTable::new();
        table.add_pair(KerningPair {
            left_char: 'A',
            right_char: 'V',
            adjustment: -0.1,
        });
        
        let config = TypographyConfig::new().with_kerning(table);
        let engine = TypographyEngine::with_config(config);
        
        let result = engine.apply_kerning("AV");
        assert!(result.contains("#h(-0.1em)"));
    }

    #[test]
    fn test_typography_engine_get_font_pairing() {
        let engine = TypographyEngine::new();
        let pairing = engine.get_font_pairing_suggestion(0);
        assert!(pairing.is_some());
    }

    #[test]
    fn test_typography_engine_apply_opentype_features() {
        let mut features = OpenTypeFeatures::new();
        features.disable_feature(OpenTypeFeature::Liga);
        
        let config = TypographyConfig::new().with_opentype_features(features);
        let engine = TypographyEngine::with_config(config);
        
        let result = engine.apply_opentype_features("Hello");
        assert!(result.contains("#set text("));
    }
}
