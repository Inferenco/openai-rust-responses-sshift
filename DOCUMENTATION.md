# Open AI Rust Responses by SShift - Documentation

This document provides comprehensive documentation for the Open AI Rust Responses by SShift library, a Rust SDK for the OpenAI Responses API with advanced reasoning parameters, background processing, enhanced models, and production-ready streaming.

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
10. [Function Calling & Tool Outputs](#function-calling--tool-outputs)
11. [**Reasoning Parameters**](#reasoning-parameters)
12. [**Background Processing**](#background-processing)
13. [**Enhanced Models**](#enhanced-models)
14. [**Type-Safe Includes**](#type-safe-includes)
15. [Production-Ready Streaming](#production-ready-streaming)
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
        .model(Model::O4Mini)  // Optimized efficient reasoning model
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
open-ai-rust-responses-by-sshift = "0.1.6"
```

If you want to use streaming responses, make sure to include the `stream` feature (enabled by default):

```toml
[dependencies]
open-ai-rust-responses-by-sshift = { version = "0.1.6", features = ["stream"] }
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
    .model(Model::O4Mini)
    .input("Explain quantum computing")
    .reasoning(ReasoningParams::new()
        .with_effort(Effort::Low)              // Quick responses
        .with_summary(SummarySetting::Auto))   // Automatic summary generation
    .build();

// Thorough, detailed reasoning  
let request = Request::builder()
    .model(Model::O3)
    .input("Solve this complex mathematical proof")
    .reasoning(ReasoningParams::new()
        .with_effort(Effort::High)             // Deep reasoning
        .with_summary(SummarySetting::Detailed)) // Comprehensive summaries
    .build();
```

### Available Options

- **Effort Levels**: `Effort::Low` (fast) or `Effort::High` (thorough)
- **Summary Settings**: `SummarySetting::Auto`, `SummarySetting::Concise`, or `SummarySetting::Detailed`

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
Model::O4Mini          // Efficient reasoning (recommended for most use cases)

// O1 family - original reasoning models  
Model::O1              // Original reasoning model
Model::O1Mini          // Compact reasoning
Model::O1Preview       // Preview version

// GPT-4 Omni family - multimodal capabilities
Model::GPT4o          // Latest GPT-4 Omni
Model::GPT4oMini      // Compact GPT-4 Omni  
Model::GPT4o20241120  // Specific version for consistency

// Legacy models still supported
Model::GPT4Turbo      // Previous generation
Model::GPT35Turbo     // Cost-effective option
```

### Model Optimization Recommendations

```rust
// For production - balanced performance/cost
let request = Request::builder()
    .model(Model::O4Mini)  // Efficient reasoning
    .reasoning(ReasoningParams::new().with_effort(Effort::Low))
    .build();

// For complex reasoning tasks
let request = Request::builder()
    .model(Model::O3)      // Most capable
    .reasoning(ReasoningParams::high_effort_detailed())
    .build();

// For general conversations
let request = Request::builder()
    .model(Model::GPT4o)   // Latest GPT-4 Omni
    .build();
```

## **Type-Safe Includes**

Compile-time validated include options replace error-prone strings:

### Migration from String-Based Includes

```rust
// Old way (still supported for backward compatibility)
let request = Request::builder()
    .model(Model::O4Mini)
    .input("Search for information")
    .include_strings(vec!["file_search.results".to_string()])
    .build();

// New way (recommended, type-safe)
use open_ai_rust_responses_by_sshift::types::Include;

let request = Request::builder()
    .model(Model::O4Mini)
    .input("Search for information")
    .include(vec![Include::FileSearchResults])
    .build();
```

### Available Include Options

```rust
use open_ai_rust_responses_by_sshift::types::Include;

let request = Request::builder()
    .model(Model::O4Mini)
    .input("Comprehensive analysis")
    .include(vec![
        Include::FileSearchResults,         // Search results from file operations
        // Note: Additional include types available based on API capabilities
    ])
    .build();
```

### Type Safety Benefits

```rust
// Compile-time error prevention
let request = Request::builder()
    .model(Model::O4Mini)
    .input("Test")
    .include(vec![
        Include::FileSearchResults,
        // Include::InvalidOption,  // This would cause a compile error!
    ])
    .build();

// IDE autocompletion and documentation available
let includes = vec![
    Include::FileSearchResults,      // Shows documentation in IDE
];
```

## Production-Ready Streaming

Streaming responses with proper HTTP chunked parsing:

```rust
use futures::StreamExt;

let request = Request::builder()
    .model(Model::O4Mini)
    .input("Write a short story about a robot")
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
open-ai-rust-responses-by-sshift = { version = "0.1.6", default-features = false, features = ["stream", "native-tls"] }
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

The `comprehensive_demo.rs` example showcases all major SDK features:

- **Response Creation**: Basic and advanced requests
- **Conversation Continuity**: Using `previous_response_id`
- **File Operations**: Upload, list, download, and delete files
- **Vector Stores**: Create, add files, search, and delete
- **Built-in Tools**: Web search and file search capabilities
- **Custom Function Calling**: Calculator tool example
- **Streaming Responses**: Real-time response streaming (if enabled)
- **Resource Management**: Proper cleanup and deletion testing

This demo creates temporary resources for testing and cleans them up afterward, making it safe to run multiple times.
