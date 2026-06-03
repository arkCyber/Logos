use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error_handling::CircuitBreaker;
use crate::config_service::ExportConfigService;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropboxConfig {
    pub app_key: String,
    pub app_secret: String,
    pub access_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropboxFile {
    pub id: String,
    pub name: String,
    pub path_lower: String,
    pub size: u64,
    pub client_modified: DateTime<Utc>,
    pub server_modified: DateTime<Utc>,
    pub is_downloadable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct DropboxListResponse {
    entries: Vec<DropboxFileEntry>,
    cursor: String,
    has_more: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct DropboxFileEntry {
    id: String,
    name: String,
    path_lower: String,
    size: Option<u64>,
    client_modified: Option<DateTime<Utc>>,
    server_modified: Option<DateTime<Utc>>,
    #[serde(rename = ".tag")]
    tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DropboxMetadataResponse {
    id: String,
    name: String,
    path_lower: String,
    size: u64,
    client_modified: DateTime<Utc>,
    server_modified: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DropboxSharedLinkResponse {
    url: String,
}

#[allow(dead_code)]
pub struct DropboxClient {
    config: DropboxConfig,
    http_client: Client,
    circuit_breaker: CircuitBreaker,
}

#[allow(dead_code)]
impl DropboxClient {
    pub fn new(config: DropboxConfig, config_service: Arc<ExportConfigService>) -> Self {
        let circuit_breaker = CircuitBreaker::new(config_service);
        Self {
            config,
            http_client: Client::new(),
            circuit_breaker,
        }
    }

    /// Upload a file to Dropbox
    pub async fn upload_file(&self, file_path: String, content: Vec<u8>) -> Result<String, String> {
        let url = "https://content.dropboxapi.com/2/files/upload";

        let mut args: HashMap<&str, serde_json::Value> = HashMap::new();
        args.insert("path", serde_json::json!(file_path));
        args.insert("mode", serde_json::json!("add"));
        args.insert("autorename", serde_json::json!(true));

        // Check circuit breaker
        if !self.circuit_breaker.allow_operation() {
            return Err("Circuit breaker is open, blocking Dropbox API calls".to_string());
        }

        let response = self
            .http_client
            .post(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.access_token),
            )
            .header(
                "Dropbox-API-Arg",
                serde_json::to_string(&args).map_err(|e| e.to_string())?,
            )
            .header("Content-Type", "application/octet-stream")
            .body(content)
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

        let metadata: DropboxMetadataResponse = response
            .json()
            .await
            .map_err(|e| {
                self.circuit_breaker.record_failure();
                format!("Failed to parse response: {}", e)
            })?;

        Ok(metadata.id)
    }

    /// Download a file from Dropbox
    pub async fn download_file(&self, file_path: String) -> Result<Vec<u8>, String> {
        let url = "https://content.dropboxapi.com/2/files/download";

        let mut args: HashMap<&str, serde_json::Value> = HashMap::new();
        args.insert("path", serde_json::json!(file_path));

        let response = self
            .http_client
            .post(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.access_token),
            )
            .header(
                "Dropbox-API-Arg",
                serde_json::to_string(&args).map_err(|e| e.to_string())?,
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

    /// List files in Dropbox
    pub async fn list_files(&self, path: String) -> Result<Vec<DropboxFile>, String> {
        let url = "https://api.dropboxapi.com/2/files/list_folder";

        let mut body: HashMap<&str, serde_json::Value> = HashMap::new();
        body.insert("path", serde_json::json!(path));
        body.insert("recursive", serde_json::json!(false));

        let response = self
            .http_client
            .post(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.access_token),
            )
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("List request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("List failed with status: {}", response.status()));
        }

        let list_response: DropboxListResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let files: Vec<DropboxFile> = list_response
            .entries
            .into_iter()
            .filter_map(|entry| {
                if entry.tag == "file" {
                    Some(DropboxFile {
                        id: entry.id,
                        name: entry.name,
                        path_lower: entry.path_lower,
                        size: entry.size.unwrap_or(0),
                        client_modified: entry.client_modified.unwrap_or_else(Utc::now),
                        server_modified: entry.server_modified.unwrap_or_else(Utc::now),
                        is_downloadable: true,
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(files)
    }

    /// Delete a file from Dropbox
    pub async fn delete_file(&self, file_path: String) -> Result<(), String> {
        let url = "https://api.dropboxapi.com/2/files/delete_v2";

        let mut body: HashMap<&str, serde_json::Value> = HashMap::new();
        body.insert("path", serde_json::json!(file_path));

        let response = self
            .http_client
            .post(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.access_token),
            )
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Delete request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Delete failed with status: {}", response.status()));
        }

        Ok(())
    }

    /// Get file metadata
    pub async fn get_file_metadata(&self, file_path: String) -> Result<DropboxFile, String> {
        let url = "https://api.dropboxapi.com/2/files/get_metadata";

        let mut body: HashMap<&str, serde_json::Value> = HashMap::new();
        body.insert("path", serde_json::json!(file_path));

        let response = self
            .http_client
            .post(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.access_token),
            )
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Metadata request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Metadata request failed with status: {}",
                response.status()
            ));
        }

        let metadata: DropboxMetadataResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(DropboxFile {
            id: metadata.id,
            name: metadata.name,
            path_lower: metadata.path_lower,
            size: metadata.size,
            client_modified: metadata.client_modified,
            server_modified: metadata.server_modified,
            is_downloadable: true,
        })
    }

    /// Create a shared link
    pub async fn create_shared_link(&self, file_path: String) -> Result<String, String> {
        let url = "https://api.dropboxapi.com/2/sharing/create_shared_link_with_settings";

        let mut body: HashMap<&str, serde_json::Value> = HashMap::new();
        body.insert("path", serde_json::json!(file_path));

        let mut settings: HashMap<&str, serde_json::Value> = HashMap::new();
        settings.insert("requested_visibility", serde_json::json!("public"));
        body.insert("settings", serde_json::json!(settings));

        let response = self
            .http_client
            .post(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.access_token),
            )
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Shared link request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Shared link creation failed with status: {}",
                response.status()
            ));
        }

        let link_response: DropboxSharedLinkResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(link_response.url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = DropboxConfig {
            app_key: "test".to_string(),
            app_secret: "test".to_string(),
            access_token: "test".to_string(),
        };
        let client = DropboxClient::new(config);
        assert_eq!(client.config.app_key, "test");
    }

    #[test]
    fn test_config_creation() {
        let config = DropboxConfig {
            app_key: "app_key_123".to_string(),
            app_secret: "app_secret_456".to_string(),
            access_token: "token_789".to_string(),
        };
        assert_eq!(config.app_key, "app_key_123");
        assert_eq!(config.app_secret, "app_secret_456");
        assert_eq!(config.access_token, "token_789");
    }

    #[test]
    fn test_config_serialization() {
        let config = DropboxConfig {
            app_key: "test".to_string(),
            app_secret: "test".to_string(),
            access_token: "test".to_string(),
        };
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_config_deserialization() {
        let json = r#"{
            "app_key": "test",
            "app_secret": "test",
            "access_token": "test"
        }"#;
        let config: DropboxConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.app_key, "test");
    }

    #[test]
    fn test_file_creation() {
        let file = DropboxFile {
            id: "file_123".to_string(),
            name: "test.txt".to_string(),
            path_lower: "/test.txt".to_string(),
            size: 1024,
            client_modified: Utc::now(),
            server_modified: Utc::now(),
            is_downloadable: true,
        };
        assert_eq!(file.id, "file_123");
        assert_eq!(file.name, "test.txt");
        assert_eq!(file.size, 1024);
        assert!(file.is_downloadable);
    }

    #[test]
    fn test_file_creation_not_downloadable() {
        let file = DropboxFile {
            id: "file_123".to_string(),
            name: "test.txt".to_string(),
            path_lower: "/test.txt".to_string(),
            size: 1024,
            client_modified: Utc::now(),
            server_modified: Utc::now(),
            is_downloadable: false,
        };
        assert!(!file.is_downloadable);
    }

    #[test]
    fn test_file_serialization() {
        let file = DropboxFile {
            id: "file_123".to_string(),
            name: "test.txt".to_string(),
            path_lower: "/test.txt".to_string(),
            size: 1024,
            client_modified: Utc::now(),
            server_modified: Utc::now(),
            is_downloadable: true,
        };
        let json = serde_json::to_string(&file);
        assert!(json.is_ok());
    }

    #[test]
    fn test_file_deserialization() {
        let json = r#"{
            "id": "file_123",
            "name": "test.txt",
            "path_lower": "/test.txt",
            "size": 1024,
            "client_modified": "2024-01-01T00:00:00Z",
            "server_modified": "2024-01-01T00:00:00Z",
            "is_downloadable": true
        }"#;
        let file: DropboxFile = serde_json::from_str(json).unwrap();
        assert_eq!(file.id, "file_123");
        assert_eq!(file.name, "test.txt");
    }

    #[tokio::test]
    async fn test_upload_file() {
        let config = DropboxConfig {
            app_key: "test".to_string(),
            app_secret: "test".to_string(),
            access_token: "test".to_string(),
        };
        let client = DropboxClient::new(config);
        let result = client
            .upload_file("/test.txt".to_string(), vec![1, 2, 3])
            .await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_upload_file_empty_content() {
        let config = DropboxConfig {
            app_key: "test".to_string(),
            app_secret: "test".to_string(),
            access_token: "test".to_string(),
        };
        let client = DropboxClient::new(config);
        let result = client.upload_file("/test.txt".to_string(), vec![]).await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_download_file() {
        let config = DropboxConfig {
            app_key: "test".to_string(),
            app_secret: "test".to_string(),
            access_token: "test".to_string(),
        };
        let client = DropboxClient::new(config);
        let result = client.download_file("/test.txt".to_string()).await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_files() {
        let config = DropboxConfig {
            app_key: "test".to_string(),
            app_secret: "test".to_string(),
            access_token: "test".to_string(),
        };
        let client = DropboxClient::new(config);
        let result = client.list_files("/".to_string()).await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_files_subfolder() {
        let config = DropboxConfig {
            app_key: "test".to_string(),
            app_secret: "test".to_string(),
            access_token: "test".to_string(),
        };
        let client = DropboxClient::new(config);
        let result = client.list_files("/documents".to_string()).await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_file() {
        let config = DropboxConfig {
            app_key: "test".to_string(),
            app_secret: "test".to_string(),
            access_token: "test".to_string(),
        };
        let client = DropboxClient::new(config);
        let result = client.delete_file("/test.txt".to_string()).await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_file_metadata() {
        let config = DropboxConfig {
            app_key: "test".to_string(),
            app_secret: "test".to_string(),
            access_token: "test".to_string(),
        };
        let client = DropboxClient::new(config);
        let result = client.get_file_metadata("/test.txt".to_string()).await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_shared_link() {
        let config = DropboxConfig {
            app_key: "test".to_string(),
            app_secret: "test".to_string(),
            access_token: "test".to_string(),
        };
        let client = DropboxClient::new(config);
        let result = client.create_shared_link("/test.txt".to_string()).await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[test]
    fn test_file_path_variants() {
        let file1 = DropboxFile {
            id: "file_1".to_string(),
            name: "doc.pdf".to_string(),
            path_lower: "/documents/doc.pdf".to_string(),
            size: 2048,
            client_modified: Utc::now(),
            server_modified: Utc::now(),
            is_downloadable: true,
        };

        let file2 = DropboxFile {
            id: "file_2".to_string(),
            name: "image.png".to_string(),
            path_lower: "/images/image.png".to_string(),
            size: 4096,
            client_modified: Utc::now(),
            server_modified: Utc::now(),
            is_downloadable: true,
        };

        assert_eq!(file1.path_lower, "/documents/doc.pdf");
        assert_eq!(file2.path_lower, "/images/image.png");
    }

    #[test]
    fn test_file_size_variants() {
        let file = DropboxFile {
            id: "file_123".to_string(),
            name: "test.txt".to_string(),
            path_lower: "/test.txt".to_string(),
            size: 0,
            client_modified: Utc::now(),
            server_modified: Utc::now(),
            is_downloadable: true,
        };
        assert_eq!(file.size, 0);
    }

    #[test]
    fn test_file_timestamps() {
        let now = Utc::now();
        let file = DropboxFile {
            id: "file_123".to_string(),
            name: "test.txt".to_string(),
            path_lower: "/test.txt".to_string(),
            size: 1024,
            client_modified: now,
            server_modified: now,
            is_downloadable: true,
        };
        assert_eq!(file.client_modified, now);
        assert_eq!(file.server_modified, now);
    }
}
