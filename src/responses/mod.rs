use crate::error::{try_parse_api_error, Result};
use crate::types::{RecoveryCallback, RecoveryPolicy};
use reqwest::Client as HttpClient;
use std::sync::Arc;

/// Recovery result information
#[derive(Debug, Clone)]
pub struct RecoveryInfo {
    /// Whether recovery was attempted
    pub attempted: bool,

    /// Number of retry attempts made
    pub retry_count: u32,

    /// Whether the recovery was successful
    pub successful: bool,

    /// User-friendly message about the recovery
    pub message: Option<String>,

    /// Original error that triggered recovery
    pub original_error: Option<String>,
}

impl RecoveryInfo {
    /// Creates a new recovery info with no recovery attempted
    #[must_use]
    pub fn none() -> Self {
        Self {
            attempted: false,
            retry_count: 0,
            successful: false,
            message: None,
            original_error: None,
        }
    }

    /// Creates a new recovery info for a successful recovery
    #[must_use]
    pub fn success(
        retry_count: u32,
        message: Option<String>,
        original_error: Option<String>,
    ) -> Self {
        Self {
            attempted: true,
            retry_count,
            successful: true,
            message,
            original_error,
        }
    }

    /// Creates a new recovery info for a failed recovery
    #[must_use]
    pub fn failure(retry_count: u32, original_error: Option<String>) -> Self {
        Self {
            attempted: true,
            retry_count,
            successful: false,
            message: None,
            original_error,
        }
    }
}

/// Enhanced response with recovery information
#[derive(Debug, Clone)]
pub struct ResponseWithRecovery {
    /// The actual response from the API
    pub response: crate::Response,

    /// Information about any recovery that was performed
    pub recovery_info: RecoveryInfo,
}

impl ResponseWithRecovery {
    /// Creates a new response with no recovery
    #[must_use]
    pub fn new(response: crate::Response) -> Self {
        Self {
            response,
            recovery_info: RecoveryInfo::none(),
        }
    }

    /// Creates a new response with recovery information
    #[must_use]
    pub fn with_recovery(response: crate::Response, recovery_info: RecoveryInfo) -> Self {
        Self {
            response,
            recovery_info,
        }
    }

    /// Returns true if recovery was attempted
    #[must_use]
    pub fn had_recovery(&self) -> bool {
        self.recovery_info.attempted
    }

    /// Returns true if recovery was successful
    #[must_use]
    pub fn recovery_successful(&self) -> bool {
        self.recovery_info.successful
    }

    /// Returns the recovery message if available
    #[must_use]
    pub fn recovery_message(&self) -> Option<&str> {
        self.recovery_info.message.as_deref()
    }
}

/// Responses API endpoints
#[derive(Clone)]
pub struct Responses {
    client: HttpClient,
    base_url: String,
    recovery_policy: RecoveryPolicy,
    recovery_callback: Option<Arc<RecoveryCallback>>,
}

impl std::fmt::Debug for Responses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Responses")
            .field("client", &self.client)
            .field("base_url", &self.base_url)
            .field("recovery_policy", &self.recovery_policy)
            .field("recovery_callback", &self.recovery_callback.is_some())
            .finish()
    }
}

impl Responses {
    /// Creates a new Responses API client
    pub(crate) fn new(client: HttpClient, base_url: String) -> Self {
        Self {
            client,
            base_url,
            recovery_policy: RecoveryPolicy::default(),
            recovery_callback: None,
        }
    }

    /// Creates a new Responses API client with recovery policy
    pub(crate) fn new_with_recovery(
        client: HttpClient,
        base_url: String,
        recovery_policy: RecoveryPolicy,
    ) -> Self {
        Self {
            client,
            base_url,
            recovery_policy,
            recovery_callback: None,
        }
    }

    /// Sets a callback function to be called when recovery occurs
    #[must_use]
    pub fn with_recovery_callback(mut self, callback: RecoveryCallback) -> Self {
        self.recovery_callback = Some(Arc::new(callback));
        self
    }

