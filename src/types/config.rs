use serde::{Deserialize, Serialize};
use std::env;

/// Scope that controls which recoverable errors should be retried automatically.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RetryScope {
    /// Retry any recoverable error class.
    #[default]
    AllRecoverable,
    /// Retry only container expiration style errors.
    ContainerOnly,
    /// Retry transient HTTP or server failures.
    TransientOnly,
}

impl RetryScope {
    /// Returns a human-friendly label for telemetry and logging.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllRecoverable => "all_recoverable",
            Self::ContainerOnly => "container_only",
            Self::TransientOnly => "transient_only",
        }
    }
}

/// Recovery policy for handling container expiration and other recoverable errors.
///
/// When constructed via [`RecoveryPolicy::from_env`], any environment variables that are
/// missing (or fail to parse) leave the default values unchanged.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecoveryPolicy {
    /// Whether to automatically retry on expired container errors
    pub auto_retry_on_expired_container: bool,

    /// Whether to notify the application when a reset occurs
    pub notify_on_reset: bool,

    /// Maximum number of retry attempts for recoverable errors
    pub max_retries: u32,

    /// Whether to automatically prune expired containers from context
    pub auto_prune_expired_containers: bool,

    /// Custom user-friendly message to show when containers are reset
    pub reset_message: Option<String>,

    /// Whether to log recovery attempts (useful for debugging)
    pub log_recovery_attempts: bool,

    /// Scope that limits which recoverable errors are retried
    #[serde(default)]
    pub retry_scope: RetryScope,
}

impl Default for RecoveryPolicy {
    fn default() -> Self {
        Self {
            auto_retry_on_expired_container: true,
            notify_on_reset: false,
            max_retries: 1,
            auto_prune_expired_containers: true,
            reset_message: None,
            log_recovery_attempts: false,
            retry_scope: RetryScope::default(),
        }
    }
}

impl RecoveryPolicy {
    /// Creates a new recovery policy with default settings
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a recovery policy by reading optional environment overrides.
    ///
    /// Supported variables:
    ///
    /// - `OAI_RECOVERY_MAX_RETRIES` (`u32`)
    /// - `OAI_RECOVERY_AUTO_RETRY` (`bool`)
    /// - `OAI_RECOVERY_AUTO_PRUNE` (`bool`)
    /// - `OAI_RECOVERY_LOG` (`bool`)
    /// - `OAI_RECOVERY_SCOPE` (`all | container | transient`)
    ///
    /// Any variable that is unset or fails to parse will leave the default value intact.
    #[must_use]
    pub fn from_env() -> Self {
        let mut policy = Self::default();

        if let Ok(value) = env::var("OAI_RECOVERY_MAX_RETRIES") {
            let trimmed = value.trim();
            match trimmed.parse::<u32>() {
                Ok(parsed) => {
                    policy.max_retries = parsed;
                }
                Err(error) => {
                    log::warn!(
                        "Failed to parse OAI_RECOVERY_MAX_RETRIES='{}': {error}; using default {}",
                        trimmed,
                        policy.max_retries
                    );
                }
            }
        }

        if let Ok(value) = env::var("OAI_RECOVERY_AUTO_RETRY") {
            let trimmed = value.trim();
            match trimmed.parse::<bool>() {
                Ok(parsed) => {
                    policy.auto_retry_on_expired_container = parsed;
                }
                Err(error) => {
                    log::warn!(
                        "Failed to parse OAI_RECOVERY_AUTO_RETRY='{}': {error}; using default {}",
                        trimmed,
                        policy.auto_retry_on_expired_container
                    );
                }
            }
        }

        if let Ok(value) = env::var("OAI_RECOVERY_AUTO_PRUNE") {
            let trimmed = value.trim();
            match trimmed.parse::<bool>() {
                Ok(parsed) => {
                    policy.auto_prune_expired_containers = parsed;
                }
                Err(error) => {
                    log::warn!(
                        "Failed to parse OAI_RECOVERY_AUTO_PRUNE='{}': {error}; using default {}",
                        trimmed,
                        policy.auto_prune_expired_containers
                    );
                }
            }
        }

        if let Ok(value) = env::var("OAI_RECOVERY_LOG") {
            let trimmed = value.trim();
            match trimmed.parse::<bool>() {
                Ok(parsed) => {
                    policy.log_recovery_attempts = parsed;
                }
                Err(error) => {
                    log::warn!(
                        "Failed to parse OAI_RECOVERY_LOG='{}': {error}; using default {}",
                        trimmed,
                        policy.log_recovery_attempts
                    );
                }
            }
        }

        if let Ok(value) = env::var("OAI_RECOVERY_SCOPE") {
            let trimmed = value.trim().to_ascii_lowercase();
            match trimmed.as_str() {
                "all" => {
                    policy.retry_scope = RetryScope::AllRecoverable;
                }
                "container" => {
                    policy.retry_scope = RetryScope::ContainerOnly;
                }
                "transient" => {
                    policy.retry_scope = RetryScope::TransientOnly;
                }
                _ => {
                    log::warn!(
                        "Unrecognized OAI_RECOVERY_SCOPE='{}'; expected all|container|transient; using default {}",
                        trimmed,
                        policy.retry_scope.as_str()
                    );
                }
            }
        }

        policy
    }

