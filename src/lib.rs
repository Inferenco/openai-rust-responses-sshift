#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::doc_markdown
)]

//! # Open AI Rust Responses by SShift
//!
//! A comprehensive Rust SDK for the OpenAI Responses API that provides asynchronous access
//! to all major endpoints and features.
//!
//! ## Features
//!
//! - Full support for the Responses API endpoints
//! - Conversation continuity through response IDs
//! - Message history retrieval
//! - File and vector store operations
//! - Streaming responses via Server-Sent Events (SSE)
//! - Built-in tools support (web search, file search)
//! - Function calling capabilities

mod error;
pub mod files;
pub mod images;
pub mod messages;
pub mod responses;
#[cfg(test)]
mod tests;
pub mod tools;
pub mod types;
pub mod vector_stores;

// Re-export types from the types module
pub use types::{
    FunctionCallInfo, Input, InputItem, MessageContent, Model, PaginatedList, PaginationParams,
    ReasoningEffort, Request, RequestBuilder, Response, ResponseItem, StreamEvent, Tool, ToolCall,
    ToolChoice, Verbosity,
};

// Re-export container and tool types
pub use types::{Container, RecoveryCallback, RecoveryPolicy};

// Re-export recovery types
pub use responses::{RecoveryInfo, ResponseWithRecovery};

// Re-export image types
pub use images::{ImageData, ImageGenerateRequest, ImageGenerateResponse};

// Re-export vector store types
pub use vector_stores::{
    AddFileToVectorStoreRequest, CreateVectorStoreRequest, SearchVectorStoreRequest,
    SearchVectorStoreResponse, VectorStore, VectorStoreFileDeleteResponse,
};

// Re-export error types
pub use error::{Error, Result};

use reqwest::{header, Client as HttpClient};
use std::env;

/// Error that can occur when creating a client
#[derive(Debug, thiserror::Error)]
pub enum CreateError {
    /// API key is invalid
    #[error("Invalid API key format")]
    InvalidApiKey,

    /// API key not found in environment
    #[error("API key not found in environment")]
    ApiKeyNotFound,

    /// HTTP client creation error
    #[error("Failed to create HTTP client: {0}")]
    HttpClient(#[from] reqwest::Error),
}

/// Client for the OpenAI Responses API
#[derive(Debug, Clone)]
pub struct Client {
    /// Responses API endpoints
    pub responses: responses::Responses,

    /// Messages API endpoints
    pub messages: messages::Messages,

    /// Files API endpoints
    pub files: files::Files,

    /// Vector stores API endpoints
    pub vector_stores: vector_stores::VectorStores,

    /// Tools API endpoints
    pub tools: tools::Tools,

    /// Images API endpoints
    pub images: images::Images,
}

impl Client {
    /// Creates a new client with the given API key
    ///
    /// # Errors
    ///
    /// Returns `CreateError::InvalidApiKey` if the API key is empty or doesn't start with "sk-"
    pub fn new(api_key: &str) -> std::result::Result<Self, CreateError> {
        Self::new_with_base_url(api_key, "https://api.openai.com/v1")
    }

    /// Creates a new client with the given API key and base URL
    ///
    /// # Errors
    ///
    /// Returns `CreateError::InvalidApiKey` if the API key is empty, doesn't start with "sk-", or contains invalid characters
    pub fn new_with_base_url(
        api_key: &str,
        base_url: &str,
    ) -> std::result::Result<Self, CreateError> {
        if api_key.is_empty() || !api_key.starts_with("sk-") {
            return Err(CreateError::InvalidApiKey);
        }

        let mut headers = header::HeaderMap::new();
        let auth_value = format!("Bearer {api_key}");
        let auth_header =
            header::HeaderValue::from_str(&auth_value).map_err(|_| CreateError::InvalidApiKey)?;
        headers.insert(header::AUTHORIZATION, auth_header);

        let user_agent = format!(
            "open-ai-rust-responses-by-sshift/{}",
            env!("CARGO_PKG_VERSION")
        );

        let http_client = HttpClient::builder()
            .default_headers(headers)
            .user_agent(user_agent)
            .build()?;

        Ok(Self::new_with_http_client(&http_client, base_url))
    }

    /// Creates a client from the `OPENAI_API_KEY` environment variable
    ///
    /// # Errors
    ///
    /// Returns `CreateError::InvalidApiKey` if the environment variable is not set or invalid
    pub fn from_env() -> std::result::Result<Self, CreateError> {
        Self::from_env_with_base_url("https://api.openai.com/v1")
    }

