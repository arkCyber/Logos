use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneDriveConfig {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneDriveFile {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub created_datetime: DateTime<Utc>,
    pub last_modified_datetime: DateTime<Utc>,
    pub web_url: String,
    pub parent_reference: Option<ParentReference>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OneDriveListResponse {
    value: Vec<OneDriveFileEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OneDriveFileEntry {
    id: String,
    name: String,
    size: u64,
    created_datetime: DateTime<Utc>,
    last_modified_datetime: DateTime<Utc>,
    web_url: String,
    parent_reference: Option<ParentReferenceEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ParentReferenceEntry {
    id: String,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OneDriveFileResponse {
    id: String,
    name: String,
    size: u64,
    created_datetime: DateTime<Utc>,
    last_modified_datetime: DateTime<Utc>,
    web_url: String,
    parent_reference: Option<ParentReferenceEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OneDriveShareResponse {
    link: OneDriveLink,
}

#[derive(Debug, Serialize, Deserialize)]
struct OneDriveLink {
    web_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentReference {
    pub id: String,
    pub path: String,
}

#[allow(dead_code)]
pub struct OneDriveClient {
    config: OneDriveConfig,
    http_client: Client,
}

#[allow(dead_code)]
impl OneDriveClient {
    pub fn new(config: OneDriveConfig) -> Self {
        Self {
            config,
            http_client: Client::new(),
        }
    }

    /// Upload a file to OneDrive
    pub async fn upload_file(&self, file_path: String, content: Vec<u8>) -> Result<String, String> {
        let url = format!(
            "https://graph.microsoft.com/v1.0/me/drive/root:{}:/content",
            file_path
        );

        let response = self
            .http_client
            .put(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.access_token),
            )
            .header("Content-Type", "application/octet-stream")
            .body(content)
            .send()
            .await
            .map_err(|e| format!("Upload request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Upload failed with status: {}", response.status()));
        }

        let file_response: OneDriveFileResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(file_response.id)
    }

    /// Download a file from OneDrive
    pub async fn download_file(&self, file_id: String) -> Result<Vec<u8>, String> {
        let url = format!(
            "https://graph.microsoft.com/v1.0/me/drive/items/{}/content",
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

    /// List files in OneDrive
    pub async fn list_files(&self, path: Option<String>) -> Result<Vec<OneDriveFile>, String> {
        let url = if let Some(p) = path {
            format!(
                "https://graph.microsoft.com/v1.0/me/drive/root:{}:/children",
                p
            )
        } else {
            "https://graph.microsoft.com/v1.0/me/drive/root/children".to_string()
        };

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

        let list_response: OneDriveListResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let files: Vec<OneDriveFile> = list_response
            .value
            .into_iter()
            .map(|entry| OneDriveFile {
                id: entry.id,
                name: entry.name,
                size: entry.size,
                created_datetime: entry.created_datetime,
                last_modified_datetime: entry.last_modified_datetime,
                web_url: entry.web_url,
                parent_reference: entry.parent_reference.map(|pr| ParentReference {
                    id: pr.id,
                    path: pr.path,
                }),
            })
            .collect();

        Ok(files)
    }

    /// Delete a file from OneDrive
    pub async fn delete_file(&self, file_id: String) -> Result<(), String> {
        let url = format!(
            "https://graph.microsoft.com/v1.0/me/drive/items/{}",
            file_id
        );

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
    pub async fn get_file_metadata(&self, file_id: String) -> Result<OneDriveFile, String> {
        let url = format!(
            "https://graph.microsoft.com/v1.0/me/drive/items/{}",
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
            .map_err(|e| format!("Metadata request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Metadata request failed with status: {}",
                response.status()
            ));
        }

        let file_response: OneDriveFileResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(OneDriveFile {
            id: file_response.id,
            name: file_response.name,
            size: file_response.size,
            created_datetime: file_response.created_datetime,
            last_modified_datetime: file_response.last_modified_datetime,
            web_url: file_response.web_url,
            parent_reference: file_response.parent_reference.map(|pr| ParentReference {
                id: pr.id,
                path: pr.path,
            }),
        })
    }

    /// Create a sharing link
    pub async fn create_sharing_link(
        &self,
        file_id: String,
        link_type: String,
    ) -> Result<String, String> {
        let url = format!(
            "https://graph.microsoft.com/v1.0/me/drive/items/{}/createLink",
            file_id
        );

        let mut body = HashMap::new();
        body.insert("type", link_type);
        body.insert("scope", "anonymous".to_string());

        let response = self
            .http_client
            .post(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.access_token),
            )
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Share link request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Share link creation failed with status: {}",
                response.status()
            ));
        }

        let share_response: OneDriveShareResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(share_response.link.web_url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = OneDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = OneDriveClient::new(config);
        assert_eq!(client.config.client_id, "test");
    }

    #[test]
    fn test_config_creation() {
        let config = OneDriveConfig {
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
        let config = OneDriveConfig {
            client_id: "client_123".to_string(),
            client_secret: "secret_456".to_string(),
            access_token: "token_789".to_string(),
            refresh_token: None,
        };
        assert!(config.refresh_token.is_none());
    }

    #[test]
    fn test_config_serialization() {
        let config = OneDriveConfig {
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
        let config: OneDriveConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.client_id, "test");
    }

    #[test]
    fn test_parent_reference_creation() {
        let parent = ParentReference {
            id: "parent_123".to_string(),
            path: "/drive/root:/Documents".to_string(),
        };
        assert_eq!(parent.id, "parent_123");
        assert_eq!(parent.path, "/drive/root:/Documents");
    }

    #[test]
    fn test_parent_reference_serialization() {
        let parent = ParentReference {
            id: "parent_123".to_string(),
            path: "/drive/root:/Documents".to_string(),
        };
        let json = serde_json::to_string(&parent);
        assert!(json.is_ok());
    }

    #[test]
    fn test_parent_reference_deserialization() {
        let json = r#"{
            "id": "parent_123",
            "path": "/drive/root:/Documents"
        }"#;
        let parent: ParentReference = serde_json::from_str(json).unwrap();
        assert_eq!(parent.id, "parent_123");
    }

    #[test]
    fn test_file_creation() {
        let file = OneDriveFile {
            id: "file_123".to_string(),
            name: "test.txt".to_string(),
            size: 1024,
            created_datetime: Utc::now(),
            last_modified_datetime: Utc::now(),
            web_url: "https://onedrive.live.com/?id=file_123".to_string(),
            parent_reference: Some(ParentReference {
                id: "parent_123".to_string(),
                path: "/drive/root:/Documents".to_string(),
            }),
        };
        assert_eq!(file.id, "file_123");
        assert_eq!(file.name, "test.txt");
        assert_eq!(file.size, 1024);
        assert!(file.parent_reference.is_some());
    }

    #[test]
    fn test_file_creation_without_parent() {
        let file = OneDriveFile {
            id: "file_123".to_string(),
            name: "test.txt".to_string(),
            size: 1024,
            created_datetime: Utc::now(),
            last_modified_datetime: Utc::now(),
            web_url: "https://onedrive.live.com/?id=file_123".to_string(),
            parent_reference: None,
        };
        assert!(file.parent_reference.is_none());
    }

    #[test]
    fn test_file_serialization() {
        let file = OneDriveFile {
            id: "file_123".to_string(),
            name: "test.txt".to_string(),
            size: 1024,
            created_datetime: Utc::now(),
            last_modified_datetime: Utc::now(),
            web_url: "https://onedrive.live.com/?id=file_123".to_string(),
            parent_reference: None,
        };
        let json = serde_json::to_string(&file);
        assert!(json.is_ok());
    }

    #[test]
    fn test_file_deserialization() {
        let json = r#"{
            "id": "file_123",
            "name": "test.txt",
            "size": 1024,
            "created_datetime": "2024-01-01T00:00:00Z",
            "last_modified_datetime": "2024-01-01T00:00:00Z",
            "web_url": "https://onedrive.live.com/?id=file_123",
            "parent_reference": null
        }"#;
        let file: OneDriveFile = serde_json::from_str(json).unwrap();
        assert_eq!(file.id, "file_123");
        assert_eq!(file.name, "test.txt");
    }

    #[tokio::test]
    async fn test_upload_file() {
        let config = OneDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = OneDriveClient::new(config);
        let result = client
            .upload_file("/test.txt".to_string(), vec![1, 2, 3])
            .await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_upload_file_empty_content() {
        let config = OneDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = OneDriveClient::new(config);
        let result = client.upload_file("/test.txt".to_string(), vec![]).await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_download_file() {
        let config = OneDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = OneDriveClient::new(config);
        let result = client.download_file("file_123".to_string()).await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_files() {
        let config = OneDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = OneDriveClient::new(config);
        let result = client.list_files(None).await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_files_with_path() {
        let config = OneDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = OneDriveClient::new(config);
        let result = client.list_files(Some("/Documents".to_string())).await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_file() {
        let config = OneDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = OneDriveClient::new(config);
        let result = client.delete_file("file_123".to_string()).await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_file_metadata() {
        let config = OneDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = OneDriveClient::new(config);
        let result = client.get_file_metadata("file_123".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_sharing_link() {
        let config = OneDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = OneDriveClient::new(config);
        let result = client
            .create_sharing_link("file_123".to_string(), "view".to_string())
            .await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_sharing_link_edit() {
        let config = OneDriveConfig {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: None,
        };
        let client = OneDriveClient::new(config);
        let result = client
            .create_sharing_link("file_123".to_string(), "edit".to_string())
            .await;
        // Expected to fail with invalid credentials
        assert!(result.is_err());
    }

    #[test]
    fn test_file_size_variants() {
        let file = OneDriveFile {
            id: "file_123".to_string(),
            name: "test.txt".to_string(),
            size: 0,
            created_datetime: Utc::now(),
            last_modified_datetime: Utc::now(),
            web_url: "https://onedrive.live.com/?id=file_123".to_string(),
            parent_reference: None,
        };
        assert_eq!(file.size, 0);
    }

    #[test]
    fn test_file_timestamps() {
        let now = Utc::now();
        let file = OneDriveFile {
            id: "file_123".to_string(),
            name: "test.txt".to_string(),
            size: 1024,
            created_datetime: now,
            last_modified_datetime: now,
            web_url: "https://onedrive.live.com/?id=file_123".to_string(),
            parent_reference: None,
        };
        assert_eq!(file.created_datetime, now);
        assert_eq!(file.last_modified_datetime, now);
    }

    #[test]
    fn test_file_web_url() {
        let file = OneDriveFile {
            id: "file_123".to_string(),
            name: "test.txt".to_string(),
            size: 1024,
            created_datetime: Utc::now(),
            last_modified_datetime: Utc::now(),
            web_url: "https://onedrive.live.com/?id=file_123".to_string(),
            parent_reference: None,
        };
        assert_eq!(file.web_url, "https://onedrive.live.com/?id=file_123");
    }

    #[test]
    fn test_config_with_refresh_token() {
        let config = OneDriveConfig {
            client_id: "client".to_string(),
            client_secret: "secret".to_string(),
            access_token: "token".to_string(),
            refresh_token: Some("refresh".to_string()),
        };
        assert_eq!(config.refresh_token.unwrap(), "refresh");
    }
}
