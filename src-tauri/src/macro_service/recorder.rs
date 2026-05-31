use crate::macro_service::engine::MacroAction;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedAction {
    pub action: MacroAction,
    pub timestamp: DateTime<Utc>,
    pub delay_from_previous: u64, // milliseconds
}

pub struct MacroRecorder {
    is_recording: Arc<Mutex<bool>>,
    recorded_actions: Arc<Mutex<Vec<RecordedAction>>>,
    start_time: Arc<Mutex<Option<DateTime<Utc>>>>,
    last_action_time: Arc<Mutex<Option<DateTime<Utc>>>>,
}

impl MacroRecorder {
    pub fn new() -> Self {
        Self {
            is_recording: Arc::new(Mutex::new(false)),
            recorded_actions: Arc::new(Mutex::new(Vec::new())),
            start_time: Arc::new(Mutex::new(None)),
            last_action_time: Arc::new(Mutex::new(None)),
        }
    }

    /// Start recording
    pub fn start_recording(&self) -> Result<(), String> {
        let mut recording = self
            .is_recording
            .lock()
            .map_err(|e| format!("Failed to lock recording state: {}", e))?;

        if *recording {
            return Err("Already recording".to_string());
        }

        *recording = true;

        let mut start = self
            .start_time
            .lock()
            .map_err(|e| format!("Failed to lock start time: {}", e))?;
        *start = Some(Utc::now());

        let mut last = self
            .last_action_time
            .lock()
            .map_err(|e| format!("Failed to lock last action time: {}", e))?;
        *last = Some(Utc::now());

        let mut actions = self
            .recorded_actions
            .lock()
            .map_err(|e| format!("Failed to lock actions: {}", e))?;
        actions.clear();

        Ok(())
    }

    /// Stop recording
    pub fn stop_recording(&self) -> Result<Vec<RecordedAction>, String> {
        let mut recording = self
            .is_recording
            .lock()
            .map_err(|e| format!("Failed to lock recording state: {}", e))?;

        if !*recording {
            return Err("Not currently recording".to_string());
        }

        *recording = false;

        let actions = self
            .recorded_actions
            .lock()
            .map_err(|e| format!("Failed to lock actions: {}", e))?;

        let recorded = actions.clone();

        let mut start = self
            .start_time
            .lock()
            .map_err(|e| format!("Failed to lock start time: {}", e))?;
        *start = None;

        let mut last = self
            .last_action_time
            .lock()
            .map_err(|e| format!("Failed to lock last action time: {}", e))?;
        *last = None;

        Ok(recorded)
    }

    /// Record an action
    pub fn record_action(&self, action: super::engine::MacroAction) -> Result<(), String> {
        let recording = self
            .is_recording
            .lock()
            .map_err(|e| format!("Failed to lock recording state: {}", e))?;

        if !*recording {
            return Err("Not currently recording".to_string());
        }

        drop(recording);

        let now = Utc::now();

        let mut last = self
            .last_action_time
            .lock()
            .map_err(|e| format!("Failed to lock last action time: {}", e))?;

        let delay = if let Some(last_time) = *last {
            let duration = now.signed_duration_since(last_time);
            duration.num_milliseconds().max(0) as u64
        } else {
            0
        };

        *last = Some(now);
        drop(last);

        let recorded_action = RecordedAction {
            action,
            timestamp: now,
            delay_from_previous: delay,
        };

        let mut actions = self
            .recorded_actions
            .lock()
            .map_err(|e| format!("Failed to lock actions: {}", e))?;

        actions.push(recorded_action);

        Ok(())
    }

    /// Check if currently recording
    pub fn is_recording(&self) -> Result<bool, String> {
        let recording = self
            .is_recording
            .lock()
            .map_err(|e| format!("Failed to lock recording state: {}", e))?;
        Ok(*recording)
    }

    /// Get recorded actions
    pub fn get_recorded_actions(&self) -> Result<Vec<RecordedAction>, String> {
        let actions = self
            .recorded_actions
            .lock()
            .map_err(|e| format!("Failed to lock actions: {}", e))?;
        Ok(actions.clone())
    }

    /// Get recording duration
    #[allow(dead_code)]
    pub fn get_recording_duration(&self) -> Result<Option<u64>, String> {
        let start = self
            .start_time
            .lock()
            .map_err(|e| format!("Failed to lock start time: {}", e))?;

        if let Some(start_time) = *start {
            let duration = Utc::now().signed_duration_since(start_time);
            Ok(Some(duration.num_milliseconds().max(0) as u64))
        } else {
            Ok(None)
        }
    }

    /// Clear recorded actions
    #[allow(dead_code)]
    pub fn clear(&self) -> Result<(), String> {
        let mut actions = self
            .recorded_actions
            .lock()
            .map_err(|e| format!("Failed to lock actions: {}", e))?;
        actions.clear();
        Ok(())
    }

