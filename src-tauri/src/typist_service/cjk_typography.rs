/*!
 * 航空航天级CJK排版增强系统
 * 实现中文排版规则、标点压缩、避头尾、行首行尾禁则
 */

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// CJK 语言类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CJKLanguage {
    /// 简体中文
    SimplifiedChinese,
    /// 繁体中文
    TraditionalChinese,
    /// 日语
    Japanese,
    /// 韩语
    Korean,
}

/// 标点压缩规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PunctuationCompressionRule {
    pub punctuation: char,
    pub compression_ratio: f64, // 0.0-1.0, 1.0 = no compression
    pub context: CompressionContext,
}

/// 压缩上下文
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompressionContext {
    /// 行首
    LineStart,
    /// 行尾
    LineEnd,
    /// 行中
    LineMiddle,
    /// 任意位置
    Anywhere,
}

/// 避头尾规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineBreakRule {
    pub characters: HashSet<char>,
    pub forbidden_position: BreakPosition,
}

/// 禁止断行位置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BreakPosition {
    /// 行首禁止
    LineStart,
    /// 行尾禁止
    LineEnd,
    /// 任意位置禁止
    Anywhere,
}

/// CJK 排版配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CJKTypographyConfig {
    pub language: CJKLanguage,
    pub punctuation_compression: bool,
    pub line_break_rules: bool,
    pub vertical_text: bool,
    pub writing_mode: WritingMode,
    pub punctuation_width: PunctuationWidth,
}

/// 书写模式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WritingMode {
    /// 横排
    Horizontal,
    /// 竖排
    Vertical,
}

/// 标点宽度
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PunctuationWidth {
    /// 全角
    FullWidth,
    /// 半角
    HalfWidth,
    /// 自动
    Auto,
}

impl Default for CJKTypographyConfig {
    fn default() -> Self {
        Self {
            language: CJKLanguage::SimplifiedChinese,
            punctuation_compression: true,
            line_break_rules: true,
            vertical_text: false,
            writing_mode: WritingMode::Horizontal,
            punctuation_width: PunctuationWidth::Auto,
        }
    }
}

/// CJK 排版引擎
pub struct CJKTypographyEngine {
    config: CJKTypographyConfig,
    compression_rules: Vec<PunctuationCompressionRule>,
    line_break_rules: Vec<LineBreakRule>,
}

impl CJKTypographyEngine {
    pub fn new(config: CJKTypographyConfig) -> Self {
        let engine = Self {
            config: config.clone(),
            compression_rules: Self::default_compression_rules(&config.language),
            line_break_rules: Self::default_line_break_rules(&config.language),
        };
        engine
    }

    /// 默认标点压缩规则
    fn default_compression_rules(language: &CJKLanguage) -> Vec<PunctuationCompressionRule> {
        match language {
            CJKLanguage::SimplifiedChinese | CJKLanguage::TraditionalChinese => {
                vec![
                    PunctuationCompressionRule {
                        punctuation: '，',
                        compression_ratio: 0.5,
                        context: CompressionContext::LineEnd,
                    },
                    PunctuationCompressionRule {
                        punctuation: '。',
                        compression_ratio: 0.5,
                        context: CompressionContext::LineEnd,
                    },
                    PunctuationCompressionRule {
                        punctuation: '、',
                        compression_ratio: 0.5,
                        context: CompressionContext::LineEnd,
                    },
                    PunctuationCompressionRule {
                        punctuation: '：',
                        compression_ratio: 0.5,
                        context: CompressionContext::LineEnd,
                    },
                    PunctuationCompressionRule {
                        punctuation: '；',
                        compression_ratio: 0.5,
                        context: CompressionContext::LineEnd,
                    },
                ]
            }
            CJKLanguage::Japanese => {
                vec![
                    PunctuationCompressionRule {
                        punctuation: '、',
                        compression_ratio: 0.5,
                        context: CompressionContext::LineEnd,
                    },
                    PunctuationCompressionRule {
                        punctuation: '。',
                        compression_ratio: 0.5,
                        context: CompressionContext::LineEnd,
                    },
                ]
            }
            CJKLanguage::Korean => {
                vec![]
            }
        }
    }

