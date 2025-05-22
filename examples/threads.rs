//! Threads example showing organized conversation management
//!
//! Run with: `cargo run --example threads`
//!
//! Make sure to set your OpenAI API key:
//! ```bash
//! export OPENAI_API_KEY=sk-your-api-key-here
//! ```

use open_ai_rust_responses_by_sshift::threads::CreateThreadRequest;
use open_ai_rust_responses_by_sshift::{Client, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable
    let client = Client::from_env()?;

    println!("🧵 OpenAI Rust Responses - Threads Example");
    println!("==========================================\n");

    // Create a new thread for a cooking conversation
    println!("🆕 Creating a new cooking thread...");

    let thread_request = CreateThreadRequest {
        model: Model::GPT4o,
        instructions: Some("You are a professional chef and cooking instructor. You're patient, detailed, and love sharing cooking tips. Remember details about the person you're talking to.".to_string()),
        initial_message: "Hi! I'm Sarah, and I'm a complete beginner at cooking. Can you help me learn to make pasta from scratch?".to_string(),
        metadata: None,
    };

    let (thread, initial_response) = client.threads.create(thread_request).await?;

    println!("✅ Thread created with ID: {}", thread.id);
    println!("👤 Sarah: Hi! I'm Sarah, and I'm a complete beginner at cooking. Can you help me learn to make pasta from scratch?");
    println!("🤖 Chef: {}\n", initial_response.output_text());

    // Continue the conversation in the thread
    println!("💬 Continuing the conversation...");

    let (updated_thread, response2) = client
        .threads
        .continue_thread(
            &thread,
            Model::GPT4o,
            "That sounds great! What kind of flour should I use?",
        )
        .await?;

    println!("👤 Sarah: That sounds great! What kind of flour should I use?");
    println!("🤖 Chef: {}\n", response2.output_text());

    // Ask another question in the same thread
    let (updated_thread, response3) = client
        .threads
        .continue_thread(
            &updated_thread,
            Model::GPT4o,
            "I don't have a pasta machine. Can I still make it by hand?",
        )
        .await?;

    println!("👤 Sarah: I don't have a pasta machine. Can I still make it by hand?");
    println!("🤖 Chef: {}\n", response3.output_text());

    // One more follow-up question
    let (_final_thread, response4) = client
        .threads
        .continue_thread(
            &updated_thread,
            Model::GPT4o,
            "Perfect! How long should I knead the dough?",
        )
        .await?;

    println!("👤 Sarah: Perfect! How long should I knead the dough?");
    println!("🤖 Chef: {}\n", response4.output_text());

    // Show thread information
    println!("📊 Thread Information:");
    println!("   🆔 Thread ID: {}", thread.id);
    println!("   🤖 Model: {:?}", thread.current_model);
    println!(
        "   🔗 Current Response ID: {:?}",
        thread.current_response_id
    );
    println!("   🕐 Created: {}", thread.created_at);

    // Retrieve the thread to show it persists
    println!("\n🔍 Retrieving thread from server...");
    let retrieved_thread = client.threads.retrieve(&thread.id).await?;
    println!("✅ Successfully retrieved thread: {}", retrieved_thread.id);

    println!("\n✅ Thread example completed! The conversation context was maintained throughout.");
    println!(
        "💡 Notice how the chef remembered Sarah's name and her beginner status in each response!"
    );

    Ok(())
}
