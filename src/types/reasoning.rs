use serde::{Deserialize, Serialize};

/// Effort level for reasoning models
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Effort {
    /// Low effort reasoning - faster responses
    Low,
    /// Medium effort reasoning - balanced speed and thoroughness
    Medium,
    /// High effort reasoning - more thorough analysis (enables background mode)
    High,
}

impl Default for Effort {
    fn default() -> Self {
        Self::Low
    }
}

/// Summary setting for reasoning output
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SummarySetting {
    /// Automatically generate a reasoning summary
    Auto,
    /// Generate a concise reasoning summary
    Concise,
    /// Generate a detailed reasoning summary
    Detailed,
}

impl From<&str> for SummarySetting {
    fn from(s: &str) -> Self {
        match s {
            "concise" => Self::Concise,
            "detailed" => Self::Detailed,
            _ => Self::Auto, // Default fallback for "auto" and any other values
        }
    }
}

/// Reasoning parameters for controlling reasoning model behavior
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReasoningParams {
    /// Effort level for reasoning (low/medium/high)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<Effort>,

    /// Summary setting for reasoning output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<SummarySetting>,

    /// GPT-5 reasoning effort control (minimal/medium/high)
    /// Internal only: we derive `effort` for wire format as API expects
    #[serde(skip_serializing)]
    pub reasoning_effort: Option<ReasoningEffort>,
}

impl ReasoningParams {
    /// Create new reasoning parameters
    #[must_use]
    pub fn new() -> Self {
        Self {
            effort: None,
            summary: None,
            reasoning_effort: None,
        }
    }

    /// Set effort level
    #[must_use]
    pub fn with_effort(mut self, effort: Effort) -> Self {
        self.effort = Some(effort);
        self
    }

    /// Set summary setting
    #[must_use]
    pub fn with_summary(mut self, summary: SummarySetting) -> Self {
        self.summary = Some(summary);
        self
    }

    /// Set GPT-5 reasoning effort level; also map to `effort` for backward compatibility
    #[must_use]
    pub fn with_reasoning_effort(mut self, effort: ReasoningEffort) -> Self {
        self.effort = Some(match effort {
            ReasoningEffort::Minimal => Effort::Low,
            ReasoningEffort::Medium => Effort::Medium,
            ReasoningEffort::High => Effort::High,
        });
        self.reasoning_effort = Some(effort);
        self
    }

    // (second with_reasoning_effort removed; deduplicated)

    /// Enable medium effort reasoning (balanced speed and thoroughness)
    #[must_use]
    pub fn medium_effort() -> Self {
        Self::new().with_effort(Effort::Medium)
    }

    /// Enable high effort reasoning (enables background mode)
    #[must_use]
    pub fn high_effort() -> Self {
        Self::new().with_effort(Effort::High)
    }

    /// Enable automatic summary generation
    #[must_use]
    pub fn auto_summary() -> Self {
        Self::new().with_summary(SummarySetting::Auto)
    }

    /// Enable concise summary generation
    #[must_use]
    pub fn concise_summary() -> Self {
        Self::new().with_summary(SummarySetting::Concise)
    }

    /// Enable detailed summary generation
    #[must_use]
    pub fn detailed_summary() -> Self {
        Self::new().with_summary(SummarySetting::Detailed)
    }

    /// Create reasoning params with medium effort and auto summary
    #[must_use]
    pub fn medium_effort_with_summary() -> Self {
        Self::new()
            .with_effort(Effort::Medium)
            .with_summary(SummarySetting::Auto)
    }

    /// Create reasoning params with medium effort and concise summary
    #[must_use]
    pub fn medium_effort_concise() -> Self {
        Self::new()
            .with_effort(Effort::Medium)
            .with_summary(SummarySetting::Concise)
    }

    /// Create reasoning params with medium effort and detailed summary
    #[must_use]
    pub fn medium_effort_detailed() -> Self {
        Self::new()
            .with_effort(Effort::Medium)
            .with_summary(SummarySetting::Detailed)
    }

    /// Create reasoning params with high effort and auto summary
    #[must_use]
    pub fn high_effort_with_summary() -> Self {
        Self::new()
            .with_effort(Effort::High)
            .with_summary(SummarySetting::Auto)
    }

    /// Create reasoning params with high effort and concise summary
    #[must_use]
    pub fn high_effort_concise() -> Self {
        Self::new()
            .with_effort(Effort::High)
            .with_summary(SummarySetting::Concise)
    }

    /// Create reasoning params with high effort and detailed summary
    #[must_use]
    pub fn high_effort_detailed() -> Self {
        Self::new()
            .with_effort(Effort::High)
            .with_summary(SummarySetting::Detailed)
    }
}

/// Reasoning effort levels for GPT-5 models
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    /// Minimal reasoning tokens for fast, deterministic tasks
    Minimal,
    /// Balanced reasoning depth and performance (default)
    Medium,
    /// Maximum reasoning depth for complex problem-solving
    High,
}

impl Default for ReasoningParams {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reasoning_params_creation() {
        let params = ReasoningParams::new();
        assert_eq!(params.effort, None);
        assert_eq!(params.summary, None);
    }

    #[test]
    fn test_reasoning_params_builders() {
        let params = ReasoningParams::high_effort_with_summary();
        assert_eq!(params.effort, Some(Effort::High));
        assert_eq!(params.summary, Some(SummarySetting::Auto));
    }

    #[test]
    fn test_effort_serialization() {
        let effort_low = Effort::Low;
        let effort_medium = Effort::Medium;
        let effort_high = Effort::High;

        assert_eq!(
            serde_json::to_string(&effort_low).expect("effort_low should serialize"),
            r#""low""#
        );
        assert_eq!(
            serde_json::to_string(&effort_medium).expect("effort_medium should serialize"),
            r#""medium""#
        );
        assert_eq!(
            serde_json::to_string(&effort_high).expect("effort_high should serialize"),
            r#""high""#
        );
    }

    #[test]
    fn test_medium_effort_builders() {
        // Test medium effort builder
        let medium_effort = ReasoningParams::medium_effort();
        assert_eq!(medium_effort.effort, Some(Effort::Medium));
        assert_eq!(medium_effort.summary, None);

        // Test combined medium effort builders
        let medium_with_summary = ReasoningParams::medium_effort_with_summary();
        assert_eq!(medium_with_summary.effort, Some(Effort::Medium));
        assert_eq!(medium_with_summary.summary, Some(SummarySetting::Auto));

        let medium_concise = ReasoningParams::medium_effort_concise();
        assert_eq!(medium_concise.effort, Some(Effort::Medium));
        assert_eq!(medium_concise.summary, Some(SummarySetting::Concise));

        let medium_detailed = ReasoningParams::medium_effort_detailed();
        assert_eq!(medium_detailed.effort, Some(Effort::Medium));
        assert_eq!(medium_detailed.summary, Some(SummarySetting::Detailed));
    }

    #[test]
    fn test_summary_setting_serialization() {
        let auto = SummarySetting::Auto;
        let concise = SummarySetting::Concise;
        let detailed = SummarySetting::Detailed;

        // Auto should serialize to just "auto"
        assert_eq!(
            serde_json::to_string(&auto).expect("auto should serialize"),
            r#""auto""#
        );
        // Concise should serialize to "concise"
        assert_eq!(
            serde_json::to_string(&concise).expect("concise should serialize"),
            r#""concise""#
        );
        // Detailed should serialize to "detailed"
        assert_eq!(
            serde_json::to_string(&detailed).expect("detailed should serialize"),
            r#""detailed""#
        );
    }
}
