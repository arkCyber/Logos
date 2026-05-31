//! Rehearsal Timing Module
//! 
//! Aerospace-grade rehearsal timing implementation with:
//! - Input validation
//! - Bounds checking
//! - Comprehensive error handling
//! - Timing state management
//! - Performance monitoring

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Rehearsal state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RehearsalState {
    /// Not started
    NotStarted,
    /// Recording
    Recording,
    /// Paused
    Paused,
    /// Completed
    Completed,
}

/// Slide timing record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlideTimingRecord {
    /// Slide index
    pub slide_index: usize,
    /// Duration in seconds
    pub duration: f64,
    /// Notes
    pub notes: String,
}

/// Rehearsal timer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RehearsalTimer {
    /// Timer ID
    pub id: String,
    /// Total slide count
    pub total_slides: usize,
    /// Current slide index
    pub current_slide_index: usize,
    /// Rehearsal state
    pub state: RehearsalState,
    /// Current slide start time
    pub current_slide_start_time: f64,
    /// Total elapsed time in seconds
    pub total_elapsed_time: f64,
    /// Current slide elapsed time in seconds
    pub current_slide_elapsed_time: f64,
    /// Slide timing records
    pub slide_timings: HashMap<usize, SlideTimingRecord>,
    /// Target total duration in seconds (optional)
    pub target_duration: Option<f64>,
    /// Target slide durations (slide index -> target duration in seconds)
    pub target_slide_durations: HashMap<usize, f64>,
}

impl RehearsalTimer {
    /// Maximum slide count to prevent memory exhaustion
    const MAX_SLIDE_COUNT: usize = 1000;

    /// Maximum duration in seconds (24 hours)
    const MAX_DURATION: f64 = 86400.0;

    /// Maximum notes length per slide
    const MAX_NOTES_LENGTH: usize = 1000;

    /// Create a new rehearsal timer
    pub fn new(id: String, total_slides: usize) -> Self {
        Self {
            id,
            total_slides,
            current_slide_index: 0,
            state: RehearsalState::NotStarted,
            current_slide_start_time: 0.0,
            total_elapsed_time: 0.0,
            current_slide_elapsed_time: 0.0,
            slide_timings: HashMap::new(),
            target_duration: None,
            target_slide_durations: HashMap::new(),
        }
    }

    /// Set target duration
    pub fn with_target_duration(mut self, duration: f64) -> Self {
        self.target_duration = Some(duration.clamp(0.0, Self::MAX_DURATION));
        self
    }

    /// Set target slide duration
    pub fn with_target_slide_duration(mut self, slide_index: usize, duration: f64) -> Self {
        self.target_slide_durations.insert(slide_index, duration.clamp(0.0, Self::MAX_DURATION));
        self
    }

    /// Start rehearsal
    pub fn start(&mut self) -> Result<(), String> {
        self.validate()?;

        if self.state == RehearsalState::Recording {
            return Err("Rehearsal is already recording".to_string());
        }

        self.state = RehearsalState::Recording;
        self.current_slide_start_time = 0.0;
        self.current_slide_elapsed_time = 0.0;
        self.total_elapsed_time = 0.0;
        Ok(())
    }

    /// Pause rehearsal
    pub fn pause(&mut self) -> Result<(), String> {
        if self.state != RehearsalState::Recording {
            return Err("Cannot pause: rehearsal is not recording".to_string());
        }

        self.state = RehearsalState::Paused;
        Ok(())
    }

    /// Resume rehearsal
    pub fn resume(&mut self) -> Result<(), String> {
        if self.state != RehearsalState::Paused {
            return Err("Cannot resume: rehearsal is not paused".to_string());
        }

        self.state = RehearsalState::Recording;
        Ok(())
    }

    /// Stop rehearsal
    pub fn stop(&mut self) -> Result<(), String> {
        if self.state == RehearsalState::NotStarted {
            return Err("Cannot stop: rehearsal has not started".to_string());
        }

        // Record current slide timing
        if self.state == RehearsalState::Recording {
            self.record_current_slide()?;
        }

        self.state = RehearsalState::Completed;
        Ok(())
    }

    /// Go to next slide
    pub fn next_slide(&mut self) -> Result<(), String> {
        if self.state != RehearsalState::Recording {
            return Err("Cannot advance: rehearsal is not recording".to_string());
        }

        // Record current slide timing
        self.record_current_slide()?;

        // Move to next slide
        if self.current_slide_index + 1 >= self.total_slides {
            self.state = RehearsalState::Completed;
            return Err("Already at last slide".to_string());
        }

        self.current_slide_index += 1;
        self.current_slide_start_time = self.total_elapsed_time;
        self.current_slide_elapsed_time = 0.0;
        Ok(())
    }

