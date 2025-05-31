mod types;
pub use types::*;

use crate::error::{try_parse_api_error, Result};
use reqwest::Client as HttpClient;

/// Images API endpoints
#[derive(Debug, Clone)]
pub struct Images {
    client: HttpClient,
    base_url: String,
}

impl Images {
    /// Creates a new Images API client
    pub(crate) fn new(client: HttpClient, base_url: String) -> Self {
        Self { client, base_url }
    }

    /// Generate images using gpt-image-1 model
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn generate(&self, request: ImageGenerateRequest) -> Result<ImageGenerateResponse> {
        let response = self
            .client
            .post(format!("{}/images/generations", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }
}
