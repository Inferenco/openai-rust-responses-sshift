use serde::{Deserialize, Serialize};

/// Additional fields that can be included in the response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Include {
    /// Include file search results in the response
    #[serde(rename = "file_search_call.results")]
    FileSearchResults,

    /// Include web search results in the response
    #[serde(rename = "web_search_call.results")]
    WebSearchResults,

    /// Include message input image URLs in the response
    #[serde(rename = "message.input_image.image_url")]
    MessageInputImageUrl,

    /// Include computer call output image URLs in the response
    #[serde(rename = "computer_call_output.output.image_url")]
    ComputerCallOutputImageUrl,

    /// Include encrypted reasoning content in the response (May 2025)
    /// Note: reasoning.summary is not yet supported by the API
    #[serde(rename = "reasoning.encrypted_content")]
    ReasoningEncryptedContent,
}

impl Include {
    /// Converts the include variant to its string representation
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FileSearchResults => "file_search_call.results",
            Self::WebSearchResults => "web_search_call.results",
            Self::MessageInputImageUrl => "message.input_image.image_url",
            Self::ComputerCallOutputImageUrl => "computer_call_output.output.image_url",
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

    /// The maximum number of tokens to generate (alias for max_output_tokens)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// The maximum number of output tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,

    /// Sampling temperature between 0 and 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Nucleus sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Number of top log probabilities to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,

    /// Whether to stream the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Tools that the model may call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<crate::types::Tool>>,

    /// Controls which (if any) tool is called by the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<crate::types::ToolChoice>,

    /// Whether tools can be called in parallel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

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

    /// Whether to store the conversation state (default: true)
    /// Set to false for stateless requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,

    /// Truncation configuration for automatic context management
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<crate::types::TruncationSetting>,

    /// Text generation configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<crate::types::TextConfig>,

    /// User identifier for tracking and abuse prevention
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            model: crate::types::Model::GPT4o,
            input: crate::types::Input::Text(String::new()),
            instructions: None,
            max_tokens: None,
            max_output_tokens: None,
            temperature: None,
            top_p: None,
            top_logprobs: None,
            stream: None,
            tools: None,
            tool_choice: None,
            parallel_tool_calls: None,
            previous_response_id: None,
            metadata: None,
            include: None,
            reasoning: None,
            background: None,
            store: None,
            truncation: None,
            text: None,
            user: None,
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

