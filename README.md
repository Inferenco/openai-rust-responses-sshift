# OpenAI Rust Responses by SShift

[![CI](https://github.com/Singularity-Shift/openai-rust-responses-sshift/workflows/CI/badge.svg)](https://github.com/Singularity-Shift/openai-rust-responses-sshift/actions)
[![Crates.io](https://img.shields.io/crates/v/open-ai-rust-responses-by-sshift.svg)](https://crates.io/crates/open-ai-rust-responses-by-sshift)
[![Documentation](https://docs.rs/open-ai-rust-responses-by-sshift/badge.svg)](https://docs.rs/open-ai-rust-responses-by-sshift)

A comprehensive, async Rust SDK for the OpenAI Responses API that provides full access to conversation continuity, streaming responses, and advanced features.

## ✨ Features

- **🔄 Conversation Continuity**: Use response IDs to maintain conversation context
- **🌊 Streaming Support**: Real-time SSE streaming with `futures::Stream`
- **🧵 Thread Management**: Organized conversation threads
- **📁 File Operations**: Upload, download, and manage files
- **🔍 Vector Stores**: Semantic search and knowledge retrieval
- **🛠️ Built-in Tools**: Web search, file search, and custom function calling
- **⚡ Async/Await**: Built on `tokio` and `reqwest` for high performance
- **🔒 Type Safety**: Comprehensive error handling and type definitions
- **📚 Rich Documentation**: Extensive examples and API documentation

## 🚀 Quick Start

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
open-ai-rust-responses-by-sshift = "0.1"
tokio = { version = "1.0", features = ["full"] }

# Optional: Enable streaming
# open-ai-rust-responses-by-sshift = { version = "0.1", features = ["stream"] }
```

### Basic Usage

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model, Input};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with API key
    let client = Client::new("sk-your-api-key")?;
    
    // Or use environment variable
    let client = Client::from_env()?;
    
    // Create a simple request
    let request = Request::builder()
        .model(Model::GPT4o)
        .input("Hello, how are you today?")
        .max_tokens(100)
        .temperature(0.7)
        .build();
    
    // Get response
    let response = client.responses.create(request).await?;
    println!("Response: {}", response.output_text());
    
    Ok(())
}
```

### Conversation Continuity

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    // First message
    let request = Request::builder()
        .model(Model::GPT4o)
        .input("My name is Alice. What's a good recipe for pasta?")
        .build();
    
    let response1 = client.responses.create(request).await?;
    println!("Chef: {}", response1.output_text());
    
    // Continue conversation with response ID
    let request2 = Request::builder()
        .model(Model::GPT4o)
        .input("Can you make it vegetarian?")
        .previous_response_id(response1.id())
        .build();
    
    let response2 = client.responses.create(request2).await?;
    println!("Chef: {}", response2.output_text());
    
    Ok(())
}
```

### Streaming Responses

Enable the `stream` feature:

```toml
[dependencies]
open-ai-rust-responses-by-sshift = { version = "0.1", features = ["stream"] }
```

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    let request = Request::builder()
        .model(Model::GPT4o)
        .input("Tell me a story about a robot.")
        .max_tokens(200)
        .build();
    
    let mut stream = client.responses.stream(request);
    
    while let Some(event) = stream.next().await {
        match event? {
            StreamEvent::TextDelta { content, .. } => {
                print!("{}", content);
            }
            StreamEvent::Done => break,
            _ => {}
        }
    }
    
    Ok(())
}
```

### Thread Management

```rust
use open_ai_rust_responses_by_sshift::{Client, Model};
use open_ai_rust_responses_by_sshift::threads::CreateThreadRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    // Create a new thread
    let thread_request = CreateThreadRequest {
        model: Model::GPT4o,
        instructions: Some("You are a helpful cooking assistant.".to_string()),
        initial_message: "Hello! I need help with dinner ideas.".to_string(),
        metadata: None,
    };
    
    let (thread, initial_response) = client.threads.create(thread_request).await?;
    println!("Assistant: {}", initial_response.output_text());
    
    // Continue the conversation
    let (updated_thread, response) = client.threads
        .continue_thread(&thread, Model::GPT4o, "I'm vegetarian.")
        .await?;
    
    println!("Assistant: {}", response.output_text());
    
    Ok(())
}
```

### File Operations

```rust
use open_ai_rust_responses_by_sshift::Client;
use open_ai_rust_responses_by_sshift::files::FilePurpose;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    // Upload a file
    let file = client.files
        .upload_file("./data/document.pdf", FilePurpose::Assistants, None)
        .await?;
    
    println!("Uploaded file: {} ({})", file.filename, file.id);
    
    // List files
    let files = client.files.list(None).await?;
    println!("You have {} files", files.len());
    
    // Download file content
    let content = client.files.download(&file.id).await?;
    println!("Downloaded {} bytes", content.len());
    
    Ok(())
}
```

## 🔧 Configuration

### Environment Variables

```bash
# Required
OPENAI_API_KEY=sk-your-api-key-here

# Optional
OPENAI_BASE_URL=https://api.openai.com/v1  # Custom base URL
OPENAI_ORG_ID=org-your-organization-id     # Organization ID
```

### Custom Configuration

```rust
use open_ai_rust_responses_by_sshift::{Client, Config};

let config = Config::new("sk-your-api-key")
    .with_base_url("https://api.openai.com/v1")
    .with_organization_id("org-your-org-id");

let client = Client::new_with_config(config)?;
```

## 📊 Examples

Check out the `examples/` directory for more comprehensive examples:

- [`basic.rs`](examples/basic.rs) - Simple request/response
- [`conversation.rs`](examples/conversation.rs) - Multi-turn conversations
- [`streaming.rs`](examples/streaming.rs) - Real-time streaming
- [`threads.rs`](examples/threads.rs) - Thread management
- [`files.rs`](examples/files.rs) - File operations
- [`tools.rs`](examples/tools.rs) - Function calling

Run examples with:

```bash
cargo run --example basic
cargo run --example streaming --features stream
```

## 🎯 API Coverage

This crate provides comprehensive coverage of the OpenAI Responses API:

| Feature | Status | Notes |
|---------|---------|--------|
| Responses | ✅ | Create, retrieve, cancel, delete |
| Streaming | ✅ | Server-sent events with `futures::Stream` |
| Conversation Continuity | ✅ | Response ID linking |
| Threads | ✅ | Thread management and continuation |
| Messages | ✅ | Message CRUD operations |
| Files | ✅ | Upload, download, list, delete |
| Vector Stores | ✅ | Create, search, manage |
| Tools | ✅ | Built-in and custom function calling |

## 🚦 Error Handling

The crate uses comprehensive error types:

```rust
use open_ai_rust_responses_by_sshift::{Client, Error};

match client.responses.create(request).await {
    Ok(response) => println!("Success: {}", response.output_text()),
    Err(Error::Api { message, error_type, code }) => {
        eprintln!("API Error: {} ({})", message, error_type);
    }
    Err(Error::Http(e)) => {
        eprintln!("HTTP Error: {}", e);
    }
    Err(Error::Json(e)) => {
        eprintln!("JSON Error: {}", e);
    }
    Err(Error::Stream(msg)) => {
        eprintln!("Stream Error: {}", msg);
    }
}
```

## ⚡ Performance Tips

1. **Reuse the client**: `Client` is designed to be reused across requests
2. **Connection pooling**: The underlying `reqwest` client pools connections automatically
3. **Streaming**: Use streaming for long responses to get results faster
4. **Async**: Always use in an async context for best performance

## 🔐 Security

- API keys are never logged or exposed in error messages
- All requests use HTTPS by default
- Supports custom certificate validation
- Environment variable support for secure key management

## 📖 Documentation

- [API Documentation](https://docs.rs/open-ai-rust-responses-by-sshift)
- [Examples](./examples/)
- [Changelog](./CHANGELOG.md)

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](./CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [tokio](https://tokio.rs/) and [reqwest](https://github.com/seanmonstar/reqwest)
- Inspired by the official OpenAI Python client
- Thanks to the Rust community for excellent async ecosystem
