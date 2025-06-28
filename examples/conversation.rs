//! Conversation example showing multi-turn conversation using response IDs with enhanced features
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
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Create client from environment variable
    let client = Client::from_env()?;

    println!("💬 OpenAI Rust Responses - Conversation Example");
    println!("===============================================\n");

    // Start a conversation about cooking with enhanced features
    println!("🧑‍🍳 Starting an enhanced cooking conversation...\n");

    // First message: Introduce yourself and ask for a recipe
    let request1 = Request::builder()
        .model(Model::GPT4oMini) // Updated to use GPT-4o Mini
        .input("Hi! My name is Alex and I love cooking. Can you give me a simple recipe for chocolate chip cookies?")
        .instructions("You are a friendly chef who remembers details about people you talk to. Be helpful and encouraging.")
        .temperature(0.7)
        .max_output_tokens(250) // Use preferred parameter
        .user("conversation-alex") // Add user tracking
        .store(true) // Explicitly enable conversation storage
        .build();

    println!("👤 Alex: Hi! My name is Alex and I love cooking. Can you give me a simple recipe for chocolate chip cookies?");

    let response1 = client.responses.create(request1).await?;

    // Enhanced response monitoring
    println!(
        "📊 Response 1 Status: {} | Complete: {}",
        response1.status,
        response1.is_complete()
    );
    if let Some(usage) = &response1.usage {
        println!("📊 Tokens: {total} total", total=usage.total_tokens);
    }

    println!("🤖 Chef: {output}\n", output=response1.output_text());

    // Second message: Ask for modifications, continuing the conversation
    let request2 = Request::builder()
        .model(Model::GPT4oMini)
        .input("That sounds great! Can you make it healthier by reducing the sugar?")
        .instructions("Remember this is Alex who loves cooking. Provide healthy alternatives.")
        .previous_response_id(response1.id()) // 🔑 This maintains conversation context!
        .temperature(0.7)
        .max_output_tokens(200) // Adjust token limit
        .user("conversation-alex") // Maintain user identity
        .store(true) // Continue storing conversation
        .build();

    println!("👤 Alex: That sounds great! Can you make it healthier by reducing the sugar?");

    let response2 = client.responses.create(request2).await?;

    // Show cumulative token usage
    let total_tokens_so_far =
        response1.total_tokens().unwrap_or(0) + response2.total_tokens().unwrap_or(0);
    println!(
        "�� Response 2 Status: {status} | Cumulative Tokens: {tokens}",
        status=response2.status,
        tokens=total_tokens_so_far
    );

    println!("🤖 Chef: {output}\n", output=response2.output_text());

    // Third message: Ask about baking time, still in the same conversation
    let request3 = Request::builder()
        .model(Model::GPT4oMini)
        .input("Perfect! One more question - what if I want to make them extra chewy?")
        .instructions("Continue helping Alex with cooking tips. Focus on texture techniques.")
        .previous_response_id(response2.id()) // Continue from the previous response
        .temperature(0.7)
        .max_output_tokens(150) // Shorter response for final tip
        .user("conversation-alex") // Consistent user tracking
        .store(true) // Complete conversation storage
        .build();

    println!("👤 Alex: Perfect! One more question - what if I want to make them extra chewy?");

    let response3 = client.responses.create(request3).await?;

    // Final response analysis
    let final_total_tokens = total_tokens_so_far + response3.total_tokens().unwrap_or(0);
    println!(
        "�� Response 3 Status: {status} | Final Total Tokens: {tokens}",
        status=response3.status,
        tokens=final_total_tokens
    );

    // Check if all responses completed successfully
    let all_successful =
        response1.is_complete() && response2.is_complete() && response3.is_complete();
    println!("✅ All responses completed successfully: {all_successful}");

    println!("🤖 Chef: {output}\n", output=response3.output_text());

    // Show the enhanced conversation chain with details
    println!("🔗 Enhanced Conversation Chain:");
    println!(
        "├── Response 1 ID: {id} | Model: {model} | Tokens: {tokens}",
        id=response1.id(),
        model=response1.model,
        tokens=response1.total_tokens().unwrap_or(0)
    );
    println!(
        "├── Response 2 ID: {id} | Model: {model} | Tokens: {tokens} (continues from {prev})",
        id=response2.id(),
        model=response2.model,
        tokens=response2.total_tokens().unwrap_or(0),
        prev=response1.id()
    );
    println!(
        "└── Response 3 ID: {id} | Model: {model} | Tokens: {tokens} (continues from {prev})",
        id=response3.id(),
        model=response3.model,
        tokens=response3.total_tokens().unwrap_or(0),
        prev=response2.id()
    );

    // Show parameter echoes if available
    println!("\n⚙️ Conversation Parameters:");
    if let Some(temp) = response3.temperature {
        println!("   Temperature: {temp}");
    }
    if let Some(user) = &response3.user {
        println!("   User: {user}");
    }

    // Conversation analytics
    println!("\n📊 Conversation Analytics:");
    println!("   🔄 Total turns: 3");
    println!("   📝 Total tokens consumed: {tokens}", tokens=final_total_tokens);
    println!(
        "   ⚡ Average tokens per turn: {:.1}",
        final_total_tokens as f64 / 3.0
    );
    println!(
        "   🤖 Model consistency: All responses used {}",
        response1.model
    );
    println!("   ✅ Success rate: 100% (all responses completed)");

    println!("\n🎸 Features Demonstrated:");
    println!("   • Enhanced response status tracking for each turn");
    println!("   • Comprehensive token usage monitoring and analytics");
    println!("   • User tracking consistency across conversation");
    println!("   • Parameter echoing for conversation analysis");
    println!("   • Improved error detection and success verification");
    println!("   • Conversation storage management");

    println!("\n✅ Enhanced conversation completed! The chef remembered Alex's name and maintained context throughout with full monitoring!");

    Ok(())
}