    /// Creates a client from the `OPENAI_API_KEY` environment variable with a custom base URL
    ///
    /// # Errors
    ///
    /// Returns `CreateError::InvalidApiKey` if the environment variable is not set or invalid
    pub fn from_env_with_base_url(base_url: &str) -> std::result::Result<Self, CreateError> {
        let api_key = std::env::var("OPENAI_API_KEY").map_err(|_| CreateError::InvalidApiKey)?;
        Self::new_with_base_url(&api_key, base_url)
    }

    /// Creates a new client with the given HTTP client and base URL
    #[must_use]
    pub fn new_with_http_client(http_client: &HttpClient, base_url: &str) -> Self {
        Self::new_with_http_client_and_recovery(http_client, base_url, RecoveryPolicy::default())
    }

    /// Creates a new client with the given HTTP client, base URL, and recovery policy
    #[must_use]
    pub fn new_with_http_client_and_recovery(
        http_client: &HttpClient,
        base_url: &str,
        recovery_policy: RecoveryPolicy,
    ) -> Self {
        let base_url = base_url.trim_end_matches('/').to_string();

        let responses = responses::Responses::new_with_recovery(
            http_client.clone(),
            base_url.clone(),
            recovery_policy,
        );
        let messages = messages::Messages::new(http_client.clone(), base_url.clone());
        let files = files::Files::new(http_client.clone(), base_url.clone());
        let vector_stores = vector_stores::VectorStores::new(http_client.clone(), base_url.clone());
        let tools = tools::Tools::new(http_client.clone(), base_url.clone());
        let images = images::Images::new(http_client.clone(), base_url.clone());

        Self {
            responses,
            messages,
            files,
            vector_stores,
            tools,
            images,
        }
    }

    /// Creates a new client with recovery policy from the given API key
    ///
    /// # Errors
    ///
    /// Returns `CreateError::InvalidApiKey` if the API key is empty or doesn't start with "sk-"
    pub fn new_with_recovery(
        api_key: &str,
        recovery_policy: RecoveryPolicy,
    ) -> std::result::Result<Self, CreateError> {
        Self::new_with_base_url_and_recovery(api_key, "https://api.openai.com/v1", recovery_policy)
    }

    /// Creates a new client with recovery policy from the given API key and base URL
    ///
    /// # Errors
    ///
    /// Returns `CreateError::InvalidApiKey` if the API key is empty, doesn't start with "sk-", or contains invalid characters
    pub fn new_with_base_url_and_recovery(
        api_key: &str,
        base_url: &str,
        recovery_policy: RecoveryPolicy,
    ) -> std::result::Result<Self, CreateError> {
        if api_key.is_empty() || !api_key.starts_with("sk-") {
            return Err(CreateError::InvalidApiKey);
        }

        let mut headers = header::HeaderMap::new();
        let auth_value = format!("Bearer {api_key}");
        let auth_header =
            header::HeaderValue::from_str(&auth_value).map_err(|_| CreateError::InvalidApiKey)?;
        headers.insert(header::AUTHORIZATION, auth_header);

        let user_agent = format!(
            "open-ai-rust-responses-by-sshift/{}",
            env!("CARGO_PKG_VERSION")
        );

        let http_client = HttpClient::builder()
            .default_headers(headers)
            .user_agent(user_agent)
            .build()?;

        Ok(Self::new_with_http_client_and_recovery(
            &http_client,
            base_url,
            recovery_policy,
        ))
    }

    /// Creates a client with recovery policy from the `OPENAI_API_KEY` environment variable
    ///
    /// # Errors
    ///
    /// Returns `CreateError::InvalidApiKey` if the environment variable is not set or invalid
    pub fn from_env_with_recovery(
        recovery_policy: RecoveryPolicy,
    ) -> std::result::Result<Self, CreateError> {
        Self::from_env_with_base_url_and_recovery("https://api.openai.com/v1", recovery_policy)
    }

    /// Creates a client with recovery policy from the `OPENAI_API_KEY` environment variable with a custom base URL
    ///
    /// # Errors
    ///
    /// Returns `CreateError::InvalidApiKey` if the environment variable is not set or invalid
    pub fn from_env_with_base_url_and_recovery(
        base_url: &str,
        recovery_policy: RecoveryPolicy,
    ) -> std::result::Result<Self, CreateError> {
        let api_key = std::env::var("OPENAI_API_KEY").map_err(|_| CreateError::InvalidApiKey)?;
        Self::new_with_base_url_and_recovery(&api_key, base_url, recovery_policy)
    }
}
