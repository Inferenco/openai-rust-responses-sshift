//! # Container Recovery Demo
//!
//! This example demonstrates the advanced container recovery features of the SDK:
//!
//! 1. **Automatic Expired Container Handling** - SDK automatically detects and retries on container expiration
//! 2. **Configurable Recovery Policy** - Control retry behavior, notifications, and context pruning
//! 3. **Context Pruning** - Automatically or manually clean expired containers from context
//! 4. **Recovery Callbacks** - Get notified when recovery occurs
//! 5. **User-Friendly Error Messages** - Provide smooth UX even when containers expire
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example container_recovery_demo
//! ```

use open_ai_rust_responses_by_sshift::{Client, Container, RecoveryPolicy, Request, Tool};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize simple logging to see recovery attempts
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    println!("🔧 Container Recovery Demo");
    println!("==========================\n");

    // Demo 1: Default Recovery Policy
    println!("📋 Demo 1: Default Recovery Policy");
    println!("   - Auto-retry on container expiration: enabled");
    println!("   - Max retries: 1");
    println!("   - Auto-prune expired containers: enabled");
    println!("   - Notifications: disabled\n");

    let default_client = Client::new(&api_key)?;
    demo_basic_recovery(&default_client).await?;

    // Demo 2: Conservative Recovery Policy
    println!("\n📋 Demo 2: Conservative Recovery Policy");
    println!("   - Auto-retry: disabled");
    println!("   - Notifications: enabled");
    println!("   - Logging: enabled\n");

    let conservative_policy = RecoveryPolicy::conservative();
    let conservative_client = Client::new_with_recovery(&api_key, conservative_policy)?;
    demo_conservative_recovery(&conservative_client).await?;

    // Demo 3: Aggressive Recovery Policy
    println!("\n📋 Demo 3: Aggressive Recovery Policy");
    println!("   - Auto-retry: enabled");
    println!("   - Max retries: 3");
    println!("   - Custom reset message");
    println!("   - Logging: enabled\n");

    let aggressive_policy = RecoveryPolicy::aggressive();
    let aggressive_client = Client::new_with_recovery(&api_key, aggressive_policy)?;
    demo_aggressive_recovery(&aggressive_client).await?;

    // Demo 4: Custom Recovery Policy with Callback
    println!("\n📋 Demo 4: Custom Recovery Policy with Callback");
    println!("   - Custom recovery settings");
    println!("   - Recovery callback notifications\n");

    demo_custom_recovery_with_callback(&api_key).await?;

    // Demo 5: Manual Context Pruning
    println!("\n📋 Demo 5: Manual Context Pruning");
    println!("   - Proactive context cleanup");
    println!("   - Application-controlled recovery\n");

    demo_manual_context_pruning(&default_client).await?;

    println!("\n✅ All recovery demos completed successfully!");
    println!("\n💡 Key Takeaways:");
    println!("   • SDK automatically handles container expiration");
    println!("   • Recovery policies provide fine-grained control");
    println!("   • Callbacks enable custom notification handling");
    println!("   • Context pruning keeps conversations clean");
    println!("   • User experience remains smooth despite backend issues");

    Ok(())
}

/// Demo basic recovery with default settings
async fn demo_basic_recovery(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::builder()
        .model("gpt-4o-mini")
        .input("Calculate the 10th Fibonacci number using Python code.")
        .tools(vec![Tool::code_interpreter(Some(Container::auto_type()))])
        .build();

    match client.responses.create(request).await {
        Ok(response) => {
            println!("✅ Request succeeded: {}", response.output_text());
        }
        Err(e) if e.is_container_expired() => {
            println!("⚠️  Container expired (this would be auto-handled with recovery enabled)");
        }
        Err(e) => {
            println!("❌ Request failed: {e}");
        }
    }

    Ok(())
}

/// Demo conservative recovery (no auto-retry, but notifications enabled)
async fn demo_conservative_recovery(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::builder()
        .model("gpt-4o-mini")
        .input("What's the square root of 144?")
        .tools(vec![Tool::code_interpreter(Some(Container::auto_type()))])
        .build();

    // With conservative policy, we need to handle recovery manually
    match client.responses.create(request.clone()).await {
        Ok(response) => {
            println!("✅ Request succeeded: {}", response.output_text());
        }
        Err(e) if e.is_container_expired() => {
            println!("⚠️  Container expired - manual recovery needed");

            // Manually prune context and retry
            let cleaned_request = client.responses.prune_expired_context_manual(request);
            match client.responses.create(cleaned_request).await {
                Ok(response) => {
                    println!("✅ Manual recovery succeeded: {}", response.output_text());
                }
                Err(e) => {
                    println!("❌ Manual recovery failed: {e}");
                }
            }
        }
        Err(e) => {
            println!("❌ Request failed: {e}");
        }
    }

    Ok(())
}

