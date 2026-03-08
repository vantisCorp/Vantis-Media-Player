//! Cloud storage integration service
//! 
//! Provides integration with cloud storage providers (Google Drive, Dropbox)
//! for accessing media files stored in the cloud.

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// ============================================================================
// Google Drive Integration
// ============================================================================

/// Google Drive OAuth configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleDriveOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
}

impl Default for GoogleDriveOAuthConfig {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            client_secret: String::new(),
            redirect_uri: "http://localhost:8080/callback/gdrive".to_string(),
            scopes: vec![
                "https://www.googleapis.com/auth/drive.readonly".to_string(),
            ],
        }
    }
}

/// Google Drive configuration
#[derive(Debug, Clone)]
pub struct GoogleDriveConfig {
    pub oauth: GoogleDriveOAuthConfig,
    pub api_base_url: String,
    pub timeout_seconds: u64,
    pub page_size: i32,
}

impl Default for GoogleDriveConfig {
    fn default() -> Self {
        Self {
            oauth: GoogleDriveOAuthConfig::default(),
            api_base_url: "https://www.googleapis.com/drive/v3".to_string(),
            timeout_seconds: 60,
            page_size: 100,
        }
    }
}

/// Google Drive credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleDriveCredentials {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: DateTime<Utc>,
}

impl GoogleDriveCredentials {
    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now()
    }
}

/// Google Drive file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleDriveFile {
    pub id: String,
    pub name: String,
    pub mime_type: Option<String>,
    pub size: Option<u64>,
    pub created_time: Option<String>,
    pub modified_time: Option<String>,
    pub is_folder: bool,
    pub parent_id: Option<String>,
    pub thumbnail_url: Option<String>,
    pub web_view_link: Option<String>,
    pub download_url: Option<String>,
}

/// Google Drive storage quota
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleDriveQuota {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
}

/// Google Drive client
pub struct GoogleDriveClient {
    config: GoogleDriveConfig,
    client: Client,
    credentials: Option<GoogleDriveCredentials>,
}

impl GoogleDriveClient {
    pub fn new(config: GoogleDriveConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()?;
        
        Ok(Self {
            config,
            client,
            credentials: None,
        })
    }
    
    /// Generate OAuth authorization URL
    pub fn get_auth_url(&self, state: &str) -> String {
        let scopes = self.config.oauth.scopes.join(" ");
        format!(
            "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}&access_type=offline&prompt=consent",
            urlencoding::encode(&self.config.oauth.client_id),
            urlencoding::encode(&self.config.oauth.redirect_uri),
            urlencoding::encode(&scopes),
            urlencoding::encode(state),
        )
    }
    
