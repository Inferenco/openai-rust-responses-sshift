#![allow(deprecated)] // Example demonstrates deprecated methods for backward compatibility

//! Streaming example showing real-time response streaming
//!
//! Features enhanced streaming capabilities:
//! - Image generation progress events with partial images
//! - Enhanced event handling and helper methods
//! - Better statistics and monitoring
//! - New streaming events for improved user experience
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
        use open_ai_rust_responses_by_sshift::{Client, Model, Request, Tool};

        // Create client from environment variable
        let client = Client::from_env()?;

        println!("🌊 OpenAI Rust Responses - Enhanced Streaming Example");
        println!("=====================================================\n");

        // Create a request that might use multiple tools including enhanced image generation
        let request = Request::builder()
            .model(Model::GPT4oMini) // Updated to use GPT-4o Mini
            .input("Write a short, engaging story about a robot who discovers music for the first time, and if appropriate, describe what visual elements could accompany this story.")
            .instructions("Be creative and descriptive, consider visual elements that might enhance the story")
            .max_output_tokens(400) // Use preferred parameter
            .temperature(0.8)
            .parallel_tool_calls(true) // Enable parallel tool execution
            .user("streaming-example") // Add user tracking
            .tools(vec![
                Tool::web_search_preview(),
                Tool::image_generation(),
            ])
            .build();

        println!("📤 Starting enhanced streaming request...");
        println!("🔧 Tools available: web search, image generation");
        println!("✨ Features: parallel tools, enhanced monitoring");
        println!("📖 Response:\n");
        print!("   "); // Indent for the response

        // Create the stream
        let mut stream = client.responses.stream(request);
        let mut total_chunks = 0;
        let mut total_chars = 0;
        let mut image_events = 0;
        let mut tool_calls = 0;
        let mut error_events = 0;
        let start_time = std::time::Instant::now();

        // Process the stream with enhanced event handling
        while let Some(event) = stream.next().await {
            match event {
                Ok(stream_event) => match stream_event {
                    StreamEvent::TextDelta { content, index: _ } => {
                        print!("{content}");
                        std::io::Write::flush(&mut std::io::stdout())?; // Flush to show immediately
                        total_chunks += 1;
                        total_chars += content.len();
                    }
                    StreamEvent::TextStop { index } => {
                        println!("\n📝 Text stream {index} stopped");
                    }
                    StreamEvent::ToolCallCreated { id, name, index: _ } => {
                        println!("\n🛠️ Tool call created: {name} ({id})");
                        tool_calls += 1;
                    }
                    StreamEvent::ToolCallDelta {
                        content,
                        id: _,
                        index: _,
                    } => {
                        print!("{content}");
                        std::io::Write::flush(&mut std::io::stdout())?;
                    }
                    StreamEvent::ToolCallCompleted { id, index: _ } => {
                        println!("\n✅ Tool call completed: {id}");
                    }
                    // Note: ImageProgress event is for the deprecated partials tool.
                    // The new built-in tool returns the full image in an ImageGenerationCall.
                    // This event is kept for backward compatibility tests but won't be triggered by Tool::image_generation().
                    StreamEvent::ImageProgress { .. } => {
                        image_events += 1;
                        println!("\n📸 (Legacy) Image progress event received.");
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
                },
                Err(e) => {
                    error_events += 1;
                    println!("\n❌ Stream error: {e}");
                    println!("   This demonstrates proper error handling for streaming");

                    // Demonstrate enhanced error classification
                    println!("   🔍 Error Analysis:");
                    println!("      Error type: {:?}", std::mem::discriminant(&e));
                    println!("      User message: {msg}", msg=e.user_message());

                    if e.is_recoverable() {
                        println!("      🔄 This error is recoverable");
                        if let Some(retry_after) = e.retry_after() {
                            println!("      ⏱️ Suggested retry delay: {retry_after}s");
                        }
                    } else {
                        println!("      ❌ This error is not recoverable");
                    }

                    if e.is_transient() {
                        println!("      ⚡ This is a transient error");
                    }

                    // In a real application, you might want to retry or handle the error appropriately
                    break;
                }
            }
        }

        let duration = start_time.elapsed();

        println!("\n📊 Enhanced Stream Statistics:");
        println!("   📦 Total text chunks: {total_chunks}");
        println!("   📝 Total characters: {total_chars}");
        println!("   🛠️ Tool calls made: {tool_calls}");
        println!("   📸 (Legacy) Image events: {image_events}");
        println!("   ❌ Error events: {error_events}");
        println!("   ⏱️ Stream duration: {:.2}s", duration.as_secs_f64());

        if total_chunks > 0 {
            println!(
                "   ⚡ Average chunk size: {:.1} characters",
                total_chars as f64 / total_chunks as f64
            );
            println!(
                "   🚀 Streaming rate: {:.1} chars/sec",
                total_chars as f64 / duration.as_secs_f64()
            );
        }

        // Demonstrate streaming helper methods
        println!("\n🔧 Event Helper Method Demo:");

        // Create sample events to show helper methods
        let text_event = StreamEvent::TextDelta {
            content: "Sample text".to_string(),
            index: 0,
        };
        let image_event = StreamEvent::ImageProgress {
            url: Some("https://example.com/partial-image-1.jpg".to_string()),
            index: 0,
        };
        let tool_event = StreamEvent::ToolCallDelta {
            id: "call_123".to_string(),
            content: "tool output".to_string(),
            index: 0,
        };

        println!("   📝 Text delta helper: {:?}", text_event.as_text_delta());
        println!(
            "   📸 Image progress helper: {:?}",
            image_event.as_image_progress()
        );
        println!(
            "   🔧 Tool call delta helper: {:?}",
            tool_event.as_tool_call_delta()
        );
        println!("   🏁 Is done check: {done}", done=text_event.is_done());
        println!("   ✅ Done event check: {done}", done=StreamEvent::Done.is_done());

        println!("\n✨ Streaming Enhancements:");
        println!("   🖼️ Built-in Image Generation - Full image data in a single response item");
        println!("   🔧 Parallel Tool Execution - Multiple tools running simultaneously");
        println!("   📝 Enhanced Text Events - Better granular control and monitoring");
        println!("   🛠️ Helper Methods - Convenient event data extraction");
        println!("   📊 Detailed Statistics - Comprehensive performance monitoring");
        println!("   ⚡ Real-time Metrics - Live streaming performance data");
        println!("   🎯 User Tracking - Request attribution and analytics");

        println!("\n🎸 Technical Improvements:");
        println!("   • Enhanced event parsing for better reliability");
        println!("   • Improved error handling with detailed error events");
        println!("   • Performance metrics for streaming optimization");
        println!("   • Helper methods for common event data extraction patterns");

        println!("\n✅ Enhanced streaming example completed!");
        println!(
            "💡 Try varying the prompt to see different tool interactions and streaming patterns!"
        );

        return Ok(());
    }
}
