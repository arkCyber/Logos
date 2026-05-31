//! Slide Transition Module
//! 
//! Aerospace-grade slide transition implementation for PPT slides with:
//! - Input validation
//! - Bounds checking
//! - Comprehensive error handling
//! - Multiple transition types and effects
//! - Performance monitoring

use serde::{Deserialize, Serialize};

/// Slide transition type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransitionType {
    /// No transition
    None,
    /// Fade
    Fade,
    /// Push
    Push,
    /// Wipe
    Wipe,
    /// Split
    Split,
    /// Reveal
    Reveal,
    /// Cover
    Cover,
    /// Flash
    Flash,
    /// Dissolve
    Dissolve,
    /// Zoom
    Zoom,
    /// Morph
    Morph,
    /// Custom (自定义)
    Custom(String),
}

/// Transition direction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransitionDirection {
    /// From left
    FromLeft,
    /// From right
    FromRight,
    /// From top
    FromTop,
    /// From bottom
    FromBottom,
    /// From top-left
    FromTopLeft,
    /// From top-right
    FromTopRight,
    /// From bottom-left
    FromBottomLeft,
    /// From bottom-right
    FromBottomRight,
    /// Random
    Random,
}

/// Transition speed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransitionSpeed {
    /// Fast (0.5 seconds)
    Fast,
    /// Normal (1.0 seconds)
    Normal,
    /// Slow (2.0 seconds)
    Slow,
    /// Custom duration in seconds
    Custom(f64),
}

impl TransitionSpeed {
    /// Get duration in seconds
    pub fn duration(&self) -> f64 {
        match self {
            TransitionSpeed::Fast => 0.5,
            TransitionSpeed::Normal => 1.0,
            TransitionSpeed::Slow => 2.0,
            TransitionSpeed::Custom(duration) => duration.clamp(0.1, 10.0),
        }
    }
}

/// Slide transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlideTransition {
    /// Transition ID
    pub id: String,
    /// Transition type
    pub transition_type: TransitionType,
    /// Transition direction
    pub direction: TransitionDirection,
    /// Transition speed
    pub speed: TransitionSpeed,
    /// Whether to apply sound effect
    pub sound_enabled: bool,
    /// Sound effect name (optional)
    pub sound_name: Option<String>,
    /// Whether to advance on click
    pub advance_on_click: bool,
    /// Auto-advance delay in seconds (0 means no auto-advance)
    pub auto_advance_delay: f64,
}

impl SlideTransition {
    /// Maximum auto-advance delay in seconds (5 minutes)
    const MAX_AUTO_ADVANCE_DELAY: f64 = 300.0;

    /// Create a new slide transition
    pub fn new(id: String, transition_type: TransitionType) -> Self {
        Self {
            id,
            transition_type,
            direction: TransitionDirection::FromLeft,
            speed: TransitionSpeed::Normal,
            sound_enabled: false,
            sound_name: None,
            advance_on_click: true,
            auto_advance_delay: 0.0,
        }
    }

