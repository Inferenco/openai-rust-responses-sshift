use serde::{Deserialize, Serialize};

/// Configuration for the OpenAI Responses API client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// API key for authentication
    pub api_key: String,
    
    /// Base URL for the API
    #[serde(default = "default_base_url")]
    pub base_url: String,
    
    /// Organization ID for the API
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
}

fn default_base_url() -> String {
    "https://api.openai.com/v1".to_string()
}

impl Config {
    /// Creates a new configuration with the given API key
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: default_base_url(),
            organization_id: None,
        }
    }
    
    /// Sets the base URL
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }
    
    /// Sets the organization ID
    pub fn with_organization_id(mut self, organization_id: impl Into<String>) -> Self {
        self.organization_id = Some(organization_id.into());
        self
    }
}

/// Model types for the OpenAI Responses API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Model {
    /// GPT-4o model
    #[serde(rename = "gpt-4o")]
    GPT4o,
    
    /// GPT-4 Turbo model
    #[serde(rename = "gpt-4-turbo")]
    GPT4Turbo,
    
    /// GPT-4 model
    #[serde(rename = "gpt-4")]
    GPT4,
    
    /// GPT-3.5 Turbo model
    #[serde(rename = "gpt-3.5-turbo")]
    GPT35Turbo,
    
    /// Custom model
    #[serde(untagged)]
    Custom(String),
}

impl From<String> for Model {
    fn from(s: String) -> Self {
        match s.as_str() {
            "gpt-4o" => Self::GPT4o,
            "gpt-4-turbo" => Self::GPT4Turbo,
            "gpt-4" => Self::GPT4,
            "gpt-3.5-turbo" => Self::GPT35Turbo,
            _ => Self::Custom(s),
        }
    }
}

impl From<&str> for Model {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}
