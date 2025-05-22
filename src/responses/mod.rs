use crate::error::{Result, try_parse_api_error};
use crate::types::{Input, Model, ResponseItem};
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};

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
    /// # Errors
    ///
    /// Returns a stream of events or errors if the request fails to send or has a non-200 status code.
    #[cfg(feature = "stream")]
    pub fn stream(&self, mut request: crate::Request) -> impl futures::Stream<Item = Result<crate::types::StreamEvent>> {
        use futures::StreamExt;
        use reqwest_eventsource::{Event, EventSource};
        
        // Ensure stream is set to true
        request.stream = Some(true);
        
        let url = format!("{}/responses", self.base_url);
        let client = self.client.clone();
        
        async_fn_stream::fn_stream(move || {
            let req = client
                .post(&url)
                .json(&request);
                
            let mut event_source = EventSource::new(req).unwrap();
            
            async move {
                while let Some(event) = event_source.next().await {
                    match event {
                        Ok(Event::Open) => continue,
                        Ok(Event::Message(message)) => {
                            if message.data == "[DONE]" {
                                break;
                            }
                            
                            match serde_json::from_str::<crate::types::StreamEvent>(&message.data) {
                                Ok(event) => yield Ok(event),
                                Err(e) => yield Err(crate::Error::Json(e)),
                            }
                        }
                        Err(e) => {
                            yield Err(crate::Error::Http(e));
                            break;
                        }
                    }
                }
                
                event_source.close();
            }
        })
    }
}
