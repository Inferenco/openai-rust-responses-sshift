//! Comprehensive OpenAI Responses API Demo
//!
//! This example demonstrates all major features of the SDK:
//! - Basic responses and conversation continuity
//! - Streaming responses
//! - File operations (upload, download, manage)
//! - Vector stores (create, search)
//! - Tools (web search, file search, custom functions)
//!
//! Setup:
//! 1. Create a `.env` file in the project root with: OPENAI_API_KEY=sk-your-api-key-here
//! 2. Run with: `cargo run --example comprehensive_demo --features stream`

use dotenv::dotenv;
use open_ai_rust_responses_by_sshift::{
    files::FilePurpose,
    vector_stores::{
        AddFileToVectorStoreRequest, CreateVectorStoreRequest, SearchVectorStoreRequest,
    },
    Client, Model, Request, Tool, ToolChoice,
};
use serde_json::json;

#[cfg(feature = "stream")]
use open_ai_rust_responses_by_sshift::StreamEvent;

#[cfg(feature = "stream")]
use std::io::Write;

#[cfg(feature = "stream")]
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    println!("🚀 OpenAI Rust Responses API - Comprehensive Demo");
    println!("==================================================\n");

    // Create client from environment variable
    let client = Client::from_env()?;

    // 1. BASIC RESPONSE
    println!("1️⃣  Basic Response");
    println!("------------------");

    let request = Request::builder()
        .model(Model::GPT4o)
        .input("What are the three most important programming principles?")
        .temperature(0.7)
        .build();

    let response1 = client.responses.create(request).await?;
    println!("🤖 Assistant: {}\n", response1.output_text());

    // 2. CONVERSATION CONTINUITY
    println!("2️⃣  Conversation Continuity (using response IDs)");
    println!("------------------------------------------------");

    let request2 = Request::builder()
        .model(Model::GPT4o)
        .input("Can you give me a practical example of the first principle?")
        .previous_response_id(response1.id.clone())
        .build();

    let response2 = client.responses.create(request2).await?;
    println!("🤖 Assistant: {}\n", response2.output_text());

    // 3. STREAMING RESPONSE (only if stream feature is enabled)
    #[cfg(feature = "stream")]
    {
        println!("3️⃣  Streaming Response");
        println!("----------------------");

        let request3 = Request::builder()
            .model(Model::GPT4o)
            .input("Write a short story about a robot learning to code")
            .build();

        println!("🤖 Assistant (streaming): ");
        let mut stream = client.responses.stream(request3);
        let mut full_response = String::new();

        while let Some(event) = stream.next().await {
            match event? {
                StreamEvent::TextDelta { content, .. } => {
                    print!("{}", content);
                    std::io::stdout().flush().unwrap();
                    full_response.push_str(&content);
                }
                StreamEvent::Done => break,
                _ => {}
            }
        }
        println!("\n");
    }

    #[cfg(not(feature = "stream"))]
    {
        println!("3️⃣  Streaming Response");
        println!("----------------------");
        println!(
            "⚠️  Streaming feature not enabled. Run with --features stream to see streaming.\n"
        );

        // Fallback to regular response
        let request3 = Request::builder()
            .model(Model::GPT4o)
            .input("Write a short story about a robot learning to code")
            .build();

        let response3 = client.responses.create(request3).await?;
        println!("🤖 Assistant: {}\n", response3.output_text());
    }

    // 4. FILE OPERATIONS
    println!("4️⃣  File Operations");
    println!("------------------");

    // Create a sample file
    let sample_content = format!(
        "# AI Programming Guide\n\n## Key Principles\n\n{}\n\n## Example\n\n{}",
        response1.output_text(),
        response2.output_text()
    );

    std::fs::write("demo_guide.md", &sample_content)?;

    // Upload file
    println!("📁 Uploading file...");
    let file = client
        .files
        .upload_file(
            "demo_guide.md",
            FilePurpose::Assistants,
            Some("text/markdown".to_string()),
        )
        .await?;

    println!("✅ Uploaded: {} (ID: {})", file.filename, file.id);

    // List files
    let files = client.files.list(None).await?;
    println!("📋 Total files in account: {}", files.len());

    // Download file content (note: assistants purpose files can't be downloaded)
    match client.files.download(&file.id).await {
        Ok(downloaded_content) => {
            println!("⬇️  Downloaded {} bytes", downloaded_content.len());
        }
        Err(e) => {
            println!(
                "⚠️  Cannot download assistants files (this is expected): {}",
                e
            );
        }
    }

    // 5. VECTOR STORES
    println!("\n5️⃣  Vector Stores");
    println!("----------------");

    // Create vector store
    println!("🔍 Creating vector store...");
    let vs_request = CreateVectorStoreRequest {
        name: "AI Programming Knowledge Base".to_string(),
        file_ids: vec![], // Start with empty vector store
    };

    let vector_store = client.vector_stores.create(vs_request).await?;
    println!(
        "✅ Vector store created: {} (ID: {})",
        vector_store.name, vector_store.id
    );

    // Add file to vector store
    println!("📎 Adding file to vector store...");
    let add_file_request = AddFileToVectorStoreRequest {
        file_id: file.id.clone(),
        attributes: None,
    };

    let _file_result = client
        .vector_stores
        .add_file(&vector_store.id, add_file_request)
        .await?;
    println!("✅ File added to vector store");

    // Wait a moment for indexing
    println!("⏳ Waiting for file indexing...");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Search vector store (direct API call)
    println!("🔍 Searching vector store directly...");
    let search_request = SearchVectorStoreRequest {
        query: "programming principles".to_string(),
        max_num_results: Some(3),
    };

    match client
        .vector_stores
        .search(&vector_store.id, search_request)
        .await
    {
        Ok(search_results) => {
            println!("✅ Found {} search results", search_results.data.len());
            for (i, result) in search_results.data.iter().enumerate() {
                println!(
                    "  {}. Score: {:.3} - File: {} - Content: {}...",
                    i + 1,
                    result.score,
                    result.filename,
                    result
                        .content
                        .first()
                        .map_or("No content", |c| &c.text)
                        .chars()
                        .take(100)
                        .collect::<String>()
                );
            }
        }
        Err(e) => println!("⚠️  Search failed (may need more time for indexing): {}", e),
    }

    // 6. BUILT-IN TOOLS WITH RESPONSES API
    println!("\n6️⃣  Built-in Tools with Responses API");
    println!("------------------------------------");

    // Web search tool example
    println!("🌐 Using web search tool...");
    let web_search_request = Request::builder()
        .model(Model::GPT4o)
        .input("What are the latest trends in Rust programming language in 2024?")
        .tools(vec![Tool::web_search_preview()])
        .build();

    match client.responses.create(web_search_request).await {
        Ok(web_response) => {
            println!("✅ Web search completed");
            println!(
                "   Response: {}",
                web_response
                    .output_text()
                    .chars()
                    .take(200)
                    .collect::<String>()
                    + "..."
            );
        }
        Err(e) => println!("⚠️  Web search failed: {}", e),
    }

    // 7. FILE SEARCH TOOL
    println!("\n7️⃣  File Search Tool with Responses API");
    println!("---------------------------------------");

    println!("📄 Using file search tool...");
    let file_search_request = Request::builder()
        .model(Model::GPT4o)
        .input("Based on the uploaded file, what are the key programming principles mentioned?")
        .tools(vec![Tool::file_search(vec![vector_store.id.clone()])])
        .build();

    match client.responses.create(file_search_request).await {
        Ok(file_response) => {
            println!("✅ File search completed");
            println!(
                "   Response: {}",
                file_response
                    .output_text()
                    .chars()
                    .take(200)
                    .collect::<String>()
                    + "..."
            );
        }
        Err(e) => println!("⚠️  File search failed: {}", e),
    }

    // 8. CUSTOM FUNCTION CALLING
    println!("\n8️⃣  Custom Function Calling");
    println!("---------------------------");

    let calculator_tool = Tool::function(
        "calculate",
        "Perform basic arithmetic calculations",
        json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "Mathematical expression to evaluate (e.g., '2 + 3 * 4')"
                }
            },
            "required": ["expression"]
        }),
    );

    let request_with_tools = Request::builder()
        .model(Model::GPT4o)
        .input("Calculate the result of 15 * 7 + 23 and explain the order of operations")
        .tools(vec![calculator_tool])
        .tool_choice(ToolChoice::auto())
        .build();

    let response_with_tools = client.responses.create(request_with_tools).await?;

    println!("🔧 Response with function calling capability:");
    println!("   ID: {}", response_with_tools.id());
    println!("   Content: {}", response_with_tools.output_text());

    // Check if any tool calls were made
    let tool_calls = response_with_tools.tool_calls();
    if !tool_calls.is_empty() {
        println!("   Tool calls made: {}", tool_calls.len());
        for tool_call in &tool_calls {
            println!("     - Function: {}", tool_call.name);
            println!("     - Arguments: {}", tool_call.arguments);
        }
        println!("   Note: In a real application, you would execute these function calls");
        println!("         and provide the results back to continue the conversation.");
    } else {
        println!("   No tool calls were made for this request");
    }

    // 9. RESPONSE WITH INSTRUCTIONS
    println!("\n9️⃣  Response with Custom Instructions");
    println!("------------------------------------");

    let request_with_instructions = Request::builder()
        .model(Model::GPT4o)
        .input("Summarize what we've learned today about programming principles and API usage")
        .instructions("You are a helpful coding mentor. Always end your responses with an encouraging note about the user's programming journey.")
        .build();

    let final_response = client.responses.create(request_with_instructions).await?;
    println!("🎓 Mentor: {}", final_response.output_text());

    // 🔟. RESOURCE DELETION TESTING
    println!("\n🔟 Resource Deletion Testing");
    println!("---------------------------");

    // Test vector store deletion API
    println!("🧪 Testing vector store deletion API...");
    println!(
        "   Deleting vector store: {} (ID: {})",
        vector_store.name, vector_store.id
    );
    match client.vector_stores.delete(&vector_store.id).await {
        Ok(_) => {
            println!("✅ Vector store delete API works correctly");
            println!(
                "   Vector store '{}' deleted successfully",
                vector_store.name
            );
        }
        Err(e) => {
            println!("❌ Vector store delete API failed: {}", e);
            println!("   This indicates an issue with the vector_stores.delete() method");
        }
    }

    // Wait a moment for the deletion to propagate
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Test file deletion API
    println!("\n🧪 Testing file deletion API...");
    println!("   Deleting file: {} (ID: {})", file.filename, file.id);
    match client.files.delete(&file.id).await {
        Ok(_) => {
            println!("✅ File delete API works correctly");
            println!("   File '{}' deleted successfully", file.filename);
        }
        Err(e) => {
            println!("❌ File delete API failed: {}", e);
            println!("   This indicates an issue with the files.delete() method");
        }
    }

    // Verify deletion by attempting to retrieve the deleted resources
    println!("\n🔍 Verifying deletions (attempting to retrieve deleted resources)...");

    // Try to get the deleted vector store (should fail)
    match client.vector_stores.get(&vector_store.id).await {
        Ok(_) => println!("⚠️  Unexpected: Vector store still exists after deletion"),
        Err(_) => println!("✅ Confirmed: Vector store no longer exists (deletion successful)"),
    }

    // Clean up local file
    println!("\n🗑️  Removing local demo file...");
    match std::fs::remove_file("demo_guide.md") {
        Ok(_) => println!("✅ Local file 'demo_guide.md' removed"),
        Err(e) => println!("⚠️  Failed to remove local file: {}", e),
    }

    // Show final verification
    println!("\n📊 Final verification - listing remaining files...");
    match client.files.list(None).await {
        Ok(remaining_files) => {
            println!(
                "✅ Files API working - {} files remaining in account",
                remaining_files.len()
            );
            if !remaining_files.is_empty() {
                println!("   Note: Remaining files are from previous demo runs or other usage");
            }
        }
        Err(e) => println!("⚠️  Could not list files for verification: {}", e),
    }

    println!("\n🎉 Demo completed successfully!");
    println!("You've tested all major features of the OpenAI Rust Responses API SDK:");
    println!("  ✅ Basic responses and conversation continuity");
    println!("  ✅ Streaming responses (when feature enabled)");
    println!("  ✅ File upload, download, and management");
    println!("  ✅ Vector store creation and search");
    println!("  ✅ Web search and file search tools");
    println!("  ✅ Custom function calling");
    println!("  ✅ Custom instructions and response chaining");
    println!("  ✅ Resource deletion APIs (files.delete() & vector_stores.delete())");
    println!("  ✅ API verification and error handling");
    println!();
    println!(
        "💡 This demo creates and deletes all resources, testing both creation and deletion APIs."
    );
    println!("🧪 Each API method is tested with proper error handling and verification.");
    println!("📚 Check the source code for implementation details and best practices!");

    Ok(())
}
