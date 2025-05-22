#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::similar_names, clippy::doc_markdown)]

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
pub mod messages;
pub mod responses;
pub mod threads;
pub mod tools;
pub mod types;
pub mod vector_stores;
#[cfg(test)]
mod tests;

// Re-export types from the types module
pub use types::{
    Input, InputItem, Model, PaginatedList, PaginationParams, Request, RequestBuilder, Response, 
    ResponseItem, Tool, ToolCall, ToolChoice, StreamEvent,
};

// Re-export error types
pub use error::{Error, Result};

use reqwest::{Client as HttpClient, header};
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
    /// HTTP client
    http_client: HttpClient,
    
    /// Base URL for the API
    base_url: String,
    
    /// Responses API endpoints
    pub responses: responses::Responses,
    
    /// Threads API endpoints
    pub threads: threads::Threads,
    
    /// Messages API endpoints
    pub messages: messages::Messages,
    
    /// Files API endpoints
    pub files: files::Files,
    
    /// Vector stores API endpoints
    pub vector_stores: vector_stores::VectorStores,
    
    /// Tools API endpoints
    pub tools: tools::Tools,
}

impl Client {
    /// Creates a new client with the given API key
    pub fn new(api_key: &str) -> Result<Self, CreateError> {
        Self::new_with_base_url(api_key, "https://api.openai.com/v1")
    }
    
    /// Creates a new client with the given API key and base URL
    pub fn new_with_base_url(api_key: &str, base_url: &str) -> Result<Self, CreateError> {
        if api_key.is_empty() || !api_key.starts_with("sk-") {
            return Err(CreateError::InvalidApiKey);
        }
        
        let mut headers = header::HeaderMap::new();
        let auth_value = format!("Bearer {}", api_key);
        let auth_header = header::HeaderValue::from_str(&auth_value)
            .map_err(|_| CreateError::InvalidApiKey)?;
        headers.insert(header::AUTHORIZATION, auth_header);
        
        let user_agent = format!("open-ai-rust-responses-by-sshift/{}", env!("CARGO_PKG_VERSION"));
        
        let http_client = HttpClient::builder()
            .default_headers(headers)
            .user_agent(user_agent)
            .build()?;
        
        Ok(Self::new_with_http_client(http_client, base_url))
    }
    
    /// Creates a new client from the OPENAI_API_KEY environment variable
    pub fn from_env() -> Result<Self, CreateError> {
        let api_key = env::var("OPENAI_API_KEY").map_err(|_| CreateError::ApiKeyNotFound)?;
        Self::new(&api_key)
    }
    
    /// Creates a new client from the OPENAI_API_KEY environment variable with a custom base URL
    pub fn from_env_with_base_url(base_url: &str) -> Result<Self, CreateError> {
        let api_key = env::var("OPENAI_API_KEY").map_err(|_| CreateError::ApiKeyNotFound)?;
        Self::new_with_base_url(&api_key, base_url)
    }
    
    /// Creates a new client with a custom HTTP client
    pub fn new_with_http_client(http_client: HttpClient, base_url: &str) -> Self {
        let base_url = base_url.trim_end_matches('/').to_string();
        
        let responses = responses::Responses::new(http_client.clone(), base_url.clone());
        let threads = threads::Threads::new(http_client.clone(), base_url.clone(), responses.clone());
        let messages = messages::Messages::new(http_client.clone(), base_url.clone());
        let files = files::Files::new(http_client.clone(), base_url.clone());
        let vector_stores = vector_stores::VectorStores::new(http_client.clone(), base_url.clone());
        let tools = tools::Tools::new(http_client.clone(), base_url.clone());
        
        Self {
            http_client,
            base_url,
            responses,
            threads,
            messages,
            files,
            vector_stores,
            tools,
        }
    }
}
