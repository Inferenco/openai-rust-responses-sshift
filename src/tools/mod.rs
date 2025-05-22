use crate::error::{try_parse_api_error, Result};
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};

/// Tools API endpoints
#[derive(Debug, Clone)]
pub struct Tools {
    client: HttpClient,
    base_url: String,
}

/// Web search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSearchResult {
    /// Title of the search result
    pub title: String,

    /// URL of the search result
    pub url: String,

    /// Snippet of text from the search result
    pub snippet: String,
}

/// Response from a web search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSearchResponse {
    /// Results from the search
    pub results: Vec<WebSearchResult>,
}

/// File search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSearchResult {
    /// ID of the file that matched the search
    pub file_id: String,

    /// Snippet of text from the file that matched the search
    pub snippet: String,

    /// Score indicating how well the snippet matched the search
    pub score: f32,
}

/// Response from a file search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSearchResponse {
    /// Results from the search
    pub results: Vec<FileSearchResult>,
}

impl Tools {
    /// Creates a new Tools API client
    pub(crate) fn new(client: HttpClient, base_url: String) -> Self {
        Self { client, base_url }
    }

    /// Path constants for web search endpoint
    const WEB_SEARCH_PATH: &'static str = "/web_search"; // canonical
    const LEGACY_WEB_SEARCH_PATH: &'static str = "/tools/web_search";

    /// Performs a web search.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn web_search(&self, query: &str) -> Result<WebSearchResponse> {
        // Try the canonical path first
        let response = self
            .client
            .get(format!("{}{}", self.base_url, Self::WEB_SEARCH_PATH))
            .query(&[("query", query)])
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() || resp.status().as_u16() != 404 {
                    // If successful or any error other than 404, process normally
                    let response = try_parse_api_error(resp).await?;
                    response.json().await.map_err(crate::Error::Http)
                } else {
                    // If 404, try the legacy path
                    log::warn!(
                        "Web search endpoint {} returned 404, trying legacy path {}",
                        Self::WEB_SEARCH_PATH,
                        Self::LEGACY_WEB_SEARCH_PATH
                    );

                    let legacy_response = self
                        .client
                        .get(format!("{}{}", self.base_url, Self::LEGACY_WEB_SEARCH_PATH))
                        .query(&[("query", query)])
                        .send()
                        .await
                        .map_err(crate::Error::Http)?;

                    let response = try_parse_api_error(legacy_response).await?;
                    response.json().await.map_err(crate::Error::Http)
                }
            }
            Err(e) => Err(crate::Error::Http(e)),
        }
    }

    /// Searches files in a vector store.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn file_search(
        &self,
        vector_store_id: &str,
        query: &str,
    ) -> Result<FileSearchResponse> {
        let request = serde_json::json!({
            "query": query
        });

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
