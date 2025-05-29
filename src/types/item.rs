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

impl InputItem {
    /// Creates a text input item
    pub fn text(content: impl Into<String>) -> Self {
        Self {
            item_type: "text".to_string(),
            content: serde_json::Value::String(content.into()),
        }
    }

    /// Creates a function call output input item for submitting tool results
    pub fn function_call_output(call_id: impl Into<String>, output: impl Into<String>) -> Self {
        Self {
            item_type: "function_call_output".to_string(),
            content: serde_json::json!({
                "call_id": call_id.into(),
                "output": output.into()
            }),
        }
    }
}

/// Response item from the OpenAI Responses API
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseItem {
    /// Message response
    Message {
        /// ID of the message
        id: String,

        /// Content of the message
        content: Vec<MessageContent>,

        /// Role of the message
        role: String,

        /// Status of the message
        status: Option<String>,
    },

    /// Reasoning response
    Reasoning {
        /// ID of the reasoning item
        id: String,

        /// Summary of the reasoning
        summary: Vec<serde_json::Value>,

        /// Status of the reasoning
        status: Option<String>,
    },

    /// Web search call
    WebSearchCall {
        /// ID of the web search call
        id: String,

        /// Status of the web search call
        status: String,
    },

    /// File search call
    FileSearchCall {
        /// ID of the file search call
        id: String,

        /// Status of the file search call
        status: String,
    },

    /// Function call
    FunctionCall {
        /// ID of the function call
        id: String,

        /// Arguments for the function call
        arguments: String,

        /// Call ID
        call_id: String,

        /// Name of the function
        name: String,

        /// Status of the function call
        status: String,
    },

    /// Text response (legacy)
    Text {
        /// Content of the text response
        content: String,

        /// Index of the text response
        index: u32,
    },

    /// Tool call response (legacy)
    #[serde(rename = "tool_call")]
    ToolCall(ToolCall),
}

/// Message content item
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageContent {
    /// Output text content
    OutputText {
        /// Text content
        text: String,

        /// Annotations
        annotations: Vec<serde_json::Value>,

        /// Log probabilities
        logprobs: Option<serde_json::Value>,
    },
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

/// Function call information
#[derive(Debug, Clone)]
pub struct FunctionCallInfo {
    /// Name of the function
    pub name: String,

    /// Arguments for the function call
    pub arguments: String,

    /// Call ID
    pub call_id: String,
}
