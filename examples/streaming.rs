//! Streaming example showing real-time response streaming
//!
//! Features enhanced streaming with new Phase 1 capabilities:
//! - Image generation progress events
//! - Enhanced event handling
//! - Better statistics and monitoring
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
        use open_ai_rust_responses_by_sshift::types::Container;
        use open_ai_rust_responses_by_sshift::types::StreamEvent;
        use open_ai_rust_responses_by_sshift::{Client, Model, Request, Tool};

        // Create client from environment variable
        let client = Client::from_env()?;

        println!("🌊 OpenAI Rust Responses - Enhanced Streaming Example (Phase 1)");
        println!("================================================================\n");

        // Create a request that might use multiple tools including image generation
        let request = Request::builder()
            .model(Model::GPT4o)
            .input("Write a short, engaging story about a robot who discovers music for the first time, and if appropriate, describe what visual elements could accompany this story.")
            .max_tokens(300)
            .temperature(0.8)
            .tools(vec![
                Tool::web_search_preview(),
                Tool::image_generation(Some(Container::default_type())),
            ])
            .build();

        println!("📤 Starting enhanced streaming request...");
        println!("🔧 Tools available: web search, image generation");
        println!("📖 Response:\n");
        print!("   "); // Indent for the response

        // Create the stream
        let mut stream = client.responses.stream(request);
        let mut total_chunks = 0;
        let mut total_chars = 0;
        let mut image_events = 0;
        let mut tool_calls = 0;

        // Process the stream with enhanced event handling
        while let Some(event) = stream.next().await {
            match event? {
                StreamEvent::TextDelta { content, index: _ } => {
                    print!("{}", content);
                    std::io::Write::flush(&mut std::io::stdout())?; // Flush to show immediately
                    total_chunks += 1;
                    total_chars += content.len();
                }
                StreamEvent::TextStop { index } => {
                    println!("\n📝 Text stream {} stopped", index);
                }
                StreamEvent::ToolCallCreated { id, name, index: _ } => {
                    println!("\n🛠️ Tool call created: {} ({})", name, id);
                    tool_calls += 1;
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
                // NEW Phase 1 Feature: Image generation progress
                StreamEvent::ImageProgress { url, index } => {
                    image_events += 1;
                    if let Some(progress_url) = url {
                        println!(
                            "\n📸 Image progress ({}): Generated - {}",
                            index, progress_url
                        );
                    } else {
                        println!("\n📸 Image generation in progress ({})...", index);
                    }
                }
                StreamEvent::Done => {
                    println!("\n\n🏁 Stream completed!");
                    break;
                }
                StreamEvent::Chunk => {
                    // Heartbeat - just continue
                }
                StreamEvent::Unknown => {
                    println!("\n❓ Unknown event received (future feature)");
                }
            }
        }

        println!("\n📊 Enhanced Stream Statistics:");
        println!("   📦 Total text chunks: {}", total_chunks);
        println!("   📝 Total characters: {}", total_chars);
        println!("   🛠️ Tool calls made: {}", tool_calls);
        println!("   📸 Image events: {}", image_events);
        if total_chunks > 0 {
            println!(
                "   ⚡ Average chunk size: {:.1} characters",
                total_chars as f64 / total_chunks as f64
            );
        }

        // Demonstrate new Phase 1 streaming helper methods
        println!("\n🔧 Phase 1 Event Helper Demo:");

        // Create sample events to show helper methods
        let text_event = StreamEvent::TextDelta {
            content: "Sample text".to_string(),
            index: 0,
        };
        let image_event = StreamEvent::ImageProgress {
            url: Some("https://example.com/image.jpg".to_string()),
            index: 0,
        };

        println!("   📝 Text delta helper: {:?}", text_event.as_text_delta());
        println!(
            "   📸 Image progress helper: {:?}",
            image_event.as_image_progress()
        );
        println!("   🏁 Is done check: {}", text_event.is_done());

        println!("\n✨ Phase 1 Streaming Enhancements:");
        println!("   📸 Image Progress Events - Track visual content generation");
        println!("   📝 Enhanced Text Events - Better granular control");
        println!("   🔧 Helper Methods - Convenient event data extraction");
        println!("   📊 Detailed Statistics - Comprehensive monitoring");

        println!("\n✅ Enhanced streaming example completed!");

        return Ok(());
    }
}
