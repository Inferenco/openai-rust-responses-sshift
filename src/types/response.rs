use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Token usage information for the response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Usage {
    /// Number of tokens in the input (including images and tools if any)
    pub input_tokens: u32,

    /// Number of tokens generated in the output
    pub output_tokens: u32,

    /// Total number of tokens used (input + output)
    pub total_tokens: u32,

    /// Additional details about output tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens_details: Option<OutputTokensDetails>,

    /// Additional details about input tokens  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokensDetails>,

    /// Number of web search tool calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search: Option<u32>,

    /// Number of file search tool calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_search: Option<u32>,

    /// Number of image generation tool calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_generation: Option<u32>,

    /// Number of code interpreter tool calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_interpreter: Option<u32>,
}

/// Details about output tokens
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutputTokensDetails {
    /// Number of tokens used for reasoning (for reasoning models)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u32>,
}

/// Details about input tokens
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PromptTokensDetails {
    /// Number of cached tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_tokens: Option<u32>,
}

/// Details about incomplete responses
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IncompleteDetails {
    /// Reason the response was incomplete
    pub reason: String,
}

/// Error information in the response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseError {
    /// Error code
    pub code: String,

    /// Error message
    pub message: String,

    /// Additional error metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Text generation configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextConfig {
    /// Text format configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<TextFormat>,

    /// Stop sequences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
}

/// Text format configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextFormat {
    /// Format type (e.g., "text")
    #[serde(rename = "type")]
    pub format_type: String,
}

/// Truncation configuration - can be either a string ("disabled", "auto") or a config object
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TruncationSetting {
    /// Simple string setting like "disabled" or "auto"
    Simple(String),
    /// Full configuration object
    Config(TruncationConfig),
}

/// Truncation configuration object
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TruncationConfig {
    /// Truncation type (e.g., "auto", "disabled")
    #[serde(rename = "type")]
    pub truncation_type: String,

    /// Last messages to keep when truncating
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_messages: Option<u32>,
}

/// Reasoning output from the model
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReasoningOutput {
    /// Reasoning trace content (encrypted when using store=false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<ReasoningContent>>,

    /// Encrypted reasoning content for stateless mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
}

/// Individual reasoning content item
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReasoningContent {
    /// Type of reasoning content
    #[serde(rename = "type")]
    pub content_type: String,

    /// Reasoning text content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Response from the OpenAI Responses API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    /// Unique identifier for the response
    pub id: String,

    /// Object type (always "response")
    #[serde(default = "default_object_type")]
    pub object: String,

    /// Creation timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,

    /// The model used to generate the response
    pub model: String,

    /// Current status of the response (queued | in_progress | completed | cancelled | failed)
    #[serde(default = "default_status")]
    pub status: String,

    /// The output items generated by the model
    pub output: Vec<crate::types::ResponseItem>,

    /// Convenience field containing merged output text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_text: Option<String>,

    /// Optional ID of the previous response in the conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,

    /// System instructions that guided the model's behavior
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Optional metadata associated with the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    /// Token usage statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,

    /// Sampling temperature used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Nucleus sampling parameter used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Maximum output tokens requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,

    /// Whether parallel tool calls were enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// Tool choice configuration used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<crate::types::ToolChoice>,

    /// Tools that were available to the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<crate::types::Tool>>,

    /// Text generation configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextConfig>,

    /// Number of top log probabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,

    /// Truncation configuration used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<TruncationSetting>,

    /// Reasoning output (for reasoning models)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningOutput>,

    /// Reasoning effort level used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,

    /// User identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Details about incomplete responses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<IncompleteDetails>,

    /// Error information if the response failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,
}

fn default_object_type() -> String {
    "response".to_string()
}

fn default_status() -> String {
    "completed".to_string()
}

impl Response {
    /// Returns the response ID
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns true if the response is in a completed state
    #[must_use]
    pub fn is_complete(&self) -> bool {
        matches!(self.status.as_str(), "completed" | "cancelled" | "failed")
    }

    /// Returns true if the response is currently being processed
    #[must_use]
    pub fn is_in_progress(&self) -> bool {
        matches!(self.status.as_str(), "queued" | "in_progress")
    }

