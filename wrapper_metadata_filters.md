My draft wrapper upgrade plan as discussed

---

# 1) Add `filters` support to the `file_search` tool

**Why:** Quark needs to narrow retrieval to *only* files matching user‑supplied tags / time windows. Your wrapper currently exposes `Tool::file_search(vec![...])` but **no way to transmit filters**.

### File: `src/types/tools.rs`

#### A) Extend `Tool` with a `filters` field

```rust
// add near other Option<> fields on Tool
/// Metadata filters for file_search (schema is backend-defined; pass raw JSON)
#[serde(skip_serializing_if = "Option::is_none")]
pub filters: Option<serde_json::Value>,
```

#### B) Add a new constructor

```rust
impl Tool {
    /// Creates a file search tool with filters
    #[must_use]
    pub fn file_search_with_filters(
        vector_store_ids: Vec<String>,
        filters: serde_json::Value,
    ) -> Self {
        Self {
            tool_type: "file_search".to_string(),
            name: None,
            description: None,
            parameters: None,
            function: None,
            vector_store_ids: Some(vector_store_ids),
            container: None,
            partial_images: None,
            require_approval: None,
            server_label: None,
            server_url: None,
            headers: None,
            free_form: None,
            grammar: None,
            filters: Some(filters), // ✅ new
        }
    }
}
```

#### C) Keep the existing `file_search(...)` constructor as-is

No behavior change; it’ll serialize without `filters`.

---

# 2) (Optional but recommended) Provide a typed filter builder

**Why:** You can pass raw JSON, but a typed builder reduces mistakes. Keep it entirely optional so you aren’t locked into a server schema.

