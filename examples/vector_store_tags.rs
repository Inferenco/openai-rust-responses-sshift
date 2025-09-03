//! Vector Store Tags Example
//!
//! This example demonstrates the new metadata filtering capabilities for vector stores:
//! - Adding files with attributes (tags, tenant_id, validity periods)
//! - Listing files with their attributes
//! - Using filtered file search with typed filter builders
//! - Managing file attributes with upsert operations
//!
//! Setup:
//! 1. Create a `.env` file with: OPENAI_API_KEY=sk-your-api-key-here
//! 2. Run with: `cargo run --example vector_store_tags`

use dotenv::dotenv;
use open_ai_rust_responses_by_sshift::{
    types::{Filter, FilterCondition, Model, Request, Tool, ToolChoice},
    vector_stores::{AddFileToVectorStoreRequest, CreateVectorStoreRequest},
    Client,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();

    println!("🏷️  Vector Store Tags & Metadata Filtering Demo");
    println!("===============================================\n");

    // Create client
    let client = Client::from_env()?;

    // Create a sample document for demonstration
    let sample_content = r#"
# Aptos Validator Setup Guide

## Overview
This guide covers setting up and running an Aptos validator node.

## Prerequisites
- Hardware requirements: 8+ CPU cores, 32GB+ RAM
- Network requirements: Stable internet connection
- Software: Docker, Git

## Installation Steps
1. Clone the Aptos repository
2. Configure your validator settings
3. Start the validator node
4. Monitor performance

## Security Considerations
- Keep your private keys secure
- Regularly update your node software
- Monitor for security patches

## Troubleshooting
Common issues and their solutions...
"#;

    std::fs::write("aptos_validator_guide.md", sample_content)?;

    // 1. UPLOAD FILE WITH ATTRIBUTES
    println!("1️⃣  Uploading file with metadata attributes");
    println!("------------------------------------------");

    let file = client
        .files
        .upload_file(
            "aptos_validator_guide.md",
            "assistants",
            Some("text/markdown".to_string()),
        )
        .await?;

    println!("✅ File uploaded: {} (ID: {})", file.filename, file.id);

    // 2. CREATE VECTOR STORE AND ADD FILE WITH ATTRIBUTES
    println!("\n2️⃣  Creating vector store and adding file with attributes");
    println!("--------------------------------------------------------");

    let vs_request = CreateVectorStoreRequest {
        name: "Aptos Documentation with Tags".to_string(),
        file_ids: vec![], // Start empty, add with attributes
    };

    let vector_store = client.vector_stores.create(vs_request).await?;
    println!("✅ Vector store created: {}", vector_store.name);

    // Get current timestamp for validity period
    let now_unix = chrono::Utc::now().timestamp();
    let valid_until = now_unix + (30 * 24 * 60 * 60); // Valid for 30 days

    // Add file with comprehensive attributes
    let add_file_request = AddFileToVectorStoreRequest {
        file_id: file.id.clone(),
        attributes: Some(json!({
            "tags": ["aptos", "validators", "blockchain", "setup-guide"],
            "tenant_id": "user_demo_123",
            "category": "documentation",
            "language": "english",
            "difficulty": "intermediate",
            "uploaded_at": now_unix,
            "valid_from": now_unix,
            "valid_to": valid_until,
            "author": "demo-user",
            "version": "1.0"
        })),
    };

    let _add_result = client
        .vector_stores
        .add_file(&vector_store.id, add_file_request)
        .await?;

    println!("✅ File added to vector store with comprehensive attributes:");
    println!("   Tags: aptos, validators, blockchain, setup-guide");
    println!("   Tenant: user_demo_123");
    println!("   Category: documentation");
    println!("   Valid until: {} days from now", 30);

    // Wait for indexing
    println!("\n⏳ Waiting for file indexing...");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // 3. LIST FILES WITH ATTRIBUTES (if supported)
    println!("\n3️⃣  Listing files with attributes");
    println!("--------------------------------");

    match client
        .vector_stores
        .list_files(&vector_store.id, None)
        .await
    {
        Ok(files_list) => {
            println!("✅ Found {} files in vector store:", files_list.len());
            for file in files_list.items() {
                println!("   📄 File: {} (ID: {})", file.filename, file.id);
                if let Some(attrs) = &file.attributes {
                    println!(
                        "      🏷️  Attributes: {}",
                        serde_json::to_string_pretty(attrs)?
                    );
                } else {
                    println!("      🏷️  No attributes");
                }
                if !file.extra.is_empty() {
                    println!("      📋 Extra fields: {:?}", file.extra);
                }
            }
        }
        Err(e) => {
            println!("⚠️  List files endpoint not yet available: {}", e);
            println!("   This is expected if the upstream API doesn't support it yet.");
        }
    }

    // 4. TYPED FILTER EXAMPLES
    println!("\n4️⃣  Creating typed filters for file search");
    println!("------------------------------------------");

    // Example 1: Search for Aptos-related documents
    let aptos_filter = Filter::and(vec![
        FilterCondition::contains_any("tags", vec![json!("aptos"), json!("blockchain")]),
        FilterCondition::eq("tenant_id", json!("user_demo_123")),
        FilterCondition::eq("category", json!("documentation")),
    ]);

    println!("✅ Created Aptos filter:");
    println!("   - Tags contain 'aptos' OR 'blockchain'");
    println!("   - Tenant ID equals 'user_demo_123'");
    println!("   - Category equals 'documentation'");

    // Example 2: Time-based filter (currently valid documents)
    let time_filter = Filter::and(vec![
        FilterCondition::lte("valid_from", json!(now_unix)),
        FilterCondition::gte("valid_to", json!(now_unix)),
        FilterCondition::eq("tenant_id", json!("user_demo_123")),
    ]);

    println!("\n✅ Created time-based filter:");
    println!("   - Valid from <= now");
    println!("   - Valid to >= now");
    println!("   - Tenant ID equals 'user_demo_123'");

    // Example 3: Complex OR filter
    let _complex_filter = Filter::or(vec![
        FilterCondition::contains_any("tags", vec![json!("validators"), json!("setup-guide")]),
        FilterCondition::eq("difficulty", json!("intermediate")),
    ]);

    println!("\n✅ Created complex OR filter:");
    println!("   - (Tags contain 'validators' OR 'setup-guide') OR");
    println!("   - (Difficulty equals 'intermediate')");

    // 5. FILTERED FILE SEARCH
    println!("\n5️⃣  Using filtered file search with Responses API");
    println!("------------------------------------------------");

    // Test with the Aptos filter
    let filtered_search_request = Request::builder()
        .model(Model::GPT4o)
        .input("What are the hardware requirements for running an Aptos validator?")
        .tools(vec![Tool::file_search_with_filters(
            vec![vector_store.id.clone()],
            serde_json::to_value(aptos_filter)?,
        )])
        .tool_choice(ToolChoice::auto())
        .build();

    println!("🔍 Searching with Aptos filter...");
    match client.responses.create(filtered_search_request).await {
        Ok(response) => {
            println!("✅ Filtered search completed!");
            println!(
                "   Response: {}",
                response.output_text().chars().take(200).collect::<String>() + "..."
            );

            // Show tool calls if any
            let tool_calls = response.tool_calls();
            if !tool_calls.is_empty() {
                println!("   🔧 Tool calls made: {}", tool_calls.len());
                for call in &tool_calls {
                    println!("      - {}: {}", call.name, call.call_id);
                }
            }
        }
        Err(e) => {
            println!("⚠️  Filtered search failed: {}", e);
            println!("   This may indicate the backend doesn't support filters yet.");
        }
    }

    // 6. BUILDER PATTERN EXAMPLE
    println!("\n6️⃣  Using builder pattern for filters");
    println!("------------------------------------");

    let builder_tool = Tool::file_search(vec![vector_store.id.clone()])
        .with_filters(serde_json::to_value(time_filter)?);

    println!("✅ Created tool using builder pattern:");
    println!("   - Started with basic file_search");
    println!("   - Added time-based filter with .with_filters()");

    let builder_request = Request::builder()
        .model(Model::GPT4oMini)
        .input("Summarize the security considerations for Aptos validators")
        .tools(vec![builder_tool])
        .build();

    println!("\n🔍 Testing builder pattern search...");
    match client.responses.create(builder_request).await {
        Ok(response) => {
            println!("✅ Builder pattern search completed!");
            println!(
                "   Response: {}",
                response.output_text().chars().take(200).collect::<String>() + "..."
            );
        }
        Err(e) => {
            println!("⚠️  Builder pattern search failed: {}", e);
        }
    }

    // 7. UPSERT ATTRIBUTES EXAMPLE
    println!("\n7️⃣  Updating file attributes with upsert");
    println!("---------------------------------------");

    let updated_attributes = json!({
        "tags": ["aptos", "validators", "blockchain", "setup-guide", "updated"],
        "tenant_id": "user_demo_123",
        "category": "documentation",
        "language": "english",
        "difficulty": "advanced", // Changed from intermediate
        "uploaded_at": now_unix,
        "valid_from": now_unix,
        "valid_to": valid_until,
        "author": "demo-user",
        "version": "1.1", // Incremented version
        "last_updated": chrono::Utc::now().timestamp(),
    });

    println!("🔄 Updating attributes (difficulty: intermediate → advanced, version: 1.0 → 1.1)...");
    match client
        .vector_stores
        .upsert_file_attributes(&vector_store.id, &file.id, updated_attributes)
        .await
    {
        Ok(_) => {
            println!("✅ Attributes updated successfully!");
            println!("   - Added 'updated' tag");
            println!("   - Changed difficulty to 'advanced'");
            println!("   - Incremented version to 1.1");
            println!("   - Added last_updated timestamp");
        }
        Err(e) => {
            println!("⚠️  Attribute update failed: {}", e);
        }
    }

    // 8. DEMONSTRATE VARIOUS FILTER CONDITIONS
    println!("\n8️⃣  Demonstrating all filter condition types");
    println!("--------------------------------------------");

    let comprehensive_examples = vec![
        ("Equality", FilterCondition::eq("status", json!("active"))),
        (
            "In Array",
            FilterCondition::in_array("type", vec![json!("guide"), json!("tutorial")]),
        ),
        (
            "Contains Any",
            FilterCondition::contains_any("tags", vec![json!("rust"), json!("api")]),
        ),
        (
            "Less Than or Equal",
            FilterCondition::lte("created_at", json!(now_unix)),
        ),
        (
            "Greater Than or Equal",
            FilterCondition::gte("priority", json!(5)),
        ),
        (
            "Less Than",
            FilterCondition::lt("file_size", json!(1000000)),
        ),
        ("Greater Than", FilterCondition::gt("rating", json!(4.0))),
        ("Not Equal", FilterCondition::ne("archived", json!(true))),
    ];

    for (name, condition) in comprehensive_examples {
        let single_filter = Filter::and(vec![condition]);
        let json_repr = serde_json::to_string(&single_filter)?;
        println!("   {} filter: {}", name, json_repr);
    }

    // 9. CLEANUP
    println!("\n9️⃣  Cleaning up resources");
    println!("-------------------------");

    // Remove file from vector store
    println!("🗑️  Removing file from vector store...");
    match client
        .vector_stores
        .delete_file(&vector_store.id, &file.id)
        .await
    {
        Ok(_) => println!("✅ File removed from vector store"),
        Err(e) => println!("⚠️  Failed to remove file: {}", e),
    }

    // Delete vector store
    println!("🗑️  Deleting vector store...");
    match client.vector_stores.delete(&vector_store.id).await {
        Ok(_) => println!("✅ Vector store deleted"),
        Err(e) => println!("⚠️  Failed to delete vector store: {}", e),
    }

    // Delete file
    println!("🗑️  Deleting file...");
    match client.files.delete(&file.id).await {
        Ok(_) => println!("✅ File deleted"),
        Err(e) => println!("⚠️  Failed to delete file: {}", e),
    }

    // Clean up local file
    std::fs::remove_file("aptos_validator_guide.md").ok();
    println!("✅ Local file cleaned up");

    // 10. SUMMARY
    println!("\n🎉 Vector Store Tags Demo Complete!");
    println!("===================================");
    println!("✅ Demonstrated features:");
    println!("   • File upload with comprehensive attributes");
    println!("   • Vector store creation and file addition with metadata");
    println!("   • Typed filter builders (Filter, FilterCondition)");
    println!("   • All filter condition types (eq, in, contains_any, lte, gte, lt, gt, ne)");
    println!("   • Filtered file search with Responses API");
    println!("   • Builder pattern for adding filters to tools");
    println!("   • Attribute upsert operations");
    println!("   • File listing with attributes (when supported)");
    println!("   • Proper resource cleanup");

    println!("\n💡 Key Benefits:");
    println!("   🏷️  Rich metadata tagging for better organization");
    println!("   🔍 Precise filtering for targeted search results");
    println!("   👥 Multi-tenant support with tenant_id filtering");
    println!("   ⏰ Time-based validity periods for content freshness");
    println!("   🔧 Type-safe filter construction prevents errors");
    println!("   📦 Flexible attribute storage with JSON values");

    println!("\n🚀 Ready for production use with comprehensive metadata filtering!");

    Ok(())
}
