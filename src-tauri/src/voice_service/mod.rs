pub mod speech_recognition;
pub mod text_to_speech;

pub use speech_recognition::{RecognitionConfig, RecognitionResult, SpeechRecognizer};
pub use text_to_speech::{TTSConfig, TextToSpeech, Voice};