### File: `src/types/tools.rs` (or new `src/types/filters.rs` and re-export)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Filter {
    And { conditions: Vec<FilterCondition> },
    Or  { conditions: Vec<FilterCondition> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilterCondition {
    pub field: String,
    pub operator: String,            // "eq" | "in" | "contains_any" | "lte" | "gte" | ...
    pub value: serde_json::Value,    // string | number | array, etc.
}

impl Filter {
    pub fn and(conditions: Vec<FilterCondition>) -> Self {
        Self::And { conditions }
    }
    pub fn or(conditions: Vec<FilterCondition>) -> Self {
        Self::Or { conditions }
    }
}

impl FilterCondition {
    pub fn new(field: impl Into<String>, operator: impl Into<String>, value: serde_json::Value) -> Self {
        Self { field: field.into(), operator: operator.into(), value }
    }
}
```

Usage in client code:

```rust
use open_ai_rust_responses_by_sshift::types::{Tool, Filter, FilterCondition};
use serde_json::json;

let filters = Filter::and(vec![
    FilterCondition::new("tags", "contains_any", json!(["aptos", "validators"])),
    FilterCondition::new("valid_from", "lte", json!(now_unix)),
    FilterCondition::new("valid_to", "gte", json!(now_unix)),
]);

let tool = Tool::file_search_with_filters(vec![vs_id], serde_json::to_value(filters)?);
```

---

# 3) Add **List Files** (with attributes) to vector stores

**Why:** Quark wants to show users their files *with* tags. Your wrapper exposes `add_file`, `delete_file`, and `search`, but **no listing endpoint**. Add a read path so you can inspect attributes.

### File: `src/vector_stores/mod.rs`

#### A) Add response types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorStoreFile {
    pub id: String,
    pub filename: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>, // ✅ tags, tenant_id, validity, etc.

    // Keep extra fields flexible for forward compatibility
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
```

> You already have a `PaginatedList` and `PaginationParams`; reuse those.

#### B) Add the method

```rust
impl VectorStores {
    /// Lists files in a vector store (attributes included, if any).
    ///
    /// GET /vector_stores/{id}/files?limit&after
    pub async fn list_files(
        &self,
        vector_store_id: &str,
        params: Option<PaginationParams>,
    ) -> Result<PaginatedList<VectorStoreFile>> {
        let mut req = self
            .client
            .get(format!("{}/vector_stores/{}/files", self.base_url, vector_store_id));
        if let Some(p) = params {
            req = req.query(&p);
        }
        let response = req.send().await.map_err(crate::Error::Http)?;
        let response = try_parse_api_error(response).await?;
        response.json().await.map_err(crate::Error::Http)
    }
}
```

*(If your upstream doesn’t yet expose this exact route, keep this commit behind a feature flag, or document that it may 404 until generally available. Quark can still rely on its local sled view.)*

---

# 4) Keep **Add File with attributes** as-is (you already support it)

You already have:

```rust
pub struct AddFileToVectorStoreRequest {
    pub file_id: String,
    pub attributes: Option<serde_json::Value>,
}
```

No change needed. Quark will pass:

```rust
AddFileToVectorStoreRequest {
  file_id,
  attributes: Some(json!({
    "tags": ["aptos", "validators"],
    "tenant_id": format!("user_{user_id}"),
    "uploaded_at": now_unix,
    "valid_from": valid_from_opt,
    "valid_to": valid_to_opt
  })),
}
```

---

# 5) (Optional utility) Upsert attributes helper

OpenAI’s public surface doesn’t always expose “update attributes” for a file already in a store. A safe, wrapper‑level helper is:

```rust
impl VectorStores {
    /// Convenience: replace attributes by delete + re-add.
    pub async fn upsert_file_attributes(
        &self,
        vector_store_id: &str,
        file_id: &str,
        attributes: serde_json::Value,
    ) -> Result<()> {
        // delete, ignore 404
        let _ = self.delete_file(vector_store_id, file_id).await;
        // re-add with attributes
        let req = AddFileToVectorStoreRequest {
            file_id: file_id.to_string(),
            attributes: Some(attributes),
        };
        let _ = self.add_file(vector_store_id, req).await?;
        Ok(())
    }
}
```

This keeps business logic in the SDK so Quark code stays small.

---

# 6) Examples & tests

### A) New example: `examples/vector_store_tags.rs`

```rust
use open_ai_rust_responses_by_sshift::{
    Client, vector_stores::{AddFileToVectorStoreRequest, CreateVectorStoreRequest},
    types::{Tool, Filter, FilterCondition, Request, ToolChoice},
};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::from_env()?;

    // ensure a VS
    let vs = client.vector_stores.create(CreateVectorStoreRequest {
        name: "demo_tags".into(),
        file_ids: vec![], // create empty; we will add with attributes
    }).await?;

    // assume you already uploaded a file and have file_id
    let file_id = "file_123";
    client.vector_stores.add_file(&vs.id, AddFileToVectorStoreRequest {
        file_id: file_id.into(),
        attributes: Some(json!({"tags":["aptos","validators"], "valid_from": 1720560000, "valid_to": null})),
    }).await?;

    let filters = Filter::and(vec![
        FilterCondition::new("tags", "contains_any", json!(["aptos"])),
    ]);
    let tool = Tool::file_search_with_filters(vec![vs.id.clone()], serde_json::to_value(filters)?);

    let req = Request::builder()
        .model(open_ai_rust_responses_by_sshift::types::Model::GPT4oMini)
        .input("Summarize validator requirements.")
        .tools(vec![tool])
        .tool_choice(ToolChoice::auto())
        .build();

    let resp = client.responses.create(req).await?;
    println!("{:#?}", resp);
    Ok(())
}
```

### B) Unit test (serialization)

Create `src/tests.rs` (or extend):

```rust
#[test]
fn serializes_file_search_with_filters() {
    use crate::types::{Tool, Filter, FilterCondition};
    use serde_json::json;

    let filters = Filter::and(vec![
        FilterCondition::new("tags", "contains_any", json!(["aptos","validators"])),
    ]);
    let tool = Tool::file_search_with_filters(vec!["vs_abc".into()], serde_json::to_value(filters).unwrap());
    let s = serde_json::to_string(&tool).unwrap();
    assert!(s.contains("\"type\":\"file_search\""));
    assert!(s.contains("\"vector_store_ids\":[\"vs_abc\"]"));
    assert!(s.contains("\"filters\""));
}
```

---

# 7) Versioning & docs

- **`Cargo.toml`**: bump `version` (e.g., `0.5.0` → `0.6.0`).
- **`CHANGELOG.md`**:
  - Added: `Tool.filters` + `Tool::file_search_with_filters(...)`
  - Added: `VectorStores::list_files(...)` returning attributes
  - (Optional) Added: `VectorStores::upsert_file_attributes(...)`
- **`DOCUMENTATION.md`**: show a metadata‑filter example and note the “create empty + add with attributes” pattern.

---

##

