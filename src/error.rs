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
}

impl From<ApiErrorDetails> for Error {
    fn from(error: ApiErrorDetails) -> Self {
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
