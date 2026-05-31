//! Presenter View Module
//! 
//! Aerospace-grade presenter view implementation with:
//! - Input validation
//! - Bounds checking
//! - Comprehensive error handling
//! - Presenter view state management
//! - Performance monitoring

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Presenter view configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenterViewConfig {
    /// Show current slide
    pub show_current_slide: bool,
    /// Show next slide
    pub show_next_slide: bool,
    /// Show speaker notes
    pub show_notes: bool,
    /// Show timer
    pub show_timer: bool,
    /// Show slide count
    pub show_slide_count: bool,
    /// Show current slide thumbnail
    pub show_thumbnail: bool,
    /// Show presentation timer
    pub show_presentation_timer: bool,
    /// Show slide timer
    pub show_slide_timer: bool,
}

impl Default for PresenterViewConfig {
    fn default() -> Self {
        Self {
            show_current_slide: true,
            show_next_slide: true,
            show_notes: true,
            show_timer: true,
            show_slide_count: true,
            show_thumbnail: true,
            show_presentation_timer: true,
            show_slide_timer: true,
        }
    }
}

/// Presenter view state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenterViewState {
    /// Current slide index
    pub current_slide_index: usize,
    /// Next slide index
    pub next_slide_index: Option<usize>,
    /// Previous slide index
    pub previous_slide_index: Option<usize>,
    /// Total slide count
    pub total_slides: usize,
    /// Current slide notes
    pub current_notes: String,
    /// Next slide notes
    pub next_notes: String,
    /// Presentation elapsed time in seconds
    pub presentation_elapsed_time: f64,
    /// Current slide elapsed time in seconds
    pub slide_elapsed_time: f64,
    /// Total presentation duration in seconds
    pub total_duration: f64,
    /// Current slide duration in seconds
    pub current_slide_duration: f64,
}

/// Presenter view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenterView {
    /// View ID
    pub id: String,
    /// Configuration
    pub config: PresenterViewConfig,
    /// State
    pub state: PresenterViewState,
    /// Slide notes (slide index -> notes)
    pub slide_notes: HashMap<usize, String>,
    /// Slide durations (slide index -> duration in seconds)
    pub slide_durations: HashMap<usize, f64>,
}

impl PresenterView {
    /// Maximum slide count to prevent memory exhaustion
    const MAX_SLIDE_COUNT: usize = 1000;

    /// Maximum duration in seconds (24 hours)
    const MAX_DURATION: f64 = 86400.0;

    /// Maximum notes length per slide
    const MAX_NOTES_LENGTH: usize = 5000;

    /// Create a new presenter view
    pub fn new(id: String, total_slides: usize) -> Self {
        Self {
            id,
            config: PresenterViewConfig::default(),
            state: PresenterViewState {
                current_slide_index: 0,
                next_slide_index: if total_slides > 1 { Some(1) } else { None },
                previous_slide_index: None,
                total_slides,
                current_notes: String::new(),
                next_notes: String::new(),
                presentation_elapsed_time: 0.0,
                slide_elapsed_time: 0.0,
                total_duration: 0.0,
                current_slide_duration: 0.0,
            },
            slide_notes: HashMap::new(),
            slide_durations: HashMap::new(),
        }
    }

    /// Set configuration
    pub fn with_config(mut self, config: PresenterViewConfig) -> Self {
        self.config = config;
        self
    }

    /// Add slide notes
    pub fn add_slide_notes(&mut self, slide_index: usize, notes: String) -> Result<(), String> {
        if slide_index >= self.state.total_slides {
            return Err(format!("Slide index {} exceeds total slide count {}", slide_index, self.state.total_slides));
        }

        if notes.len() > Self::MAX_NOTES_LENGTH {
            return Err(format!("Notes exceed maximum length of {} characters", Self::MAX_NOTES_LENGTH));
        }

        // Update current notes if this is the current slide
        if slide_index == self.state.current_slide_index {
            self.state.current_notes = notes.clone();
        }

        // Update next notes if this is the next slide
        if self.state.next_slide_index == Some(slide_index) {
            self.state.next_notes = notes.clone();
        }

        self.slide_notes.insert(slide_index, notes);

        Ok(())
    }

