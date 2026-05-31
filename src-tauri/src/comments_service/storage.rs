use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CommentStatus {
    Active,
    Resolved,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub thread_id: String,
    pub author_id: String,
    pub author_name: String,
    pub content: String,
    pub position: Option<CommentPosition>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: CommentStatus,
    pub mentions: Vec<String>,
    pub attachments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentPosition {
    pub start: usize,
    pub end: usize,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentThread {
    pub id: String,
    pub document_id: String,
    pub title: String,
    pub comments: Vec<Comment>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: CommentStatus,
    pub resolved_by: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
}

pub struct CommentStorage {
    threads: HashMap<String, CommentThread>,
}

impl CommentStorage {
    pub fn new() -> Self {
        Self {
            threads: HashMap::new(),
        }
    }

    /// Create a new comment thread
    pub fn create_thread(
        &mut self,
        document_id: String,
        title: String,
        first_comment: Comment,
    ) -> Result<String, String> {
        let thread_id = self.generate_id();
        let now = Utc::now();

        let thread = CommentThread {
            id: thread_id.clone(),
            document_id,
            title,
            comments: vec![first_comment],
            created_at: now,
            updated_at: now,
            status: CommentStatus::Active,
            resolved_by: None,
            resolved_at: None,
        };

        self.threads.insert(thread_id.clone(), thread);
        Ok(thread_id)
    }

    /// Get a comment thread
    pub fn get_thread(&self, thread_id: &str) -> Option<&CommentThread> {
        self.threads.get(thread_id)
    }

    /// Get all threads for a document
    pub fn get_document_threads(&self, document_id: &str) -> Vec<&CommentThread> {
        self.threads
            .values()
            .filter(|t| t.document_id == document_id)
            .collect()
    }

    /// Add a comment to a thread
    pub fn add_comment(&mut self, thread_id: String, comment: Comment) -> Result<(), String> {
        let thread = self
            .threads
            .get_mut(&thread_id)
            .ok_or_else(|| format!("Thread {} not found", thread_id))?;

        thread.comments.push(comment);
        thread.updated_at = Utc::now();
        Ok(())
    }

    /// Update a comment
    pub fn update_comment(&mut self, comment_id: String, content: String) -> Result<(), String> {
        for thread in self.threads.values_mut() {
            if let Some(comment) = thread.comments.iter_mut().find(|c| c.id == comment_id) {
                comment.content = content;
                comment.updated_at = Utc::now();
                thread.updated_at = Utc::now();
                return Ok(());
            }
        }
        Err(format!("Comment {} not found", comment_id))
    }

    /// Delete a comment
    pub fn delete_comment(&mut self, comment_id: String) -> Result<(), String> {
        for thread in self.threads.values_mut() {
            if let Some(pos) = thread.comments.iter().position(|c| c.id == comment_id) {
                thread.comments.remove(pos);
                thread.updated_at = Utc::now();
                return Ok(());
            }
        }
        Err(format!("Comment {} not found", comment_id))
    }

    /// Resolve a thread
    pub fn resolve_thread(&mut self, thread_id: String, resolved_by: String) -> Result<(), String> {
        let thread = self
            .threads
            .get_mut(&thread_id)
            .ok_or_else(|| format!("Thread {} not found", thread_id))?;

        thread.status = CommentStatus::Resolved;
        thread.resolved_by = Some(resolved_by);
        thread.resolved_at = Some(Utc::now());
        thread.updated_at = Utc::now();
        Ok(())
    }

    /// Reopen a thread
    pub fn reopen_thread(&mut self, thread_id: String) -> Result<(), String> {
        let thread = self
            .threads
            .get_mut(&thread_id)
            .ok_or_else(|| format!("Thread {} not found", thread_id))?;

        thread.status = CommentStatus::Active;
        thread.resolved_by = None;
        thread.resolved_at = None;
        thread.updated_at = Utc::now();
        Ok(())
    }

    /// Archive a thread
    pub fn archive_thread(&mut self, thread_id: String) -> Result<(), String> {
        let thread = self
            .threads
            .get_mut(&thread_id)
            .ok_or_else(|| format!("Thread {} not found", thread_id))?;

        thread.status = CommentStatus::Archived;
        thread.updated_at = Utc::now();
        Ok(())
    }

    /// Delete a thread
    pub fn delete_thread(&mut self, thread_id: String) -> Result<(), String> {
        self.threads
            .remove(&thread_id)
            .ok_or_else(|| format!("Thread {} not found", thread_id))?;
        Ok(())
    }

    /// Get all threads
    pub fn get_all_threads(&self) -> Vec<&CommentThread> {
        self.threads.values().collect()
    }

    fn generate_id(&self) -> String {
        format!(
            "comment-{}-{}",
            chrono::Utc::now().timestamp_millis(),
            rand::random::<u32>()
        )
    }
}

impl Default for CommentStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_creation() {
        let storage = CommentStorage::new();
        assert_eq!(storage.get_all_threads().len(), 0);
    }

    #[test]
    fn test_storage_default() {
        let storage = CommentStorage::default();
        assert_eq!(storage.get_all_threads().len(), 0);
    }

    #[test]
    fn test_create_thread() {
        let mut storage = CommentStorage::new();
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "t1".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let result = storage.create_thread("doc1".to_string(), "Test Thread".to_string(), comment);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_thread() {
        let mut storage = CommentStorage::new();
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "t1".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let thread_id = storage
            .create_thread("doc1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();
        let thread = storage.get_thread(&thread_id);
        assert!(thread.is_some());
    }

    #[test]
    fn test_get_thread_not_found() {
        let storage = CommentStorage::new();
        let thread = storage.get_thread("nonexistent");
        assert!(thread.is_none());
    }

    #[test]
    fn test_get_document_threads() {
        let mut storage = CommentStorage::new();
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "t1".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        storage
            .create_thread("doc1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();
        let threads = storage.get_document_threads("doc1");
        assert_eq!(threads.len(), 1);
    }

    #[test]
    fn test_get_document_threads_empty() {
        let storage = CommentStorage::new();
        let threads = storage.get_document_threads("doc1");
        assert!(threads.is_empty());
    }

    #[test]
    fn test_add_comment() {
        let mut storage = CommentStorage::new();
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "t1".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let thread_id = storage
            .create_thread("doc1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let reply = Comment {
            id: "c2".to_string(),
            thread_id: "".to_string(),
            author_id: "u2".to_string(),
            author_name: "User 2".to_string(),
            content: "Reply".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let result = storage.add_comment(thread_id, reply);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_comment_not_found() {
        let mut storage = CommentStorage::new();
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let result = storage.add_comment("nonexistent".to_string(), comment);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_comment() {
        let mut storage = CommentStorage::new();
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "t1".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        storage
            .create_thread("doc1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let result = storage.update_comment("c1".to_string(), "Updated content".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_comment_not_found() {
        let mut storage = CommentStorage::new();
        let result =
            storage.update_comment("nonexistent".to_string(), "Updated content".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_comment() {
        let mut storage = CommentStorage::new();
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "t1".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        storage
            .create_thread("doc1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let result = storage.delete_comment("c1".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_comment_not_found() {
        let mut storage = CommentStorage::new();
        let result = storage.delete_comment("nonexistent".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_thread() {
        let mut storage = CommentStorage::new();
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "t1".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let thread_id = storage
            .create_thread("doc1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let result = storage.resolve_thread(thread_id, "user1".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_resolve_thread_not_found() {
        let mut storage = CommentStorage::new();
        let result = storage.resolve_thread("nonexistent".to_string(), "user1".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_reopen_thread() {
        let mut storage = CommentStorage::new();
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "t1".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let thread_id = storage
            .create_thread("doc1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();
        storage
            .resolve_thread(thread_id.clone(), "user1".to_string())
            .unwrap();

        let result = storage.reopen_thread(thread_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_reopen_thread_not_found() {
        let mut storage = CommentStorage::new();
        let result = storage.reopen_thread("nonexistent".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_archive_thread() {
        let mut storage = CommentStorage::new();
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "t1".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let thread_id = storage
            .create_thread("doc1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let result = storage.archive_thread(thread_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_archive_thread_not_found() {
        let mut storage = CommentStorage::new();
        let result = storage.archive_thread("nonexistent".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_thread() {
        let mut storage = CommentStorage::new();
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "t1".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let thread_id = storage
            .create_thread("doc1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let result = storage.delete_thread(thread_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_thread_not_found() {
        let mut storage = CommentStorage::new();
        let result = storage.delete_thread("nonexistent".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_get_all_threads() {
        let mut storage = CommentStorage::new();
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "t1".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        storage
            .create_thread("doc1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();
        let threads = storage.get_all_threads();
        assert_eq!(threads.len(), 1);
    }

    #[test]
    fn test_comment_status_variants() {
        let active = CommentStatus::Active;
        let resolved = CommentStatus::Resolved;
        let archived = CommentStatus::Archived;

        let _ = (active, resolved, archived);
    }

    #[test]
    fn test_comment_status_serialization() {
        let status = CommentStatus::Active;
        let json = serde_json::to_string(&status);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"active\"");
    }

    #[test]
    fn test_comment_status_deserialization() {
        let status: CommentStatus = serde_json::from_str("\"active\"").unwrap();
        assert!(matches!(status, CommentStatus::Active));
    }

    #[test]
    fn test_comment_creation() {
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "t1".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };
        assert_eq!(comment.id, "c1");
        assert_eq!(comment.author_id, "u1");
    }

    #[test]
    fn test_comment_with_position() {
        let position = CommentPosition {
            start: 0,
            end: 10,
            line: Some(1),
            column: Some(5),
        };
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "t1".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: Some(position),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };
        assert!(comment.position.is_some());
    }

    #[test]
    fn test_comment_with_mentions() {
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "t1".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec!["user2".to_string(), "user3".to_string()],
            attachments: vec![],
        };
        assert_eq!(comment.mentions.len(), 2);
    }

    #[test]
    fn test_comment_with_attachments() {
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "t1".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec!["file1.png".to_string(), "file2.pdf".to_string()],
        };
        assert_eq!(comment.attachments.len(), 2);
    }

    #[test]
    fn test_comment_serialization() {
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "t1".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };
        let json = serde_json::to_string(&comment);
        assert!(json.is_ok());
    }

    #[test]
    fn test_comment_deserialization() {
        let json = r#"{
            "id": "c1",
            "thread_id": "t1",
            "author_id": "u1",
            "author_name": "User",
            "content": "Test",
            "position": null,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z",
            "status": "active",
            "mentions": [],
            "attachments": []
        }"#;
        let comment: Comment = serde_json::from_str(json).unwrap();
        assert_eq!(comment.id, "c1");
    }

    #[test]
    fn test_comment_position_creation() {
        let position = CommentPosition {
            start: 0,
            end: 10,
            line: Some(1),
            column: Some(5),
        };
        assert_eq!(position.start, 0);
        assert_eq!(position.end, 10);
    }

    #[test]
    fn test_comment_position_without_line_column() {
        let position = CommentPosition {
            start: 0,
            end: 10,
            line: None,
            column: None,
        };
        assert!(position.line.is_none());
        assert!(position.column.is_none());
    }

    #[test]
    fn test_comment_position_serialization() {
        let position = CommentPosition {
            start: 0,
            end: 10,
            line: Some(1),
            column: Some(5),
        };
        let json = serde_json::to_string(&position);
        assert!(json.is_ok());
    }

    #[test]
    fn test_comment_position_deserialization() {
        let json = r#"{
            "start": 0,
            "end": 10,
            "line": 1,
            "column": 5
        }"#;
        let position: CommentPosition = serde_json::from_str(json).unwrap();
        assert_eq!(position.start, 0);
    }

    #[test]
    fn test_comment_thread_creation() {
        let thread = CommentThread {
            id: "t1".to_string(),
            document_id: "doc1".to_string(),
            title: "Test Thread".to_string(),
            comments: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            resolved_by: None,
            resolved_at: None,
        };
        assert_eq!(thread.id, "t1");
        assert_eq!(thread.document_id, "doc1");
    }

    #[test]
    fn test_comment_thread_with_comments() {
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "t1".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };
        let thread = CommentThread {
            id: "t1".to_string(),
            document_id: "doc1".to_string(),
            title: "Test Thread".to_string(),
            comments: vec![comment],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            resolved_by: None,
            resolved_at: None,
        };
        assert_eq!(thread.comments.len(), 1);
    }

    #[test]
    fn test_comment_thread_serialization() {
        let thread = CommentThread {
            id: "t1".to_string(),
            document_id: "doc1".to_string(),
            title: "Test Thread".to_string(),
            comments: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            resolved_by: None,
            resolved_at: None,
        };
        let json = serde_json::to_string(&thread);
        assert!(json.is_ok());
    }

    #[test]
    fn test_comment_thread_deserialization() {
        let json = r#"{
            "id": "t1",
            "document_id": "doc1",
            "title": "Test Thread",
            "comments": [],
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z",
            "status": "active",
            "resolved_by": null,
            "resolved_at": null
        }"#;
        let thread: CommentThread = serde_json::from_str(json).unwrap();
        assert_eq!(thread.id, "t1");
    }

    #[test]
    fn test_comment_status_equality() {
        assert_eq!(CommentStatus::Active, CommentStatus::Active);
        assert_ne!(CommentStatus::Active, CommentStatus::Resolved);
    }

    #[test]
    fn test_comment_with_all_fields() {
        let position = CommentPosition {
            start: 0,
            end: 10,
            line: Some(1),
            column: Some(5),
        };
        let comment = Comment {
            id: "c1".to_string(),
            thread_id: "t1".to_string(),
            author_id: "u1".to_string(),
            author_name: "User".to_string(),
            content: "Test".to_string(),
            position: Some(position),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec!["user2".to_string()],
            attachments: vec!["file.png".to_string()],
        };
        assert!(comment.position.is_some());
        assert!(!comment.mentions.is_empty());
        assert!(!comment.attachments.is_empty());
    }

    #[test]
    fn test_comment_thread_resolved() {
        let thread = CommentThread {
            id: "t1".to_string(),
            document_id: "doc1".to_string(),
            title: "Test Thread".to_string(),
            comments: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Resolved,
            resolved_by: Some("user1".to_string()),
            resolved_at: Some(Utc::now()),
        };
        assert_eq!(thread.status, CommentStatus::Resolved);
        assert!(thread.resolved_by.is_some());
        assert!(thread.resolved_at.is_some());
    }
}
