use serde::{Deserialize, Serialize};

/// Request for image generation
#[derive(Debug, Clone, Serialize)]
pub struct ImageGenerateRequest {
    /// Model to use (always "gpt-image-1")
    pub model: String,
    /// Prompt for image generation
    pub prompt: String,
    /// Number of images to generate (1-10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    /// Size of generated images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Quality level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
    /// Output format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// Output compression (0-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_compression: Option<u32>,
    /// Background type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    /// Seed for reproducibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    /// User identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Response from image generation
#[derive(Debug, Clone, Deserialize)]
pub struct ImageGenerateResponse {
    /// Unix timestamp of when the image was created
    pub created: u64,
    /// Array of generated image data
    pub data: Vec<ImageData>,
}

/// Individual image data
#[derive(Debug, Clone, Deserialize)]
pub struct ImageData {
    /// URL of the generated image (if using URL response)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Base64 encoded image data (if using base64 response)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b64_json: Option<String>,
    /// The revised prompt that was used for generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revised_prompt: Option<String>,
}

impl ImageGenerateRequest {
    /// Create a new image generation request
    #[must_use]
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            model: "gpt-image-1".to_string(),
            prompt: prompt.into(),
            n: None,
            size: None,
            quality: None,
            output_format: None,
            output_compression: None,
            background: None,
            seed: None,
            user: None,
        }
    }

    /// Set the number of images to generate (1-10)
    #[must_use]
    pub fn with_n(mut self, n: u32) -> Self {
        self.n = Some(n.clamp(1, 10));
        self
    }

    /// Set image size (1024x1024, 1024x1536, 1536x1024)
    #[must_use]
    pub fn with_size(mut self, size: impl Into<String>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Set quality level (low, medium, high, auto)
    #[must_use]
    pub fn with_quality(mut self, quality: impl Into<String>) -> Self {
        self.quality = Some(quality.into());
        self
    }

    /// Set output format (png, jpeg, webp)
    #[must_use]
    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.output_format = Some(format.into());
        self
    }

    /// Set compression level (0-100)
    #[must_use]
    pub fn with_compression(mut self, compression: u32) -> Self {
        self.output_compression = Some(compression.min(100));
        self
    }

    /// Set background type (transparent, etc.)
    #[must_use]
    pub fn with_background(mut self, background: impl Into<String>) -> Self {
        self.background = Some(background.into());
        self
    }

    /// Set seed for reproducibility
    #[must_use]
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set user identifier
    #[must_use]
    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}
