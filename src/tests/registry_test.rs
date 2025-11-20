#[test]
fn test_tool_registry_dispatch() {
    use crate::mcp::transport::McpTransport;
    use crate::mcp::types::{JsonRpcRequest, JsonRpcResponse};
    use crate::mcp::{LocalTool, McpClient, ToolRegistry};
    use async_trait::async_trait;
    use serde_json::json;
    use std::sync::{Arc, Mutex};

    struct MockTransport {
        calls: Arc<Mutex<Vec<String>>>,
    }

    #[async_trait]
    impl McpTransport for MockTransport {
        async fn send(&self, message: &JsonRpcRequest) -> crate::Result<JsonRpcResponse> {
            self.calls.lock().unwrap().push(message.method.clone());

            let result = if message.method == "tools/call" {
                json!({
                    "content": [{
                        "type": "text",
                        "text": "{\"result\": \"mcp\"}"
                    }]
                })
            } else {
                json!({ "tools": [] })
            };

            Ok(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(result),
                error: None,
                id: message.id.clone(),
            })
        }
    }

    struct MockLocalTool {
        name: String,
    }

    #[async_trait]
    impl LocalTool for MockLocalTool {
        fn name(&self) -> &str {
            &self.name
        }
        fn description(&self) -> &'static str {
            "Mock tool"
        }
        fn schema(&self) -> serde_json::Value {
            json!({})
        }
        async fn call(&self, _args: serde_json::Value) -> crate::Result<serde_json::Value> {
            Ok(json!({ "result": "local" }))
        }
    }

    tokio_test::block_on(async {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let transport = MockTransport {
            calls: calls.clone(),
        };
        let client = Arc::new(McpClient::new(Box::new(transport)));

        let mut registry = ToolRegistry::new();
        registry.set_mcp_client(client);
        registry.register_local_tool(Box::new(MockLocalTool {
            name: "local_tool".to_string(),
        }));

        // Test calling local tool
        let result = registry.call_tool("local_tool", json!({})).await.unwrap();
        assert_eq!(result["result"], "local");

        // Verify MCP was NOT called for local tool
        {
            let calls = calls.lock().unwrap();
            assert!(!calls.contains(&"tools/call".to_string()));
        }

        // Test calling MCP tool
        let result = registry.call_tool("mcp_tool", json!({})).await.unwrap();
        assert_eq!(result["result"], "mcp");

        // Verify MCP WAS called for MCP tool
        {
            let calls = calls.lock().unwrap();
            assert!(calls.contains(&"tools/call".to_string()));
        }
    });
}
