use serde::{Deserialize, Serialize};

/// API error response
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ApiError {
    /// Error message
    pub error: ApiErrorDetails,
}

/// API error details
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ApiErrorDetails {
    /// Error message
    pub message: String,

    /// Error type
    #[serde(rename = "type")]
    pub error_type: String,

    /// Error code
    pub code: Option<String>,

    /// Parameter that caused the error
    pub param: Option<String>,
}

/// Error type for the crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// API error
    #[error("API error: {message} (type: {error_type}, code: {code:?})")]
    Api {
        /// Error message
        message: String,

        /// Error type
        error_type: String,

        /// Error code
        code: Option<String>,
    },

    /// Container expired error (special case of API error)
    #[error("Container expired: {message}")]
    ContainerExpired {
        /// Error message
        message: String,

        /// Whether this error was automatically handled
        auto_handled: bool,
    },

    /// HTTP error
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// HTTP status error
    #[error("HTTP status error: {0}")]
    HttpStatus(reqwest::StatusCode),

    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Stream error
    #[error("Stream error: {0}")]
    Stream(String),

    /// Invalid API key
    #[error("Invalid API key format")]
    InvalidApiKey,

    /// API key not found in environment
    #[error("API key not found in environment")]
    ApiKeyNotFound,

    /// Context recovery error
    #[error("Context recovery failed: {0}")]
    ContextRecovery(String),

    /// Maximum retry attempts exceeded
    #[error("Maximum retry attempts exceeded: {attempts}")]
    MaxRetriesExceeded { attempts: u32 },
}

impl Error {
    /// Returns true if this error indicates a container has expired
    #[must_use]
    pub fn is_container_expired(&self) -> bool {
        match self {
            Self::ContainerExpired { .. } => true,
            Self::Api { message, .. } => {
                message.to_lowercase().contains("container is expired")
                    || message.to_lowercase().contains("container expired")
                    || message.to_lowercase().contains("session expired")
            }
            _ => false,
        }
    }

    /// Returns true if this error can be automatically recovered from
    #[must_use]
    pub fn is_recoverable(&self) -> bool {
        self.is_container_expired()
    }

    /// Creates a container expired error
    #[must_use]
    pub fn container_expired(message: impl Into<String>, auto_handled: bool) -> Self {
        Self::ContainerExpired {
            message: message.into(),
            auto_handled,
        }
    }
}

impl From<ApiErrorDetails> for Error {
    fn from(error: ApiErrorDetails) -> Self {
        // Check if this is a container expiration error
        if error
            .message
            .to_lowercase()
            .contains("container is expired")
            || error.message.to_lowercase().contains("container expired")
            || error.message.to_lowercase().contains("session expired")
        {
            return Self::ContainerExpired {
                message: error.message,
                auto_handled: false,
            };
        }

        Self::Api {
            message: error.message,
            error_type: error.error_type,
            code: error.code,
        }
    }
}

/// Result type for the crate
pub type Result<T> = std::result::Result<T, Error>;

/// Helper function to try parsing API errors from responses
pub(crate) async fn try_parse_api_error(response: reqwest::Response) -> Result<reqwest::Response> {
    let status = response.status();
    if status.is_success() {
        return Ok(response);
    }

    // Read body **once**
    let bytes = response.bytes().await.map_err(Error::Http)?;
    if let Ok(api_error) = serde_json::from_slice::<ApiError>(&bytes) {
        return Err(Error::from(api_error.error));
    }

    // Fall back to HTTP status error
    Err(Error::HttpStatus(status))
}
