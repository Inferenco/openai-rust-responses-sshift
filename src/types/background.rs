use serde::{Deserialize, Serialize};

/// Status of a background processing operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BackgroundStatus {
    /// Background task is queued but not started
    Queued,
    /// Background task is currently running
    Running,
    /// Background task completed successfully
    Completed,
    /// Background task failed with an error
    Failed,
    /// Background task was cancelled
    Cancelled,
}

/// Handle for background processing operations
///
/// When a request is submitted with `background: true` and returns HTTP 202,
/// this handle allows you to poll for status or stream results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundHandle {
    /// Unique identifier for the background operation
    pub id: String,

    /// URL to poll for status updates
    pub status_url: String,

    /// URL to stream results (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_url: Option<String>,

    /// Current status of the operation
    pub status: BackgroundStatus,

    /// Estimated completion time (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_completion: Option<String>,

    /// Error message (if status is Failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Response from polling a background operation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundStatusResponse {
    /// Unique identifier for the background operation
    pub id: String,

    /// Current status of the operation
    pub status: BackgroundStatus,

    /// Progress percentage (0-100, if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<u8>,

    /// Estimated completion time (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_completion: Option<String>,

    /// Error message (if status is Failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Result data (if status is Completed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

impl BackgroundHandle {
    /// Create a new background handle
    #[must_use]
    pub fn new(id: String, status_url: String) -> Self {
        Self {
            id,
            status_url,
            stream_url: None,
            status: BackgroundStatus::Queued,
            estimated_completion: None,
            error: None,
        }
    }

    /// Set the stream URL for this handle
    #[must_use]
    pub fn with_stream_url(mut self, stream_url: String) -> Self {
        self.stream_url = Some(stream_url);
        self
    }

    /// Set the estimated completion time
    #[must_use]
    pub fn with_estimated_completion(mut self, estimated_completion: String) -> Self {
        self.estimated_completion = Some(estimated_completion);
        self
    }

    /// Check if the operation is still in progress
    #[must_use]
    pub fn is_running(&self) -> bool {
        matches!(
            self.status,
            BackgroundStatus::Queued | BackgroundStatus::Running
        )
    }

    /// Check if the operation completed successfully
    #[must_use]
    pub fn is_completed(&self) -> bool {
        self.status == BackgroundStatus::Completed
    }

    /// Check if the operation failed
    #[must_use]
    pub fn is_failed(&self) -> bool {
        self.status == BackgroundStatus::Failed
    }

    /// Check if the operation was cancelled
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.status == BackgroundStatus::Cancelled
    }

    /// Check if the operation is done (completed, failed, or cancelled)
    #[must_use]
    pub fn is_done(&self) -> bool {
        !self.is_running()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_background_handle_creation() {
        let handle = BackgroundHandle::new(
            "bg_123".to_string(),
            "https://api.openai.com/v1/backgrounds/bg_123/status".to_string(),
        );

        assert_eq!(handle.id, "bg_123");
        assert_eq!(handle.status, BackgroundStatus::Queued);
        assert!(handle.is_running());
        assert!(!handle.is_done());
    }

    #[test]
    fn test_background_status_checks() {
        let mut handle = BackgroundHandle::new("bg_123".to_string(), "status_url".to_string());

        // Initially queued
        assert!(handle.is_running());
        assert!(!handle.is_done());

        // Mark as completed
        handle.status = BackgroundStatus::Completed;
        assert!(!handle.is_running());
        assert!(handle.is_completed());
        assert!(handle.is_done());

        // Mark as failed
        handle.status = BackgroundStatus::Failed;
        assert!(!handle.is_running());
        assert!(handle.is_failed());
        assert!(handle.is_done());
    }

    #[test]
    fn test_background_status_serialization() {
        let status = BackgroundStatus::Running;
        assert_eq!(serde_json::to_string(&status).unwrap(), r#""running""#);

        let status = BackgroundStatus::Completed;
        assert_eq!(serde_json::to_string(&status).unwrap(), r#""completed""#);
    }
}
