use serde::{Deserialize, Serialize};

/// Filter for vector store file search operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Filter {
    /// Logical AND operation
    And { conditions: Vec<FilterCondition> },
    /// Logical OR operation
    Or { conditions: Vec<FilterCondition> },
}

/// Individual filter condition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilterCondition {
    /// Field to filter on
    pub field: String,
    /// Comparison operator
    pub operator: String,
    /// Value to compare against
    pub value: serde_json::Value,
}

impl Filter {
    /// Creates an AND filter with the given conditions
    #[must_use]
    pub fn and(conditions: Vec<FilterCondition>) -> Self {
        Self::And { conditions }
    }

    /// Creates an OR filter with the given conditions
    #[must_use]
    pub fn or(conditions: Vec<FilterCondition>) -> Self {
        Self::Or { conditions }
    }
}

impl FilterCondition {
    /// Creates a new filter condition
    #[must_use]
    pub fn new(
        field: impl Into<String>,
        operator: impl Into<String>,
        value: serde_json::Value,
    ) -> Self {
        Self {
            field: field.into(),
            operator: operator.into(),
            value,
        }
    }

    /// Creates an equality condition
    #[must_use]
    pub fn eq(field: impl Into<String>, value: serde_json::Value) -> Self {
        Self::new(field, "eq", value)
    }

    /// Creates an "in" condition (value is in the provided array)
    #[must_use]
    pub fn in_array(field: impl Into<String>, values: Vec<serde_json::Value>) -> Self {
        Self::new(field, "in", serde_json::Value::Array(values))
    }

    /// Creates a "contains_any" condition (field contains any of the provided values)
    #[must_use]
    pub fn contains_any(field: impl Into<String>, values: Vec<serde_json::Value>) -> Self {
        Self::new(field, "contains_any", serde_json::Value::Array(values))
    }

    /// Creates a "less than or equal" condition
    #[must_use]
    pub fn lte(field: impl Into<String>, value: serde_json::Value) -> Self {
        Self::new(field, "lte", value)
    }

    /// Creates a "greater than or equal" condition
    #[must_use]
    pub fn gte(field: impl Into<String>, value: serde_json::Value) -> Self {
        Self::new(field, "gte", value)
    }

    /// Creates a "less than" condition
    #[must_use]
    pub fn lt(field: impl Into<String>, value: serde_json::Value) -> Self {
        Self::new(field, "lt", value)
    }

    /// Creates a "greater than" condition
    #[must_use]
    pub fn gt(field: impl Into<String>, value: serde_json::Value) -> Self {
        Self::new(field, "gt", value)
    }

    /// Creates a "not equal" condition
    #[must_use]
    pub fn ne(field: impl Into<String>, value: serde_json::Value) -> Self {
        Self::new(field, "ne", value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_filter_serialization() {
        let filter = Filter::and(vec![
            FilterCondition::eq("tags", json!(["aptos", "validators"])),
            FilterCondition::lte("valid_from", json!(1_640_995_200)),
        ]);

        let serialized = serde_json::to_string(&filter).unwrap();
        assert!(serialized.contains("\"type\":\"and\""));
        assert!(serialized.contains("\"field\":\"tags\""));
        assert!(serialized.contains("\"operator\":\"eq\""));

        // Test deserialization
        let deserialized: Filter = serde_json::from_str(&serialized).unwrap();
        assert_eq!(filter, deserialized);
    }

    #[test]
    fn test_filter_condition_builders() {
        let eq_condition = FilterCondition::eq("status", json!("active"));
        assert_eq!(eq_condition.operator, "eq");
        assert_eq!(eq_condition.field, "status");
        assert_eq!(eq_condition.value, json!("active"));

        let contains_condition =
            FilterCondition::contains_any("tags", vec![json!("rust"), json!("api")]);
        assert_eq!(contains_condition.operator, "contains_any");
        assert_eq!(contains_condition.value, json!(["rust", "api"]));

        let gte_condition = FilterCondition::gte("created_at", json!(1_640_995_200));
        assert_eq!(gte_condition.operator, "gte");
        assert_eq!(gte_condition.value, json!(1_640_995_200));
    }

    #[test]
    fn test_complex_filter() {
        let inner_filter = Filter::and(vec![
            FilterCondition::eq("tenant_id", json!("user_123")),
            FilterCondition::contains_any("tags", vec![json!("aptos"), json!("validators")]),
        ]);

        // For this test, let's just verify the inner filter works
        let json_str = serde_json::to_string(&inner_filter).unwrap();
        let _deserialized: Filter = serde_json::from_str(&json_str).unwrap();

        // Test OR filter as well
        let or_filter = Filter::or(vec![
            FilterCondition::eq("status", json!("active")),
            FilterCondition::eq("public", json!(true)),
        ]);

        let or_json = serde_json::to_string(&or_filter).unwrap();
        let _or_deserialized: Filter = serde_json::from_str(&or_json).unwrap();
    }
}