    /// Sets the maximum number of tokens to generate (legacy parameter)
    #[must_use]
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.request.max_tokens = Some(max_tokens);
        self
    }

    /// Sets the maximum number of output tokens to generate
    #[must_use]
    pub fn max_output_tokens(mut self, max_output_tokens: u32) -> Self {
        self.request.max_output_tokens = Some(max_output_tokens);
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

    /// Sets the number of top log probabilities to return
    #[must_use]
    pub fn top_logprobs(mut self, top_logprobs: u32) -> Self {
        self.request.top_logprobs = Some(top_logprobs);
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

    /// Sets whether tools can be called in parallel
    #[must_use]
    pub fn parallel_tool_calls(mut self, parallel: bool) -> Self {
        self.request.parallel_tool_calls = Some(parallel);
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
                // Current API values
                "web_search_call.results" => Some(Include::WebSearchResults),
                "message.input_image.image_url" => Some(Include::MessageInputImageUrl),
                "computer_call_output.output.image_url" => {
                    Some(Include::ComputerCallOutputImageUrl)
                }
                "reasoning.encrypted_content" => Some(Include::ReasoningEncryptedContent),
                // Legacy and current values for file search results
                "file_search.results" | "file_search_call.results" => {
                    Some(Include::FileSearchResults)
                }
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

    /// Sets whether to store conversation state (default: true)
    /// Set to false for stateless requests
    #[must_use]
    pub fn store(mut self, store: bool) -> Self {
        self.request.store = Some(store);
        self
    }

    /// Sets truncation configuration for automatic context management
    #[must_use]
    pub fn truncation(mut self, truncation: crate::types::TruncationSetting) -> Self {
        self.request.truncation = Some(truncation);
        self
    }

    /// Sets text generation configuration
    #[must_use]
    pub fn text(mut self, text: crate::types::TextConfig) -> Self {
        self.request.text = Some(text);
        self
    }

    /// Sets user identifier
    #[must_use]
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.request.user = Some(user.into());
        self
    }

    /// Creates a request to continue a conversation with function call outputs
    /// This is the correct way to submit tool results in the Responses API
    #[must_use]
    pub fn with_function_outputs(
        mut self,
        previous_response_id: impl Into<String>,
        function_outputs: Vec<(String, String)>, // (call_id, output) pairs
    ) -> Self {
        self.request.previous_response_id = Some(previous_response_id.into());

        let input_items: Vec<crate::types::InputItem> = function_outputs
            .into_iter()
            .map(|(call_id, output)| crate::types::InputItem::function_call_output(call_id, output))
            .collect();

        self.request.input = crate::types::Input::Items(input_items);
        self
    }

    /// Sets the input as a single image URL in a user message
    #[must_use]
    pub fn input_image_url(mut self, url: impl Into<String>) -> Self {
        let message = crate::types::InputItem::message(
            "user",
            vec![crate::types::InputItem::content_image(url)],
        );
        self.request.input = crate::types::Input::Items(vec![message]);
        self
    }

    /// Sets the input as a single image URL with detail level in a user message
    #[must_use]
    pub fn input_image_url_with_detail(
        mut self,
        url: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        let message = crate::types::InputItem::message(
            "user",
            vec![crate::types::InputItem::content_image_with_detail(
                url, detail,
            )],
        );
        self.request.input = crate::types::Input::Items(vec![message]);
        self
    }

    /// Sets the input as a single base64 image in a user message
    #[must_use]
    pub fn input_image_base64(
        mut self,
        base64_data: impl Into<String>,
        mime_type: impl Into<String>,
    ) -> Self {
        let message = crate::types::InputItem::message(
            "user",
            vec![crate::types::InputItem::content_image_base64(
                base64_data,
                mime_type,
            )],
        );
        self.request.input = crate::types::Input::Items(vec![message]);
        self
    }

    /// Sets the input as a single base64 image with detail level in a user message
    #[must_use]
    pub fn input_image_base64_with_detail(
        mut self,
        base64_data: impl Into<String>,
        mime_type: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        let message = crate::types::InputItem::message(
            "user",
            vec![crate::types::InputItem::content_image_base64_with_detail(
                base64_data,
                mime_type,
                detail,
            )],
        );
        self.request.input = crate::types::Input::Items(vec![message]);
        self
    }

    /// Sets the input as a single file ID image in a user message
    #[must_use]
    pub fn input_image_file_id(mut self, file_id: impl Into<String>) -> Self {
        let message = crate::types::InputItem::message(
            "user",
            vec![crate::types::InputItem::content_image_file_id(file_id)],
        );
        self.request.input = crate::types::Input::Items(vec![message]);
        self
    }

    /// Sets the input as a single file ID image with detail level in a user message
    #[must_use]
    pub fn input_image_file_id_with_detail(
        mut self,
        file_id: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        let message = crate::types::InputItem::message(
            "user",
            vec![crate::types::InputItem::content_image_file_id_with_detail(
                file_id, detail,
            )],
        );
        self.request.input = crate::types::Input::Items(vec![message]);
        self
    }

    /// Sets the input as multiple image URLs in a single user message
    #[must_use]
    pub fn input_image_urls<I, S>(mut self, urls: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let content: Vec<serde_json::Value> = urls
            .into_iter()
            .map(|u| crate::types::InputItem::content_image(u))
            .collect();
        let message = crate::types::InputItem::message("user", content);
        self.request.input = crate::types::Input::Items(vec![message]);
        self
    }

    /// Appends a single image URL to the current user message. If no message
    /// exists yet it behaves like `input_image_url`.
    #[must_use]
    pub fn push_image_url(mut self, url: impl Into<String>) -> Self {
        match &mut self.request.input {
            crate::types::Input::Items(items)
                if !items.is_empty() && items[0].item_type == "message" =>
            {
                if let Some(serde_json::Value::Array(content)) = items[0].content.as_mut() {
                    content.push(crate::types::InputItem::content_image(url));
                } else {
                    // Fallback: rebuild the message content correctly
                    let message = crate::types::InputItem::message(
                        "user",
                        vec![crate::types::InputItem::content_image(url)],
                    );
                    *items = vec![message];
                }
            }
            _ => {
                // No existing message – create one
                let message = crate::types::InputItem::message(
                    "user",
                    vec![crate::types::InputItem::content_image(url)],
                );
                self.request.input = crate::types::Input::Items(vec![message]);
            }
        }
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
