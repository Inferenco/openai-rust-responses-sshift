# Open AI Rust Responses by SShift - Documentation

> **🚀 v0.3.0 Update**: **GPT‑5 support** (flagship, mini, nano), new `Verbosity` control, `ReasoningEffort` tuning for GPT‑5, structured/free‑form function improvements, and a richer example. Note: minor source‑level break for struct literals and exhaustive `Model` matches.
> **🔧 v0.2.9 Update**: **Automatic Tool Usage Tracking** - Revolutionary analytics! SDK now automatically tracks built-in tool usage (web search, file search, image generation, code interpreter) with zero client changes. Get detailed usage statistics in your exact requested format. Zero breaking changes!
> **🛡️ v0.2.5 Update**: **Advanced Container Recovery System** - Revolutionary error handling! SDK now automatically handles container expiration with configurable recovery policies. Choose from Default (auto-retry), Conservative (manual control), or Aggressive (maximum resilience) strategies. Zero breaking changes!
> **🎨 v0.2.4 Update**: **Image-Guided Generation** - Revolutionary new feature! Use input images to guide image generation with the GPT Image 1 model. Create style transfers, combine multiple images into logos, and generate artistic interpretations. See the comprehensive new example!
> **🧑‍💻 v0.2.3 Update**: Code Interpreter tool support! Run Python code in a secure container and get results directly from the model. See the new example and docs.
> **🔥 v0.2.0 Update**: Major update to image generation! The SDK now supports the official built-in `image_generation` tool, replacing the previous function-based workaround. This is a breaking change.
> **🎉 v0.2.1 Update**: Vision input (image understanding) now supported! Use `input_image_url(...)` to send pictures to GPT-4o and get rich descriptions back.
> **🚀 v0.2.2 Update**: Multi-image vision support added! Use `input_image_urls(&[...])` or chain `push_image_url()` for albums and comparisons.

