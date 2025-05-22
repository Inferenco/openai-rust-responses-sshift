# Open AI Rust Responses by SShift

A comprehensive Rust SDK for the OpenAI Responses API that provides asynchronous access to all major endpoints and features.

[![Crates.io](https://img.shields.io/crates/v/open-ai-rust-responses-by-sshift.svg)](https://crates.io/crates/open-ai-rust-responses-by-sshift)
[![Documentation](https://docs.rs/open-ai-rust-responses-by-sshift/badge.svg)](https://docs.rs/open-ai-rust-responses-by-sshift)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- Full support for the OpenAI Responses API endpoints
- Conversation continuity through response IDs
- Thread management with helper methods
- Message history retrieval
- File and vector store operations
- Streaming responses via Server-Sent Events (SSE)
- Built-in tools support (web search, file search)
- Function calling capabilities
- Comprehensive error handling

## Installation

Add the library to your `Cargo.toml`:

```toml
[dependencies]
open-ai-rust-responses-by-sshift = "0.1.0"
```

## Quick Start

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client using your API key
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

## Conversation Management

Maintain conversation context across multiple interactions:

```rust
// Initial response
let request = Request::builder()
    .model(Model::GPT4o)
    .input("What is the capital of France?")
    .build();

let response = client.responses.create(request).await?;

// Follow-up using previous response ID
let follow_up = Request::builder()
    .model(Model::GPT4o)
    .input("Tell me more about its history")
    .previous_response_id(response.id())
    .build();

let response = client.responses.create(follow_up).await?;
```

Or use the thread helper methods for a more intuitive experience:

```rust
// Create a thread with an initial message
let request = threads::CreateThreadRequest {
    model: Model::GPT4o,
    instructions: Some("You are a helpful assistant".to_string()),
    initial_message: "Tell me about Paris".to_string(),
    metadata: None,
};

let (thread, response) = client.threads.create(request).await?;

// Continue the conversation
let (updated_thread, follow_up) = client.threads.continue_thread(&thread, "What are some famous landmarks?").await?;
```

## Streaming Responses

Process responses as they're generated:

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

## Vector Store Search

Create and search vector stores for semantic search:

```rust
// Create a vector store with files
let create_request = vector_stores::CreateVectorStoreRequest {
    name: "Research Papers".to_string(),
    file_ids: vec!["file_abc123".to_string()],
};

let vector_store = client.vector_stores.create(create_request).await?;

// Search the vector store
let search_request = vector_stores::SearchVectorStoreRequest {
    query: "quantum computing applications",
    limit: Some(5),
};

let results = client.vector_stores.search(&vector_store.id, search_request).await?;

// Process search results
for result in results.results {
    println!("File: {}, Snippet: {}", result.file_id, result.snippet);
}
```

## Tool Usage

Enable the model to use tools:

```rust
use open_ai_rust_responses_by_sshift::{Request, Tool, ToolChoice};

let tools = vec![
    Tool::function(
        "get_weather",
        "Get the current weather for a location",
        serde_json::json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city and state, e.g. San Francisco, CA"
                }
            },
            "required": ["location"]
        }),
    )
];

let request = Request::builder()
    .model(Model::GPT4o)
    .input("What's the weather like in San Francisco?")
    .tools(tools)
    .tool_choice(ToolChoice::auto())
    .build();

let response = client.responses.create(request).await?;

// Process tool calls
for tool_call in response.tool_calls() {
    println!("Tool: {}, Arguments: {}", tool_call.name, tool_call.arguments);
    
    // Handle the tool call and submit results
    // ...
}
```

## Feature Flags

The library provides several feature flags:

- `stream`: Enables streaming responses (enabled by default)
- `rustls`: Uses rustls for TLS support (enabled by default)
- `rustls-webpki-roots`: Uses rustls with webpki-roots
- `native-tls`: Uses native-tls for TLS support
- `native-tls-vendored`: Uses native-tls-vendored for TLS support

## Documentation

For more detailed documentation, see [DOCUMENTATION.md](DOCUMENTATION.md) or the [API reference](https://docs.rs/open-ai-rust-responses-by-sshift).

## License

This project is licensed under the MIT License - see the LICENSE file for details.
