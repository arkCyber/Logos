use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognitionConfig {
    pub language: String,
    pub continuous: bool,
    pub interim_results: bool,
    pub max_alternatives: u32,
}

impl Default for RecognitionConfig {
    fn default() -> Self {
        Self {
            language: "en-US".to_string(),
            continuous: false,
            interim_results: false,
            max_alternatives: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognitionResult {
    pub transcript: String,
    pub confidence: f64,
    pub is_final: bool,
    pub alternatives: Vec<String>,
    pub language: String,
}

pub struct SpeechRecognizer {
    config: RecognitionConfig,
    is_listening: bool,
}

impl SpeechRecognizer {
    pub fn new(config: RecognitionConfig) -> Self {
        Self {
            config,
            is_listening: false,
        }
    }

    /// Start speech recognition
    pub fn start(&mut self) -> Result<(), String> {
        if self.is_listening {
            return Err("Already listening".to_string());
        }

        self.is_listening = true;
        // In production, this would initialize the speech recognition engine
        Ok(())
    }

    /// Stop speech recognition
    pub fn stop(&mut self) -> Result<(), String> {
        if !self.is_listening {
            return Err("Not currently listening".to_string());
        }

        self.is_listening = false;
        // In production, this would stop the speech recognition engine
        Ok(())
    }

    /// Process audio data
    pub fn process_audio(&self, _audio_data: &[u8]) -> Result<RecognitionResult, String> {
        if !self.is_listening {
            return Err("Not currently listening".to_string());
        }

        // Placeholder implementation
        // In production, this would use a speech recognition library like vosk or pocketsphinx
        Ok(RecognitionResult {
            transcript: "Placeholder recognized text".to_string(),
            confidence: 0.95,
            is_final: true,
            alternatives: vec![],
            language: self.config.language.clone(),
        })
    }

    /// Check if currently listening
    pub fn is_listening(&self) -> bool {
        self.is_listening
    }

    /// Update configuration
    pub fn update_config(&mut self, config: RecognitionConfig) {
        self.config = config;
    }

    /// Get current configuration
    #[allow(dead_code)]
    pub fn get_config(&self) -> &RecognitionConfig {
        &self.config
    }

    /// Get supported languages
    pub fn get_supported_languages(&self) -> Vec<String> {
        vec![
            "en-US".to_string(),
            "en-GB".to_string(),
            "es-ES".to_string(),
            "fr-FR".to_string(),
            "de-DE".to_string(),
            "it-IT".to_string(),
            "pt-BR".to_string(),
            "zh-CN".to_string(),
            "ja-JP".to_string(),
            "ko-KR".to_string(),
        ]
    }
}

impl Default for SpeechRecognizer {
    fn default() -> Self {
        Self::new(RecognitionConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recognizer_creation() {
        let recognizer = SpeechRecognizer::new(RecognitionConfig::default());
        assert!(!recognizer.is_listening());
    }

    #[test]
    fn test_start_stop() {
        let mut recognizer = SpeechRecognizer::new(RecognitionConfig::default());
        recognizer.start().unwrap();
        assert!(recognizer.is_listening());
        recognizer.stop().unwrap();
        assert!(!recognizer.is_listening());
    }

    #[test]
    fn test_start_when_already_listening() {
        let mut recognizer = SpeechRecognizer::new(RecognitionConfig::default());
        recognizer.start().unwrap();
        let result = recognizer.start();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Already listening"));
    }

    #[test]
    fn test_stop_when_not_listening() {
        let mut recognizer = SpeechRecognizer::new(RecognitionConfig::default());
        let result = recognizer.stop();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Not currently listening"));
    }

    #[test]
    fn test_process_audio_when_listening() {
        let mut recognizer = SpeechRecognizer::new(RecognitionConfig::default());
        recognizer.start().unwrap();
        let audio_data = vec![0u8; 100];
        let result = recognizer.process_audio(&audio_data);
        assert!(result.is_ok());
        let recognition_result = result.unwrap();
        assert_eq!(recognition_result.transcript, "Placeholder recognized text");
        assert_eq!(recognition_result.confidence, 0.95);
        assert!(recognition_result.is_final);
    }

    #[test]
    fn test_process_audio_when_not_listening() {
        let recognizer = SpeechRecognizer::new(RecognitionConfig::default());
        let audio_data = vec![0u8; 100];
        let result = recognizer.process_audio(&audio_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Not currently listening"));
    }

    #[test]
    fn test_process_audio_empty_data() {
        let mut recognizer = SpeechRecognizer::new(RecognitionConfig::default());
        recognizer.start().unwrap();
        let audio_data: Vec<u8> = vec![];
        let result = recognizer.process_audio(&audio_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_listening() {
        let mut recognizer = SpeechRecognizer::new(RecognitionConfig::default());
        assert!(!recognizer.is_listening());
        recognizer.start().unwrap();
        assert!(recognizer.is_listening());
        recognizer.stop().unwrap();
        assert!(!recognizer.is_listening());
    }

    #[test]
    fn test_update_config() {
        let mut recognizer = SpeechRecognizer::new(RecognitionConfig::default());
        let new_config = RecognitionConfig {
            language: "es-ES".to_string(),
            continuous: true,
            interim_results: true,
            max_alternatives: 3,
        };
        recognizer.update_config(new_config.clone());
        assert_eq!(recognizer.get_config().language, "es-ES");
        assert!(recognizer.get_config().continuous);
    }

    #[test]
    fn test_get_config() {
        let config = RecognitionConfig::default();
        let recognizer = SpeechRecognizer::new(config.clone());
        let retrieved_config = recognizer.get_config();
        assert_eq!(retrieved_config.language, config.language);
        assert_eq!(retrieved_config.continuous, config.continuous);
    }

    #[test]
    fn test_get_supported_languages() {
        let recognizer = SpeechRecognizer::new(RecognitionConfig::default());
        let languages = recognizer.get_supported_languages();
        assert!(!languages.is_empty());
        assert!(languages.contains(&"en-US".to_string()));
        assert!(languages.contains(&"zh-CN".to_string()));
    }

    #[test]
    fn test_recognizer_default() {
        let recognizer = SpeechRecognizer::default();
        assert!(!recognizer.is_listening());
        assert_eq!(recognizer.get_config().language, "en-US");
    }

    #[test]
    fn test_recognition_config_default() {
        let config = RecognitionConfig::default();
        assert_eq!(config.language, "en-US");
        assert!(!config.continuous);
        assert!(!config.interim_results);
        assert_eq!(config.max_alternatives, 1);
    }

    #[test]
    fn test_recognition_config_creation() {
        let config = RecognitionConfig {
            language: "fr-FR".to_string(),
            continuous: true,
            interim_results: true,
            max_alternatives: 5,
        };
        assert_eq!(config.language, "fr-FR");
        assert!(config.continuous);
        assert_eq!(config.max_alternatives, 5);
    }

    #[test]
    fn test_recognition_result_creation() {
        let result = RecognitionResult {
            transcript: "Hello world".to_string(),
            confidence: 0.98,
            is_final: true,
            alternatives: vec!["Hello world".to_string()],
            language: "en-US".to_string(),
        };
        assert_eq!(result.transcript, "Hello world");
        assert_eq!(result.confidence, 0.98);
        assert!(result.is_final);
    }

    #[test]
    fn test_recognition_config_serialization() {
        let config = RecognitionConfig::default();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_recognition_config_deserialization() {
        let json = r#"{"language":"en-US","continuous":false,"interim_results":false,"max_alternatives":1}"#;
        let config: Result<RecognitionConfig, _> = serde_json::from_str(json);
        assert!(config.is_ok());
    }

    #[test]
    fn test_recognition_result_serialization() {
        let result = RecognitionResult {
            transcript: "test".to_string(),
            confidence: 0.9,
            is_final: true,
            alternatives: vec![],
            language: "en-US".to_string(),
        };
        let json = serde_json::to_string(&result);
        assert!(json.is_ok());
    }

    #[test]
    fn test_recognition_result_deserialization() {
        let json = r#"{"transcript":"test","confidence":0.9,"is_final":true,"alternatives":[],"language":"en-US"}"#;
        let result: Result<RecognitionResult, _> = serde_json::from_str(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_recognition_result_with_alternatives() {
        let result = RecognitionResult {
            transcript: "Hello".to_string(),
            confidence: 0.95,
            is_final: true,
            alternatives: vec!["Hello".to_string(), "Help".to_string()],
            language: "en-US".to_string(),
        };
        assert_eq!(result.alternatives.len(), 2);
    }

    #[test]
    fn test_recognition_result_empty_alternatives() {
        let result = RecognitionResult {
            transcript: "test".to_string(),
            confidence: 0.9,
            is_final: true,
            alternatives: vec![],
            language: "en-US".to_string(),
        };
        assert!(result.alternatives.is_empty());
    }

    #[test]
    fn test_recognition_result_not_final() {
        let result = RecognitionResult {
            transcript: "partial".to_string(),
            confidence: 0.5,
            is_final: false,
            alternatives: vec![],
            language: "en-US".to_string(),
        };
        assert!(!result.is_final);
    }

    #[test]
    fn test_config_with_custom_language() {
        let config = RecognitionConfig {
            language: "zh-CN".to_string(),
            ..Default::default()
        };
        assert_eq!(config.language, "zh-CN");
    }

    #[test]
    fn test_config_with_continuous_mode() {
        let config = RecognitionConfig {
            continuous: true,
            ..Default::default()
        };
        assert!(config.continuous);
    }

    #[test]
    fn test_config_with_interim_results() {
        let config = RecognitionConfig {
            interim_results: true,
            ..Default::default()
        };
        assert!(config.interim_results);
    }

    #[test]
    fn test_config_with_max_alternatives() {
        let config = RecognitionConfig {
            max_alternatives: 10,
            ..Default::default()
        };
        assert_eq!(config.max_alternatives, 10);
    }

    #[test]
    fn test_process_audio_large_data() {
        let mut recognizer = SpeechRecognizer::new(RecognitionConfig::default());
        recognizer.start().unwrap();
        let audio_data = vec![0u8; 10000];
        let result = recognizer.process_audio(&audio_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_start_stop_cycles() {
        let mut recognizer = SpeechRecognizer::new(RecognitionConfig::default());
        for _ in 0..3 {
            recognizer.start().unwrap();
            assert!(recognizer.is_listening());
            recognizer.stop().unwrap();
            assert!(!recognizer.is_listening());
        }
    }
}
