use open_ai_rust_responses_by_sshift::types::Container;
use open_ai_rust_responses_by_sshift::{Client, Model, Request, ResponseItem, Tool};

/// # Code Interpreter Example
///
/// This example demonstrates how to use the built-in `code_interpreter` tool to execute Python code.
/// The model is asked to calculate the 47th digit of Pi.
///
/// The example will:
/// 1. Create a request with the `code_interpreter` tool enabled.
/// 2. Send the request to the OpenAI Responses API.
/// 3. Print the entire response structure to show how the tool output is returned.
/// 4. Iterate through the response items to find and display the code interpreter's output and the final text response.
///
/// # Usage
///
/// ```
/// export OPENAI_API_KEY=sk-your-api-key
/// cargo run --example code_interpreter
/// ```
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create a client from the environment variable
    let client = Client::from_env()?;
    println!("✓ Client created from environment.");

    // 2. Create a request with the code_interpreter tool
    println!("▶️  Creating request with code_interpreter tool...");
    let request = Request::builder()
        .model(Model::GPT4o) // Use a model that supports the code interpreter tool
        .input("Please calculate the 47th digit of pi using Python and tell me the result.")
        .tools(vec![Tool::code_interpreter(Some(Container::auto_type()))]) // Enable the code interpreter with auto container
        .build();
    println!("✓ Request created.");

    // 3. Send the request
    println!("\n▶️  Sending request to the API...");
    let response = client.responses.create(request).await?;
    println!("✓ Response received.");

    // 4. Print the raw response structure for inspection
    println!("\n🔍 Raw response output object:");
    println!("{:#?}", response.output);

    // 5. Process and display the response
    println!("\n💡 Processing response items...");
    if response.output.is_empty() {
        println!("   -> The response output is empty.");
    } else {
        for (i, item) in response.output.iter().enumerate() {
            println!("   -> Item {}:", i + 1);
            match item {
                ResponseItem::Message { content, role, .. } => {
                    println!("      - Type: Message (role: {role})");
                    for (j, message_content) in content.iter().enumerate() {
                        println!("      - Content {}: {message_content:#?}", j + 1);
                    }
                }
                ResponseItem::FunctionCall {
                    id,
                    arguments,
                    call_id,
                    name,
                    status,
                } => {
                    println!("      - Type: FunctionCall");
                    println!("        - ID: {id}");
                    println!("        - Call ID: {call_id}");
                    println!("        - Name: {name}");
                    println!("        - Status: {status}");
                    println!("        - Arguments: {arguments}");
                }
                // The API might use a generic "tool_call" for code interpreter
                #[allow(deprecated)]
                ResponseItem::ToolCall(tool_call) => {
                    println!("      - Type: ToolCall (Legacy)");
                    println!("        - ID: {id}", id=tool_call.id);
                    println!("        - Name: {name}", name=tool_call.name);
                    println!("        - Arguments: {arguments:#?}", arguments=tool_call.arguments);
                }
                ResponseItem::CodeInterpreterCall {
                    id,
                    container_id,
                    status,
                } => {
                    println!("      - Type: CodeInterpreterCall");
                    println!("        - ID: {id}");
                    println!("        - Container ID: {container_id}");
                    println!("        - Status: {status}");
                    println!(
                        "        - Note: Use container API to retrieve files and execution details"
                    );
                }
                _ => {
                    println!("      - Type: Other ({})", item_type_name(item));
                    println!("      - Content: {item:#?}");
                }
            }
        }
    }

    // 6. Display the final, user-facing text response
    println!("\n💬 Final Answer:");
    println!("{output}", output=response.output_text());

    Ok(())
}

fn item_type_name(item: &ResponseItem) -> &'static str {
    match item {
        ResponseItem::Message { .. } => "Message",
        ResponseItem::Reasoning { .. } => "Reasoning",
        ResponseItem::WebSearchCall { .. } => "WebSearchCall",
        ResponseItem::FileSearchCall { .. } => "FileSearchCall",
        ResponseItem::ImageGenerationCall { .. } => "ImageGenerationCall",
        ResponseItem::CodeInterpreterCall { .. } => "CodeInterpreterCall",
        ResponseItem::FunctionCall { .. } => "FunctionCall",
        ResponseItem::Text { .. } => "Text",
        #[allow(deprecated)]
        ResponseItem::ToolCall(_) => "ToolCall",
    }
}
