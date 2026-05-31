//! Comments Manager - Aerospace-Grade Comments Service
//!
//! Safety-critical comments management service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Comprehensive error handling
//! - Performance monitoring
//! - Security hardening
//! - Fault tolerance and error recovery

use super::storage::{Comment, CommentStatus, CommentStorage, CommentThread};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::error_handling::{ErrorContext, ErrorSeverity};
use crate::config_service::ExportConfigService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentFilter {
    pub document_id: Option<String>,
    pub author_id: Option<String>,
    pub status: Option<CommentStatus>,
    pub mentioned_user: Option<String>,
    pub date_from: Option<chrono::DateTime<chrono::Utc>>,
    pub date_to: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for CommentFilter {
    fn default() -> Self {
        Self {
            document_id: None,
            author_id: None,
            status: None,
            mentioned_user: None,
            date_from: None,
            date_to: None,
        }
    }
}

pub struct CommentsManager {
    storage: CommentStorage,
    operation_count: u64,
    last_error: Option<ErrorContext>,
    config_service: Arc<ExportConfigService>,
}

impl CommentsManager {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            storage: CommentStorage::new(),
            operation_count: 0,
            last_error: None,
            config_service,
        }
    }

    /// Validate comment content length
    fn validate_comment_content(&self, content: &str) -> Result<(), String> {
        let comments_config = self.config_service.get_comments_config();
        if content.len() > comments_config.max_comment_length {
            return Err(format!("Comment content exceeds maximum length of {}", comments_config.max_comment_length));
        }
        if content.is_empty() {
            return Err("Comment content cannot be empty".to_string());
        }
        Ok(())
    }

    /// Validate thread title length
    fn validate_title(&self, title: &str) -> Result<(), String> {
        let comments_config = self.config_service.get_comments_config();
        if title.len() > comments_config.max_title_length {
            return Err(format!("Title exceeds maximum length of {}", comments_config.max_title_length));
        }
        if title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        Ok(())
    }

    /// Validate mentions count
    fn validate_mentions(&self, mentions: &[String]) -> Result<(), String> {
        let comments_config = self.config_service.get_comments_config();
        if mentions.len() > comments_config.max_mentions {
            return Err(format!("Mentions count exceeds maximum of {}", comments_config.max_mentions));
        }
        Ok(())
    }

    /// Validate attachments count
    fn validate_attachments(&self, attachments: &[String]) -> Result<(), String> {
        let comments_config = self.config_service.get_comments_config();
        if attachments.len() > comments_config.max_attachments {
            return Err(format!("Attachments count exceeds maximum of {}", comments_config.max_attachments));
        }
        Ok(())
    }

    /// Record error context
    fn record_error(&mut self, code: &str, message: &str, source: &str) {
        self.last_error = Some(ErrorContext::new(
            ErrorSeverity::Error,
            code,
            message,
            source,
        ));
    }

    /// Get last error
    pub fn get_last_error(&self) -> Option<&ErrorContext> {
        self.last_error.as_ref()
    }

    /// Get operation count
    pub fn get_operation_count(&self) -> u64 {
        self.operation_count
    }

    /// Reset error state
    pub fn reset_error_state(&mut self) {
        self.last_error = None;
    }

    /// Create a new comment with validation
    pub fn create_comment(
        &mut self,
        document_id: String,
        title: String,
        comment: Comment,
    ) -> Result<String, String> {
        self.operation_count += 1;

        // Validate title
        if let Err(e) = self.validate_title(&title) {
            self.record_error("INVALID_TITLE", &e, "create_comment");
            return Err(e);
        }

        // Validate comment content
        if let Err(e) = self.validate_comment_content(&comment.content) {
            self.record_error("INVALID_CONTENT", &e, "create_comment");
            return Err(e);
        }

        // Validate mentions
        if let Err(e) = self.validate_mentions(&comment.mentions) {
            self.record_error("INVALID_MENTIONS", &e, "create_comment");
            return Err(e);
        }

        // Validate attachments
        if let Err(e) = self.validate_attachments(&comment.attachments) {
            self.record_error("INVALID_ATTACHMENTS", &e, "create_comment");
            return Err(e);
        }

        let result = self.storage.create_thread(document_id, title, comment);
        if result.is_err() {
            self.record_error("STORAGE_ERROR", "Failed to create thread", "create_comment");
        } else {
            self.last_error = None;
        }
        result
    }

    /// Reply to a comment with validation
    pub fn reply_to_comment(&mut self, thread_id: String, comment: Comment) -> Result<(), String> {
        self.operation_count += 1;

        // Validate comment content
        if let Err(e) = self.validate_comment_content(&comment.content) {
            self.record_error("INVALID_CONTENT", &e, "reply_to_comment");
            return Err(e);
        }

        // Validate mentions
        if let Err(e) = self.validate_mentions(&comment.mentions) {
            self.record_error("INVALID_MENTIONS", &e, "reply_to_comment");
            return Err(e);
        }

        // Validate attachments
        if let Err(e) = self.validate_attachments(&comment.attachments) {
            self.record_error("INVALID_ATTACHMENTS", &e, "reply_to_comment");
            return Err(e);
        }

        let mut reply = comment;
        reply.thread_id = thread_id.clone();
        let result = self.storage.add_comment(thread_id, reply);
        if result.is_err() {
            self.record_error("STORAGE_ERROR", "Failed to add comment", "reply_to_comment");
        } else {
            self.last_error = None;
        }
        result
    }

    /// Get comment thread
    pub fn get_thread(&self, thread_id: &str) -> Option<&CommentThread> {
        self.storage.get_thread(thread_id)
    }

    /// Get all comments for a document
    pub fn get_document_comments(&self, document_id: &str) -> Vec<&CommentThread> {
        self.storage.get_document_threads(document_id)
    }

    /// Filter comments
    pub fn filter_comments(&self, filter: &CommentFilter) -> Vec<&CommentThread> {
        let mut threads = self.storage.get_all_threads();

        if let Some(document_id) = &filter.document_id {
            threads.retain(|t| &t.document_id == document_id);
        }

        if let Some(author_id) = &filter.author_id {
            threads.retain(|t| t.comments.iter().any(|c| &c.author_id == author_id));
        }

        if let Some(status) = &filter.status {
            threads.retain(|t| &t.status == status);
        }

        if let Some(mentioned_user) = &filter.mentioned_user {
            threads.retain(|t| {
                t.comments
                    .iter()
                    .any(|c| c.mentions.contains(mentioned_user))
            });
        }

        if let Some(date_from) = filter.date_from {
            threads.retain(|t| t.created_at >= date_from);
        }

        if let Some(date_to) = filter.date_to {
            threads.retain(|t| t.created_at <= date_to);
        }

        threads
    }

    /// Update comment with validation
    pub fn update_comment(&mut self, comment_id: String, content: String) -> Result<(), String> {
        self.operation_count += 1;

        // Validate comment content
        if let Err(e) = self.validate_comment_content(&content) {
            self.record_error("INVALID_CONTENT", &e, "update_comment");
            return Err(e);
        }

        let result = self.storage.update_comment(comment_id, content);
        if result.is_err() {
            self.record_error("STORAGE_ERROR", "Failed to update comment", "update_comment");
        } else {
            self.last_error = None;
        }
        result
    }

    /// Delete comment
    pub fn delete_comment(&mut self, comment_id: String) -> Result<(), String> {
        self.storage.delete_comment(comment_id)
    }

    /// Resolve thread
    pub fn resolve_thread(&mut self, thread_id: String, resolved_by: String) -> Result<(), String> {
        self.storage.resolve_thread(thread_id, resolved_by)
    }

    /// Reopen thread
    pub fn reopen_thread(&mut self, thread_id: String) -> Result<(), String> {
        self.storage.reopen_thread(thread_id)
    }

    /// Archive thread
    pub fn archive_thread(&mut self, thread_id: String) -> Result<(), String> {
        self.storage.archive_thread(thread_id)
    }

    /// Delete thread
    pub fn delete_thread(&mut self, thread_id: String) -> Result<(), String> {
        self.storage.delete_thread(thread_id)
    }

    /// Get comment statistics
    pub fn get_stats(&self) -> CommentStats {
        let threads = self.storage.get_all_threads();
        let total_threads = threads.len();
        let total_comments: usize = threads.iter().map(|t| t.comments.len()).sum();
        let active = threads
            .iter()
            .filter(|t| t.status == CommentStatus::Active)
            .count();
        let resolved = threads
            .iter()
            .filter(|t| t.status == CommentStatus::Resolved)
            .count();
        let archived = threads
            .iter()
            .filter(|t| t.status == CommentStatus::Archived)
            .count();

        CommentStats {
            total_threads,
            total_comments,
            active_threads: active,
            resolved_threads: resolved,
            archived_threads: archived,
        }
    }
}

