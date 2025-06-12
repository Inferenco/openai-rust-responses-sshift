//! Simple Image Input (Vision) Example
//!
//! Demonstrates two modes:
//! 1. Single-image description
//! 2. Two-image comparison (multi-image input)
//!
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

    // -----------------------------
    // 1️⃣  Single-image description
    // -----------------------------

    // First image URL (provided earlier)
    let image_url_1 = "https://storage.googleapis.com/sshift-gpt-bucket/ledger-app/generated-image-1746132697428.png";

    println!("🖼️  Requesting description for image 1: {image_url_1}");

    // Build a request for a single image
    let request_single = Request::builder()
        .model(Model::GPT4o) // GPT-4o supports vision inputs
        .input_image_url(image_url_1)
        .instructions("Describe the image in detail, mentioning colours, objects, and mood.")
        .max_output_tokens(300)
        .build();

    // Send the request
    let response_single = client.responses.create(request_single).await?;

    // Print the description
    println!(
        "\n🤖 Assistant description (image 1):\n{}",
        response_single.output_text()
    );

    // --------------------------------------------
    // 2️⃣  Two-image comparison (multi-image input)
    // --------------------------------------------

    let image_url_2 = "https://storage.googleapis.com/sshift-gpt-bucket/quark/images/bc841573-b9bf-46c1-b417-266fbb1e91d0.png";

    println!("\n🖼️🖼️  Requesting comparison of two images:\n  • {image_url_1}\n  • {image_url_2}");

    // Build a request that contains *both* images.
    // We use push_image_url twice, but you could also use input_image_urls(&[..])
    let request_compare = Request::builder()
        .model(Model::GPT4o)
        .push_image_url(image_url_1)
        .push_image_url(image_url_2)
        .instructions("Compare the two images, highlighting key similarities and differences.")
        .max_output_tokens(400)
        .build();

    // Send the request
    let response_compare = client.responses.create(request_compare).await?;

    // Print the comparison
    println!(
        "\n🤖 Assistant comparison:\n{}",
        response_compare.output_text()
    );

    Ok(())
}
