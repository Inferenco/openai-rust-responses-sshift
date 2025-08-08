#![allow(deprecated)] // Tests intentionally use deprecated methods for compatibility testing

#[cfg(test)]
mod unit_tests {
    use crate::types::{Container, Include, StreamEvent};
    use crate::{Client, Input, Model, Request, Tool};
    use std::collections::HashMap;

    #[test]
    fn test_client_creation() {
        let client = Client::new("sk-test-key-1234567890abcdef");
        assert!(client.is_ok());
    }

    #[test]
    fn test_new_model_serialization() {
        use crate::types::Model;

        // Test latest generation models (2025)
        assert_eq!(Model::O3.to_string(), "o3");
        assert_eq!(Model::O4Mini.to_string(), "o4-mini");
        assert_eq!(Model::GPT41.to_string(), "gpt-4.1");
        assert_eq!(Model::GPT41Nano.to_string(), "gpt-4.1-nano");
        assert_eq!(Model::GPT41Mini.to_string(), "gpt-4.1-mini");

        // Test O-Series reasoning models
        assert_eq!(Model::O3Mini.to_string(), "o3-mini");
        assert_eq!(Model::O1.to_string(), "o1");
        assert_eq!(Model::O1Preview.to_string(), "o1-preview");
        assert_eq!(Model::O1Mini.to_string(), "o1-mini");

        // Test GPT-4o family
        assert_eq!(Model::GPT4o.to_string(), "gpt-4o");
        assert_eq!(Model::GPT4o20241120.to_string(), "gpt-4o-2024-11-20");
        assert_eq!(Model::GPT4o20240806.to_string(), "gpt-4o-2024-08-06");
        assert_eq!(Model::GPT4o20240513.to_string(), "gpt-4o-2024-05-13");
        assert_eq!(Model::GPT4oMini.to_string(), "gpt-4o-mini");

        // Test GPT-5 and GPT-4 families
        assert_eq!(Model::GPT5.to_string(), "gpt-5");
        assert_eq!(Model::GPT5Mini.to_string(), "gpt-5-mini");
        assert_eq!(Model::GPT5Nano.to_string(), "gpt-5-nano");
        assert_eq!(Model::GPT4Turbo.to_string(), "gpt-4-turbo");
        assert_eq!(
            Model::GPT4Turbo20240409.to_string(),
            "gpt-4-turbo-2024-04-09"
        );
        assert_eq!(Model::GPT4_32k.to_string(), "gpt-4-32k");

        // Test GPT-3.5 family
        assert_eq!(Model::GPT35Turbo0125.to_string(), "gpt-3.5-turbo-0125");
        assert_eq!(Model::GPT35Turbo1106.to_string(), "gpt-3.5-turbo-1106");
        assert_eq!(
            Model::GPT35TurboInstruct.to_string(),
            "gpt-3.5-turbo-instruct"
        );
    }

    #[test]
    fn test_new_model_from_string() {
        use crate::types::Model;

        // Test latest generation models (2025)
        assert_eq!(Model::from("o3"), Model::O3);
        assert_eq!(Model::from("o4-mini"), Model::O4Mini);
        assert_eq!(Model::from("gpt-4.1"), Model::GPT41);
        assert_eq!(Model::from("gpt-4.1-nano"), Model::GPT41Nano);
        assert_eq!(Model::from("gpt-4.1-mini"), Model::GPT41Mini);

        // Test O-Series reasoning models
        assert_eq!(Model::from("o3-mini"), Model::O3Mini);
        assert_eq!(Model::from("o1"), Model::O1);
        assert_eq!(Model::from("o1-preview"), Model::O1Preview);
        assert_eq!(Model::from("o1-mini"), Model::O1Mini);

        // Test GPT-5 and GPT-4o family
        assert_eq!(Model::from("gpt-5"), Model::GPT5);
        assert_eq!(Model::from("gpt-5-mini"), Model::GPT5Mini);
        assert_eq!(Model::from("gpt-5-nano"), Model::GPT5Nano);
        assert_eq!(Model::from("gpt-4o-2024-11-20"), Model::GPT4o20241120);
        assert_eq!(Model::from("gpt-4o-2024-08-06"), Model::GPT4o20240806);
        assert_eq!(Model::from("gpt-4o-2024-05-13"), Model::GPT4o20240513);
        assert_eq!(Model::from("gpt-4o-mini"), Model::GPT4oMini);

        // Test custom model fallback
        assert_eq!(
            Model::from("custom-model-123"),
            Model::Custom("custom-model-123".to_string())
        );
    }

