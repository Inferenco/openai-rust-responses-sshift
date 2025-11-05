use serde::{Deserialize, Serialize};
use std::fmt;

/// High-level classification for errors to drive retry and logging behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorClass {
    ContainerExpired,
    TransientHttp,
    RetryableServer,
    RateLimited,
    ApiContainerExpired,
    NonRecoverable,
}

impl ErrorClass {
    /// Returns a human-friendly label for logging purposes
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ContainerExpired => "container_expired",
            Self::TransientHttp => "transient_http",
            Self::RetryableServer => "retryable_server",
            Self::RateLimited => "rate_limited",
            Self::ApiContainerExpired => "api_container_expired",
            Self::NonRecoverable => "non_recoverable",
        }
    }
}

impl fmt::Display for ErrorClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

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

    /// Server error (500 Internal Server Error)
    #[error("OpenAI server error: {message}")]
    ServerError {
        /// Error message
        message: String,

        /// Request ID for debugging
        request_id: Option<String>,

        /// Whether retry is suggested
        retry_suggested: bool,

        /// User-friendly message
        user_message: String,
    },

    /// Bad Gateway error (502)
    #[error("Service temporarily unavailable (Bad Gateway). Please try again in a moment.")]
    BadGateway {
        /// When to retry (seconds from now)
        retry_after: Option<u64>,

        /// Original status code
        status_code: u16,
    },

    /// Service Unavailable error (503)
    #[error("Service temporarily unavailable. Please try again{retry_message}.")]
    ServiceUnavailable {
        /// When to retry (seconds from now)
        retry_after: Option<u64>,

        /// Formatted retry message
        retry_message: String,
    },

    /// Gateway Timeout error (504)
    #[error("Request timed out at the gateway. Please try again.")]
    GatewayTimeout {
        /// When to retry (seconds from now)
        retry_after: Option<u64>,
    },

    /// Rate limiting error (429)
    #[error("Rate limit exceeded. Please try again{retry_message}.")]
    RateLimited {
        /// When the rate limit resets
        retry_after: Option<u64>,

        /// Formatted retry message
        retry_message: String,

        /// Rate limit type (requests, tokens, etc.)
        limit_type: Option<String>,
    },

    /// Authentication error (401)
    #[error("Authentication failed: {message}")]
    AuthenticationFailed {
        /// Error message
        message: String,

        /// Suggested action for user
        suggestion: String,
    },

    /// Authorization error (403)
    #[error("Access denied: {message}")]
    AuthorizationFailed {
        /// Error message
        message: String,

        /// Suggested action for user
        suggestion: String,
    },

    /// Client error (400, 422)
    #[error("Request error: {message}")]
    ClientError {
        /// Error message
        message: String,

        /// HTTP status code
        status_code: u16,

        /// Field that caused the error (if available)
        field: Option<String>,

        /// Suggested fix
        suggestion: Option<String>,
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

    /// HTTP status error (fallback for unhandled status codes)
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
    /// Returns a classification label for logging and retry policies
    #[must_use]
    pub fn classify(&self) -> ErrorClass {
        match self {
            Self::ContainerExpired { .. } => ErrorClass::ContainerExpired,
            Self::Api { message, .. } if message_indicates_container_expired(message) => {
                ErrorClass::ApiContainerExpired
            }
            Self::BadGateway { .. }
            | Self::ServiceUnavailable { .. }
            | Self::GatewayTimeout { .. }
            | Self::ServerError {
                retry_suggested: true,
                ..
            } => ErrorClass::RetryableServer,
            Self::RateLimited { .. } => ErrorClass::RateLimited,
            Self::Http(reqwest_error)
                if reqwest_error.is_timeout()
                    || reqwest_error.is_connect()
                    || reqwest_error.is_request() =>
            {
                ErrorClass::TransientHttp
            }
            _ => ErrorClass::NonRecoverable,
        }
    }

    /// Returns true if this error indicates a container has expired
    #[must_use]
    pub fn is_container_expired(&self) -> bool {
        matches!(
            self.classify(),
            ErrorClass::ContainerExpired | ErrorClass::ApiContainerExpired
        )
    }

    /// Returns true if this error can be automatically recovered from
    #[must_use]
    pub fn is_recoverable(&self) -> bool {
        match self.classify() {
            ErrorClass::ContainerExpired
            | ErrorClass::RetryableServer
            | ErrorClass::RateLimited
            | ErrorClass::ApiContainerExpired => true,
            ErrorClass::TransientHttp => matches!(
                self,
                Self::Http(reqwest_error)
                    if reqwest_error.is_timeout()
                        || reqwest_error.is_connect()
                        || reqwest_error.is_request()
            ),
            ErrorClass::NonRecoverable => false,
        }
    }

    /// Returns true if this is a transient error that should be retried
    #[must_use]
    pub fn is_transient(&self) -> bool {
        match self.classify() {
            ErrorClass::ContainerExpired
            | ErrorClass::RetryableServer
            | ErrorClass::RateLimited
            | ErrorClass::ApiContainerExpired => true,
            ErrorClass::TransientHttp => matches!(
                self,
                Self::Http(reqwest_error)
                    if reqwest_error.is_timeout() || reqwest_error.is_connect()
            ),
            ErrorClass::NonRecoverable => false,
        }
    }

    /// Returns the suggested retry delay in seconds
    #[must_use]
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            Self::BadGateway { retry_after, .. }
            | Self::ServiceUnavailable { retry_after, .. }
            | Self::GatewayTimeout { retry_after, .. }
            | Self::RateLimited { retry_after, .. } => *retry_after,

            // Default retry delays for other recoverable errors
            Self::ServerError {
                retry_suggested: true,
                ..
            } => Some(5), // 5 seconds
            Self::ContainerExpired { .. } => Some(1), // 1 second
            Self::Http(reqwest_error) if reqwest_error.is_timeout() => Some(10), // 10 seconds
            Self::Http(reqwest_error) if reqwest_error.is_connect() => Some(3), // 3 seconds

            _ => None,
        }
    }

    /// Returns a user-friendly error message
    #[must_use]
    pub fn user_message(&self) -> String {
        match self {
            Self::ServerError { user_message, .. } => user_message.clone(),

            Self::BadGateway { .. } => {
                "The service is temporarily unavailable. Please try again in a moment.".to_string()
            }

            Self::ServiceUnavailable { retry_after, .. } => {
                if let Some(seconds) = retry_after {
                    format!("Service is temporarily unavailable. Please try again in {seconds} seconds.")
                } else {
                    "Service is temporarily unavailable. Please try again shortly.".to_string()
                }
            }

            Self::GatewayTimeout { .. } => "The request timed out. Please try again.".to_string(),

            Self::RateLimited { retry_after, .. } => {
                if let Some(seconds) = retry_after {
                    format!("Rate limit exceeded. Please try again in {seconds} seconds.")
                } else {
                    "Rate limit exceeded. Please try again shortly.".to_string()
                }
            }

            Self::AuthenticationFailed { suggestion, .. }
            | Self::AuthorizationFailed { suggestion, .. }
            | Self::ClientError {
                suggestion: Some(suggestion),
                ..
            } => suggestion.clone(),
            Self::ClientError { message, .. } => format!("Request error: {message}"),

            Self::ContainerExpired { .. } => {
                "Session expired. Retrying with a new session...".to_string()
            }

            Self::InvalidApiKey => "Invalid API key. Please check your API key format.".to_string(),

            Self::ApiKeyNotFound => {
                "API key not found. Please set the OPENAI_API_KEY environment variable.".to_string()
            }

            // For other errors, use the default Display implementation
            _ => self.to_string(),
        }
    }

    /// Creates a container expired error
    #[must_use]
    pub fn container_expired(message: impl Into<String>, auto_handled: bool) -> Self {
        Self::ContainerExpired {
            message: message.into(),
            auto_handled,
        }
    }

    /// Creates a server error with retry suggestion
    #[must_use]
    pub fn server_error(
        message: impl Into<String>,
        request_id: Option<String>,
        retry_suggested: bool,
    ) -> Self {
        let message = message.into();
        let user_message = if retry_suggested {
            "The server encountered an error. Please try again in a moment.".to_string()
        } else {
            format!("Server error: {message}")
        };

        Self::ServerError {
            message,
            request_id,
            retry_suggested,
            user_message,
        }
    }

    /// Creates a bad gateway error
    #[must_use]
    pub fn bad_gateway(retry_after: Option<u64>) -> Self {
        Self::BadGateway {
            retry_after,
            status_code: 502,
        }
    }

    /// Creates a service unavailable error
    #[must_use]
    pub fn service_unavailable(retry_after: Option<u64>) -> Self {
        let retry_message = if let Some(seconds) = retry_after {
            format!(" in {seconds} seconds")
        } else {
            " shortly".to_string()
        };

        Self::ServiceUnavailable {
            retry_after,
            retry_message,
        }
    }

    /// Creates a gateway timeout error
    #[must_use]
    pub fn gateway_timeout(retry_after: Option<u64>) -> Self {
        Self::GatewayTimeout { retry_after }
    }

    /// Creates a rate limited error
    #[must_use]
    pub fn rate_limited(retry_after: Option<u64>, limit_type: Option<String>) -> Self {
        let retry_message = if let Some(seconds) = retry_after {
            format!(" in {seconds} seconds")
        } else {
            " shortly".to_string()
        };

        Self::RateLimited {
            retry_after,
            retry_message,
            limit_type,
        }
    }
}

