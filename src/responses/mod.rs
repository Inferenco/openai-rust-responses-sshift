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
    /// This method may panic if the EventSource fails to initialize properly with the provided request.
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
        use futures::StreamExt;
        use reqwest_eventsource::{Event, EventSource};

        // Ensure stream is set to true
        request.stream = Some(true);

        let url = format!("{}/responses", self.base_url);
        let client = self.client.clone();

        // Create event source and convert to stream
        let stream = stream::unfold(None, move |mut event_source_opt| {
            let url = url.clone();
            let client = client.clone();
            let request = request.clone();

            async move {
                if event_source_opt.is_none() {
                    // Initialize EventSource on first call
                    let req = client.post(&url).json(&request);
                    match EventSource::new(req) {
                        Ok(event_source) => {
                            event_source_opt = Some(event_source);
                        }
                        Err(e) => {
                            return Some((
                                Err(crate::Error::Stream(format!(
                                    "Failed to create EventSource: {e}"
                                ))),
                                None,
                            ));
                        }
                    }
                }

                let event_source = event_source_opt.as_mut().unwrap();
                match event_source.next().await {
                    Some(event) => match event {
                        Ok(Event::Message(msg)) => {
                            if msg.data == "[DONE]" {
                                Some((Ok(crate::types::StreamEvent::Done), None))
                            } else {
                                match serde_json::from_str::<crate::types::StreamEvent>(&msg.data) {
                                    Ok(stream_event) => Some((Ok(stream_event), event_source_opt)),
                                    Err(e) => Some((
                                        Err(crate::Error::Stream(format!(
                                            "Failed to parse event: {e}"
                                        ))),
                                        event_source_opt,
                                    )),
                                }
                            }
                        }
                        Ok(_) => {
                            // Skip other event types and continue
                            Some((Ok(crate::types::StreamEvent::Done), event_source_opt))
                        }
                        Err(e) => Some((
                            Err(crate::Error::Stream(format!("EventSource error: {e}"))),
                            None,
                        )),
                    },
                    None => None, // End of stream
                }
            }
        });

        Box::pin(stream)
    }
}
