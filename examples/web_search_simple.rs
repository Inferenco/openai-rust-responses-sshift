use open_ai_rust_responses_by_sshift::{Client, Request, Model};
use open_ai_rust_responses_by_sshift::types::{Tool, ToolChoice};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API key from environment variable
    let client = Client::from_env()?;
    
    // Create a request with web search tool enabled
    let request = Request::builder()
        .model(Model::O4Mini)
        .input("What are the latest developments in Rust programming language in 2024?")
        .tools(vec![Tool::web_search_preview()])
        .tool_choice(ToolChoice::auto())
        .build();
    
    // Send the request
    let response = client.responses.create(request).await?;
    
    // Print the response
    println!("Response: {}", response.output_text());
    
    Ok(())
} 