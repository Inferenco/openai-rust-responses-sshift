use super::transport::McpTransport;
use super::types::*;
use crate::error::Result;
use serde_json::json;
use std::sync::{Arc, Mutex};

pub struct McpClient {
    transport: Box<dyn McpTransport>,
    request_id: Arc<Mutex<u64>>,
}

impl McpClient {
    pub fn new(transport: Box<dyn McpTransport>) -> Self {
        Self {
            transport,
            request_id: Arc::new(Mutex::new(0)),
        }
    }

    fn next_id(&self) -> u64 {
        let mut id = self.request_id.lock().unwrap();
        *id += 1;
        *id
    }

    pub async fn send_request<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<T> {
        let id = self.next_id();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id: Some(json!(id)),
        };

        let response = self.transport.send(&request).await?;

        if let Some(error) = response.error {
            return Err(crate::Error::Mcp(format!(
                "MCP Error {}: {}",
                error.code, error.message
            )));
        }

        if let Some(result) = response.result {
            serde_json::from_value(result)
                .map_err(|e| crate::Error::Mcp(format!("Failed to parse result: {}", e)))
        } else {
            Err(crate::Error::Mcp("Empty result in response".to_string()))
        }
    }

    pub async fn initialize(&self) -> Result<()> {
        let _: serde_json::Value = self
            .send_request(
                "initialize",
                Some(json!({
                    "protocolVersion": "0.1.0",
                    "capabilities": {},
                    "clientInfo": {
                        "name": "openai-rust-responses-sshift",
                        "version": "0.1.0"
                    }
                })),
            )
            .await?;

        // Send initialized notification
        let notification = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "notifications/initialized".to_string(),
            params: None,
            id: None,
        };

        self.transport.send(&notification).await?;

        Ok(())
    }

    pub async fn list_tools(&self) -> Result<Vec<McpTool>> {
        let result: ListToolsResult = self.send_request("tools/list", None).await?;
        Ok(result.tools)
    }

    pub async fn call_tool(
        &self,
        name: &str,
        arguments: serde_json::Value,
    ) -> Result<CallToolResult> {
        self.send_request(
            "tools/call",
            Some(json!({
                "name": name,
                "arguments": arguments
            })),
        )
        .await
    }
}
