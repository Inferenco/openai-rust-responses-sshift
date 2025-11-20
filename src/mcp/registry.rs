use crate::error::Result;
use crate::mcp::client::McpClient;
use crate::types::Tool;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Trait for local tools
#[async_trait]
pub trait LocalTool: Send + Sync {
    /// Returns the name of the tool
    fn name(&self) -> &str;

    /// Returns the description of the tool
    fn description(&self) -> &str;

    /// Returns the input schema of the tool
    fn schema(&self) -> Value;

    /// Executes the tool with the given arguments
    async fn call(&self, args: Value) -> Result<Value>;
}

/// Registry for managing both local and MCP tools.
///
/// The `ToolRegistry` serves as a unified interface for handling tools from different sources.
/// It allows you to register local Rust-based tools (implementing `LocalTool`) and connect
/// to a remote Model Context Protocol (MCP) server.
///
/// # Priority Logic
/// When `call_tool` is invoked, the registry follows this priority:
/// 1. **Local Tools**: Checks if a local tool with the given name exists. If found, it is executed locally.
/// 2. **MCP Tools**: If no local tool is found, it delegates the call to the configured MCP client.
///
/// # OpenAI Integration
/// The `list_tools` method aggregates tools from both sources and converts them into the
/// `Tool` format expected by the OpenAI API. This allows you to pass a single list of tools
/// to the LLM, which can then invoke either type transparently.
pub struct ToolRegistry {
    local_tools: HashMap<String, Box<dyn LocalTool>>,
    mcp_client: Option<Arc<McpClient>>,
}

impl ToolRegistry {
    /// Creates a new, empty tool registry.
    pub fn new() -> Self {
        Self {
            local_tools: HashMap::new(),
            mcp_client: None,
        }
    }

    /// Registers a local tool with the registry.
    ///
    /// Local tools take precedence over MCP tools with the same name.
    pub fn register_local_tool(&mut self, tool: Box<dyn LocalTool>) {
        self.local_tools.insert(tool.name().to_string(), tool);
    }

    /// Sets the MCP client for the registry.
    ///
    /// This enables the registry to discover and call tools from a remote MCP server.
    pub fn set_mcp_client(&mut self, client: Arc<McpClient>) {
        self.mcp_client = Some(client);
    }

    /// Returns a combined list of all tools (local + MCP) as OpenAI `Tool` objects.
    ///
    /// This method:
    /// 1. Collects all registered local tools.
    /// 2. Fetches available tools from the configured MCP server (if any).
    /// 3. Converts MCP tools to the OpenAI `Tool` format using `mcp_tool_to_openai_tool`.
    /// 4. Returns a unified vector ready to be sent in an OpenAI API request.
    pub async fn list_tools(&self) -> Result<Vec<Tool>> {
        let mut tools = Vec::new();

        // Add local tools
        for tool in self.local_tools.values() {
            tools.push(Tool::function(
                tool.name(),
                tool.description(),
                tool.schema(),
            ));
        }

        // Add MCP tools if client is configured
        if let Some(client) = &self.mcp_client {
            let mcp_tools = client.list_tools().await?;
            for mcp_tool in mcp_tools {
                tools.push(super::adapter::mcp_tool_to_openai_tool(mcp_tool));
            }
        }

        Ok(tools)
    }

    /// Calls a tool by name, handling dispatch to either a local implementation or the MCP server.
    ///
    /// # Arguments
    /// * `name` - The name of the tool to call.
    /// * `args` - The arguments to pass to the tool (as a JSON Value).
    ///
    /// # Returns
    /// * `Result<Value>` - The result of the tool execution.
    ///
    /// # Errors
    /// * Returns `Error::Mcp` if the tool is not found or if the MCP call fails.
    pub async fn call_tool(&self, name: &str, args: Value) -> Result<Value> {
        // Check local tools first
        if let Some(tool) = self.local_tools.get(name) {
            return tool.call(args).await;
        }

        // Fallback to MCP client
        if let Some(client) = &self.mcp_client {
            let result = client.call_tool(name, args).await?;
            // Convert CallToolResult content to Value
            // Assuming the first content item is the result text/json
            if let Some(content) = result.content.first() {
                match content {
                    crate::mcp::types::ToolContent::Text { text } => {
                        // Try to parse as JSON, otherwise return as string
                        return Ok(serde_json::from_str(text)
                            .unwrap_or_else(|_| Value::String(text.clone())));
                    }
                    _ => return Ok(Value::Null),
                }
            }
            return Ok(Value::Null);
        }

        Err(crate::Error::Mcp(format!("Tool not found: {}", name)))
    }
}
