use crate::error::{Result, try_parse_api_error};
use crate::types::{PaginatedList, PaginationParams};
use reqwest::{Client as HttpClient, StatusCode};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::path::Path;

/// Files API endpoints
#[derive(Debug, Clone)]
pub struct Files {
    client: HttpClient,
    base_url: String,
}

/// File object representing a file in the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    /// Unique identifier for the file
    pub id: String,
    
    /// Type of object (always "file")
    pub object: String,
    
    /// Name of the file
    pub filename: String,
    
    /// Purpose of the file
    pub purpose: String,
    
    /// Size of the file in bytes
    pub bytes: u64,
    
    /// Unix timestamp for when the file was created
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    
    /// Status of the file
    pub status: String,
    
    /// Status details if the file is in an error state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
}

/// Purpose of a file
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilePurpose {
    /// File for assistants
    Assistants,
    
    /// File for fine-tuning
    FineTuning,
    
    /// Custom purpose
    #[serde(untagged)]
    Custom(String),
}

impl From<&str> for FilePurpose {
    fn from(s: &str) -> Self {
        match s {
            "assistants" => Self::Assistants,
            "fine-tuning" => Self::FineTuning,
            _ => Self::Custom(s.to_string()),
        }
    }
}

impl From<String> for FilePurpose {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

/// Request to create a new file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileRequest {
    /// Purpose of the file
    pub purpose: String,
    
    /// File data
    #[serde(skip)]
    pub file: Vec<u8>,
    
    /// Filename
    pub filename: String,
    
    /// Optional MIME type for the file
    #[serde(skip)]
    pub mime_type: Option<String>,
}

impl Files {
    /// Creates a new Files API client
    pub(crate) fn new(client: HttpClient, base_url: String) -> Self {
        Self { client, base_url }
    }

    /// Creates a new file.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn create(&self, request: CreateFileRequest) -> Result<File> {
        let file_part = if let Some(mime) = &request.mime_type {
            reqwest::multipart::Part::bytes(request.file)
                .file_name(request.filename.clone())
                .mime_str(mime)
                .map_err(|e| crate::Error::Stream(e.to_string()))?
        } else {
            // Infer MIME type from filename
            let mime = mime_guess::from_path(&request.filename)
                .first_or_octet_stream();
                
            reqwest::multipart::Part::bytes(request.file)
                .file_name(request.filename.clone())
                .mime_str(mime.as_ref())
                .map_err(|e| crate::Error::Stream(e.to_string()))?
        };

        let form = reqwest::multipart::Form::new()
            .text("purpose", request.purpose)
            .part("file", file_part);

        let response = self
            .client
            .post(format!("{}/files", self.base_url))
            .multipart(form)
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }
    
    /// Uploads a file from a path.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read, the request fails to send, or has a non-200 status code.
    pub async fn upload_file<P: AsRef<Path>>(
        &self,
        path: P,
        purpose: impl Into<FilePurpose>,
        mime_type: Option<String>,
    ) -> Result<File> {
        let path = path.as_ref();
        let filename = path.file_name()
            .ok_or_else(|| crate::Error::Stream("Invalid file path".to_string()))?
            .to_string_lossy()
            .to_string();
            
        let file_data = tokio::fs::read(path)
            .await
            .map_err(|e| crate::Error::Stream(format!("Failed to read file: {}", e)))?;
            
        let purpose_str = match purpose.into() {
            FilePurpose::Assistants => "assistants".to_string(),
            FilePurpose::FineTuning => "fine-tuning".to_string(),
            FilePurpose::Custom(s) => s,
        };
        
        let request = CreateFileRequest {
            purpose: purpose_str,
            file: file_data,
            filename,
            mime_type,
        };
        
        self.create(request).await
    }

    /// Retrieves a file with the given ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn get(&self, file_id: &str) -> Result<File> {
        let response = self
            .client
            .get(format!("{}/files/{}", self.base_url, file_id))
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }

    /// Lists all files.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn list(&self, params: Option<PaginationParams>) -> Result<PaginatedList<File>> {
        let mut request = self
            .client
            .get(format!("{}/files", self.base_url));
            
        if let Some(params) = params {
            request = request.query(&params);
        }
        
        let response = request
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }

    /// Deletes a file with the given ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn delete(&self, file_id: &str) -> Result<()> {
        let response = self
            .client
            .delete(format!("{}/files/{}", self.base_url, file_id))
            .send()
            .await
            .map_err(crate::Error::Http)?;

        try_parse_api_error(response).await?;
        Ok(())
    }

    /// Downloads the content of a file with the given ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn download(&self, file_id: &str) -> Result<Vec<u8>> {
        let response = self
            .client
            .get(format!("{}/files/{}/content", self.base_url, file_id))
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.bytes().await.map(|b| b.to_vec()).map_err(crate::Error::Http)
    }
}