    /// 默认避头尾规则
    fn default_line_break_rules(language: &CJKLanguage) -> Vec<LineBreakRule> {
        match language {
            CJKLanguage::SimplifiedChinese | CJKLanguage::TraditionalChinese => {
                let mut start_chars = HashSet::new();
                // 行首禁止字符
                let forbidden_start = ['，', '。', '！', '？', '；', '：', '"', '"', '\'', '\'', '（', '）', '【', '】', '《', '》'];
                for c in forbidden_start {
                    start_chars.insert(c);
                }
                
                let mut end_chars = HashSet::new();
                // 行尾禁止字符
                let forbidden_end = ['（', '《', '「'];
                for c in forbidden_end {
                    end_chars.insert(c);
                }
                
                vec![
                    LineBreakRule {
                        characters: start_chars,
                        forbidden_position: BreakPosition::LineStart,
                    },
                    LineBreakRule {
                        characters: end_chars,
                        forbidden_position: BreakPosition::LineEnd,
                    },
                ]
            }
            CJKLanguage::Japanese => {
                let mut start_chars = HashSet::new();
                let forbidden_start = ['、', '。', '！', '？', '；', '：', '"', '"', '\'', '\'', '（', '）', '【', '】', '《', '》'];
                for c in forbidden_start {
                    start_chars.insert(c);
                }
                
                let mut end_chars = HashSet::new();
                let forbidden_end = ['（', '《', '「'];
                for c in forbidden_end {
                    end_chars.insert(c);
                }
                
                vec![
                    LineBreakRule {
                        characters: start_chars,
                        forbidden_position: BreakPosition::LineStart,
                    },
                    LineBreakRule {
                        characters: end_chars,
                        forbidden_position: BreakPosition::LineEnd,
                    },
                ]
            }
            CJKLanguage::Korean => {
                vec![]
            }
        }
    }

    /// 应用标点压缩
    pub fn apply_punctuation_compression(&self, text: &str) -> String {
        if !self.config.punctuation_compression {
            return text.to_string();
        }

        let chars: Vec<char> = text.chars().collect();
        let mut result = String::new();
        
        for (i, &ch) in chars.iter().enumerate() {
            let context = if i == 0 {
                CompressionContext::LineStart
            } else if i == chars.len() - 1 {
                CompressionContext::LineEnd
            } else {
                CompressionContext::LineMiddle
            };
            
            let compressed = self.compression_rules
                .iter()
                .find(|rule| rule.punctuation == ch && (rule.context == context || rule.context == CompressionContext::Anywhere));
            
            if let Some(rule) = compressed {
                // 在 Typst 中使用 #h() 调整间距
                let spacing = (1.0 - rule.compression_ratio) * 0.5; // 假设全角标点为 0.5em
                result.push(ch);
                if spacing > 0.0 {
                    result.push_str(&format!("#h(-{}em)", spacing));
                }
            } else {
                result.push(ch);
            }
        }
        
        result
    }

    /// 检查字符是否可以在指定位置断行
    pub fn can_break_at(&self, char: char, position: BreakPosition) -> bool {
        if !self.config.line_break_rules {
            return true;
        }
        
        for rule in &self.line_break_rules {
            if rule.characters.contains(&char) && rule.forbidden_position == position {
                return false;
            }
        }
        
        true
    }

    /// 检查字符是否可以在行首
    pub fn can_start_line(&self, char: char) -> bool {
        self.can_break_at(char, BreakPosition::LineStart)
    }

    /// 检查字符是否可以在行尾
    pub fn can_end_line(&self, char: char) -> bool {
        self.can_break_at(char, BreakPosition::LineEnd)
    }

    /// 智能断行
    pub fn smart_line_break(&self, text: &str, max_width: usize) -> Vec<String> {
        let chars: Vec<char> = text.chars().collect();
        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut current_width = 0;
        
        for (i, &ch) in chars.iter().enumerate() {
            let char_width = if self.is_cjk_char(ch) { 2 } else { 1 };
            
            if current_width + char_width > max_width {
                // 尝试断行
                if self.can_end_line(ch) && i > 0 {
                    let next_char = chars.get(i + 1);
                    if let Some(&next) = next_char {
                        if self.can_start_line(next) {
                            lines.push(current_line.clone());
                            current_line = String::new();
                            current_width = 0;
                        }
                    }
                }
            }
            
            current_line.push(ch);
            current_width += char_width;
        }
        
        if !current_line.is_empty() {
            lines.push(current_line);
        }
        
        lines
    }

    /// 判断是否为 CJK 字符
    fn is_cjk_char(&self, ch: char) -> bool {
        match self.config.language {
            CJKLanguage::SimplifiedChinese | CJKLanguage::TraditionalChinese => {
                // CJK 统一表意文字范围
                (ch >= '\u{4E00}' && ch <= '\u{9FFF}') ||
                (ch >= '\u{3400}' && ch <= '\u{4DBF}') ||
                (ch >= '\u{20000}' && ch <= '\u{2A6DF}') ||
                (ch >= '\u{2A700}' && ch <= '\u{2B73F}') ||
                (ch >= '\u{2B740}' && ch <= '\u{2B81F}') ||
                (ch >= '\u{2B820}' && ch <= '\u{2CEAF}') ||
                (ch >= '\u{F900}' && ch <= '\u{FAFF}') ||
                (ch >= '\u{2F800}' && ch <= '\u{2FA1F}')
            }
            CJKLanguage::Japanese => {
                // 日语汉字和假名
                (ch >= '\u{4E00}' && ch <= '\u{9FFF}') ||
                (ch >= '\u{3040}' && ch <= '\u{309F}') ||
                (ch >= '\u{30A0}' && ch <= '\u{30FF}')
            }
            CJKLanguage::Korean => {
                // 韩语文字
                (ch >= '\u{AC00}' && ch <= '\u{D7AF}') ||
                (ch >= '\u{1100}' && ch <= '\u{11FF}') ||
                (ch >= '\u{3130}' && ch <= '\u{318F}')
            }
        }
    }

