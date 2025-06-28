use open_ai_rust_responses_by_sshift::{Client, Model, Request, Tool, ToolChoice};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let client = Client::from_env()?;

    // Define a calculator function tool
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

    // Define a weather tool for demonstration
    let weather_tool = Tool::function(
        "get_weather",
        "Get current weather information for a location",
        json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City name or location"
                }
            },
            "required": ["location"]
        }),
    );

    println!("🧮 Function Calling Example with OpenAI Responses API");
    println!("===================================================\n");

    // Step 1: Initial request with tools and enhanced features
    println!("1️⃣ Making initial request with function tools and enhanced features...");
    let request = Request::builder()
        .model(Model::GPT4oMini) // Updated to use GPT-4o Mini
        .input("Calculate the result of 15 * 7 + 23, explain the calculation, and tell me the weather in San Francisco")
        .instructions("Be thorough in your explanations and use the available tools")
        .tools(vec![calculator_tool.clone(), weather_tool.clone()])
        .tool_choice(ToolChoice::auto())
        .parallel_tool_calls(true) // Enable parallel tool execution
        .max_output_tokens(500) // Use preferred parameter
        .temperature(0.3) // Lower temperature for more consistent tool usage
        .user("function-calling-example") // Add user tracking
        .build();

    let response = client.responses.create(request).await?;

    println!("📊 Response Status: {}", response.status);
    println!("🤖 Model Used: {}", response.model);

    // Show response status checks
    if response.has_errors() {
        println!("❌ Response has errors!");
        if let Some(error) = &response.error {
            println!("   Error: {} - {}", error.code, error.message);
        }
        return Ok(());
    }

    println!("📝 Model Response:");
    println!("   Text: {}", response.output_text());

    // Display token usage
    if let Some(usage) = &response.usage {
        println!("\n📊 Token Usage:");
        println!(
            "   Input: {}, Output: {}, Total: {}",
            usage.input_tokens, usage.output_tokens, usage.total_tokens
        );
    }

    // Step 2: Check for tool calls
    let tool_calls = response.tool_calls();
    if tool_calls.is_empty() {
        println!("❌ No tool calls were made. Try a different prompt that requires calculation.");
        return Ok(());
    }

    println!(
        "\n2️⃣ Tool calls detected (parallel execution: {}):",
        response.parallel_tool_calls.unwrap_or(false)
    );
    let mut function_outputs = Vec::new();

    for tool_call in &tool_calls {
        println!("   🔧 Function: {}", tool_call.name);
        println!("   📋 Arguments: {}", tool_call.arguments);
        println!("   🆔 Call ID: {}", tool_call.call_id);

        // Step 3: Execute the function (simulate both calculator and weather)
        match tool_call.name.as_str() {
            "calculate" => {
                match serde_json::from_str::<HashMap<String, String>>(&tool_call.arguments) {
                    Ok(args) => {
                        if let Some(expression) = args.get("expression") {
                            let result = evaluate_expression(expression);
                            println!("   ✅ Calculated result: {result}");
                            function_outputs.push((tool_call.call_id.clone(), result));
                        } else {
                            let error_msg = "Error: Missing required 'expression' parameter";
                            println!("   ❌ Function error: {error_msg}");
                            function_outputs
                                .push((tool_call.call_id.clone(), error_msg.to_string()));
                        }
                    }
                    Err(json_err) => {
                        let error_msg = format!("Error: Invalid function arguments - {json_err}");
                        println!("   ❌ Argument parsing error: {error_msg}");
                        function_outputs.push((tool_call.call_id.clone(), error_msg));
                    }
                }
            }
            "get_weather" => {
                match serde_json::from_str::<HashMap<String, String>>(&tool_call.arguments) {
                    Ok(args) => {
                        if let Some(location) = args.get("location") {
                            // Simulate potential API failure
                            match get_mock_weather_with_error_handling(location) {
                                Ok(weather_result) => {
                                    println!("   🌤️ Weather result: {weather_result}");
                                    function_outputs
                                        .push((tool_call.call_id.clone(), weather_result));
                                }
                                Err(weather_err) => {
                                    let error_msg = format!("Weather API error: {weather_err}");
                                    println!("   ❌ Weather service error: {error_msg}");
                                    function_outputs.push((tool_call.call_id.clone(), error_msg));
                                }
                            }
                        } else {
                            let error_msg = "Error: Missing required 'location' parameter";
                            println!("   ❌ Function error: {error_msg}");
                            function_outputs
                                .push((tool_call.call_id.clone(), error_msg.to_string()));
                        }
                    }
                    Err(json_err) => {
                        let error_msg = format!("Error: Invalid function arguments - {json_err}");
                        println!("   ❌ Argument parsing error: {error_msg}");
                        function_outputs.push((tool_call.call_id.clone(), error_msg));
                    }
                }
            }
            _ => {
                let error_msg = format!("Error: Unknown function '{}'", tool_call.name);
                println!("   ⚠️ {error_msg}");
                function_outputs.push((tool_call.call_id.clone(), error_msg));
            }
        }
    }

    // Step 4: Continue conversation with function outputs using enhanced features
    println!("\n3️⃣ Submitting function outputs and continuing conversation...");

    let continuation_request = Request::builder()
        .model(Model::GPT4oMini)
        .with_function_outputs(response.id(), function_outputs)
        .tools(vec![calculator_tool, weather_tool]) // Keep tools available for potential follow-ups
        .instructions("Provide a comprehensive summary based on the tool results")
        .store(true) // Explicitly store conversation state
        .user("function-calling-example") // Maintain user tracking
        .build();

    // Enhanced error handling for continuation request
    let final_response = match client.responses.create(continuation_request).await {
        Ok(response) => {
            println!("   ✅ Successfully submitted function outputs");
            response
        }
        Err(e) => {
            println!("   ❌ Error submitting function outputs:");
            println!("      Error type: {:?}", std::mem::discriminant(&e));
            println!("      User message: {}", e.user_message());

            if e.is_recoverable() {
                println!("      🔄 This error is recoverable");
                if let Some(retry_after) = e.retry_after() {
                    println!("      ⏱️ Suggested retry delay: {}s", retry_after);
                }
            }

            if e.is_transient() {
                println!("      ⚡ This is a transient error - consider retrying");
            }

            return Err(e.into());
        }
    };

    // Enhanced response analysis
    println!("📊 Final Response Status: {}", final_response.status);
    println!("✅ Is Complete: {}", final_response.is_complete());

    if let Some(usage) = &final_response.usage {
        println!("📊 Final Token Usage: {} total tokens", usage.total_tokens);

        // Show cumulative token count if available
        let total_tokens =
            response.total_tokens().unwrap_or(0) + final_response.total_tokens().unwrap_or(0);
        println!("📊 Total Conversation Tokens: {total_tokens}");
    }

    println!("\n📝 Final Model Response:");
    println!("   {}", final_response.output_text());

    // Show parameter echoes
    if let Some(temp) = final_response.temperature {
        println!("\n⚙️ Temperature used: {temp}");
    }

    println!("\n✅ Enhanced function calling workflow completed successfully!");
    println!("\n🎸 Features Demonstrated:");
    println!("   • Parallel tool calls for improved efficiency");
    println!("   • Enhanced response status tracking");
    println!("   • Comprehensive token usage monitoring");
    println!("   • Parameter echoing and user tracking");
    println!("   • Improved error handling with detailed error info");
    println!("   • Robust function argument parsing and validation");
    println!("   • Graceful handling of function execution failures");
    println!("   • Proper error message propagation to the model");
    println!("\n📚 Key Points:");
    println!("   • OpenAI Responses API doesn't have submit_tool_outputs endpoint");
    println!("   • Tool outputs are submitted as input items with type 'function_call_output'");
    println!("   • Use previous_response_id to maintain conversation context");
    println!("   • Each function call has a unique call_id that must match the output");
    println!("   • Enhanced monitoring provides better insights into API usage");
    println!("   • Always provide function outputs, even for errors");
    println!("   • Use descriptive error messages to help the model understand failures");
    println!("   • Handle JSON parsing errors gracefully in function arguments");
    println!("   • Validate required parameters before function execution");

    Ok(())
}

