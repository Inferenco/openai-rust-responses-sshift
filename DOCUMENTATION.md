# Open AI Rust Responses by SShift - Documentation

This document provides comprehensive documentation for the Open AI Rust Responses by SShift library, a Rust SDK for the OpenAI Responses API.

## Table of Contents

1. [Installation](#installation)
2. [Authentication](#authentication)
3. [Basic Usage](#basic-usage)
4. [Responses API](#responses-api)
5. [Messages API](#messages-api)
6. [Files API](#files-api)
7. [Vector Stores API](#vector-stores-api)
8. [Tools API](#tools-api)
9. [Streaming Responses](#streaming-responses)
10. [Error Handling](#error-handling)
11. [Advanced Configuration](#advanced-configuration)
12. [Feature Flags](#feature-flags)

## Installation

Add the library to your `Cargo.toml`:

```toml
[dependencies]
open-ai-rust-responses-by-sshift = "0.1.0"
```

If you want to use streaming responses, make sure to include the `stream` feature (enabled by default):

```toml
[dependencies]
open-ai-rust-responses-by-sshift = { version = "0.1.0", features = ["stream"] }
```

## Authentication

You can create a client using your OpenAI API key:

```rust
use open_ai_rust_responses_by_sshift::Client;

// Create a client with an API key
let client = Client::new("sk-your-api-key").expect("Failed to create client");

// Or from environment variable OPENAI_API_KEY
let client = Client::from_env().expect("Failed to create client");
```

## Basic Usage

Here's a simple example of creating a response:

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = Client::from_env()?;
    
    // Create a request
    let request = Request::builder()
        .model(Model::GPT4o)
        .input("What is the capital of France?")
        .build();
    
    // Send the request
    let response = client.responses.create(request).await?;
    
    // Print the response
    println!("Response: {}", response.output_text());
    
    Ok(())
}
```

## Responses API

The Responses API allows you to create, retrieve, and manage responses.

### Creating a Response

```rust
// Using the builder pattern
let request = Request::builder()
    .model(Model::GPT4o)
    .input("What is the capital of France?")
    .instructions("Keep your answer brief")
    .max_tokens(100)
    .temperature(0.7)
    .build();

let response = client.responses.create(request).await?;
```

### Retrieving a Response

```rust
let response = client.responses.retrieve("resp_abc123").await?;
```

### Canceling a Response

```rust
let response = client.responses.cancel("resp_abc123").await?;
```

### Deleting a Response

```rust
client.responses.delete("resp_abc123").await?;
```

## Messages API

The Messages API allows you to manage messages.

### Creating a Message

```rust
let request = messages::CreateMessageRequest {
    role: "user".to_string(),
    content: "What's the weather like today?".to_string(),
    metadata: None,
};

let message = client.messages.create("thread_abc123", request).await?;
```

### Retrieving a Message

```rust
let message = client.messages.retrieve("thread_abc123", "msg_abc123").await?;
```

### Listing Messages

```rust
let messages = client.messages.list("thread_abc123", None).await?;
```

## Files API

The Files API allows you to upload, retrieve, and manage files.

### Uploading a File

```rust
let request = files::CreateFileRequest {
    purpose: "assistants".to_string(),
    file: file_bytes,
    filename: "document.pdf".to_string(),
};

let file = client.files.create(request).await?;
```

### Retrieving a File

```rust
let file = client.files.get("file_abc123").await?;
```

### Listing Files

```rust
let files = client.files.list(None).await?;
```

### Downloading File Content

```rust
let content = client.files.download("file_abc123").await?;
```

### Deleting a File

```rust
client.files.delete("file_abc123").await?;
```

## Vector Stores API

The Vector Stores API allows you to create and manage vector stores for semantic search.

### Creating a Vector Store

```rust
let request = vector_stores::CreateVectorStoreRequest {
    name: "My Vector Store".to_string(),
    file_ids: vec!["file_abc123".to_string()],
};

let vector_store = client.vector_stores.create(request).await?;
```

### Adding Files to a Vector Store

```rust
let request = vector_stores::AddFilesToVectorStoreRequest {
    file_ids: vec!["file_def456".to_string()],
};

let updated_store = client.vector_stores.add_files("vs_abc123", request).await?;
```

### Searching a Vector Store

```rust
let request = vector_stores::SearchVectorStoreRequest {
    query: "quantum computing".to_string(),
    limit: Some(5),
};

let results = client.vector_stores.search("vs_abc123", request).await?;
```

### Removing a File from a Vector Store

```rust
let updated_store = client.vector_stores.delete_file("vs_abc123", "file_abc123").await?;
```

## Tools API

The Tools API provides access to specialized tools like web search.

### Web Search

```rust
let results = client.tools.web_search("latest news about AI").await?;
```

### File Search

```rust
let results = client.tools.file_search("vs_abc123", "quantum computing").await?;
```

## Streaming Responses

To use streaming responses, enable the `stream` feature (enabled by default).

```rust
use futures::StreamExt;

let request = Request::builder()
    .model(Model::GPT4o)
    .input("Write a short story about a robot")
    .stream(true)
    .build();

let mut stream = client.responses.stream(request);

while let Some(event) = stream.next().await {
    match event {
        Ok(event) => {
            if let Some(content) = event.as_text_delta() {
                print!("{}", content);
            }
            
            if event.is_done() {
                break;
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            break;
        }
    }
}
```

## Error Handling

The library provides a comprehensive error type that covers various failure scenarios:

```rust
match client.responses.create(request).await {
    Ok(response) => {
        println!("Success: {}", response.output_text());
    },
    Err(e) => match e {
        Error::Api { message, error_type, code } => {
            eprintln!("API error: {} (type: {}, code: {:?})", message, error_type, code);
        },
        Error::Http(e) => {
            eprintln!("HTTP error: {}", e);
        },
        Error::Json(e) => {
            eprintln!("JSON error: {}", e);
        },
        Error::Stream(e) => {
            eprintln!("Stream error: {}", e);
        },
        Error::InvalidApiKey => {
            eprintln!("Invalid API key");
        },
        Error::ApiKeyNotFound => {
            eprintln!("API key not found in environment");
        },
    }
}
```

## Advanced Configuration

### Custom Base URL

```rust
let client = Client::new_with_base_url("sk-your-api-key", "https://custom-openai-api.example.com/v1")?;
```

### Custom HTTP Client

```rust
use reqwest::Client as HttpClient;

let http_client = HttpClient::builder()
    .timeout(std::time::Duration::from_secs(30))
    .build()?;

let client = Client::new_with_http_client("sk-your-api-key", http_client, "https://api.openai.com/v1");
```

## Feature Flags

The library provides several feature flags to customize its behavior:

- `stream`: Enables streaming responses (enabled by default)
- `rustls`: Uses rustls for TLS support (enabled by default)
- `rustls-webpki-roots`: Uses rustls with webpki-roots
- `native-tls`: Uses native-tls for TLS support
- `native-tls-vendored`: Uses native-tls-vendored for TLS support

Example of using a specific TLS implementation:

```toml
[dependencies]
open-ai-rust-responses-by-sshift = { version = "0.1.0", default-features = false, features = ["stream", "native-tls"] }
```

This will use the native TLS implementation instead of rustls.