    #[test]
    fn test_request_with_new_models() {
        use crate::types::{Model, Request};
        let request_gpt5 = Request::builder()
            .model(Model::GPT5)
            .input("Test with gpt-5 model")
            .build();
        assert_eq!(request_gpt5.model, Model::GPT5);

        // Test that we can create requests with the new models
        let request_o3 = Request::builder()
            .model(Model::O3)
            .input("Test with o3 model")
            .build();
        assert_eq!(request_o3.model, Model::O3);

        let request_o4_mini = Request::builder()
            .model(Model::O4Mini)
            .input("Test with o4-mini model")
            .build();
        assert_eq!(request_o4_mini.model, Model::O4Mini);

        let request_gpt41 = Request::builder()
            .model(Model::GPT41)
            .input("Test with GPT-4.1 model")
            .build();
        assert_eq!(request_gpt41.model, Model::GPT41);

        let request_o1 = Request::builder()
            .model(Model::O1)
            .input("Test with o1 reasoning model")
            .build();
        assert_eq!(request_o1.model, Model::O1);
    }

    #[test]
    fn test_gpt5_reasoning_effort_and_verbosity_builders() {
        let request = crate::Request::builder()
            .model(crate::Model::GPT5)
            .input("Short answer please")
            .verbosity(crate::types::Verbosity::Low)
            .reasoning_effort(crate::types::ReasoningEffort::Minimal)
            .build();
        assert_eq!(request.model, crate::Model::GPT5);
        assert_eq!(
            request
                .text
                .as_ref()
                .and_then(|t| t.verbosity.as_ref())
                .cloned(),
            Some(crate::types::Verbosity::Low)
        );
        assert_eq!(
            request
                .reasoning
                .as_ref()
                .and_then(|r| r.reasoning_effort.as_ref())
                .cloned(),
            Some(crate::types::ReasoningEffort::Minimal)
        );
    }

    #[test]
    fn test_request_builder() {
        let request = Request::builder()
            .model(Model::GPT4o)
            .input("Test input")
            .temperature(0.7)
            .build();

        assert_eq!(request.model, Model::GPT4o);
        assert!(matches!(request.input, Input::Text(ref text) if text == "Test input"));
        assert_eq!(request.temperature, Some(0.7));
    }

    // NEW PHASE 1 TESTS
    #[test]
    fn test_new_tool_creation() {
        // Test Image Generation Tool
        let image_tool = Tool::image_generation();
        assert_eq!(image_tool.tool_type, "image_generation");
        assert!(image_tool.container.is_none());

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
        let reasoning_encrypted = Include::ReasoningEncryptedContent;

        // Test string representation (updated to correct API values)
        assert_eq!(file_search.as_str(), "file_search_call.results");
        assert_eq!(reasoning_encrypted.as_str(), "reasoning.encrypted_content");

        // Test display formatting
        assert_eq!(format!("{file_search}"), "file_search_call.results");
        assert_eq!(
            format!("{reasoning_encrypted}"),
            "reasoning.encrypted_content"
        );
    }

    #[test]
    fn test_request_with_new_features() {
        let includes = vec![
            Include::FileSearchResults,
            Include::ReasoningEncryptedContent,
        ];

        let tools = vec![
            Tool::web_search_preview(),
            Tool::image_generation(),
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
                "reasoning.encrypted_content".to_string(),
                "unknown.option".to_string(), // Should be filtered out
            ])
            .build();

        let includes = request.include.unwrap();
        assert_eq!(includes.len(), 2); // unknown.option filtered out
        assert!(includes.contains(&Include::FileSearchResults));
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
        let include = Include::ReasoningEncryptedContent;
        let serialized = serde_json::to_string(&include).unwrap();
        assert!(serialized.contains("reasoning.encrypted_content"));