    /// Creates a conservative recovery policy (no automatic retries)
    #[must_use]
    pub fn conservative() -> Self {
        Self {
            auto_retry_on_expired_container: false,
            notify_on_reset: true,
            max_retries: 0,
            auto_prune_expired_containers: false,
            reset_message: None,
            log_recovery_attempts: true,
            retry_scope: RetryScope::ContainerOnly,
        }
    }

    /// Creates an aggressive recovery policy (multiple retries, automatic pruning)
    #[must_use]
    pub fn aggressive() -> Self {
        Self {
            auto_retry_on_expired_container: true,
            notify_on_reset: false,
            max_retries: 3,
            auto_prune_expired_containers: true,
            reset_message: Some(
                "Your previous code session expired, so I've started a fresh conversation for you."
                    .to_string(),
            ),
            log_recovery_attempts: true,
            retry_scope: RetryScope::AllRecoverable,
        }
    }

    /// Sets whether to automatically retry on expired container errors
    #[must_use]
    pub fn with_auto_retry(mut self, auto_retry: bool) -> Self {
        self.auto_retry_on_expired_container = auto_retry;
        self
    }

    /// Sets whether to notify the application when a reset occurs
    #[must_use]
    pub fn with_notify_on_reset(mut self, notify: bool) -> Self {
        self.notify_on_reset = notify;
        self
    }

    /// Sets the maximum number of retry attempts
    #[must_use]
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Sets whether to automatically prune expired containers
    #[must_use]
    pub fn with_auto_prune(mut self, auto_prune: bool) -> Self {
        self.auto_prune_expired_containers = auto_prune;
        self
    }

    /// Sets a custom reset message
    #[must_use]
    pub fn with_reset_message(mut self, message: impl Into<String>) -> Self {
        self.reset_message = Some(message.into());
        self
    }

    /// Sets whether to log recovery attempts
    #[must_use]
    pub fn with_logging(mut self, log: bool) -> Self {
        self.log_recovery_attempts = log;
        self
    }

    /// Sets the retry scope that controls which errors can trigger retries
    #[must_use]
    pub fn with_retry_scope(mut self, retry_scope: RetryScope) -> Self {
        self.retry_scope = retry_scope;
        self
    }

    /// Returns the user-friendly reset message
    #[must_use]
    pub fn get_reset_message(&self) -> String {
        self.reset_message.clone().unwrap_or_else(|| {
            "Your previous session expired, so I've started a fresh conversation for you."
                .to_string()
        })
    }
}

/// Callback function type for recovery notifications
pub type RecoveryCallback = Box<dyn Fn(&crate::Error, u32) + Send + Sync>;

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

    /// Recovery policy for handling container expiration and other recoverable errors
    #[serde(default)]
    pub recovery_policy: RecoveryPolicy,
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
            recovery_policy: RecoveryPolicy::default(),
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

    /// Sets a custom recovery policy for the client
    #[must_use]
    pub fn with_recovery_policy(mut self, policy: RecoveryPolicy) -> Self {
        self.recovery_policy = policy;
        self
    }

    /// Sets a conservative recovery policy (no automatic retries)
    #[must_use]
    pub fn with_conservative_recovery(mut self) -> Self {
        self.recovery_policy = RecoveryPolicy::conservative();
        self
    }

    /// Sets an aggressive recovery policy (multiple retries, automatic pruning)
    #[must_use]
    pub fn with_aggressive_recovery(mut self) -> Self {
        self.recovery_policy = RecoveryPolicy::aggressive();
        self
    }
}

/// Model types for the OpenAI Responses API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Model {
    // === GPT-5 Series (2025-08-07) ===
    /// GPT-5 model (2025-08-07) - Latest flagship model with advanced agentic capabilities
    #[serde(rename = "gpt-5")]
    GPT5,

    /// GPT-5 Mini model (2025-08-07) - Balanced performance and efficiency
    #[serde(rename = "gpt-5-mini")]
    GPT5Mini,

    /// GPT-5 Nano model (2025-08-07) - Fastest, most efficient variant
    #[serde(rename = "gpt-5-nano")]
    GPT5Nano,

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

    // === Image Generation Models ===
    /// GPT Image 1 model - For Images API only (not Responses API)
    #[serde(rename = "gpt-image-1")]
    GPTImage1,

    /// Custom model string for future models or specialized deployments
    #[serde(untagged)]
    Custom(String),
}

impl From<String> for Model {
    fn from(s: String) -> Self {
        match s.as_str() {
            // GPT-5 Series
            "gpt-5" => Self::GPT5,
            "gpt-5-mini" => Self::GPT5Mini,
            "gpt-5-nano" => Self::GPT5Nano,

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

            // Image Generation Models
            "gpt-image-1" => Self::GPTImage1,

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
            // GPT-5 Series
            Model::GPT5 => write!(f, "gpt-5"),
            Model::GPT5Mini => write!(f, "gpt-5-mini"),
            Model::GPT5Nano => write!(f, "gpt-5-nano"),

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

            // Image Generation Models
            Model::GPTImage1 => write!(f, "gpt-image-1"),

            // Custom fallback
            Model::Custom(s) => write!(f, "{s}"),
        }
    }
}