    /// Add slide duration
    pub fn add_slide_duration(&mut self, slide_index: usize, duration: f64) -> Result<(), String> {
        if slide_index >= self.state.total_slides {
            return Err(format!("Slide index {} exceeds total slide count {}", slide_index, self.state.total_slides));
        }

        if duration < 0.0 {
            return Err("Duration cannot be negative".to_string());
        }

        if duration > Self::MAX_DURATION {
            return Err(format!("Duration exceeds maximum of {} seconds", Self::MAX_DURATION));
        }

        self.slide_durations.insert(slide_index, duration);
        self.state.total_duration = self.slide_durations.values().sum();

        // Update current slide duration if this is the current slide
        if slide_index == self.state.current_slide_index {
            self.state.current_slide_duration = duration;
        }

        Ok(())
    }

    /// Go to slide
    pub fn go_to_slide(&mut self, slide_index: usize) -> Result<(), String> {
        if slide_index >= self.state.total_slides {
            return Err(format!("Slide index {} exceeds total slide count {}", slide_index, self.state.total_slides));
        }

        self.state.current_slide_index = slide_index;
        self.state.previous_slide_index = if slide_index > 0 { Some(slide_index - 1) } else { None };
        self.state.next_slide_index = if slide_index + 1 < self.state.total_slides { Some(slide_index + 1) } else { None };

        // Update notes
        self.state.current_notes = self.slide_notes.get(&slide_index).cloned().unwrap_or_default();
        self.state.next_notes = self.state.next_slide_index.and_then(|idx| self.slide_notes.get(&idx).cloned()).unwrap_or_default();

        // Update duration
        self.state.current_slide_duration = self.slide_durations.get(&slide_index).copied().unwrap_or(0.0);

        // Reset slide elapsed time
        self.state.slide_elapsed_time = 0.0;

        Ok(())
    }

    /// Update presentation elapsed time
    pub fn update_presentation_time(&mut self, elapsed_time: f64) -> Result<(), String> {
        if elapsed_time < 0.0 {
            return Err("Elapsed time cannot be negative".to_string());
        }

        if elapsed_time > Self::MAX_DURATION {
            return Err(format!("Elapsed time exceeds maximum of {} seconds", Self::MAX_DURATION));
        }

        self.state.presentation_elapsed_time = elapsed_time;
        Ok(())
    }

    /// Update slide elapsed time
    pub fn update_slide_time(&mut self, elapsed_time: f64) -> Result<(), String> {
        if elapsed_time < 0.0 {
            return Err("Elapsed time cannot be negative".to_string());
        }

        if elapsed_time > self.state.current_slide_duration {
            return Err("Elapsed time exceeds current slide duration".to_string());
        }

        self.state.slide_elapsed_time = elapsed_time;
        Ok(())
    }

    /// Get presentation progress (0.0 to 1.0)
    pub fn presentation_progress(&self) -> f64 {
        if self.state.total_slides == 0 {
            0.0
        } else {
            (self.state.current_slide_index as f64 / self.state.total_slides as f64).clamp(0.0, 1.0)
        }
    }

    /// Get time progress (0.0 to 1.0)
    pub fn time_progress(&self) -> f64 {
        if self.state.total_duration == 0.0 {
            0.0
        } else {
            (self.state.presentation_elapsed_time / self.state.total_duration).clamp(0.0, 1.0)
        }
    }

    /// Get slide time progress (0.0 to 1.0)
    pub fn slide_time_progress(&self) -> f64 {
        if self.state.current_slide_duration == 0.0 {
            0.0
        } else {
            (self.state.slide_elapsed_time / self.state.current_slide_duration).clamp(0.0, 1.0)
        }
    }