fn message_indicates_container_expired(message: &str) -> bool {
    let normalized = message.to_ascii_lowercase();
    normalized.contains("container is expired")
        || normalized.contains("container expired")
        || normalized.contains("session expired")
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

/// Helper function to handle specific HTTP status codes
fn handle_http_status_code(status: reqwest::StatusCode, retry_after: Option<u64>) -> Option<Error> {
    match status.as_u16() {
        // Bad Gateway - always transient
        502 => Some(Error::bad_gateway(retry_after.or(Some(30)))), // Default 30s retry

        // Service Unavailable - always transient
        503 => Some(Error::service_unavailable(retry_after.or(Some(60)))), // Default 60s retry

        // Gateway Timeout - always transient
        504 => Some(Error::gateway_timeout(retry_after.or(Some(45)))), // Default 45s retry

        // Rate Limited - always recoverable
        429 => Some(Error::rate_limited(retry_after.or(Some(60)), None)), // Default 60s retry

        _ => None,
    }
}

/// Helper function to handle authentication and authorization errors
fn handle_auth_errors(status: reqwest::StatusCode) -> Option<Error> {
    match status.as_u16() {
        // Authentication errors
        401 => {
            let suggestion = "Please check your API key and ensure it's valid.".to_string();
            Some(Error::AuthenticationFailed {
                message: "Invalid or expired API key".to_string(),
                suggestion,
            })
        }

        // Authorization errors
        403 => {
            let suggestion = "Please check your API key permissions.".to_string();
            Some(Error::AuthorizationFailed {
                message: "Access denied".to_string(),
                suggestion,
            })
        }

        _ => None,
    }
}

/// Helper function to handle client errors (400, 422)
async fn handle_client_errors(response: reqwest::Response) -> Result<Error> {
    let status_code = response.status().as_u16();
    let bytes = response.bytes().await.map_err(Error::Http)?;

    if let Ok(api_error) = serde_json::from_slice::<ApiError>(&bytes) {
        let suggestion = match status_code {
            400 => Some("Please check your request format and parameters.".to_string()),
            422 => Some("Please check your request data for validation errors.".to_string()),
            _ => None,
        };

        return Ok(Error::ClientError {
            message: api_error.error.message,
            status_code,
            field: api_error.error.param,
            suggestion,
        });
    }

    // Fallback for unparseable client errors
    let suggestion = "Please check your request format and try again.".to_string();
    Ok(Error::ClientError {
        message: format!("Client error: {status_code}"),
        status_code,
        field: None,
        suggestion: Some(suggestion),
    })
}

/// Helper function to handle server errors (500-599)
async fn handle_server_errors(
    response: reqwest::Response,
    request_id: Option<String>,
) -> Result<Error> {
    let status = response.status();
    let bytes = response.bytes().await.map_err(Error::Http)?;

    if let Ok(api_error) = serde_json::from_slice::<ApiError>(&bytes) {
        // Determine if this server error is retryable
        let is_retryable = !api_error.error.message.to_lowercase().contains("permanent")
            && !api_error.error.message.to_lowercase().contains("invalid")
            && !api_error.error.message.to_lowercase().contains("malformed");

        // Extract request ID from error message if available
        let extracted_request_id = request_id.or_else(|| {
            // Look for request ID patterns in the message
            if api_error.error.message.contains("request ID") {
                // Extract request ID from patterns like "req_ce5eff5edacde8f5f7a59eb261f53013"
                // Simple string parsing without regex
                if let Some(start) = api_error.error.message.find("req_") {
                    let req_part = &api_error.error.message[start..];
                    if let Some(end) = req_part.find(|c: char| !c.is_alphanumeric() && c != '_') {
                        Some(req_part[..end].to_string())
                    } else {
                        // Take first word if no delimiter found
                        req_part
                            .split_whitespace()
                            .next()
                            .map(std::string::ToString::to_string)
                    }
                } else {
                    None
                }
            } else {
                None
            }
        });

        return Ok(Error::server_error(
            api_error.error.message,
            extracted_request_id,
            is_retryable,
        ));
    }

    // Fallback for unparseable server errors
    Ok(Error::server_error(
        format!("Server error: {status}"),
        request_id,
        true, // Assume server errors are retryable by default
    ))
}

/// Helper function to try parsing API errors from responses
pub(crate) async fn try_parse_api_error(response: reqwest::Response) -> Result<reqwest::Response> {
    let status = response.status();
    if status.is_success() {
        return Ok(response);
    }

    // Extract useful headers before consuming the response
    let retry_after = response
        .headers()
        .get("retry-after")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok());

    let request_id = response
        .headers()
        .get("x-request-id")
        .or_else(|| response.headers().get("request-id"))
        .and_then(|h| h.to_str().ok())
        .map(std::string::ToString::to_string);

    // Handle specific HTTP status codes first
    if let Some(error) = handle_http_status_code(status, retry_after) {
        return Err(error);
    }

    // Handle authentication and authorization errors
    if let Some(error) = handle_auth_errors(status) {
        return Err(error);
    }

    // Handle client errors (400, 422)
    if matches!(status.as_u16(), 400 | 422) {
        return Err(handle_client_errors(response).await?);
    }

    // Handle server errors (500-599, except those handled above)
    if status.is_server_error() {
        return Err(handle_server_errors(response, request_id).await?);
    }

    // Try to parse as structured API error for remaining status codes
    let bytes = response.bytes().await.map_err(Error::Http)?;
    if let Ok(api_error) = serde_json::from_slice::<ApiError>(&bytes) {
        return Err(Error::from(api_error.error));
    }

    // Final fallback to generic HTTP status error
    Err(Error::HttpStatus(status))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_new_error_types() {
        // Test BadGateway error
        let bad_gateway = Error::bad_gateway(Some(30));
        assert!(bad_gateway.is_recoverable());
        assert!(bad_gateway.is_transient());
        assert_eq!(bad_gateway.retry_after(), Some(30));
        assert_eq!(
            bad_gateway.user_message(),
            "The service is temporarily unavailable. Please try again in a moment."
        );

        // Test ServerError
        let server_error =
            Error::server_error("Internal server error", Some("req_123".to_string()), true);
        assert!(server_error.is_recoverable());
        assert!(server_error.is_transient());
        assert_eq!(server_error.retry_after(), Some(5));

        // Test non-retryable server error
        let non_retryable = Error::server_error("Fatal error", None, false);
        assert!(!non_retryable.is_recoverable());
        assert!(!non_retryable.is_transient());

        // Test ServiceUnavailable
        let service_unavailable = Error::service_unavailable(Some(60));
        assert!(service_unavailable.is_recoverable());
        assert!(service_unavailable.is_transient());
        assert_eq!(service_unavailable.retry_after(), Some(60));

        // Test RateLimited
        let rate_limited = Error::rate_limited(Some(120), Some("requests".to_string()));
        assert!(rate_limited.is_recoverable());
        assert!(rate_limited.is_transient());
        assert_eq!(rate_limited.retry_after(), Some(120));

        // Test ContainerExpired (existing functionality)
        let container_expired = Error::container_expired("Container expired", false);
        assert!(container_expired.is_recoverable());
        assert!(container_expired.is_container_expired());
        assert_eq!(container_expired.retry_after(), Some(1));
    }

    #[test]
    fn test_user_messages() {
        let bad_gateway = Error::bad_gateway(None);
        assert_eq!(
            bad_gateway.user_message(),
            "The service is temporarily unavailable. Please try again in a moment."
        );

        let service_unavailable = Error::service_unavailable(Some(30));
        assert_eq!(
            service_unavailable.user_message(),
            "Service is temporarily unavailable. Please try again in 30 seconds."
        );

        let rate_limited = Error::rate_limited(Some(60), None);
        assert_eq!(
            rate_limited.user_message(),
            "Rate limit exceeded. Please try again in 60 seconds."
        );

        let auth_failed = Error::AuthenticationFailed {
            message: "Invalid key".to_string(),
            suggestion: "Check your API key".to_string(),
        };
        assert_eq!(auth_failed.user_message(), "Check your API key");
    }

    #[test]
    fn test_error_classification() {
        // Test that new transient errors are properly classified
        let transient_errors = vec![
            Error::bad_gateway(None),
            Error::service_unavailable(None),
            Error::gateway_timeout(None),
            Error::rate_limited(None, None),
            Error::server_error("test", None, true),
        ];

        for error in transient_errors {
            assert!(error.is_transient(), "Error should be transient: {error:?}");
            assert!(
                error.is_recoverable(),
                "Error should be recoverable: {error:?}"
            );
        }

        // Test non-transient errors
        let non_transient_errors = vec![
            Error::AuthenticationFailed {
                message: "test".to_string(),
                suggestion: "test".to_string(),
            },
            Error::ClientError {
                message: "test".to_string(),
                status_code: 400,
                field: None,
                suggestion: None,
            },
            Error::InvalidApiKey,
        ];

        for error in non_transient_errors {
            assert!(
                !error.is_transient(),
                "Error should not be transient: {error:?}"
            );
            assert!(
                !error.is_recoverable(),
                "Error should not be recoverable: {error:?}"
            );
        }
    }

    #[test]
    fn test_comprehensive_error_handling() {
        // Test all new error types
        test_bad_gateway_error();
        test_service_unavailable_error();
        test_gateway_timeout_error();
        test_rate_limited_error();
        test_server_error();
        test_authentication_failed_error();
        test_authorization_failed_error();
        test_client_error();
    }

    fn test_bad_gateway_error() {
        let error = Error::bad_gateway(Some(30));
        assert!(error.is_transient());
        assert!(error.is_recoverable());
        assert_eq!(error.retry_after(), Some(30));
        assert!(error.user_message().contains("temporarily unavailable"));
    }

    fn test_service_unavailable_error() {
        let error = Error::service_unavailable(Some(60));
        assert!(error.is_transient());
        assert!(error.is_recoverable());
        assert_eq!(error.retry_after(), Some(60));
        assert!(error.user_message().contains("unavailable"));
    }

    fn test_gateway_timeout_error() {
        let error = Error::gateway_timeout(Some(45));
        assert!(error.is_transient());
        assert!(error.is_recoverable());
        assert_eq!(error.retry_after(), Some(45));
        assert!(error.user_message().contains("timed out"));
    }

    fn test_rate_limited_error() {
        let error = Error::rate_limited(
            Some(120),
            Some("You have exceeded your rate limit".to_string()),
        );
        assert!(error.is_transient()); // Rate limiting is transient - it will reset after time
        assert!(error.is_recoverable());
        assert_eq!(error.retry_after(), Some(120));
        assert!(error.user_message().contains("Rate limit"));
    }

    fn test_server_error() {
        let error = Error::server_error(
            "Internal server error".to_string(),
            Some("req_123".to_string()),
            true,
        );
        assert!(error.is_transient());
        assert!(error.is_recoverable());
        assert_eq!(error.retry_after(), Some(5)); // Default retry for server errors
        assert!(error.user_message().contains("server encountered an error"));
    }

    fn test_authentication_failed_error() {
        let error = Error::AuthenticationFailed {
            message: "Invalid API key".to_string(),
            suggestion: "Check your API key".to_string(),
        };
        assert!(!error.is_transient());
        assert!(!error.is_recoverable());
        assert_eq!(error.retry_after(), None);
        assert!(error.user_message().contains("Check your API key"));
    }

    fn test_authorization_failed_error() {
        let error = Error::AuthorizationFailed {
            message: "Access denied".to_string(),
            suggestion: "Check permissions".to_string(),
        };
        assert!(!error.is_transient());
        assert!(!error.is_recoverable());
        assert_eq!(error.retry_after(), None);
        assert!(error.user_message().contains("Check permissions"));
    }

    fn test_client_error() {
        let error = Error::ClientError {
            message: "Bad request".to_string(),
            status_code: 400,
            field: Some("prompt".to_string()),
            suggestion: Some("Check your request".to_string()),
        };
        assert!(!error.is_transient());
        assert!(!error.is_recoverable());
        assert_eq!(error.retry_after(), None);
        assert!(error.user_message().contains("Check your request"));
    }

    #[test]
    fn test_error_helper_methods() {
        // Test user message generation for different error types
        let errors_and_expected_messages = vec![
            (
                Error::bad_gateway(None),
                "The service is temporarily unavailable. Please try again in a moment.",
            ),
            (
                Error::service_unavailable(Some(60)),
                "Service is temporarily unavailable. Please try again in 60 seconds.",
            ),
            (
                Error::gateway_timeout(Some(45)),
                "The request timed out. Please try again.",
            ),
            (
                Error::rate_limited(Some(120), None),
                "Rate limit exceeded. Please try again in 120 seconds.",
            ),
            (
                Error::InvalidApiKey,
                "Invalid API key. Please check your API key format.",
            ),
            (
                Error::ApiKeyNotFound,
                "API key not found. Please set the OPENAI_API_KEY environment variable.",
            ),
        ];

        for (error, expected_message) in errors_and_expected_messages {
            assert_eq!(
                error.user_message(),
                expected_message,
                "User message mismatch for error: {error:?}"
            );
        }
    }

    #[test]
    fn test_error_factory_methods() {
        // Test all error factory methods work correctly

        let bad_gateway = Error::bad_gateway(Some(30));
        if let Error::BadGateway {
            retry_after,
            status_code,
        } = bad_gateway
        {
            assert_eq!(retry_after, Some(30));
            assert_eq!(status_code, 502);
        } else {
            panic!("Expected BadGateway error");
        }

        let service_unavailable = Error::service_unavailable(Some(60));
        if let Error::ServiceUnavailable {
            retry_after,
            retry_message,
        } = service_unavailable
        {
            assert_eq!(retry_after, Some(60));
            assert_eq!(retry_message, " in 60 seconds");
        } else {
            panic!("Expected ServiceUnavailable error");
        }

        let rate_limited = Error::rate_limited(Some(120), Some("tokens".to_string()));
        if let Error::RateLimited {
            retry_after,
            limit_type,
            ..
        } = rate_limited
        {
            assert_eq!(retry_after, Some(120));
            assert_eq!(limit_type, Some("tokens".to_string()));
        } else {
            panic!("Expected RateLimited error");
        }

        let server_error = Error::server_error("Test error", Some("req_123".to_string()), true);
        if let Error::ServerError {
            message,
            request_id,
            retry_suggested,
            ..
        } = server_error
        {
            assert_eq!(message, "Test error");
            assert_eq!(request_id, Some("req_123".to_string()));
            assert!(retry_suggested);
        } else {
            panic!("Expected ServerError");
        }
    }

    #[test]
    fn classify_error_classes() {
        let container = Error::container_expired("Session expired", false);
        assert_eq!(container.classify(), ErrorClass::ContainerExpired);
        assert!(container.is_recoverable());
        assert!(container.is_transient());

        let api_container = Error::Api {
            message: "Your container expired in the middle of processing".to_string(),
            error_type: "api_error".to_string(),
            code: None,
        };
        assert_eq!(api_container.classify(), ErrorClass::ApiContainerExpired);
        assert!(api_container.is_recoverable());
        assert!(api_container.is_transient());

        let retryable_server = Error::server_error("Server hiccup", None, true);
        assert_eq!(retryable_server.classify(), ErrorClass::RetryableServer);

        let rate_limited = Error::rate_limited(Some(1), None);
        assert_eq!(rate_limited.classify(), ErrorClass::RateLimited);

        let runtime = tokio::runtime::Runtime::new().expect("runtime");

        let timeout_http = runtime.block_on(async {
            reqwest::Client::builder()
                .timeout(Duration::from_millis(1))
                .build()
                .unwrap()
                .get("http://10.255.255.1")
                .send()
                .await
                .unwrap_err()
        });
        assert!(timeout_http.is_timeout());
        let timeout_error = Error::Http(timeout_http);
        assert_eq!(timeout_error.classify(), ErrorClass::TransientHttp);
        assert!(timeout_error.is_recoverable());
        assert!(timeout_error.is_transient());

        let connect_http = runtime.block_on(async {
            reqwest::Client::new()
                .get("http://127.0.0.1:1")
                .send()
                .await
                .unwrap_err()
        });
        assert!(connect_http.is_connect());
        let connect_error = Error::Http(connect_http);
        assert_eq!(connect_error.classify(), ErrorClass::TransientHttp);
        assert!(connect_error.is_recoverable());
        assert!(connect_error.is_transient());

        let request_http = runtime.block_on(async {
            reqwest::Client::builder()
                .timeout(Duration::from_millis(50))
                .build()
                .unwrap()
                .get("http://127.0.0.1:9")
                .send()
                .await
                .unwrap_err()
        });
        assert!(request_http.is_request());
        let request_error = Error::Http(request_http);
        assert_eq!(request_error.classify(), ErrorClass::TransientHttp);
        assert!(request_error.is_recoverable());

        drop(runtime);

        let hard_failure = Error::InvalidApiKey;
        assert_eq!(hard_failure.classify(), ErrorClass::NonRecoverable);
        assert!(!hard_failure.is_recoverable());
        assert!(!hard_failure.is_transient());
    }
}
