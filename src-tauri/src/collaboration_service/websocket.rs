use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use crate::config_service::ExportConfigService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceInfo {
    pub user_id: String,
    pub user_name: String,
    pub cursor_position: Option<usize>,
    pub selection: Option<(usize, usize)>,
    pub last_seen: DateTime<Utc>,
    pub is_online: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "message_type")]
#[allow(dead_code)]
pub enum CollaborationMessage {
    Join {
        user_id: String,
        user_name: String,
        document_id: String,
    },
    Leave {
        user_id: String,
        document_id: String,
    },
    Operation {
        user_id: String,
        document_id: String,
        operation: super::crdt::CRDTOperation,
    },
    Presence {
        user_id: String,
        document_id: String,
        presence: PresenceInfo,
    },
    SyncRequest {
        user_id: String,
        document_id: String,
        since_version: u64,
    },
    SyncResponse {
        user_id: String,
        document_id: String,
        operations: Vec<super::crdt::CRDTOperation>,
        current_version: u64,
    },
    Error {
        message: String,
    },
}

#[allow(dead_code)]
pub struct CollaborationServer {
    documents: Arc<Mutex<HashMap<String, super::crdt::CRDTDocument>>>,
    presence: Arc<Mutex<HashMap<String, HashMap<String, PresenceInfo>>>>,
    operation_channel: broadcast::Sender<CollaborationMessage>,
}

