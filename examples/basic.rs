//! Basic example showing simple request/response usage
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

    // Create a simple request
    let request = Request::builder()
        .model(Model::GPT4o)
        .input("What is the meaning of life in exactly 42 words?")
        .max_tokens(100)
        .temperature(0.7)
        .build();

    println!("📤 Sending request...");

    // Get the response
    let response = client.responses.create(request).await?;

    println!("📥 Response received!\n");
    println!("🆔 Response ID: {}", response.id());
    println!("📝 Content: {}", response.output_text());
    println!("🕐 Created at: {}", response.created_at);

    // Check if there are any tool calls
    let tool_calls = response.tool_calls();
    if !tool_calls.is_empty() {
        println!("\n🛠️ Tool calls:");
        for tool_call in tool_calls {
            println!("  - {}: {}", tool_call.name, tool_call.arguments);
        }
    }

    println!("\n✅ Example completed successfully!");

    Ok(())
}
