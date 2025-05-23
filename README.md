# OpenAI Rust Responses by SShift

[![CI](https://github.com/Singularity-Shift/openai-rust-responses-sshift/workflows/CI/badge.svg)](https://github.com/Singularity-Shift/openai-rust-responses-sshift/actions)
[![Crates.io](https://img.shields.io/crates/v/open-ai-rust-responses-by-sshift.svg)](https://crates.io/crates/open-ai-rust-responses-by-sshift)
[![Documentation](https://docs.rs/open-ai-rust-responses-by-sshift/badge.svg)](https://docs.rs/open-ai-rust-responses-by-sshift)

A comprehensive, async Rust SDK for the OpenAI Responses API that provides full access to conversation continuity, streaming responses, and advanced features.

## ✨ Features

- **🔄 Conversation Continuity**: Use response IDs to maintain conversation context
- **🌊 Streaming Support**: Real-time SSE streaming with `futures::Stream`
- **📁 File Operations**: Upload, download, and manage files
- **🔍 Vector Stores**: Semantic search and knowledge retrieval
- **🛠️ Built-in Tools**: Web search, file search, and custom function calling
- **⚡ Async/Await**: Built on `tokio` and `reqwest` for high performance
- **🔒 Type Safety**: Comprehensive error handling and type definitions
- **📚 Rich Documentation**: Extensive examples and API documentation

## 🚀 Quick Start

### 30-Second Demo

Want to try it right now? 

```bash
# Add to Cargo.toml
cargo add open-ai-rust-responses-by-sshift tokio --features tokio/full

# Set your API key
export OPENAI_API_KEY=sk-your-api-key

# Run the comprehensive demo
cargo run --example comprehensive_demo --features stream
```

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
use open_ai_rust_responses_by_sshift::{Client, Request, Model, StreamEvent};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    let request = Request::builder()
        .model(Model::GPT4o)
        .input("Tell me a story about a robot.")
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

Check out the `examples/` directory for comprehensive examples:

- [`basic.rs`](examples/basic.rs) - Simple request/response
- [`conversation.rs`](examples/conversation.rs) - Multi-turn conversations  
- [`streaming.rs`](examples/streaming.rs) - Real-time streaming
- [`comprehensive_demo.rs`](examples/comprehensive_demo.rs) - **Complete feature showcase** (files, vector stores, tools, etc.)

### Quick Start with Full Demo

Create a `.env` file with your API key:
```bash
echo "OPENAI_API_KEY=sk-your-api-key-here" > .env
```

Run the comprehensive demo to see all features:
```bash
cargo run --example comprehensive_demo --features stream
```

**This demo showcases ALL major features:**
- 🔄 **Conversation Continuity** - Response ID linking
- 🌊 **Streaming Responses** - Real-time text generation  
- 📁 **File Operations** - Upload, download, delete
- 🔍 **Vector Stores** - Semantic search and knowledge retrieval
- 🌐 **Web Search Tool** - Built-in web searching capability
- 📄 **File Search Tool** - Search through uploaded documents
- ⚙️ **Custom Functions** - Define and call custom tools
- 🧪 **Resource Management** - Proper cleanup and deletion testing

Other examples:
```bash
cargo run --example basic
cargo run --example conversation
cargo run --example streaming --features stream
```

## 🎯 API Coverage

This crate provides comprehensive coverage of the OpenAI Responses API:

| Feature | Status | Notes |
|---------|---------|--------|
| Responses | ✅ | Create, retrieve, cancel, delete |
| Streaming | ✅ | Server-sent events with `futures::Stream` |
| Conversation Continuity | ✅ | Response ID linking |
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

## 🧪 Testing

To run the test suite:

```bash
# Run unit and integration tests
cargo test

# Run tests with all features
cargo test --all-features

# Run integration tests that need API key (streaming, actual API calls)
OPENAI_API_KEY=sk-your-key cargo test --features stream -- --ignored --nocapture

# Run the comprehensive demo (requires API key)
OPENAI_API_KEY=sk-your-key cargo run --example comprehensive_demo --features stream
```

### Streaming Test Output

The `--nocapture` flag is important for streaming tests because it allows you to see the real-time streaming output. The streaming test will show:

```bash
🌊 Starting streaming test...
📖 Response: 1, 2, 3, 4, 5...
✅ Stream completed!
📊 Test results:
   Events received: 12
   Content length: 45 characters
```

For detailed test coverage and results, see [TEST_REPORT.md](./TEST_REPORT.md).

## 🔧 Troubleshooting

### Tests Show "ignored" - Is This an Error?

**No!** ✅ Tests marked `ignored` are **intentional**:
- `ignored` = Integration tests that need API keys (expensive/slow)
- Regular tests = Unit tests (fast, no API needed)
- Use `--ignored` flag to run integration tests when you have an API key

### Not Seeing Streaming Output?

Make sure to use both flags:
```bash
cargo test test_create_stream --features stream -- --ignored --nocapture
#                                               ^^^^^^^^^ ^^^^^^^^^
#                                               run ignored  show output
```

### API Key Issues?

```bash
# Check if set
echo $OPENAI_API_KEY

# Set for current session
export OPENAI_API_KEY=sk-your-api-key

# Or use .env file
echo "OPENAI_API_KEY=sk-your-api-key" > .env
```

## 📖 Documentation

- [API Documentation](https://docs.rs/open-ai-rust-responses-by-sshift)
- [Examples](./examples/)
- [Detailed Documentation](./DOCUMENTATION.md)
- [Test Report](./TEST_REPORT.md)

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](./CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [tokio](https://tokio.rs/) and [reqwest](https://github.com/seanmonstar/reqwest)
- Inspired by the official OpenAI Python client
- Thanks to the Rust community for excellent async ecosystem
