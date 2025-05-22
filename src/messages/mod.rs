use crate::error::{Result, try_parse_api_error};
use crate::types::{PaginatedList, PaginationParams};
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Messages API endpoints
#[derive(Debug, Clone)]
pub struct Messages {
    client: HttpClient,
    base_url: String,
}

/// Message object representing a message in a thread
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique identifier for the message
    pub id: String,
    
    /// Type of object (always "message")
    pub object: String,
    
    /// Thread ID that this message belongs to
    pub thread_id: String,
    
    /// Role of the message sender (user or assistant)
    pub role: String,
    
    /// Content of the message
    pub content: String,
    
    /// Unix timestamp for when the message was created
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    
    /// Optional metadata associated with the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Request to create a message
#[derive(Debug, Clone, Serialize)]
pub struct CreateMessageRequest {
    /// Role of the message sender (user or assistant)
    pub role: String,
    
    /// Content of the message
    pub content: String,
    
    /// Optional metadata to associate with the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Response containing a list of messages
#[derive(Debug, Clone, Deserialize)]
pub struct ListMessagesResponse {
    /// List of messages
    pub data: Vec<Message>,
    
    /// Whether there are more messages to retrieve
    pub has_more: bool,
}

impl ListMessagesResponse {
    /// Returns the messages in this response
    pub fn items(&self) -> &[Message] {
        &self.data
    }
    
    /// Returns the number of messages in this response
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Returns true if there are no messages in this response
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Messages {
    /// Creates a new Messages API client
    pub(crate) fn new(client: HttpClient, base_url: String) -> Self {
        Self { client, base_url }
    }

    /// Creates a message in a thread.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn create(&self, thread_id: &str, request: CreateMessageRequest) -> Result<Message> {
        let response = self
            .client
            .post(format!("{}/threads/{}/messages", self.base_url, thread_id))
            .json(&request)
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }

    /// Retrieves a message by ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn retrieve(&self, thread_id: &str, message_id: &str) -> Result<Message> {
        let response = self
            .client
            .get(format!("{}/threads/{}/messages/{}", self.base_url, thread_id, message_id))
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }

    /// Lists messages in a thread.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn list(&self, thread_id: &str, params: Option<PaginationParams>) -> Result<ListMessagesResponse> {
        let mut request = self
            .client
            .get(format!("{}/threads/{}/messages", self.base_url, thread_id));
            
        if let Some(params) = params {
            request = request.query(&params);
        }
        
        let response = request
            .send()
            .await
            .map_err(crate::Error::Http)?;

        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }
    
    /// Creates a message in a conversation using response IDs.
    ///
    /// This is a helper method that uses response IDs for conversation continuity.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn create_with_response_id(&self, previous_response_id: &str, request: CreateMessageRequest) -> Result<Message> {
        // Create a response that includes this message as part of the conversation
        let response_request = crate::responses::Request {
            model: crate::types::Model::GPT4o, // Default model, can be overridden
            input: crate::types::Input::Text(request.content),
            previous_response_id: Some(previous_response_id.to_string()),
            ..Default::default()
        };
        
        let response = crate::responses::Responses::new(self.client.clone(), self.base_url.clone())
            .create(response_request)
            .await?;
        
        // Convert the response to a message format
        let message = Message {
            id: response.id().to_string(),
            object: "message".to_string(),
            thread_id: previous_response_id.to_string(), // Use previous response ID as thread ID
            role: "assistant".to_string(),
            content: response.output_text(),
            created_at: response.created_at,
            metadata: None,
        };
        
        Ok(message)
    }
    
    /// Retrieves message history using response IDs.
    ///
    /// This is a helper method that uses response IDs to retrieve conversation history.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send or has a non-200 status code.
    pub async fn list_with_response_id(&self, response_id: &str, limit: Option<u32>) -> Result<ListMessagesResponse> {
        // In the actual API, we would retrieve the conversation history
        // Since the API doesn't have a direct endpoint for this, we'll simulate it
        // by retrieving the current response and its previous responses
        
        let mut messages = Vec::new();
        let mut current_id = Some(response_id.to_string());
        let mut count = 0;
        
        while let Some(id) = current_id {
            if let Some(max) = limit {
                if count >= max {
                    break;
                }
            }
            
            let response = crate::responses::Responses::new(self.client.clone(), self.base_url.clone())
                .retrieve(&id)
                .await?;
            
            messages.push(Message {
                id: response.id().to_string(),
                object: "message".to_string(),
                thread_id: id.clone(), // Use response ID as thread ID
                role: "assistant".to_string(),
                content: response.output_text(),
                created_at: response.created_at,
                metadata: None,
            });
            
            current_id = response.previous_response_id;
            count += 1;
        }
        
        // Reverse to get chronological order
        messages.reverse();
        
        Ok(ListMessagesResponse {
            data: messages,
            has_more: false,
        })
    }
}