    /// Go to previous slide
    pub fn previous_slide(&mut self) -> Result<(), String> {
        if self.state != RehearsalState::Recording {
            return Err("Cannot go back: rehearsal is not recording".to_string());
        }

        if self.current_slide_index == 0 {
            return Err("Already at first slide".to_string());
        }

        // Remove current slide timing if exists
        self.slide_timings.remove(&self.current_slide_index);

        self.current_slide_index -= 1;
        self.current_slide_start_time = self.slide_timings.get(&self.current_slide_index)
            .map(|r| self.total_elapsed_time - r.duration)
            .unwrap_or(0.0);
        self.current_slide_elapsed_time = 0.0;
        Ok(())
    }

    /// Go to specific slide
    pub fn go_to_slide(&mut self, slide_index: usize) -> Result<(), String> {
        if slide_index >= self.total_slides {
            return Err(format!("Slide index {} exceeds total slide count {}", slide_index, self.total_slides));
        }

        if self.state != RehearsalState::Recording {
            return Err("Cannot go to slide: rehearsal is not recording".to_string());
        }

        // Record current slide timing
        self.record_current_slide()?;

        self.current_slide_index = slide_index;
        self.current_slide_start_time = self.total_elapsed_time;
        self.current_slide_elapsed_time = 0.0;
        Ok(())
    }

    /// Update elapsed time
    pub fn update_elapsed_time(&mut self, elapsed_time: f64) -> Result<(), String> {
        if elapsed_time < 0.0 {
            return Err("Elapsed time cannot be negative".to_string());
        }

        if elapsed_time > Self::MAX_DURATION {
            return Err(format!("Elapsed time exceeds maximum of {} seconds", Self::MAX_DURATION));
        }

        self.total_elapsed_time = elapsed_time;
        self.current_slide_elapsed_time = elapsed_time - self.current_slide_start_time;
        Ok(())
    }

    /// Add notes for current slide
    pub fn add_notes(&mut self, notes: String) -> Result<(), String> {
        if notes.len() > Self::MAX_NOTES_LENGTH {
            return Err(format!("Notes exceed maximum length of {} characters", Self::MAX_NOTES_LENGTH));
        }

        let record = self.slide_timings.entry(self.current_slide_index)
            .or_insert_with(|| SlideTimingRecord {
                slide_index: self.current_slide_index,
                duration: 0.0,
                notes: String::new(),
            });

        record.notes = notes;
        Ok(())
    }

    /// Record current slide timing
    fn record_current_slide(&mut self) -> Result<(), String> {
        let duration = self.current_slide_elapsed_time;
        if duration <= 0.0 {
            return Ok(()); // Skip recording if no time elapsed
        }

        let record = self.slide_timings.entry(self.current_slide_index)
            .or_insert_with(|| SlideTimingRecord {
                slide_index: self.current_slide_index,
                duration: 0.0,
                notes: String::new(),
            });

        record.duration = duration;
        Ok(())
    }

    /// Get slide timing
    pub fn get_slide_timing(&self, slide_index: usize) -> Option<&SlideTimingRecord> {
        self.slide_timings.get(&slide_index)
    }

    /// Get total recorded duration
    pub fn total_recorded_duration(&self) -> f64 {
        self.slide_timings.values().map(|r| r.duration).sum()
    }

    /// Get comparison with target duration
    pub fn compare_with_target(&self) -> Option<f64> {
        self.target_duration.map(|target| {
            let recorded = self.total_recorded_duration();
            recorded - target
        })
    }

    /// Get slide comparison with target
    pub fn compare_slide_with_target(&self, slide_index: usize) -> Option<f64> {
        self.target_slide_durations.get(&slide_index).map(|target| {
            let recorded = self.slide_timings.get(&slide_index).map(|r| r.duration).unwrap_or(0.0);
            recorded - target
        })
    }

    /// Validate rehearsal timer
    pub fn validate(&self) -> Result<(), String> {
        // Validate ID
        if self.id.trim().is_empty() {
            return Err("Timer ID cannot be empty".to_string());
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

        // Validate elapsed times
        if self.total_elapsed_time < 0.0 {
            return Err("Total elapsed time cannot be negative".to_string());
        }

        if self.current_slide_elapsed_time < 0.0 {
            return Err("Current slide elapsed time cannot be negative".to_string());
        }

        // Validate target duration
        if let Some(duration) = self.target_duration {
            if duration < 0.0 {
                return Err("Target duration cannot be negative".to_string());
            }
            if duration > Self::MAX_DURATION {
                return Err(format!("Target duration exceeds maximum of {} seconds", Self::MAX_DURATION));
            }
        }

        Ok(())
    }

    /// Check if at first slide
    pub fn is_at_first_slide(&self) -> bool {
        self.current_slide_index == 0
    }

    /// Check if at last slide
    pub fn is_at_last_slide(&self) -> bool {
        self.current_slide_index == self.total_slides.saturating_sub(1)
    }

    /// Get progress (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.total_slides == 0 {
            0.0
        } else {
            (self.current_slide_index as f64 / self.total_slides as f64).clamp(0.0, 1.0)
        }
    }
}