    /// 生成 CJK 排版配置 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();
        
        // 设置语言
        match self.config.language {
            CJKLanguage::SimplifiedChinese => {
                typst.push_str("#set text(lang: \"zh-CN\")\n");
            }
            CJKLanguage::TraditionalChinese => {
                typst.push_str("#set text(lang: \"zh-TW\")\n");
            }
            CJKLanguage::Japanese => {
                typst.push_str("#set text(lang: \"ja\")\n");
            }
            CJKLanguage::Korean => {
                typst.push_str("#set text(lang: \"ko\")\n");
            }
        }
        
        // 设置书写模式
        match self.config.writing_mode {
            WritingMode::Horizontal => {
                typst.push_str("#set text(dir: ttb)\n");
            }
            WritingMode::Vertical => {
                typst.push_str("#set text(dir: ltr)\n");
            }
        }
        
        // 标点宽度
        match self.config.punctuation_width {
            PunctuationWidth::FullWidth => {
                typst.push_str("#set text(punctuation-spacing: full)\n");
            }
            PunctuationWidth::HalfWidth => {
                typst.push_str("#set text(punctuation-spacing: half)\n");
            }
            PunctuationWidth::Auto => {
                typst.push_str("#set text(punctuation-spacing: auto)\n");
            }
        }
        
        // 标点压缩
        if self.config.punctuation_compression {
            typst.push_str("#set text(punctuation-spacing: 0.5em)\n");
        }
        
        typst
    }
}

impl Default for CJKTypographyEngine {
    fn default() -> Self {
        Self::new(CJKTypographyConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cjk_typography_config_default() {
        let config = CJKTypographyConfig::default();
        assert_eq!(config.language, CJKLanguage::SimplifiedChinese);
        assert!(config.punctuation_compression);
    }

    #[test]
    fn test_cjk_typography_engine_creation() {
        let engine = CJKTypographyEngine::default();
        assert_eq!(engine.config.language, CJKLanguage::SimplifiedChinese);
    }

    #[test]
    fn test_apply_punctuation_compression() {
        let config = CJKTypographyConfig::default();
        let engine = CJKTypographyEngine::new(config);
        
        let result = engine.apply_punctuation_compression("你好，世界。");
        assert!(result.contains("你好"));
        assert!(result.contains("，"));
        assert!(result.contains("。"));
    }

    #[test]
    fn test_can_start_line() {
        let engine = CJKTypographyEngine::default();
        
        assert!(!engine.can_start_line('，'));
        assert!(!engine.can_start_line('。'));
        assert!(engine.can_start_line('你'));
    }

    #[test]
    fn test_can_end_line() {
        let engine = CJKTypographyEngine::default();
        
        assert!(!engine.can_end_line('（'));
        assert!(!engine.can_end_line('《'));
        assert!(engine.can_end_line('你'));
    }

    #[test]
    fn test_is_cjk_char() {
        let engine = CJKTypographyEngine::default();
        
        assert!(engine.is_cjk_char('你'));
        assert!(engine.is_cjk_char('好'));
        assert!(!engine.is_cjk_char('A'));
        assert!(!engine.is_cjk_char('a'));
    }

    #[test]
    fn test_smart_line_break() {
        let engine = CJKTypographyEngine::default();
        
        let text = "你好世界，这是一个测试。";
        let lines = engine.smart_line_break(text, 10);
        
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_to_typst() {
        let engine = CJKTypographyEngine::default();
        let typst = engine.to_typst();
        
        assert!(typst.contains("#set text(lang:"));
        assert!(typst.contains("zh-CN"));
    }

    #[test]
    fn test_japanese_config() {
        let config = CJKTypographyConfig {
            language: CJKLanguage::Japanese,
            ..Default::default()
        };
        let engine = CJKTypographyEngine::new(config);
        
        assert_eq!(engine.config.language, CJKLanguage::Japanese);
    }

    #[test]
    fn test_korean_config() {
        let config = CJKTypographyConfig {
            language: CJKLanguage::Korean,
            ..Default::default()
        };
        let engine = CJKTypographyEngine::new(config);
        
        assert_eq!(engine.config.language, CJKLanguage::Korean);
    }

    #[test]
    fn test_vertical_writing_mode() {
        let config = CJKTypographyConfig {
            writing_mode: WritingMode::Vertical,
            ..Default::default()
        };
        let engine = CJKTypographyEngine::new(config);
        let typst = engine.to_typst();
        
        assert!(typst.contains("dir:"));
    }

    #[test]
    fn test_punctuation_width_full() {
        let config = CJKTypographyConfig {
            punctuation_width: PunctuationWidth::FullWidth,
            ..Default::default()
        };
        let engine = CJKTypographyEngine::new(config);
        let typst = engine.to_typst();
        
        assert!(typst.contains("punctuation-spacing: full"));
    }
}