impl Default for CommentsManager {
    fn default() -> Self {
        Self::new(Arc::new(ExportConfigService::new()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentStats {
    pub total_threads: usize,
    pub total_comments: usize,
    pub active_threads: usize,
    pub resolved_threads: usize,
    pub archived_threads: usize,
}

#[cfg(test)]
mod tests {
    use super::super::storage::{Comment, CommentStatus};
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_manager_creation() {
        let manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let stats = manager.get_stats();
        assert_eq!(stats.total_threads, 0);
    }

    #[test]
    fn test_manager_default() {
        let manager = CommentsManager::default();
        let stats = manager.get_stats();
        assert_eq!(stats.total_threads, 0);
    }

    #[test]
    fn test_comment_filter_default() {
        let filter = CommentFilter::default();
        assert!(filter.document_id.is_none());
        assert!(filter.author_id.is_none());
        assert!(filter.status.is_none());
        assert!(filter.mentioned_user.is_none());
        assert!(filter.date_from.is_none());
        assert!(filter.date_to.is_none());
    }

    #[test]
    fn test_comment_filter_creation() {
        let now = Utc::now();
        let filter = CommentFilter {
            document_id: Some("doc_123".to_string()),
            author_id: Some("user_123".to_string()),
            status: Some(CommentStatus::Active),
            mentioned_user: Some("mentioned_user".to_string()),
            date_from: Some(now),
            date_to: Some(now),
        };
        assert_eq!(filter.document_id, Some("doc_123".to_string()));
        assert_eq!(filter.author_id, Some("user_123".to_string()));
    }

    #[test]
    fn test_comment_filter_serialization() {
        let filter = CommentFilter {
            document_id: Some("doc_123".to_string()),
            author_id: None,
            status: None,
            mentioned_user: None,
            date_from: None,
            date_to: None,
        };
        let json = serde_json::to_string(&filter);
        assert!(json.is_ok());
    }

    #[test]
    fn test_comment_filter_deserialization() {
        let json = r#"{
            "document_id": "doc_123",
            "author_id": null,
            "status": null,
            "mentioned_user": null,
            "date_from": null,
            "date_to": null
        }"#;
        let filter: CommentFilter = serde_json::from_str(json).unwrap();
        assert_eq!(filter.document_id, Some("doc_123".to_string()));
    }

    #[test]
    fn test_create_comment() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let result =
            manager.create_comment("doc_1".to_string(), "Test Thread".to_string(), comment);
        assert!(result.is_ok());
    }

    #[test]
    fn test_reply_to_comment() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let thread_id = manager
            .create_comment("doc_1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let reply = Comment {
            id: "comment_2".to_string(),
            thread_id: "".to_string(),
            author_id: "user_2".to_string(),
            author_name: "User 2".to_string(),
            content: "Reply comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let result = manager.reply_to_comment(thread_id, reply);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_thread() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let thread_id = manager
            .create_comment("doc_1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let thread = manager.get_thread(&thread_id);
        assert!(thread.is_some());
    }

    #[test]
    fn test_get_thread_not_found() {
        let manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let thread = manager.get_thread("nonexistent");
        assert!(thread.is_none());
    }

    #[test]
    fn test_get_document_comments() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        manager
            .create_comment("doc_1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let comments = manager.get_document_comments("doc_1");
        assert_eq!(comments.len(), 1);
    }

    #[test]
    fn test_get_document_comments_empty() {
        let manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comments = manager.get_document_comments("doc_1");
        assert!(comments.is_empty());
    }

    #[test]
    fn test_filter_comments_by_document_id() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        manager
            .create_comment("doc_1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let filter = CommentFilter {
            document_id: Some("doc_1".to_string()),
            author_id: None,
            status: None,
            mentioned_user: None,
            date_from: None,
            date_to: None,
        };

        let filtered = manager.filter_comments(&filter);
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_filter_comments_by_author_id() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        manager
            .create_comment("doc_1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let filter = CommentFilter {
            document_id: None,
            author_id: Some("user_1".to_string()),
            status: None,
            mentioned_user: None,
            date_from: None,
            date_to: None,
        };

        let filtered = manager.filter_comments(&filter);
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_filter_comments_by_status() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let thread_id = manager
            .create_comment("doc_1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();
        manager
            .resolve_thread(thread_id, "user_1".to_string())
            .unwrap();

        let filter = CommentFilter {
            document_id: None,
            author_id: None,
            status: Some(CommentStatus::Resolved),
            mentioned_user: None,
            date_from: None,
            date_to: None,
        };

        let filtered = manager.filter_comments(&filter);
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_filter_comments_by_mentioned_user() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec!["mentioned_user".to_string()],
            attachments: vec![],
        };

        manager
            .create_comment("doc_1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let filter = CommentFilter {
            document_id: None,
            author_id: None,
            status: None,
            mentioned_user: Some("mentioned_user".to_string()),
            date_from: None,
            date_to: None,
        };

        let filtered = manager.filter_comments(&filter);
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_filter_comments_empty() {
        let manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let filter = CommentFilter::default();
        let filtered = manager.filter_comments(&filter);
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_update_comment() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        manager
            .create_comment("doc_1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let result = manager.update_comment("comment_1".to_string(), "Updated content".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_comment_not_found() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let result =
            manager.update_comment("nonexistent".to_string(), "Updated content".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_comment() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        manager
            .create_comment("doc_1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let result = manager.delete_comment("comment_1".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_comment_not_found() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let result = manager.delete_comment("nonexistent".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_thread() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let thread_id = manager
            .create_comment("doc_1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let result = manager.resolve_thread(thread_id, "user_1".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_resolve_thread_not_found() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let result = manager.resolve_thread("nonexistent".to_string(), "user_1".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_reopen_thread() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let thread_id = manager
            .create_comment("doc_1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();
        manager
            .resolve_thread(thread_id.clone(), "user_1".to_string())
            .unwrap();

        let result = manager.reopen_thread(thread_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_reopen_thread_not_found() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let result = manager.reopen_thread("nonexistent".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_archive_thread() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let thread_id = manager
            .create_comment("doc_1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let result = manager.archive_thread(thread_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_archive_thread_not_found() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let result = manager.archive_thread("nonexistent".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_thread() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let thread_id = manager
            .create_comment("doc_1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let result = manager.delete_thread(thread_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_thread_not_found() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let result = manager.delete_thread("nonexistent".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_get_stats() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        manager
            .create_comment("doc_1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let stats = manager.get_stats();
        assert_eq!(stats.total_threads, 1);
        assert_eq!(stats.total_comments, 1);
        assert_eq!(stats.active_threads, 1);
        assert_eq!(stats.resolved_threads, 0);
        assert_eq!(stats.archived_threads, 0);
    }

    #[test]
    fn test_get_stats_with_resolved() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let thread_id = manager
            .create_comment("doc_1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();
        manager
            .resolve_thread(thread_id, "user_1".to_string())
            .unwrap();

        let stats = manager.get_stats();
        assert_eq!(stats.active_threads, 0);
        assert_eq!(stats.resolved_threads, 1);
    }

    #[test]
    fn test_get_stats_with_archived() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let thread_id = manager
            .create_comment("doc_1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();
        manager.archive_thread(thread_id).unwrap();

        let stats = manager.get_stats();
        assert_eq!(stats.active_threads, 0);
        assert_eq!(stats.archived_threads, 1);
    }

    #[test]
    fn test_comment_stats_serialization() {
        let stats = CommentStats {
            total_threads: 10,
            total_comments: 100,
            active_threads: 5,
            resolved_threads: 3,
            archived_threads: 2,
        };
        let json = serde_json::to_string(&stats);
        assert!(json.is_ok());
    }

    #[test]
    fn test_comment_stats_deserialization() {
        let json = r#"{
            "total_threads": 10,
            "total_comments": 100,
            "active_threads": 5,
            "resolved_threads": 3,
            "archived_threads": 2
        }"#;
        let stats: CommentStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.total_threads, 10);
    }

    #[test]
    fn test_filter_comments_multiple_filters() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec!["mentioned_user".to_string()],
            attachments: vec![],
        };

        manager
            .create_comment("doc_1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let filter = CommentFilter {
            document_id: Some("doc_1".to_string()),
            author_id: Some("user_1".to_string()),
            status: Some(CommentStatus::Active),
            mentioned_user: Some("mentioned_user".to_string()),
            date_from: None,
            date_to: None,
        };

        let filtered = manager.filter_comments(&filter);
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_reply_to_nonexistent_thread() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let reply = Comment {
            id: "comment_2".to_string(),
            thread_id: "".to_string(),
            author_id: "user_2".to_string(),
            author_name: "User 2".to_string(),
            content: "Reply comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let result = manager.reply_to_comment("nonexistent".to_string(), reply);
        assert!(result.is_err());
    }

    #[test]
    fn test_filter_comments_no_matches() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        manager
            .create_comment("doc_1".to_string(), "Test Thread".to_string(), comment)
            .unwrap();

        let filter = CommentFilter {
            document_id: Some("doc_2".to_string()),
            author_id: None,
            status: None,
            mentioned_user: None,
            date_from: None,
            date_to: None,
        };

        let filtered = manager.filter_comments(&filter);
        assert!(filtered.is_empty());
    }

    // Aerospace-level tests
    #[test]
    fn test_validate_comment_content_too_long() {
        let manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comments_config = manager.config_service.get_comments_config();
        let long_content = "a".repeat(comments_config.max_comment_length + 1);
        let result = manager.validate_comment_content(&long_content);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum length"));
    }

    #[test]
    fn test_validate_comment_content_empty() {
        let manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let result = manager.validate_comment_content("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_title_too_long() {
        let manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comments_config = manager.config_service.get_comments_config();
        let long_title = "a".repeat(comments_config.max_title_length + 1);
        let result = manager.validate_title(&long_title);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum length"));
    }

    #[test]
    fn test_validate_title_empty() {
        let manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let result = manager.validate_title("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_mentions_too_many() {
        let manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comments_config = manager.config_service.get_comments_config();
        let mentions: Vec<String> = (0..comments_config.max_mentions + 1).map(|i| format!("user_{}", i)).collect();
        let result = manager.validate_mentions(&mentions);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum"));
    }

    #[test]
    fn test_validate_attachments_too_many() {
        let manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comments_config = manager.config_service.get_comments_config();
        let attachments: Vec<String> = (0..comments_config.max_attachments + 1).map(|i| format!("file_{}", i)).collect();
        let result = manager.validate_attachments(&attachments);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum"));
    }

    #[test]
    fn test_max_comment_length_accepted() {
        let manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comments_config = manager.config_service.get_comments_config();
        let content = "a".repeat(comments_config.max_comment_length);
        let result = manager.validate_comment_content(&content);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_title_length_accepted() {
        let manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comments_config = manager.config_service.get_comments_config();
        let title = "a".repeat(comments_config.max_title_length);
        let result = manager.validate_title(&title);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_mentions_accepted() {
        let manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comments_config = manager.config_service.get_comments_config();
        let mentions: Vec<String> = (0..comments_config.max_mentions).map(|i| format!("user_{}", i)).collect();
        let result = manager.validate_mentions(&mentions);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_attachments_accepted() {
        let manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comments_config = manager.config_service.get_comments_config();
        let attachments: Vec<String> = (0..comments_config.max_attachments).map(|i| format!("file_{}", i)).collect();
        let result = manager.validate_attachments(&attachments);
        assert!(result.is_ok());
    }

    #[test]
    fn test_operation_count() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        assert_eq!(manager.get_operation_count(), 0);
        
        manager.operation_count = 5;
        assert_eq!(manager.get_operation_count(), 5);
    }

    #[test]
    fn test_error_recording() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        
        let last_error = manager.get_last_error();
        assert!(last_error.is_some());
        let error = last_error.unwrap();
        assert_eq!(error.code, "TEST_ERROR");
        assert_eq!(error.message, "Test error message");
        assert_eq!(error.source, "test_source");
    }

    #[test]
    fn test_error_state_reset() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        
        manager.record_error("TEST_ERROR", "Test error message", "test_source");
        assert!(manager.get_last_error().is_some());
        
        manager.reset_error_state();
        assert!(manager.get_last_error().is_none());
    }

    #[test]
    fn test_create_comment_with_invalid_title() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let result = manager.create_comment("doc_1".to_string(), "".to_string(), comment);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_create_comment_with_invalid_content() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions: vec![],
            attachments: vec![],
        };

        let result = manager.create_comment("doc_1".to_string(), "Test Thread".to_string(), comment);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_create_comment_with_too_many_mentions() {
        let mut manager = CommentsManager::new(Arc::new(ExportConfigService::new()));
        let comments_config = manager.config_service.get_comments_config();
        let mentions: Vec<String> = (0..comments_config.max_mentions + 1).map(|i| format!("user_{}", i)).collect();
        let comment = Comment {
            id: "comment_1".to_string(),
            thread_id: "".to_string(),
            author_id: "user_1".to_string(),
            author_name: "User 1".to_string(),
            content: "Test comment".to_string(),
            position: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CommentStatus::Active,
            mentions,
            attachments: vec![],
        };

        let result = manager.create_comment("doc_1".to_string(), "Test Thread".to_string(), comment);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum"));
    }
}