/// Simple expression evaluator for demonstration
/// In a real application, you'd use a proper math library or external service
fn evaluate_expression(expression: &str) -> String {
    // Simple calculator for basic expressions
    // This is just for demonstration - use a proper math parser in production
    match expression {
        expr if expr.contains("15 * 7 + 23") || expr.contains("15*7+23") => {
            (15 * 7 + 23).to_string()
        }
        expr if expr.contains("*") && expr.contains("+") => {
            // Try to handle basic order of operations
            if let Some(result) = simple_calculate(expr) {
                result.to_string()
            } else {
                format!("Unable to calculate: {expr}")
            }
        }
        _ => format!("Calculation: {expression} = [result would be computed here]"),
    }
}

/// Enhanced mock weather function that demonstrates error handling
fn get_mock_weather_with_error_handling(location: &str) -> Result<String, String> {
    // Simulate various error conditions for demonstration
    match location.to_lowercase().as_str() {
        "error" => Err("Service temporarily unavailable".to_string()),
        "timeout" => Err("Request timed out".to_string()),
        "invalid" => Err("Invalid location format".to_string()),
        "unknown" => Err("Location not found".to_string()),
        _ => Ok(format!(
            "Weather in {location}: 72°F, partly cloudy with light winds. Perfect day for coding! 🌤️"
        )),
    }
}

/// Very basic calculator for demonstration
fn simple_calculate(expr: &str) -> Option<i32> {
    // Remove spaces
    let expr = expr.replace(" ", "");

    // This is a very basic implementation for demo purposes
    // In production, use a proper expression parser like `evalexpr` crate
    if expr == "15*7+23" {
        Some(15 * 7 + 23)
    } else if expr == "2+3*4" {
        Some(2 + 3 * 4)
    } else {
        None
    }
}
