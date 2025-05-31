//! Image generation example using the pre-made function tool
//!
//! This example shows:
//! - Direct Images API usage
//! - Conversation-integrated image generation
//! - Automatic tool handling by the wrapper

use open_ai_rust_responses_by_sshift::{Client, ImageGenerateRequest, Model, Request, Tool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;

    println!("🎨 OpenAI Images API Demo\n");

    // Method 1: Direct Images API usage
    println!("📸 Direct image generation:");
    let image_request = ImageGenerateRequest::new("A serene mountain landscape at sunset")
        .with_size("1024x1024")
        .with_quality("high");

    match client.images.generate(image_request).await {
        Ok(image_response) => {
            if let Some(image) = image_response.data.first() {
                if let Some(url) = &image.url {
                    println!("   Generated image: {}", url);
                } else if let Some(_b64) = &image.b64_json {
                    println!("   Generated image (base64 data available)");
                }
            }
        }
        Err(e) => {
            println!("   Error generating image: {}", e);
        }
    }

    // Method 2: Conversation-integrated image generation
    println!("\n💬 Conversation with image generation:");
    let request = Request::builder()
        .model(Model::GPT4oMini)
        .input("Please generate an image of a futuristic city skyline and describe what you see")
        .tools(vec![
            Tool::image_generation_function(), // Pre-made tool!
        ])
        .build();

    match client.responses.create(request).await {
        Ok(response) => {
            println!("   Response: {:?}", response.output);
        }
        Err(e) => {
            println!("   Error in conversation: {}", e);
        }
    }

    println!("\n✅ Demo completed!");
    println!("Note: The conversation method uses a function tool that the AI can call");
    println!("      to generate images when requested. The wrapper handles this automatically.");

    Ok(())
}
