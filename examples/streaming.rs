//! Streaming example showing real-time response streaming
//!
//! Run with: `cargo run --example streaming --features stream`
//!
//! Make sure to set your OpenAI API key:
//! ```bash
//! export OPENAI_API_KEY=sk-your-api-key-here
//! ```

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(feature = "stream"))]
    {
        println!("❌ Streaming feature is not enabled!");
        println!("Run with: cargo run --example streaming --features stream");
        return Ok(());
    }

    #[cfg(feature = "stream")]
    {
        use futures::StreamExt;
        use open_ai_rust_responses_by_sshift::types::StreamEvent;
        use open_ai_rust_responses_by_sshift::{Client, Model, Request};

        // Create client from environment variable
        let client = Client::from_env()?;

        println!("🌊 OpenAI Rust Responses - Streaming Example");
        println!("============================================\n");

        // Create a request for a story that will benefit from streaming
        let request = Request::builder()
            .model(Model::GPT4o)
            .input("Write a short, engaging story about a robot who discovers music for the first time. Make it about 150 words.")
            .max_tokens(200)
            .temperature(0.8)
            .build();

        println!("📤 Starting streaming request...");
        println!("📖 Story:\n");
        print!("   "); // Indent for the story

        // Create the stream
        let mut stream = client.responses.stream(request);
        let mut total_chunks = 0;
        let mut total_chars = 0;

        // Process the stream
        while let Some(event) = stream.next().await {
            match event? {
                StreamEvent::TextDelta { content, index: _ } => {
                    print!("{}", content);
                    std::io::Write::flush(&mut std::io::stdout())?; // Flush to show immediately
                    total_chunks += 1;
                    total_chars += content.len();
                }
                StreamEvent::ToolCallCreated { id, name, index: _ } => {
                    println!("\n🛠️ Tool call created: {} ({})", name, id);
                }
                StreamEvent::ToolCallDelta {
                    content,
                    id: _,
                    index: _,
                } => {
                    print!("{}", content);
                    std::io::Write::flush(&mut std::io::stdout())?;
                }
                StreamEvent::ToolCallCompleted { id, index: _ } => {
                    println!("\n✅ Tool call completed: {}", id);
                }
                StreamEvent::Done => {
                    println!("\n\n🏁 Stream completed!");
                    break;
                }
                StreamEvent::Chunk => {
                    // Heartbeat - just continue
                }
                StreamEvent::Unknown => {
                    // Unknown event type - just continue
                }
                _ => {
                    // Handle any other events
                }
            }
        }

        println!("\n📊 Stream Statistics:");
        println!("   📦 Total chunks received: {}", total_chunks);
        println!("   📝 Total characters: {}", total_chars);
        println!(
            "   ⚡ Average chunk size: {:.1} characters",
            total_chars as f64 / total_chunks as f64
        );

        println!("\n✅ Streaming example completed!");
        
        return Ok(());
    }
}