impl Default for RehearsalTimer {
    fn default() -> Self {
        Self::new("default".to_string(), 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rehearsal_timer_new() {
        let timer = RehearsalTimer::new("1".to_string(), 10);
        assert_eq!(timer.id, "1");
        assert_eq!(timer.total_slides, 10);
        assert_eq!(timer.state, RehearsalState::NotStarted);
    }

    #[test]
    fn test_rehearsal_timer_start() {
        let mut timer = RehearsalTimer::new("1".to_string(), 10);
        timer.start().unwrap();
        assert_eq!(timer.state, RehearsalState::Recording);
    }

    #[test]
    fn test_rehearsal_timer_pause() {
        let mut timer = RehearsalTimer::new("1".to_string(), 10);
        timer.start().unwrap();
        timer.pause().unwrap();
        assert_eq!(timer.state, RehearsalState::Paused);
    }

    #[test]
    fn test_rehearsal_timer_resume() {
        let mut timer = RehearsalTimer::new("1".to_string(), 10);
        timer.start().unwrap();
        timer.pause().unwrap();
        timer.resume().unwrap();
        assert_eq!(timer.state, RehearsalState::Recording);
    }

    #[test]
    fn test_rehearsal_timer_stop() {
        let mut timer = RehearsalTimer::new("1".to_string(), 10);
        timer.start().unwrap();
        timer.update_elapsed_time(5.0).unwrap();
        timer.stop().unwrap();
        assert_eq!(timer.state, RehearsalState::Completed);
    }

    #[test]
    fn test_rehearsal_timer_next_slide() {
        let mut timer = RehearsalTimer::new("1".to_string(), 10);
        timer.start().unwrap();
        timer.update_elapsed_time(5.0).unwrap();
        timer.next_slide().unwrap();
        assert_eq!(timer.current_slide_index, 1);
        assert!(timer.slide_timings.contains_key(&0));
    }

    #[test]
    fn test_rehearsal_timer_previous_slide() {
        let mut timer = RehearsalTimer::new("1".to_string(), 10);
        timer.start().unwrap();
        timer.update_elapsed_time(5.0).unwrap();
        timer.next_slide().unwrap();
        timer.update_elapsed_time(10.0).unwrap();
        timer.previous_slide().unwrap();
        assert_eq!(timer.current_slide_index, 0);
    }

    #[test]
    fn test_rehearsal_timer_go_to_slide() {
        let mut timer = RehearsalTimer::new("1".to_string(), 10);
        timer.start().unwrap();
        timer.update_elapsed_time(5.0).unwrap();
        timer.go_to_slide(5).unwrap();
        assert_eq!(timer.current_slide_index, 5);
    }

    #[test]
    fn test_rehearsal_timer_update_elapsed_time() {
        let mut timer = RehearsalTimer::new("1".to_string(), 10);
        timer.start().unwrap();
        timer.update_elapsed_time(10.0).unwrap();
        assert_eq!(timer.total_elapsed_time, 10.0);
    }

    #[test]
    fn test_rehearsal_timer_add_notes() {
        let mut timer = RehearsalTimer::new("1".to_string(), 10);
        timer.add_notes("Test notes".to_string()).unwrap();
        assert!(timer.slide_timings.contains_key(&0));
        assert_eq!(timer.slide_timings[&0].notes, "Test notes");
    }

    #[test]
    fn test_rehearsal_timer_total_recorded_duration() {
        let mut timer = RehearsalTimer::new("1".to_string(), 10);
        timer.start().unwrap();
        timer.update_elapsed_time(5.0).unwrap();
        timer.next_slide().unwrap();
        timer.update_elapsed_time(10.0).unwrap();
        timer.next_slide().unwrap();
        assert_eq!(timer.total_recorded_duration(), 10.0);
    }

    #[test]
    fn test_rehearsal_timer_compare_with_target() {
        let mut timer = RehearsalTimer::new("1".to_string(), 10)
            .with_target_duration(60.0);
        timer.start().unwrap();
        timer.update_elapsed_time(30.0).unwrap();
        timer.next_slide().unwrap();
        timer.update_elapsed_time(40.0).unwrap();
        timer.next_slide().unwrap();
        assert_eq!(timer.compare_with_target(), Some(-20.0));
    }

    #[test]
    fn test_rehearsal_timer_validate_empty_id() {
        let timer = RehearsalTimer::new("".to_string(), 10);
        assert!(timer.validate().is_err());
    }

    #[test]
    fn test_rehearsal_timer_serialization() {
        let timer = RehearsalTimer::new("1".to_string(), 10);
        let json = serde_json::to_string(&timer);
        assert!(json.is_ok());
    }

    #[test]
    fn test_rehearsal_timer_deserialization() {
        let timer = RehearsalTimer::new("1".to_string(), 10);
        let json = serde_json::to_string(&timer).unwrap();
        let deserialized: RehearsalTimer = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, timer.id);
        assert_eq!(deserialized.total_slides, timer.total_slides);
    }
}
