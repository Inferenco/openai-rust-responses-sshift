use crate::error::{try_parse_api_error, Result};
use crate::responses::Responses;
use crate::types::{PaginatedList, PaginationParams};
use chrono::{DateTime, Utc};
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};

/// Threads API endpoints
#[derive(Debug, Clone)]
pub struct Threads {
    client: HttpClient,
    base_url: String,
    responses: Responses,
}

/// Thread object representing a conversation thread
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    /// Unique identifier for the thread
    pub id: String,

    /// Type of object (always "thread")
    pub object: String,

    /// Unix timestamp for when the thread was created
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,

    /// Optional metadata associated with the thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    /// Current response ID for this thread (used for conversation continuity)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_response_id: Option<String>,

    /// Current model used for this thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_model: Option<crate::types::Model>,
}

/// Request to create a new thread
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateThreadRequest {
    /// Model to use for the thread
    pub model: crate::types::Model,

    /// Optional system instructions to guide the model's behavior
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Initial message to start the thread with
    #[serde(skip)]
    pub initial_message: String,

    /// Optional metadata to associate with the thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Request to update a thread
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateThreadRequest {
    /// Optional metadata to associate with the thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl Threads {
    /// Creates a new Threads API client
    pub(crate) fn new(client: HttpClient, base_url: String, responses: Responses) -> Self {
        Self {
            client,
            base_url,
            responses,
        }
    }

    /// Creates a new thread.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn create(&self, request: CreateThreadRequest) -> Result<(Thread, crate::Response)> {
        // First create a thread
        let thread_request = serde_json::json!({
            "metadata": request.metadata
        });

        let response = self
            .client
            .post(format!("{}/threads", self.base_url))
            .json(&thread_request)
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        let thread: Thread = response.json().await.map_err(crate::Error::Http)?;

        // Then create an initial message in the thread
        let response_request = crate::Request {
            model: request.model.clone(),
            input: crate::Input::Text(request.initial_message),
            instructions: request.instructions,
            ..Default::default()
        };

        let response = self.responses.create(response_request).await?;

        // Update the thread with the response ID and model
        let mut thread = thread;
        thread.current_response_id = Some(response.id().to_string());
        thread.current_model = Some(request.model);

        Ok((thread, response))
    }

    /// Retrieves a thread by ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn retrieve(&self, thread_id: &str) -> Result<Thread> {
        let response = self
            .client
            .get(format!("{}/threads/{}", self.base_url, thread_id))
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }

    /// Updates a thread.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn update(&self, thread_id: &str, request: UpdateThreadRequest) -> Result<Thread> {
        let response = self
            .client
            .patch(format!("{}/threads/{}", self.base_url, thread_id))
            .json(&request)
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }

    /// Deletes a thread.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn delete(&self, thread_id: &str) -> Result<()> {
        let response = self
            .client
            .delete(format!("{}/threads/{}", self.base_url, thread_id))
            .send()
            .await
            .map_err(crate::Error::Http)?;

        try_parse_api_error(response).await?;
        Ok(())
    }

    /// Lists all threads.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn list(&self, params: Option<PaginationParams>) -> Result<PaginatedList<Thread>> {
        let mut request = self.client.get(format!("{}/threads", self.base_url));

        if let Some(params) = params {
            request = request.query(&params);
        }

        let response = request.send().await.map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }

    /// Continue a conversation in a thread with a specific model.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn continue_thread(
        &self,
        thread: &Thread,
        model: crate::types::Model,
        message: impl Into<String>,
    ) -> Result<(Thread, crate::Response)> {
        let message = message.into();

        // Create a response that continues from the previous one
        let response_request = crate::Request {
            model: model.clone(),
            input: crate::Input::Text(message),
            previous_response_id: thread.current_response_id.clone(),
            ..Default::default()
        };

        let response = self.responses.create(response_request).await?;

        // Update the thread with the new response ID and model
        let mut updated_thread = thread.clone();
        updated_thread.current_response_id = Some(response.id().to_string());
        updated_thread.current_model = Some(model);

        Ok((updated_thread, response))
    }

    /// Continue a conversation in a thread using the same model as the previous response.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn continue_with_user_input(
        &self,
        thread: &Thread,
        input: impl Into<String>,
    ) -> Result<(Thread, crate::Response)> {
        let model = thread
            .current_model
            .clone()
            .unwrap_or(crate::types::Model::GPT4o);
        self.continue_thread(thread, model, input).await
    }
}
