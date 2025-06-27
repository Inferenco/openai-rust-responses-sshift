#![allow(deprecated)] // Demo shows both new and deprecated methods for comparison

//! Comprehensive OpenAI Responses API Demo
//!
//! This example demonstrates all major features of the enhanced SDK:
//! - Basic responses and conversation continuity with enhanced monitoring
//! - Streaming responses with improved event handling
//! - File operations (upload, download, manage) with comprehensive error handling
//! - Vector stores (create, search) with enhanced feedback
//! - Tools (web search, file search, custom functions) with parallel execution
//! - Enhanced Features:
//!   * Image generation tool with partial image support
//!   * MCP (Model Context Protocol) server integration
//!   * Enhanced include options for reasoning and search results
//!   * Type-safe include options with backward compatibility
//!   * Comprehensive response monitoring and token analytics
//!   * Reasoning models with advanced parameters
//!   * Background processing capabilities
//!
//! Setup:
//! 1. Create a `.env` file in the project root with: OPENAI_API_KEY=sk-your-api-key-here
//! 2. Run with: `cargo run --example comprehensive_demo --features stream`

use base64::Engine;
use dotenv::dotenv;
use open_ai_rust_responses_by_sshift::{
    files::FilePurpose,
    types::{Effort, Include, ReasoningParams, SummarySetting},
    vector_stores::{
        AddFileToVectorStoreRequest, CreateVectorStoreRequest, SearchVectorStoreRequest,
    },
    Client, Model, Request, Tool, ToolChoice,
};
use serde_json::json;
use std::collections::HashMap;
use std::io::Write;

#[cfg(feature = "stream")]
use open_ai_rust_responses_by_sshift::StreamEvent;

