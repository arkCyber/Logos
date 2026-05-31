//! Presentation Playback Module
//! 
//! Aerospace-grade presentation playback implementation with:
//! - Input validation
//! - Bounds checking
//! - Comprehensive error handling
//! - Playback state management
//! - Performance monitoring

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Playback state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlaybackState {
    /// Stopped
    Stopped,
    /// Playing
    Playing,
    /// Paused
    Paused,
    /// Ended
    Ended,
}

/// Playback mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlaybackMode {
    /// Normal playback
    Normal,
    /// From current slide
    FromCurrent,
    /// From beginning
    FromBeginning,
    /// From end
    FromEnd,
    /// Custom slide index
    FromSlide(usize),
}

/// Playback controller
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackController {
    /// Controller ID
    pub id: String,
    /// Current slide index
    pub current_slide_index: usize,
    /// Total slide count
    pub total_slides: usize,
    /// Playback state
    pub state: PlaybackState,
    /// Playback mode
    pub mode: PlaybackMode,
    /// Auto-advance enabled
    pub auto_advance: bool,
    /// Loop playback
    pub loop_playback: bool,
    /// Current position in seconds
    pub current_position: f64,
    /// Total duration in seconds
    pub total_duration: f64,
    /// Slide timings (slide index -> duration in seconds)
    pub slide_timings: HashMap<usize, f64>,
    /// Whether to show animations
    pub show_animations: bool,
    /// Whether to play media
    pub play_media: bool,
}

impl PlaybackController {
    /// Maximum slide count to prevent memory exhaustion
    const MAX_SLIDE_COUNT: usize = 1000;

    /// Maximum duration in seconds (24 hours)
    const MAX_DURATION: f64 = 86400.0;

    /// Create a new playback controller
    pub fn new(id: String, total_slides: usize) -> Self {
        Self {
            id,
            current_slide_index: 0,
            total_slides,
            state: PlaybackState::Stopped,
            mode: PlaybackMode::Normal,
            auto_advance: true,
            loop_playback: false,
            current_position: 0.0,
            total_duration: 0.0,
            slide_timings: HashMap::new(),
            show_animations: true,
            play_media: true,
        }
    }

