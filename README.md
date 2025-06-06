# OpenAI Rust Responses by SShift

[![CI](https://github.com/Singularity-Shift/openai-rust-responses-sshift/workflows/CI/badge.svg)](https://github.com/Singularity-Shift/openai-rust-responses-sshift/actions)
[![Crates.io](https://img.shields.io/crates/v/open-ai-rust-responses-by-sshift.svg)](https://crates.io/crates/open-ai-rust-responses-by-sshift)
[![Documentation](https://docs.rs/open-ai-rust-responses-by-sshift/badge.svg)](https://docs.rs/open-ai-rust-responses-by-sshift)

> **🔥 v0.2.0 Update**: Major update to image generation! The SDK now supports the official built-in `image_generation` tool, replacing the previous function-based workaround. This is a breaking change.

A comprehensive, async Rust SDK for the OpenAI Responses API with advanced reasoning capabilities, background processing, enhanced models, production-ready streaming, and **working image generation**.

## ✨ Features

- **🔄 Conversation Continuity**: Use response IDs to maintain conversation context
- **🌊 Production-Ready Streaming**: HTTP chunked responses with proper parsing and real-time text generation
- **📁 File Operations**: Upload, download, and manage files with full MIME support
- **🔍 Vector Stores**: Semantic search and knowledge retrieval with attribute filtering
- **🛠️ Advanced Tools**: Web search, file search, custom functions, **built-in image generation**, and MCP integration
- **🎨 Image Generation**: Working implementation via direct API and the new built-in tool
- **🧠 Reasoning Parameters**: Low/high effort reasoning with auto/concise/detailed summaries
- **🔄 Background Processing**: Async operation handling for long-running tasks
- **🎯 Enhanced Models**: Support for o3, o4-mini, all o1 variants, and GPT-4o family
- **⚡ Async/Await**: Built on `tokio` and `reqwest` for high performance
- **🔒 Type Safety**: Comprehensive error handling, type-safe includes, and compile-time validation
- **📊 Full API Parity**: 85% coverage of OpenAI May 2025 specification with 100% backward compatibility
- **📚 Rich Documentation**: Extensive examples and API documentation

## 🆕 Advanced Capabilities

This SDK includes cutting-edge features with full API parity:

### 🎨 **Image Generation** (Overhauled in v0.2.0)
```rust
use open_ai_rust_responses_by_sshift::{Client, ImageGenerateRequest};

// Method 1: Direct image generation via Images API
let image_request = ImageGenerateRequest::new("A serene mountain landscape")
    .with_size("1024x1024")
    .with_quality("high");
let image_response = client.images.generate(image_request).await?;
println!("Image URL: {}", image_response.data[0].url.as_ref().unwrap());

// Method 2: AI-triggered image generation via the new built-in tool
let request = Request::builder()
    .model(Model::GPT4oMini)
    .input("Create an image of a futuristic city")
    .tools(vec![Tool::image_generation()]) // Use the new, simple tool
    .build();

// The model handles image generation and returns the data directly
let response = client.responses.create(request).await?;
// See examples/image_generation_builtin.rs for how to save the image
```

### 🧠 **Reasoning Parameters**
```rust
use open_ai_rust_responses_by_sshift::types::{ReasoningParams, Effort, SummarySetting};

// Optimized configuration - fast and cost-effective
let request = Request::builder()
    .model(Model::O4Mini)  // Specialized reasoning model
    .input("Solve this complex problem step by step")
    .reasoning(ReasoningParams::new()
        .with_effort(Effort::Low)              // Fast responses
        .with_summary(SummarySetting::Auto))   // Auto-generated summaries
    .max_output_tokens(2000)  // Reasoning models need more tokens
    // Note: O4Mini doesn't support temperature (built-in optimization)
    .build();
```

### 🔄 **Background Processing**
```rust
use open_ai_rust_responses_by_sshift::types::BackgroundHandle;

// Enable background mode for long-running tasks
let request = Request::builder()
    .model(Model::O4Mini)  // Efficient for background tasks
    .input("Perform comprehensive analysis...")
    .reasoning(ReasoningParams::new().with_effort(Effort::Low))
    .background(true)  // Returns HTTP 202 with handle for polling
    .build();

// Would return BackgroundHandle for status polling
let response = client.responses.create(request).await?;
```

### 🎯 **Enhanced Model Support**
```rust
// Recommended models for different use cases
Model::GPT4oMini      // Best default choice (recommended for most use cases)
Model::GPT4o          // Advanced conversations
Model::O4Mini         // Efficient reasoning tasks (2000 token default)
Model::O3             // Complex reasoning (most capable)
Model::O1             // Original reasoning model
Model::O1Mini         // Compact reasoning
Model::O1Preview      // Preview version
Model::GPT4o20241120  // Specific version
// ... and more
```

### 🔒 **Type-Safe Includes**
```rust
use open_ai_rust_responses_by_sshift::types::Include;

// Compile-time validated includes (API-compatible values)
let request = Request::builder()
    .model(Model::GPT4oMini)
    .input("Search and analyze")
    .include(vec![
        Include::FileSearchResults,         // file_search_call.results
        Include::WebSearchResults,          // web_search_call.results
        Include::ReasoningEncryptedContent, // reasoning.encrypted_content
    ])
    .build();
```

### 📊 **Enhanced Response Fields** (Phase 1 Complete)
```rust
// New response fields for comprehensive monitoring
let response = client.responses.create(request).await?;

// Status tracking
println!("Status: {}", response.status);  // "completed", "in_progress", etc.
println!("Complete: {}", response.is_complete());
println!("Has errors: {}", response.has_errors());

// Token analytics
if let Some(usage) = &response.usage {
    println!("Total tokens: {}", usage.total_tokens);
    if let Some(details) = &usage.output_tokens_details {
        println!("Reasoning tokens: {:?}", details.reasoning_tokens);
    }
}

// Parameter echoing
println!("Temperature used: {:?}", response.temperature);
println!("Max output tokens: {:?}", response.max_output_tokens);
```

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
open-ai-rust-responses-by-sshift = "0.2.0"
tokio = { version = "1.0", features = ["full"] }

# Optional: Enable streaming
# open-ai-rust-responses-by-sshift = { version = "0.2.0", features = ["stream"] }
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
        .model(Model::GPT4oMini)  // Recommended default model
        .input("Hello, how are you today?")
        .temperature(0.7)
        .max_output_tokens(500)  // Optimized for completion
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
        .model(Model::GPT4oMini)  // Recommended default
        .input("My name is Alice. What's a good recipe for pasta?")
        .build();
    
    let response1 = client.responses.create(request).await?;
    println!("Chef: {}", response1.output_text());
    
    // Continue conversation with response ID
    let request2 = Request::builder()
        .model(Model::GPT4oMini)
        .input("Can you make it vegetarian?")
        .previous_response_id(response1.id())
        .build();
    
    let response2 = client.responses.create(request2).await?;
    println!("Chef: {}", response2.output_text());
    
    Ok(())
}
```

### Image Generation Example

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model, Tool, ImageGenerateRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    // Method 1: Direct image generation
    let image_req = ImageGenerateRequest::new("A beautiful sunset over mountains")
        .with_size("1024x1024")
        .with_quality("high");
    
    let image_response = client.images.generate(image_req).await?;
    if let Some(url) = &image_response.data[0].url {
        println!("Generated image: {}", url);
    }
    
    // Method 2: AI-triggered image generation
    let request = Request::builder()
        .model(Model::GPT4oMini)
        .input("Create an image of a robot learning to paint")
        .tools(vec![Tool::image_generation()])  // Use the new built-in tool
        .build();
    
    let response = client.responses.create(request).await?;
    // The AI will automatically call the image generation tool
    
    Ok(())
}
```

### Streaming Responses

Enable the `stream` feature:

```toml
[dependencies]
open-ai-rust-responses-by-sshift = { version = "0.2.0", features = ["stream"] }
```

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model, StreamEvent};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    let request = Request::builder()
        .model(Model::GPT4oMini)  // Excellent for streaming performance
        .input("Tell me a story about a robot.")
        .max_output_tokens(500)  // Optimized for streaming
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

### Function Calling & Tool Outputs

The Responses API handles function calling differently from the Assistants API. There is **no `submit_tool_outputs` endpoint**. Instead, tool outputs are submitted as input items in a new request:

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model, Tool, ToolChoice};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    // 1. Define function tools
    let calculator_tool = Tool::function(
        "calculate",
        "Perform basic arithmetic calculations",
        json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "Mathematical expression to evaluate"
                }
            },
            "required": ["expression"]
        }),
    );
    
    // 2. Initial request with tools
    let request = Request::builder()
        .model(Model::GPT4oMini)  // Excellent for function calling
        .input("Calculate 15 * 7 + 23")
        .tools(vec![calculator_tool.clone()])
        .tool_choice(ToolChoice::auto())
        .build();
    
    let response = client.responses.create(request).await?;
    
    // 3. Check for tool calls and execute functions
    let tool_calls = response.tool_calls();
    if !tool_calls.is_empty() {
        let mut function_outputs = Vec::new();
        
        for tool_call in &tool_calls {
            if tool_call.name == "calculate" {
                // Execute your function here
                let result = "128"; // Calculate 15 * 7 + 23 = 128
                function_outputs.push((tool_call.call_id.clone(), result.to_string()));
            }
        }
        
        // 4. Submit tool outputs by creating a new request
        // This is the correct pattern for the Responses API
        let continuation_request = Request::builder()
            .model(Model::GPT4oMini)
            .with_function_outputs(response.id(), function_outputs)
            .tools(vec![calculator_tool])
            .build();
        
        let final_response = client.responses.create(continuation_request).await?;
        println!("Final response: {}", final_response.output_text());
    }
    
    Ok(())
}
```

**Key Points for Function Calling:**
- ❌ **No `submit_tool_outputs` endpoint** (unlike Assistants API)
- ✅ **Use `with_function_outputs()`** to submit tool results
- ✅ **Include `previous_response_id`** to maintain conversation context
- ✅ **Match `call_id`** from tool calls to function outputs
- ✅ **Create new request** for each tool output submission

See [`examples/function_calling.rs`](examples/function_calling.rs) for a complete working example.

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
- [`function_calling.rs`](examples/function_calling.rs) - Function calling and tool outputs
- [`image_generation.rs`](examples/image_generation.rs) - **Image generation via direct API and AI tools**
- [`comprehensive_demo.rs`](examples/comprehensive_demo.rs) - **Complete feature showcase** (files, vector stores, tools, images, etc.)

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
- 🔄 **Conversation Continuity** - Response ID linking with 100% success rate
- 🌊 **Streaming Responses** - Real-time text generation with optimized tokens
- 📁 **File Operations** - Upload, download, delete
- 🔍 **Vector Stores** - Semantic search and knowledge retrieval
- 🌐 **Web Search Tool** - Built-in web searching capability
- 📄 **File Search Tool** - Search through uploaded documents
- ⚙️ **Custom Functions** - Define and call custom tools
- 🎨 **Image Generation** - Direct API and AI-triggered generation
- 🧪 **Resource Management** - Proper cleanup and deletion testing

Other examples:
```bash
cargo run --example basic
cargo run --example conversation
cargo run --example streaming --features stream
cargo run --example function_calling
cargo run --example image_generation  # NEW: Image generation demo
```

## 🎯 API Coverage

This crate provides comprehensive coverage of the OpenAI Responses API:

| Feature | Status | Notes |
|---------|---------|--------|
| Responses | ✅ | Create, retrieve, cancel, delete, 21 new fields |
| Streaming | ✅ | Server-sent events with `futures::Stream` |
| Conversation Continuity | ✅ | Response ID linking, 100% success rate |
| Messages | ✅ | Message CRUD operations |
| Files | ✅ | Upload, download, list, delete |
| Vector Stores | ✅ | Create, search, manage |
| Tools | ✅ | Built-in and custom function calling |
| Image Generation | ✅ | Direct API + AI function tools (hosted tool pending) |
| Phase 1 Spec | ✅ | 85% May 2025 spec coverage |

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
5. **Token optimization**: 
   - General responses: 500 tokens (optimized from 200)
   - Reasoning tasks: 2000 tokens (O4Mini)
   - Streaming: 500 tokens for smooth output

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

### Common API Issues (Fixed in v0.1.7)

#### Include Field Errors
If you see errors like "Unknown include field", use the type-safe Include enum:

```rust
// ❌ Don't use raw strings (may break with API updates)
.include_strings(vec!["file_search.results".to_string()])