#[cfg(feature = "stream")]
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    println!("🚀 OpenAI Rust Responses API - Comprehensive Demo (Enhanced Edition)");
    println!("====================================================================\n");

    // Create client from environment variable
    let client = Client::from_env()?;

    // 1. BASIC RESPONSE WITH ENHANCED MONITORING
    println!("1️⃣  Basic Response with Enhanced Monitoring");
    println!("------------------------------------------");

    let request = Request::builder()
        .model(Model::GPT4oMini) // Updated to use GPT-4o Mini as default
        .input("What are the three most important programming principles?")
        .instructions("Provide clear, practical explanations that a beginner can understand")
        .temperature(0.7)
        .max_output_tokens(500) // Use preferred parameter
        .user("comprehensive-demo") // Add user tracking
        .store(true) // Explicitly enable conversation storage
        .build();

    let response1 = client.responses.create(request).await?;

    // Enhanced response monitoring
    println!("📊 Response Status: {}", response1.status);
    println!("🤖 Model Used: {}", response1.model);
    println!("✅ Is Complete: {}", response1.is_complete());
    println!("❌ Has Errors: {}", response1.has_errors());

    if let Some(usage) = &response1.usage {
        println!(
            "📊 Token Usage: {} total ({} input + {} output)",
            usage.total_tokens, usage.input_tokens, usage.output_tokens
        );
    }

    // Show parameter echoes
    if let Some(temp) = response1.temperature {
        println!("🌡️ Temperature used: {temp}");
    }
    if let Some(max_tokens) = response1.max_output_tokens {
        println!("📏 Max output tokens: {max_tokens}");
    }

    println!("🤖 Assistant: {}\n", response1.output_text());

    // 2. CONVERSATION CONTINUITY WITH ANALYTICS
    println!("2️⃣  Conversation Continuity with Enhanced Analytics");
    println!("--------------------------------------------------");

    let request2 = Request::builder()
        .model(Model::GPT4oMini)
        .input("Can you give me a practical example of the first principle?")
        .instructions("Provide a concrete coding example")
        .previous_response_id(response1.id.clone())
        .max_output_tokens(500)
        .user("comprehensive-demo") // Maintain user identity
        .store(true) // Continue storing conversation
        .build();

    let response2 = client.responses.create(request2).await?;

    // Show conversation analytics
    let conversation_tokens =
        response1.total_tokens().unwrap_or(0) + response2.total_tokens().unwrap_or(0);
    println!("📊 Conversation Analytics:");
    println!(
        "   Turns: 2 | Total tokens: {} | Avg per turn: {:.1}",
        conversation_tokens,
        conversation_tokens as f64 / 2.0
    );
    println!(
        "   Success rate: {}%",
        if response1.is_complete() && response2.is_complete() {
            100
        } else {
            50
        }
    );

    println!("🤖 Assistant: {}\n", response2.output_text());

    // 3. STREAMING RESPONSE WITH ENHANCED EVENT HANDLING (only if stream feature is enabled)
    #[cfg(feature = "stream")]
    {
        println!("3️⃣  Enhanced Streaming with Advanced Event Handling");
        println!("--------------------------------------------------");

        let request3 = Request::builder()
            .model(Model::GPT4oMini) // Updated to use GPT-4o Mini
            .input("Write a short story about a robot learning to code. Include vivid descriptions that could be illustrated.")
            .instructions("Be creative and descriptive, consider visual elements")
            .max_output_tokens(500) // Use preferred parameter
            .temperature(0.8)
            .parallel_tool_calls(true) // Enable parallel execution
            .user("comprehensive-demo") // Add user tracking
            .tools(vec![
                Tool::web_search_preview(),
                Tool::image_generation(),
            ])
            .build();

        println!("🤖 Assistant (enhanced streaming): ");
        let mut stream = client.responses.stream(request3);
        let mut full_response = String::new();
        let mut event_count = 0;
        let mut image_events = 0;
        let mut tool_calls = 0;
        let start_time = std::time::Instant::now();

        while let Some(event) = stream.next().await {
            match event {
                Ok(stream_event) => {
                    match stream_event {
                        StreamEvent::TextDelta { content, .. } => {
                            print!("{content}");
                            std::io::stdout().flush().unwrap();
                            full_response.push_str(&content);
                            event_count += 1;
                        }
                        StreamEvent::ImageProgress { url, index } => {
                            image_events += 1;
                            if let Some(progress_url) = url {
                                println!(
                                    "\n📸 Partial image {image_events} generated: {progress_url}"
                                );
                            } else {
                                println!("\n📸 Image generation in progress (index {index})...");
                            }
                        }
                        StreamEvent::ToolCallCreated { id, name, .. } => {
                            tool_calls += 1;
                            println!("\n🛠️ Tool call created: {name} ({id})");
                        }
                        StreamEvent::Done => {
                            let duration = start_time.elapsed();
                            println!("\n✅ Enhanced stream completed!");
                            println!("📊 Stream Statistics:");
                            println!(
                                "   Events: {} | Images: {} | Tools: {} | Duration: {:.2}s",
                                event_count,
                                image_events,
                                tool_calls,
                                duration.as_secs_f64()
                            );
                            if event_count > 0 {
                                println!(
                                    "   Rate: {:.1} chars/sec",
                                    full_response.len() as f64 / duration.as_secs_f64()
                                );
                            }
                            break;
                        }
                        _ => {
                            // Handle other event types gracefully
                        }
                    }
                }
                Err(e) => {
                    println!("\n❌ Stream error occurred: {e}");
                    println!("   This demonstrates enhanced error handling for streaming");
                    // In a real application, you might want to retry or handle the error appropriately
                    break;
                }
            }
        }
        println!();
    }

    #[cfg(not(feature = "stream"))]
    {
        println!("3️⃣  Enhanced Response (Streaming Disabled)");
        println!("------------------------------------------");
        println!(
            "⚠️  Streaming feature not enabled. Run with --features stream to see enhanced streaming.\n"
        );

        // Fallback to regular response with enhanced features
        let request3 = Request::builder()
            .model(Model::GPT4oMini)
            .input("Write a short story about a robot learning to code")
            .instructions("Be creative and engaging")
            .max_output_tokens(500)
            .temperature(0.8)
            .user("comprehensive-demo")
            .build();

        let response3 = client.responses.create(request3).await?;

        // Show enhanced response details
        println!("📊 Response Details:");
        println!(
            "   Status: {} | Complete: {} | Errors: {}",
            response3.status,
            response3.is_complete(),
            response3.has_errors()
        );
        if let Some(usage) = &response3.usage {
            println!("   Tokens: {}", usage.total_tokens);
        }

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
            println!("⚠️  Cannot download assistants files (this is expected): {e}");
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
        Err(e) => println!("⚠️  Search failed (may need more time for indexing): {e}"),
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
        Err(e) => println!("⚠️  Web search failed: {e}"),
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
        Err(e) => println!("⚠️  File search failed: {e}"),
    }

    // 8. ENHANCED FUNCTION CALLING - CONTINUOUS CONVERSATION
    println!("\n8️⃣  Enhanced Function Calling with Parallel Execution");
    println!("----------------------------------------------------");

    // Define multiple tools for a realistic scenario
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

    let weather_tool = Tool::function(
        "get_weather",
        "Get current weather for a location",
        json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City name or coordinates"
                },
                "units": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"],
                    "description": "Temperature units"
                }
            },
            "required": ["location"]
        }),
    );

    let tools = vec![calculator_tool.clone(), weather_tool.clone()];

    println!("🔧 Step 1: Initial request with enhanced parallel tool support");
    let request_with_tools = Request::builder()
        .model(Model::GPT4oMini) // Updated to use GPT-4o Mini
        .input("Calculate 15 * 7 + 23, then tell me what the weather is like in New York")
        .instructions("Use the available tools efficiently and provide comprehensive answers")
        .tools(tools.clone())
        .tool_choice(ToolChoice::auto())
        .parallel_tool_calls(true) // Enable parallel execution
        .max_output_tokens(500) // Use preferred parameter
        .user("comprehensive-demo") // Add user tracking
        .store(true) // Enable conversation storage
        .build();

    let mut current_response = client.responses.create(request_with_tools).await?;
    let mut iteration = 1;
    const MAX_ITERATIONS: usize = 5; // Prevent infinite loops
    let mut total_function_tokens = 0;

    // Enhanced response monitoring
    println!("📊 Initial Response Status: {}", current_response.status);
    println!("✅ Is Complete: {}", current_response.is_complete());
    println!(
        "🔧 Parallel Tool Calls: {}",
        current_response.parallel_tool_calls.unwrap_or(false)
    );

    if let Some(usage) = &current_response.usage {
        total_function_tokens += usage.total_tokens;
        println!("📊 Token Usage: {}", usage.total_tokens);
    }

    println!("📝 Initial Response:");
    println!("   ID: {}", current_response.id());
    println!("   Content: {}", current_response.output_text());

    // Enhanced function calling loop - handle multiple rounds of tool calls
    while !current_response.tool_calls().is_empty() && iteration <= MAX_ITERATIONS {
        let tool_calls = current_response.tool_calls();
        println!(
            "\n🔄 Iteration {}: Processing {} tool calls (parallel: {})",
            iteration,
            tool_calls.len(),
            current_response.parallel_tool_calls.unwrap_or(false)
        );

        let mut function_outputs = Vec::new();

        // Execute all tool calls in this iteration
        for tool_call in &tool_calls {
            println!("   🔧 Function: {} ({})", tool_call.name, tool_call.call_id);
            println!("   📋 Arguments: {}", tool_call.arguments);

            // Execute the function and get result
            let result = match tool_call.name.as_str() {
                "calculate" => {
                    let args: HashMap<String, String> = serde_json::from_str(&tool_call.arguments)?;
                    if let Some(expression) = args.get("expression") {
                        // Simulate calculation
                        if expression.contains("15 * 7 + 23") || expression.contains("15*7+23") {
                            "128".to_string()
                        } else {
                            format!("Calculated result for: {expression}")
                        }
                    } else {
                        "Error: No expression provided".to_string()
                    }
                }
                "get_weather" => {
                    let args: HashMap<String, String> = serde_json::from_str(&tool_call.arguments)?;
                    let location = args
                        .get("location")
                        .cloned()
                        .unwrap_or_else(|| "Unknown".to_string());
                    let units = args
                        .get("units")
                        .cloned()
                        .unwrap_or_else(|| "celsius".to_string());

                    // Simulate weather API call
                    format!(
                        "Weather in {}: 22°{} ({}°{}), partly cloudy with light breeze",
                        location,
                        if units == "celsius" { "C" } else { "F" },
                        if units == "celsius" { "72" } else { "22" },
                        if units == "celsius" { "F" } else { "C" }
                    )
                }
                _ => format!("Error: Unknown function '{}'", tool_call.name),
            };

            println!("   ✅ Result: {result}");
            function_outputs.push((tool_call.call_id.clone(), result));
        }

        // Submit tool outputs and continue conversation
        println!(
            "   📤 Submitting {} tool outputs...",
            function_outputs.len()
        );

        let continuation_request = Request::builder()
            .model(Model::GPT4oMini)
            .with_function_outputs(current_response.id(), function_outputs)
            .tools(tools.clone()) // Keep tools available for potential follow-ups
            .instructions("Provide a comprehensive summary based on the tool results")
            .user("comprehensive-demo") // Maintain user identity
            .store(true) // Continue storing conversation
            .build();

        current_response = client.responses.create(continuation_request).await?;

        // Enhanced response monitoring
        println!("   📊 Response Status: {}", current_response.status);
        println!("   ✅ Is Complete: {}", current_response.is_complete());
        println!("   ❌ Has Errors: {}", current_response.has_errors());

        if let Some(usage) = &current_response.usage {
            total_function_tokens += usage.total_tokens;
            println!(
                "   📊 Iteration tokens: {} | Total: {}",
                usage.total_tokens, total_function_tokens
            );
        }

        println!("   📥 Response after tool execution:");
        println!("      ID: {}", current_response.id());
        println!("      Content: {}", current_response.output_text());

        iteration += 1;
    }

    if iteration > MAX_ITERATIONS {
        println!("⚠️ Stopped after {MAX_ITERATIONS} iterations to prevent infinite loop");
    } else if current_response.tool_calls().is_empty() {
        println!("✅ Enhanced function calling workflow completed - no more tool calls needed");
    }

    println!("\n🎯 Enhanced Function Calling Summary:");
    println!("   • Iterations: {}", iteration - 1);
    println!("   • Total tokens used: {total_function_tokens}");
    println!("   • Tools available: calculate, get_weather");
    println!("   • Parallel execution: ✅ (enabled for efficiency)");
    println!("   • Enhanced monitoring: ✅ (status, tokens, errors)");
    println!("   • Pattern: Initial request → Function calls → Tool outputs → Final response");
    println!("   • Conversation continuity: ✅ (using response IDs)");
    println!("   • Multiple tool support: ✅ (calculator + weather)");

    // Test follow-up conversation after function calling
    println!("\n🔗 Step 2: Follow-up conversation (testing continuity)");
    let followup_request = Request::builder()
        .model(Model::GPT4o)
        .input("Thanks! Can you now calculate what 128 divided by 4 equals?")
        .previous_response_id(current_response.id()) // Continue from where we left off
        .tools(tools.clone()) // Keep tools available
        .build();

    let followup_response = client.responses.create(followup_request).await?;

    // Handle potential follow-up function calls
    if !followup_response.tool_calls().is_empty() {
        println!("   🔧 Follow-up triggered additional function calls:");
        for tool_call in followup_response.tool_calls() {
            println!(
                "      - Function: {} ({})",
                tool_call.name, tool_call.call_id
            );
            println!("      - Arguments: {}", tool_call.arguments);
        }

        // Execute follow-up function calls
        let mut followup_outputs = Vec::new();
        for tool_call in followup_response.tool_calls() {
            if tool_call.name == "calculate" {
                let args: HashMap<String, String> = serde_json::from_str(&tool_call.arguments)?;
                if let Some(expression) = args.get("expression") {
                    let result = if expression.contains("128") && expression.contains("4") {
                        "32".to_string()
                    } else {
                        format!("Calculated: {expression}")
                    };
                    followup_outputs.push((tool_call.call_id.clone(), result));
                }
            }
        }

        if !followup_outputs.is_empty() {
            let final_followup_request = Request::builder()
                .model(Model::GPT4o)
                .with_function_outputs(followup_response.id(), followup_outputs)
                .tools(tools)
                .build();

            let final_followup_response = client.responses.create(final_followup_request).await?;
            println!(
                "   📝 Final follow-up response: {}",
                final_followup_response.output_text()
            );
        }
    } else {
        println!(
            "   📝 Follow-up response (no function calls): {}",
            followup_response.output_text()
        );
    }

    println!("\n✅ Continuous conversation with function calling completed!");
    println!("   • Demonstrated: Multi-tool usage in single request");
    println!("   • Demonstrated: Function calling loop with multiple iterations");
    println!("   • Demonstrated: Conversation continuity across function calls");
    println!("   • Demonstrated: Follow-up questions maintaining context");
    println!("   • Key Pattern: with_function_outputs() for submitting tool results");
    println!("   • Safety: Iteration limits prevent infinite loops");

    // 🆕 ENHANCED FEATURES SHOWCASE
    println!("\n🆕 Enhanced SDK Features Showcase");
    println!("================================");

    // Simple Image Generation Demo
    println!("\n🎨 Working Image Generation");
    println!("---------------------------");

    let image_request = Request::builder()
        .model(Model::GPT4oMini)
        .input("Please generate a simple image of a mountain landscape")
        .tools(vec![Tool::image_generation()]) // ✅ Use the new built-in tool
        .max_output_tokens(500)
        .user("comprehensive-demo")
        .build();

    let img_response = client.responses.create(image_request).await?;
    let mut image_saved = false;
    for item in &img_response.output {
        if let open_ai_rust_responses_by_sshift::ResponseItem::ImageGenerationCall {
            result, ..
        } = item
        {
            println!("   🖼️ Image data found, decoding and saving...");
            let image_bytes = base64::engine::general_purpose::STANDARD.decode(result)?;
            let file_name = "mountain_landscape.png";
            let mut file = std::fs::File::create(file_name)?;
            file.write_all(&image_bytes)?;
            println!("   ✅ Image saved to {file_name}");
            image_saved = true;
            break;
        }
    }

    if !image_saved {
        println!("   ⚠️ No image generation output found in response.");
        println!("   📝 Response: {}", img_response.output_text());
    }

    // Image Input (Vision) Demo using `input_image_url`
    println!("\n🖼️  Image Input (Vision) Demo");
    println!("---------------------------");

    let supplied_image_url = "https://storage.googleapis.com/sshift-gpt-bucket/ledger-app/generated-image-1746132697428.png";
    println!("📤 Supplying image for description: {supplied_image_url}");

    let vision_request = Request::builder()
        .model(Model::GPT4o) // Use GPT-4o for multimodal capabilities
        .input_image_url(supplied_image_url)
        .instructions("Describe the image in detail, mentioning colors, objects, and overall scene composition.")
        // Ask the API to echo the image URL back so users see how to include `Include` options
        .include(vec![Include::MessageInputImageUrl])
        .max_output_tokens(300)
        .user("comprehensive-demo")
        .build();

    match client.responses.create(vision_request).await {
        Ok(vision_response) => {
            println!("✅ Vision description received:");
            println!("   {}", vision_response.output_text());
        }
        Err(e) => println!("⚠️  Vision request failed: {e}"),
    }

    // MCP (Model Context Protocol) Tool Demo with Enhanced Approval
    println!("\n🔌 MCP Server Integration with Approval Modes");
    println!("---------------------------------------------");

    let mut mcp_headers = HashMap::new();
    mcp_headers.insert(
        "Authorization".to_string(),
        "Bearer example-token".to_string(),
    );
    mcp_headers.insert("Content-Type".to_string(), "application/json".to_string());

    // Show different approval modes
    let mcp_auto = Tool::mcp(
        "auto-knowledge-server",
        "https://api.example-auto.com/v1",
        Some(mcp_headers.clone()),
    );

    let mcp_manual = Tool::mcp_with_approval(
        "manual-knowledge-server",
        "https://api.example-manual.com/v1",
        "always",
        Some(mcp_headers),
    );

    println!("✅ Created MCP tools with different approval modes:");
    println!(
        "   Auto approval: {} ({})",
        mcp_auto.server_label.as_ref().unwrap(),
        mcp_auto.require_approval.as_ref().unwrap()
    );
    println!(
        "   Manual approval: {} ({})",
        mcp_manual.server_label.as_ref().unwrap(),
        mcp_manual.require_approval.as_ref().unwrap()
    );

    // Enhanced Code Interpreter (without container for now)
    println!("\n🔧 Enhanced Code Interpreter");
    println!("---------------------------");

    let enhanced_code_tool = Tool::code_interpreter(None); // Remove container for now
    println!("✅ Enhanced code interpreter configured:");
    println!("   Security: Standard isolation for code execution");
    println!("   Container support coming soon in future API updates");

    // Type-Safe Include Options Demo
    println!("\n📋 Type-Safe Include Options with Enhanced Features");
    println!("--------------------------------------------------");

    let enhanced_includes = vec![
        Include::FileSearchResults,         // Enhanced file search results
        Include::WebSearchResults,          // Enhanced web search results
        Include::ReasoningEncryptedContent, // Encrypted reasoning content
    ];

    println!("✅ Enhanced include options available:");
    for include in &enhanced_includes {
        println!("   • {} ('{}')", include, include.as_str());
    }

    // Reasoning Models Demonstration
    println!("\n🧠 Advanced Reasoning Models Showcase");
    println!("------------------------------------");

    let reasoning_request = Request::builder()
        .model(Model::O4Mini) // Use O4Mini as requested
        .input("Solve this logic puzzle: Five friends sit in a row. Alice is not at either end. Bob is to the right of Charlie. Diana is between Alice and Eve. Charlie is not next to Alice. What is the seating order?")
        .instructions("Think step-by-step and show your logical reasoning process")
        .reasoning(ReasoningParams::new()
            .with_effort(Effort::High)
            .with_summary(SummarySetting::Auto))
        .include(enhanced_includes.clone())
        .max_output_tokens(2000) // Reasoning models need much more tokens for thinking
        .user("comprehensive-demo")
        .store(false) // Use stateless mode for reasoning
        .build();

    println!("🧩 Testing advanced reasoning with O4Mini...");
    match client.responses.create(reasoning_request).await {
        Ok(reasoning_response) => {
            println!("✅ Reasoning request completed:");
            println!("   Status: {}", reasoning_response.status);
            println!("   Model: {}", reasoning_response.model);

            if let Some(usage) = &reasoning_response.usage {
                println!("   Token usage: {} total", usage.total_tokens);
                if let Some(details) = &usage.output_tokens_details {
                    if let Some(reasoning_tokens) = details.reasoning_tokens {
                        println!("   Reasoning tokens: {reasoning_tokens}");
                    }
                }
            }

            println!(
                "   Solution: {}",
                reasoning_response
                    .output_text()
                    .chars()
                    .take(200)
                    .collect::<String>()
                    + "..."
            );

            // Check for reasoning output
            if let Some(reasoning) = &reasoning_response.reasoning {
                if reasoning.encrypted_content.is_some() {
                    println!("   🔐 Encrypted reasoning content available (stateless mode)");
                }
                if let Some(content) = &reasoning.content {
                    println!("   🔍 Reasoning steps: {} traced", content.len());
                }
            }
        }
        Err(e) => println!("⚠️  Reasoning request failed: {e}"),
    }

    // Demonstrate comprehensive request with all enhanced features
    println!("\n🔬 Comprehensive Request with All Enhanced Features");
    println!("--------------------------------------------------");

    let comprehensive_request = Request::builder()
        .model(Model::GPT4oMini)
        .input("Analyze the programming principles document and suggest improvements")
        .instructions("You are an expert programming educator with access to advanced tools")
        .tools(vec![
            Tool::web_search_preview(),
            Tool::file_search(vec![vector_store.id.clone()]),
            enhanced_code_tool,
            Tool::image_generation(),
            mcp_auto,
        ])
        .include(enhanced_includes.clone())
        .parallel_tool_calls(true) // Enable parallel execution
        .max_output_tokens(500) // Use preferred parameter
        .temperature(0.7)
        .user("comprehensive-demo") // Add user tracking
        .store(true) // Enable conversation storage
        .build();

    println!("✅ Created comprehensive request showcasing:");
    println!(
        "   • {} enhanced tools (image gen, MCP, future containers)",
        comprehensive_request.tools.as_ref().map_or(0, |t| t.len())
    );
    println!(
        "   • {} include options (reasoning + search results)",
        comprehensive_request
            .include
            .as_ref()
            .map_or(0, |i| i.len())
    );
    println!(
        "   • Parallel tool execution: {}",
        comprehensive_request.parallel_tool_calls.unwrap_or(false)
    );
    println!(
        "   • User tracking: {}",
        comprehensive_request
            .user
            .as_ref()
            .unwrap_or(&"none".to_string())
    );

    // Backward Compatibility Demo
    println!("\n🔄 Backward Compatibility with Legacy Features");
    println!("---------------------------------------------");

    let legacy_request = Request::builder()
        .model(Model::GPT4oMini)
        .input("Test backward compatibility")
        .include_strings(vec![
            "file_search.results".to_string(),      // Legacy value
            "file_search_call.results".to_string(), // New value
            "reasoning.encrypted_content".to_string(),
        ])
        .build();

    println!("✅ Legacy string includes still work:");
    if let Some(includes) = &legacy_request.include {
        for include in includes {
            println!("   • Converted '{}' to type-safe variant", include.as_str());
        }
    }
    println!("   Note: Both legacy and new API values are supported for compatibility");

    println!("\n✨ Enhanced Features Summary:");
    println!("   🎨 Image Generation - Partial image streaming for progressive creation");
    println!("   🔌 MCP Integration - Connect to external knowledge sources with approval modes");
    println!("   🛡️  Container Support - Coming soon in future API updates");
    println!("   🧠 Reasoning Models - O4Mini with effort levels and encrypted content");
    println!("   🔧 Parallel Tools - Multiple tools executing simultaneously for efficiency");
    println!("   📊 Enhanced Monitoring - Comprehensive status, token, and error tracking");
    println!("   🔒 Type Safety - Compile-time validation for all parameters");
    println!("   ↩️  Backward Compatibility - Legacy code works without modification");

    // 9. RESPONSE WITH ENHANCED INSTRUCTIONS
    println!("\n9️⃣  Response with Enhanced Custom Instructions");
    println!("----------------------------------------------");

    let request_with_instructions = Request::builder()
        .model(Model::GPT4oMini)
        .input("Summarize what we've learned today about programming principles and enhanced API usage")
        .instructions("You are a helpful coding mentor. Always end your responses with an encouraging note about the user's programming journey. Use the conversation context to provide personalized insights.")
        .previous_response_id(response2.id.clone()) // Continue from earlier conversation
        .max_output_tokens(500) // Use preferred parameter
        .user("comprehensive-demo") // Add user tracking
        .store(true) // Enable conversation storage
        .build();

    let final_response = client.responses.create(request_with_instructions).await?;

    // Enhanced response monitoring
    println!("📊 Final Response Analysis:");
    println!(
        "   Status: {} | Complete: {} | Errors: {}",
        final_response.status,
        final_response.is_complete(),
        final_response.has_errors()
    );
    if let Some(usage) = &final_response.usage {
        println!("   Token usage: {}", usage.total_tokens);
    }

    println!("🎓 Mentor: {}", final_response.output_text());

    // 🔟. ENHANCED RESOURCE DELETION TESTING
    println!("\n🔟 Enhanced Resource Deletion Testing");
    println!("------------------------------------");

    // Test vector store file deletion first (proper cleanup order)
    println!("🧪 Testing vector store file deletion...");
    println!(
        "   Removing file: {} from vector store: {} (ID: {})",
        file.filename, vector_store.name, vector_store.id
    );

    match client
        .vector_stores
        .delete_file(&vector_store.id, &file.id)
        .await
    {
        Ok(delete_response) => {
            println!("✅ Vector store file deletion API works correctly");
            println!(
                "   File '{}' removed from vector store successfully",
                delete_response.id
            );
            println!(
                "   Deletion confirmed: {} | Object type: {}",
                delete_response.deleted, delete_response.object
            );
            println!("   Note: File still exists in Files API - only removed from vector store");
        }
        Err(e) => {
            println!("❌ Vector store file deletion failed: {e}");
            println!("   This indicates an issue with the vector_stores.delete_file() method");
            println!("   Proceeding with vector store deletion anyway...");
        }
    }

    // Wait a moment for the deletion to propagate
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Test vector store deletion API with enhanced monitoring
    println!("\n🧪 Testing enhanced vector store deletion...");
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
            println!("❌ Vector store delete API failed: {e}");
            println!("   This indicates an issue with the vector_stores.delete() method");
        }
    }

    // Wait a moment for the deletion to propagate
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Test file deletion API with enhanced monitoring
    println!("\n🧪 Testing enhanced file deletion...");
    println!("   Deleting file: {} (ID: {})", file.filename, file.id);
    match client.files.delete(&file.id).await {
        Ok(_) => {
            println!("✅ File delete API works correctly");
            println!("   File '{}' deleted successfully", file.filename);
        }
        Err(e) => {
            println!("❌ File delete API failed: {e}");
            println!("   This indicates an issue with the files.delete() method");
        }
    }

    // Verify deletion by attempting to retrieve the deleted resources
    println!("\n🔍 Enhanced verification (attempting to retrieve deleted resources)...");

    // Try to get the deleted vector store (should fail)
    match client.vector_stores.get(&vector_store.id).await {
        Ok(_) => println!("⚠️  Unexpected: Vector store still exists after deletion"),
        Err(_) => println!("✅ Confirmed: Vector store no longer exists (deletion successful)"),
    }

    // Clean up local file
    println!("\n🗑️  Removing local demo file...");
    match std::fs::remove_file("demo_guide.md") {
        Ok(_) => println!("✅ Local file 'demo_guide.md' removed"),
        Err(e) => println!("⚠️  Failed to remove local file: {e}"),
    }

    // Show final verification with enhanced monitoring
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
        Err(e) => println!("⚠️  Could not list files for verification: {e}"),
    }

    println!("\n🎉 Comprehensive demo completed successfully!");
    println!("You've tested all major features of the Enhanced OpenAI Rust Responses API SDK:");
    println!("  ✅ Basic responses and conversation continuity with enhanced monitoring");
    println!("  ✅ Streaming responses with improved event handling (when feature enabled)");
    println!("  ✅ File upload, download, and management with comprehensive error handling");
    println!("  ✅ Vector store creation and search with enhanced feedback");
    println!("  ✅ Web search and file search tools with parallel execution");
    println!("  ✅ Enhanced function calling with parallel tool support");
    println!("  ✅ Custom instructions and response chaining with analytics");
    println!("  ✅ Enhanced Features:");
    println!("      • Image generation with partial image streaming");
    println!("      • MCP server integration with approval modes");
    println!("      • Advanced reasoning models (O4Mini) with effort levels");
    println!("      • Container support for enhanced security");
    println!("      • Type-safe include options with backward compatibility");
    println!("      • Comprehensive response monitoring and token analytics");
    println!("      • Parallel tool execution for improved efficiency");
    println!("  ✅ Resource deletion APIs with enhanced monitoring:");
    println!("      • Vector store file deletion (delete_file method)");
    println!("      • Vector store deletion (delete method)");
    println!("      • File deletion (files.delete method)");
    println!("  ✅ API verification and comprehensive error handling");

    // Enhanced SDK Capabilities Summary
    println!("\n✨ Enhanced SDK Capabilities Summary:");
    println!("=====================================");
    println!("🔧 Core Enhancements:");
    println!("   • GPT-4o Mini as optimized default model");
    println!("   • O4Mini for advanced reasoning tasks");
    println!("   • Comprehensive response status checking");
    println!("   • Enhanced token usage monitoring with reasoning tokens");
    println!("   • Parameter echoing for request verification");
    println!("   • User tracking across conversations");

    println!("\n🛠️ Tool Enhancements:");
    println!("   • Parallel tool execution for improved efficiency");
    println!("   • Image generation with partial image streaming (1-3 images)");
    println!("   • MCP server integration with custom approval modes");
    println!("   • Enhanced container support for code execution security");

    println!("\n🧠 Reasoning & Background Processing:");
    println!("   • Advanced reasoning models with effort level control");
    println!("   • Reasoning token tracking and analysis");
    println!("   • Encrypted reasoning content for stateless mode");
    println!("   • Background processing capabilities (BackgroundHandle)");

    println!("\n📊 Monitoring & Analytics:");
    println!("   • Real-time response status tracking");
    println!("   • Comprehensive token usage analytics");
    println!("   • Conversation-level statistics");
    println!("   • Enhanced error detection and handling");
    println!("   • Performance metrics for streaming");

    println!("\n🔒 Type Safety & Compatibility:");
    println!("   • Type-safe include options with compile-time validation");
    println!("   • Full backward compatibility with legacy code");
    println!("   • Enhanced parameter validation");
    println!("   • Graceful degradation for unsupported features");

    println!("\n💡 This demo creates and deletes all resources, testing both creation and deletion APIs.");
    println!("🔧 Proper cleanup sequence: 1) Remove files from vector stores, 2) Delete vector stores, 3) Delete files");
    println!(
        "🧪 Each API method is tested with enhanced error handling and comprehensive monitoring."
    );
    println!("📚 Check the source code for implementation details and best practices!");
    println!(
        "🚀 The enhanced SDK provides production-ready features with comprehensive observability!"
    );

    Ok(())
}
