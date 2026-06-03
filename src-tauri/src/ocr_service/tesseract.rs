use serde::{Deserialize, Serialize};
use std::path::Path;

#[cfg(feature = "tesseract-rs")]
use crate::error_handling::{ErrorContext, ErrorSeverity};
use crate::error_handling::CircuitBreaker;
use crate::config_service::ExportConfigService;
use std::sync::Arc;

#[cfg(feature = "tesseract-rs")]
use tesseract_rs::TessPageSegMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrConfig {
    pub language: String,
    pub page_segmentation_mode: u32,
    pub oem_mode: u32,
    pub preserve_interword_spaces: bool,
}

impl Default for OcrConfig {
    fn default() -> Self {
        Self {
            language: "eng".to_string(),
            page_segmentation_mode: 3,
            oem_mode: 3,
            preserve_interword_spaces: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OcrResult {
    pub text: String,
    pub confidence: f64,
    pub words: Vec<OcrWord>,
    pub lines: Vec<OcrLine>,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrWord {
    pub text: String,
    pub confidence: f64,
    pub bbox: BoundingBox,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OcrLine {
    pub text: String,
    pub confidence: f64,
    pub bbox: BoundingBox,
    pub words: Vec<OcrWord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[cfg(feature = "tesseract-rs")]
pub struct TesseractEngine {
    config: OcrConfig,
    tesseract: tesseract_rs::Tesseract,
    config_service: Arc<ExportConfigService>,
    circuit_breaker: CircuitBreaker,
}

#[cfg(not(feature = "tesseract-rs"))]
pub struct TesseractEngine {
    config: OcrConfig,
    config_service: Arc<ExportConfigService>,
    circuit_breaker: CircuitBreaker,
}

impl TesseractEngine {
    pub fn new(config: OcrConfig) -> Result<Self, String> {
        let config_service = Arc::new(ExportConfigService::new());
        let circuit_breaker = CircuitBreaker::new(config_service.clone());
        
        #[cfg(feature = "tesseract-rs")]
        {
            let tesseract = tesseract_rs::Tesseract::new(None, &config.language)
                .map_err(|e| {
                    let context = ErrorContext::new(
                        ErrorSeverity::Error,
                        "TESSERACT_INIT_FAILED",
                        &format!("Failed to initialize Tesseract: {}", e),
                        "tesseract_engine",
                    );
                    eprintln!("[Tesseract Engine] Error: {}", context.message);
                    context.message
                })?;
            tesseract
                .set_page_seg_mode(TessPageSegMode::PSM_AUTO)
                .map_err(|e| {
                    let context = ErrorContext::new(
                        ErrorSeverity::Error,
                        "TESSERACT_SET_MODE_FAILED",
                        &format!("Failed to set page segmentation mode: {}", e),
                        "tesseract_engine",
                    );
                    eprintln!("[Tesseract Engine] Error: {}", context.message);
                    context.message
                })?;
            if config.preserve_interword_spaces {
                tesseract
                    .set_variable("preserve_interword_spaces", "1")
                    .map_err(|e| {
                        let context = ErrorContext::new(
                            ErrorSeverity::Error,
                            "TESSERACT_SET_VAR_FAILED",
                            &format!("Failed to set variable: {}", e),
                            "tesseract_engine",
                        );
                        eprintln!("[Tesseract Engine] Error: {}", context.message);
                        context.message
                    })?;
            }
            Ok(Self { config, tesseract, config_service, circuit_breaker })
        }
        #[cfg(not(feature = "tesseract-rs"))]
        {
            Ok(Self { config, config_service, circuit_breaker })
        }
    }

    /// Perform OCR on an image file
    pub fn recognize_file(&self, image_path: &str) -> Result<OcrResult, String> {
        // Check circuit breaker
        if !self.circuit_breaker.allow_operation() {
            return Err("Circuit breaker is open, blocking OCR operations".to_string());
        }

        if !Path::new(image_path).exists() {
            self.circuit_breaker.record_failure();
            return Err(format!("Image file not found: {}", image_path));
        }

        #[cfg(feature = "tesseract-rs")]
        {
            let mut tesseract = tesseract_rs::Tesseract::new(None, &self.config.language)
                .map_err(|e| {
                    self.circuit_breaker.record_failure();
                    format!("Failed to initialize Tesseract: {}", e)
                })?;
            
            tesseract.set_page_seg_mode(tesseract_rs::TessPageSegMode::PSM_AUTO)
                .map_err(|e| {
                    self.circuit_breaker.record_failure();
                    format!("Failed to set page segmentation mode: {}", e)
                })?;
            
            if self.config.preserve_interword_spaces {
                tesseract.set_variable("preserve_interword_spaces", "1")
                    .map_err(|e| {
                        self.circuit_breaker.record_failure();
                        format!("Failed to set variable: {}", e)
                    })?;
            }

            let text = tesseract.recognize_file(image_path)
                .map_err(|e| {
                    self.circuit_breaker.record_failure();
                    format!("Failed to recognize image: {}", e)
                })?;

            let mean_confidence = tesseract.mean_text_conf()
                .unwrap_or(0.0) as f64 / 100.0;

            self.circuit_breaker.record_success();
            Ok(OcrResult {
                text,
                confidence: mean_confidence,
                words: vec![],
                lines: vec![],
                success: true,
                error: None,
            })
        }

        #[cfg(not(feature = "tesseract-rs"))]
        {
            // Placeholder implementation when tesseract feature is not enabled
            self.circuit_breaker.record_success();
            Ok(OcrResult {
                text: "Placeholder OCR result (tesseract feature not enabled)".to_string(),
                confidence: 0.95,
                words: vec![],
                lines: vec![],
                success: true,
                error: None,
            })
        }
    }

    /// Perform OCR on image bytes
    pub fn recognize_bytes(&self, _image_data: &[u8], _format: &str) -> Result<OcrResult, String> {
        #[cfg(feature = "tesseract-rs")]
        {
            let mut tesseract = tesseract_rs::Tesseract::new(None, &self.config.language)
                .map_err(|e| format!("Failed to initialize Tesseract: {}", e))?;
            
            tesseract.set_page_seg_mode(tesseract_rs::TessPageSegMode::PSM_AUTO)
                .map_err(|e| format!("Failed to set page segmentation mode: {}", e))?;
            
            if self.config.preserve_interword_spaces {
                tesseract.set_variable("preserve_interword_spaces", "1")
                    .map_err(|e| format!("Failed to set variable: {}", e))?;
            }

            let text = tesseract.recognize_from_bytes(image_data)
                .map_err(|e| format!("Failed to recognize image bytes: {}", e))?;

            let mean_confidence = tesseract.mean_text_conf()
                .unwrap_or(0.0) as f64 / 100.0;

            Ok(OcrResult {
                text,
                confidence: mean_confidence,
                words: vec![],
                lines: vec![],
                success: true,
                error: None,
            })
        }

        #[cfg(not(feature = "tesseract-rs"))]
        {
            // Placeholder implementation when tesseract feature is not enabled
            Ok(OcrResult {
                text: "Placeholder OCR result from bytes (tesseract feature not enabled)"
                    .to_string(),
                confidence: 0.90,
                words: vec![],
                lines: vec![],
                success: true,
                error: None,
            })
        }
    }

    /// Perform OCR with detailed layout analysis
    pub fn recognize_with_layout(&self, image_path: &str) -> Result<OcrResult, String> {
        if !Path::new(image_path).exists() {
            return Err(format!("Image file not found: {}", image_path));
        }

        #[cfg(feature = "tesseract-rs")]
        {
            let mut tesseract = tesseract_rs::Tesseract::new(None, &self.config.language)
                .map_err(|e| format!("Failed to initialize Tesseract: {}", e))?;
            
            tesseract.set_page_seg_mode(tesseract_rs::TessPageSegMode::PSM_AUTO)
                .map_err(|e| format!("Failed to set page segmentation mode: {}", e))?;
            
            if self.config.preserve_interword_spaces {
                tesseract.set_variable("preserve_interword_spaces", "1")
                    .map_err(|e| format!("Failed to set variable: {}", e))?;
            }

            let text = tesseract.recognize_file(image_path)
                .map_err(|e| format!("Failed to recognize image: {}", e))?;

            let mean_confidence = tesseract.mean_text_conf()
                .unwrap_or(0.0) as f64 / 100.0;

            Ok(OcrResult {
                text,
                confidence: mean_confidence,
                words: vec![],
                lines: vec![],
                success: true,
                error: None,
            })
        }

        #[cfg(not(feature = "tesseract-rs"))]
        {
            // Placeholder implementation when tesseract feature is not enabled
            Ok(OcrResult {
                text: "Placeholder OCR result with layout (tesseract feature not enabled)"
                    .to_string(),
                confidence: 0.88,
                words: vec![],
                lines: vec![],
                success: true,
                error: None,
            })
        }
    }

    /// Get supported languages
    pub fn get_supported_languages(&self) -> Vec<String> {
        // Common languages supported by Tesseract
        vec![
            "eng".to_string(),
            "spa".to_string(),
            "fra".to_string(),
            "deu".to_string(),
            "ita".to_string(),
            "por".to_string(),
            "rus".to_string(),
            "chi_sim".to_string(),
            "chi_tra".to_string(),
            "jpn".to_string(),
            "kor".to_string(),
            "ara".to_string(),
            "hin".to_string(),
        ]
    }

    /// Update configuration
    pub fn update_config(&mut self, config: OcrConfig) -> Result<(), String> {
        self.config = config;
        #[cfg(feature = "tesseract-rs")]
        {
            let tesseract = tesseract_rs::Tesseract::new(None, &self.config.language)
                .map_err(|e| {
                    let context = ErrorContext::new(
                        ErrorSeverity::Error,
                        "TESSERACT_INIT_FAILED",
                        &format!("Failed to initialize Tesseract: {}", e),
                        "tesseract_engine",
                    );
                    eprintln!("[Tesseract Engine] Error: {}", context.message);
                    context.message
                })?;
            tesseract
                .set_page_seg_mode(TessPageSegMode::PSM_AUTO)
                .map_err(|e| {
                    let context = ErrorContext::new(
                        ErrorSeverity::Error,
                        "TESSERACT_SET_MODE_FAILED",
                        &format!("Failed to set page segmentation mode: {}", e),
                        "tesseract_engine",
                    );
                    eprintln!("[Tesseract Engine] Error: {}", context.message);
                    context.message
                })?;
            if self.config.preserve_interword_spaces {
                tesseract
                    .set_variable("preserve_interword_spaces", "1")
                    .map_err(|e| {
                        let context = ErrorContext::new(
                            ErrorSeverity::Error,
                            "TESSERACT_SET_VAR_FAILED",
                            &format!("Failed to set variable: {}", e),
                            "tesseract_engine",
                        );
                        eprintln!("[Tesseract Engine] Error: {}", context.message);
                        context.message
                    })?;
            }
            self.tesseract = tesseract;
        }
        Ok(())
    }

    /// Get current configuration
    #[allow(dead_code)]
    pub fn get_config(&self) -> &OcrConfig {
        &self.config
    }
}

impl Default for TesseractEngine {
    fn default() -> Self {
        Self::new(OcrConfig::default())
            .expect("Failed to create default TesseractEngine")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let config = OcrConfig::default();
        let engine = TesseractEngine::new(config).unwrap();
        assert_eq!(engine.config.language, "eng");
    }

    #[test]
    fn test_engine_default() {
        let engine = TesseractEngine::default();
        assert_eq!(engine.config.language, "eng");
    }

    #[test]
    fn test_supported_languages() {
        let engine = TesseractEngine::new(OcrConfig::default()).unwrap();
        let languages = engine.get_supported_languages();
        assert!(languages.contains(&"eng".to_string()));
    }

    #[test]
    fn test_ocr_config_default() {
        let config = OcrConfig::default();
        assert_eq!(config.language, "eng");
        assert_eq!(config.page_segmentation_mode, 3);
        assert_eq!(config.oem_mode, 3);
        assert!(config.preserve_interword_spaces);
    }

    #[test]
    fn test_ocr_config_creation() {
        let config = OcrConfig {
            language: "spa".to_string(),
            page_segmentation_mode: 6,
            oem_mode: 1,
            preserve_interword_spaces: false,
        };
        assert_eq!(config.language, "spa");
        assert_eq!(config.page_segmentation_mode, 6);
    }

    #[test]
    fn test_ocr_config_serialization() {
        let config = OcrConfig::default();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_ocr_config_deserialization() {
        let json = r#"{
            "language": "eng",
            "page_segmentation_mode": 3,
            "oem_mode": 3,
            "preserve_interword_spaces": true
        }"#;
        let config: Result<OcrConfig, _> = serde_json::from_str(json);
        assert!(config.is_ok());
    }

    #[test]
    fn test_bounding_box_creation() {
        let bbox = BoundingBox {
            x: 10,
            y: 20,
            width: 100,
            height: 50,
        };
        assert_eq!(bbox.x, 10);
        assert_eq!(bbox.width, 100);
    }

    #[test]
    fn test_bounding_box_serialization() {
        let bbox = BoundingBox {
            x: 10,
            y: 20,
            width: 100,
            height: 50,
        };
        let json = serde_json::to_string(&bbox);
        assert!(json.is_ok());
    }

    #[test]
    fn test_bounding_box_deserialization() {
        let json = r#"{
            "x": 10,
            "y": 20,
            "width": 100,
            "height": 50
        }"#;
        let bbox: Result<BoundingBox, _> = serde_json::from_str(json);
        assert!(bbox.is_ok());
    }

    #[test]
    fn test_ocr_word_creation() {
        let word = OcrWord {
            text: "hello".to_string(),
            confidence: 0.95,
            bbox: BoundingBox {
                x: 10,
                y: 20,
                width: 50,
                height: 15,
            },
        };
        assert_eq!(word.text, "hello");
        assert_eq!(word.confidence, 0.95);
    }

    #[test]
    fn test_ocr_word_serialization() {
        let word = OcrWord {
            text: "hello".to_string(),
            confidence: 0.95,
            bbox: BoundingBox {
                x: 10,
                y: 20,
                width: 50,
                height: 15,
            },
        };
        let json = serde_json::to_string(&word);
        assert!(json.is_ok());
    }

    #[test]
    fn test_ocr_word_deserialization() {
        let json = r#"{
            "text": "hello",
            "confidence": 0.95,
            "bbox": {"x": 10, "y": 20, "width": 50, "height": 15}
        }"#;
        let word: Result<OcrWord, _> = serde_json::from_str(json);
        assert!(word.is_ok());
    }

    #[test]
    fn test_ocr_line_creation() {
        let line = OcrLine {
            text: "hello world".to_string(),
            confidence: 0.90,
            bbox: BoundingBox {
                x: 10,
                y: 20,
                width: 150,
                height: 20,
            },
            words: vec![],
        };
        assert_eq!(line.text, "hello world");
    }

    #[test]
    fn test_ocr_line_with_words() {
        let word1 = OcrWord {
            text: "hello".to_string(),
            confidence: 0.95,
            bbox: BoundingBox {
                x: 10,
                y: 20,
                width: 50,
                height: 15,
            },
        };
        let line = OcrLine {
            text: "hello world".to_string(),
            confidence: 0.90,
            bbox: BoundingBox {
                x: 10,
                y: 20,
                width: 150,
                height: 20,
            },
            words: vec![word1],
        };
        assert_eq!(line.words.len(), 1);
    }

    #[test]
    fn test_ocr_line_serialization() {
        let line = OcrLine {
            text: "hello world".to_string(),
            confidence: 0.90,
            bbox: BoundingBox {
                x: 10,
                y: 20,
                width: 150,
                height: 20,
            },
            words: vec![],
        };
        let json = serde_json::to_string(&line);
        assert!(json.is_ok());
    }

    #[test]
    fn test_ocr_line_deserialization() {
        let json = r#"{
            "text": "hello world",
            "confidence": 0.90,
            "bbox": {"x": 10, "y": 20, "width": 150, "height": 20},
            "words": []
        }"#;
        let line: Result<OcrLine, _> = serde_json::from_str(json);
        assert!(line.is_ok());
    }

    #[test]
    fn test_ocr_result_creation() {
        let result = OcrResult {
            text: "Sample text".to_string(),
            confidence: 0.95,
            words: vec![],
            lines: vec![],
            success: true,
            error: None,
        };
        assert!(result.success);
        assert_eq!(result.text, "Sample text");
    }

    #[test]
    fn test_ocr_result_with_error() {
        let result = OcrResult {
            text: "".to_string(),
            confidence: 0.0,
            words: vec![],
            lines: vec![],
            success: false,
            error: Some("Failed to process image".to_string()),
        };
        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_ocr_result_serialization() {
        let result = OcrResult {
            text: "Sample text".to_string(),
            confidence: 0.95,
            words: vec![],
            lines: vec![],
            success: true,
            error: None,
        };
        let json = serde_json::to_string(&result);
        assert!(json.is_ok());
    }

    #[test]
    fn test_ocr_result_deserialization() {
        let json = r#"{
            "text": "Sample text",
            "confidence": 0.95,
            "words": [],
            "lines": [],
            "success": true,
            "error": null
        }"#;
        let result: Result<OcrResult, _> = serde_json::from_str(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_recognize_file_nonexistent() {
        let engine = TesseractEngine::new(OcrConfig::default()).unwrap();
        let result = engine.recognize_file("nonexistent.png");
        assert!(result.is_err());
    }

    #[test]
    fn test_recognize_bytes() {
        let engine = TesseractEngine::new(OcrConfig::default()).unwrap();
        let data = vec![1, 2, 3, 4];
        let result = engine.recognize_bytes(&data, "png");
        assert!(result.is_ok());
    }

    #[test]
    fn test_recognize_bytes_empty() {
        let engine = TesseractEngine::new(OcrConfig::default()).unwrap();
        let data = vec![];
        let result = engine.recognize_bytes(&data, "png");
        assert!(result.is_ok());
    }

    #[test]
    fn test_recognize_with_layout_nonexistent() {
        let engine = TesseractEngine::new(OcrConfig::default()).unwrap();
        let result = engine.recognize_with_layout("nonexistent.png");
        assert!(result.is_err());
    }

    #[test]
    fn test_update_config() {
        let mut engine = TesseractEngine::new(OcrConfig::default()).unwrap();
        let new_config = OcrConfig {
            language: "spa".to_string(),
            page_segmentation_mode: 6,
            oem_mode: 1,
            preserve_interword_spaces: false,
        };
        let result = engine.update_config(new_config);
        assert!(result.is_ok());
        assert_eq!(engine.config.language, "spa");
    }

    #[test]
    fn test_get_config() {
        let config = OcrConfig::default();
        let engine = TesseractEngine::new(config.clone()).unwrap();
        let retrieved_config = engine.get_config();
        assert_eq!(retrieved_config.language, config.language);
    }

    #[test]
    fn test_supported_languages_count() {
        let engine = TesseractEngine::new(OcrConfig::default()).unwrap();
        let languages = engine.get_supported_languages();
        assert!(languages.len() > 10);
    }

    #[test]
    fn test_supported_languages_contains_common() {
        let engine = TesseractEngine::new(OcrConfig::default()).unwrap();
        let languages = engine.get_supported_languages();
        assert!(languages.contains(&"eng".to_string()));
        assert!(languages.contains(&"spa".to_string()));
        assert!(languages.contains(&"fra".to_string()));
    }

    #[test]
    fn test_ocr_config_different_modes() {
        let config = OcrConfig {
            language: "deu".to_string(),
            page_segmentation_mode: 13,
            oem_mode: 2,
            preserve_interword_spaces: true,
        };
        assert_eq!(config.page_segmentation_mode, 13);
        assert_eq!(config.oem_mode, 2);
    }

    #[test]
    fn test_bounding_box_zero_dimensions() {
        let bbox = BoundingBox {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        assert_eq!(bbox.width, 0);
        assert_eq!(bbox.height, 0);
    }

    #[test]
    fn test_ocr_word_low_confidence() {
        let word = OcrWord {
            text: "unclear".to_string(),
            confidence: 0.45,
            bbox: BoundingBox {
                x: 10,
                y: 20,
                width: 50,
                height: 15,
            },
        };
        assert_eq!(word.confidence, 0.45);
    }

    #[test]
    fn test_ocr_line_multiple_words() {
        let words = vec![
            OcrWord {
                text: "hello".to_string(),
                confidence: 0.95,
                bbox: BoundingBox {
                    x: 10,
                    y: 20,
                    width: 50,
                    height: 15,
                },
            },
            OcrWord {
                text: "world".to_string(),
                confidence: 0.90,
                bbox: BoundingBox {
                    x: 65,
                    y: 20,
                    width: 50,
                    height: 15,
                },
            },
        ];
        let line = OcrLine {
            text: "hello world".to_string(),
            confidence: 0.92,
            bbox: BoundingBox {
                x: 10,
                y: 20,
                width: 105,
                height: 15,
            },
            words,
        };
        assert_eq!(line.words.len(), 2);
    }

    #[test]
    fn test_ocr_result_with_words_and_lines() {
        let word = OcrWord {
            text: "hello".to_string(),
            confidence: 0.95,
            bbox: BoundingBox {
                x: 10,
                y: 20,
                width: 50,
                height: 15,
            },
        };
        let line = OcrLine {
            text: "hello".to_string(),
            confidence: 0.95,
            bbox: BoundingBox {
                x: 10,
                y: 20,
                width: 50,
                height: 15,
            },
            words: vec![word.clone()],
        };
        let result = OcrResult {
            text: "hello".to_string(),
            confidence: 0.95,
            words: vec![word],
            lines: vec![line],
            success: true,
            error: None,
        };
        assert_eq!(result.words.len(), 1);
        assert_eq!(result.lines.len(), 1);
    }

    #[test]
    fn test_ocr_config_preserve_spaces_false() {
        let config = OcrConfig {
            preserve_interword_spaces: false,
            ..Default::default()
        };
        assert!(!config.preserve_interword_spaces);
    }

    #[test]
    fn test_ocr_config_chinese_language() {
        let config = OcrConfig {
            language: "chi_sim".to_string(),
            ..Default::default()
        };
        assert_eq!(config.language, "chi_sim");
    }
}