    /// Set playback mode
    pub fn with_mode(mut self, mode: PlaybackMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set auto-advance
    pub fn with_auto_advance(mut self, auto_advance: bool) -> Self {
        self.auto_advance = auto_advance;
        self
    }

    /// Set loop playback
    pub fn with_loop_playback(mut self, loop_playback: bool) -> Self {
        self.loop_playback = loop_playback;
        self
    }

    /// Set show animations
    pub fn with_show_animations(mut self, show_animations: bool) -> Self {
        self.show_animations = show_animations;
        self
    }

    /// Set play media
    pub fn with_play_media(mut self, play_media: bool) -> Self {
        self.play_media = play_media;
        self
    }

    /// Add slide timing
    pub fn add_slide_timing(&mut self, slide_index: usize, duration: f64) -> Result<(), String> {
        if slide_index >= self.total_slides {
            return Err(format!("Slide index {} exceeds total slide count {}", slide_index, self.total_slides));
        }

        if duration < 0.0 {
            return Err("Slide duration cannot be negative".to_string());
        }

        if duration > Self::MAX_DURATION {
            return Err(format!("Slide duration exceeds maximum of {} seconds", Self::MAX_DURATION));
        }

        self.slide_timings.insert(slide_index, duration);
        self.total_duration = self.slide_timings.values().sum();
        Ok(())
    }

    /// Start playback
    pub fn start(&mut self) -> Result<(), String> {
        self.validate()?;

        match self.mode {
            PlaybackMode::FromBeginning => {
                self.current_slide_index = 0;
            }
            PlaybackMode::FromEnd => {
                self.current_slide_index = self.total_slides.saturating_sub(1);
            }
            PlaybackMode::FromSlide(index) => {
                if index >= self.total_slides {
                    return Err(format!("Slide index {} exceeds total slide count {}", index, self.total_slides));
                }
                self.current_slide_index = index;
            }
            PlaybackMode::FromCurrent | PlaybackMode::Normal => {
                // Keep current slide index
            }
        }

        self.state = PlaybackState::Playing;
        self.current_position = 0.0;
        Ok(())
    }

    /// Pause playback
    pub fn pause(&mut self) -> Result<(), String> {
        if self.state != PlaybackState::Playing {
            return Err("Cannot pause: playback is not playing".to_string());
        }

        self.state = PlaybackState::Paused;
        Ok(())
    }

    /// Resume playback
    pub fn resume(&mut self) -> Result<(), String> {
        if self.state != PlaybackState::Paused {
            return Err("Cannot resume: playback is not paused".to_string());
        }

        self.state = PlaybackState::Playing;
        Ok(())
    }

    /// Stop playback
    pub fn stop(&mut self) -> Result<(), String> {
        self.state = PlaybackState::Stopped;
        self.current_position = 0.0;
        Ok(())
    }

    /// Go to next slide
    pub fn next_slide(&mut self) -> Result<(), String> {
        if self.current_slide_index + 1 >= self.total_slides {
            if self.loop_playback {
                self.current_slide_index = 0;
            } else {
                self.state = PlaybackState::Ended;
                return Err("Already at last slide".to_string());
            }
        } else {
            self.current_slide_index += 1;
        }

        Ok(())
    }

    /// Go to previous slide
    pub fn previous_slide(&mut self) -> Result<(), String> {
        if self.current_slide_index == 0 {
            if self.loop_playback {
                self.current_slide_index = self.total_slides.saturating_sub(1);
            } else {
                return Err("Already at first slide".to_string());
            }
        } else {
            self.current_slide_index -= 1;
        }

        Ok(())
    }

    /// Go to specific slide
    pub fn go_to_slide(&mut self, slide_index: usize) -> Result<(), String> {
        if slide_index >= self.total_slides {
            return Err(format!("Slide index {} exceeds total slide count {}", slide_index, self.total_slides));
        }

        self.current_slide_index = slide_index;
        Ok(())
    }

    /// Update current position
    pub fn update_position(&mut self, position: f64) -> Result<(), String> {
        if position < 0.0 {
            return Err("Position cannot be negative".to_string());
        }

        if position > self.total_duration {
            return Err(format!("Position exceeds total duration of {} seconds", self.total_duration));
        }

        self.current_position = position;
        Ok(())
    }

    /// Get current slide duration
    pub fn current_slide_duration(&self) -> f64 {
        self.slide_timings.get(&self.current_slide_index).copied().unwrap_or(0.0)
    }

    /// Get progress (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.total_duration == 0.0 {
            0.0
        } else {
            (self.current_position / self.total_duration).clamp(0.0, 1.0)
        }
    }

    /// Validate playback controller
    pub fn validate(&self) -> Result<(), String> {
        // Validate ID
        if self.id.trim().is_empty() {
            return Err("Controller ID cannot be empty".to_string());
        }

        // Validate slide count
        if self.total_slides == 0 {
            return Err("Total slide count cannot be zero".to_string());
        }

        if self.total_slides > Self::MAX_SLIDE_COUNT {
            return Err(format!("Total slide count exceeds maximum of {}", Self::MAX_SLIDE_COUNT));
        }

        // Validate current slide index
        if self.current_slide_index >= self.total_slides {
            return Err(format!(
                "Current slide index {} exceeds total slide count {}",
                self.current_slide_index, self.total_slides
            ));
        }

        // Validate total duration
        if self.total_duration < 0.0 {
            return Err("Total duration cannot be negative".to_string());
        }

        if self.total_duration > Self::MAX_DURATION {
            return Err(format!("Total duration exceeds maximum of {} seconds", Self::MAX_DURATION));
        }

        // Validate current position
        if self.current_position < 0.0 {
            return Err("Current position cannot be negative".to_string());
        }

        if self.current_position > self.total_duration {
            return Err("Current position exceeds total duration".to_string());
        }

        Ok(())
    }

    /// Check if playback can advance
    pub fn can_advance(&self) -> bool {
        self.current_slide_index + 1 < self.total_slides || self.loop_playback
    }

    /// Check if playback can go back
    pub fn can_go_back(&self) -> bool {
        self.current_slide_index > 0 || self.loop_playback
    }
}

