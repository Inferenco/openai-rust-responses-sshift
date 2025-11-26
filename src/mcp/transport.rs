use super::types::{JsonRpcRequest, JsonRpcResponse};
use crate::error::Result;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;

#[async_trait]
pub trait McpTransport: Send + Sync {
    async fn send(&self, message: &JsonRpcRequest) -> Result<JsonRpcResponse>;
}

/// HTTP transport for MCP servers.
///
/// This transport implements `Clone`, allowing you to reuse the same transport
/// configuration (URL, headers, authentication) across multiple clients or contexts.
///
/// # Example
///
/// ```rust
/// use open_ai_rust_responses_by_sshift::mcp::transport::HttpTransport;
///
/// let transport = HttpTransport::new("http://localhost:8000/mcp")
///     .with_bearer_token("your-token")?;
///
/// // Clone the transport to reuse the same configuration
/// let cloned = transport.clone();
/// ```
#[derive(Clone)]
pub struct HttpTransport {
    client: Client,
    url: String,
    headers: HeaderMap,
}

impl HttpTransport {
    #[must_use]
    pub fn new(url: &str) -> Self {
        Self {
            client: Client::new(),
            url: url.to_string(),
            headers: HeaderMap::new(),
        }
    }

    /// Adds a header to the transport.
    ///
    /// # Errors
    /// Returns an error if the header name or value is invalid.
    pub fn with_header(mut self, key: &str, value: &str) -> Result<Self> {
        let name = HeaderName::from_bytes(key.as_bytes())
            .map_err(|e| crate::Error::Mcp(format!("Invalid header name: {e}")))?;
        let value = HeaderValue::from_str(value)
            .map_err(|e| crate::Error::Mcp(format!("Invalid header value: {e}")))?;
        self.headers.insert(name, value);
        Ok(self)
    }

    /// Adds Bearer token authorization to the transport.
    ///
    /// This is a convenience method for adding `Authorization: Bearer <token>` header.
    ///
    /// # Errors
    /// Returns an error if the token cannot be converted to a valid header value.
    pub fn with_bearer_token(mut self, token: &str) -> Result<Self> {
        let auth_value = format!("Bearer {token}");
        let value = HeaderValue::from_str(&auth_value)
            .map_err(|e| crate::Error::Mcp(format!("Invalid bearer token: {e}")))?;
        self.headers.insert(reqwest::header::AUTHORIZATION, value);
        Ok(self)
    }
}

#[async_trait]
impl McpTransport for HttpTransport {
    async fn send(&self, message: &JsonRpcRequest) -> Result<JsonRpcResponse> {
        let response = self
            .client
            .post(&self.url)
            .headers(self.headers.clone())
            .json(message)
            .send()
            .await
            .map_err(|e| crate::Error::Mcp(format!("HTTP request failed: {e}")))?;

        let rpc_response: JsonRpcResponse = response
            .json()
            .await
            .map_err(|e| crate::Error::Mcp(format!("Failed to parse response: {e}")))?;

        Ok(rpc_response)
    }
}
