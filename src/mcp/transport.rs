use super::types::*;
use crate::error::Result;
use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait McpTransport: Send + Sync {
    async fn send(&self, message: &JsonRpcRequest) -> Result<JsonRpcResponse>;
}

pub struct HttpTransport {
    client: Client,
    url: String,
}

impl HttpTransport {
    pub fn new(url: &str) -> Self {
        Self {
            client: Client::new(),
            url: url.to_string(),
        }
    }
}

#[async_trait]
impl McpTransport for HttpTransport {
    async fn send(&self, message: &JsonRpcRequest) -> Result<JsonRpcResponse> {
        let response = self
            .client
            .post(&self.url)
            .json(message)
            .send()
            .await
            .map_err(|e| crate::Error::Mcp(format!("HTTP request failed: {}", e)))?;

        let rpc_response: JsonRpcResponse = response
            .json()
            .await
            .map_err(|e| crate::Error::Mcp(format!("Failed to parse response: {}", e)))?;

        Ok(rpc_response)
    }
}