    /// Exchange authorization code for access token
    pub async fn authenticate(&mut self, code: &str) -> Result<GoogleDriveCredentials> {
        let response = self.client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("client_id", self.config.oauth.client_id.as_str()),
                ("client_secret", self.config.oauth.client_secret.as_str()),
                ("code", code),
                ("redirect_uri", self.config.oauth.redirect_uri.as_str()),
                ("grant_type", "authorization_code"),
            ])
            .send()
            .await?;
        
        let token: serde_json::Value = response.json().await?;
        
        let credentials = GoogleDriveCredentials {
            access_token: token["access_token"].as_str().unwrap_or_default().to_string(),
            refresh_token: token["refresh_token"].as_str().map(|s| s.to_string()),
            expires_at: Utc::now() + chrono::Duration::seconds(
                token["expires_in"].as_i64().unwrap_or(3600)
            ),
        };
        
        self.credentials = Some(credentials.clone());
        Ok(credentials)
    }
    
    /// Refresh access token
    pub async fn refresh_token(&mut self, refresh_token: &str) -> Result<GoogleDriveCredentials> {
        let response = self.client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("client_id", self.config.oauth.client_id.as_str()),
                ("client_secret", self.config.oauth.client_secret.as_str()),
                ("refresh_token", refresh_token),
                ("grant_type", "refresh_token"),
            ])
            .send()
            .await?;
        
        let token: serde_json::Value = response.json().await?;
        
        let credentials = GoogleDriveCredentials {
            access_token: token["access_token"].as_str().unwrap_or_default().to_string(),
            refresh_token: Some(refresh_token.to_string()),
            expires_at: Utc::now() + chrono::Duration::seconds(
                token["expires_in"].as_i64().unwrap_or(3600)
            ),
        };
        
        self.credentials = Some(credentials.clone());
        Ok(credentials)
    }
    
    /// Get storage quota
    pub async fn get_quota(&self) -> Result<GoogleDriveQuota> {
        let token = self.credentials.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        let url = format!("{}/about?fields=storageQuota", self.config.api_base_url);
        
        let response = self.client
            .get(&url)
            .bearer_auth(&token.access_token)
            .send()
            .await?;
        
        let result: serde_json::Value = response.json().await?;
        let quota = &result["storageQuota"];
        
        let total: u64 = quota["limit"].as_str()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let used: u64 = quota["usage"].as_str()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        
        Ok(GoogleDriveQuota {
            total_bytes: total,
            used_bytes: used,
            available_bytes: total.saturating_sub(used),
        })
    }
    
    /// List files in a folder
    pub async fn list_files(&self, folder_id: Option<&str>, page_size: Option<i32>) -> Result<Vec<GoogleDriveFile>> {
        let token = self.credentials.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        let folder = folder_id.unwrap_or("root");
        let page_size = page_size.unwrap_or(self.config.page_size);
        
        let query = format!("'{}' in parents and trashed = false", folder);
        let url = format!(
            "{}/files?pageSize={}&q={}&fields=files(id,name,mimeType,size,createdTime,modifiedTime,parents,thumbnailLink,webViewLink)",
            self.config.api_base_url,
            page_size,
            urlencoding::encode(&query),
        );
        
        let response = self.client
            .get(&url)
            .bearer_auth(&token.access_token)
            .send()
            .await?;
        
        let result: serde_json::Value = response.json().await?;
        let items: Vec<serde_json::Value> = result["files"].as_array()
            .cloned().unwrap_or_default();
        
        let files: Vec<GoogleDriveFile> = items.into_iter()
            .filter_map(|item| {
                let mime_type = item["mimeType"].as_str().unwrap_or_default().to_string();
                Some(GoogleDriveFile {
                    id: item["id"].as_str()?.to_string(),
                    name: item["name"].as_str()?.to_string(),
                    mime_type: Some(mime_type.clone()),
                    size: item["size"].as_str().and_then(|s| s.parse().ok()),
                    created_time: item["createdTime"].as_str().map(|s| s.to_string()),
                    modified_time: item["modifiedTime"].as_str().map(|s| s.to_string()),
                    is_folder: mime_type == "application/vnd.google-apps.folder",
                    parent_id: item["parents"].as_array().and_then(|p| p.first()?.as_str().map(|s| s.to_string())),
                    thumbnail_url: item["thumbnailLink"].as_str().map(|s| s.to_string()),
                    web_view_link: item["webViewLink"].as_str().map(|s| s.to_string()),
                    download_url: None,
                })
            })
            .collect();
        
        Ok(files)
    }
    
    /// Search files
    pub async fn search_files(&self, query: &str, mime_types: Option<&[&str]>, page_size: Option<i32>) -> Result<Vec<GoogleDriveFile>> {
        let token = self.credentials.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        let mut query_parts = vec![format!("name contains '{}'", query), "trashed = false".to_string()];
        
        if let Some(mimes) = mime_types {
            let mime_query: String = mimes.iter()
                .map(|m| format!("mimeType contains '{}'", m))
                .collect::<Vec<_>>()
                .join(" or ");
            query_parts.push(format!("({})", mime_query));
        }
        
        let full_query = query_parts.join(" and ");
        let page_size = page_size.unwrap_or(self.config.page_size);
        
        let url = format!(
            "{}/files?pageSize={}&q={}&fields=files(id,name,mimeType,size,createdTime,modifiedTime,parents,thumbnailLink,webViewLink)",
            self.config.api_base_url,
            page_size,
            urlencoding::encode(&full_query),
        );
        
        let response = self.client
            .get(&url)
            .bearer_auth(&token.access_token)
            .send()
            .await?;
        
        let result: serde_json::Value = response.json().await?;
        let items: Vec<serde_json::Value> = result["files"].as_array()
            .cloned().unwrap_or_default();
        
        let files: Vec<GoogleDriveFile> = items.into_iter()
            .filter_map(|item| {
                let mime_type = item["mimeType"].as_str().unwrap_or_default().to_string();
                Some(GoogleDriveFile {
                    id: item["id"].as_str()?.to_string(),
                    name: item["name"].as_str()?.to_string(),
                    mime_type: Some(mime_type.clone()),
                    size: item["size"].as_str().and_then(|s| s.parse().ok()),
                    created_time: item["createdTime"].as_str().map(|s| s.to_string()),
                    modified_time: item["modifiedTime"].as_str().map(|s| s.to_string()),
                    is_folder: mime_type == "application/vnd.google-apps.folder",
                    parent_id: item["parents"].as_array().and_then(|p| p.first()?.as_str().map(|s| s.to_string())),
                    thumbnail_url: item["thumbnailLink"].as_str().map(|s| s.to_string()),
                    web_view_link: item["webViewLink"].as_str().map(|s| s.to_string()),
                    download_url: None,
                })
            })
            .collect();
        
        Ok(files)
    }
    
    /// Get download URL for a file
    pub fn get_download_url(&self, file_id: &str) -> Option<String> {
        let token = self.credentials.as_ref()?;
        Some(format!(
            "{}/files/{}?alt=media&access_token={}",
            self.config.api_base_url,
            file_id,
            urlencoding::encode(&token.access_token)
        ))
    }
    
    /// Get credentials
    pub fn get_credentials(&self) -> Option<&GoogleDriveCredentials> {
        self.credentials.as_ref()
    }
    
    /// Set credentials
    pub fn set_credentials(&mut self, credentials: GoogleDriveCredentials) {
        self.credentials = Some(credentials);
    }
}

