//! Reasoning example demonstrating O4Mini model with enhanced reasoning capabilities
//!
//! Run with: `cargo run --example reasoning_demo`
//!
//! Make sure to set your OpenAI API key:
//! ```bash
//! export OPENAI_API_KEY=sk-your-api-key-here
//! ```

use open_ai_rust_responses_by_sshift::types::{Effort, ReasoningParams, SummarySetting};
use open_ai_rust_responses_by_sshift::{Client, Model, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable
    let client = Client::from_env()?;

    println!("🧠 OpenAI Rust Responses - Reasoning Example");
    println!("=============================================\n");

    // Problem 1: Mathematical reasoning with O4Mini
    println!("🔢 Problem 1: Mathematical Reasoning");
    println!("====================================");

    let reasoning_params = ReasoningParams::new()
        .with_effort(Effort::High)
        .with_summary(SummarySetting::Auto);

    let math_request = Request::builder()
        .model(Model::O4Mini) // Use O4Mini for reasoning tasks
        .input("A farmer has 17 sheep. All but 9 die. How many sheep are left? Please show your reasoning process.")
        .instructions("Think carefully and show your reasoning process clearly")
        .reasoning(reasoning_params.clone())
        .max_output_tokens(300)
        // Note: O4Mini doesn't support temperature parameter
        .user("reasoning-demo") // Track user for analytics
        .store(false) // Use stateless mode for reasoning tasks
        .build();

    println!("🤔 Asking: A farmer has 17 sheep. All but 9 die. How many sheep are left?");
    println!("💭 Using high-effort reasoning with O4Mini...\n");

    let math_response = client.responses.create(math_request).await?;

    // Show response details
    println!("📊 Response Status: {}", math_response.status);
    println!("🤖 Model: {}", math_response.model);

    if let Some(usage) = &math_response.usage {
        println!("📊 Token Usage: {} total", usage.total_tokens);
        if let Some(details) = &usage.output_tokens_details {
            if let Some(reasoning_tokens) = details.reasoning_tokens {
                println!("🧠 Reasoning tokens: {reasoning_tokens}");
            }
        }
    }

    println!("\n🤖 Answer: {}\n", math_response.output_text());

    // Check for reasoning output
    if let Some(reasoning) = &math_response.reasoning {
        if let Some(content) = &reasoning.content {
            println!("🔍 Reasoning Trace:");
            for (i, step) in content.iter().enumerate() {
                println!(
                    "   {}. [{}] {}",
                    i + 1,
                    step.content_type,
                    step.text.as_deref().unwrap_or("No text")
                );
            }
        }
        if reasoning.encrypted_content.is_some() {
            println!("🔐 Encrypted reasoning content available (stateless mode)");
        }
    }

    println!("\n{}\n", "=".repeat(60));

    // Problem 2: Logical puzzle
    println!("🧩 Problem 2: Logical Puzzle");
    println!("============================");

    let logic_request = Request::builder()
        .model(Model::O4Mini) // Continue using O4Mini for reasoning
        .input("Three friends - Alice, Bob, and Charlie - each have a different pet: a cat, a dog, and a bird. Alice doesn't have the cat. Bob doesn't have the dog. Who has which pet?")
        .instructions("Solve this step-by-step using logical deduction")
        .reasoning(ReasoningParams::high_effort_with_summary())
        .max_output_tokens(300)
        // Note: O4Mini doesn't support temperature parameter
        .user("reasoning-demo") // Maintain user identity across requests
        .store(false) // Continue with stateless mode
        .build();

    println!("🧩 Puzzle: Three friends with different pets and three clues...");
    println!("💭 Using medium-effort reasoning...\n");

    let logic_response = client.responses.create(logic_request).await?;

    println!("📊 Response Status: {}", logic_response.status);

    if let Some(usage) = &logic_response.usage {
        let math_tokens = math_response.total_tokens().unwrap_or(0);
        let logic_tokens = usage.total_tokens;
        println!(
            "📊 This response: {} tokens | Total session: {} tokens",
            logic_tokens,
            math_tokens + logic_tokens
        );
    }

    println!("\n🤖 Solution: {}\n", logic_response.output_text());

    println!("{}\n", "=".repeat(60));

    // Problem 3: Creative reasoning
    println!("🎨 Problem 3: Creative Problem Solving");
    println!("======================================");

    let creative_request = Request::builder()
        .model(Model::O4Mini)
        .input("You have a 3-gallon jug and a 5-gallon jug. How can you measure exactly 4 gallons of water?")
        .instructions("Think creatively about this classic water jug problem")
        .reasoning(ReasoningParams::auto_summary()) // Use auto summary
        .max_output_tokens(400) // More tokens for detailed creative solution
        // Note: O4Mini doesn't support temperature parameter
        .user("reasoning-demo") // Continue tracking
        .store(false) // Stateless mode
        .build();

    println!("🪣 Classic Problem: Measure 4 gallons using 3-gallon and 5-gallon jugs");
    println!("💭 Using high-effort reasoning with creative thinking...\n");

    let creative_response = client.responses.create(creative_request).await?;

    println!("📊 Final Response Status: {}", creative_response.status);

    // Show session summary
    let total_session_tokens = math_response.total_tokens().unwrap_or(0)
        + logic_response.total_tokens().unwrap_or(0)
        + creative_response.total_tokens().unwrap_or(0);

    println!("📊 Session Summary: {total_session_tokens} total tokens across 3 reasoning problems");

    println!("\n🤖 Solution: {}\n", creative_response.output_text());

    // Show reasoning insights
    println!("🧠 Reasoning Session Insights:");
    println!("   🎯 Model: O4Mini optimized for reasoning tasks");
    println!("   📊 Problems solved: 3 (math, logic, creative)");
    println!("   💭 Reasoning efforts: High, Medium, High");
    println!("   🔐 Stateless mode: All responses used store=false");
    println!("   📝 Total computation: {total_session_tokens} tokens");

    // Show parameter consistency
    if let Some(user) = &creative_response.user {
        println!("   👤 User session: {user}");
    }
    println!("   🌡️ Temperature: Not supported by O4Mini (reasoning model)");

    println!("\n✨ Features Demonstrated:");
    println!("   🧠 O4Mini model for enhanced reasoning capabilities");
    println!("   💭 Reasoning parameters with effort levels (low/medium/high)");
    println!("   📊 Reasoning token tracking and analysis");
    println!("   🔐 Stateless mode for independent problem solving");
    println!("   📈 Session-level token usage monitoring");
    println!("   🎯 Built-in reasoning optimization (temperature not needed)");

    println!("\n✅ Reasoning demonstration completed!");
    println!("💡 O4Mini excels at step-by-step logical reasoning and problem solving!");

    Ok(())
}