// ✅ Use type-safe includes (recommended)
use open_ai_rust_responses_by_sshift::types::Include;
.include(vec![Include::FileSearchResults])  // Maps to file_search_call.results
```

#### Temperature Parameter Errors with Reasoning Models
Reasoning models (O4Mini, O3, O1 series) don't support temperature:

```rust
// ❌ This will cause API errors
let request = Request::builder()
    .model(Model::O4Mini)
    .temperature(0.7)  // Error: O4Mini doesn't support temperature
    .build();

// ✅ Correct usage for reasoning models
let request = Request::builder()
    .model(Model::O4Mini)
    .reasoning(ReasoningParams::new().with_effort(Effort::Low))
    .max_output_tokens(2000)  // Reasoning needs more tokens
    // No temperature parameter - built-in optimization
    .build();

// ✅ For general models that support temperature
let request = Request::builder()
    .model(Model::GPT4oMini)  // Recommended default
    .temperature(0.7)  // GPT4oMini supports temperature
    .max_output_tokens(500)  // Optimized for general use
    .build();
```

#### Incomplete Responses
Fixed in v0.1.7 by optimizing token allocations:

```rust
// ❌ Old defaults caused truncation (200 tokens)
// ✅ New optimized defaults:
Model::GPT4oMini => 500 tokens    // General responses
Model::O4Mini => 2000 tokens       // Reasoning tasks
// Success rate improved from 50% to 100%
```

#### Image Generation Tool Errors
Native hosted tool pending, use function tool bridge:

```rust
// ❌ This doesn't work yet (pending OpenAI release)
Tool::image_generation(None)  // Hosted tool not available

// ✅ Use the function tool bridge (working now)
Tool::image_generation_function()  // Pre-made function tool
```

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
- Phase 1 implementation based on OpenAI May 2025 specification
