//! Image generation example using both direct API and the built-in tool
//!
//! This example shows:
//! - Direct Images API usage
//! - Conversation-integrated image generation using the built-in tool

use base64::{engine::general_purpose, Engine as _};
use open_ai_rust_responses_by_sshift::{
    Client, ImageGenerateRequest, Model, Request, ResponseItem, Tool,
};
use std::io::Write;

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
                    println!("   Generated image URL: {url}");
                } else if let Some(_b64) = &image.b64_json {
                    println!("   Generated image (base64 data available)");
                }
            }
        }
        Err(e) => {
            println!("   Error generating image: {e}");
        }
    }

    // Method 2: Conversation-integrated image generation with the built-in tool
    println!("\n💬 Conversation with built-in image generation:");
    let request = Request::builder()
        .model(Model::GPT4oMini)
        .input("Please generate an image of a futuristic city skyline")
        .tools(vec![
            Tool::image_generation(), // Use the new built-in tool!
        ])
        .build();

    match client.responses.create(request).await {
        Ok(response) => {
            let mut image_saved = false;
            for item in &response.output {
                if let ResponseItem::ImageGenerationCall { result, .. } = item {
                    println!("   🖼️ Image data found, decoding and saving...");
                    let image_bytes = general_purpose::STANDARD.decode(result)?;
                    let file_name = "futuristic_city.png";
                    let mut file = std::fs::File::create(file_name)?;
                    file.write_all(&image_bytes)?;
                    println!("   ✅ Image saved successfully as '{file_name}'");
                    image_saved = true;
                    break;
                }
            }
            if !image_saved {
                println!("   ⚠️ No image generation output found in the response.");
                println!("      Final response: {}", response.output_text());
            }
        }
        Err(e) => {
            println!("   Error in conversation: {e}");
        }
    }

    println!("\n✅ Demo completed!");
    println!("Note: The conversation method now uses the built-in `image_generation` tool.");
    println!("      The model handles the generation and returns the image data directly.");

    Ok(())
}
