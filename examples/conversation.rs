//! Conversation example showing multi-turn conversation using response IDs
//!
//! Run with: `cargo run --example conversation`
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

    println!("💬 OpenAI Rust Responses - Conversation Example");
    println!("===============================================\n");

    // Start a conversation about cooking
    println!("🧑‍🍳 Starting a cooking conversation...\n");

    // First message: Introduce yourself and ask for a recipe
    let request1 = Request::builder()
        .model(Model::GPT4o)
        .input("Hi! My name is Alex and I love cooking. Can you give me a simple recipe for chocolate chip cookies?")
        .instructions("You are a friendly chef who remembers details about people you talk to.")
        .max_tokens(200)
        .temperature(0.7)
        .build();

    println!("👤 Alex: Hi! My name is Alex and I love cooking. Can you give me a simple recipe for chocolate chip cookies?");

    let response1 = client.responses.create(request1).await?;
    println!("🤖 Chef: {}\n", response1.output_text());

    // Second message: Ask for modifications, continuing the conversation
    let request2 = Request::builder()
        .model(Model::GPT4o)
        .input("That sounds great! Can you make it healthier by reducing the sugar?")
        .previous_response_id(response1.id()) // 🔑 This maintains conversation context!
        .max_tokens(200)
        .temperature(0.7)
        .build();

    println!("👤 Alex: That sounds great! Can you make it healthier by reducing the sugar?");

    let response2 = client.responses.create(request2).await?;
    println!("🤖 Chef: {}\n", response2.output_text());

    // Third message: Ask about baking time, still in the same conversation
    let request3 = Request::builder()
        .model(Model::GPT4o)
        .input("Perfect! One more question - what if I want to make them extra chewy?")
        .previous_response_id(response2.id()) // Continue from the previous response
        .max_tokens(150)
        .temperature(0.7)
        .build();

    println!("👤 Alex: Perfect! One more question - what if I want to make them extra chewy?");

    let response3 = client.responses.create(request3).await?;
    println!("🤖 Chef: {}\n", response3.output_text());

    // Show the conversation chain
    println!("🔗 Conversation Chain:");
    println!("├── Response 1 ID: {}", response1.id());
    println!(
        "├── Response 2 ID: {} (continues from {})",
        response2.id(),
        response1.id()
    );
    println!(
        "└── Response 3 ID: {} (continues from {})",
        response3.id(),
        response2.id()
    );

    println!("\n✅ Conversation completed! The chef remembered Alex's name and maintained context throughout!");

    Ok(())
}