// ============================================================================
// Dropbox Integration
// ============================================================================

/// Dropbox OAuth configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropboxOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl Default for DropboxOAuthConfig {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            client_secret: String::new(),
            redirect_uri: "http://localhost:8080/callback/dropbox".to_string(),
        }
    }
}

/// Dropbox configuration
#[derive(Debug, Clone)]
pub struct DropboxConfig {
    pub oauth: DropboxOAuthConfig,
    pub api_base_url: String,
    pub content_base_url: String,
    pub timeout_seconds: u64,
}

impl Default for DropboxConfig {
    fn default() -> Self {
        Self {
            oauth: DropboxOAuthConfig::default(),
            api_base_url: "https://api.dropboxapi.com/2".to_string(),
            content_base_url: "https://content.dropboxapi.com/2".to_string(),
            timeout_seconds: 60,
        }
    }
}

/// Dropbox credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropboxCredentials {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: DateTime<Utc>,
}

/// Dropbox file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropboxFile {
    pub id: String,
    pub name: String,
    pub path: String,
    pub size: Option<u64>,
    pub modified_time: Option<String>,
    pub is_folder: bool,
    pub download_url: Option<String>,
}

/// Dropbox storage quota
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropboxQuota {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
}

/// Dropbox client
pub struct DropboxClient {
    config: DropboxConfig,
    client: Client,
    credentials: Option<DropboxCredentials>,
}

impl DropboxClient {
    pub fn new(config: DropboxConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()?;
        
        Ok(Self {
            config,
            client,
            credentials: None,
        })
    }
    
    /// Generate OAuth authorization URL
    pub fn get_auth_url(&self, state: &str) -> String {
        format!(
            "https://www.dropbox.com/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&state={}",
            urlencoding::encode(&self.config.oauth.client_id),
            urlencoding::encode(&self.config.oauth.redirect_uri),
            urlencoding::encode(state),
        )
    }
    
