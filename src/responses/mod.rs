use crate::error::{try_parse_api_error, Result};
use reqwest::Client as HttpClient;

/// Responses API endpoints
#[derive(Debug, Clone)]
pub struct Responses {
    client: HttpClient,
    base_url: String,
}

impl Responses {
    /// Creates a new Responses API client
    pub(crate) fn new(client: HttpClient, base_url: String) -> Self {
        Self { client, base_url }
    }

    /// Creates a response.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn create(&self, request: crate::Request) -> Result<crate::Response> {
        let response = self
            .client
            .post(format!("{}/responses", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
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