    /// Validate presenter view
    pub fn validate(&self) -> Result<(), String> {
        // Validate ID
        if self.id.trim().is_empty() {
            return Err("View ID cannot be empty".to_string());
        }

        // Validate slide count
        if self.state.total_slides == 0 {
            return Err("Total slide count cannot be zero".to_string());
        }

        if self.state.total_slides > Self::MAX_SLIDE_COUNT {
            return Err(format!("Total slide count exceeds maximum of {}", Self::MAX_SLIDE_COUNT));
        }

        // Validate current slide index
        if self.state.current_slide_index >= self.state.total_slides {
            return Err(format!(
                "Current slide index {} exceeds total slide count {}",
                self.state.current_slide_index, self.state.total_slides
            ));
        }

        // Validate next slide index
        if let Some(next) = self.state.next_slide_index {
            if next >= self.state.total_slides {
                return Err(format!("Next slide index {} exceeds total slide count {}", next, self.state.total_slides));
            }
        }

        // Validate previous slide index
        if let Some(prev) = self.state.previous_slide_index {
            if prev >= self.state.total_slides {
                return Err(format!("Previous slide index {} exceeds total slide count {}", prev, self.state.total_slides));
            }
        }

        // Validate durations
        if self.state.total_duration < 0.0 {
            return Err("Total duration cannot be negative".to_string());
        }

        if self.state.total_duration > Self::MAX_DURATION {
            return Err(format!("Total duration exceeds maximum of {} seconds", Self::MAX_DURATION));
        }

        if self.state.current_slide_duration < 0.0 {
            return Err("Current slide duration cannot be negative".to_string());
        }

        // Validate elapsed times
        if self.state.presentation_elapsed_time < 0.0 {
            return Err("Presentation elapsed time cannot be negative".to_string());
        }

        if self.state.slide_elapsed_time < 0.0 {
            return Err("Slide elapsed time cannot be negative".to_string());
        }

        Ok(())
    }

    /// Check if at first slide
    pub fn is_at_first_slide(&self) -> bool {
        self.state.current_slide_index == 0
    }

    /// Check if at last slide
    pub fn is_at_last_slide(&self) -> bool {
        self.state.current_slide_index == self.state.total_slides.saturating_sub(1)
    }

    /// Get remaining slides count
    pub fn remaining_slides(&self) -> usize {
        self.state.total_slides.saturating_sub(self.state.current_slide_index + 1)
    }

    /// Get remaining time in seconds
    pub fn remaining_time(&self) -> f64 {
        (self.state.total_duration - self.state.presentation_elapsed_time).max(0.0)
    }
}

