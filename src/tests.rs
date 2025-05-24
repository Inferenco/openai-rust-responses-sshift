#[cfg(test)]
mod unit_tests {
    use crate::types::{Container, Include, StreamEvent};
    use crate::{Client, CreateError, Input, Model, Request, Tool};
    use std::collections::HashMap;

    #[test]
    fn test_client_creation() {
        // Test invalid API key
        assert!(matches!(Client::new(""), Err(CreateError::InvalidApiKey)));
        assert!(matches!(
            Client::new("invalid"),
            Err(CreateError::InvalidApiKey)
        ));

        // Test valid API key format (doesn't verify it works)
        assert!(Client::new("sk-test-key").is_ok());
    }

    #[test]
    fn test_request_builder() {
        let request = Request::builder()
            .model(Model::GPT4o)
            .input("Test input")
            .instructions("Test instructions")
            .temperature(0.7)
            .build();

        assert_eq!(request.model, Model::GPT4o);
        assert!(matches!(request.input, Input::Text(text) if text == "Test input"));
        assert_eq!(request.instructions, Some("Test instructions".to_string()));
        assert_eq!(request.temperature, Some(0.7));
    }

    // NEW PHASE 1 TESTS
    #[test]
    fn test_new_tool_creation() {
        // Test Image Generation Tool
        let image_tool = Tool::image_generation(Some(Container::default_type()));
        assert_eq!(image_tool.tool_type, "image_generation");
        assert!(image_tool.container.is_some());
        assert_eq!(image_tool.container.unwrap().container_type, "default");

        // Test MCP Tool
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token".to_string());

        let mcp_tool = Tool::mcp(
            "test-server",
            "https://test.example.com",
            Some(headers.clone()),
        );

        assert_eq!(mcp_tool.tool_type, "mcp");
        assert_eq!(mcp_tool.server_label, Some("test-server".to_string()));
        assert_eq!(
            mcp_tool.server_url,
            Some("https://test.example.com".to_string())
        );
        assert_eq!(mcp_tool.headers, Some(headers));

        // Test Enhanced Code Interpreter
        let code_tool = Tool::code_interpreter(Some(Container::default_type()));
        assert_eq!(code_tool.tool_type, "code_interpreter");
        assert!(code_tool.container.is_some());

        // Test Computer Use Preview
        let computer_tool = Tool::computer_use_preview();
        assert_eq!(computer_tool.tool_type, "computer_use_preview");
        assert!(computer_tool.container.is_none());
    }

    #[test]
    fn test_container_creation() {
        let container = Container::default_type();
        assert_eq!(container.container_type, "default");
    }

    #[test]
    fn test_include_enum() {
        // Test all include variants
        let file_search = Include::FileSearchResults;
        let reasoning_summary = Include::ReasoningSummary;
        let reasoning_encrypted = Include::ReasoningEncryptedContent;

        // Test string representation
        assert_eq!(file_search.as_str(), "file_search.results");
        assert_eq!(reasoning_summary.as_str(), "reasoning.summary");
        assert_eq!(reasoning_encrypted.as_str(), "reasoning.encrypted_content");

        // Test display formatting
        assert_eq!(format!("{file_search}"), "file_search.results");
        assert_eq!(format!("{reasoning_summary}"), "reasoning.summary");
        assert_eq!(
            format!("{reasoning_encrypted}"),
            "reasoning.encrypted_content"
        );
    }

    #[test]
    fn test_request_with_new_features() {
        let includes = vec![
            Include::FileSearchResults,
            Include::ReasoningSummary,
            Include::ReasoningEncryptedContent,
        ];

        let tools = vec![
            Tool::web_search_preview(),
            Tool::image_generation(Some(Container::default_type())),
            Tool::file_search(vec!["vector_store_123".to_string()]),
            Tool::mcp("test-server", "https://api.test.com", None),
        ];

        let request = Request::builder()
            .model(Model::GPT4o)
            .input("Test with new features")
            .include(includes.clone())
            .tools(tools.clone())
            .build();

        assert_eq!(request.include, Some(includes));
        assert_eq!(request.tools, Some(tools));
    }

    #[test]
    fn test_backward_compatibility_include_strings() {
        let request = Request::builder()
            .model(Model::GPT4o)
            .input("Test backward compatibility")
            .include_strings(vec![
                "file_search.results".to_string(),
                "reasoning.summary".to_string(),
                "reasoning.encrypted_content".to_string(),
                "unknown.option".to_string(), // Should be filtered out
            ])
            .build();

        let includes = request.include.unwrap();
        assert_eq!(includes.len(), 3); // unknown.option filtered out
        assert!(includes.contains(&Include::FileSearchResults));
        assert!(includes.contains(&Include::ReasoningSummary));
        assert!(includes.contains(&Include::ReasoningEncryptedContent));
    }

    #[test]
    fn test_stream_event_helpers() {
        // Test text delta helper
        let text_event = StreamEvent::TextDelta {
            content: "Hello world".to_string(),
            index: 0,
        };
        assert_eq!(text_event.as_text_delta(), Some("Hello world"));
        assert!(!text_event.is_done());

        // Test image progress helper
        let image_event = StreamEvent::ImageProgress {
            url: Some("https://example.com/image.jpg".to_string()),
            index: 0,
        };
        assert_eq!(
            image_event.as_image_progress(),
            Some("https://example.com/image.jpg")
        );
        assert!(!image_event.is_done());

        // Test image progress without URL
        let image_event_no_url = StreamEvent::ImageProgress {
            url: None,
            index: 0,
        };
        assert_eq!(image_event_no_url.as_image_progress(), None);

        // Test done event
        let done_event = StreamEvent::Done;
        assert!(done_event.is_done());
        assert_eq!(done_event.as_text_delta(), None);
        assert_eq!(done_event.as_image_progress(), None);

        // Test tool call delta helper
        let tool_event = StreamEvent::ToolCallDelta {
            id: "call_123".to_string(),
            content: "tool output".to_string(),
            index: 0,
        };
        assert_eq!(tool_event.as_tool_call_delta(), Some("tool output"));
    }

