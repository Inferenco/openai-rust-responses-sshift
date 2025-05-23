use crate::error::{try_parse_api_error, Result};
use crate::types::{PaginatedList, PaginationParams};
use chrono::{DateTime, Utc};
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};

/// Vector stores API endpoints
#[derive(Debug, Clone)]
pub struct VectorStores {
    client: HttpClient,
    base_url: String,
}

/// Vector store object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorStore {
    /// Unique identifier for the vector store
    pub id: String,

    /// Type of object (always "vector_store")
    pub object: String,

    /// Name of the vector store
    pub name: String,

    /// Unix timestamp for when the vector store was created
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,

    /// Status of the vector store
    pub status: String,

    /// Status details if the vector store is in an error state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,

    /// File IDs associated with this vector store
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
}

/// Request to create a new vector store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVectorStoreRequest {
    /// Name of the vector store
    pub name: String,

    /// File IDs to include in the vector store
    pub file_ids: Vec<String>,
}

/// Request to add a file to a vector store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddFileToVectorStoreRequest {
    /// File ID to add to the vector store
    pub file_id: String,

    /// Optional attributes for the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
}

/// Request to search a vector store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchVectorStoreRequest {
    /// Query to search for
    pub query: String,

    /// Maximum number of results to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_num_results: Option<u32>,
}

/// Result from searching a vector store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchVectorStoreResult {
    /// Filename of the file that matched the query
    pub filename: String,

    /// Content that matched the query
    pub content: Vec<SearchContent>,

    /// Score indicating how well the result matched the query
    pub score: f64,
}

/// Content structure in search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchContent {
    /// The actual text content
    pub text: String,
}

/// Response from searching a vector store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchVectorStoreResponse {
    /// Results from the search
    pub data: Vec<SearchVectorStoreResult>,
}

impl VectorStores {
    /// Creates a new Vector Stores API client
    pub(crate) fn new(client: HttpClient, base_url: String) -> Self {
        Self { client, base_url }
    }

    /// Creates a new vector store.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn create(&self, request: CreateVectorStoreRequest) -> Result<VectorStore> {
        let response = self
            .client
            .post(format!("{}/vector_stores", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }

    /// Retrieves a vector store by ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn get(&self, vector_store_id: &str) -> Result<VectorStore> {
        let response = self
            .client
            .get(format!(
                "{}/vector_stores/{}",
                self.base_url, vector_store_id
            ))
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }

    /// Lists all vector stores.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn list(
        &self,
        params: Option<PaginationParams>,
    ) -> Result<PaginatedList<VectorStore>> {
        let mut request = self.client.get(format!("{}/vector_stores", self.base_url));

        if let Some(params) = params {
            request = request.query(&params);
        }

        let response = request.send().await.map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }

    /// Deletes a vector store.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn delete(&self, vector_store_id: &str) -> Result<()> {
        let response = self
            .client
            .delete(format!(
                "{}/vector_stores/{}",
                self.base_url, vector_store_id
            ))
            .send()
            .await
            .map_err(crate::Error::Http)?;

        try_parse_api_error(response).await?;
        Ok(())
    }

    /// Adds a file to a vector store.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn add_file(
        &self,
        vector_store_id: &str,
        request: AddFileToVectorStoreRequest,
    ) -> Result<serde_json::Value> {
        let response = self
            .client
            .post(format!(
                "{}/vector_stores/{}/files",
                self.base_url, vector_store_id
            ))
            .json(&request)
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }

    /// Removes a file from a vector store.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn delete_file(&self, vector_store_id: &str, file_id: &str) -> Result<VectorStore> {
        let response = self
            .client
            .delete(format!(
                "{}/vector_stores/{}/files/{}",
                self.base_url, vector_store_id, file_id
            ))
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }

    /// Searches a vector store.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn search(
        &self,
        vector_store_id: &str,
        request: SearchVectorStoreRequest,
    ) -> Result<SearchVectorStoreResponse> {
        let response = self
            .client
            .post(format!(
                "{}/vector_stores/{}/search",
                self.base_url, vector_store_id
            ))
            .json(&request)
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }
}
