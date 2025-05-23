#[cfg(test)]
mod unit_tests {
    use crate::{Client, CreateError, Input, Model, Request};

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
}
