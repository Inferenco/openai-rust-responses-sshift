use async_trait::async_trait;
use open_ai_rust_responses_by_sshift::mcp::{HttpTransport, LocalTool, McpClient, ToolRegistry};
use serde_json::{json, Value};
use std::sync::Arc;

/// A simple local calculator tool
struct CalculatorTool;

#[async_trait]
impl LocalTool for CalculatorTool {
    fn name(&self) -> &str {
        "calculator"
    }

    fn description(&self) -> &str {
        "Performs basic arithmetic operations"
    }

    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["add", "subtract", "multiply", "divide"],
                    "description": "The operation to perform"
                },
                "a": {
                    "type": "number",
                    "description": "The first number"
                },
                "b": {
                    "type": "number",
                    "description": "The second number"
                }
            },
            "required": ["operation", "a", "b"]
        })
    }

    async fn call(&self, args: Value) -> open_ai_rust_responses_by_sshift::Result<Value> {
        let op = args["operation"].as_str().unwrap_or("add");
        let a = args["a"].as_f64().unwrap_or(0.0);
        let b = args["b"].as_f64().unwrap_or(0.0);

        let result = match op {
            "add" => a + b,
            "subtract" => a - b,
            "multiply" => a * b,
            "divide" => {
                if b == 0.0 {
                    return Err(open_ai_rust_responses_by_sshift::Error::Mcp(
                        "Division by zero".to_string(),
                    ));
                }
                a / b
            }
            _ => {
                return Err(open_ai_rust_responses_by_sshift::Error::Mcp(
                    "Unknown operation".to_string(),
                ))
            }
        };

        Ok(json!({ "result": result }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create a ToolRegistry
    let mut registry = ToolRegistry::new();

    // 2. Register a local tool
    registry.register_local_tool(Box::new(CalculatorTool));
    println!("Registered local tool: calculator");

    // 3. (Optional) Configure an MCP client
    // In a real scenario, you would connect to a running MCP server.
    // For this example, we'll show the setup code.
    let mcp_url =
        std::env::var("MCP_SERVER_URL").unwrap_or_else(|_| "http://localhost:3400/rpc".to_string());

    println!("Connecting to MCP server at: {}", mcp_url);
    let transport = HttpTransport::new(&mcp_url);
    let client = Arc::new(McpClient::new(Box::new(transport)));

    // Note: In a real app, you might want to initialize the client first
    // client.initialize().await?;

    registry.set_mcp_client(client);

    // 4. List all tools (local + MCP)
    // This would fetch tools from the MCP server and combine them with local tools
    // let tools = registry.list_tools().await?;
    // println!("Available tools: {}", tools.len());

    // 5. Execute a tool
    // The registry automatically routes the call to the correct handler

    // Call local tool
    let args = json!({
        "operation": "multiply",
        "a": 5,
        "b": 3
    });

    println!("Calling local tool 'calculator' with args: {}", args);
    let result = registry.call_tool("calculator", args).await?;
    println!("Result: {}", result);

    // Call MCP tool (if one existed with this name)
    // let mcp_result = registry.call_tool("mcp_tool_name", json!({})).await?;

    Ok(())
}
