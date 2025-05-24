use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Container configuration for tools that support it
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Container {
    /// Container type (e.g., "default", "secure", etc.)
    #[serde(rename = "type")]
    pub container_type: String,
}

impl Container {
    /// Creates a default container configuration
    #[must_use]
    pub fn default_type() -> Self {
        Self {
            container_type: "default".to_string(),
        }
    }
}

/// Tool definition for the OpenAI Responses API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tool {
    /// Type of tool
    #[serde(rename = "type")]
    pub tool_type: String,

    /// Name of the function (for function tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Description of the function (for function tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Parameters for the function in JSON Schema format (for function tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,

    /// Vector store IDs for file search tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_store_ids: Option<Vec<String>>,

    /// Container configuration for tools that support it (code_interpreter, image_generation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,

    /// Server label for MCP tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_label: Option<String>,

    /// Server URL for MCP tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,

    /// Headers for MCP tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,

    /// Function definition for the tool (legacy support)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<ToolFunction>,
}

/// Function definition for a tool
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolFunction {
    /// Name of the function
    pub name: String,

    /// Description of the function
    pub description: String,

    /// Parameters for the function in JSON Schema format
    pub parameters: serde_json::Value,
}

/// Tool choice configuration for the OpenAI Responses API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ToolChoice {
    /// Automatic tool choice
    String(String),

    /// Specific tool choice
    Object {
        /// Type of tool choice (always "function")
        #[serde(rename = "type")]
        choice_type: String,

        /// Function to use
        function: ToolChoiceFunction,
    },
}

/// Function choice for tool choice
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolChoiceFunction {
    /// Name of the function to use
    pub name: String,
}

impl Tool {
    /// Creates a new function tool
    pub fn function(
        name: impl Into<String>,
        description: impl Into<String>,
        parameters: serde_json::Value,
    ) -> Self {
        Self {
            tool_type: "function".to_string(),
            name: Some(name.into()),
            description: Some(description.into()),
            parameters: Some(parameters),
            vector_store_ids: None,
            container: None,
            server_label: None,
            server_url: None,
            headers: None,
            function: None,
        }
    }

    /// Creates a web search preview tool
    #[must_use]
    pub fn web_search_preview() -> Self {
        Self {
            tool_type: "web_search_preview".to_string(),
            name: None,
            description: None,
            parameters: None,
            vector_store_ids: None,
            container: None,
            server_label: None,
            server_url: None,
            headers: None,
            function: None,
        }
    }

    /// Creates a file search tool
    #[must_use]
    pub fn file_search(vector_store_ids: Vec<String>) -> Self {
        Self {
            tool_type: "file_search".to_string(),
            name: None,
            description: None,
            parameters: None,
            function: None,
            vector_store_ids: Some(vector_store_ids),
            container: None,
            server_label: None,
            server_url: None,
            headers: None,
        }
    }

    /// Creates a computer use preview tool
    #[must_use]
    pub fn computer_use_preview() -> Self {
        Self {
            tool_type: "computer_use_preview".to_string(),
            name: None,
            description: None,
            parameters: None,
            vector_store_ids: None,
            container: None,
            server_label: None,
            server_url: None,
            headers: None,
            function: None,
        }
    }

    /// Creates a code interpreter tool
    #[must_use]
    pub fn code_interpreter(container: Option<Container>) -> Self {
        Self {
            tool_type: "code_interpreter".to_string(),
            name: None,
            description: None,
            parameters: None,
            vector_store_ids: None,
            container,
            server_label: None,
            server_url: None,
            headers: None,
            function: None,
        }
    }

    /// Creates an image generation tool (NEW for May 2025)
    #[must_use]
    pub fn image_generation(container: Option<Container>) -> Self {
        Self {
            tool_type: "image_generation".to_string(),
            name: None,
            description: None,
            parameters: None,
            vector_store_ids: None,
            container,
            server_label: None,
            server_url: None,
            headers: None,
            function: None,
        }
    }

    /// Creates an MCP (Model Context Protocol) server tool (NEW for May 2025)
    #[must_use]
    pub fn mcp(
        server_label: impl Into<String>,
        server_url: impl Into<String>,
        headers: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            tool_type: "mcp".to_string(),
            name: None,
            description: None,
            parameters: None,
            vector_store_ids: None,
            container: None,
            server_label: Some(server_label.into()),
            server_url: Some(server_url.into()),
            headers,
            function: None,
        }
    }
}

impl ToolChoice {
    /// Auto tool choice - let the model decide when to use tools
    #[must_use]
    pub fn auto() -> Self {
        Self::String("auto".to_string())
    }

    /// Required tool choice - model must use a tool
    #[must_use]
    pub fn required() -> Self {
        Self::String("required".to_string())
    }

    /// Creates a tool choice that specifies a specific function
    pub fn function(name: impl Into<String>) -> Self {
        Self::Object {
            choice_type: "function".to_string(),
            function: ToolChoiceFunction { name: name.into() },
        }
    }
}
