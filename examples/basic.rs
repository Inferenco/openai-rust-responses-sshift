//! Basic example showing simple request/response usage with enhanced features
//!
//! Run with: `cargo run --example basic`
//!
//! Make sure to set your OpenAI API key:
//! ```bash
//! export OPENAI_API_KEY=sk-your-api-key-here
//! ```

use open_ai_rust_responses_by_sshift::{Client, Model, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable
    let client = Client::from_env()?;

    println!("🤖 OpenAI Rust Responses - Basic Example");
    println!("=========================================\n");

    // Create a simple request with enhanced features
    let request = Request::builder()
        .model(Model::GPT4oMini) // Updated to use GPT-4o Mini as default
        .input("What is the meaning of life in exactly 42 words?")
        .instructions("Be concise and philosophical")
        .temperature(0.7)
        .max_output_tokens(100) // Use new preferred parameter
        .top_logprobs(3) // Request log probabilities
        .parallel_tool_calls(true) // Enable parallel tool execution
        .user("basic-example-user") // Add user tracking
        .build();

    println!("📤 Sending request with enhanced parameters...");

    // Get the response
    let response = client.responses.create(request).await?;

    println!("📥 Response received!\n");

    // Display core response information
    println!("🆔 Response ID: {}", response.id());
    println!("📊 Status: {}", response.status);
    println!("🤖 Model: {}", response.model);
    println!("🕐 Created at: {}", response.created_at);
    println!("📝 Content: {}", response.output_text());

    // Show response status helper methods
    println!("\n🔍 Response Status Checks:");
    println!("  ✅ Is complete: {}", response.is_complete());
    println!("  ⏳ Is in progress: {}", response.is_in_progress());
    println!("  ❌ Has errors: {}", response.has_errors());

    // Display token usage information
    if let Some(usage) = &response.usage {
        println!("\n📊 Token Usage:");
        println!("  Input tokens: {}", usage.input_tokens);
        println!("  Output tokens: {}", usage.output_tokens);
        println!("  Total tokens: {}", usage.total_tokens);

        if let Some(details) = &usage.output_tokens_details {
            if let Some(reasoning_tokens) = details.reasoning_tokens {
                println!("  Reasoning tokens: {reasoning_tokens}");
            }
        }

        // Use helper method
        println!(
            "  Total (via helper): {}",
            response.total_tokens().unwrap_or(0)
        );
    }

    // Display parameter echoes
    if let Some(temp) = response.temperature {
        println!("\n⚙️ Request Parameters Echoed:");
        println!("  Temperature: {temp}");
    }
    if let Some(top_p) = response.top_p {
        println!("  Top-p: {top_p}");
    }
    if let Some(max_tokens) = response.max_output_tokens {
        println!("  Max output tokens: {max_tokens}");
    }

    // Check if there are any tool calls
    let tool_calls = response.tool_calls();
    if !tool_calls.is_empty() {
        println!("\n🛠️ Tool calls:");
        for tool_call in tool_calls {
            println!("  - {}: {}", tool_call.name, tool_call.arguments);
        }
    }

    // Show system instructions echo
    if let Some(instructions) = &response.instructions {
        println!("\n📋 Instructions: {instructions}");
    }

    // Show user identifier echo
    if let Some(user) = &response.user {
        println!("\n👤 User: {user}");
    }

    println!("\n✅ Example completed successfully!");
    println!("🎸 Features demonstrated: status checking, token usage, parameter echoes");

    Ok(())
}
