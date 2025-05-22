use serde::{Deserialize, Serialize};

/// Tool definition for the OpenAI Responses API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Type of tool
    #[serde(rename = "type")]
    pub tool_type: String,
    
    /// Function definition for the tool
    pub function: ToolFunction,
}

/// Function definition for a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFunction {
    /// Name of the function
    pub name: String,
    
    /// Description of the function
    pub description: String,
    
    /// Parameters for the function in JSON Schema format
    pub parameters: serde_json::Value,
}

/// Tool choice configuration for the OpenAI Responses API
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolChoiceFunction {
    /// Name of the function to use
    pub name: String,
}

impl Tool {
    /// Creates a new function tool
    pub fn function(name: impl Into<String>, description: impl Into<String>, parameters: serde_json::Value) -> Self {
        Self {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: name.into(),
                description: description.into(),
                parameters,
            },
        }
    }
}

impl ToolChoice {
    /// Creates an automatic tool choice
    pub fn auto() -> Self {
        Self::String("auto".to_string())
    }
    
    /// Creates a tool choice that requires the model to use a function
    pub fn required() -> Self {
        Self::String("required".to_string())
    }
    
    /// Creates a tool choice that specifies a specific function
    pub fn function(name: impl Into<String>) -> Self {
        Self::Object {
            choice_type: "function".to_string(),
            function: ToolChoiceFunction {
                name: name.into(),
            },
        }
    }
}