        // Test Tool serialization with new fields
        let mcp_tool = Tool::mcp("test", "https://test.com", None);
        let serialized = serde_json::to_string(&mcp_tool).unwrap();
        assert!(serialized.contains("mcp"));
        assert!(serialized.contains("test"));
        assert!(serialized.contains("https://test.com"));
    }

    // NEW PHASE 2 TESTS
    #[test]
    fn test_reasoning_params_in_request() {
        use crate::types::{Effort, ReasoningParams, SummarySetting};

        let reasoning = ReasoningParams::new()
            .with_effort(Effort::High)
            .with_summary(SummarySetting::Auto);

        let request = Request::builder()
            .model(Model::O1)
            .input("Complex reasoning task")
            .reasoning(reasoning.clone())
            .build();

        assert_eq!(request.reasoning, Some(reasoning));
        assert_eq!(request.model, Model::O1);
    }

    #[test]
    fn test_background_mode_request() {
        let request = Request::builder()
            .model(Model::O1)
            .input("Long-running analysis task")
            .background(true)
            .build();

        assert_eq!(request.background, Some(true));
    }

    #[test]
    fn test_reasoning_with_background_request() {
        use crate::types::ReasoningParams;

        let reasoning = ReasoningParams::high_effort_with_summary();

        let request = Request::builder()
            .model(Model::O1)
            .input("Complex task requiring high effort reasoning")
            .reasoning(reasoning.clone())
            .background(true)
            .include(vec![Include::ReasoningEncryptedContent])
            .build();

        assert_eq!(request.reasoning, Some(reasoning));
        assert_eq!(request.background, Some(true));
        assert!(request.include.is_some());
        let includes = request.include.unwrap();
        assert!(includes.contains(&Include::ReasoningEncryptedContent));
    }

    #[test]
    fn test_reasoning_params_builders() {
        use crate::types::{Effort, ReasoningParams, SummarySetting};

        // Test high effort builder
        let high_effort = ReasoningParams::high_effort();
        assert_eq!(high_effort.effort, Some(Effort::High));
        assert_eq!(high_effort.summary, None);

        // Test auto summary builder
        let auto_summary = ReasoningParams::auto_summary();
        assert_eq!(auto_summary.effort, None);
        assert_eq!(auto_summary.summary, Some(SummarySetting::Auto));

        // Test combined builder
        let combined = ReasoningParams::high_effort_with_summary();
        assert_eq!(combined.effort, Some(Effort::High));
        assert_eq!(combined.summary, Some(SummarySetting::Auto));
    }

    #[test]
    fn test_reasoning_model_compatibility() {
        use crate::types::ReasoningParams;

        // Test that reasoning models work with reasoning params
        let reasoning_models = vec![
            Model::O1,
            Model::O1Preview,
            Model::O1Mini,
            Model::O3,
            Model::O3Mini,
            Model::O4Mini,
        ];

        for model in reasoning_models {
            let request = Request::builder()
                .model(model.clone())
                .input("Test reasoning task")
                .reasoning(ReasoningParams::high_effort())
                .build();

            assert_eq!(request.model, model);
            assert!(request.reasoning.is_some());
        }
    }

    #[test]
    fn test_background_handle_functionality() {
        use crate::types::BackgroundHandle;

        let handle = BackgroundHandle::new(
            "bg_test_123".to_string(),
            "https://api.openai.com/v1/backgrounds/bg_test_123/status".to_string(),
        )
        .with_stream_url("https://api.openai.com/v1/backgrounds/bg_test_123/stream".to_string())
        .with_estimated_completion("2025-01-15T10:30:00Z".to_string());

        assert_eq!(handle.id, "bg_test_123");
        assert!(handle.stream_url.is_some());
        assert!(handle.estimated_completion.is_some());
        assert!(handle.is_running());
        assert!(!handle.is_done());
    }

    // Original tests maintained
    #[test]
    #[ignore = "requires OPENAI_API_KEY environment variable"]
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
    #[ignore = "requires OPENAI_API_KEY environment variable"]
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
    #[ignore = "requires OPENAI_API_KEY environment variable"]
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
                .tools(vec![Tool::web_search_preview(), Tool::image_generation()])
                .include(vec![
                    Include::FileSearchResults,
                    Include::ReasoningEncryptedContent,
                ])
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

    #[test]
    fn test_function_call_output_format() {
        // Test with a simple tool call
        let tool_call = crate::types::ResponseItem::FunctionCall {
            id: "call_123".to_string(),
            arguments: "{\"key\":\"value\"}".to_string(),
            call_id: "call_abc_123".to_string(),
            name: "test_function".to_string(),
            status: "completed".to_string(),
        };

        // Test image generation call
        let image_call = crate::types::ResponseItem::ImageGenerationCall {
            id: "img_call_123".to_string(),
            result: "base64-data-goes-here".to_string(),
            status: "completed".to_string(),
        };

        // Test reasoning item
        let reasoning_item = crate::types::ResponseItem::Reasoning {
            id: "reasoning_123".to_string(),
            summary: vec![],
            status: Some("completed".to_string()),
        };

        let response = crate::Response {
            id: "test_resp".to_string(),
            object: "response".to_string(),
            created_at: chrono::Utc::now(),
            model: "gpt-4o".to_string(),
            status: "completed".to_string(),
            output: vec![tool_call, image_call, reasoning_item],
            output_text: None,
            previous_response_id: None,
            instructions: None,
            metadata: None,
            usage: None,
            temperature: None,
            top_p: None,
            max_output_tokens: None,
            parallel_tool_calls: None,
            tool_choice: None,
            tools: None,
            text: None,
            top_logprobs: None,
            truncation: None,
            reasoning: None,
            reasoning_effort: None,
            user: None,
            incomplete_details: None,
            error: None,
        };

        // Test tool calls
        let tool_calls = response.tool_calls();
        assert_eq!(tool_calls.len(), 1);
        assert_eq!(tool_calls[0].name, "test_function");
    }

    // Phase 1 tests - New Response fields

    #[test]
    fn test_response_with_all_new_fields() {
        let response_json = r#"{
            "id": "resp_test123",
            "object": "response",
            "created_at": 1234567890,
            "model": "gpt-4o",
            "status": "completed",
            "output": [],
            "output_text": "Hello, world!",
            "previous_response_id": "resp_previous",
            "instructions": "Be helpful",
            "metadata": {"key": "value"},
            "usage": {
                "input_tokens": 100,
                "output_tokens": 50,
                "total_tokens": 150,
                "output_tokens_details": {
                    "reasoning_tokens": 25
                }
            },
            "temperature": 0.7,
            "top_p": 0.9,
            "max_output_tokens": 1000,
            "parallel_tool_calls": true,
            "tool_choice": "auto",
            "tools": [],
            "text": {
                "format": {"type": "text"},
                "stop": ["END"]
            },
            "top_logprobs": 5,
            "truncation": {
                "type": "auto",
                "last_messages": 10
            },
            "reasoning": {
                "content": [{"type": "thinking", "text": "Let me think..."}],
                "encrypted_content": "encrypted_data_here"
            },
            "user": "user123",
            "incomplete_details": {
                "reason": "max_tokens"
            },
            "error": null
        }"#;

        let response: crate::Response = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.id, "resp_test123");
        assert_eq!(response.object, "response");
        assert_eq!(response.status, "completed");
        assert_eq!(response.output_text, Some("Hello, world!".to_string()));
        assert_eq!(response.usage.as_ref().unwrap().total_tokens, 150);
        assert_eq!(response.temperature, Some(0.7));
        assert!(response.reasoning.is_some());
        assert!(response.is_complete());
        assert!(!response.is_in_progress());
        assert!(!response.has_errors());
        assert_eq!(response.total_tokens(), Some(150));
    }

    #[test]
    fn test_response_helper_methods() {
        let mut response = crate::Response {
            id: "test".to_string(),
            object: "response".to_string(),
            created_at: chrono::Utc::now(),
            model: "gpt-4o".to_string(),
            status: "in_progress".to_string(),
            output: vec![],
            output_text: None,
            previous_response_id: None,
            instructions: None,
            metadata: None,
            usage: Some(crate::types::Usage {
                input_tokens: 10,
                output_tokens: 20,
                total_tokens: 30,
                output_tokens_details: None,
                prompt_tokens_details: None,
                web_search: None,
                file_search: None,
                image_generation: None,
                code_interpreter: None,
            }),
            temperature: None,
            top_p: None,
            max_output_tokens: None,
            parallel_tool_calls: None,
            tool_choice: None,
            tools: None,
            text: None,
            top_logprobs: None,
            truncation: None,
            reasoning: None,
            reasoning_effort: None,
            user: None,
            incomplete_details: None,
            error: None,
        };

        assert!(!response.is_complete());
        assert!(response.is_in_progress());
        assert!(!response.has_errors());
        assert_eq!(response.total_tokens(), Some(30));

        // Test failed status
        response.status = "failed".to_string();
        assert!(response.is_complete());
        assert!(!response.is_in_progress());
        assert!(response.has_errors());

        // Test with error
        response.status = "completed".to_string();
        response.error = Some(crate::types::ResponseError {
            code: "500".to_string(),
            message: "Internal error".to_string(),
            metadata: None,
        });
        assert!(response.has_errors());
    }

    #[test]
    fn test_request_with_all_new_fields() {
        use crate::types::{Effort, ReasoningParams, SummarySetting};

        let request = crate::Request::builder()
            .model(crate::Model::GPT4o)
            .input("Hello")
            .instructions("Be helpful")
            .max_tokens(100)
            .max_output_tokens(150)
            .temperature(0.7)
            .top_p(0.9)
            .top_logprobs(5)
            .stream(true)
            .parallel_tool_calls(true)
            .reasoning(
                ReasoningParams::new()
                    .with_effort(Effort::High)
                    .with_summary(SummarySetting::Auto),
            )
            .background(true)
            .store(false)
            .user("user123")
            .build();

        assert_eq!(request.max_tokens, Some(100));
        assert_eq!(request.max_output_tokens, Some(150));
        assert_eq!(request.top_logprobs, Some(5));
        assert_eq!(request.parallel_tool_calls, Some(true));
        assert!(request.reasoning.is_some());
        assert_eq!(request.background, Some(true));
        assert_eq!(request.store, Some(false));
        assert_eq!(request.user, Some("user123".to_string()));

        // Test serialization
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("max_output_tokens"));
        assert!(json.contains("top_logprobs"));
        assert!(json.contains("parallel_tool_calls"));
        assert!(json.contains("reasoning"));
        assert!(json.contains("store"));
    }

    #[test]
    fn test_image_generation_with_partial_images() {
        // This test is now obsolete as partial images are not supported by the new built-in tool.
        // It can be removed or adapted if a similar feature is added in the future.
    }

    #[test]
    fn test_tool_usage_tracking() {
        use crate::types::{MessageContent, Response, ResponseItem, Usage};
        use chrono::Utc;

        // Create a response with mixed tool calls and token usage
        let response = Response {
            id: "resp_test123".to_string(),
            object: "response".to_string(),
            created_at: Utc::now(),
            model: "gpt-4o".to_string(),
            status: "completed".to_string(),
            output: vec![
                ResponseItem::Message {
                    id: "msg_1".to_string(),
                    content: vec![MessageContent::OutputText {
                        text: "I'll search for that information.".to_string(),
                        annotations: vec![],
                        logprobs: None,
                    }],
                    role: "assistant".to_string(),
                    status: Some("completed".to_string()),
                },
                ResponseItem::WebSearchCall {
                    id: "ws_1".to_string(),
                    status: "completed".to_string(),
                },
                ResponseItem::ImageGenerationCall {
                    id: "img_1".to_string(),
                    result: "base64_image_data".to_string(),
                    status: "completed".to_string(),
                },
                ResponseItem::ImageGenerationCall {
                    id: "img_2".to_string(),
                    result: "base64_image_data_2".to_string(),
                    status: "completed".to_string(),
                },
                ResponseItem::Message {
                    id: "msg_2".to_string(),
                    content: vec![MessageContent::OutputText {
                        text: "Here's what I found and the images I generated.".to_string(),
                        annotations: vec![],
                        logprobs: None,
                    }],
                    role: "assistant".to_string(),
                    status: Some("completed".to_string()),
                },
            ],
            output_text: None,
            previous_response_id: None,
            instructions: None,
            metadata: None,
            usage: Some(Usage {
                input_tokens: 150,
                output_tokens: 75,
                total_tokens: 225,
                output_tokens_details: None,
                prompt_tokens_details: None,
                web_search: None,
                file_search: None,
                image_generation: None,
                code_interpreter: None,
            }),
            temperature: None,
            top_p: None,
            max_output_tokens: None,
            parallel_tool_calls: None,
            tool_choice: None,
            tools: None,
            text: None,
            top_logprobs: None,
            truncation: None,
            reasoning: None,
            reasoning_effort: None,
            user: None,
            incomplete_details: None,
            error: None,
        };

        // Test tool usage calculation
        let (web_search, file_search, image_generation, code_interpreter) =
            response.calculate_tool_usage();
        assert_eq!(web_search, 1);
        assert_eq!(file_search, 0);
        assert_eq!(image_generation, 2);
        assert_eq!(code_interpreter, 0);

        // Test usage with tools
        let usage_with_tools = response.usage_with_tools().unwrap();
        assert_eq!(usage_with_tools.input_tokens, 150);
        assert_eq!(usage_with_tools.output_tokens, 75);
        assert_eq!(usage_with_tools.total_tokens, 225);
        assert_eq!(usage_with_tools.web_search, Some(1));
        assert_eq!(usage_with_tools.file_search, None);
        assert_eq!(usage_with_tools.image_generation, Some(2));
        assert_eq!(usage_with_tools.code_interpreter, None);

        // Test formatted output
        let formatted = response.format_usage();
        let expected = "input tokens: 150\noutput tokens: 75\ntotal tokens: 225\nweb search: 1\nfile search: 0\nimage generation: 2\ncode interpreter: 0";
        assert_eq!(formatted, expected);

        println!("Tool usage tracking test passed!");
        println!("Formatted usage:\n{formatted}");
    }

    #[test]
    fn test_mcp_tool_with_approval() {
        let tool =
            crate::Tool::mcp_with_approval("github", "https://api.github.com", "never", None);

        assert_eq!(tool.tool_type, "mcp");
        assert_eq!(tool.server_label, Some("github".to_string()));
        assert_eq!(tool.server_url, Some("https://api.github.com".to_string()));
        assert_eq!(tool.require_approval, Some("never".to_string()));

        // Test default MCP tool
        let default_tool = crate::Tool::mcp("github", "https://api.github.com", None);
        assert_eq!(default_tool.require_approval, Some("auto".to_string()));
    }

    #[test]
    fn test_truncation_config() {
        let config = crate::types::TruncationConfig {
            truncation_type: "auto".to_string(),
            last_messages: Some(10),
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("auto"));
        assert!(json.contains("last_messages"));

        // Test deserialization
        let deserialized: crate::types::TruncationConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.truncation_type, "auto");
        assert_eq!(deserialized.last_messages, Some(10));
    }

    #[test]
    fn test_text_config() {
        let config = crate::types::TextConfig {
            format: Some(crate::types::TextFormat {
                format_type: "text".to_string(),
            }),
            stop: Some(vec!["END".to_string(), "STOP".to_string()]),
            verbosity: Some(crate::types::Verbosity::Medium),
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("text"));
        assert!(json.contains("END"));
        assert!(json.contains("STOP"));
        assert!(json.contains("\"medium\""));
    }

    #[test]
    fn test_usage_with_details() {
        let usage = crate::types::Usage {
            input_tokens: 100,
            output_tokens: 50,
            total_tokens: 150,
            output_tokens_details: Some(crate::types::OutputTokensDetails {
                reasoning_tokens: Some(25),
            }),
            prompt_tokens_details: Some(crate::types::PromptTokensDetails {
                cached_tokens: Some(30),
            }),
            web_search: None,
            file_search: None,
            image_generation: None,
            code_interpreter: None,
        };

        let json = serde_json::to_string(&usage).unwrap();
        assert!(json.contains("reasoning_tokens"));
        assert!(json.contains("cached_tokens"));
        assert!(json.contains("150"));

        // Test deserialization
        let deserialized: crate::types::Usage = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.total_tokens, 150);
        assert_eq!(
            deserialized
                .output_tokens_details
                .as_ref()
                .unwrap()
                .reasoning_tokens,
            Some(25)
        );
    }

    #[test]
    fn test_reasoning_output() {
        let reasoning = crate::types::ReasoningOutput {
            content: Some(vec![crate::types::ReasoningContent {
                content_type: "thinking".to_string(),
                text: Some("Let me think about this...".to_string()),
            }]),
            encrypted_content: Some("encrypted_data".to_string()),
        };

        let json = serde_json::to_string(&reasoning).unwrap();
        assert!(json.contains("thinking"));
        assert!(json.contains("encrypted_data"));
        assert!(json.contains("Let me think"));
    }

    #[test]
    fn test_backward_compatibility() {
        // Test that old Response format still deserializes
        let old_response_json = r#"{
            "id": "resp_old",
            "model": "gpt-4o",
            "output": [],
            "previous_response_id": null,
            "created_at": 1234567890,
            "metadata": null
        }"#;

        let response: crate::Response = serde_json::from_str(old_response_json).unwrap();
        assert_eq!(response.id, "resp_old");
        assert_eq!(response.object, "response"); // Default value
        assert_eq!(response.status, "completed"); // Default value
        assert!(response.usage.is_none());
        assert!(response.temperature.is_none());
    }

    #[test]
    fn test_response_output_text_priority() {
        let response = crate::Response {
            id: "test".to_string(),
            object: "response".to_string(),
            created_at: chrono::Utc::now(),
            model: "gpt-4o".to_string(),
            status: "completed".to_string(),
            output: vec![],
            output_text: Some("Direct output text".to_string()),
            previous_response_id: None,
            instructions: None,
            metadata: None,
            usage: None,
            temperature: None,
            top_p: None,
            max_output_tokens: None,
            parallel_tool_calls: None,
            tool_choice: None,
            tools: None,
            text: None,
            top_logprobs: None,
            truncation: None,
            reasoning: None,
            reasoning_effort: None,
            user: None,
            incomplete_details: None,
            error: None,
        };

        // Should prioritize output_text field over extracting from output items
        assert_eq!(response.output_text(), "Direct output text");
    }

    // ===== Image Generation Tests =====

    #[test]
    fn test_image_generate_request_builder() {
        use crate::images::ImageGenerateRequest;

        let request = ImageGenerateRequest::new("test prompt")
            .with_size("1024x1024")
            .with_quality("high")
            .with_format("png")
            .with_compression(80)
            .with_seed(12345);

        assert_eq!(request.model, "gpt-image-1");
        assert_eq!(request.prompt, "test prompt");
        assert_eq!(request.size, Some("1024x1024".to_string()));
        assert_eq!(request.quality, Some("high".to_string()));
        assert_eq!(request.output_format, Some("png".to_string()));
        assert_eq!(request.output_compression, Some(80));
        assert_eq!(request.seed, Some(12345));
    }

    #[test]
    fn test_builtin_image_generation_tool() {
        let tool = crate::Tool::image_generation();
        assert_eq!(tool.tool_type, "image_generation");
        assert!(tool.name.is_none());
        assert!(tool.description.is_none());
        assert!(tool.parameters.is_none());
    }

    #[test]
    fn test_gpt_image1_model_serialization() {
        let model = crate::Model::GPTImage1;
        let serialized = serde_json::to_string(&model).unwrap();
        assert_eq!(serialized, "\"gpt-image-1\"");

        let deserialized: crate::Model = serde_json::from_str("\"gpt-image-1\"").unwrap();
        assert_eq!(deserialized, crate::Model::GPTImage1);
    }

    #[test]
    fn test_image_request_serialization() {
        use crate::images::ImageGenerateRequest;

        let request = ImageGenerateRequest::new("A red circle")
            .with_size("1024x1024")
            .with_quality("auto");

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("gpt-image-1"));
        assert!(json.contains("A red circle"));
        assert!(json.contains("1024x1024"));
        assert!(json.contains("auto"));
    }

    #[test]
    fn test_input_item_image_url_builder() {
        use crate::types::InputItem;
        let url = "https://example.com/sample.png";
        let content_item = InputItem::content_image(url);
        assert_eq!(content_item["type"], "input_image");
        assert_eq!(content_item["image_url"], url);
    }

    #[test]
    fn test_request_builder_input_image_urls() {
        let urls = vec!["https://example.com/one.png", "https://example.com/two.png"];
        let request = crate::Request::builder()
            .model(crate::Model::GPT4o)
            .input_image_urls(urls.clone())
            .build();

        match request.input {
            crate::Input::Items(items) => {
                assert_eq!(items.len(), 1);
                assert_eq!(items[0].item_type, "message");
                assert_eq!(items[0].role.as_ref().unwrap(), "user");

                let content = items[0].content.as_ref().unwrap().as_array().unwrap();
                assert_eq!(content.len(), urls.len());
                for (i, url) in urls.iter().enumerate() {
                    assert_eq!(content[i]["type"], "input_image");
                    assert_eq!(content[i]["image_url"], *url);
                }
            }
            crate::Input::Text(_) => panic!("Expected input items with message"),
        }
    }

    #[test]
    fn test_request_builder_input_image_url_single() {
        let url = "https://example.com/sample.png";
        let request = crate::Request::builder()
            .model(crate::Model::GPT4o)
            .input_image_url(url)
            .build();

        match request.input {
            crate::Input::Items(items) => {
                assert_eq!(items.len(), 1);
                assert_eq!(items[0].item_type, "message");
                assert_eq!(items[0].role.as_ref().unwrap(), "user");

                let content = items[0].content.as_ref().unwrap().as_array().unwrap();
                assert_eq!(content.len(), 1);
                assert_eq!(content[0]["type"], "input_image");
                assert_eq!(content[0]["image_url"], url);
            }
            crate::Input::Text(_) => panic!("Expected input items with message"),
        }
    }

    #[test]
    fn test_request_builder_push_image_url() {
        let urls = vec![
            "https://example.com/first.png",
            "https://example.com/second.png",
            "https://example.com/third.png",
        ];
        let mut builder = crate::Request::builder().model(crate::Model::GPT4o);
        for url in &urls {
            builder = builder.push_image_url(*url);
        }
        let request = builder.build();

        match request.input {
            crate::Input::Items(items) => {
                assert_eq!(items.len(), 1);
                assert_eq!(items[0].item_type, "message");

                let content = items[0].content.as_ref().unwrap().as_array().unwrap();
                assert_eq!(content.len(), urls.len());
                for (i, url) in urls.iter().enumerate() {
                    assert_eq!(content[i]["type"], "input_image");
                    assert_eq!(content[i]["image_url"], *url);
                }
            }
            crate::Input::Text(_) => panic!("Expected input items with message"),
        }
    }

    #[test]
    fn test_enhanced_image_generation_tools() {
        use crate::types::Tool;

        // Test basic image generation tool
        let basic_tool = Tool::image_generation();
        assert_eq!(basic_tool.tool_type, "image_generation");
        assert!(basic_tool.container.is_none());
        assert!(basic_tool.partial_images.is_none());

        // Note: Advanced image generation methods were removed in v0.2.4
        // as they are not yet supported by the OpenAI API.
        // The basic Tool::image_generation() method works perfectly
        // for image-guided generation workflows.
    }

    #[test]
    fn test_image_input_base64() {
        use crate::types::InputItem;

        let base64_data = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChAI9jU77ygAAAABJRU5ErkJggg==";
        let mime_type = "image/png";

        // Test basic base64 input
        let item = InputItem::image_base64(base64_data, mime_type);
        assert_eq!(item.item_type, "input_image");
        assert!(item.image_url.is_some());
        assert!(item
            .image_url
            .as_ref()
            .unwrap()
            .starts_with("data:image/png;base64,"));
        assert_eq!(item.detail, Some("auto".to_string()));

        // Test base64 input with detail
        let item_with_detail = InputItem::image_base64_with_detail(base64_data, mime_type, "high");
        assert_eq!(item_with_detail.detail, Some("high".to_string()));

        // Test content helpers
        let content = InputItem::content_image_base64(base64_data, mime_type);
        assert_eq!(content["type"], "input_image");
        assert!(content["image_url"]
            .as_str()
            .unwrap()
            .starts_with("data:image/png;base64,"));

        let content_with_detail =
            InputItem::content_image_base64_with_detail(base64_data, mime_type, "low");
        assert_eq!(content_with_detail["detail"], "low");
    }

    #[test]
    fn test_image_input_file_id() {
        use crate::types::InputItem;

        let file_id = "file-abc123";

        // Test basic file ID input
        let item = InputItem::image_file_id(file_id);
        assert_eq!(item.item_type, "input_image");
        assert_eq!(item.text, Some(file_id.to_string()));
        assert!(item.image_url.is_none());
        assert_eq!(item.detail, Some("auto".to_string()));

        // Test file ID input with detail
        let item_with_detail = InputItem::image_file_id_with_detail(file_id, "high");
        assert_eq!(item_with_detail.detail, Some("high".to_string()));

        // Test content helpers
        let content = InputItem::content_image_file_id(file_id);
        assert_eq!(content["type"], "input_image");
        assert_eq!(content["file_id"], file_id);

        let content_with_detail = InputItem::content_image_file_id_with_detail(file_id, "low");
        assert_eq!(content_with_detail["detail"], "low");
    }

    #[test]
    fn test_image_input_detail_levels() {
        use crate::types::InputItem;

        let url = "https://example.com/test.jpg";

        // Test URL with detail
        let high_detail = InputItem::image_url_with_detail(url, "high");
        assert_eq!(high_detail.detail, Some("high".to_string()));

        let low_detail = InputItem::image_url_with_detail(url, "low");
        assert_eq!(low_detail.detail, Some("low".to_string()));

        let auto_detail = InputItem::image_url_with_detail(url, "auto");
        assert_eq!(auto_detail.detail, Some("auto".to_string()));

        // Test content with detail
        let content_high = InputItem::content_image_with_detail(url, "high");
        assert_eq!(content_high["detail"], "high");
    }

    #[test]
    fn test_request_builder_enhanced_image_methods() {
        use crate::{Model, Request};

        // Test base64 image input
        let base64_data = "test_base64_data";
        let request_base64 = Request::builder()
            .model(Model::GPT4o)
            .input_image_base64(base64_data, "image/jpeg")
            .build();

        match request_base64.input {
            crate::Input::Items(items) => {
                assert_eq!(items.len(), 1);
                let content = items[0].content.as_ref().unwrap().as_array().unwrap();
                assert_eq!(content[0]["type"], "input_image");
                assert!(content[0]["image_url"]
                    .as_str()
                    .unwrap()
                    .contains("data:image/jpeg;base64,"));
            }
            crate::Input::Text(_) => panic!("Expected items input"),
        }

        // Test base64 with detail
        let request_base64_detail = Request::builder()
            .model(Model::GPT4o)
            .input_image_base64_with_detail(base64_data, "image/png", "high")
            .build();

        match request_base64_detail.input {
            crate::Input::Items(items) => {
                let content = items[0].content.as_ref().unwrap().as_array().unwrap();
                assert_eq!(content[0]["detail"], "high");
            }
            crate::Input::Text(_) => panic!("Expected items input"),
        }

        // Test file ID input
        let file_id = "file-123";
        let request_file_id = Request::builder()
            .model(Model::GPT4o)
            .input_image_file_id(file_id)
            .build();

        match request_file_id.input {
            crate::Input::Items(items) => {
                let content = items[0].content.as_ref().unwrap().as_array().unwrap();
                assert_eq!(content[0]["type"], "input_image");
                assert_eq!(content[0]["file_id"], file_id);
            }
            crate::Input::Text(_) => panic!("Expected items input"),
        }

        // Test URL with detail
        let url = "https://example.com/test.jpg";
        let request_url_detail = Request::builder()
            .model(Model::GPT4o)
            .input_image_url_with_detail(url, "low")
            .build();

        match request_url_detail.input {
            crate::Input::Items(items) => {
                let content = items[0].content.as_ref().unwrap().as_array().unwrap();
                assert_eq!(content[0]["detail"], "low");
            }
            crate::Input::Text(_) => panic!("Expected items input"),
        }
    }
}
