//! Simple web search example demonstrating enhanced features
//!
//! Run with: `cargo run --example web_search_simple`
//!
//! Make sure to set your OpenAI API key:
//! ```bash
//! export OPENAI_API_KEY=sk-your-api-key-here
//! ```

use open_ai_rust_responses_by_sshift::types::{Include, Tool, ToolChoice};
use open_ai_rust_responses_by_sshift::{Client, Model, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API key from environment variable
    let client = Client::from_env()?;

    println!("🔍 Web Search Simple Example");
    println!("============================\n");

    // Create a request with web search tool enabled and enhanced features
    let request = Request::builder()
        .model(Model::GPT4oMini) // Updated to use GPT-4o Mini as default
        .input("What are the latest developments in Rust programming language in 2024?")
        .instructions("Provide comprehensive and up-to-date information")
        .tools(vec![Tool::web_search_preview()])
        .tool_choice(ToolChoice::auto())
        .include(vec![Include::FileSearchResults]) // Include search results details
        .max_output_tokens(300) // Use preferred parameter
        .temperature(0.2) // Lower temperature for factual queries
        .user("web-search-example") // Add user tracking
        .store(true) // Explicitly store for conversation continuity
        .build();

    println!("🌐 Searching for latest Rust developments...");

    // Send the request
    let response = client.responses.create(request).await?;

    // Enhanced response analysis
    println!("📊 Response Status: {}", response.status);
    println!("🤖 Model Used: {}", response.model);

    // Check for errors
    if response.has_errors() {
        println!("❌ Search encountered errors!");
        if let Some(error) = &response.error {
            println!("   Error: {} - {}", error.code, error.message);
        }
        return Ok(());
    }

    // Display token usage
    if let Some(usage) = &response.usage {
        println!(
            "📊 Token Usage: {} total ({} input + {} output)",
            usage.total_tokens, usage.input_tokens, usage.output_tokens
        );
    }

    // Print the response with enhanced formatting
    println!("\n📝 Search Results & Analysis:");
    println!("{}", response.output_text());

    // Show any tool calls that were made
    let tool_calls = response.tool_calls();
    if !tool_calls.is_empty() {
        println!("\n🛠️ Tool Calls Made:");
        for (i, tool_call) in tool_calls.iter().enumerate() {
            println!(
                "   {}. {} (ID: {})",
                i + 1,
                tool_call.name,
                tool_call.call_id
            );
        }
    }

    // Show parameter echoes
    if let Some(temp) = response.temperature {
        println!("\n⚙️ Temperature used: {}", temp);
    }

    println!("\n✅ Enhanced web search completed!");
    println!("🎸 Features: status tracking, token monitoring, error handling");

    Ok(())
}