#[allow(dead_code)]
impl CollaborationServer {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1000);
        Self {
            documents: Arc::new(Mutex::new(HashMap::new())),
            presence: Arc::new(Mutex::new(HashMap::new())),
            operation_channel: tx,
        }
    }

    pub fn handle_message(
        &self,
        message: CollaborationMessage,
    ) -> Result<CollaborationMessage, String> {
        match message {
            CollaborationMessage::Join {
                user_id,
                user_name,
                document_id,
            } => self.handle_join(user_id, user_name, document_id),
            CollaborationMessage::Leave {
                user_id,
                document_id,
            } => self.handle_leave(user_id, document_id),
            CollaborationMessage::Operation {
                user_id,
                document_id,
                operation,
            } => self.handle_operation(user_id, document_id, operation),
            CollaborationMessage::Presence {
                user_id,
                document_id,
                presence,
            } => self.handle_presence(user_id, document_id, presence),
            CollaborationMessage::SyncRequest {
                user_id,
                document_id,
                since_version,
            } => self.handle_sync_request(user_id, document_id, since_version),
            _ => Err("Unhandled message type".to_string()),
        }
    }

    fn handle_join(
        &self,
        user_id: String,
        user_name: String,
        document_id: String,
    ) -> Result<CollaborationMessage, String> {
        // Create document if it doesn't exist
        let mut documents = self
            .documents
            .lock()
            .map_err(|e| format!("Failed to lock documents: {}", e))?;

        if !documents.contains_key(&document_id) {
            let config_service = Arc::new(ExportConfigService::new());
            documents.insert(
                document_id.clone(),
                super::crdt::CRDTDocument::new(
                    document_id.clone(),
                    super::crdt::CRDTType::RichText,
                    config_service,
                ),
            );
        }

        // Add user to presence
        let mut presence = self
            .presence
            .lock()
            .map_err(|e| format!("Failed to lock presence: {}", e))?;

        let doc_presence = presence
            .entry(document_id.clone())
            .or_insert_with(HashMap::new);
        doc_presence.insert(
            user_id.clone(),
            PresenceInfo {
                user_id: user_id.clone(),
                user_name,
                cursor_position: None,
                selection: None,
                last_seen: Utc::now(),
                is_online: true,
            },
        );

        let user_id_clone = user_id.clone();
        let presence = doc_presence.get(&user_id_clone)
            .cloned()
            .unwrap_or_else(|| PresenceInfo {
                user_id: user_id_clone.clone(),
                user_name: "Unknown".to_string(),
                cursor_position: None,
                selection: None,
                last_seen: Utc::now(),
                is_online: false,
            });
        Ok(CollaborationMessage::Presence {
            user_id,
            document_id,
            presence,
        })
    }

    fn handle_leave(
        &self,
        user_id: String,
        document_id: String,
    ) -> Result<CollaborationMessage, String> {
        let mut presence = self
            .presence
            .lock()
            .map_err(|e| format!("Failed to lock presence: {}", e))?;

        if let Some(doc_presence) = presence.get_mut(&document_id) {
            if let Some(mut user_presence) = doc_presence.remove(&user_id) {
                user_presence.is_online = false;
                user_presence.last_seen = Utc::now();
                doc_presence.insert(user_id.clone(), user_presence);
            }
        }

        Ok(CollaborationMessage::Leave {
            user_id,
            document_id,
        })
    }

    fn handle_operation(
        &self,
        user_id: String,
        document_id: String,
        operation: super::crdt::CRDTOperation,
    ) -> Result<CollaborationMessage, String> {
        let mut documents = self
            .documents
            .lock()
            .map_err(|e| format!("Failed to lock documents: {}", e))?;

        let document = documents
            .get_mut(&document_id)
            .ok_or_else(|| format!("Document {} not found", document_id))?;

        document.apply_operation(operation.clone())?;

        // Broadcast the operation to all connected clients
        let document_id_clone = document_id.clone();
        let _ = self
            .operation_channel
            .send(CollaborationMessage::Operation {
                user_id,
                document_id: document_id_clone,
                operation,
            });

        Ok(CollaborationMessage::SyncResponse {
            user_id: String::new(),
            document_id,
            operations: vec![],
            current_version: document.version,
        })
    }

    fn handle_presence(
        &self,
        user_id: String,
        document_id: String,
        presence: PresenceInfo,
    ) -> Result<CollaborationMessage, String> {
        let mut presence_map = self
            .presence
            .lock()
            .map_err(|e| format!("Failed to lock presence: {}", e))?;

        let doc_presence = presence_map
            .entry(document_id.clone())
            .or_insert_with(HashMap::new);
        doc_presence.insert(user_id.clone(), presence.clone());

        Ok(CollaborationMessage::Presence {
            user_id,
            document_id,
            presence,
        })
    }

    fn handle_sync_request(
        &self,
        user_id: String,
        document_id: String,
        since_version: u64,
    ) -> Result<CollaborationMessage, String> {
        let documents = self
            .documents
            .lock()
            .map_err(|e| format!("Failed to lock documents: {}", e))?;

        let document = documents
            .get(&document_id)
            .ok_or_else(|| format!("Document {} not found", document_id))?;

        let operations = document.get_operations_since(since_version);

        Ok(CollaborationMessage::SyncResponse {
            user_id,
            document_id,
            operations,
            current_version: document.version,
        })
    }

    pub fn subscribe_operations(&self) -> broadcast::Receiver<CollaborationMessage> {
        self.operation_channel.subscribe()
    }

    pub fn get_document_users(&self, document_id: &str) -> Result<Vec<PresenceInfo>, String> {
        let presence = self
            .presence
            .lock()
            .map_err(|e| format!("Failed to lock presence: {}", e))?;

        if let Some(doc_presence) = presence.get(document_id) {
            Ok(doc_presence.values().cloned().collect())
        } else {
            Ok(Vec::new())
        }
    }
}

impl Default for CollaborationServer {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
pub struct CollaborationClient {
    server: Arc<CollaborationServer>,
    user_id: String,
    user_name: String,
    current_document: Option<String>,
    operation_receiver: Option<broadcast::Receiver<CollaborationMessage>>,
}

#[allow(dead_code)]
impl CollaborationClient {
    pub fn new(server: Arc<CollaborationServer>, user_id: String, user_name: String) -> Self {
        Self {
            server,
            user_id,
            user_name,
            current_document: None,
            operation_receiver: None,
        }
    }

    pub async fn join_document(&mut self, document_id: String) -> Result<(), String> {
        let message = CollaborationMessage::Join {
            user_id: self.user_id.clone(),
            user_name: self.user_name.clone(),
            document_id: document_id.clone(),
        };

        self.server.handle_message(message)?;
        self.current_document = Some(document_id);
        self.operation_receiver = Some(self.server.subscribe_operations());

        Ok(())
    }

