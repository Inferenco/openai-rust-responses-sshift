use serde::{Deserialize, Serialize};

/// Effort level for reasoning models
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Effort {
    /// Low effort reasoning - faster responses
    Low,
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
pub enum SummarySetting {
    /// Automatically generate a reasoning summary
    #[serde(rename = "auto")]
    Auto,
    /// Custom summary text
    #[serde(untagged)]
    Text(String),
}

impl From<&str> for SummarySetting {
    fn from(s: &str) -> Self {
        match s {
            "auto" => Self::Auto,
            text => Self::Text(text.to_string()),
        }
    }
}

/// Reasoning parameters for controlling reasoning model behavior
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReasoningParams {
    /// Effort level for reasoning (low/high)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<Effort>,
    
    /// Summary setting for reasoning output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<SummarySetting>,
}

impl ReasoningParams {
    /// Create new reasoning parameters
    pub fn new() -> Self {
        Self {
            effort: None,
            summary: None,
        }
    }
    
    /// Set effort level
    pub fn with_effort(mut self, effort: Effort) -> Self {
        self.effort = Some(effort);
        self
    }
    
    /// Set summary setting
    pub fn with_summary(mut self, summary: SummarySetting) -> Self {
        self.summary = Some(summary);
        self
    }
    
    /// Enable high effort reasoning (enables background mode)
    pub fn high_effort() -> Self {
        Self::new().with_effort(Effort::High)
    }
    
    /// Enable automatic summary generation
    pub fn auto_summary() -> Self {
        Self::new().with_summary(SummarySetting::Auto)
    }
    
    /// Create reasoning params with high effort and auto summary
    pub fn high_effort_with_summary() -> Self {
        Self::new()
            .with_effort(Effort::High)
            .with_summary(SummarySetting::Auto)
    }
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
        let effort_high = Effort::High;
        
        assert_eq!(serde_json::to_string(&effort_low).unwrap(), r#""low""#);
        assert_eq!(serde_json::to_string(&effort_high).unwrap(), r#""high""#);
    }

    #[test]
    fn test_summary_setting_serialization() {
        let auto = SummarySetting::Auto;
        let text = SummarySetting::Text("custom summary".to_string());
        
        // Auto should serialize to just "auto"
        assert_eq!(serde_json::to_string(&auto).unwrap(), r#""auto""#);
        // Text should serialize to the string content
        assert_eq!(serde_json::to_string(&text).unwrap(), r#""custom summary""#);
    }
} 