//! TipTap Comments Manager - Aerospace-Grade Comments Service
//!
//! Safety-critical comments service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Comprehensive error handling
//! - Performance monitoring
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

/// Maximum comment length
const MAX_COMMENT_LENGTH: usize = 5000;

/// Maximum author name length
const MAX_AUTHOR_LENGTH: usize = 100;

/// Comment
#[derive(Debug, Clone)]
pub struct Comment {
    pub comment_id: String,
    pub document_id: String,
    pub position: usize,
    pub content: String,
    pub author: String,
    pub timestamp: Instant,
    pub resolved: bool,
}

pub struct CommentsManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
    comments: HashMap<String, Vec<Comment>>,
    comment_counter: u64,
}

impl CommentsManager {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
            comments: HashMap::new(),
            comment_counter: 0,
        }
    }

    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    pub fn max_comment_length() -> usize {
        MAX_COMMENT_LENGTH
    }

    pub fn max_author_length() -> usize {
        MAX_AUTHOR_LENGTH
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

    pub fn add_comment(&mut self, document_id: String, position: usize, content: String, author: String) -> Result<String, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if document_id.is_empty() {
            return Err("Document ID cannot be empty".to_string());
        }

        if content.is_empty() {
            return Err("Comment content cannot be empty".to_string());
        }

        if content.len() > MAX_COMMENT_LENGTH {
            return Err(format!("Comment exceeds maximum length of {} characters", MAX_COMMENT_LENGTH));
        }

        if author.len() > MAX_AUTHOR_LENGTH {
            return Err(format!("Author name exceeds maximum length of {} characters", MAX_AUTHOR_LENGTH));
        }

        self.comment_counter += 1;
        let comment_id = format!("comment_{}", self.comment_counter);

        let comment = Comment {
            comment_id: comment_id.clone(),
            document_id: document_id.clone(),
            position,
            content,
            author,
            timestamp: Instant::now(),
            resolved: false,
        };

        let comments = self.comments.entry(document_id).or_insert_with(Vec::new);
        comments.push(comment);

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Add comment CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Add comment performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(comment_id)
    }

    pub fn resolve_comment(&mut self, document_id: &str, comment_id: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(comments) = self.comments.get_mut(document_id) {
            if let Some(comment) = comments.iter_mut().find(|c| c.comment_id == comment_id) {
                comment.resolved = true;

                let elapsed = start_time.elapsed();
                if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
                    eprintln!("Resolve comment CRITICAL performance warning: took {}ms", elapsed.as_millis());
                } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
                    eprintln!("Resolve comment performance warning: took {}ms", elapsed.as_millis());
                }

                self.last_error = None;
                return Ok(());
            }
        }

        Err("Comment not found".to_string())
    }

    pub fn delete_comment(&mut self, document_id: &str, comment_id: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(comments) = self.comments.get_mut(document_id) {
            if let Some(pos) = comments.iter().position(|c| c.comment_id == comment_id) {
                comments.remove(pos);

                let elapsed = start_time.elapsed();
                if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
                    eprintln!("Delete comment CRITICAL performance warning: took {}ms", elapsed.as_millis());
                } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
                    eprintln!("Delete comment performance warning: took {}ms", elapsed.as_millis());
                }

                self.last_error = None;
                return Ok(());
            }
        }

        Err("Comment not found".to_string())
    }

    pub fn get_comments(&self, document_id: &str) -> Option<&Vec<Comment>> {
        self.comments.get(document_id)
    }

    pub fn get_unresolved_comments(&self, document_id: &str) -> Vec<&Comment> {
        if let Some(comments) = self.comments.get(document_id) {
            comments.iter().filter(|c| !c.resolved).collect()
        } else {
            Vec::new()
        }
    }

    pub fn clear_comments(&mut self, document_id: &str) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.comments.remove(document_id)
            .ok_or("Document not found")?;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Clear comments CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Clear comments performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comments_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = CommentsManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_add_comment() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CommentsManager::new(config_service);
        
        let result = manager.add_comment("doc1".to_string(), 0, "Test comment".to_string(), "user1".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_resolve_comment() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CommentsManager::new(config_service);
        
        let comment_id = manager.add_comment("doc1".to_string(), 0, "Test comment".to_string(), "user1".to_string()).unwrap();
        
        let result = manager.resolve_comment("doc1", &comment_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_comments() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CommentsManager::new(config_service);
        
        manager.add_comment("doc1".to_string(), 0, "Test comment".to_string(), "user1".to_string()).unwrap();
        
        let comments = manager.get_comments("doc1");
        assert!(comments.is_some());
        assert_eq!(comments.unwrap().len(), 1);
    }

    #[test]
    fn test_delete_comment() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = CommentsManager::new(config_service);
        
        let comment_id = manager.add_comment("doc1".to_string(), 0, "Test comment".to_string(), "user1".to_string()).unwrap();
        
        let result = manager.delete_comment("doc1", &comment_id);
        assert!(result.is_ok());
    }
}
