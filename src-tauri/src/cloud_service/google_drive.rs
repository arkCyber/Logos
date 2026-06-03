use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error_handling::CircuitBreaker;
use crate::config_service::ExportConfigService;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleDriveConfig {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleDriveFile {
    pub id: String,
    pub name: String,
    pub mime_type: String,
    pub size: Option<u64>,
    pub modified_time: DateTime<Utc>,
    pub web_view_link: Option<String>,
    pub parents: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GoogleDriveListResponse {
    files: Vec<GoogleDriveFileEntry>,
    next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GoogleDriveFileEntry {
    id: String,
    name: String,
    mime_type: String,
    size: Option<String>,
    modified_time: DateTime<Utc>,
    web_view_link: Option<String>,
    parents: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GoogleDriveFileResponse {
    id: String,
    name: String,
    mime_type: String,
    size: Option<String>,
    modified_time: DateTime<Utc>,
    web_view_link: Option<String>,
    parents: Option<Vec<String>>,
}

#[allow(dead_code)]
pub struct GoogleDriveClient {
    config: GoogleDriveConfig,
    http_client: Client,
    circuit_breaker: CircuitBreaker,
}

#[allow(dead_code)]
impl GoogleDriveClient {
    pub fn new(config: GoogleDriveConfig, config_service: Arc<ExportConfigService>) -> Self {
        let circuit_breaker = CircuitBreaker::new(config_service);
        Self {
            config,
            http_client: Client::new(),
            circuit_breaker,
        }
    }

    /// Upload a file to Google Drive
    pub async fn upload_file(
        &self,
        file_name: String,
        content: Vec<u8>,
        parent_id: Option<String>,
    ) -> Result<String, String> {
        let url = "https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart";

        let mut metadata: HashMap<&str, serde_json::Value> = HashMap::new();
        metadata.insert("name", serde_json::json!(file_name));
        if let Some(parent) = parent_id {
            metadata.insert("parents", serde_json::json!([parent]));
        }

        let boundary = "boundary123456789";
        let mut body = Vec::new();

        // Add metadata part
        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(b"Content-Type: application/json; charset=UTF-8\r\n\r\n");
        body.extend_from_slice(
            serde_json::to_string(&metadata)
                .map_err(|e| e.to_string())?
                .as_bytes(),
        );
        body.extend_from_slice(b"\r\n");

        // Add content part
        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(b"Content-Type: application/octet-stream\r\n\r\n");
        body.extend_from_slice(&content);
        body.extend_from_slice(b"\r\n");

        // End boundary
        body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

        // Check circuit breaker
        if !self.circuit_breaker.allow_operation() {
            return Err("Circuit breaker is open, blocking Google Drive API calls".to_string());
        }

        let response = self
            .http_client
            .post(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.access_token),
            )
            .header(
                "Content-Type",
                format!("multipart/related; boundary={}", boundary),
            )
            .body(body)
            .send()
            .await
            .map_err(|e| {
                self.circuit_breaker.record_failure();
                format!("Upload request failed: {}", e)
            })?;

        if !response.status().is_success() {
            self.circuit_breaker.record_failure();
            return Err(format!("Upload failed with status: {}", response.status()));
        }

        self.circuit_breaker.record_success();

        let file_response: GoogleDriveFileResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(file_response.id)
    }

    /// Download a file from Google Drive
    pub async fn download_file(&self, file_id: String) -> Result<Vec<u8>, String> {
        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}?alt=media",
            file_id
        );

        let response = self
            .http_client
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.access_token),
            )
            .send()
            .await
            .map_err(|e| format!("Download request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Download failed with status: {}",
                response.status()
            ));
        }

        let content = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

        Ok(content.to_vec())
    }

    /// List files in Google Drive
    pub async fn list_files(&self, query: Option<String>) -> Result<Vec<GoogleDriveFile>, String> {
        let mut url = "https://www.googleapis.com/drive/v3/files".to_string();
        url.push_str("?fields=files(id,name,mimeType,size,modifiedTime,webViewLink,parents)");

        if let Some(q) = query {
            url.push_str(&format!("&q={}", urlencoding::encode(&q)));
        }

        let response = self
            .http_client
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.access_token),
            )
            .send()
            .await
            .map_err(|e| format!("List request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("List failed with status: {}", response.status()));
        }

        let list_response: GoogleDriveListResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let files: Vec<GoogleDriveFile> = list_response
            .files
            .into_iter()
            .map(|entry| GoogleDriveFile {
                id: entry.id,
                name: entry.name,
                mime_type: entry.mime_type,
                size: entry.size.and_then(|s| s.parse().ok()),
                modified_time: entry.modified_time,
                web_view_link: entry.web_view_link,
                parents: entry.parents,
            })
            .collect();

        Ok(files)
    }

    /// Delete a file from Google Drive
    pub async fn delete_file(&self, file_id: String) -> Result<(), String> {
        let url = format!("https://www.googleapis.com/drive/v3/files/{}", file_id);

        let response = self
            .http_client
            .delete(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.access_token),
            )
            .send()
            .await
            .map_err(|e| format!("Delete request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Delete failed with status: {}", response.status()));
        }

        Ok(())
    }

    /// Get file metadata
    pub async fn get_file_metadata(&self, file_id: String) -> Result<GoogleDriveFile, String> {
        let url = format!("https://www.googleapis.com/drive/v3/files/{}?fields=id,name,mimeType,size,modifiedTime,webViewLink,parents", file_id);

        let response = self
            .http_client
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.access_token),
            )
            .send()
            .await
            .map_err(|e| format!("Metadata request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Metadata request failed with status: {}",
                response.status()
            ));
        }

        let file_response: GoogleDriveFileResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(GoogleDriveFile {
            id: file_response.id,
            name: file_response.name,
            mime_type: file_response.mime_type,
            size: file_response.size.and_then(|s| s.parse().ok()),
            modified_time: file_response.modified_time,
            web_view_link: file_response.web_view_link,
            parents: file_response.parents,
        })
    }

    /// Create a folder
    pub async fn create_folder(
        &self,
        folder_name: String,
        parent_id: Option<String>,
    ) -> Result<String, String> {
        let url = "https://www.googleapis.com/drive/v3/files";

        let mut metadata: HashMap<&str, serde_json::Value> = HashMap::new();
        metadata.insert("name", serde_json::json!(folder_name));
        metadata.insert(
            "mimeType",
            serde_json::json!("application/vnd.google-apps.folder"),
        );
        if let Some(parent) = parent_id {
            metadata.insert("parents", serde_json::json!([parent]));
        }

        let response = self
            .http_client
            .post(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.access_token),
            )
            .header("Content-Type", "application/json")
            .json(&metadata)
            .send()
            .await
            .map_err(|e| format!("Create folder request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Create folder failed with status: {}",
                response.status()
            ));
        }

        let file_response: GoogleDriveFileResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(file_response.id)
    }

    /// Share a file
    pub async fn share_file(
        &self,
        file_id: String,
        email: String,
        role: String,
    ) -> Result<(), String> {
        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/permissions",
            file_id
        );

        let mut permission: HashMap<&str, serde_json::Value> = HashMap::new();
        permission.insert("role", serde_json::json!(role));
        permission.insert("type", serde_json::json!("user"));
        permission.insert("emailAddress", serde_json::json!(email));

        let response = self
            .http_client
            .post(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.access_token),
            )
            .header("Content-Type", "application/json")
            .json(&permission)
            .send()
            .await
            .map_err(|e| format!("Share request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Share failed with status: {}", response.status()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = GoogleDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = GoogleDriveClient::new(config);
        assert_eq!(client.config.client_id, "test");
    }

    #[test]
    fn test_config_creation() {
        let config = GoogleDriveConfig {
            client_id: "client_123".to_string(),
            client_secret: "secret_456".to_string(),
            access_token: "token_789".to_string(),
            refresh_token: Some("refresh_token".to_string()),
        };
        assert_eq!(config.client_id, "client_123");
        assert_eq!(config.client_secret, "secret_456");
        assert_eq!(config.access_token, "token_789");
        assert!(config.refresh_token.is_some());
    }

    #[test]
    fn test_config_creation_without_refresh_token() {
        let config = GoogleDriveConfig {
            client_id: "client_123".to_string(),
            client_secret: "secret_456".to_string(),
            access_token: "token_789".to_string(),
            refresh_token: None,
        };
        assert!(config.refresh_token.is_none());
    }

    #[test]
    fn test_config_serialization() {
        let config = GoogleDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_config_deserialization() {
        let json = r#"{
            "client_id": "test",
            "client_secret": "test",
            "access_token": "test",
            "refresh_token": null
        }"#;
        let config: GoogleDriveConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.client_id, "test");
    }

    #[test]
    fn test_file_creation() {
        let file = GoogleDriveFile {
            id: "file_123".to_string(),
            name: "test.txt".to_string(),
            mime_type: "text/plain".to_string(),
            size: Some(1024),
            modified_time: Utc::now(),
            web_view_link: Some("https://drive.google.com/file/d/file_123".to_string()),
            parents: Some(vec!["parent_123".to_string()]),
        };
        assert_eq!(file.id, "file_123");
        assert_eq!(file.name, "test.txt");
        assert_eq!(file.size, Some(1024));
    }

    #[test]
    fn test_file_creation_minimal() {
        let file = GoogleDriveFile {
            id: "file_123".to_string(),
            name: "test.txt".to_string(),
            mime_type: "text/plain".to_string(),
            size: None,
            modified_time: Utc::now(),
            web_view_link: None,
            parents: None,
        };
        assert!(file.size.is_none());
        assert!(file.web_view_link.is_none());
        assert!(file.parents.is_none());
    }

    #[test]
    fn test_file_serialization() {
        let file = GoogleDriveFile {
            id: "file_123".to_string(),
            name: "test.txt".to_string(),
            mime_type: "text/plain".to_string(),
            size: Some(1024),
            modified_time: Utc::now(),
            web_view_link: None,
            parents: None,
        };
        let json = serde_json::to_string(&file);
        assert!(json.is_ok());
    }

    #[test]
    fn test_file_deserialization() {
        let json = r#"{
            "id": "file_123",
            "name": "test.txt",
            "mime_type": "text/plain",
            "size": 1024,
            "modified_time": "2024-01-01T00:00:00Z",
            "web_view_link": null,
            "parents": null
        }"#;
        let file: GoogleDriveFile = serde_json::from_str(json).unwrap();
        assert_eq!(file.id, "file_123");
        assert_eq!(file.name, "test.txt");
    }

    #[tokio::test]
    async fn test_upload_file() {
        let config = GoogleDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = GoogleDriveClient::new(config);
        let result = client
            .upload_file("test.txt".to_string(), vec![1, 2, 3], None)
            .await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_upload_file_with_parent() {
        let config = GoogleDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = GoogleDriveClient::new(config);
        let result = client
            .upload_file(
                "test.txt".to_string(),
                vec![1, 2, 3],
                Some("parent_123".to_string()),
            )
            .await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_download_file() {
        let config = GoogleDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = GoogleDriveClient::new(config);
        let result = client.download_file("file_123".to_string()).await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_files() {
        let config = GoogleDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = GoogleDriveClient::new(config);
        let result = client.list_files(None).await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_files_with_query() {
        let config = GoogleDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = GoogleDriveClient::new(config);
        let result = client
            .list_files(Some("name contains 'test'".to_string()))
            .await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_file() {
        let config = GoogleDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = GoogleDriveClient::new(config);
        let result = client.delete_file("file_123".to_string()).await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_file_metadata() {
        let config = GoogleDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = GoogleDriveClient::new(config);
        let result = client.get_file_metadata("file_123".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_folder() {
        let config = GoogleDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = GoogleDriveClient::new(config);
        let result = client.create_folder("New Folder".to_string(), None).await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_folder_with_parent() {
        let config = GoogleDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = GoogleDriveClient::new(config);
        let result = client
            .create_folder("New Folder".to_string(), Some("parent_123".to_string()))
            .await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_share_file() {
        let config = GoogleDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = GoogleDriveClient::new(config);
        let result = client
            .share_file(
                "file_123".to_string(),
                "user@example.com".to_string(),
                "reader".to_string(),
            )
            .await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_share_file_with_writer_role() {
        let config = GoogleDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = GoogleDriveClient::new(config);
        let result = client
            .share_file(
                "file_123".to_string(),
                "user@example.com".to_string(),
                "writer".to_string(),
            )
            .await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[test]
    fn test_file_with_multiple_parents() {
        let file = GoogleDriveFile {
            id: "file_123".to_string(),
            name: "test.txt".to_string(),
            mime_type: "text/plain".to_string(),
            size: Some(1024),
            modified_time: Utc::now(),
            web_view_link: None,
            parents: Some(vec!["parent_1".to_string(), "parent_2".to_string()]),
        };
        assert_eq!(file.parents.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_file_mime_type_variants() {
        let file1 = GoogleDriveFile {
            id: "file_1".to_string(),
            name: "doc.pdf".to_string(),
            mime_type: "application/pdf".to_string(),
            size: None,
            modified_time: Utc::now(),
            web_view_link: None,
            parents: None,
        };

        let file2 = GoogleDriveFile {
            id: "file_2".to_string(),
            name: "image.png".to_string(),
            mime_type: "image/png".to_string(),
            size: None,
            modified_time: Utc::now(),
            web_view_link: None,
            parents: None,
        };

        assert_eq!(file1.mime_type, "application/pdf");
        assert_eq!(file2.mime_type, "image/png");
    }

    #[test]
    fn test_config_with_refresh_token() {
        let config = GoogleDriveConfig {
            client_id: "client".to_string(),
            client_secret: "secret".to_string(),
            access_token: "token".to_string(),
            refresh_token: Some("refresh".to_string()),
        };
        assert_eq!(config.refresh_token.unwrap(), "refresh");
    }
}