    pub async fn leave_document(&mut self) -> Result<(), String> {
        if let Some(document_id) = &self.current_document {
            let message = CollaborationMessage::Leave {
                user_id: self.user_id.clone(),
                document_id: document_id.clone(),
            };
            self.server.handle_message(message)?;
            self.current_document = None;
        }
        Ok(())
    }

    pub async fn send_operation(
        &self,
        operation: super::crdt::CRDTOperation,
    ) -> Result<(), String> {
        if let Some(document_id) = &self.current_document {
            let message = CollaborationMessage::Operation {
                user_id: self.user_id.clone(),
                document_id: document_id.clone(),
                operation,
            };
            self.server.handle_message(message)?;
        }
        Ok(())
    }

    pub async fn update_presence(&self, presence: PresenceInfo) -> Result<(), String> {
        if let Some(document_id) = &self.current_document {
            let message = CollaborationMessage::Presence {
                user_id: self.user_id.clone(),
                document_id: document_id.clone(),
                presence,
            };
            self.server.handle_message(message)?;
        }
        Ok(())
    }

    pub async fn request_sync(
        &self,
        since_version: u64,
    ) -> Result<Vec<super::crdt::CRDTOperation>, String> {
        if let Some(document_id) = &self.current_document {
            let message = CollaborationMessage::SyncRequest {
                user_id: self.user_id.clone(),
                document_id: document_id.clone(),
                since_version,
            };

            if let CollaborationMessage::SyncResponse { operations, .. } =
                self.server.handle_message(message)?
            {
                return Ok(operations);
            }
        }
        Ok(Vec::new())
    }

    pub async fn receive_operations(&mut self) -> Option<CollaborationMessage> {
        if let Some(ref mut receiver) = self.operation_receiver {
            receiver.recv().await.ok()
        } else {
            None
        }
    }

