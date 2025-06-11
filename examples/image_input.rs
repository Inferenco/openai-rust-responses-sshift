//! Simple Image Input (Vision) Example
//!
//! Supplies an image URL to GPT-4o and asks for a description.
//! Run with:
//!   cargo run --example image_input --features stream

use dotenv::dotenv;
use open_ai_rust_responses_by_sshift::{Client, Model, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the OPENAI_API_KEY from .env if present
    dotenv().ok();

    // Initialize the SDK client
    let client = Client::from_env()?;

    // Publicly hosted demo image
    let image_url = "https://storage.googleapis.com/sshift-gpt-bucket/ledger-app/generated-image-1746132697428.png";

    println!("🖼️  Requesting description for image: {image_url}");

    // Build a request using the new helper
    let request = Request::builder()
        .model(Model::GPT4o) // GPT-4o supports vision inputs
        .input_image_url(image_url)
        .instructions("Describe the image in detail, mentioning colours, objects, and mood.")
        .max_output_tokens(300)
        .build();

    // Send the request
    let response = client.responses.create(request).await?;

    // Print the description
    println!("\n🤖 Assistant description:\n{}", response.output_text());

    Ok(())
}