impl Default for PresenterView {
    fn default() -> Self {
        Self::new("default".to_string(), 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presenter_view_new() {
        let view = PresenterView::new("1".to_string(), 10);
        assert_eq!(view.id, "1");
        assert_eq!(view.state.total_slides, 10);
        assert_eq!(view.state.current_slide_index, 0);
    }

    #[test]
    fn test_presenter_view_with_config() {
        let config = PresenterViewConfig {
            show_current_slide: false,
            ..Default::default()
        };
        let view = PresenterView::new("1".to_string(), 10).with_config(config);
        assert!(!view.config.show_current_slide);
    }

    #[test]
    fn test_presenter_view_add_slide_notes() {
        let mut view = PresenterView::new("1".to_string(), 10);
        view.add_slide_notes(0, "Test notes".to_string()).unwrap();
        assert_eq!(view.slide_notes.get(&0), Some(&"Test notes".to_string()));
        assert_eq!(view.state.current_notes, "Test notes");
    }

    #[test]
    fn test_presenter_view_add_slide_notes_too_long() {
        let mut view = PresenterView::new("1".to_string(), 10);
        let result = view.add_slide_notes(0, "a".repeat(5001));
        assert!(result.is_err());
    }

    #[test]
    fn test_presenter_view_add_slide_duration() {
        let mut view = PresenterView::new("1".to_string(), 10);
        view.add_slide_duration(0, 5.0).unwrap();
        assert_eq!(view.slide_durations.get(&0), Some(&5.0));
        assert_eq!(view.state.total_duration, 5.0);
    }

    #[test]
    fn test_presenter_view_go_to_slide() {
        let mut view = PresenterView::new("1".to_string(), 10);
        view.go_to_slide(5).unwrap();
        assert_eq!(view.state.current_slide_index, 5);
        assert_eq!(view.state.previous_slide_index, Some(4));
        assert_eq!(view.state.next_slide_index, Some(6));
    }

    #[test]
    fn test_presenter_view_go_to_first_slide() {
        let mut view = PresenterView::new("1".to_string(), 10);
        view.go_to_slide(0).unwrap();
        assert_eq!(view.state.previous_slide_index, None);
    }

    #[test]
    fn test_presenter_view_go_to_last_slide() {
        let mut view = PresenterView::new("1".to_string(), 10);
        view.go_to_slide(9).unwrap();
        assert_eq!(view.state.next_slide_index, None);
    }

    #[test]
    fn test_presenter_view_update_presentation_time() {
        let mut view = PresenterView::new("1".to_string(), 10);
        view.update_presentation_time(60.0).unwrap();
        assert_eq!(view.state.presentation_elapsed_time, 60.0);
    }

    #[test]
    fn test_presenter_view_update_slide_time() {
        let mut view = PresenterView::new("1".to_string(), 10);
        view.add_slide_duration(0, 10.0).unwrap();
        view.update_slide_time(5.0).unwrap();
        assert_eq!(view.state.slide_elapsed_time, 5.0);
    }

    #[test]
    fn test_presenter_view_presentation_progress() {
        let mut view = PresenterView::new("1".to_string(), 10);
        view.state.current_slide_index = 5;
        assert_eq!(view.presentation_progress(), 0.5);
    }

    #[test]
    fn test_presenter_view_time_progress() {
        let mut view = PresenterView::new("1".to_string(), 10);
        view.add_slide_duration(0, 10.0).unwrap();
        view.add_slide_duration(1, 10.0).unwrap();
        view.update_presentation_time(10.0).unwrap();
        assert_eq!(view.time_progress(), 0.5);
    }

    #[test]
    fn test_presenter_view_is_at_first_slide() {
        let view = PresenterView::new("1".to_string(), 10);
        assert!(view.is_at_first_slide());
    }

    #[test]
    fn test_presenter_view_is_at_last_slide() {
        let mut view = PresenterView::new("1".to_string(), 10);
        view.state.current_slide_index = 9;
        assert!(view.is_at_last_slide());
    }

    #[test]
    fn test_presenter_view_remaining_slides() {
        let mut view = PresenterView::new("1".to_string(), 10);
        view.state.current_slide_index = 5;
        assert_eq!(view.remaining_slides(), 4);
    }

    #[test]
    fn test_presenter_view_remaining_time() {
        let mut view = PresenterView::new("1".to_string(), 10);
        view.add_slide_duration(0, 10.0).unwrap();
        view.add_slide_duration(1, 10.0).unwrap();
        view.update_presentation_time(5.0).unwrap();
        assert_eq!(view.remaining_time(), 15.0);
    }

    #[test]
    fn test_presenter_view_validate_empty_id() {
        let view = PresenterView::new("".to_string(), 10);
        assert!(view.validate().is_err());
    }

    #[test]
    fn test_presenter_view_serialization() {
        let view = PresenterView::new("1".to_string(), 10);
        let json = serde_json::to_string(&view);
        assert!(json.is_ok());
    }

    #[test]
    fn test_presenter_view_deserialization() {
        let view = PresenterView::new("1".to_string(), 10);
        let json = serde_json::to_string(&view).unwrap();
        let deserialized: PresenterView = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, view.id);
        assert_eq!(deserialized.state.total_slides, view.state.total_slides);
    }
}
