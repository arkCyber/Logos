use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AnnouncementPriority {
    Polite,
    Assertive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenReaderAnnouncement {
    pub message: String,
    pub priority: AnnouncementPriority,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct ScreenReaderAnnouncer {
    announcements: Arc<Mutex<Vec<ScreenReaderAnnouncement>>>,
    max_announcements: usize,
}

impl ScreenReaderAnnouncer {
    pub fn new() -> Self {
        Self {
            announcements: Arc::new(Mutex::new(Vec::new())),
            max_announcements: 100,
        }
    }

    /// Announce a message to screen readers
    pub fn announce(&self, message: String, priority: AnnouncementPriority) {
        let announcement = ScreenReaderAnnouncement {
            message,
            priority,
            timestamp: chrono::Utc::now(),
        };

        let mut announcements = self.announcements.lock().unwrap();
        announcements.push(announcement);

        // Enforce max announcements limit
        if announcements.len() > self.max_announcements {
            announcements.remove(0);
        }
    }

    /// Get all announcements
    pub fn get_announcements(&self) -> Vec<ScreenReaderAnnouncement> {
        let announcements = self.announcements.lock().unwrap();
        announcements.clone()
    }

    /// Clear all announcements
    pub fn clear(&self) {
        let mut announcements = self.announcements.lock().unwrap();
        announcements.clear();
    }

    /// Get announcements by priority
    #[allow(dead_code)]
    pub fn get_by_priority(&self, priority: AnnouncementPriority) -> Vec<ScreenReaderAnnouncement> {
        let announcements = self.announcements.lock().unwrap();
        announcements
            .iter()
            .filter(|a| a.priority == priority)
            .cloned()
            .collect()
    }

    /// Get recent announcements
    #[allow(dead_code)]
    pub fn get_recent(&self, count: usize) -> Vec<ScreenReaderAnnouncement> {
        let announcements = self.announcements.lock().unwrap();
        let len = announcements.len();
        let start = if len > count { len - count } else { 0 };
        announcements[start..].to_vec()
    }
}

impl Default for ScreenReaderAnnouncer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_announcer_creation() {
        let announcer = ScreenReaderAnnouncer::new();
        assert_eq!(announcer.get_announcements().len(), 0);
    }

    #[test]
    fn test_announcer_default() {
        let announcer = ScreenReaderAnnouncer::default();
        assert_eq!(announcer.get_announcements().len(), 0);
    }

    #[test]
    fn test_announce() {
        let announcer = ScreenReaderAnnouncer::new();
        announcer.announce("Test message".to_string(), AnnouncementPriority::Polite);
        assert_eq!(announcer.get_announcements().len(), 1);
    }

    #[test]
    fn test_announce_multiple() {
        let announcer = ScreenReaderAnnouncer::new();
        announcer.announce("Message 1".to_string(), AnnouncementPriority::Polite);
        announcer.announce("Message 2".to_string(), AnnouncementPriority::Assertive);
        announcer.announce("Message 3".to_string(), AnnouncementPriority::Polite);
        assert_eq!(announcer.get_announcements().len(), 3);
    }

    #[test]
    fn test_announce_assertive() {
        let announcer = ScreenReaderAnnouncer::new();
        announcer.announce(
            "Urgent message".to_string(),
            AnnouncementPriority::Assertive,
        );
        let announcements = announcer.get_announcements();
        assert_eq!(announcements.len(), 1);
        assert_eq!(announcements[0].priority, AnnouncementPriority::Assertive);
    }

    #[test]
    fn test_clear() {
        let announcer = ScreenReaderAnnouncer::new();
        announcer.announce("Message 1".to_string(), AnnouncementPriority::Polite);
        announcer.announce("Message 2".to_string(), AnnouncementPriority::Assertive);
        announcer.clear();
        assert_eq!(announcer.get_announcements().len(), 0);
    }

    #[test]
    fn test_clear_empty() {
        let announcer = ScreenReaderAnnouncer::new();
        announcer.clear();
        assert_eq!(announcer.get_announcements().len(), 0);
    }

    #[test]
    fn test_get_by_priority_polite() {
        let announcer = ScreenReaderAnnouncer::new();
        announcer.announce("Polite 1".to_string(), AnnouncementPriority::Polite);
        announcer.announce("Assertive 1".to_string(), AnnouncementPriority::Assertive);
        announcer.announce("Polite 2".to_string(), AnnouncementPriority::Polite);

        let polite = announcer.get_by_priority(AnnouncementPriority::Polite);
        assert_eq!(polite.len(), 2);
    }

    #[test]
    fn test_get_by_priority_assertive() {
        let announcer = ScreenReaderAnnouncer::new();
        announcer.announce("Polite 1".to_string(), AnnouncementPriority::Polite);
        announcer.announce("Assertive 1".to_string(), AnnouncementPriority::Assertive);
        announcer.announce("Polite 2".to_string(), AnnouncementPriority::Polite);

        let assertive = announcer.get_by_priority(AnnouncementPriority::Assertive);
        assert_eq!(assertive.len(), 1);
    }

    #[test]
    fn test_get_by_priority_none() {
        let announcer = ScreenReaderAnnouncer::new();
        announcer.announce("Polite 1".to_string(), AnnouncementPriority::Polite);

        let assertive = announcer.get_by_priority(AnnouncementPriority::Assertive);
        assert_eq!(assertive.len(), 0);
    }

    #[test]
    fn test_get_recent() {
        let announcer = ScreenReaderAnnouncer::new();
        announcer.announce("Message 1".to_string(), AnnouncementPriority::Polite);
        announcer.announce("Message 2".to_string(), AnnouncementPriority::Polite);
        announcer.announce("Message 3".to_string(), AnnouncementPriority::Polite);
        announcer.announce("Message 4".to_string(), AnnouncementPriority::Polite);
        announcer.announce("Message 5".to_string(), AnnouncementPriority::Polite);

        let recent = announcer.get_recent(3);
        assert_eq!(recent.len(), 3);
    }

    #[test]
    fn test_get_recent_more_than_available() {
        let announcer = ScreenReaderAnnouncer::new();
        announcer.announce("Message 1".to_string(), AnnouncementPriority::Polite);
        announcer.announce("Message 2".to_string(), AnnouncementPriority::Polite);

        let recent = announcer.get_recent(10);
        assert_eq!(recent.len(), 2);
    }

    #[test]
    fn test_get_recent_empty() {
        let announcer = ScreenReaderAnnouncer::new();
        let recent = announcer.get_recent(5);
        assert_eq!(recent.len(), 0);
    }

    #[test]
    fn test_announcement_priority_variants() {
        let polite = AnnouncementPriority::Polite;
        let assertive = AnnouncementPriority::Assertive;

        assert_eq!(polite, AnnouncementPriority::Polite);
        assert_eq!(assertive, AnnouncementPriority::Assertive);
        assert_ne!(polite, assertive);
    }

    #[test]
    fn test_announcement_priority_serialization() {
        let priority = AnnouncementPriority::Polite;
        let json = serde_json::to_string(&priority);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"polite\"");
    }

    #[test]
    fn test_announcement_priority_deserialization() {
        let priority: AnnouncementPriority = serde_json::from_str("\"polite\"").unwrap();
        assert_eq!(priority, AnnouncementPriority::Polite);
    }

    #[test]
    fn test_announcement_priority_assertive_serialization() {
        let priority = AnnouncementPriority::Assertive;
        let json = serde_json::to_string(&priority);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"assertive\"");
    }

    #[test]
    fn test_screen_reader_announcement_creation() {
        let announcement = ScreenReaderAnnouncement {
            message: "Test message".to_string(),
            priority: AnnouncementPriority::Polite,
            timestamp: chrono::Utc::now(),
        };
        assert_eq!(announcement.message, "Test message");
        assert_eq!(announcement.priority, AnnouncementPriority::Polite);
    }

    #[test]
    fn test_screen_reader_announcement_serialization() {
        let announcement = ScreenReaderAnnouncement {
            message: "Test message".to_string(),
            priority: AnnouncementPriority::Polite,
            timestamp: chrono::Utc::now(),
        };
        let json = serde_json::to_string(&announcement);
        assert!(json.is_ok());
    }

    #[test]
    fn test_screen_reader_announcement_deserialization() {
        let timestamp = chrono::Utc::now();
        let json = format!(
            r#"{{"message":"Test","priority":"polite","timestamp":"{}"}}"#,
            timestamp.to_rfc3339()
        );
        let announcement: Result<ScreenReaderAnnouncement, _> = serde_json::from_str(&json);
        assert!(announcement.is_ok());
    }

    #[test]
    fn test_max_announcements_limit() {
        let announcer = ScreenReaderAnnouncer::new();
        // Add more than 100 announcements
        for i in 0..105 {
            announcer.announce(format!("Message {}", i), AnnouncementPriority::Polite);
        }
        let announcements = announcer.get_announcements();
        assert_eq!(announcements.len(), 100);
    }

    #[test]
    fn test_announcement_message_content() {
        let announcer = ScreenReaderAnnouncer::new();
        announcer.announce("Hello, world!".to_string(), AnnouncementPriority::Polite);
        let announcements = announcer.get_announcements();
        assert_eq!(announcements[0].message, "Hello, world!");
    }

    #[test]
    fn test_announcement_timestamp() {
        let announcer = ScreenReaderAnnouncer::new();
        let before = chrono::Utc::now();
        announcer.announce("Test".to_string(), AnnouncementPriority::Polite);
        let after = chrono::Utc::now();

        let announcements = announcer.get_announcements();
        assert!(announcements[0].timestamp >= before);
        assert!(announcements[0].timestamp <= after);
    }

    #[test]
    fn test_get_announcements_returns_clone() {
        let announcer = ScreenReaderAnnouncer::new();
        announcer.announce("Message 1".to_string(), AnnouncementPriority::Polite);

        let announcements1 = announcer.get_announcements();
        announcer.announce("Message 2".to_string(), AnnouncementPriority::Polite);

        let announcements2 = announcer.get_announcements();
        assert_eq!(announcements1.len(), 1);
        assert_eq!(announcements2.len(), 2);
    }

    #[test]
    fn test_empty_message() {
        let announcer = ScreenReaderAnnouncer::new();
        announcer.announce("".to_string(), AnnouncementPriority::Polite);
        let announcements = announcer.get_announcements();
        assert_eq!(announcements.len(), 1);
        assert_eq!(announcements[0].message, "");
    }

    #[test]
    fn test_long_message() {
        let announcer = ScreenReaderAnnouncer::new();
        let long_message = "a".repeat(10000);
        announcer.announce(long_message.clone(), AnnouncementPriority::Polite);
        let announcements = announcer.get_announcements();
        assert_eq!(announcements[0].message.len(), 10000);
    }
}