    pub fn get_document_users(&self) -> Result<Vec<PresenceInfo>, String> {
        if let Some(document_id) = &self.current_document {
            self.server.get_document_users(document_id)
        } else {
            Ok(Vec::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server = CollaborationServer::new();
        assert_eq!(server.get_document_users("test").unwrap().len(), 0);
    }

    #[test]
    fn test_server_default() {
        let server = CollaborationServer::default();
        assert_eq!(server.get_document_users("test").unwrap().len(), 0);
    }

    #[test]
    fn test_join_document() {
        let server = CollaborationServer::new();
        let message = CollaborationMessage::Join {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            document_id: "doc1".to_string(),
        };

        let result = server.handle_message(message);
        assert!(result.is_ok());

        let users = server.get_document_users("doc1").unwrap();
        assert_eq!(users.len(), 1);
    }

    #[test]
    fn test_presence_info_creation() {
        let presence = PresenceInfo {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            cursor_position: Some(10),
            selection: Some((5, 15)),
            last_seen: Utc::now(),
            is_online: true,
        };
        assert_eq!(presence.user_id, "user1");
        assert!(presence.is_online);
    }

    #[test]
    fn test_presence_info_serialization() {
        let presence = PresenceInfo {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            cursor_position: Some(10),
            selection: Some((5, 15)),
            last_seen: Utc::now(),
            is_online: true,
        };
        let json = serde_json::to_string(&presence);
        assert!(json.is_ok());
    }

    #[test]
    fn test_collaboration_message_join() {
        let message = CollaborationMessage::Join {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            document_id: "doc1".to_string(),
        };
        assert!(matches!(message, CollaborationMessage::Join { .. }));
    }

    #[test]
    fn test_collaboration_message_leave() {
        let message = CollaborationMessage::Leave {
            user_id: "user1".to_string(),
            document_id: "doc1".to_string(),
        };
        assert!(matches!(message, CollaborationMessage::Leave { .. }));
    }

    #[test]
    fn test_collaboration_message_operation() {
        let operation = crate::collaboration_service::crdt::CRDTOperation::Insert {
            id: "1".to_string(),
            position: 0,
            content: "Test".to_string(),
            author: "user1".to_string(),
            timestamp: Utc::now(),
        };
        let message = CollaborationMessage::Operation {
            user_id: "user1".to_string(),
            document_id: "doc1".to_string(),
            operation,
        };
        assert!(matches!(message, CollaborationMessage::Operation { .. }));
    }

    #[test]
    fn test_collaboration_message_presence() {
        let presence = PresenceInfo {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            cursor_position: None,
            selection: None,
            last_seen: Utc::now(),
            is_online: true,
        };
        let message = CollaborationMessage::Presence {
            user_id: "user1".to_string(),
            document_id: "doc1".to_string(),
            presence,
        };
        assert!(matches!(message, CollaborationMessage::Presence { .. }));
    }

    #[test]
    fn test_collaboration_message_sync_request() {
        let message = CollaborationMessage::SyncRequest {
            user_id: "user1".to_string(),
            document_id: "doc1".to_string(),
            since_version: 0,
        };
        assert!(matches!(message, CollaborationMessage::SyncRequest { .. }));
    }

    #[test]
    fn test_collaboration_message_sync_response() {
        let message = CollaborationMessage::SyncResponse {
            user_id: "user1".to_string(),
            document_id: "doc1".to_string(),
            operations: vec![],
            current_version: 0,
        };
        assert!(matches!(message, CollaborationMessage::SyncResponse { .. }));
    }

    #[test]
    fn test_collaboration_message_error() {
        let message = CollaborationMessage::Error {
            message: "Test error".to_string(),
        };
        assert!(matches!(message, CollaborationMessage::Error { .. }));
    }

    #[test]
    fn test_collaboration_message_serialization() {
        let message = CollaborationMessage::Join {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            document_id: "doc1".to_string(),
        };
        let json = serde_json::to_string(&message);
        assert!(json.is_ok());
    }

    #[test]
    fn test_leave_document() {
        let server = CollaborationServer::new();

        // First join
        let join_message = CollaborationMessage::Join {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            document_id: "doc1".to_string(),
        };
        server.handle_message(join_message).unwrap();

        // Then leave
        let leave_message = CollaborationMessage::Leave {
            user_id: "user1".to_string(),
            document_id: "doc1".to_string(),
        };
        let result = server.handle_message(leave_message);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_operation() {
        let server = CollaborationServer::new();

        // First join to create document
        let join_message = CollaborationMessage::Join {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            document_id: "doc1".to_string(),
        };
        server.handle_message(join_message).unwrap();

        // Then send operation
        let operation = crate::collaboration_service::crdt::CRDTOperation::Insert {
            id: "1".to_string(),
            position: 0,
            content: "Test".to_string(),
            author: "user1".to_string(),
            timestamp: Utc::now(),
        };
        let op_message = CollaborationMessage::Operation {
            user_id: "user1".to_string(),
            document_id: "doc1".to_string(),
            operation,
        };
        let result = server.handle_message(op_message);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_operation_nonexistent_document() {
        let server = CollaborationServer::new();

        let operation = crate::collaboration_service::crdt::CRDTOperation::Insert {
            id: "1".to_string(),
            position: 0,
            content: "Test".to_string(),
            author: "user1".to_string(),
            timestamp: Utc::now(),
        };
        let op_message = CollaborationMessage::Operation {
            user_id: "user1".to_string(),
            document_id: "doc1".to_string(),
            operation,
        };
        let result = server.handle_message(op_message);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_presence() {
        let server = CollaborationServer::new();

        let presence = PresenceInfo {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            cursor_position: Some(10),
            selection: None,
            last_seen: Utc::now(),
            is_online: true,
        };
        let presence_message = CollaborationMessage::Presence {
            user_id: "user1".to_string(),
            document_id: "doc1".to_string(),
            presence,
        };
        let result = server.handle_message(presence_message);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_sync_request() {
        let server = CollaborationServer::new();

        // First join to create document
        let join_message = CollaborationMessage::Join {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            document_id: "doc1".to_string(),
        };
        server.handle_message(join_message).unwrap();

        // Then request sync
        let sync_message = CollaborationMessage::SyncRequest {
            user_id: "user1".to_string(),
            document_id: "doc1".to_string(),
            since_version: 0,
        };
        let result = server.handle_message(sync_message);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_sync_request_nonexistent_document() {
        let server = CollaborationServer::new();

        let sync_message = CollaborationMessage::SyncRequest {
            user_id: "user1".to_string(),
            document_id: "doc1".to_string(),
            since_version: 0,
        };
        let result = server.handle_message(sync_message);
        assert!(result.is_err());
    }

    #[test]
    fn test_subscribe_operations() {
        let server = CollaborationServer::new();
        let _receiver = server.subscribe_operations();
        // Just test that it doesn't panic
    }

    #[test]
    fn test_get_document_users_empty() {
        let server = CollaborationServer::new();
        let users = server.get_document_users("nonexistent").unwrap();
        assert_eq!(users.len(), 0);
    }

    #[test]
    fn test_get_document_users_multiple() {
        let server = CollaborationServer::new();

        // Join multiple users
        let join1 = CollaborationMessage::Join {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            document_id: "doc1".to_string(),
        };
        server.handle_message(join1).unwrap();

        let join2 = CollaborationMessage::Join {
            user_id: "user2".to_string(),
            user_name: "User 2".to_string(),
            document_id: "doc1".to_string(),
        };
        server.handle_message(join2).unwrap();

        let users = server.get_document_users("doc1").unwrap();
        assert_eq!(users.len(), 2);
    }

    #[test]
    fn test_client_creation() {
        let server = Arc::new(CollaborationServer::new());
        let client =
            CollaborationClient::new(server.clone(), "user1".to_string(), "User 1".to_string());
        assert_eq!(client.user_id, "user1");
    }

    #[tokio::test]
    async fn test_client_join_document() {
        let server = Arc::new(CollaborationServer::new());
        let mut client =
            CollaborationClient::new(server.clone(), "user1".to_string(), "User 1".to_string());

        let result = client.join_document("doc1".to_string()).await;
        assert!(result.is_ok());
        assert!(client.current_document.is_some());
    }

    #[tokio::test]
    async fn test_client_leave_document() {
        let server = Arc::new(CollaborationServer::new());
        let mut client =
            CollaborationClient::new(server.clone(), "user1".to_string(), "User 1".to_string());

        client.join_document("doc1".to_string()).await.unwrap();
        let result = client.leave_document().await;
        assert!(result.is_ok());
        assert!(client.current_document.is_none());
    }

    #[tokio::test]
    async fn test_client_leave_without_join() {
        let server = Arc::new(CollaborationServer::new());
        let mut client =
            CollaborationClient::new(server.clone(), "user1".to_string(), "User 1".to_string());

        let result = client.leave_document().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_send_operation() {
        let server = Arc::new(CollaborationServer::new());
        let client =
            CollaborationClient::new(server.clone(), "user1".to_string(), "User 1".to_string());

        let operation = crate::collaboration_service::crdt::CRDTOperation::Insert {
            id: "1".to_string(),
            position: 0,
            content: "Test".to_string(),
            author: "user1".to_string(),
            timestamp: Utc::now(),
        };

        let result = client.send_operation(operation).await;
        assert!(result.is_ok()); // Returns Ok even without current_document
    }

    #[tokio::test]
    async fn test_client_send_operation_after_join() {
        let server = Arc::new(CollaborationServer::new());
        let mut client =
            CollaborationClient::new(server.clone(), "user1".to_string(), "User 1".to_string());

        client.join_document("doc1".to_string()).await.unwrap();

        let operation = crate::collaboration_service::crdt::CRDTOperation::Insert {
            id: "1".to_string(),
            position: 0,
            content: "Test".to_string(),
            author: "user1".to_string(),
            timestamp: Utc::now(),
        };

        let result = client.send_operation(operation).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_update_presence() {
        let server = Arc::new(CollaborationServer::new());
        let client =
            CollaborationClient::new(server.clone(), "user1".to_string(), "User 1".to_string());

        let presence = PresenceInfo {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            cursor_position: Some(10),
            selection: None,
            last_seen: Utc::now(),
            is_online: true,
        };

        let result = client.update_presence(presence).await;
        assert!(result.is_ok()); // Returns Ok even without current_document
    }

    #[tokio::test]
    async fn test_client_update_presence_after_join() {
        let server = Arc::new(CollaborationServer::new());
        let mut client =
            CollaborationClient::new(server.clone(), "user1".to_string(), "User 1".to_string());

        client.join_document("doc1".to_string()).await.unwrap();

        let presence = PresenceInfo {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            cursor_position: Some(10),
            selection: None,
            last_seen: Utc::now(),
            is_online: true,
        };

        let result = client.update_presence(presence).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_request_sync() {
        let server = Arc::new(CollaborationServer::new());
        let client =
            CollaborationClient::new(server.clone(), "user1".to_string(), "User 1".to_string());

        let result = client.request_sync(0).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_client_request_sync_after_join() {
        let server = Arc::new(CollaborationServer::new());
        let mut client =
            CollaborationClient::new(server.clone(), "user1".to_string(), "User 1".to_string());

        client.join_document("doc1".to_string()).await.unwrap();

        let result = client.request_sync(0).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_receive_operations() {
        let server = Arc::new(CollaborationServer::new());
        let mut client =
            CollaborationClient::new(server.clone(), "user1".to_string(), "User 1".to_string());

        client.join_document("doc1".to_string()).await.unwrap();

        // Use timeout to prevent hanging
        let result = tokio::time::timeout(
            tokio::time::Duration::from_millis(100),
            client.receive_operations(),
        )
        .await;

        // Should timeout since no operations have been broadcast
        assert!(result.is_err()); // Timeout error is expected
    }

    #[tokio::test]
    async fn test_client_receive_operations_without_join() {
        let server = Arc::new(CollaborationServer::new());
        let mut client =
            CollaborationClient::new(server.clone(), "user1".to_string(), "User 1".to_string());

        let result = client.receive_operations().await;
        assert!(result.is_none());
    }

    #[test]
    fn test_client_get_document_users() {
        let server = Arc::new(CollaborationServer::new());
        let client =
            CollaborationClient::new(server.clone(), "user1".to_string(), "User 1".to_string());

        let result = client.get_document_users();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_client_get_document_users_after_join() {
        let server = Arc::new(CollaborationServer::new());
        let mut client =
            CollaborationClient::new(server.clone(), "user1".to_string(), "User 1".to_string());

        // Use blocking version for test
        let join = CollaborationMessage::Join {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            document_id: "doc1".to_string(),
        };
        server.handle_message(join).unwrap();

        client.current_document = Some("doc1".to_string());

        let result = client.get_document_users();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn test_presence_info_with_cursor() {
        let presence = PresenceInfo {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            cursor_position: Some(100),
            selection: None,
            last_seen: Utc::now(),
            is_online: true,
        };
        assert_eq!(presence.cursor_position, Some(100));
    }

    #[test]
    fn test_presence_info_with_selection() {
        let presence = PresenceInfo {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            cursor_position: None,
            selection: Some((10, 20)),
            last_seen: Utc::now(),
            is_online: true,
        };
        assert_eq!(presence.selection, Some((10, 20)));
    }

    #[test]
    fn test_presence_info_offline() {
        let presence = PresenceInfo {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            cursor_position: None,
            selection: None,
            last_seen: Utc::now(),
            is_online: false,
        };
        assert!(!presence.is_online);
    }

    #[test]
    fn test_multiple_joins_same_user() {
        let server = CollaborationServer::new();

        let join1 = CollaborationMessage::Join {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            document_id: "doc1".to_string(),
        };
        server.handle_message(join1).unwrap();

        let join2 = CollaborationMessage::Join {
            user_id: "user1".to_string(),
            user_name: "User 1 Updated".to_string(),
            document_id: "doc1".to_string(),
        };
        let result = server.handle_message(join2);
        assert!(result.is_ok());

        let users = server.get_document_users("doc1").unwrap();
        assert_eq!(users.len(), 1);
    }

    #[test]
    fn test_different_documents() {
        let server = CollaborationServer::new();

        let join1 = CollaborationMessage::Join {
            user_id: "user1".to_string(),
            user_name: "User 1".to_string(),
            document_id: "doc1".to_string(),
        };
        server.handle_message(join1).unwrap();

        let join2 = CollaborationMessage::Join {
            user_id: "user2".to_string(),
            user_name: "User 2".to_string(),
            document_id: "doc2".to_string(),
        };
        server.handle_message(join2).unwrap();

        let users1 = server.get_document_users("doc1").unwrap();
        let users2 = server.get_document_users("doc2").unwrap();

        assert_eq!(users1.len(), 1);
        assert_eq!(users2.len(), 1);
    }
}