    /// Creates a response with automatic recovery handling.
    ///
    /// # Panics
    ///
    /// This method may panic if a recovery callback is set and the last error becomes None
    /// during recovery processing, though this should not occur during normal operation.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code,
    /// and recovery attempts (if any) also fail.
    pub async fn create_with_recovery(
        &self,
        request: crate::Request,
    ) -> Result<ResponseWithRecovery> {
        let mut current_request = request;
        let mut retry_count = 0;
        let mut last_error: Option<crate::Error> = None;

        loop {
            match self.create_internal(&current_request).await {
                Ok(response) => {
                    if retry_count > 0 {
                        // We had to recover, create recovery info
                        let recovery_info = RecoveryInfo::success(
                            retry_count,
                            if self.recovery_policy.notify_on_reset {
                                Some(self.recovery_policy.get_reset_message())
                            } else {
                                None
                            },
                            last_error.as_ref().map(std::string::ToString::to_string),
                        );

                        if self.recovery_policy.log_recovery_attempts {
                            log::info!("Successfully recovered from container expiration after {retry_count} attempts");
                        }

                        return Ok(ResponseWithRecovery::with_recovery(response, recovery_info));
                    }
                    // No recovery needed
                    return Ok(ResponseWithRecovery::new(response));
                }
                Err(error) => {
                    if error.is_recoverable()
                        && self.recovery_policy.auto_retry_on_expired_container
                        && retry_count < self.recovery_policy.max_retries
                    {
                        retry_count += 1;
                        last_error = Some(error);

                        if self.recovery_policy.log_recovery_attempts {
                            log::warn!(
                                "Container expired, attempting recovery (attempt {}/{})",
                                retry_count,
                                self.recovery_policy.max_retries
                            );
                        }

                        // Notify callback if set
                        if let Some(callback) = &self.recovery_callback {
                            callback(last_error.as_ref().unwrap(), retry_count);
                        }

                        // Prune expired containers from context if enabled
                        if self.recovery_policy.auto_prune_expired_containers {
                            current_request = self.prune_expired_context(current_request);
                        } else {
                            // Just clear the previous_response_id to start fresh
                            current_request.previous_response_id = None;
                        }
                    } else {
                        // Can't recover or max retries exceeded
                        if retry_count > 0 {
                            if self.recovery_policy.log_recovery_attempts {
                                log::error!(
                                    "Recovery failed after {retry_count} attempts: {error}"
                                );
                            }
                            return Err(crate::Error::MaxRetriesExceeded {
                                attempts: retry_count,
                            });
                        }
                        return Err(error);
                    }
                }
            }
        }
    }

    /// Creates a response (internal method without recovery).
    async fn create_internal(&self, request: &crate::Request) -> Result<crate::Response> {
        let response = self
            .client
            .post(format!("{}/responses", self.base_url))
            .json(request)
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }

    /// Prunes expired containers from the request context
    fn prune_expired_context(&self, mut request: crate::Request) -> crate::Request {
        // For now, we'll implement a simple strategy: clear the previous_response_id
        // In a more sophisticated implementation, we could track container lifecycles
        // and selectively prune only expired ones while preserving fresh context
        request.previous_response_id = None;

        if self.recovery_policy.log_recovery_attempts {
            log::debug!("Pruned expired context from request");
        }

        request
    }

    /// Manually prunes expired containers from a request
    ///
    /// This method can be called by applications that want to proactively
    /// clean up their context before making requests.
    #[must_use]
    pub fn prune_expired_context_manual(&self, request: crate::Request) -> crate::Request {
        self.prune_expired_context(request)
    }

    /// Creates a response (legacy method for backward compatibility).
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn create(&self, request: crate::Request) -> Result<crate::Response> {
        if self.recovery_policy.auto_retry_on_expired_container {
            // Use the recovery-enabled version and extract just the response
            self.create_with_recovery(request).await.map(|r| r.response)
        } else {
            // Use the direct version without recovery
            self.create_internal(&request).await
        }
    }

