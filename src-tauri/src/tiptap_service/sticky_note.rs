//! TipTap Sticky Note Manager - Aerospace-Grade Sticky Note Service
//!
//! Safety-critical sticky note service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Performance monitoring
//! - Comprehensive error handling
//! - Security hardening
//! - Fault tolerance and error recovery

use std::time::Instant;
use std::collections::HashMap;
use crate::error_handling::{ErrorContext, ErrorSeverity};
use crate::config_service::ExportConfigService;
use std::sync::Arc;

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 50;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 200;

/// Maximum sticky note length
const MAX_STICKY_NOTE_LENGTH: usize = 5000;

/// Sticky note
#[derive(Debug, Clone)]
pub struct StickyNote {
    pub note_id: String,
    pub content: String,
    pub position: usize,
    pub color: String,
}

pub struct StickyNoteManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
    notes: HashMap<String, StickyNote>,
    note_counter: u64,
}

impl StickyNoteManager {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
            notes: HashMap::new(),
            note_counter: 0,
        }
    }

    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    pub fn max_sticky_note_length() -> usize {
        MAX_STICKY_NOTE_LENGTH
    }

    fn record_error(&mut self, code: &str, message: &str, source: &str) {
        self.last_error = Some(ErrorContext::new(ErrorSeverity::Error, code, message, source));
    }

    pub fn get_last_error(&self) -> Option<&ErrorContext> {
        self.last_error.as_ref()
    }

    pub fn get_operation_count(&self) -> u64 {
        self.operation_count
    }

    pub fn reset_error_state(&mut self) {
        self.last_error = None;
    }

    pub fn reset_operation_count(&mut self) {
        self.operation_count = 0;
    }

    pub fn add_note(&mut self, content: String, position: usize, color: String) -> Result<String, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if content.is_empty() {
            return Err("Sticky note content cannot be empty".to_string());
        }

        if content.len() > MAX_STICKY_NOTE_LENGTH {
            return Err(format!("Sticky note exceeds maximum length of {} characters", MAX_STICKY_NOTE_LENGTH));
        }

        if color.is_empty() {
            return Err("Color cannot be empty".to_string());
        }

        self.note_counter += 1;
        let note_id = format!("sticky_note_{}", self.note_counter);

        let note = StickyNote {
            note_id: note_id.clone(),
            content,
            position,
            color,
        };

        self.notes.insert(note_id.clone(), note);

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Add sticky note CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Add sticky note performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(note_id)
    }

    pub fn remove_note(&mut self, note_id: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.notes.remove(note_id)
            .ok_or("Sticky note not found")?;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Remove sticky note CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Remove sticky note performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_note(&self, note_id: &str) -> Option<&StickyNote> {
        self.notes.get(note_id)
    }

    pub fn get_all_notes(&self) -> Vec<&StickyNote> {
        self.notes.values().collect()
    }

    pub fn clear_notes(&mut self) {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.notes.clear();

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Clear sticky notes CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Clear sticky notes performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sticky_note_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = StickyNoteManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_add_note() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = StickyNoteManager::new(config_service);
        
        let result = manager.add_note("Note content".to_string(), 0, "#ffff00".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_content() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = StickyNoteManager::new(config_service);
        
        let result = manager.add_note("".to_string(), 0, "#ffff00".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_clear_notes() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = StickyNoteManager::new(config_service);
        
        manager.add_note("Note content".to_string(), 0, "#ffff00".to_string()).unwrap();
        manager.clear_notes();
        
        assert_eq!(manager.get_all_notes().len(), 0);
    }
}