This document provides comprehensive documentation for the Open AI Rust Responses by SShift library, a Rust SDK for the OpenAI Responses API with advanced reasoning parameters, background processing, enhanced models, production-ready streaming, **working image generation**, **automatic tool usage tracking**, and **revolutionary image-guided generation**.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Installation](#installation)
3. [Authentication](#authentication)
4. [Basic Usage](#basic-usage)
5. [GPT‑5 Usage](#gpt5-usage)
6. [Tool Usage Tracking](#tool-usage-tracking)
7. [Responses API](#responses-api)
8. [Advanced Container Recovery System](#advanced-container-recovery-system)
9. [Messages API](#messages-api)
10. [Files API](#files-api)
11. [Vector Stores API](#vector-stores-api)
12. [Tools API](#tools-api)
13. [Image-Guided Generation](#image-guided-generation)
14. [Image Generation](#image-generation)
15. [Image Input (Vision)](#image-input-vision)
16. [Code Interpreter](#code-interpreter)
17. [Function Calling & Tool Outputs](#function-calling--tool-outputs)
18. [Reasoning Parameters](#reasoning-parameters)
19. [Model Parameter Compatibility and Requirements](#model-parameter-compatibility-and-requirements)
20. [Background Processing](#background-processing)
21. [Enhanced Models](#enhanced-models)
22. [Type-Safe Includes](#type-safe-includes)
23. [Enhanced Response Fields](#enhanced-response-fields)
24. [Production-Ready Streaming](#production-ready-streaming)
25. [Error Handling](#error-handling)
26. [Advanced Configuration](#advanced-configuration)
27. [Feature Flags](#feature-flags)
28. [Testing and Examples](#testing-and-examples)
29. [MCP Integration (Model Context Protocol)](#mcp-integration-model-context-protocol)

## Quick Start

Get up and running in under a minute:

```bash
# 1. Add to your project
cargo add open-ai-rust-responses-by-sshift tokio --features tokio/full

# 2. Set API key
export OPENAI_API_KEY=sk-your-api-key-here

# 3. Try the comprehensive demo with working streaming
cargo run --example comprehensive_demo --features stream
```

Or create a simple example:

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    let request = Request::builder()
        .model(Model::GPT4oMini)  // Optimized default model (cost-effective)
        .input("What is Rust programming language?")
        .max_output_tokens(500)   // Optimized for completion (was 200)
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
open-ai-rust-responses-by-sshift = "0.3.0"
```

If you want to use streaming responses, make sure to include the `stream` feature (enabled by default):

```toml
[dependencies]
open-ai-rust-responses-by-sshift = { version = "0.3.0", features = ["stream"] }
```

## GPT‑5 Usage

GPT‑5 can be used as a standard model or with explicit reasoning control.

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model, ReasoningEffort, Verbosity};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;

    // Concise general generation
    let concise = Request::builder()
        .model(Model::GPT5Nano)
        .input("Explain Raft in 3 bullets")
        .verbosity(Verbosity::Low)
        .build();

    // Reasoning depth control
    let deep = Request::builder()
        .model(Model::GPT5)
        .input("Design a blue/green deployment strategy with rollback")
        .reasoning_effort(ReasoningEffort::High)
        .build();

    let _ = client.responses.create(concise).await?;
    let _ = client.responses.create(deep).await?;
    Ok(())
}
```

See `examples/gpt5_demo.rs` for a full walkthrough including structured tool calling.

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
        .model(Model::GPT4oMini)  // Recommended default model
        .input("What is the capital of France?")
        .max_output_tokens(500)   // Optimized for better completion
        .build();
    
    // Send the request
    let response = client.responses.create(request).await?;
    
    // Print the response
    println!("Response: {}", response.output_text());
    
    Ok(())
}
```

### 🔧 **Advanced Usage Patterns**

#### **Enhanced Usage Object**
```rust
// Access enhanced usage object with tool counts
if let Some(usage) = response.usage_with_tools() {
    println!("Token usage:");
    println!("  Input tokens: {}", usage.input_tokens);
    println!("  Output tokens: {}", usage.output_tokens);
    println!("  Total tokens: {}", usage.total_tokens);
    
    println!("Tool usage:");
    if let Some(web_count) = usage.web_search {
        println!("  Web searches: {}", web_count);
    }
    if let Some(image_count) = usage.image_generation {
        println!("  Images generated: {}", image_count);
    }
    if let Some(code_count) = usage.code_interpreter {
        println!("  Code executions: {}", code_count);
    }
    if let Some(file_count) = usage.file_search {
        println!("  File searches: {}", file_count);
    }
}
```

#### **Raw Tool Counts**
```rust
// Get raw counts for custom formatting
let (web_search, file_search, image_generation, code_interpreter) = response.calculate_tool_usage();

println!("Custom format:");
println!("Web: {}, File: {}, Image: {}, Code: {}", 
    web_search, file_search, image_generation, code_interpreter);
```

#### **Conditional Tool Tracking**
```rust
// Only show tools that were actually used
let (web, file, image, code) = response.calculate_tool_usage();

if web > 0 { println!("🔍 Web searches: {}", web); }
if file > 0 { println!("📄 File searches: {}", file); }
if image > 0 { println!("🎨 Images generated: {}", image); }
if code > 0 { println!("💻 Code executions: {}", code); }
```

### 📊 **Usage Statistics Structure**

The enhanced `Usage` struct now includes:

```rust
pub struct Usage {
    // Existing token fields
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
    
    // New tool usage fields (optional, only included when > 0)
    pub web_search: Option<u32>,
    pub file_search: Option<u32>,
    pub image_generation: Option<u32>,
    pub code_interpreter: Option<u32>,
    
    // Additional token details
    pub output_tokens_details: Option<OutputTokensDetails>,
    pub prompt_tokens_details: Option<PromptTokensDetails>,
}
```

### 🎯 **Response Methods**

Three new methods provide different levels of tool usage access:

| Method | Returns | Use Case |
|--------|---------|----------|
| `format_usage()` | `String` | Exact requested format output |
| `usage_with_tools()` | `Option<Usage>` | Enhanced usage object with tool counts |
| `calculate_tool_usage()` | `(u32, u32, u32, u32)` | Raw counts as tuple |

### 🔍 **Automatic Detection**

The SDK automatically detects these tool usage patterns in response items:

- **`WebSearchCall`** - Web search tool invocations
- **`FileSearchCall`** - File search tool invocations  
- **`ImageGenerationCall`** - Image generation tool invocations
- **`CodeInterpreterCall`** - Code interpreter tool invocations

### 💡 **Key Benefits**

- **🔄 Automatic Operation**: No client-side changes required
- **📊 Detailed Analytics**: Track both token usage and tool utilization
- **🎯 User-Requested Format**: Exact output format as specified
- **💰 Cost Monitoring**: Monitor tool costs alongside token costs
- **📈 Usage Insights**: Understand AI tool utilization patterns
- **🔗 Backward Compatible**: All existing code continues to work unchanged

### 🎯 **Real-World Applications**

**Usage Analytics**:
- Monitor which tools are used most frequently in your application
- Track tool utilization trends over time
- Optimize tool selection based on actual usage patterns

**Cost Tracking**:
- Calculate costs for both token usage and tool invocations
- Allocate costs to different users or projects
- Budget planning with detailed usage breakdowns

**Application Insights**:
- Understand how users interact with AI tools
- Identify bottlenecks and optimization opportunities
- Make data-driven decisions about tool availability

**Billing Integration**:
- Provide detailed usage reports to customers
- Implement usage-based pricing models
- Track costs across different service tiers

### ✅ **Zero Configuration Required**

Tool usage tracking works automatically without any setup:

```rust
// This code automatically includes tool usage tracking
let response = client.responses.create(request).await?;

// All three methods are immediately available:
println!("{}", response.format_usage());                    // Formatted output
let usage = response.usage_with_tools();                     // Enhanced object  
let (w, f, i, c) = response.calculate_tool_usage();          // Raw counts
```

The tool usage tracking feature provides comprehensive analytics while maintaining the exact format you requested, with zero breaking changes to existing code.

## Responses API

The Responses API allows you to create, retrieve, and manage responses.

### Creating a Response

```rust
// Using the builder pattern
let request = Request::builder()
    .model(Model::GPT4oMini)  // Cost-effective default choice
    .input("What is the capital of France?")
    .instructions("Keep your answer brief")
    .temperature(0.7)
    .max_output_tokens(500)   // Optimized from 200
    .store(false)             // Stateless mode (new in v0.1.7)
    .user("user-123")         // User tracking (new in v0.1.7)
    .build();

let response = client.responses.create(request).await?;

// New helper methods (v0.1.7)
println!("Status: {}", response.status);
println!("Complete: {}", response.is_complete());
println!("Has errors: {}", response.has_errors());
if let Some(usage) = &response.usage {
    println!("Total tokens: {}", response.total_tokens());
}
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

## **Advanced Container Recovery System** *(Revolutionary New Feature in v0.2.5)*

**Revolutionary error handling**: The SDK now automatically detects and recovers from expired containers without breaking user flow! This advanced system provides configurable recovery policies, smart context pruning, and transparent error handling for a seamless developer experience.

### 🛡️ **Core Concept**

Container recovery works by:
1. **Automatic Detection**: SDK detects container expiration errors from the API
2. **Smart Recovery**: Automatically retries with cleaned context based on policy
3. **Context Pruning**: Removes expired container references to enable fresh execution
4. **Transparent Operation**: Applications continue working without manual intervention

### 🎯 **Recovery Policies**

Choose the recovery strategy that fits your application:

#### **Default Policy** (Recommended)
Balanced approach with automatic retry:
```rust
use open_ai_rust_responses_by_sshift::{Client, RecoveryPolicy};

// Implicit default policy
let client = Client::new(&api_key)?;

// Explicit default policy
let client = Client::new_with_recovery(&api_key, RecoveryPolicy::default())?;

// Default settings:
// - Auto-retry: enabled (1 attempt)
// - Notifications: disabled
// - Auto-prune: enabled
// - Logging: disabled
```

#### **Conservative Policy**
Full control with manual handling:
```rust
let policy = RecoveryPolicy::conservative();
let client = Client::new_with_recovery(&api_key, policy)?;

// Conservative settings:
// - Auto-retry: disabled
// - Notifications: enabled
// - Auto-prune: disabled
// - Logging: enabled
```

#### **Aggressive Policy**
Maximum resilience with multiple retries:
```rust
let policy = RecoveryPolicy::aggressive();
let client = Client::new_with_recovery(&api_key, policy)?;

// Aggressive settings:
// - Auto-retry: enabled (3 attempts)
// - Notifications: disabled
// - Auto-prune: enabled
// - Custom reset message
// - Logging: enabled
```

### 🔧 **Custom Recovery Policies**

Build your own recovery strategy:
```rust
let custom_policy = RecoveryPolicy::new()
    .with_auto_retry(true)
    .with_max_retries(2)
    .with_notify_on_reset(true)
    .with_auto_prune(true)
    .with_reset_message("Your session was refreshed for optimal performance.")
    .with_logging(true);

let client = Client::new_with_recovery(&api_key, custom_policy)?;
```

### 🚀 **Basic Usage**

#### **Automatic Recovery** (Default Behavior)
```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Tool, Container};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(&api_key)?; // Default policy enabled

    // Make requests normally - expiration handled automatically!
    let request = Request::builder()
        .model("gpt-4o-mini")
        .input("Continue our Python session from earlier...")
        .tools(vec![Tool::code_interpreter(Some(Container::auto_type()))])
        .previous_response_id("resp_123") // May reference expired container
        .build();

    // SDK automatically detects expiration and retries with fresh context
    let response = client.responses.create(request).await?;
    println!("Success: {}", response.output_text());
    Ok(())
}
```

#### **Recovery with Detailed Information**
```rust
use open_ai_rust_responses_by_sshift::ResponseWithRecovery;

// Get detailed recovery information
let response_with_recovery = client.responses.create_with_recovery(request).await?;

if response_with_recovery.had_recovery() {
    println!("🔄 Recovery performed:");
    println!("   - Attempts: {}", response_with_recovery.recovery_info.retry_count);
    println!("   - Successful: {}", response_with_recovery.recovery_info.successful);
    
    if let Some(msg) = response_with_recovery.recovery_message() {
        println!("   - Message: {}", msg);
    }
    
    if let Some(original_error) = &response_with_recovery.recovery_info.original_error {
        println!("   - Original error: {}", original_error);
    }
}

println!("Response: {}", response_with_recovery.response.output_text());
```

### 🧹 **Manual Context Pruning**

For proactive context management:
```rust
// Manually clean expired context before making requests
let cleaned_request = client.responses.prune_expired_context_manual(request);
let response = client.responses.create(cleaned_request).await?;

// Or use in a conversation loop
let mut previous_response_id: Option<String> = None;

for i in 1..=5 {
    let mut request = Request::builder()
        .model("gpt-4o-mini")
        .input(format!("Step {} of our analysis", i))
        .tools(vec![Tool::code_interpreter(Some(Container::auto_type()))])
        .build();

    // Add conversation continuity
    if let Some(ref prev_id) = previous_response_id {
        request.previous_response_id = Some(prev_id.clone());
    }

    // Proactively prune expired context (optional)
    if i > 2 {
        request = client.responses.prune_expired_context_manual(request);
    }

    match client.responses.create(request).await {
        Ok(response) => {
            previous_response_id = Some(response.id().to_string());
            println!("Step {} completed", i);
        }
        Err(e) => {
            println!("Step {} failed: {}", i, e);
            previous_response_id = None; // Reset on error
        }
    }
}
```

### 🔔 **Recovery Callbacks**

Get notified when recovery occurs:
```rust
use open_ai_rust_responses_by_sshift::RecoveryCallback;

let callback: RecoveryCallback = Box::new(|error, attempt| {
    println!("🔔 Recovery callback triggered:");
    println!("   - Error: {}", error);
    println!("   - Attempt: {}", attempt);
    println!("   - Action: Retrying with fresh context...");
});

let client_with_callback = client.responses.clone().with_recovery_callback(callback);

// Now all requests through this client will trigger the callback on recovery
let response = client_with_callback.create_with_recovery(request).await?;
```

### 🎛️ **Advanced Configuration**

#### **Environment-Based Client Creation**
```rust
// From environment with recovery policy
let client = Client::from_env_with_recovery(RecoveryPolicy::aggressive())?;

// With custom base URL and recovery
let client = Client::from_env_with_base_url_and_recovery(
    "https://api.openai.com/v1",
    RecoveryPolicy::conservative()
)?;
```

#### **HTTP Client with Recovery**
```rust
use reqwest::Client as HttpClient;

let http_client = HttpClient::new();
let client = Client::new_with_http_client_and_recovery(
    &http_client,
    "https://api.openai.com/v1",
    RecoveryPolicy::default()
)?;
```

### 🧪 **Testing Container Expiration**

Use the interactive test to see recovery in action:
```bash
cargo run --example container_expiration_test
```

This example:
1. Creates a container with initial code execution
2. Waits for user input (allowing container to expire)
3. Makes a follow-up request that triggers expiration
4. Demonstrates automatic recovery based on chosen policy

### 💡 **Best Practices**

#### **For Chatbots and Interactive Apps**
- Use **Default** or **Aggressive** policy for seamless user experience
- Enable notifications for debugging during development
- Disable notifications in production for clean UX

#### **For Code Execution Apps**
- Use **Aggressive** policy for maximum resilience
- Enable logging for debugging container lifecycle issues
- Consider proactive context pruning for long sessions

#### **For Enterprise Applications**
- Use **Conservative** policy for full control over error handling
- Implement custom recovery callbacks for monitoring and alerting
- Enable detailed logging for operational insights

#### **For Background Processing**
- Use **Aggressive** policy with custom retry counts
- Implement recovery callbacks for job status tracking
- Consider manual context pruning for resource optimization

### 🔍 **Error Detection**

The SDK automatically detects these container expiration patterns:
- `"Container is expired"`
- `"Container expired"`
- `"Session expired"`
- API error types indicating container lifecycle issues

### 📊 **Benefits**

- **🔄 Transparent Recovery**: Container expiration handled automatically
- **⚙️ Configurable Policies**: Choose strategy that fits your application
- **🔍 Detailed Feedback**: Optional recovery information for monitoring
- **🔒 Zero Breaking Changes**: All existing code works with enhanced error handling
- **🎯 Production Ready**: Enterprise-grade error recovery with logging and callbacks
- **🚀 Better UX**: Users experience smooth operation even when containers expire
- **🛠️ Developer Friendly**: Comprehensive examples and clear documentation

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
let delete_response = client.vector_stores.delete_file("vs_abc123", "file_abc123").await?;
println!("Deleted file: {}, Success: {}", delete_response.id, delete_response.deleted);
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

### MCP (Model Context Protocol)

Integrate external knowledge systems via the built-in `mcp` tool. You declare an MCP server as a tool, and the model may call it during a response.

#### Quick Start

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model, Tool};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;

    // Optional: headers for the MCP server (e.g., auth)
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), "Bearer example-token".to_string());

    // Define an MCP server tool
    let mcp_tool = Tool::mcp(
        "knowledge-server",
        "https://api.example.com/v1",
        Some(headers),
    );

    // Include the MCP tool in your request
    let request = Request::builder()
        .model(Model::GPT4oMini)
        .input("Use the external knowledge server to answer succinctly.")
        .tools(vec![mcp_tool])
        .max_output_tokens(500)
        .build();

    let response = client.responses.create(request).await?;
    println!("Answer: {}", response.output_text());
    Ok(())
}
```

#### Approval Modes

MCP tools support an optional `require_approval` setting, communicated to the API:

- `auto`: Default behavior; the platform decides when to prompt for approval.
- `always`: Always require explicit approval before MCP calls.
- `never`: Never require approval.

Use `mcp_with_approval(...)` to set it explicitly:

```rust
use open_ai_rust_responses_by_sshift::Tool;
use std::collections::HashMap;

let mut headers = HashMap::new();
headers.insert("Authorization".to_string(), "Bearer example-token".to_string());

let auto = Tool::mcp("auto-knowledge", "https://api.auto.example/v1", Some(headers.clone()));
let manual = Tool::mcp_with_approval(
    "manual-knowledge",
    "https://api.manual.example/v1",
    "always",
    Some(headers),
);
```

Then pass one or both tools in `.tools([...])` as needed. The SDK serializes `server_label`, `server_url`, `headers`, and `require_approval` into the request.

#### Notes

- Ensure your `server_url` points to a compliant MCP server and that any required headers (e.g., `Authorization`) are provided.
- The SDK does not enforce approval locally; `require_approval` is forwarded to the API/platform handling approvals.
- Responses from MCP usage are surfaced through standard response items/messages per the Responses API.

## **Image-Guided Generation** *(Revolutionary New Feature in v0.2.4)*

**Revolutionary breakthrough**: Use input images to guide and influence image generation with the GPT Image 1 model! This powerful feature enables style transfer, logo creation, artistic interpretation, and more by combining vision and generation capabilities.

### 🎯 **Core Concept**

Image-guided generation works by:
1. **Input Images**: Provide one or more reference images as context
2. **Text Instructions**: Describe what you want to create based on those images  
3. **AI Understanding**: The model analyzes both the images and instructions
4. **Guided Generation**: Creates new images informed by the visual context

### 🎨 **Use Cases & Applications**

#### **Style Transfer**
Transform existing images into different artistic styles:

```rust
use open_ai_rust_responses_by_sshift::{Client, InputItem, Request, Tool, Model, ResponseItem};
use base64::{engine::general_purpose, Engine as _};
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;

    let content_image = "https://example.com/landscape.jpg";
    
    let request = Request::builder()
        .model(Model::GPT4o)
        .input_items(vec![
            InputItem::message("system", vec![
                InputItem::content_text("You are an expert in artistic style transfer.")
            ]),
            InputItem::message("user", vec![
                InputItem::content_text("Transform this landscape into Van Gogh's Starry Night style - swirling skies, bold brushstrokes, vibrant blues and yellows."),
                InputItem::content_image_with_detail(content_image, "high")
            ])
        ])
        .tools(vec![Tool::image_generation()])
        .temperature(0.8)
        .build();

    let response = client.responses.create(request).await?;
    
    // Save the generated image
    for item in &response.output {
        if let ResponseItem::ImageGenerationCall { result, .. } = item {
            let image_bytes = general_purpose::STANDARD.decode(result)?;
            let mut file = File::create("van_gogh_style.png")?;
            file.write_all(&image_bytes)?;
            println!("Style transfer complete! Saved as van_gogh_style.png");
            break;
        }
    }
    
    Ok(())
}
```

#### **Multi-Image Logo Creation**
Combine elements from multiple reference images:

```rust
let request = Request::builder()
    .model(Model::GPT4o)
    .input_items(vec![
        InputItem::message("system", vec![
            InputItem::content_text("You are a creative designer who combines elements from multiple reference images.")
        ]),
        InputItem::message("user", vec![
            InputItem::content_text("Create a modern logo combining the natural serenity from the first image with the character from the second image. Make it minimalist and professional."),
            InputItem::content_image_with_detail(nature_image, "high"),
            InputItem::content_image_with_detail(character_image, "high")
        ])
    ])
    .tools(vec![Tool::image_generation()])
    .temperature(0.7)
    .build();
```

#### **Product Design from References**
Create product concepts inspired by reference imagery:

```rust
let request = Request::builder()
    .model(Model::GPT4o)
    .input_items(vec![
        InputItem::message("system", vec![
            InputItem::content_text("You are a product designer who creates modern, sleek designs based on reference images.")
        ]),
        InputItem::message("user", vec![
            InputItem::content_text("Design a modern water bottle that captures the serenity and natural beauty of this landscape. Make it minimalist, elegant, and eco-friendly looking."),
            InputItem::content_image_with_detail(landscape_image, "high")
        ])
    ])
    .tools(vec![Tool::image_generation()])
    .temperature(0.6)
    .build();
```

### 📸 **Image Input Methods**

#### **URL Images**
```rust
// Basic URL input
InputItem::content_image(image_url)

// URL with detail level control
InputItem::content_image_with_detail(image_url, "high")  // "high", "low", "auto"
```

#### **Base64 Local Images**
```rust
// Read local file and encode
let image_data = std::fs::read("local_image.jpg")?;
let base64_data = base64::encode(&image_data);

// Use in request
InputItem::content_image_base64_with_detail(base64_data, "image/jpeg", "high")
```

#### **File ID Images**
```rust
// Upload file first
let file_response = client.files.upload_file("image.jpg", "vision", None).await?;

// Use file ID
InputItem::content_image_file_id_with_detail(file_response.id, "auto")
```

### 🎛️ **Configuration Options**

#### **Detail Levels**
- **`"high"`**: Maximum detail, higher cost, better quality analysis
- **`"low"`**: Faster processing, lower cost, basic analysis  
- **`"auto"`**: Balanced approach, automatically optimized

#### **Temperature Settings**
- **`0.6-0.7`**: More consistent, professional results
- **`0.8-0.9`**: More creative and artistic variations
- **`1.0+`**: Maximum creativity and randomness

#### **Model Selection**
- **`Model::GPT4o`**: Best for complex image analysis and generation
- **`Model::GPT4oMini`**: Cost-effective for simpler tasks

### 🔧 **Technical Implementation**

#### **Message Structure**
```rust
// System message for context and expertise
InputItem::message("system", vec![
    InputItem::content_text("You are an expert artist/designer/etc.")
]),

// User message with mixed content
InputItem::message("user", vec![
    InputItem::content_text("Instructions for what to create..."),
    InputItem::content_image_with_detail(image1_url, "high"),
    InputItem::content_image_with_detail(image2_url, "high"),
    // ... more images or text as needed
])
```

#### **Response Handling**
```rust
// Extract generated image from response
for item in &response.output {
    match item {
        ResponseItem::ImageGenerationCall { result, id, status } => {
            println!("Generated image ID: {}, Status: {}", id, status);
            
            // Decode and save
            let image_bytes = base64::decode(result)?;
            std::fs::write("generated_image.png", &image_bytes)?;
        },
        ResponseItem::Message { content, .. } => {
            // Model's description of what it created
            println!("Description: {}", content[0].text);
        },
        _ => {}
    }
}
```

### 💡 **Best Practices**

#### **Prompt Engineering**
- **Be Specific**: Describe exactly what elements to combine or transform
- **Provide Context**: Use system messages to set the AI's role/expertise
- **Style Guidance**: Mention specific artistic styles, techniques, or aesthetics
- **Quality Indicators**: Specify desired quality level (professional, artistic, etc.)

#### **Image Selection**
- **High Quality**: Use clear, well-lit reference images
- **Relevant Content**: Ensure images contain the elements you want to use
- **Appropriate Size**: Larger images work better with "high" detail setting
- **Multiple Angles**: For 3D objects, provide multiple viewpoints if possible

#### **Error Handling**
```rust
match client.responses.create(request).await {
    Ok(response) => {
        // Check for successful generation
        let mut found_image = false;
        for item in &response.output {
            if let ResponseItem::ImageGenerationCall { .. } = item {
                found_image = true;
                break;
            }
        }
        
        if !found_image {
            println!("No image was generated. Model response: {}", response.output_text());
        }
    },
    Err(e) => {
        eprintln!("Generation failed: {}", e);
    }
}
```

### 🚀 **Complete Example**

Run the comprehensive example that demonstrates all capabilities:

```bash
cargo run --example image_guided_generation
```

This example includes:
- ✅ Single image style transfer (watercolor landscape)
- ✅ Multi-image logo creation (combining nature + character)
- ✅ Base64 local image processing (enhanced with magical elements)
- ✅ Van Gogh style transformation (artistic style transfer)
- ✅ Product design inspiration (nature-inspired water bottle)

### 🎯 **Real-World Applications**

**Creative Agencies**:
- Generate brand variations from existing assets
- Create artistic interpretations for campaigns
- Transform client photos into different styles

**Product Development**:
- Design concepts from inspiration boards
- Visualize products in different styles/contexts
- Create variations of existing designs

**Content Creation**:
- Transform stock photos into unique artwork
- Create consistent visual styles across content
- Generate themed variations of images

**Software Applications**:
- Build image transformation tools
- Create style transfer applications
- Develop creative AI assistants

The image-guided generation feature represents a major breakthrough in AI-powered creativity, enabling previously impossible workflows that combine the power of vision understanding with generation capabilities.

### Method 1: Direct Images API

```rust
use open_ai_rust_responses_by_sshift::{Client, ImageGenerateRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    // Create an image generation request
    let image_request = ImageGenerateRequest::new("A serene mountain landscape at sunset")
        .with_size("1024x1024")      // Options: 256x256, 512x512, 1024x1024, 1024x1792, 1792x1024
        .with_quality("high")        // Options: standard, high
        .with_style("natural")       // Options: natural, vivid
        .with_format("png")          // Options: png, jpg, webp
        .with_compression(90)        // 0-100 for jpg/webp
        .with_background("white")    // For transparent images
        .with_seed(12345)            // For reproducibility
        .with_user("user-123");      // User tracking
    
    // Generate the image
    let response = client.images.generate(image_request).await?;
    
    // Access the generated image
    if let Some(url) = &response.data[0].url {
        println!("Generated image URL: {}", url);
    }
    
    // Or get base64 data if requested
    if let Some(b64) = &response.data[0].b64_json {
        println!("Base64 data length: {}", b64.len());
    }
    
    Ok(())
}
```

### Method 2: Built-in Image Generation

The model can now generate images directly when you include the built-in `image_generation` tool. It returns the image data as a base64-encoded string within a new `ImageGenerationCall` response item.

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model, Tool, ResponseItem};
use base64::{Engine as _, engine::general_purpose};
use std::io::Write;
use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    // Create a request using the built-in image_generation tool
    let request = Request::builder()
        .model(Model::GPT4oMini)
        .input("Generate an image of a rusty robot programming on a vintage computer")
        .tools(vec![Tool::image_generation()])
        .build();

    let response = client.responses.create(request).await?;

    // Find the image data in the response output
    for item in &response.output {
        if let ResponseItem::ImageGenerationCall { result, .. } = item {
            // Decode the base64 string
            let image_bytes = general_purpose::STANDARD.decode(result)?;
            let mut file = File::create("robot.png")?;
            file.write_all(&image_bytes)?;
            println!("Image saved to robot.png");
            break;
        }
    }
    
    Ok(())
}
```
The built-in tool does not take parameters. The model infers the image content from the `input` prompt. To control image parameters like size, quality, etc., use the Direct Images API (Method 1).

## **Image Generation** *(Overhauled in v0.2.0)*

The SDK now includes comprehensive image generation support through two methods:

### Method 1: Direct Images API

```rust
use open_ai_rust_responses_by_sshift::{Client, ImageGenerateRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    // Create an image generation request
    let image_request = ImageGenerateRequest::new("A serene mountain landscape at sunset")
        .with_size("1024x1024")      // Options: 256x256, 512x512, 1024x1024, 1024x1792, 1792x1024
        .with_quality("high")        // Options: standard, high
        .with_style("natural")       // Options: natural, vivid
        .with_format("png")          // Options: png, jpg, webp
        .with_compression(90)        // 0-100 for jpg/webp
        .with_background("white")    // For transparent images
        .with_seed(12345)            // For reproducibility
        .with_user("user-123");      // User tracking
    
    // Generate the image
    let response = client.images.generate(image_request).await?;
    
    // Access the generated image
    if let Some(url) = &response.data[0].url {
        println!("Generated image URL: {}", url);
    }
    
    // Or get base64 data if requested
    if let Some(b64) = &response.data[0].b64_json {
        println!("Base64 data length: {}", b64.len());
    }
    
    Ok(())
}
```

### Method 2: Built-in Image Generation

The model can now generate images directly when you include the built-in `image_generation` tool. It returns the image data as a base64-encoded string within a new `ImageGenerationCall` response item.

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model, Tool, ResponseItem};
use base64::{Engine as _, engine::general_purpose};
use std::io::Write;
use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    // Create a request using the built-in image_generation tool
    let request = Request::builder()
        .model(Model::GPT4oMini)
        .input("Generate an image of a rusty robot programming on a vintage computer")
        .tools(vec![Tool::image_generation()])
        .build();

    let response = client.responses.create(request).await?;

    // Find the image data in the response output
    for item in &response.output {
        if let ResponseItem::ImageGenerationCall { result, .. } = item {
            // Decode the base64 string
            let image_bytes = general_purpose::STANDARD.decode(result)?;
            let mut file = File::create("robot.png")?;
            file.write_all(&image_bytes)?;
            println!("Image saved to robot.png");
            break;
        }
    }
    
    Ok(())
}
```
The built-in tool does not take parameters. The model infers the image content from the `input` prompt. To control image parameters like size, quality, etc., use the Direct Images API (Method 1).

## **Image Input (Vision)** *(Updated in v0.2.2)*

The SDK can now send user-supplied images to multimodal models like **GPT-4o** and receive textual descriptions or other vision-based results.

### Quick Example

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;

    let image_url = "https://storage.googleapis.com/sshift-gpt-bucket/ledger-app/generated-image-1746132697428.png";

    let request = Request::builder()
        .model(Model::GPT4o)
        .input_image_url(image_url)
        .instructions("Describe the image in detail, mentioning colours, objects, and composition.")
        .build();

    let response = client.responses.create(request).await?;
    println!("Description: {}", response.output_text());

    Ok(())
}
```

Under the hood, `.input_image_url(...)` constructs a `message` payload with a single `content` item of type `input_image` and the provided URL, matching the OpenAI Responses API specification.

For granular control you can build messages manually:

```rust
use open_ai_rust_responses_by_sshift::types::InputItem;

let content = vec![
    InputItem::content_image("https://example.com/cat.png"),
    InputItem::content_text("Describe this cat, please"),
];

let message = InputItem::message("user", content);
```

See the runnable example in [`examples/image_input.rs`](examples/image_input.rs).

### Multi-Image Comparison *(v0.2.2)*

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;

    let first = "https://example.com/one.png";
    let second = "https://example.com/two.png";

    let request = Request::builder()
        .model(Model::GPT4o)
        .input_image_urls(&[first, second])
        .instructions("Compare the two images, outlining key similarities and differences.")
        .build();

    let response = client.responses.create(request).await?;
    println!("Comparison: {}", response.output_text());

    Ok(())
}
```

Both helper paths – `input_image_urls([...])` or chained `push_image_url(...)` – create a single user message with multiple `input_image` items, matching the Responses API spec.

See the runnable example in [`examples/image_input.rs`](examples/image_input.rs).

## **Code Interpreter** *(New in v0.2.3)*

The SDK now supports the official built-in code interpreter tool, allowing you to execute Python code in a secure container and retrieve the output as part of the response.

### Quick Example

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model, Tool};
use open_ai_rust_responses_by_sshift::types::Container;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    let request = Request::builder()
        .model(Model::GPT4o)
        .input("Calculate the 47th digit of pi using Python.")
        .tools(vec![Tool::code_interpreter(Some(Container::auto_type()))])
        .build();
    let response = client.responses.create(request).await?;
    println!("Result: {}", response.output_text());
    Ok(())
}
```

- The code interpreter tool requires a container with type `"auto"` (see `Container::auto_type()`).
- See `examples/code_interpreter.rs` for a full workflow and output parsing.
- The response includes both the code execution metadata and the final answer as a message.

## **Function Calling (Tools)**

> **✅ FIXED in v0.1.5**: Critical bug fix for multiple function calls! Previous versions had issues with multiple tool calls in sequence. This is now resolved.

Function calling allows the AI to use tools and functions you provide. The OpenAI Responses API uses a stateless approach where you submit tool outputs by creating new requests.

## **Function Calling & Tool Outputs**

The OpenAI Responses API handles function calling differently from the Assistants API. **There is no `submit_tool_outputs` endpoint** like in the Assistants API. Instead, tool outputs are submitted as input items in a new request.

### Complete Function Calling Workflow

```rust
use open_ai_rust_responses_by_sshift::{Client, Request, Model, Tool, ToolChoice};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    // Step 1: Define function tools
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
    
    // Step 2: Initial request with tools
    let request = Request::builder()
        .model(Model::GPT4o)
        .input("Calculate the result of 15 * 7 + 23 and explain the calculation")
        .tools(vec![calculator_tool.clone()])
        .tool_choice(ToolChoice::auto())
        .build();

    let response = client.responses.create(request).await?;
    
    // Step 3: Check for tool calls and execute functions
    let tool_calls = response.tool_calls();
    if !tool_calls.is_empty() {
        let mut function_outputs = Vec::new();
        
        for tool_call in &tool_calls {
            println!("Function: {}", tool_call.name);
            println!("Arguments: {}", tool_call.arguments);
            println!("Call ID: {}", tool_call.call_id);
            
            if tool_call.name == "calculate" {
                // Parse arguments and execute function
                let args: HashMap<String, String> = serde_json::from_str(&tool_call.arguments)?;
                if let Some(expression) = args.get("expression") {
                    let result = execute_calculator(expression); // Your function implementation
                    function_outputs.push((tool_call.call_id.clone(), result));
                }
            }
        }
        
        // Step 4: Submit tool outputs using the correct pattern
        let continuation_request = Request::builder()
            .model(Model::GPT4o)
            .with_function_outputs(response.id(), function_outputs)
            .tools(vec![calculator_tool]) // Keep tools available for follow-ups
            .build();
        
        let final_response = client.responses.create(continuation_request).await?;
        println!("Final response: {}", final_response.output_text());
    }
    
    Ok(())
}

fn execute_calculator(expression: &str) -> String {
    // Your function implementation here
    // This is just a placeholder
    format!("Result of {}", expression)
}

### Key Differences from Assistants API

| Aspect | Assistants API | Responses API |
|--------|----------------|---------------|
| Tool Output Submission | `submit_tool_outputs()` endpoint | New request with input items |
| State Management | Server-side state | Stateless with `previous_response_id` |
| Continuation | Single stream/connection | Multiple request/response cycles |
| Function Call Format | Tool outputs array | `function_call_output` input items |

### Creating Function Call Output Items

You can create function call output items manually using the `InputItem` helper:

```rust
use open_ai_rust_responses_by_sshift::types::InputItem;

// Manual creation of function call output
let function_output = InputItem::function_call_output(
    "call_abc123",  // Must match the call_id from the tool call
    "42"            // Your function result as a string
);

let request = Request::builder()
    .model(Model::GPT4o)
    .previous_response_id("resp_xyz789")
    .input_items(vec![function_output])
    .build();
```

### Multiple Tool Calls

When the model makes multiple tool calls, you need to provide outputs for all of them:

```rust
let function_outputs = vec![
    ("call_abc123".to_string(), "42".to_string()),      // First function result
    ("call_def456".to_string(), "true".to_string()),    // Second function result
    ("call_ghi789".to_string(), "error".to_string()),   // Third function result (can be error)
];

let continuation_request = Request::builder()
    .model(Model::GPT4o)
    .with_function_outputs(response.id(), function_outputs)
    .tools(original_tools)  // Keep same tools available
    .build();
```

### Error Handling in Function Calls

When your function encounters an error, you should still provide an output:

```rust
for tool_call in &tool_calls {
    let result = match execute_function(&tool_call.name, &tool_call.arguments) {
        Ok(value) => value,
        Err(e) => format!("Error: {}", e), // Provide error as string
    };
    
    function_outputs.push((tool_call.call_id.clone(), result));
}
```

### Recursive Function Calling

The model might call functions multiple times in a conversation. Handle this by checking for tool calls in each response:

```rust
let mut current_response = initial_response;
let mut max_iterations = 5; // Prevent infinite loops

while !current_response.tool_calls().is_empty() && max_iterations > 0 {
    // Execute tool calls
    let function_outputs = execute_tool_calls(&current_response.tool_calls())?;
    
    // Submit outputs and get next response
    let request = Request::builder()
        .model(Model::GPT4o)
        .with_function_outputs(current_response.id(), function_outputs)
        .tools(tools.clone())
        .build();
    
    current_response = client.responses.create(request).await?;
    max_iterations -= 1;
}

println!("Final response: {}", current_response.output_text());
```

### Best Practices

1. **Always match call_id**: Each function output must have the exact `call_id` from the corresponding tool call
2. **Handle all tool calls**: Provide outputs for every tool call the model makes
3. **Preserve tools**: Include tools in continuation requests for potential follow-up calls
4. **Error handling**: Provide meaningful error messages as function outputs when execution fails
5. **Prevent infinite loops**: Set a maximum number of function call iterations
6. **Use proper types**: Parse function arguments properly and validate inputs

### Example: Weather Function

```rust
let weather_tool = Tool::function(
    "get_weather",
    "Get current weather for a location",
    json!({
        "type": "object",
        "properties": {
            "location": {
                "type": "string",
                "description": "City name or coordinates"
            },
            "units": {
                "type": "string",
                "enum": ["celsius", "fahrenheit"],
                "description": "Temperature units"
            }
        },
        "required": ["location"]
    }),
);

// In your tool call handler:
if tool_call.name == "get_weather" {
    let args: serde_json::Value = serde_json::from_str(&tool_call.arguments)?;
    let location = args["location"].as_str().unwrap_or("unknown");
    let units = args["units"].as_str().unwrap_or("celsius");
    
    let weather_data = fetch_weather(location, units).await?;
    let result = serde_json::to_string(&weather_data)?;
    
    function_outputs.push((tool_call.call_id.clone(), result));
}
```

This pattern ensures proper integration with the OpenAI Responses API's stateless design while maintaining conversation context through response IDs.

## **Reasoning Parameters**

Control how the AI thinks through problems with reasoning parameters:

```rust
use open_ai_rust_responses_by_sshift::types::{ReasoningParams, Effort, SummarySetting};

// Fast, cost-effective reasoning
let request = Request::builder()
    .model(Model::O4Mini)  // Specialized reasoning model
    .input("Explain quantum computing")
    .reasoning(ReasoningParams::new()
        .with_effort(Effort::Low)              // Quick responses
        .with_summary(SummarySetting::Auto))   // Automatic summary generation
    .max_output_tokens(2000)  // Reasoning models need more tokens (optimized from 200)
    // Note: O4Mini doesn't support temperature parameter (built-in optimization)
    .build();

// Balanced reasoning  
let request = Request::builder()
    .model(Model::O4Mini)  // Good balance of speed and capability
    .input("Analyze this business case study")
    .reasoning(ReasoningParams::new()
        .with_effort(Effort::Medium)           // Balanced approach
        .with_summary(SummarySetting::Auto))   // Automatic summary generation
    .max_output_tokens(2000)  // Allow space for reasoning
    .build();

// Thorough, detailed reasoning  
let request = Request::builder()
    .model(Model::O3)  // Most capable reasoning model
    .input("Solve this complex mathematical proof")
    .reasoning(ReasoningParams::new()
        .with_effort(Effort::High)             // Deep reasoning
        .with_summary(SummarySetting::Detailed)) // Comprehensive summaries
    .max_output_tokens(2000)  // Allow space for complex reasoning
    // Note: O3 also has built-in reasoning optimization
    .build();
```

### Available Options

- **Effort Levels**: `Effort::Low` (fast), `Effort::Medium` (balanced), or `Effort::High` (thorough)
- **Summary Settings**: `SummarySetting::Auto`, `SummarySetting::Concise`, or `SummarySetting::Detailed`

### Token Optimization for Reasoning

Reasoning models require significantly more tokens than standard models due to their internal thinking process:

```rust
// Standard model token allocation
let standard_request = Request::builder()
    .model(Model::GPT4oMini)
    .input("Write a short story")
    .max_output_tokens(500)  // Optimized for general responses
    .build();

// Reasoning model token allocation
let reasoning_request = Request::builder()
    .model(Model::O4Mini)
    .input("Solve this logic puzzle step by step")
    .reasoning(ReasoningParams::new().with_effort(Effort::High))
    .max_output_tokens(2000)  // 4x more tokens for reasoning chains
    .build();
```

### Important Notes

⚠️ **Reasoning models (O4Mini, O3, O1 series) don't support the `temperature` parameter** - they have built-in reasoning optimization instead.

### Best Practices

1. **Token Allocation**: Use 2000+ tokens for reasoning models (vs 500 for standard models)
2. **Model Selection**: O4Mini for efficient reasoning, O3 for complex problems
3. **Effort Level**: Start with Low effort and increase if needed
4. **Summary Setting**: Use Auto for most cases, Detailed for complex outputs

## Model Parameter Compatibility and Requirements

This section summarizes which request parameters are supported or discouraged by model family. Always check API error messages in case of upstream changes.

### Key requirements (all models)
- model: required (use `Model` enum)
- input (or input_items/messages): required
- max_output_tokens: optional but recommended
- tools/tool_choice/parallel_tool_calls: optional, supported on all
- previous_response_id/store: optional, recommended for conversation continuity
- include: optional

### Sampling and control parameters

| Parameter | GPT‑4o family (general) | O‑series (o1/o3/o4‑mini) | GPT‑5 family |
|---|---|---|---|
| temperature | Supported | Not supported (ignored/invalid) | Not supported (returns error) |
| top_p | Supported | Not supported | Not supported |
| top_logprobs | Supported | Not supported | Not supported |
| max_output_tokens | Supported (typical 500) | Supported (recommend 2000) | Supported |
| reasoning.effort | N/A | Supported (Low/Medium/High) | Supported via `ReasoningEffort` (mapped to `reasoning.effort`) |
| text.verbosity | N/A | N/A | Supported (Low/Medium/High) |
| tools/function calling | Supported | Supported | Supported |

Notes
- O‑series and GPT‑5 optimize internally and generally reject classic sampling controls (temperature/top_p/top_logprobs).
- For GPT‑5, use `RequestBuilder::reasoning_effort(...)` to engage deeper reasoning behavior when needed, and `verbosity(...)` to tune output detail.
- For long chains with O‑series, prefer `store(false)` and larger `max_output_tokens` (≈2000) to avoid truncation.

### Minimal request examples

```rust
// General chat (GPT‑4o family)
let req = Request::builder()
    .model(Model::GPT4oMini)
    .input("Tell me a short joke")
    .temperature(0.7)
    .max_output_tokens(500)
    .build();

// Reasoning (O‑series)
let req = Request::builder()
    .model(Model::O3)
    .input("Prove a property about AVL trees")
    .reasoning(ReasoningParams::new().with_effort(Effort::High))
    .max_output_tokens(2000)
    .build();

// GPT‑5, concise vs deep
let concise = Request::builder()
    .model(Model::GPT5Mini)
    .input("Summarize in 2 bullets")
    .verbosity(Verbosity::Low)
    .build();

let deep = Request::builder()
    .model(Model::GPT5)
    .input("Plan a data migration with rollback strategies")
    .reasoning_effort(ReasoningEffort::High)
    .build();
```

## **Background Processing**

Handle long-running operations with background processing:

```rust
use open_ai_rust_responses_by_sshift::types::{BackgroundHandle, BackgroundStatus};

// Start a background task
let request = Request::builder()
    .model(Model::O4Mini)
    .input("Analyze this large dataset...")
    .background(true)  // Enable background processing
    .build();

// Returns immediately with a handle for polling
let response = client.responses.create(request).await?;

// Poll for completion (when implemented)
// let status = client.background.check_status(&response.background_handle).await?;
```

Background processing returns HTTP 202 status and provides handles for checking operation status.

## **Enhanced Models**

Support for the latest OpenAI models with optimal recommendations:

```rust
// Reasoning models - optimal for complex thinking
Model::O3              // Latest, most capable reasoning
Model::O4Mini          // Efficient reasoning (recommended for most reasoning tasks)

// O1 family - original reasoning models  
Model::O1              // Original reasoning model
Model::O1Mini          // Compact reasoning
Model::O1Preview       // Preview version

// GPT-4 Omni family - multimodal capabilities (recommended for general use)
Model::GPT4o          // Latest GPT-4 Omni
Model::GPT4oMini      // Compact GPT-4 Omni (recommended default)
Model::GPT4o20241120  // Specific version for consistency

// Legacy models still supported
Model::GPT4Turbo      // Previous generation
Model::GPT35Turbo     // Cost-effective option
```

### Model Optimization Recommendations

```rust
// For production - balanced performance/cost (RECOMMENDED)
let request = Request::builder()
    .model(Model::GPT4oMini)  // Best default choice
    .temperature(0.7)         // Temperature supported
    .max_output_tokens(500)   // Optimized for general use
    .build();

// For reasoning tasks
let request = Request::builder()
    .model(Model::O4Mini)     // Efficient reasoning
    .reasoning(ReasoningParams::new().with_effort(Effort::Low))
    .max_output_tokens(2000)  // Reasoning needs more tokens
    // Note: No temperature parameter (built-in optimization)
    .build();

// For complex reasoning tasks
let request = Request::builder()
    .model(Model::O3)         // Most capable
    .reasoning(ReasoningParams::high_effort_detailed())
    .max_output_tokens(2000)  // Maximum reasoning space
    // Note: No temperature parameter (built-in optimization)
    .build();

// For general conversations
let request = Request::builder()
    .model(Model::GPT4o)      // Latest GPT-4 Omni
    .temperature(0.7)         // Temperature supported
    .max_output_tokens(500)   // Standard allocation
    .build();
```

### Model Capabilities Matrix

| Model | Use Case | Temperature | Reasoning | Cost | Speed | Token Default |
|-------|----------|-------------|-----------|------|-------|---------------|
| `GPT4oMini` | **General use (recommended)** | ✅ | ❌ | 💚 Low | 🚀 Fast | 500 |
| `GPT4o` | Advanced conversations | ✅ | ❌ | 🟡 Medium | ⚡ Fast | 500 |
| `O4Mini` | **Reasoning tasks** | ❌ | ✅ | 💚 Low | 🚀 Fast | 2000 |
| `O3` | Complex reasoning | ❌ | ✅ | 🔴 High | 🐌 Slow | 2000 |
| `O1Mini` | Legacy reasoning | ❌ | ✅ | 🟡 Medium | ⚡ Medium | 2000 |

### Token Optimization by Use Case

```rust
// Quick responses (chat, Q&A)
.max_output_tokens(500)    // GPT4oMini, GPT4o

// Reasoning tasks (problem solving, analysis)
.max_output_tokens(2000)   // O4Mini, O3, O1 series

// Streaming (real-time generation)
.max_output_tokens(500)    // All models, chunked output

// Long-form content (articles, stories)
.max_output_tokens(1000)   // GPT4o for quality
```

### Performance Tips

1. **Default to GPT4oMini**: Best balance of cost, speed, and quality
2. **Use O4Mini for reasoning**: 4x faster than O3 with good results
3. **Allocate tokens wisely**: 500 for chat, 2000 for reasoning
4. **Avoid temperature on reasoning models**: They ignore it anyway
5. **Consider streaming**: For better perceived performance

## **Type-Safe Includes**

Compile-time validated include options replace error-prone strings:

### Migration from String-Based Includes

```rust
// Old way (still supported for backward compatibility)
let request = Request::builder()
    .model(Model::GPT4oMini)
    .input("Search for information")
    .include_strings(vec![
        "file_search.results".to_string(),        // Legacy value (still works)
        "file_search_call.results".to_string(),   // Current API value
    ])
    .build();

// New way (recommended, type-safe)
use open_ai_rust_responses_by_sshift::types::Include;

let request = Request::builder()
    .model(Model::GPT4oMini)
    .input("Search for information")
    .include(vec![Include::FileSearchResults])
    .build();
```

### Available Include Options

```rust
use open_ai_rust_responses_by_sshift::types::Include;

let request = Request::builder()
    .model(Model::GPT4oMini)
    .input("Comprehensive analysis")
    .include(vec![
        Include::FileSearchResults,         // file_search_call.results
        Include::WebSearchResults,          // web_search_call.results  
        Include::MessageInputImageUrl,      // message.input_image.image_url
        Include::ComputerCallOutputImageUrl, // computer_call_output.output.image_url
        Include::ReasoningEncryptedContent, // reasoning.encrypted_content
    ])
    .build();
```

### Type Safety Benefits

```rust
// Compile-time error prevention
let request = Request::builder()
    .model(Model::GPT4oMini)
    .input("Test")
    .include(vec![
        Include::FileSearchResults,
        // Include::InvalidOption,  // This would cause a compile error!
    ])
    .build();

// IDE autocompletion and documentation available
let includes = vec![
    Include::FileSearchResults,      // Shows documentation in IDE
    Include::WebSearchResults,       // Auto-completion available
];
```

### API Field Mapping

The include options map to these API field names:

| Type-Safe Enum | API Field Name | Description |
|-----------------|----------------|-------------|
| `FileSearchResults` | `file_search_call.results` | Results from file search operations |
| `WebSearchResults` | `web_search_call.results` | Results from web search operations |
| `MessageInputImageUrl` | `message.input_image.image_url` | Image URLs in message inputs |
| `ComputerCallOutputImageUrl` | `computer_call_output.output.image_url` | Image URLs from computer use |
| `ReasoningEncryptedContent` | `reasoning.encrypted_content` | Encrypted reasoning traces |

### Backward Compatibility

Both legacy and current API values are supported:

```rust
// These all work and map to Include::FileSearchResults
.include_strings(vec![
    "file_search.results".to_string(),      // Legacy (still supported)
    "file_search_call.results".to_string(), // Current API value
])
```

## **Enhanced Response Fields** *(Phase 1 Complete)*

Version 0.1.7 adds 21 new fields to the Response struct for full OpenAI May 2025 spec compatibility:

### Core Response Fields

```rust
let response = client.responses.create(request).await?;

// Basic fields
println!("ID: {}", response.id);
println!("Object type: {}", response.object);           // "response"
println!("Status: {}", response.status);                // "completed", "in_progress", etc.
println!("Model: {}", response.model);
println!("Created at: {}", response.created_at);

// Output fields
println!("Output text: {}", response.output_text());    // Helper method
println!("Raw output: {:?}", response.output);          // Full output structure

// Context fields
println!("Instructions: {:?}", response.instructions);
println!("User: {:?}", response.user);
println!("Previous response: {:?}", response.previous_response_id);
```

### Parameter Echo Fields

The response echoes back the parameters used:

```rust
// Temperature and sampling
println!("Temperature: {:?}", response.temperature);     // None for reasoning models
println!("Top P: {:?}", response.top_p);
println!("Top logprobs: {:?}", response.top_logprobs);

// Token limits
println!("Max output tokens: {:?}", response.max_output_tokens);

// Tool configuration
println!("Parallel tool calls: {:?}", response.parallel_tool_calls);
println!("Tool choice: {:?}", response.tool_choice);
println!("Tools: {:?}", response.tools);

// Reasoning
println!("Reasoning effort: {:?}", response.reasoning_effort);
```

### Usage Analytics

Comprehensive token usage tracking:

```rust
if let Some(usage) = &response.usage {
    println!("Total tokens: {}", usage.total_tokens);
    println!("Prompt tokens: {}", usage.prompt_tokens);
    println!("Output tokens: {}", usage.output_tokens);
    
    // Detailed output token breakdown
    if let Some(details) = &usage.output_tokens_details {
        println!("Text tokens: {:?}", details.text_tokens);
        println!("Reasoning tokens: {:?}", details.reasoning_tokens);
        println!("Rejected tokens: {:?}", details.rejected_tokens);
    }
    
    // Detailed prompt token breakdown
    if let Some(details) = &usage.prompt_tokens_details {
        println!("Text tokens: {:?}", details.text_tokens);
        println!("Cached tokens: {:?}", details.cached_tokens);
    }
}

// Helper method
println!("Total tokens (helper): {}", response.total_tokens());
```

### Advanced Configuration

```rust
// Text configuration
if let Some(text_config) = &response.text {
    println!("Format: {:?}", text_config.format);       // TextFormat enum
    println!("Stop sequences: {:?}", text_config.stop);
}

// Truncation settings
match &response.truncation {
    TruncationSetting::Disabled => println!("Truncation: disabled"),
    TruncationSetting::Config(config) => {
        println!("Truncation type: {}", config.truncation_type);
        println!("Max tokens: {:?}", config.max_tokens);
        println!("Max messages: {:?}", config.max_messages);
    }
}

// Reasoning output (for reasoning models)
if let Some(reasoning) = &response.reasoning {
    println!("Reasoning type: {}", reasoning.reasoning_type);
    if let Some(content) = &reasoning.content {
        println!("Reasoning content: {}", content);
    }
}
```

### Error and Status Information

```rust
// Helper methods for status checking
if response.is_complete() {
    println!("Response completed successfully");
}

if response.is_in_progress() {
    println!("Response still processing");
}

if response.has_errors() {
    println!("Response encountered errors");
    if let Some(error) = &response.error {
        println!("Error type: {}", error.error_type);
        println!("Error message: {}", error.message);
        println!("Error code: {:?}", error.code);
    }
}

// Incomplete details
if let Some(incomplete) = &response.incomplete_details {
    println!("Reason: {}", incomplete.reason);
    if let Some(details) = &incomplete.details {
        println!("Details: {}", details);
    }
}
```

### Complete Field Reference

| Field | Type | Description | New in v0.1.7 |
|-------|------|-------------|---------------|
| `id` | `String` | Unique response identifier | ❌ |
| `object` | `String` | Object type ("response") | ✅ |
| `status` | `String` | Response status | ✅ |
| `model` | `String` | Model used | ❌ |
| `output` | `Option<Output>` | Response output | ❌ |
| `output_text` | `Option<String>` | Text output (convenience) | ✅ |
| `previous_response_id` | `Option<String>` | Previous response ID | ❌ |
| `created_at` | `i64` | Creation timestamp | ❌ |
| `metadata` | `Option<HashMap>` | Custom metadata | ❌ |
| `instructions` | `Option<String>` | Instructions echo | ✅ |
| `user` | `Option<String>` | User identifier | ✅ |
| `temperature` | `Option<f32>` | Temperature used | ✅ |
| `top_p` | `Option<f32>` | Top-p used | ✅ |
| `max_output_tokens` | `Option<u32>` | Max tokens used | ✅ |
| `parallel_tool_calls` | `Option<bool>` | Parallel tools enabled | ✅ |
| `tool_choice` | `Option<ToolChoice>` | Tool choice used | ✅ |
| `tools` | `Option<Vec<Tool>>` | Tools available | ✅ |
| `top_logprobs` | `Option<u32>` | Top logprobs requested | ✅ |
| `reasoning_effort` | `Option<String>` | Reasoning effort used | ✅ |
| `usage` | `Option<Usage>` | Token usage details | ✅ |
| `text` | `Option<TextConfig>` | Text configuration | ✅ |
| `truncation` | `Option<TruncationSetting>` | Truncation settings | ✅ |
| `reasoning` | `Option<ReasoningOutput>` | Reasoning output | ✅ |
| `incomplete_details` | `Option<IncompleteDetails>` | Incomplete info | ✅ |
| `error` | `Option<ResponseError>` | Error details | ✅ |

### Backward Compatibility

All new fields are optional and use serde defaults, ensuring 100% backward compatibility:

```rust
// Old code continues to work
let response = client.responses.create(request).await?;
println!("Output: {}", response.output_text());  // Works as before

// New code can access enhanced fields
if let Some(usage) = &response.usage {
    println!("Tokens used: {}", usage.total_tokens);
}
```

## Production-Ready Streaming

Streaming responses with proper HTTP chunked parsing:

```rust
use futures::StreamExt;

let request = Request::builder()
    .model(Model::GPT4oMini)  // Optimized for streaming performance
    .input("Write a short story about a robot")
    .max_output_tokens(500)   // Optimized for smooth streaming
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

### Streaming Optimization

Token allocation affects streaming smoothness:

```rust
// Optimal streaming configuration
let request = Request::builder()
    .model(Model::GPT4oMini)
    .input("Tell me a story")
    .max_output_tokens(500)   // Balanced for smooth chunks
    .temperature(0.7)         // Natural variation
    .build();

// For reasoning models with streaming
let request = Request::builder()
    .model(Model::O4Mini)
    .input("Explain this concept step by step")
    .reasoning(ReasoningParams::new().with_effort(Effort::Low))
    .max_output_tokens(2000)  // More tokens for reasoning chains
    .build();
```

### Best Practices

1. **Token Limits**: 500 tokens for smooth streaming, 2000 for reasoning
2. **Model Choice**: GPT4oMini provides best streaming performance
3. **Error Handling**: Always handle stream errors gracefully
4. **Buffer Management**: Process chunks immediately to avoid memory buildup

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
open-ai-rust-responses-by-sshift = { version = "0.3.0", default-features = false, features = ["stream", "native-tls"] }
```

---

### Migration notes (0.2.x → 0.3.0)

- Potential source breaks only if:
  - You construct public structs with literals (`Tool`, `TextConfig`, `ReasoningParams`). Use builders or `..Default::default()`.
  - You exhaustively match `Model`. Handle new GPT‑5 variants or add a wildcard arm.
- Existing runtime behavior (wire format, defaults) is unchanged for prior models.

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

# Function calling
cargo run --example function_calling

# Built-in image generation
cargo run --example image_generation_builtin
# Direct API image generation
cargo run --example image_generation

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

The `comprehensive_demo.rs` example showcases all major SDK features:

- **Response Creation**: Basic and advanced requests with optimized tokens
- **Conversation Continuity**: Using `previous_response_id` (100% success rate)
- **File Operations**: Upload, list, download, and delete files
- **Vector Stores**: Create, add files, search, and delete
- **Built-in Tools**: Web search and file search capabilities
- **Custom Function Calling**: Calculator tool example
- **Image Generation**: AI-triggered image creation (NEW)
- **Streaming Responses**: Real-time response streaming (if enabled)
- **Resource Management**: Proper cleanup and deletion testing
- **Token Optimization**: Demonstrates proper token allocation for different models

This demo creates temporary resources for testing and cleans them up afterward, making it safe to run multiple times.

### Image Generation Examples

The `image_generation_builtin.rs` example demonstrates using the new, simple, built-in tool.
The `image_generation.rs` example demonstrates both direct API usage and the built-in tool side-by-side.

```bash
# Run the built-in tool example
cargo run --example image_generation_builtin
```

# Code Interpreter example
cargo run --example code_interpreter
