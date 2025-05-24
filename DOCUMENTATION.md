# Open AI Rust Responses by SShift - Documentation

This document provides comprehensive documentation for the Open AI Rust Responses by SShift library, a Rust SDK for the OpenAI Responses API with **May 2025 API extensions**.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Installation](#installation)
3. [Authentication](#authentication)
4. [Basic Usage](#basic-usage)
5. [Responses API](#responses-api)
6. [Messages API](#messages-api)
7. [Files API](#files-api)
8. [Vector Stores API](#vector-stores-api)
9. [Tools API](#tools-api)
10. [**NEW: May 2025 Features**](#may-2025-features)
11. [**NEW: Image Generation**](#image-generation)
12. [**NEW: MCP Integration**](#mcp-integration)
13. [**NEW: Enhanced Reasoning**](#enhanced-reasoning)
14. [**NEW: Type-Safe Includes**](#type-safe-includes)
15. [Streaming Responses](#streaming-responses)
16. [Error Handling](#error-handling)
17. [Advanced Configuration](#advanced-configuration)
18. [Feature Flags](#feature-flags)
19. [Testing and Examples](#testing-and-examples)

## Quick Start

Get up and running in under a minute:

```bash
# 1. Add to your project
cargo add open-ai-rust-responses-by-sshift tokio --features tokio/full

# 2. Set API key
export OPENAI_API_KEY=sk-your-api-key-here

# 3. Try the comprehensive demo
cargo run --example comprehensive_demo --features stream
```

Or create a simple example:

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    let request = Request::builder()
        .model(Model::GPT4o)
        .input("What is Rust programming language?")
        .build();
    
    let response = client.responses.create(request).await?;
    println!("Response: {}", response.output_text());
    
    Ok(())
}
```

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

To use streaming responses, enable the `stream` feature (enabled by default). The May 2025 extensions include enhanced streaming with new event types.

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

### Enhanced Streaming with May 2025 Events

The new streaming API includes additional event types for richer real-time experiences:

```rust
use open_ai_rust_responses_by_sshift::StreamEvent;
use futures::StreamExt;

let request = Request::builder()
    .model(Model::GPT4o)
    .input("Create a presentation with images about space exploration")
    .tools(vec![Tool::image_generation(None)])
    .build();

let mut stream = client.responses.stream(request);
let mut image_count = 0;
let mut text_length = 0;

while let Some(event) = stream.next().await {
    match event? {
        StreamEvent::TextDelta { content, .. } => {
            print!("{}", content);
            text_length += content.len();
        }
        StreamEvent::ImageProgress { url, index } => {
            if let Some(image_url) = url {
                println!("\n🎨 Image {} completed: {}", index, image_url);
                image_count += 1;
            } else {
                println!("\n🎨 Generating image {}...", index);
            }
        }
        StreamEvent::Done => {
            println!("\n✅ Stream completed!");
            println!("📊 Generated {} characters and {} images", text_length, image_count);
            break;
        }
        _ => {} // Handle other event types as needed
    }
}
```

### Streaming with All Event Types

```rust
while let Some(event) = stream.next().await {
    match event? {
        StreamEvent::ResponseCreated { response_id } => {
            println!("🚀 Response started: {}", response_id);
        }
        StreamEvent::TextDelta { content, .. } => {
            print!("{}", content);
        }
        StreamEvent::ImageProgress { url, index } => {
            if let Some(image_url) = url {
                println!("\n🎨 Image {}: {}", index, image_url);
            } else {
                println!("\n🔄 Processing image {}...", index);
            }
        }
        StreamEvent::ToolCallStart { tool_name } => {
            println!("\n🛠️ Using tool: {}", tool_name);
        }
        StreamEvent::ToolCallEnd { tool_name, result } => {
            println!("\n✅ Tool {} completed: {:?}", tool_name, result);
        }
        StreamEvent::Error { error } => {
            eprintln!("\n❌ Error: {}", error);
            break;
        }
        StreamEvent::Done => {
            println!("\n🏁 Complete!");
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

## Testing and Examples

### Running the Test Suite

```bash
# Run unit and integration tests
cargo test

# Run tests with all features enabled
cargo test --all-features

# Run specific test modules
cargo test responses
cargo test files

# Run integration tests that require API key (shows streaming output)
OPENAI_API_KEY=sk-your-key cargo test --features stream -- --ignored --nocapture
```

### Understanding Test Output

Most tests run without API calls using mocked responses. However, integration tests marked with `#[ignore]` require a real API key and demonstrate actual functionality:

- **Unit Tests**: Fast, no API key required, test internal logic
- **Integration Tests**: Require `--ignored` flag and API key, test real API calls  
- **Streaming Tests**: Use `--nocapture` to see real-time streaming output

> **Note**: If you see tests marked as `ignored`, this is **not an error**! These are intentionally skipped integration tests that require API keys. Use the `--ignored` flag to run them when you have an API key available.

### Running Examples

The library includes several examples to demonstrate different features:

```bash
# Basic usage
cargo run --example basic

# Conversation continuity
cargo run --example conversation

# Streaming responses (requires stream feature)
cargo run --example streaming --features stream

# Comprehensive demo of all features (requires API key)
OPENAI_API_KEY=sk-your-key cargo run --example comprehensive_demo --features stream
```

### Environment Setup for Examples

Create a `.env` file with your OpenAI API key:

```bash
echo "OPENAI_API_KEY=sk-your-api-key-here" > .env
```

The examples will automatically load the API key from this file or from the environment variable.

### Comprehensive Demo Features

The `comprehensive_demo.rs` example showcases all major SDK features including the new May 2025 extensions:

- **Response Creation**: Basic and advanced requests
- **Conversation Continuity**: Using `previous_response_id`
- **File Operations**: Upload, list, download, and delete files
- **Vector Stores**: Create, add files, search, and delete
- **Built-in Tools**: Web search and file search capabilities
- **Custom Function Calling**: Calculator tool example
- **🎨 NEW: Image Generation**: AI-powered visual content creation
- **🔌 NEW: MCP Integration**: External knowledge source connections
- **🧠 NEW: Enhanced Reasoning**: Access to AI reasoning processes
- **🔒 NEW: Type-Safe Includes**: Compile-time validation for include options
- **Streaming Responses**: Real-time response streaming (if enabled) with ImageProgress events
- **Resource Management**: Proper cleanup and deletion testing

This demo creates temporary resources for testing and cleans them up afterward, making it safe to run multiple times. It now includes comprehensive testing of all Phase 1 features to demonstrate ~95% API coverage.

## May 2025 Features

OpenAI's May 2025 API release introduced several cutting-edge capabilities that are fully supported in this SDK. These features represent the latest in AI technology and are designed for future-forward applications.

### Overview of New Capabilities

The May 2025 extensions include:

- **🎨 Image Generation Tools**: AI-powered visual content creation with container support
- **🔌 MCP Integration**: Model Context Protocol for external knowledge sources
- **🧠 Enhanced Reasoning**: Access to AI reasoning processes and encrypted content
- **🔒 Type-Safe Includes**: Compile-time validation for include options
- **📸 Enhanced Streaming**: Image progress events and advanced streaming capabilities

These features are production-ready and have been tested for reliability and performance.

## Image Generation

The Image Generation tool allows you to create AI-powered visual content directly through the Responses API.

### Basic Image Generation

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model, Tool, Container};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    // Create an image generation tool
    let image_tool = Tool::image_generation(None); // No container restrictions
    
    let request = Request::builder()
        .model(Model::GPT4o)
        .input("Create a beautiful sunset over a mountain range")
        .tools(vec![image_tool])
        .build();
    
    let response = client.responses.create(request).await?;
    println!("Response: {}", response.output_text());
    
    Ok(())
}
```

### Image Generation with Container Support

For enhanced security and isolation, use containers:

```rust
use open_ai_rust_responses_by_sshift::{Tool, Container};

// Create a container with specific security settings
let container = Container::default_type(); // Uses default container settings

// Create image generation tool with container
let secure_image_tool = Tool::image_generation(Some(container));

let request = Request::builder()
    .model(Model::GPT4o)
    .input("Generate a professional logo for a tech company")
    .tools(vec![secure_image_tool])
    .build();
```

### Streaming Image Generation

Monitor image generation progress in real-time:

```rust
use open_ai_rust_responses_by_sshift::StreamEvent;
use futures::StreamExt;

let request = Request::builder()
    .model(Model::GPT4o)
    .input("Create an abstract art piece")
    .tools(vec![Tool::image_generation(None)])
    .build();

let mut stream = client.responses.stream(request);

while let Some(event) = stream.next().await {
    match event? {
        StreamEvent::ImageProgress { url, index } => {
            if let Some(image_url) = url {
                println!("🎨 Image {} completed: {}", index, image_url);
            } else {
                println!("🎨 Generating image {}...", index);
            }
        }
        StreamEvent::TextDelta { content, .. } => {
            print!("{}", content);
        }
        StreamEvent::Done => break,
        _ => {}
    }
}
```

## MCP Integration

Model Context Protocol (MCP) integration allows you to connect to external knowledge sources and services.

### Basic MCP Setup

```rust
use open_ai_rust_responses_by_sshift::Tool;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    // Create MCP tool for external knowledge server
    let mcp_tool = Tool::mcp(
        "knowledge-base",                           // Server label
        "https://api.knowledge-server.com/v1",     // Server URL
        None                                        // No custom headers
    );
    
    let request = Request::builder()
        .model(Model::GPT4o)
        .input("What's the latest research on quantum computing?")
        .tools(vec![mcp_tool])
        .build();
    
    let response = client.responses.create(request).await?;
    println!("Knowledge-enhanced response: {}", response.output_text());
    
    Ok(())
}
```

### MCP with Authentication

```rust
use std::collections::HashMap;

// Set up authentication headers
let mut headers = HashMap::new();
headers.insert("Authorization".to_string(), "Bearer your-api-token".to_string());
headers.insert("X-Custom-Header".to_string(), "custom-value".to_string());

// Create authenticated MCP tool
let authenticated_mcp = Tool::mcp(
    "secure-knowledge-server",
    "https://secure-api.example.com/v1",
    Some(headers)
);

let request = Request::builder()
    .model(Model::GPT4o)
    .input("Access proprietary research database for latest findings")
    .tools(vec![authenticated_mcp])
    .build();
```

### Multiple MCP Sources

```rust
// Connect to multiple knowledge sources
let research_mcp = Tool::mcp("research-db", "https://research.example.com/v1", None);
let news_mcp = Tool::mcp("news-feed", "https://news-api.example.com/v1", None);
let docs_mcp = Tool::mcp("documentation", "https://docs.example.com/v1", None);

let request = Request::builder()
    .model(Model::GPT4o)
    .input("Compile a comprehensive report on AI developments")
    .tools(vec![research_mcp, news_mcp, docs_mcp])
    .build();
```

## Enhanced Reasoning

Access the AI's reasoning process for transparency and debugging.

### Reasoning Summary

```rust
use open_ai_rust_responses_by_sshift::types::Include;

let request = Request::builder()
    .model(Model::GPT4o)
    .input("Solve this complex mathematical problem: optimize f(x) = x^3 - 6x^2 + 9x + 1")
    .include(vec![Include::ReasoningSummary])
    .build();

let response = client.responses.create(request).await?;

// Access reasoning information
if let Some(reasoning) = response.reasoning_summary() {
    println!("AI Reasoning Process:");
    println!("{}", reasoning);
}

println!("\nFinal Answer:");
println!("{}", response.output_text());
```

### Encrypted Reasoning Content

For sensitive reasoning processes:

```rust
let request = Request::builder()
    .model(Model::GPT4o)
    .input("Analyze this confidential business strategy")
    .include(vec![
        Include::ReasoningSummary,
        Include::ReasoningEncryptedContent
    ])
    .build();

let response = client.responses.create(request).await?;

// Access encrypted reasoning data
if let Some(encrypted_reasoning) = response.reasoning_encrypted_content() {
    println!("Encrypted reasoning data available: {} bytes", encrypted_reasoning.len());
    // Decrypt using your preferred encryption library
}
```

### Combined Reasoning and Results

```rust
let request = Request::builder()
    .model(Model::GPT4o)
    .input("Design a secure authentication system")
    .include(vec![
        Include::ReasoningSummary,
        Include::ReasoningEncryptedContent,
        Include::FileSearchResults,
    ])
    .build();

let response = client.responses.create(request).await?;

println!("Reasoning: {:?}", response.reasoning_summary());
println!("File Results: {:?}", response.file_search_results());
println!("Answer: {}", response.output_text());
```

## Type-Safe Includes

The new `Include` enum provides compile-time validation and better IDE support.

### Migration from String-Based Includes

```rust
// Old way (still supported for backward compatibility)
let request = Request::builder()
    .model(Model::GPT4o)
    .input("Search for information")
    .include_strings(vec!["file_search_results".to_string()])
    .build();

// New way (recommended, type-safe)
use open_ai_rust_responses_by_sshift::types::Include;

let request = Request::builder()
    .model(Model::GPT4o)
    .input("Search for information")
    .include(vec![Include::FileSearchResults])
    .build();
```

### All Available Include Options

```rust
use open_ai_rust_responses_by_sshift::types::Include;

let request = Request::builder()
    .model(Model::GPT4o)
    .input("Comprehensive analysis")
    .include(vec![
        Include::FileSearchResults,         // Search results from file operations
        Include::ReasoningSummary,          // NEW: AI reasoning process summary
        Include::ReasoningEncryptedContent, // NEW: Encrypted reasoning data
    ])
    .build();
```

### Type Safety Benefits

```rust
// Compile-time error prevention
let request = Request::builder()
    .model(Model::GPT4o)
    .input("Test")
    .include(vec![
        Include::FileSearchResults,
        // Include::InvalidOption,  // This would cause a compile error!
    ])
    .build();

// IDE autocompletion and documentation
let includes = vec![
    Include::FileSearchResults,      // Shows documentation in IDE
    Include::ReasoningSummary,       // Auto-completes available options
];
```
