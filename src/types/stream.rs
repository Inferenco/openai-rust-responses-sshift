#[cfg(feature = "stream")]
use futures::Stream;
use serde::{Deserialize, Serialize};

/// Stream event types for the OpenAI Responses API
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StreamEvent {
    /// Text delta event
    TextDelta {
        /// Content of the text delta
        content: String,
        /// Index of the text delta
        index: u32,
    },

    /// Text stop event
    TextStop {
        /// Index of the text stop
        index: u32,
    },

    /// Tool call created event
    ToolCallCreated {
        /// Tool call ID
        id: String,
        /// Tool call name
        name: String,
        /// Index of the tool call
        index: u32,
    },

    /// Tool call delta event
    ToolCallDelta {
        /// Tool call ID
        id: String,
        /// Delta content
        content: String,
        /// Index of the tool call
        index: u32,
    },

    /// Tool call completed event
    ToolCallCompleted {
        /// Tool call ID
        id: String,
        /// Index of the tool call
        index: u32,
    },

    /// Image generation progress event (NEW for May 2025)
    ImageProgress {
        /// URL of the progressive image (if available)
        url: Option<String>,
        /// Index of the image being generated
        index: u32,
    },

    /// Chunk heartbeat event
    Chunk,

    /// Done event
    Done,

    /// Unknown event type (catch-all for future event types)
    #[serde(other)]
    Unknown,
}

impl StreamEvent {
    /// Returns text delta if this is a text event
    #[must_use]
    pub fn as_text_delta(&self) -> Option<&str> {
        match self {
            Self::TextDelta { content, .. } => Some(content),
            _ => None,
        }
    }

    /// Returns tool call delta if this is a tool call event
    #[must_use]
    pub fn as_tool_call_delta(&self) -> Option<&str> {
        match self {
            Self::ToolCallDelta { content, .. } => Some(content),
            _ => None,
        }
    }

    /// Returns image progress URL if this is an image progress event
    #[must_use]
    pub fn as_image_progress(&self) -> Option<&str> {
        match self {
            Self::ImageProgress { url: Some(url), .. } => Some(url),
            _ => None,
        }
    }

    /// Returns true if this is a done event
    #[must_use]
    pub fn is_done(&self) -> bool {
        matches!(self, Self::Done)
    }
}

/// Stream of events from the OpenAI Responses API
#[cfg(feature = "stream")]
pub type EventStream = dyn Stream<Item = crate::Result<StreamEvent>> + Send + Unpin;