impl Default for PlaybackController {
    fn default() -> Self {
        Self::new("default".to_string(), 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playback_controller_new() {
        let controller = PlaybackController::new("1".to_string(), 10);
        assert_eq!(controller.id, "1");
        assert_eq!(controller.total_slides, 10);
        assert_eq!(controller.current_slide_index, 0);
        assert_eq!(controller.state, PlaybackState::Stopped);
    }

    #[test]
    fn test_playback_controller_with_mode() {
        let controller = PlaybackController::new("1".to_string(), 10)
            .with_mode(PlaybackMode::FromBeginning);
        assert_eq!(controller.mode, PlaybackMode::FromBeginning);
    }

    #[test]
    fn test_playback_controller_start() {
        let mut controller = PlaybackController::new("1".to_string(), 10);
        controller.start().unwrap();
        assert_eq!(controller.state, PlaybackState::Playing);
        assert_eq!(controller.current_position, 0.0);
    }

    #[test]
    fn test_playback_controller_pause() {
        let mut controller = PlaybackController::new("1".to_string(), 10);
        controller.start().unwrap();
        controller.pause().unwrap();
        assert_eq!(controller.state, PlaybackState::Paused);
    }

    #[test]
    fn test_playback_controller_resume() {
        let mut controller = PlaybackController::new("1".to_string(), 10);
        controller.start().unwrap();
        controller.pause().unwrap();
        controller.resume().unwrap();
        assert_eq!(controller.state, PlaybackState::Playing);
    }

    #[test]
    fn test_playback_controller_stop() {
        let mut controller = PlaybackController::new("1".to_string(), 10);
        controller.start().unwrap();
        controller.stop().unwrap();
        assert_eq!(controller.state, PlaybackState::Stopped);
        assert_eq!(controller.current_position, 0.0);
    }

    #[test]
    fn test_playback_controller_next_slide() {
        let mut controller = PlaybackController::new("1".to_string(), 10);
        controller.next_slide().unwrap();
        assert_eq!(controller.current_slide_index, 1);
    }

    #[test]
    fn test_playback_controller_next_slide_at_end() {
        let mut controller = PlaybackController::new("1".to_string(), 10);
        controller.current_slide_index = 9;
        let result = controller.next_slide();
        assert!(result.is_err());
    }

    #[test]
    fn test_playback_controller_next_slide_with_loop() {
        let mut controller = PlaybackController::new("1".to_string(), 10)
            .with_loop_playback(true);
        controller.current_slide_index = 9;
        controller.next_slide().unwrap();
        assert_eq!(controller.current_slide_index, 0);
    }

    #[test]
    fn test_playback_controller_previous_slide() {
        let mut controller = PlaybackController::new("1".to_string(), 10);
        controller.current_slide_index = 5;
        controller.previous_slide().unwrap();
        assert_eq!(controller.current_slide_index, 4);
    }

    #[test]
    fn test_playback_controller_previous_slide_at_start() {
        let mut controller = PlaybackController::new("1".to_string(), 10);
        let result = controller.previous_slide();
        assert!(result.is_err());
    }

    #[test]
    fn test_playback_controller_go_to_slide() {
        let mut controller = PlaybackController::new("1".to_string(), 10);
        controller.go_to_slide(5).unwrap();
        assert_eq!(controller.current_slide_index, 5);
    }

    #[test]
    fn test_playback_controller_go_to_slide_invalid() {
        let mut controller = PlaybackController::new("1".to_string(), 10);
        let result = controller.go_to_slide(15);
        assert!(result.is_err());
    }

    #[test]
    fn test_playback_controller_add_slide_timing() {
        let mut controller = PlaybackController::new("1".to_string(), 10);
        controller.add_slide_timing(0, 5.0).unwrap();
        assert_eq!(controller.slide_timings.get(&0), Some(&5.0));
        assert_eq!(controller.total_duration, 5.0);
    }

    #[test]
    fn test_playback_controller_add_slide_timing_invalid_index() {
        let mut controller = PlaybackController::new("1".to_string(), 10);
        let result = controller.add_slide_timing(15, 5.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_playback_controller_update_position() {
        let mut controller = PlaybackController::new("1".to_string(), 10);
        controller.add_slide_timing(0, 10.0).unwrap();
        controller.update_position(5.0).unwrap();
        assert_eq!(controller.current_position, 5.0);
    }

    #[test]
    fn test_playback_controller_progress() {
        let mut controller = PlaybackController::new("1".to_string(), 10);
        controller.add_slide_timing(0, 10.0).unwrap();
        controller.update_position(5.0).unwrap();
        assert_eq!(controller.progress(), 0.5);
    }

    #[test]
    fn test_playback_controller_validate_empty_id() {
        let controller = PlaybackController::new("".to_string(), 10);
        assert!(controller.validate().is_err());
    }

    #[test]
    fn test_playback_controller_validate_zero_slides() {
        let controller = PlaybackController::new("1".to_string(), 0);
        assert!(controller.validate().is_err());
    }

    #[test]
    fn test_playback_controller_can_advance() {
        let controller = PlaybackController::new("1".to_string(), 10);
        assert!(controller.can_advance());
    }

    #[test]
    fn test_playback_controller_can_advance_at_end() {
        let mut controller = PlaybackController::new("1".to_string(), 10);
        controller.current_slide_index = 9;
        assert!(!controller.can_advance());
    }

    #[test]
    fn test_playback_controller_can_advance_with_loop() {
        let mut controller = PlaybackController::new("1".to_string(), 10)
            .with_loop_playback(true);
        controller.current_slide_index = 9;
        assert!(controller.can_advance());
    }

    #[test]
    fn test_playback_controller_serialization() {
        let controller = PlaybackController::new("1".to_string(), 10);
        let json = serde_json::to_string(&controller);
        assert!(json.is_ok());
    }

    #[test]
    fn test_playback_controller_deserialization() {
        let controller = PlaybackController::new("1".to_string(), 10);
        let json = serde_json::to_string(&controller).unwrap();
        let deserialized: PlaybackController = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, controller.id);
        assert_eq!(deserialized.total_slides, controller.total_slides);
    }
}
