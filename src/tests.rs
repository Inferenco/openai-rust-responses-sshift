#[cfg(test)]
mod tests {
    use crate::{Client, CreateError, Request, Model, Input};
    
    #[test]
    fn test_client_creation() {
        // Test client creation with invalid API key
        let result = Client::new("");
        assert!(matches!(result, Err(CreateError::InvalidApiKey)));
        
        // Test client creation with valid API key
        let result = Client::new("sk-valid_api_key_for_testing");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_request_builder() {
        let request = Request::builder()
            .model(Model::GPT4o)
            .input("Test input")
            .instructions("Test instructions")
            .max_tokens(100)
            .temperature(0.7)
            .build();
            
        assert_eq!(request.model, Model::GPT4o);
        assert!(matches!(request.input, Input::Text(text) if text == "Test input"));
        assert_eq!(request.instructions, Some("Test instructions".to_string()));
        assert_eq!(request.max_tokens, Some(100));
        assert_eq!(request.temperature, Some(0.7));
    }
    
    #[tokio::test]
    #[ignore = "Requires API key"]
    async fn test_create_response() {
        let client = match Client::from_env() {
            Ok(client) => client,
            Err(_) => return, // Skip test if no API key is available
        };
        
        let result = client.responses.create(Request {
            model: Model::GPT4o,
            input: Input::Text("Hello, world!".to_string()),
            ..Default::default()
        }).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.output_text().is_empty());
    }
    
    #[tokio::test]
    #[ignore = "Requires API key"]
    async fn test_thread_operations() {
        let client = match Client::from_env() {
            Ok(client) => client,
            Err(_) => return, // Skip test if no API key is available
        };
        
        // Create a thread
        let thread_result = client.threads.create(crate::threads::CreateThreadRequest {
            model: Model::GPT4o,
            instructions: Some("Test instructions".to_string()),
            initial_message: "Hello, world!".to_string(),
            metadata: None,
        }).await;
        assert!(thread_result.is_ok());
        
        let (thread, _) = thread_result.unwrap();
        assert!(!thread.id.is_empty());
        
        // Get the thread
        let get_result = client.threads.retrieve(&thread.id).await;
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap().id, thread.id);
        
        // Delete the thread
        let delete_result = client.threads.delete(&thread.id).await;
        assert!(delete_result.is_ok());
    }
}
