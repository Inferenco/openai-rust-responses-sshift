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

    println!("🎸 OpenAI Rust Responses - Enhanced Error Handling Demo");
    println!("=====================================================\n");

    // Create a basic request
    let request = Request::builder()
        .model(Model::GPT4oMini)
        .input("Tell me a short joke about programming")
        .max_output_tokens(200)
        .temperature(0.7)
        .build();

    println!("📤 Making request with enhanced error handling...\n");

    // Demonstrate enhanced error handling
    match client.responses.create(request).await {
        Ok(response) => {
            println!("✅ Success!");
            println!("📝 Response: {response}", response = response.output_text());

            if let Some(usage) = &response.usage {
                println!("\n📊 Token Usage:");
                println!("   Input tokens: {input}", input = usage.input_tokens);
                println!("   Output tokens: {output}", output = usage.output_tokens);
                println!("   Total tokens: {total}", total = usage.total_tokens);
            }
        }
        Err(e) => {
            println!("❌ Request failed with enhanced error handling:");
            println!("   Error type: {:?}", std::mem::discriminant(&e));
            println!("   User message: {msg}", msg = e.user_message());
            println!("   Technical details: {e}");

            // Show recovery information
            if e.is_recoverable() {
                println!("   🔄 This error is recoverable");
                if let Some(retry_after) = e.retry_after() {
                    println!("   ⏱️ Suggested retry delay: {retry_after}s");
                }
            } else {
                println!("   ❌ This error is not recoverable");
            }

            if e.is_transient() {
                println!("   ⚡ This is a transient error");
            }

            // Return the error for demonstration
            return Err(e.into());
        }
    }

    println!("\n🎸 Error Handling Features Demonstrated:");
    println!("   ✅ User-friendly error messages");
    println!("   ✅ Error classification (recoverable/transient)");
    println!("   ✅ Retry delay suggestions");
    println!("   ✅ Technical details for debugging");
    println!("   ✅ Proper error type discrimination");

    Ok(())
}