    #[test]
    fn test_serialization_deserialization() {
        // Test Container serialization
        let container = Container::default_type();
        let serialized = serde_json::to_string(&container).unwrap();
        let deserialized: Container = serde_json::from_str(&serialized).unwrap();
        assert_eq!(container.container_type, deserialized.container_type);

        // Test Include serialization
        let include = Include::ReasoningSummary;
        let serialized = serde_json::to_string(&include).unwrap();
        assert!(serialized.contains("reasoning.summary"));

        // Test Tool serialization with new fields
        let mcp_tool = Tool::mcp("test", "https://test.com", None);
        let serialized = serde_json::to_string(&mcp_tool).unwrap();
        assert!(serialized.contains("mcp"));
        assert!(serialized.contains("test"));
        assert!(serialized.contains("https://test.com"));
    }

    // Original tests maintained
    #[test]
    #[ignore] // Only run with --ignored flag
    fn test_create_response() {
        tokio_test::block_on(async {
            let Ok(client) = Client::from_env() else {
                return;
            }; // Skip test if no API key is available

            let request = Request::builder()
                .model(Model::GPT4o)
                .input("Hello, world!")
                .build();

            let response = client.responses.create(request).await;
            assert!(response.is_ok());

            let response = response.unwrap();
            assert!(!response.id().is_empty());
            assert!(!response.output_text().is_empty());
        });
    }

    #[test]
    #[ignore] // Only run with --ignored flag
    #[cfg(feature = "stream")]
    fn test_create_stream() {
        use futures::StreamExt;

        tokio_test::block_on(async {
            let Ok(client) = Client::from_env() else {
                return;
            }; // Skip test if no API key is available

            let request = Request::builder()
                .model(Model::GPT4o)
                .input("Count from 1 to 5")
                .build();

            let mut stream = std::pin::pin!(client.responses.stream(request));
            let mut events_received = 0;
            let mut full_content = String::new();

            println!("🌊 Starting streaming test...");
            print!("📖 Response: ");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();

            while let Some(event) = stream.next().await {
                match event {
                    Ok(stream_event) => {
                        events_received += 1;
                        match stream_event {
                            crate::types::StreamEvent::TextDelta { content, .. } => {
                                print!("{content}");
                                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                                full_content.push_str(&content);
                            }
                            crate::types::StreamEvent::Done => {
                                println!("\n✅ Stream completed!");
                                break;
                            }
                            _ => {
                                // Handle other event types
                            }
                        }
                    }
                    Err(e) => panic!("Stream error: {e:?}"),
                }

                // Safety limit to prevent infinite streams
                if events_received >= 50 {
                    println!("\n⚠️ Stopping after 50 events");
                    break;
                }
            }

            println!("\n📊 Test results:");
            println!("   Events received: {events_received}");
            println!(
                "   Content length: {length} characters",
                length = full_content.len()
            );

            assert!(events_received > 0);
            assert!(!full_content.is_empty());
        });
    }

    #[test]
    #[ignore] // Only run with --ignored flag
    #[cfg(feature = "stream")]
    fn test_enhanced_streaming_with_new_events() {
        use futures::StreamExt;

        tokio_test::block_on(async {
            let Ok(client) = Client::from_env() else {
                return;
            };

            // Test with tools that might generate new event types
            let request = Request::builder()
                .model(Model::GPT4o)
                .input("Describe a simple diagram and mention if an image would be helpful")
                .tools(vec![
                    Tool::web_search_preview(),
                    Tool::image_generation(Some(Container::default_type())),
                ])
                .include(vec![Include::FileSearchResults, Include::ReasoningSummary])
                .build();

            let mut stream = std::pin::pin!(client.responses.stream(request));
            let mut events_received = 0;
            let mut image_events = 0;
            let mut text_events = 0;

            println!("🌊 Starting enhanced streaming test with Phase 1 features...");

            while let Some(event) = stream.next().await {
                match event {
                    Ok(stream_event) => {
                        events_received += 1;
                        match &stream_event {
                            crate::types::StreamEvent::TextDelta { .. } => {
                                text_events += 1;
                            }
                            crate::types::StreamEvent::ImageProgress { .. } => {
                                image_events += 1;
                                println!("📸 Image progress event detected!");
                            }
                            crate::types::StreamEvent::Done => {
                                println!("✅ Enhanced stream completed!");
                                break;
                            }
                            _ => {}
                        }

                        // Test new helper methods
                        if let Some(text) = stream_event.as_text_delta() {
                            assert!(!text.is_empty());
                        }
                        if let Some(_img_url) = stream_event.as_image_progress() {
                            println!("📸 Image URL extracted via helper method");
                        }
                    }
                    Err(e) => panic!("Enhanced stream error: {e:?}"),
                }

                if events_received >= 50 {
                    break;
                }
            }

            println!("📊 Enhanced test results:");
            println!("   Total events: {events_received}");
            println!("   Text events: {text_events}");
            println!("   Image events: {image_events}");

            assert!(events_received > 0);
        });
    }
}
