//! # Container Expiration Test
//!
//! This example demonstrates container expiration recovery by:
//! 1. Creating a container with an initial code interpreter request
//! 2. Waiting for user input (allowing time for container to expire)
//! 3. Making a follow-up request that references the expired container
//! 4. Showing how the SDK automatically handles the expiration
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example container_expiration_test
//! ```
//!
//! When prompted, wait a few minutes before pressing Y to continue.
//! This gives the container time to expire and demonstrates the recovery.

use open_ai_rust_responses_by_sshift::{Client, Container, RecoveryPolicy, Request, Tool};
use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Container Expiration Recovery Test");
    println!("=====================================\n");

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    // Test with different recovery policies
    println!("Choose recovery policy:");
    println!("1. Default (auto-retry enabled, 1 attempt)");
    println!("2. Conservative (no auto-retry, manual handling)");
    println!("3. Aggressive (auto-retry enabled, 3 attempts)");
    print!("Enter choice (1-3): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice = input.trim();

    let (client, policy_name) = match choice {
        "1" | "" => {
            let client = Client::new(&api_key)?;
            (client, "Default")
        }
        "2" => {
            let policy = RecoveryPolicy::conservative();
            let client = Client::new_with_recovery(&api_key, policy)?;
            (client, "Conservative")
        }
        "3" => {
            let policy = RecoveryPolicy::aggressive();
            let client = Client::new_with_recovery(&api_key, policy)?;
            (client, "Aggressive")
        }
        _ => {
            println!("Invalid choice, using default policy");
            let client = Client::new(&api_key)?;
            (client, "Default")
        }
    };

    println!("\n🔧 Using {policy_name} Recovery Policy\n");

    // Step 1: Create initial request to establish a container
    println!("📋 Step 1: Creating initial container with code execution");
    let initial_request = Request::builder()
        .model("gpt-4o-mini")
        .input("Create a Python variable called 'session_data' and set it to a dictionary with 'started_at' as the current timestamp. Print the variable.")
        .tools(vec![Tool::code_interpreter(Some(Container::auto_type()))])
        .build();

    let initial_response = client.responses.create(initial_request).await?;
    println!("✅ Initial request completed");
    println!(
        "📦 Container created with ID: {}",
        initial_response
            .output
            .iter()
            .find_map(|item| match item {
                open_ai_rust_responses_by_sshift::ResponseItem::CodeInterpreterCall {
                    container_id,
                    ..
                } => Some(container_id.as_str()),
                _ => None,
            })
            .unwrap_or("unknown")
    );
    println!("💬 Response: {}", initial_response.output_text());

    // Step 2: Wait for user input (allowing container to expire)
    println!("\n⏳ Step 2: Waiting for container expiration");
    println!("📌 The container will expire after a few minutes of inactivity.");
    println!("💡 Wait 3-5 minutes, then press 'Y' to continue with the next request.");
    println!("   This will trigger a container expiration error that the SDK should handle.");
    print!("\nReady to test expiration recovery? (Y/n): ");
    io::stdout().flush()?;

    let mut continue_input = String::new();
    io::stdin().read_line(&mut continue_input)?;
    if continue_input.trim().to_lowercase() == "n" {
        println!("Test cancelled.");
        return Ok(());
    }

    // Step 3: Make follow-up request that should trigger expiration
    println!("\n📋 Step 3: Making follow-up request (should trigger container expiration)");
    let followup_request = Request::builder()
        .model("gpt-4o-mini")
        .input("Access the 'session_data' variable we created earlier and add a new field 'followup_at' with the current timestamp. Print both timestamps and calculate the time difference.")
        .tools(vec![Tool::code_interpreter(Some(Container::auto_type()))])
        .previous_response_id(initial_response.id().to_string())
        .build();

    println!("🔄 Sending follow-up request (this may trigger container expiration)...");

    // Use the recovery-aware method to get detailed information
    match client
        .responses
        .create_with_recovery(followup_request.clone())
        .await
    {
        Ok(response_with_recovery) => {
            if response_with_recovery.had_recovery() {
                println!("\n🎉 SUCCESS: Container expiration was automatically handled!");
                println!("📊 Recovery Details:");
                println!(
                    "   - Recovery attempted: {}",
                    response_with_recovery.recovery_info.attempted
                );
                println!(
                    "   - Retry attempts: {}",
                    response_with_recovery.recovery_info.retry_count
                );
                println!(
                    "   - Recovery successful: {}",
                    response_with_recovery.recovery_info.successful
                );

                if let Some(msg) = response_with_recovery.recovery_message() {
                    println!("   - User message: {msg}");
                }

                if let Some(original_error) = &response_with_recovery.recovery_info.original_error {
                    println!("   - Original error: {original_error}");
                }
            } else {
                println!("✅ Request succeeded without needing recovery (container may not have expired yet)");
            }

            println!(
                "\n💬 Final Response: {}",
                response_with_recovery.response.output_text()
            );
        }
        Err(e) => {
            if e.is_container_expired() {
                println!("\n⚠️  Container expiration detected!");

                if policy_name == "Conservative" {
                    println!("🔧 Conservative policy - demonstrating manual recovery:");

                    // Manually prune and retry
                    let cleaned_request = client
                        .responses
                        .prune_expired_context_manual(followup_request);
                    match client.responses.create(cleaned_request).await {
                        Ok(response) => {
                            println!("✅ Manual recovery successful!");
                            println!("💬 Response: {}", response.output_text());
                        }
                        Err(e) => {
                            println!("❌ Manual recovery failed: {e}");
                        }
                    }
                } else {
                    println!("❌ Automatic recovery failed: {e}");
                }
            } else {
                println!("❌ Request failed with non-recoverable error: {e}");
            }
        }
    }

    // Step 4: Summary
    println!("\n📈 Test Summary");
    println!("===============");
    println!("✓ Created initial container");
    println!("✓ Waited for potential expiration");
    println!("✓ Tested recovery mechanism with {policy_name} policy");

    println!("\n💡 Key Insights:");
    println!("• Container expiration is handled transparently by the SDK");
    println!("• Different recovery policies provide different behaviors");
    println!("• Applications can choose the recovery strategy that fits their needs");
    println!("• User experience remains smooth even when containers expire");

    Ok(())
}
