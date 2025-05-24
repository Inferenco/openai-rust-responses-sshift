use serde::{Deserialize, Serialize};

/// Additional fields that can be included in the response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Include {
    /// Include file search results in the response
    #[serde(rename = "file_search.results")]
    FileSearchResults,

    /// Include reasoning summary in the response (NEW for May 2025)
    #[serde(rename = "reasoning.summary")]
    ReasoningSummary,

    /// Include encrypted reasoning content in the response (NEW for May 2025)
    #[serde(rename = "reasoning.encrypted_content")]
    ReasoningEncryptedContent,
}

impl Include {
    /// Converts the include variant to its string representation
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FileSearchResults => "file_search.results",
            Self::ReasoningSummary => "reasoning.summary",
            Self::ReasoningEncryptedContent => "reasoning.encrypted_content",
        }
    }
}

impl std::fmt::Display for Include {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Request for creating a response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    /// The model to use for generating the response
    pub model: crate::types::Model,

    /// The input to generate a response for
    pub input: crate::types::Input,

    /// System instructions that guide the model's behavior
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// The maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// Sampling temperature between 0 and 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Nucleus sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Whether to stream the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Tools that the model may call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<crate::types::Tool>>,

    /// Controls which (if any) tool is called by the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<crate::types::ToolChoice>,

    /// ID of a previous response to continue from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,

    /// Additional metadata to include in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    /// Additional fields to include in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<Include>>,

    /// Reasoning parameters for controlling reasoning model behavior (NEW: May 2025)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<crate::types::ReasoningParams>,

    /// Enable background processing mode (NEW: May 2025)
    /// When true, returns HTTP 202 with BackgroundHandle for long-running tasks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            model: crate::types::Model::GPT4o,
            input: crate::types::Input::Text(String::new()),
            instructions: None,
            max_tokens: None,
            temperature: None,
            top_p: None,
            stream: None,
            tools: None,
            tool_choice: None,
            previous_response_id: None,
            metadata: None,
            include: None,
            reasoning: None,
            background: None,
        }
    }
}

/// Builder for creating requests
#[derive(Debug, Clone)]
pub struct RequestBuilder {
    request: Request,
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestBuilder {
    /// Creates a new request builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            request: Request::default(),
        }
    }

    /// Sets the model to use
    #[must_use]
    pub fn model(mut self, model: impl Into<crate::types::Model>) -> Self {
        self.request.model = model.into();
        self
    }

    /// Sets the input text
    #[must_use]
    pub fn input(mut self, input: impl Into<String>) -> Self {
        self.request.input = crate::types::Input::Text(input.into());
        self
    }

    /// Sets the input items
    #[must_use]
    pub fn input_items(mut self, items: Vec<crate::types::InputItem>) -> Self {
        self.request.input = crate::types::Input::Items(items);
        self
    }

    /// Sets the system instructions
    #[must_use]
    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.request.instructions = Some(instructions.into());
        self
    }

    /// Sets the maximum number of tokens to generate
    #[must_use]
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.request.max_tokens = Some(max_tokens);
        self
    }

    /// Sets the sampling temperature
    #[must_use]
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.request.temperature = Some(temperature);
        self
    }

    /// Sets the nucleus sampling parameter
    #[must_use]
    pub fn top_p(mut self, top_p: f32) -> Self {
        self.request.top_p = Some(top_p);
        self
    }

    /// Sets whether to stream the response
    #[must_use]
    pub fn stream(mut self, stream: bool) -> Self {
        self.request.stream = Some(stream);
        self
    }

    /// Sets the tools that the model may call
    #[must_use]
    pub fn tools(mut self, tools: Vec<crate::types::Tool>) -> Self {
        self.request.tools = Some(tools);
        self
    }

    /// Sets which tool is called by the model
    #[must_use]
    pub fn tool_choice(mut self, tool_choice: crate::types::ToolChoice) -> Self {
        self.request.tool_choice = Some(tool_choice);
        self
    }

    /// Sets the ID of a previous response to continue from
    #[must_use]
    pub fn previous_response_id(mut self, id: impl Into<String>) -> Self {
        self.request.previous_response_id = Some(id.into());
        self
    }

    /// Sets additional metadata to include in the response
    #[must_use]
    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.request.metadata = Some(metadata);
        self
    }

    /// Sets additional fields to include in the response
    #[must_use]
    pub fn include(mut self, include: Vec<Include>) -> Self {
        self.request.include = Some(include);
        self
    }

    /// Converts string includes to typed includes for backward compatibility
    #[must_use]
    pub fn include_strings(mut self, include: Vec<String>) -> Self {
        let typed_includes: Vec<Include> = include
            .into_iter()
            .filter_map(|s| match s.as_str() {
                "file_search.results" => Some(Include::FileSearchResults),
                "reasoning.summary" => Some(Include::ReasoningSummary),
                "reasoning.encrypted_content" => Some(Include::ReasoningEncryptedContent),
                _ => None, // Skip unknown includes
            })
            .collect();
        self.request.include = Some(typed_includes);
        self
    }

    /// Sets reasoning parameters for controlling reasoning model behavior (NEW: May 2025)
    #[must_use]
    pub fn reasoning(mut self, reasoning: crate::types::ReasoningParams) -> Self {
        self.request.reasoning = Some(reasoning);
        self
    }

    /// Enable background processing mode (NEW: May 2025)
    /// When true, returns HTTP 202 with BackgroundHandle for long-running tasks
    #[must_use]
    pub fn background(mut self, background: bool) -> Self {
        self.request.background = Some(background);
        self
    }

    /// Builds the request
    #[must_use]
    pub fn build(self) -> Request {
        self.request
    }
}

impl Request {
    /// Creates a new request builder
    #[must_use]
    pub fn builder() -> RequestBuilder {
        RequestBuilder::new()
    }
}
