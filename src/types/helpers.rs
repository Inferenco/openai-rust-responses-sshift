use serde::{Deserialize, Serialize};

/// Pagination parameters for list endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaginationParams {
    /// Maximum number of items to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Token for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Token for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

/// Paginated list of items
#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedList<T> {
    /// List of items
    pub data: Vec<T>,

    /// Type of object
    pub object: String,

    /// Whether there are more items to retrieve
    pub has_more: bool,

    /// Token for pagination
    pub next_cursor: Option<String>,
}

impl<T> PaginatedList<T> {
    /// Returns the items in this page
    #[must_use]
    pub fn items(&self) -> &[T] {
        &self.data
    }

    /// Returns the number of items in this page
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if this page is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
