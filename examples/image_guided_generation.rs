//! Image-Guided Generation Example
//!
//! This example demonstrates using input images to guide image generation with the OpenAI Responses API.
//! It shows how to:
//! - Pass user input images as guides for the GPT Image 1 model
//! - Use multiple input images to influence generation
//! - Configure the image generation tool with advanced parameters
//! - Extract and save the generated images
//!
//! This matches the workflow shown in Python where you can provide input images
//! and ask the model to create something based on those images.
//!
//! Setup:
//! 1. Create a `.env` file in the project root with: OPENAI_API_KEY=sk-your-api-key-here
//! 2. Run with: `cargo run --example image_guided_generation`

use base64::{engine::general_purpose, Engine as _};
use open_ai_rust_responses_by_sshift::{Client, InputItem, Model, Request, ResponseItem, Tool};
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    println!("🎨 Image-Guided Generation Example");
    println!("==================================");

    let client = Client::from_env()?;

    // ==========================================
    // 1. Single Image as Guide
    // ==========================================

    println!("\n🖼️ 1. Using Single Image as Guide");
    println!("----------------------------------");

    // Example: Use a landscape image to guide generation of an artistic version
    let guide_image_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/d/dd/Gfp-wisconsin-madison-the-nature-boardwalk.jpg/2560px-Gfp-wisconsin-madison-the-nature-boardwalk.jpg";

    let single_image_request = Request::builder()
        .model(Model::GPT4o)
        .input_items(vec![
            // System message setting the context
            InputItem::message("system", vec![
                InputItem::content_text("You are an expert artist who creates beautiful artwork based on reference images.")
            ]),
            // User message with input image and instructions
            InputItem::message("user", vec![
                InputItem::content_text("Create an artistic interpretation of this landscape in the style of a watercolor painting. Make it dreamy and ethereal."),
                InputItem::content_image_with_detail(guide_image_url, "high")
            ])
        ])
        .tools(vec![Tool::image_generation()])
        .temperature(0.8)
        .build();

    println!("🎯 Generating artistic interpretation...");
    match client.responses.create(single_image_request).await {
        Ok(response) => {
            save_image_from_response(&response, "artistic_landscape.png")?;
            println!("✅ Generated artistic interpretation saved as 'artistic_landscape.png'");
        }
        Err(e) => println!("❌ Error: {e}"),
    }

    // ==========================================
    // 2. Multiple Images as Guides
    // ==========================================

    println!("\n🖼️🖼️ 2. Using Multiple Images as Guides");
    println!("---------------------------------------");

    // Example: Combine elements from two different images
    let image1_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/d/dd/Gfp-wisconsin-madison-the-nature-boardwalk.jpg/2560px-Gfp-wisconsin-madison-the-nature-boardwalk.jpg";
    let image2_url =
        "https://upload.wikimedia.org/wikipedia/commons/thumb/5/50/Vd-Orig.png/256px-Vd-Orig.png";

    let multi_image_request = Request::builder()
        .model(Model::GPT4o)
        .input_items(vec![
            // System message
            InputItem::message("system", vec![
                InputItem::content_text("You are a creative designer who combines elements from multiple reference images to create unique artwork.")
            ]),
            // User message with multiple input images
            InputItem::message("user", vec![
                InputItem::content_text("Create a logo that combines the natural serenity from the first image with the character from the second image. Make it modern and minimalist."),
                InputItem::content_image_with_detail(image1_url, "high"),
                InputItem::content_image_with_detail(image2_url, "high")
            ])
        ])
        .tools(vec![Tool::image_generation()])
        .temperature(0.7)
        .build();

    println!("🎯 Generating combined logo design...");
    match client.responses.create(multi_image_request).await {
        Ok(response) => {
            save_image_from_response(&response, "combined_logo.png")?;
            println!("✅ Generated combined logo saved as 'combined_logo.png'");
        }
        Err(e) => println!("❌ Error: {e}"),
    }

    // ==========================================
    // 3. Base64 Image Input (Local Files)
    // ==========================================

    println!("\n📁 3. Using Local Images as Base64 Input");
    println!("----------------------------------------");

    // If you have local images, you can encode them as base64
    // For this demo, we'll create a simple example with a small image
    let sample_image_data = create_sample_image_data();

    let base64_request = Request::builder()
        .model(Model::GPT4o)
        .input_items(vec![
            InputItem::message("system", vec![
                InputItem::content_text("You are an artist who creates variations and improvements of existing artwork.")
            ]),
            InputItem::message("user", vec![
                InputItem::content_text("Create a more vibrant and colorful version of this image, adding magical elements like sparkles or glowing effects."),
                InputItem::content_image_base64_with_detail(sample_image_data, "image/png", "high")
            ])
        ])
        .tools(vec![Tool::image_generation()])
        .temperature(0.9)
        .build();

    println!("🎯 Generating enhanced version from base64 input...");
    match client.responses.create(base64_request).await {
        Ok(response) => {
            save_image_from_response(&response, "enhanced_image.png")?;
            println!("✅ Generated enhanced image saved as 'enhanced_image.png'");
        }
        Err(e) => println!("❌ Error: {e}"),
    }

    // ==========================================
    // 4. Style Transfer Example
    // ==========================================

    println!("\n🎨 4. Style Transfer with Input Images");
    println!("-------------------------------------");

    let content_image = "https://upload.wikimedia.org/wikipedia/commons/thumb/d/dd/Gfp-wisconsin-madison-the-nature-boardwalk.jpg/2560px-Gfp-wisconsin-madison-the-nature-boardwalk.jpg";

    let style_transfer_request = Request::builder()
        .model(Model::GPT4o)
        .input_items(vec![
            InputItem::message("system", vec![
                InputItem::content_text("You are an expert in artistic style transfer, capable of reimagining images in different artistic styles.")
            ]),
            InputItem::message("user", vec![
                InputItem::content_text("Transform this landscape into the style of Van Gogh's Starry Night - with swirling skies, bold brushstrokes, and vibrant blues and yellows."),
                InputItem::content_image_with_detail(content_image, "high")
            ])
        ])
        .tools(vec![Tool::image_generation()])
        .temperature(0.8)
        .max_output_tokens(2048)
        .build();

    println!("🎯 Applying Van Gogh style transfer...");
    match client.responses.create(style_transfer_request).await {
        Ok(response) => {
            save_image_from_response(&response, "van_gogh_style.png")?;
            println!("✅ Generated Van Gogh style image saved as 'van_gogh_style.png'");

            // Also print the model's description of what it created
            println!("🎨 Model description: {}", response.output_text());
        }
        Err(e) => println!("❌ Error: {e}"),
    }

    // ==========================================
    // 5. Product Design Based on References
    // ==========================================

    println!("\n🏷️ 5. Product Design from Reference Images");
    println!("-------------------------------------------");

    let product_request = Request::builder()
        .model(Model::GPT4o)
        .input_items(vec![
            InputItem::message("system", vec![
                InputItem::content_text("You are a product designer who creates modern, sleek product designs based on reference images and concepts.")
            ]),
            InputItem::message("user", vec![
                InputItem::content_text("Design a modern water bottle that captures the serenity and natural beauty of this landscape. Make it minimalist, elegant, and eco-friendly looking."),
                InputItem::content_image_with_detail(guide_image_url, "high")
            ])
        ])
        .tools(vec![Tool::image_generation()])
        .temperature(0.6)
        .build();

    println!("🎯 Designing nature-inspired water bottle...");
    match client.responses.create(product_request).await {
        Ok(response) => {
            save_image_from_response(&response, "nature_water_bottle.png")?;
            println!("✅ Generated product design saved as 'nature_water_bottle.png'");
        }
        Err(e) => println!("❌ Error: {e}"),
    }

    println!("\n✅ Image-guided generation examples completed!");
    println!("   • Demonstrated single image as generation guide");
    println!("   • Showed multiple images for combined inspiration");
    println!("   • Used base64 encoding for local image input");
    println!("   • Applied artistic style transfer");
    println!("   • Created product designs from reference images");
    println!("\n💡 Key takeaway: Input images provide powerful context for the GPT Image 1 model,");
    println!("   allowing for guided generation, style transfer, and creative combinations!");

    Ok(())
}

/// Helper function to save images from response
fn save_image_from_response(
    response: &open_ai_rust_responses_by_sshift::Response,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for item in &response.output {
        if let ResponseItem::ImageGenerationCall { result, .. } = item {
            let image_bytes = general_purpose::STANDARD.decode(result)?;
            let mut file = File::create(filename)?;
            file.write_all(&image_bytes)?;
            println!("💾 Image saved as '{filename}'");
            return Ok(());
        }
    }
    println!("⚠️ No image found in response for {filename}");
    Ok(())
}

/// Creates sample base64 image data for demonstration
/// In a real application, you would read actual image files
fn create_sample_image_data() -> String {
    // This is a minimal 1x1 PNG image in base64 for demonstration
    // In practice, you would read actual image files using std::fs::read()
    // and then encode them with base64::encode()
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==".to_string()
}
