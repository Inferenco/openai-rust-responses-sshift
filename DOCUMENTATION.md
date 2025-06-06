# Open AI Rust Responses by SShift - Documentation

> **🔥 v0.2.0 Update**: Major update to image generation! The SDK now supports the official built-in `image_generation` tool, replacing the previous function-based workaround. This is a breaking change.

This document provides comprehensive documentation for the Open AI Rust Responses by SShift library, a Rust SDK for the OpenAI Responses API with advanced reasoning parameters, background processing, enhanced models, production-ready streaming, and **working image generation**.

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
10. [**Image Generation**](#image-generation) *(Overhauled in v0.2.0)*
11. [Function Calling & Tool Outputs](#function-calling--tool-outputs)
12. [**Reasoning Parameters**](#reasoning-parameters)
13. [**Background Processing**](#background-processing)
14. [**Enhanced Models**](#enhanced-models)
15. [**Type-Safe Includes**](#type-safe-includes)
16. [**Enhanced Response Fields**](#enhanced-response-fields) *(Phase 1)*
17. [Production-Ready Streaming](#production-ready-streaming)
18. [Error Handling](#error-handling)
19. [Advanced Configuration](#advanced-configuration)
20. [Feature Flags](#feature-flags)
21. [Testing and Examples](#testing-and-examples)

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
open-ai-rust-responses-by-sshift = "0.2.0"
```

If you want to use streaming responses, make sure to include the `stream` feature (enabled by default):

```toml
[dependencies]
open-ai-rust-responses-by-sshift = { version = "0.2.0", features = ["stream"] }
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

- **Effort Levels**: `Effort::Low` (fast) or `Effort::High` (thorough)
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
open-ai-rust-responses-by-sshift = { version = "0.2.0", default-features = false, features = ["stream", "native-tls"] }
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