    /// Exchange authorization code for access token
    pub async fn authenticate(&mut self, code: &str) -> Result<DropboxCredentials> {
        let response = self.client
            .post("https://api.dropboxapi.com/oauth2/token")
            .form(&[
                ("client_id", self.config.oauth.client_id.as_str()),
                ("client_secret", self.config.oauth.client_secret.as_str()),
                ("code", code),
                ("redirect_uri", self.config.oauth.redirect_uri.as_str()),
                ("grant_type", "authorization_code"),
            ])
            .send()
            .await?;
        
        let token: serde_json::Value = response.json().await?;
        
        let credentials = DropboxCredentials {
            access_token: token["access_token"].as_str().unwrap_or_default().to_string(),
            refresh_token: token["refresh_token"].as_str().map(|s| s.to_string()),
            expires_at: Utc::now() + chrono::Duration::seconds(
                token["expires_in"].as_i64().unwrap_or(14400)
            ),
        };
        
        self.credentials = Some(credentials.clone());
        Ok(credentials)
    }
    
    /// Get storage quota
    pub async fn get_quota(&self) -> Result<DropboxQuota> {
        let token = self.credentials.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        let response = self.client
            .post(format!("{}/users/get_space_usage", self.config.api_base_url))
            .bearer_auth(&token.access_token)
            .send()
            .await?;
        
        let result: serde_json::Value = response.json().await?;
        
        let used = result["used"].as_u64().unwrap_or(0);
        let total = result["allocation"]["individual"]["allocated"].as_u64().unwrap_or(0);
        
        Ok(DropboxQuota {
            total_bytes: total,
            used_bytes: used,
            available_bytes: total.saturating_sub(used),
        })
    }
    
    /// List files in a folder
    pub async fn list_files(&self, path: Option<&str>) -> Result<Vec<DropboxFile>> {
        let token = self.credentials.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        let path = path.unwrap_or("");
        
        let body = serde_json::json!({
            "path": if path.is_empty() { "" } else { path },
            "include_deleted": false,
        });
        
        let response = self.client
            .post(format!("{}/files/list_folder", self.config.api_base_url))
            .bearer_auth(&token.access_token)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;
        
        let result: serde_json::Value = response.json().await?;
        let entries: Vec<serde_json::Value> = result["entries"].as_array()
            .cloned().unwrap_or_default();
        
        let files: Vec<DropboxFile> = entries.into_iter()
            .filter_map(|entry| {
                let tag = entry[".tag"].as_str().unwrap_or("file").to_string();
                Some(DropboxFile {
                    id: entry["id"].as_str().unwrap_or_default().to_string(),
                    name: entry["name"].as_str()?.to_string(),
                    path: entry["path_display"].as_str().unwrap_or_default().to_string(),
                    size: entry["size"].as_u64(),
                    modified_time: entry["server_modified"].as_str().map(|s| s.to_string()),
                    is_folder: tag == "folder",
                    download_url: None,
                })
            })
            .collect();
        
        Ok(files)
    }
    
    /// Search files
    pub async fn search_files(&self, query: &str, max_results: u32) -> Result<Vec<DropboxFile>> {
        let token = self.credentials.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        let body = serde_json::json!({
            "query": query,
            "options": {
                "max_results": max_results,
            }
        });
        
        let response = self.client
            .post(format!("{}/files/search_v2", self.config.api_base_url))
            .bearer_auth(&token.access_token)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;
        
        let result: serde_json::Value = response.json().await?;
        let matches: Vec<serde_json::Value> = result["matches"].as_array()
            .cloned().unwrap_or_default();
        
        let files: Vec<DropboxFile> = matches.into_iter()
            .filter_map(|m| {
                let metadata = &m["metadata"]["metadata"];
                let tag = metadata[".tag"].as_str().unwrap_or("file").to_string();
                Some(DropboxFile {
                    id: metadata["id"].as_str().unwrap_or_default().to_string(),
                    name: metadata["name"].as_str()?.to_string(),
                    path: String::new(),
                    size: metadata["size"].as_u64(),
                    modified_time: metadata["server_modified"].as_str().map(|s| s.to_string()),
                    is_folder: tag == "folder",
                    download_url: None,
                })
            })
            .collect();
        
        Ok(files)
    }
    
    /// Get credentials
    pub fn get_credentials(&self) -> Option<&DropboxCredentials> {
        self.credentials.as_ref()
    }
    
    /// Set credentials
    pub fn set_credentials(&mut self, credentials: DropboxCredentials) {
        self.credentials = Some(credentials);
    }
}