use super::types::McpTool;
use crate::types::Tool;

#[must_use]
pub fn mcp_tool_to_openai_tool(mcp_tool: McpTool) -> Tool {
    Tool::function(
        mcp_tool.name,
        mcp_tool.description.unwrap_or_default(),
        mcp_tool.input_schema,
    )
}
