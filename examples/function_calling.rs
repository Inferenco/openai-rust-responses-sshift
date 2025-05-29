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

    println!("🧮 Function Calling Example with OpenAI Responses API");
    println!("===================================================\n");

    // Step 1: Initial request with tools
    println!("1️⃣ Making initial request with function tools...");
    let request = Request::builder()
        .model(Model::GPT4o)
        .input("Calculate the result of 15 * 7 + 23 and explain the calculation")
        .tools(vec![calculator_tool.clone()])
        .tool_choice(ToolChoice::auto())
        .build();

    let response = client.responses.create(request).await?;

    println!("📝 Model Response:");
    println!("   Text: {}", response.output_text());

    // Step 2: Check for tool calls
    let tool_calls = response.tool_calls();
    if tool_calls.is_empty() {
        println!("❌ No tool calls were made. Try a different prompt that requires calculation.");
        return Ok(());
    }

    println!("\n2️⃣ Tool calls detected:");
    let mut function_outputs = Vec::new();

    for tool_call in &tool_calls {
        println!("   🔧 Function: {}", tool_call.name);
        println!("   📋 Arguments: {}", tool_call.arguments);
        println!("   🆔 Call ID: {}", tool_call.call_id);

        // Step 3: Execute the function (simulate calculator)
        if tool_call.name == "calculate" {
            let args: HashMap<String, String> = serde_json::from_str(&tool_call.arguments)?;
            if let Some(expression) = args.get("expression") {
                let result = evaluate_expression(expression);
                println!("   ✅ Calculated result: {}", result);

                // Collect the function output
                function_outputs.push((tool_call.call_id.clone(), result));
            }
        }
    }

    // Step 4: Continue conversation with function outputs
    // This is the correct way to submit tool outputs in the Responses API
    println!("\n3️⃣ Submitting function outputs and continuing conversation...");

    let continuation_request = Request::builder()
        .model(Model::GPT4o)
        .with_function_outputs(response.id(), function_outputs)
        .tools(vec![calculator_tool]) // Keep tools available for potential follow-ups
        .build();

    let final_response = client.responses.create(continuation_request).await?;

    println!("📝 Final Model Response:");
    println!("   {}", final_response.output_text());

    println!("\n✅ Function calling workflow completed successfully!");
    println!("\n📚 Key Points:");
    println!("   • OpenAI Responses API doesn't have submit_tool_outputs endpoint");
    println!("   • Tool outputs are submitted as input items with type 'function_call_output'");
    println!("   • Use previous_response_id to maintain conversation context");
    println!("   • Each function call has a unique call_id that must match the output");

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
                format!("Unable to calculate: {}", expr)
            }
        }
        _ => format!(
            "Calculation: {} = [result would be computed here]",
            expression
        ),
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