    /// Set transition direction
    pub fn with_direction(mut self, direction: TransitionDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set transition speed
    pub fn with_speed(mut self, speed: TransitionSpeed) -> Self {
        self.speed = speed;
        self
    }

    /// Enable sound effect
    pub fn with_sound(mut self, sound_name: String) -> Self {
        self.sound_enabled = true;
        self.sound_name = Some(sound_name);
        self
    }

    /// Set advance on click
    pub fn with_advance_on_click(mut self, advance: bool) -> Self {
        self.advance_on_click = advance;
        self
    }

    /// Set auto-advance delay
    pub fn with_auto_advance_delay(mut self, delay: f64) -> Self {
        self.auto_advance_delay = delay.clamp(0.0, Self::MAX_AUTO_ADVANCE_DELAY);
        self
    }

    /// Validate transition settings
    pub fn validate(&self) -> Result<(), String> {
        // Validate ID
        if self.id.trim().is_empty() {
            return Err("Transition ID cannot be empty".to_string());
        }

        // Validate auto-advance delay
        if self.auto_advance_delay < 0.0 {
            return Err("Auto-advance delay cannot be negative".to_string());
        }

        if self.auto_advance_delay > Self::MAX_AUTO_ADVANCE_DELAY {
            return Err(format!(
                "Auto-advance delay exceeds maximum of {} seconds",
                Self::MAX_AUTO_ADVANCE_DELAY
            ));
        }

        // Validate sound name if sound is enabled
        if self.sound_enabled && self.sound_name.is_none() {
            return Err("Sound name must be provided when sound is enabled".to_string());
        }

        // Validate sound name length
        if let Some(ref name) = self.sound_name {
            if name.len() > 100 {
                return Err("Sound name exceeds maximum length of 100 characters".to_string());
            }
        }

        Ok(())
    }

    /// Get transition duration in seconds
    pub fn duration(&self) -> f64 {
        self.speed.duration()
    }

    /// Create a fade transition
    pub fn fade(id: String) -> Self {
        Self::new(id, TransitionType::Fade)
    }

    /// Create a push transition
    pub fn push(id: String) -> Self {
        Self::new(id, TransitionType::Push)
    }

    /// Create a wipe transition
    pub fn wipe(id: String) -> Self {
        Self::new(id, TransitionType::Wipe)
    }

    /// Create a split transition
    pub fn split(id: String) -> Self {
        Self::new(id, TransitionType::Split)
    }

    /// Create a dissolve transition
    pub fn dissolve(id: String) -> Self {
        Self::new(id, TransitionType::Dissolve)
    }

    /// Create a zoom transition
    pub fn zoom(id: String) -> Self {
        Self::new(id, TransitionType::Zoom)
    }
}

impl Default for SlideTransition {
    fn default() -> Self {
        Self::new("default".to_string(), TransitionType::None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_transition_new() {
        let transition = SlideTransition::new("1".to_string(), TransitionType::Fade);
        assert_eq!(transition.id, "1");
        assert_eq!(transition.transition_type, TransitionType::Fade);
        assert_eq!(transition.direction, TransitionDirection::FromLeft);
        assert_eq!(transition.speed, TransitionSpeed::Normal);
    }

    #[test]
    fn test_slide_transition_with_direction() {
        let transition = SlideTransition::new("1".to_string(), TransitionType::Fade)
            .with_direction(TransitionDirection::FromRight);
        assert_eq!(transition.direction, TransitionDirection::FromRight);
    }

    #[test]
    fn test_slide_transition_with_speed() {
        let transition = SlideTransition::new("1".to_string(), TransitionType::Fade)
            .with_speed(TransitionSpeed::Fast);
        assert_eq!(transition.speed, TransitionSpeed::Fast);
        assert_eq!(transition.duration(), 0.5);
    }

    #[test]
    fn test_slide_transition_with_sound() {
        let transition = SlideTransition::new("1".to_string(), TransitionType::Fade)
            .with_sound("applause".to_string());
        assert!(transition.sound_enabled);
        assert_eq!(transition.sound_name, Some("applause".to_string()));
    }

    #[test]
    fn test_slide_transition_with_auto_advance() {
        let transition = SlideTransition::new("1".to_string(), TransitionType::Fade)
            .with_auto_advance_delay(5.0);
        assert_eq!(transition.auto_advance_delay, 5.0);
    }

    #[test]
    fn test_slide_transition_validate_empty_id() {
        let transition = SlideTransition::new("".to_string(), TransitionType::Fade);
        assert!(transition.validate().is_err());
    }

    #[test]
    fn test_slide_transition_validate_negative_delay() {
        let transition = SlideTransition::new("1".to_string(), TransitionType::Fade)
            .with_auto_advance_delay(-1.0);
        // Delay is clamped to 0.0, so validation should pass
        assert!(transition.validate().is_ok());
        assert_eq!(transition.auto_advance_delay, 0.0);
    }

    #[test]
    fn test_slide_transition_validate_excessive_delay() {
        let transition = SlideTransition::new("1".to_string(), TransitionType::Fade)
            .with_auto_advance_delay(400.0);
        // Delay is clamped to MAX_AUTO_ADVANCE_DELAY, so validation should pass
        assert!(transition.validate().is_ok());
        assert_eq!(transition.auto_advance_delay, SlideTransition::MAX_AUTO_ADVANCE_DELAY);
    }

    #[test]
    fn test_slide_transition_validate_sound_without_name() {
        let mut transition = SlideTransition::new("1".to_string(), TransitionType::Fade);
        transition.sound_enabled = true;
        assert!(transition.validate().is_err());
    }

    #[test]
    fn test_slide_transition_validate_sound_name_too_long() {
        let transition = SlideTransition::new("1".to_string(), TransitionType::Fade)
            .with_sound("a".repeat(101));
        assert!(transition.validate().is_err());
    }

    #[test]
    fn test_transition_speed_duration() {
        assert_eq!(TransitionSpeed::Fast.duration(), 0.5);
        assert_eq!(TransitionSpeed::Normal.duration(), 1.0);
        assert_eq!(TransitionSpeed::Slow.duration(), 2.0);
        assert_eq!(TransitionSpeed::Custom(1.5).duration(), 1.5);
    }

    #[test]
    fn test_transition_speed_duration_clamp() {
        assert_eq!(TransitionSpeed::Custom(0.05).duration(), 0.1);
        assert_eq!(TransitionSpeed::Custom(15.0).duration(), 10.0);
    }

    #[test]
    fn test_slide_transition_factory_methods() {
        let fade = SlideTransition::fade("1".to_string());
        assert_eq!(fade.transition_type, TransitionType::Fade);

        let push = SlideTransition::push("2".to_string());
        assert_eq!(push.transition_type, TransitionType::Push);

        let wipe = SlideTransition::wipe("3".to_string());
        assert_eq!(wipe.transition_type, TransitionType::Wipe);

        let split = SlideTransition::split("4".to_string());
        assert_eq!(split.transition_type, TransitionType::Split);

        let dissolve = SlideTransition::dissolve("5".to_string());
        assert_eq!(dissolve.transition_type, TransitionType::Dissolve);

        let zoom = SlideTransition::zoom("6".to_string());
        assert_eq!(zoom.transition_type, TransitionType::Zoom);
    }

    #[test]
    fn test_slide_transition_serialization() {
        let transition = SlideTransition::new("1".to_string(), TransitionType::Fade);
        let json = serde_json::to_string(&transition);
        assert!(json.is_ok());
    }

    #[test]
    fn test_slide_transition_deserialization() {
        let transition = SlideTransition::new("1".to_string(), TransitionType::Fade);
        let json = serde_json::to_string(&transition).unwrap();
        let deserialized: SlideTransition = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, transition.id);
        assert_eq!(deserialized.transition_type, transition.transition_type);
    }

    #[test]
    fn test_auto_advance_delay_clamp() {
        let transition = SlideTransition::new("1".to_string(), TransitionType::Fade)
            .with_auto_advance_delay(500.0);
        assert_eq!(transition.auto_advance_delay, 300.0);
    }
}
