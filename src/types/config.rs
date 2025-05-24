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

    /// Sets a custom base URL for the client
    #[must_use]
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Sets an organization ID for the client
    #[must_use]
    pub fn with_organization_id(mut self, organization_id: impl Into<String>) -> Self {
        self.organization_id = Some(organization_id.into());
        self
    }
}

/// Model types for the OpenAI Responses API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Model {
    // === Latest Generation (2025) ===
    /// o3 reasoning model (2025-04-16) - Latest reasoning model
    #[serde(rename = "o3")]
    O3,

    /// o4-mini reasoning model (2025-04-16) - NEW reasoning model with enhanced capabilities
    #[serde(rename = "o4-mini")]
    O4Mini,

    /// GPT-4.1 model (2025-04-14) - Latest GPT model
    #[serde(rename = "gpt-4.1")]
    GPT41,

    /// GPT-4.1 Nano model (2025-04-14) - Fastest 4.1 model
    #[serde(rename = "gpt-4.1-nano")]
    GPT41Nano,

    /// GPT-4.1 Mini model (2025-04-14) - Smaller 4.1 model
    #[serde(rename = "gpt-4.1-mini")]
    GPT41Mini,

    // === O-Series (Reasoning Models) ===
    /// o3-mini reasoning model (2025-01-31)
    #[serde(rename = "o3-mini")]
    O3Mini,

    /// o1 reasoning model (2024-12-17) - Current production reasoning model
    #[serde(rename = "o1")]
    O1,

    /// o1-preview reasoning model (2024-09-12) - Preview version
    #[serde(rename = "o1-preview")]
    O1Preview,

    /// o1-mini reasoning model (2024-09-12) - Faster, cost-efficient reasoning
    #[serde(rename = "o1-mini")]
    O1Mini,

    // === GPT-4o Family ===
    /// GPT-4o model (latest)
    #[serde(rename = "gpt-4o")]
    GPT4o,

    /// GPT-4o model (2024-11-20) - Latest GPT-4o version
    #[serde(rename = "gpt-4o-2024-11-20")]
    GPT4o20241120,

    /// GPT-4o model (2024-08-06)
    #[serde(rename = "gpt-4o-2024-08-06")]
    GPT4o20240806,

    /// GPT-4o model (2024-05-13)
    #[serde(rename = "gpt-4o-2024-05-13")]
    GPT4o20240513,

    /// GPT-4o Mini model (2024-07-18) - Fast, inexpensive, capable model
    #[serde(rename = "gpt-4o-mini")]
    GPT4oMini,

    // === GPT-4 Family ===
    /// GPT-4 Turbo model (latest)
    #[serde(rename = "gpt-4-turbo")]
    GPT4Turbo,

    /// GPT-4 Turbo with Vision (2024-04-09)
    #[serde(rename = "gpt-4-turbo-2024-04-09")]
    GPT4Turbo20240409,

    /// GPT-4 model
    #[serde(rename = "gpt-4")]
    GPT4,

    /// GPT-4 32k context model
    #[serde(rename = "gpt-4-32k")]
    GPT4_32k,

    // === GPT-3.5 Family ===
    /// GPT-3.5 Turbo model (latest)
    #[serde(rename = "gpt-3.5-turbo")]
    GPT35Turbo,

    /// GPT-3.5 Turbo model (0125) - Latest GA model with JSON mode
    #[serde(rename = "gpt-3.5-turbo-0125")]
    GPT35Turbo0125,

    /// GPT-3.5 Turbo model (1106) - With JSON mode and parallel function calling
    #[serde(rename = "gpt-3.5-turbo-1106")]
    GPT35Turbo1106,

    /// GPT-3.5 Turbo Instruct model (0914) - Completions endpoint only
    #[serde(rename = "gpt-3.5-turbo-instruct")]
    GPT35TurboInstruct,

    /// Custom model string for future models or specialized deployments
    #[serde(untagged)]
    Custom(String),
}

impl From<String> for Model {
    fn from(s: String) -> Self {
        match s.as_str() {
            // Latest Generation (2025)
            "o3" => Self::O3,
            "o4-mini" => Self::O4Mini,
            "gpt-4.1" => Self::GPT41,
            "gpt-4.1-nano" => Self::GPT41Nano,
            "gpt-4.1-mini" => Self::GPT41Mini,

            // O-Series (Reasoning Models)
            "o3-mini" => Self::O3Mini,
            "o1" => Self::O1,
            "o1-preview" => Self::O1Preview,
            "o1-mini" => Self::O1Mini,

            // GPT-4o Family
            "gpt-4o" => Self::GPT4o,
            "gpt-4o-2024-11-20" => Self::GPT4o20241120,
            "gpt-4o-2024-08-06" => Self::GPT4o20240806,
            "gpt-4o-2024-05-13" => Self::GPT4o20240513,
            "gpt-4o-mini" => Self::GPT4oMini,

            // GPT-4 Family
            "gpt-4-turbo" => Self::GPT4Turbo,
            "gpt-4-turbo-2024-04-09" => Self::GPT4Turbo20240409,
            "gpt-4" => Self::GPT4,
            "gpt-4-32k" => Self::GPT4_32k,

            // GPT-3.5 Family
            "gpt-3.5-turbo" => Self::GPT35Turbo,
            "gpt-3.5-turbo-0125" => Self::GPT35Turbo0125,
            "gpt-3.5-turbo-1106" => Self::GPT35Turbo1106,
            "gpt-3.5-turbo-instruct" => Self::GPT35TurboInstruct,

            // Custom fallback
            _ => Self::Custom(s),
        }
    }
}

impl From<&str> for Model {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Latest Generation (2025)
            Model::O3 => write!(f, "o3"),
            Model::O4Mini => write!(f, "o4-mini"),
            Model::GPT41 => write!(f, "gpt-4.1"),
            Model::GPT41Nano => write!(f, "gpt-4.1-nano"),
            Model::GPT41Mini => write!(f, "gpt-4.1-mini"),

            // O-Series (Reasoning Models)
            Model::O3Mini => write!(f, "o3-mini"),
            Model::O1 => write!(f, "o1"),
            Model::O1Preview => write!(f, "o1-preview"),
            Model::O1Mini => write!(f, "o1-mini"),

            // GPT-4o Family
            Model::GPT4o => write!(f, "gpt-4o"),
            Model::GPT4o20241120 => write!(f, "gpt-4o-2024-11-20"),
            Model::GPT4o20240806 => write!(f, "gpt-4o-2024-08-06"),
            Model::GPT4o20240513 => write!(f, "gpt-4o-2024-05-13"),
            Model::GPT4oMini => write!(f, "gpt-4o-mini"),

            // GPT-4 Family
            Model::GPT4Turbo => write!(f, "gpt-4-turbo"),
            Model::GPT4Turbo20240409 => write!(f, "gpt-4-turbo-2024-04-09"),
            Model::GPT4 => write!(f, "gpt-4"),
            Model::GPT4_32k => write!(f, "gpt-4-32k"),

            // GPT-3.5 Family
            Model::GPT35Turbo => write!(f, "gpt-3.5-turbo"),
            Model::GPT35Turbo0125 => write!(f, "gpt-3.5-turbo-0125"),
            Model::GPT35Turbo1106 => write!(f, "gpt-3.5-turbo-1106"),
            Model::GPT35TurboInstruct => write!(f, "gpt-3.5-turbo-instruct"),

            // Custom fallback
            Model::Custom(s) => write!(f, "{s}"),
        }
    }
}