    /// Retrieves a response by ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn retrieve(&self, id: &str) -> Result<crate::Response> {
        let response = self
            .client
            .get(format!("{}/responses/{}", self.base_url, id))
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }

    /// Cancels a response that is being generated.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn cancel(&self, id: &str) -> Result<crate::Response> {
        let response = self
            .client
            .post(format!("{}/responses/{}/cancel", self.base_url, id))
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }

    /// Deletes a response.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn delete(&self, id: &str) -> Result<()> {
        let response = self
            .client
            .delete(format!("{}/responses/{}", self.base_url, id))
            .send()
            .await
            .map_err(crate::Error::Http)?;

        try_parse_api_error(response).await?;
        Ok(())
    }

    /// Creates a streaming response.
    ///
    /// # Panics
    ///
    /// This method may panic if the internal response state becomes inconsistent,
    /// though this is not expected during normal operation.
    ///
    /// # Errors
    ///
    /// Returns a stream of events or errors if the request fails to send or has a non-200 status code.
    #[cfg(feature = "stream")]
    pub fn stream(
        &self,
        mut request: crate::Request,
    ) -> std::pin::Pin<Box<dyn futures::Stream<Item = Result<crate::types::StreamEvent>> + Send>>
    {
        use futures::stream;

        // Ensure stream is set to true
        request.stream = Some(true);

        let url = format!("{}/responses", self.base_url);
        let client = self.client.clone();

        // Create stream that handles the actual OpenAI Responses API streaming format
        let stream = stream::unfold(None, move |mut response_opt| {
            let url = url.clone();
            let client = client.clone();
            let request = request.clone();

            async move {
                if response_opt.is_none() {
                    // Make the initial request
                    let response = match client.post(&url).json(&request).send().await {
                        Ok(response) => response,
                        Err(e) => {
                            return Some((
                                Err(crate::Error::Stream(format!("Failed to send request: {e}"))),
                                None,
                            ));
                        }
                    };

                    // Check if response is OK
                    if !response.status().is_success() {
                        return Some((
                            Err(crate::Error::Stream(format!(
                                "HTTP error: {} - {}",
                                response.status(),
                                response.text().await.unwrap_or_default()
                            ))),
                            None,
                        ));
                    }

                    response_opt = Some(response);
                }

                let response = response_opt.as_mut().unwrap();

                // Read chunks from the response
                match response.chunk().await {
                    Ok(Some(chunk)) => {
                        // Convert chunk to string
                        let chunk_str = match std::str::from_utf8(&chunk) {
                            Ok(s) => s,
                            Err(e) => {
                                return Some((
                                    Err(crate::Error::Stream(format!(
                                        "Invalid UTF-8 in chunk: {e}"
                                    ))),
                                    response_opt,
                                ));
                            }
                        };

                        // Try Server-Sent Events format first
                        for line in chunk_str.lines() {
                            let line = line.trim();
                            if line.is_empty() {
                                continue;
                            }

                            // Handle SSE format: "data: {...}" or "data: [DONE]"
                            if let Some(data) = line.strip_prefix("data: ") {
                                if data == "[DONE]" {
                                    return Some((Ok(crate::types::StreamEvent::Done), None));
                                }

                                // Try to parse as JSON
                                if let Ok(event) = serde_json::from_str::<serde_json::Value>(data) {
                                    if let Some(result) = Self::parse_stream_event(&event) {
                                        return Some((Ok(result), response_opt));
                                    }
                                }
                            }
                            // Handle direct JSONL format
                            else if let Ok(event) =
                                serde_json::from_str::<serde_json::Value>(line)
                            {
                                if let Some(result) = Self::parse_stream_event(&event) {
                                    return Some((Ok(result), response_opt));
                                }
                            }
                        }

                        // Continue to next chunk
                        Some((Ok(crate::types::StreamEvent::Chunk), response_opt))
                    }
                    Ok(None) => {
                        // End of stream
                        Some((Ok(crate::types::StreamEvent::Done), None))
                    }
                    Err(e) => Some((
                        Err(crate::Error::Stream(format!("Chunk read error: {e}"))),
                        None,
                    )),
                }
            }
        });

        Box::pin(stream)
    }

    #[cfg(feature = "stream")]
    fn parse_stream_event(event: &serde_json::Value) -> Option<crate::types::StreamEvent> {
        if let Some(event_type) = event.get("type").and_then(|t| t.as_str()) {
            match event_type {
                "response.output_text.delta" => {
                    if let Some(delta) = event.get("delta").and_then(|d| d.as_str()) {
                        let text_event = crate::types::StreamEvent::TextDelta {
                            content: delta.to_string(),
                            index: 0, // Default index
                        };
                        return Some(text_event);
                    }
                }
                "response.done" => {
                    return Some(crate::types::StreamEvent::Done);
                }
                "response.error" => {
                    // Handle errors outside this function since StreamEvent doesn't have Error variant
                    return None;
                }
                _ => {
                    return Some(crate::types::StreamEvent::Unknown);
                }
            }
        }
        None
    }
}
