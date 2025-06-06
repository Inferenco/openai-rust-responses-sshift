//! Built-in Image Generation Example
//!
//! This example demonstrates how to use the built-in `image_generation` tool.
//! Unlike the function-based approach, this tool is handled directly by the model,
//! which returns the generated image as a base64-encoded string.
//!
//! Setup:
//! 1. Create a `.env` file in the project root with: OPENAI_API_KEY=sk-your-api-key-here
//! 2. Run with: `cargo run --example image_generation_builtin`

use open_ai_rust_responses_by_sshift::{Client, Model, Request, Tool, ResponseItem};
use std::fs::File;
use std::io::Write;
use base64::{Engine as _, engine::general_purpose};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    println!("🎨 Built-in Image Generation Demo");
    println!("================================");

    // Create a client from the environment variable
    let client = Client::from_env()?;

    // Create a request using the built-in image_generation tool
    let request = Request::builder()
        .model(Model::GPT4oMini)
        .input("Generate an image of a rusty robot programming on a vintage computer")
        .tools(vec![Tool::image_generation()])
        .build();

    println!("🤖 Sending request to generate image...");

    let response = client.responses.create(request).await?;

    println!("✅ Response received!");

    // Find the image data in the response output
    let mut image_saved = false;
    for item in &response.output {
        if let ResponseItem::ImageGenerationCall { result, .. } = item {
            println!("🖼️ Image data found, decoding and saving...");

            // Decode the base64 string
            let image_bytes = general_purpose::STANDARD.decode(result)?;

            // Save the image to a file
            let file_name = "rusty_robot.png";
            let mut file = File::create(file_name)?;
            file.write_all(&image_bytes)?;

            println!("✅ Image saved successfully as '{}'", file_name);
            image_saved = true;
            break;
        }
    }

    if !image_saved {
        println!("⚠️ No image generation output found in the response.");
        println!("   Full response: {}", response.output_text());
    }

    Ok(())
} 