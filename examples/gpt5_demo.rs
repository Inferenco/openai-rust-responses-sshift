//! GPT-5 demo: compare fast/balanced/quality modes, verbosity and reasoning effort,
//! plus a free-form SQL tool call. Run with:
//!   cargo run --example gpt5_demo
//! Requires: OPENAI_API_KEY

use open_ai_rust_responses_by_sshift::{
    Client, Model, ReasoningEffort, Request, Tool, ToolChoice, Verbosity,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;

    println!("\n=== GPT-5 Demo: Modes & Tools ===\n");

    // 1) Fast mode: GPT-5-Nano, minimal reasoning, low verbosity
    run_config(
        &client,
        "Fast (Nano)",
        Model::GPT5Nano,
        ReasoningEffort::Minimal,
        Verbosity::Low,
        "Summarize: Key benefits of Rust for backend services in 2 bullets.",
        None,
        None,
    )
    .await?;

    // 2) Balanced mode with a free-form function call: GPT-5-Mini
    // Use a structured function for reliable extraction of the SQL string
    let sql_tool = Tool::function(
        "generate_sql",
        "Generate an ANSI SQL query for the given analytics task and return it in the `sql` field.",
        json!({
            "type": "object",
            "properties": {
                "sql": {"type": "string", "description": "The SQL query only"}
            },
            "required": ["sql"],
            "additionalProperties": false
        }),
    );
    run_config(
        &client,
        "Balanced (Mini) + Tool",
        Model::GPT5Mini,
        ReasoningEffort::Medium,
        Verbosity::Medium,
        "Create a SQL query to find the top 5 products by revenue in the last 30 days.",
        Some(vec![sql_tool]),
        Some(ToolChoice::required()),
    )
    .await?;

    // 3) Quality mode: GPT-5, high reasoning, high verbosity
    run_config(
        &client,
        "Quality (Flagship)",
        Model::GPT5,
        ReasoningEffort::High,
        Verbosity::High,
        "Provide a step-by-step plan for migrating a monolith to microservices.",
        None,
        None,
    )
    .await?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn run_config(
    client: &Client,
    label: &str,
    model: Model,
    reasoning: ReasoningEffort,
    verbosity: Verbosity,
    prompt: &str,
    tools: Option<Vec<Tool>>,
    tool_choice: Option<ToolChoice>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("--- {} ---", label);

    let mut builder = Request::builder()
        .model(model)
        .input(prompt)
        .verbosity(verbosity)
        .reasoning_effort(reasoning);

    if let Some(ts) = tools {
        builder = builder.tools(ts);
    }
    if let Some(tc) = tool_choice {
        builder = builder.tool_choice(tc);
    }

    let response = client.responses.create(builder.build()).await?;

    println!("Model: {} | Status: {}", response.model, response.status);
    let text = response.output_text();
    if !text.is_empty() {
        println!("Output:\n{}\n", text);
    }

    let tool_calls = response.tool_calls();
    if !tool_calls.is_empty() {
        println!("Tool Calls ({}):", tool_calls.len());
        for call in tool_calls {
            if call.name == "generate_sql" {
                // Extract SQL from structured arguments
                match serde_json::from_str::<serde_json::Value>(&call.arguments) {
                    Ok(v) => {
                        let sql = v.get("sql").and_then(|s| s.as_str()).unwrap_or("");
                        if !sql.is_empty() {
                            println!("Generated SQL:\n{}\n", sql);
                        } else {
                            println!("generate_sql returned empty payload: {}", call.arguments);
                        }
                    }
                    Err(_) => println!("generate_sql arguments not JSON: {}", call.arguments),
                }
            } else {
                println!("- {} -> {}", call.name, call.arguments);
            }
        }
        println!();
    }

    if let Some(usage) = response.usage_with_tools() {
        println!(
            "Usage: input={} output={} total={}",
            usage.input_tokens, usage.output_tokens, usage.total_tokens
        );
    }

    println!();
    Ok(())
}
