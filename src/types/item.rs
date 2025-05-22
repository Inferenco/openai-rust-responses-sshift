use serde::{Deserialize, Serialize};

/// Input for the OpenAI Responses API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input {
    /// Text input
    Text(String),

    /// List of input items
    Items(Vec<InputItem>),
}

/// Input item for the API request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputItem {
    /// Type of the input item
    #[serde(rename = "type")]
    pub item_type: String,

    /// Content of the input item
    pub content: serde_json::Value,
}

/// Response item from the OpenAI Responses API
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseItem {
    /// Text response
    Text {
        /// Content of the text response
        content: String,

        /// Index of the text response
        index: u32,
    },

    /// Tool call response
    #[serde(rename = "tool_call")]
    ToolCall(ToolCall),
}

/// Tool call from the OpenAI Responses API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolCall {
    /// ID of the tool call
    pub id: String,

    /// Name of the tool
    pub name: String,

    /// Arguments for the tool call
    pub arguments: serde_json::Value,

    /// Index of the tool call
    pub index: u32,
}

/// Tool result for the OpenAI Responses API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// ID of the tool call this result is for
    pub tool_call_id: String,

    /// Result of the tool call
    pub result: serde_json::Value,
}