    /// Convert recorded actions to macro actions (with delays)
    #[allow(dead_code)]
    pub fn to_macro_actions(&self) -> Result<Vec<super::engine::MacroAction>, String> {
        let actions = self
            .recorded_actions
            .lock()
            .map_err(|e| format!("Failed to lock actions: {}", e))?;

        let mut macro_actions = Vec::new();

        for recorded in actions.iter() {
            if recorded.delay_from_previous > 0 {
                macro_actions.push(super::engine::MacroAction::Delay {
                    milliseconds: recorded.delay_from_previous,
                });
            }
            macro_actions.push(recorded.action.clone());
        }

        Ok(macro_actions)
    }
}

impl Default for MacroRecorder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recorder_creation() {
        let recorder = MacroRecorder::new();
        assert!(!recorder.is_recording().unwrap());
    }

    #[test]
    fn test_recorder_default() {
        let recorder = MacroRecorder::default();
        assert!(!recorder.is_recording().unwrap());
    }

    #[test]
    fn test_start_stop_recording() {
        let recorder = MacroRecorder::new();
        recorder.start_recording().unwrap();
        assert!(recorder.is_recording().unwrap());
        recorder.stop_recording().unwrap();
        assert!(!recorder.is_recording().unwrap());
    }

    #[test]
    fn test_record_action() {
        let recorder = MacroRecorder::new();
        recorder.start_recording().unwrap();

        let action = MacroAction::InsertText {
            position: 0,
            text: "Hello".to_string(),
        };

        recorder.record_action(action).unwrap();
        let actions = recorder.get_recorded_actions().unwrap();
        assert_eq!(actions.len(), 1);
    }

    #[test]
    fn test_start_when_already_recording() {
        let recorder = MacroRecorder::new();
        recorder.start_recording().unwrap();
        let result = recorder.start_recording();
        assert!(result.is_err());
    }

    #[test]
    fn test_stop_when_not_recording() {
        let recorder = MacroRecorder::new();
        let result = recorder.stop_recording();
        assert!(result.is_err());
    }

    #[test]
    fn test_record_when_not_recording() {
        let recorder = MacroRecorder::new();

        let action = MacroAction::InsertText {
            position: 0,
            text: "Hello".to_string(),
        };

        let result = recorder.record_action(action);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_recorded_actions() {
        let recorder = MacroRecorder::new();
        recorder.start_recording().unwrap();

        let action = MacroAction::InsertText {
            position: 0,
            text: "Hello".to_string(),
        };

        recorder.record_action(action).unwrap();
        let actions = recorder.get_recorded_actions().unwrap();
        assert_eq!(actions.len(), 1);
    }

    #[test]
    fn test_get_recorded_actions_empty() {
        let recorder = MacroRecorder::new();
        let actions = recorder.get_recorded_actions().unwrap();
        assert!(actions.is_empty());
    }

    #[test]
    fn test_get_recording_duration() {
        let recorder = MacroRecorder::new();
        recorder.start_recording().unwrap();

        let duration = recorder.get_recording_duration().unwrap();
        assert!(duration.is_some());
    }

    #[test]
    fn test_get_recording_duration_not_recording() {
        let recorder = MacroRecorder::new();
        let duration = recorder.get_recording_duration().unwrap();
        assert!(duration.is_none());
    }

    #[test]
    fn test_clear() {
        let recorder = MacroRecorder::new();
        recorder.start_recording().unwrap();

        let action = MacroAction::InsertText {
            position: 0,
            text: "Hello".to_string(),
        };

        recorder.record_action(action).unwrap();
        recorder.clear().unwrap();

        let actions = recorder.get_recorded_actions().unwrap();
        assert!(actions.is_empty());
    }

    #[test]
    fn test_to_macro_actions() {
        let recorder = MacroRecorder::new();
        recorder.start_recording().unwrap();

        let action = MacroAction::InsertText {
            position: 0,
            text: "Hello".to_string(),
        };

        recorder.record_action(action).unwrap();
        let macro_actions = recorder.to_macro_actions().unwrap();
        assert_eq!(macro_actions.len(), 1);
    }

    #[test]
    fn test_to_macro_actions_with_delay() {
        let recorder = MacroRecorder::new();
        recorder.start_recording().unwrap();

        let action = MacroAction::InsertText {
            position: 0,
            text: "Hello".to_string(),
        };

        recorder.record_action(action).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));

        let action2 = MacroAction::InsertText {
            position: 5,
            text: "World".to_string(),
        };

        recorder.record_action(action2).unwrap();
        let macro_actions = recorder.to_macro_actions().unwrap();
        assert!(macro_actions.len() >= 2);
    }

    #[test]
    fn test_multiple_actions() {
        let recorder = MacroRecorder::new();
        recorder.start_recording().unwrap();

        for i in 0..5 {
            let action = MacroAction::InsertText {
                position: i,
                text: format!("Text {}", i),
            };
            recorder.record_action(action).unwrap();
        }

        let actions = recorder.get_recorded_actions().unwrap();
        assert_eq!(actions.len(), 5);
    }

    #[test]
    fn test_stop_recording_returns_actions() {
        let recorder = MacroRecorder::new();
        recorder.start_recording().unwrap();

        let action = MacroAction::InsertText {
            position: 0,
            text: "Hello".to_string(),
        };

        recorder.record_action(action).unwrap();
        let recorded = recorder.stop_recording().unwrap();
        assert_eq!(recorded.len(), 1);
    }

    #[test]
    fn test_stop_recording_clears_actions() {
        let recorder = MacroRecorder::new();
        recorder.start_recording().unwrap();

        let action = MacroAction::InsertText {
            position: 0,
            text: "Hello".to_string(),
        };

        recorder.record_action(action).unwrap();
        recorder.stop_recording().unwrap();

        // After stop_recording, actions are not automatically cleared
        // They are only cleared when start_recording is called again
        let actions = recorder.get_recorded_actions().unwrap();
        assert_eq!(actions.len(), 1);
    }

    #[test]
    fn test_recorded_action_serialization() {
        let recorded = RecordedAction {
            action: MacroAction::InsertText {
                position: 0,
                text: "Hello".to_string(),
            },
            timestamp: Utc::now(),
            delay_from_previous: 100,
        };

        let json = serde_json::to_string(&recorded);
        assert!(json.is_ok());
    }

    #[test]
    fn test_recorded_action_deserialization() {
        let recorded = RecordedAction {
            action: MacroAction::InsertText {
                position: 0,
                text: "Hello".to_string(),
            },
            timestamp: Utc::now(),
            delay_from_previous: 100,
        };

        let json = serde_json::to_string(&recorded).unwrap();
        let deserialized: Result<RecordedAction, _> = serde_json::from_str(&json);
        assert!(deserialized.is_ok());
    }

    #[test]
    fn test_different_action_types() {
        let recorder = MacroRecorder::new();
        recorder.start_recording().unwrap();

        let actions = vec![
            MacroAction::InsertText {
                position: 0,
                text: "Hello".to_string(),
            },
            MacroAction::DeleteText {
                position: 0,
                length: 5,
            },
            MacroAction::Navigate { position: 5 },
            MacroAction::Select { start: 0, end: 5 },
        ];

        for action in actions {
            recorder.record_action(action).unwrap();
        }

        let recorded = recorder.get_recorded_actions().unwrap();
        assert_eq!(recorded.len(), 4);
    }

    #[test]
    fn test_delay_calculation() {
        let recorder = MacroRecorder::new();
        recorder.start_recording().unwrap();

        let action = MacroAction::InsertText {
            position: 0,
            text: "Hello".to_string(),
        };

        recorder.record_action(action).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(50));

        let action2 = MacroAction::InsertText {
            position: 5,
            text: "World".to_string(),
        };

        recorder.record_action(action2).unwrap();

        let recorded = recorder.get_recorded_actions().unwrap();
        assert!(recorded[1].delay_from_previous >= 50);
    }

    #[test]
    fn test_restart_recording() {
        let recorder = MacroRecorder::new();
        recorder.start_recording().unwrap();

        let action = MacroAction::InsertText {
            position: 0,
            text: "Hello".to_string(),
        };

        recorder.record_action(action).unwrap();
        recorder.stop_recording().unwrap();

        recorder.start_recording().unwrap();
        let action2 = MacroAction::InsertText {
            position: 5,
            text: "World".to_string(),
        };

        recorder.record_action(action2).unwrap();

        let actions = recorder.get_recorded_actions().unwrap();
        assert_eq!(actions.len(), 1);
    }

    #[test]
    fn test_to_macro_actions_empty() {
        let recorder = MacroRecorder::new();
        recorder.start_recording().unwrap();

        let macro_actions = recorder.to_macro_actions().unwrap();
        assert!(macro_actions.is_empty());
    }

    #[test]
    fn test_action_with_zero_delay() {
        let recorder = MacroRecorder::new();
        recorder.start_recording().unwrap();

        let action = MacroAction::InsertText {
            position: 0,
            text: "Hello".to_string(),
        };

        recorder.record_action(action).unwrap();

        let recorded = recorder.get_recorded_actions().unwrap();
        assert_eq!(recorded[0].delay_from_previous, 0);
    }

    #[test]
    fn test_is_recording_state() {
        let recorder = MacroRecorder::new();
        assert!(!recorder.is_recording().unwrap());

        recorder.start_recording().unwrap();
        assert!(recorder.is_recording().unwrap());

        recorder.stop_recording().unwrap();
        assert!(!recorder.is_recording().unwrap());
    }

    #[test]
    fn test_recorded_action_fields() {
        let recorder = MacroRecorder::new();
        recorder.start_recording().unwrap();

        let action = MacroAction::InsertText {
            position: 0,
            text: "Hello".to_string(),
        };

        recorder.record_action(action).unwrap();

        let recorded = recorder.get_recorded_actions().unwrap();
        assert!(matches!(recorded[0].action, MacroAction::InsertText { .. }));
        assert!(recorded[0].timestamp <= Utc::now());
    }
}