    /// Returns true if the response has errors
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.error.is_some() || self.status == "failed"
    }

    /// Returns the total token count if available
    #[must_use]
    pub fn total_tokens(&self) -> Option<u32> {
        self.usage.as_ref().map(|u| u.total_tokens)
    }

    /// Returns the response output as text if available
    #[must_use]
    pub fn output_text(&self) -> String {
        // First try the convenience field
        if let Some(ref text) = self.output_text {
            return text.clone();
        }

        // Fall back to extracting from output items
        self.output
            .iter()
            .filter_map(|item| match item {
                crate::types::ResponseItem::Message { content, .. } => Some(
                    content
                        .iter()
                        .map(|c| match c {
                            crate::types::MessageContent::OutputText { text, .. } => text.as_str(),
                        })
                        .collect::<String>(),
                ),
                crate::types::ResponseItem::Text { content, .. } => Some(content.clone()),
                _ => None,
            })
            .collect::<String>()
    }

    /// Returns all tool calls in the response
    #[must_use]
    pub fn tool_calls(&self) -> Vec<crate::types::FunctionCallInfo> {
        self.output
            .iter()
            .filter_map(|item| match item {
                crate::types::ResponseItem::FunctionCall {
                    name,
                    arguments,
                    call_id,
                    ..
                } => Some(crate::types::FunctionCallInfo {
                    name: name.clone(),
                    arguments: arguments.clone(),
                    call_id: call_id.clone(),
                }),
                crate::types::ResponseItem::ToolCall(tool_call) => {
                    Some(crate::types::FunctionCallInfo {
                        name: tool_call.name.clone(),
                        arguments: tool_call.arguments.to_string(),
                        call_id: tool_call.id.clone(),
                    })
                }
                _ => None,
            })
            .collect()
    }

    /// Calculates tool usage counts from the response output
    #[must_use]
    pub fn calculate_tool_usage(&self) -> (u32, u32, u32, u32) {
        let mut web_search_count = 0;
        let mut file_search_count = 0;
        let mut image_generation_count = 0;
        let mut code_interpreter_count = 0;

        for item in &self.output {
            match item {
                crate::types::ResponseItem::WebSearchCall { .. } => web_search_count += 1,
                crate::types::ResponseItem::FileSearchCall { .. } => file_search_count += 1,
                crate::types::ResponseItem::ImageGenerationCall { .. } => {
                    image_generation_count += 1;
                }
                crate::types::ResponseItem::CodeInterpreterCall { .. } => {
                    code_interpreter_count += 1;
                }
                _ => {}
            }
        }

        (
            web_search_count,
            file_search_count,
            image_generation_count,
            code_interpreter_count,
        )
    }

    /// Returns a usage object with token counts and tool usage populated
    #[must_use]
    pub fn usage_with_tools(&self) -> Option<Usage> {
        let (web_search, file_search, image_generation, code_interpreter) =
            self.calculate_tool_usage();

        if let Some(existing_usage) = &self.usage {
            Some(Usage {
                input_tokens: existing_usage.input_tokens,
                output_tokens: existing_usage.output_tokens,
                total_tokens: existing_usage.total_tokens,
                output_tokens_details: existing_usage.output_tokens_details.clone(),
                prompt_tokens_details: existing_usage.prompt_tokens_details.clone(),
                web_search: if web_search > 0 {
                    Some(web_search)
                } else {
                    None
                },
                file_search: if file_search > 0 {
                    Some(file_search)
                } else {
                    None
                },
                image_generation: if image_generation > 0 {
                    Some(image_generation)
                } else {
                    None
                },
                code_interpreter: if code_interpreter > 0 {
                    Some(code_interpreter)
                } else {
                    None
                },
            })
        } else {
            // If no token usage, but we have tool usage, create a minimal usage object
            if web_search > 0 || file_search > 0 || image_generation > 0 || code_interpreter > 0 {
                Some(Usage {
                    input_tokens: 0,
                    output_tokens: 0,
                    total_tokens: 0,
                    output_tokens_details: None,
                    prompt_tokens_details: None,
                    web_search: if web_search > 0 {
                        Some(web_search)
                    } else {
                        None
                    },
                    file_search: if file_search > 0 {
                        Some(file_search)
                    } else {
                        None
                    },
                    image_generation: if image_generation > 0 {
                        Some(image_generation)
                    } else {
                        None
                    },
                    code_interpreter: if code_interpreter > 0 {
                        Some(code_interpreter)
                    } else {
                        None
                    },
                })
            } else {
                None
            }
        }
    }

    /// Returns formatted usage statistics in the requested format
    #[must_use]
    pub fn format_usage(&self) -> String {
        if let Some(usage) = self.usage_with_tools() {
            use std::fmt::Write;
            let mut result = String::new();
            writeln!(result, "input tokens: {}", usage.input_tokens).unwrap();
            writeln!(result, "output tokens: {}", usage.output_tokens).unwrap();
            writeln!(result, "total tokens: {}", usage.total_tokens).unwrap();
            writeln!(result, "web search: {}", usage.web_search.unwrap_or(0)).unwrap();
            writeln!(result, "file search: {}", usage.file_search.unwrap_or(0)).unwrap();
            writeln!(
                result,
                "image generation: {}",
                usage.image_generation.unwrap_or(0)
            )
            .unwrap();
            write!(
                result,
                "code interpreter: {}",
                usage.code_interpreter.unwrap_or(0)
            )
            .unwrap();
            result
        } else {
            "No usage information available".to_string()
        }
    }
}

impl TruncationSetting {
    /// Creates a simple truncation setting with just a string value
    #[must_use]
    pub fn simple(value: impl Into<String>) -> Self {
        Self::Simple(value.into())
    }

    /// Creates a disabled truncation setting
    #[must_use]
    pub fn disabled() -> Self {
        Self::Simple("disabled".to_string())
    }

    /// Creates an auto truncation setting
    #[must_use]
    pub fn auto() -> Self {
        Self::Simple("auto".to_string())
    }

    /// Creates a truncation setting with configuration
    #[must_use]
    pub fn config(truncation_type: impl Into<String>, last_messages: Option<u32>) -> Self {
        Self::Config(TruncationConfig {
            truncation_type: truncation_type.into(),
            last_messages,
        })
    }
}