/// Demo aggressive recovery with multiple retries
async fn demo_aggressive_recovery(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::builder()
        .model("gpt-4o-mini")
        .input("Generate a random number between 1 and 100 using Python.")
        .tools(vec![Tool::code_interpreter(Some(Container::auto_type()))])
        .build();

    // Use the recovery-aware method to get detailed recovery info
    match client.responses.create_with_recovery(request).await {
        Ok(response_with_recovery) => {
            if response_with_recovery.had_recovery() {
                println!("🔄 Recovery performed:");
                println!(
                    "   - Attempts: {}",
                    response_with_recovery.recovery_info.retry_count
                );
                println!(
                    "   - Successful: {}",
                    response_with_recovery.recovery_info.successful
                );
                if let Some(msg) = response_with_recovery.recovery_message() {
                    println!("   - Message: {msg}");
                }
            }
            println!(
                "✅ Final result: {}",
                response_with_recovery.response.output_text()
            );
        }
        Err(e) => {
            println!("❌ Request failed even with aggressive recovery: {e}");
        }
    }

    Ok(())
}

/// Demo custom recovery policy with callback notifications
async fn demo_custom_recovery_with_callback(
    api_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a custom recovery policy
    let custom_policy = RecoveryPolicy::new()
        .with_auto_retry(true)
        .with_max_retries(2)
        .with_notify_on_reset(true)
        .with_auto_prune(true)
        .with_reset_message("Your code session was refreshed for optimal performance.")
        .with_logging(true);

    let client = Client::new_with_recovery(api_key, custom_policy)?;

    // Add a recovery callback
    let client_with_callback =
        client
            .responses
            .clone()
            .with_recovery_callback(Box::new(|error, attempt| {
                println!("🔔 Recovery callback triggered:");
                println!("   - Error: {error}");
                println!("   - Attempt: {attempt}");
                println!("   - Action: Retrying with fresh context...");
            }));

    let request = Request::builder()
        .model("gpt-4o-mini")
        .input("Create a simple Python function to check if a number is prime.")
        .tools(vec![Tool::code_interpreter(Some(Container::auto_type()))])
        .build();

    match client_with_callback.create_with_recovery(request).await {
        Ok(response_with_recovery) => {
            if response_with_recovery.had_recovery() {
                println!("🔄 Custom recovery completed successfully!");
                if let Some(msg) = response_with_recovery.recovery_message() {
                    println!("   - User message: {msg}");
                }
            }
            println!(
                "✅ Result: {}",
                response_with_recovery.response.output_text()
            );
        }
        Err(e) => {
            println!("❌ Request failed: {e}");
        }
    }

    Ok(())
}

/// Demo manual context pruning for proactive cleanup
async fn demo_manual_context_pruning(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    // Simulate a conversation with multiple requests
    let mut previous_response_id: Option<String> = None;

    for i in 1..=3 {
        println!("🔄 Request {i}/3");

        let mut request = Request::builder()
            .model("gpt-4o-mini")
            .input(format!("Calculate {} factorial using Python.", i * 5))
            .tools(vec![Tool::code_interpreter(Some(Container::auto_type()))])
            .build();

        // Add previous response ID for conversation continuity
        if let Some(ref prev_id) = previous_response_id {
            request.previous_response_id = Some(prev_id.clone());
        }

        // Proactively prune expired context (this is optional but demonstrates the feature)
        if i > 1 {
            println!("   🧹 Proactively pruning expired context...");
            request = client.responses.prune_expired_context_manual(request);
        }

        match client.responses.create(request).await {
            Ok(response) => {
                println!(
                    "   ✅ Success: {}",
                    response.output_text().chars().take(100).collect::<String>()
                );
                previous_response_id = Some(response.id().to_string());
            }
            Err(e) => {
                println!("   ❌ Failed: {e}");
                // Reset conversation on error
                previous_response_id = None;
            }
        }
    }

    Ok(())
}
