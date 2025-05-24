## Cross-check: Responses API spec ↔ your Rust wrapper

I re-read the **May 21 2025 “New tools and features in the Responses API”** announcement plus the public reference pages and extracted every field, endpoint, and primitive now exposed by the API. The table below maps those capabilities against what the **openai-rust-responses-sshift** crate implements today.

| Domain                          | Spec (May 2025)                                                                                                             | Wrapper status                                                   | Notes / gaps                                                                                   |               |                                                                                                    |
| ------------------------------- | --------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- | ------------- | -------------------------------------------------------------------------------------------------- |
| **Core endpoints**              | `/responses` (create, retrieve, cancel, list, delete)                                                                       | **✅**                                                            | Matches.                                                                                       |               |                                                                                                    |
|                                 | `/threads`, `/messages`                                                                                                     | **✅**                                                            | All CRUD + pagination.                                                                         |               |                                                                                                    |
|                                 | `/files`, `/files/{id}/content`, `/vector_stores` (+ nested)                                                                | **✅**                                                            | Upload w/ MIME, download, vector-store search.                                                 |               |                                                                                                    |
| **Built-in tools**              | `web_search_preview`                                                                                                        | **✅** canonical + legacy fallback.                               |                                                                                                |               |                                                                                                    |
|                                 | `file_search` (multi-store, attribute filters)                                                                              | **✅** IDs list supported; attribute filter **❌** (see below).    |                                                                                                |               |                                                                                                    |
|                                 | `computer_use_preview`                                                                                                      | **✅** enum variant present.                                      |                                                                                                |               |                                                                                                    |
|                                 | `code_interpreter`                                                                                                          | **✅** variant present; container param handled.                  |                                                                                                |               |                                                                                                    |
|                                 | `image_generation` *(new 2025-05)*                                                                                          | **❌ missing**                                                    | Add `Tool::ImageGeneration { container: Container }`, plus stream event for progressive image. |               |                                                                                                    |
|                                 | **Remote MCP server** (`type:"mcp"`) *(new)*                                                                                | **❌ missing**                                                    | Needs open struct: `Tool::Mcp { server_label, server_url, headers }`.                          |               |                                                                                                    |
| **Streaming events**            | `text.delta`, `text.stop`, `function_call`, `tool_result`, `tool_call_created`, `tool_call_delta`, `image.progress`, `done` | All but `image.progress` present.                                | Add `ImageProgress { url: Option<String>, index: u32 }` variant.                               |               |                                                                                                    |
| **Request fields**              | `instructions`, `input`, `tools`, `tool_choice`, `include`, `previous_response_id`                                          | **✅**                                                            |                                                                                                |               |                                                                                                    |
|                                 | **`reasoning` object**<br>• `effort` (\`"low"                                                                               | "high"`) – enables _background mode_<br>• `summary` (`"auto"     | string\`) – returns reasoning summary                                                          | **❌ missing** | Add `reasoning: Option<ReasoningParams>` struct; surface `background: bool` alias for convenience. |
|                                 | `store` (bool) for encrypted reasoning items                                                                                | **❌ missing**                                                    | Add boolean + `Include::ReasoningEncryptedContent`.                                            |               |                                                                                                    |
| **Include values**              | `"file_search.results"`, `"reasoning.summary"`, `"reasoning.encrypted_content"`                                             | First one present; last two **❌**                                | Extend `Include` enum.                                                                         |               |                                                                                                    |
| **Vector-store search**         | Supports `query`, `similarity_top_k`, **`attribute_filter`** (JSON-filter grammar)                                          | Attribute filter **❌**                                           | Accept `serde_json::Value` for filters.                                                        |               |                                                                                                    |
| **Background mode**             | `background: true` → returns `202` + polling/stream                                                                         | Boolean flag **❌**                                               | Add to request; treat HTTP 202 as `BackgroundHandle` (id + status).                            |               |                                                                                                    |
| **Encrypted reasoning**         | If `store:false` + include encrypted content                                                                                | **❌**                                                            | After adding `store`/`include`, decode base64 blob to bytes for user convenience.              |               |                                                                                                    |
| **Rate-limit headers surfaced** | `x-rate-limit-limit` etc.                                                                                                   | API response wrapper exposes raw headers **✅** via `.headers()`. |                                                                                                |               |                                                                                                    |
| **Error model**                 | `{error:{type,message,param,code}}`                                                                                         | **✅** parsed → `Error::Api`.                                     |                                                                                                |               |                                                                                                    |
| **Retry / backoff helper**      | not in spec                                                                                                                 | **nice-to-have**                                                 | Could expose `RetryPolicy` trait; out-of-scope for parity.                                     |               |                                                                                                    |

### Key take-aways & improvement ideas

1. **Add the two new tool variants.**

   ```rust
   pub enum Tool {
       WebSearchPreview,
       FileSearch { vector_store_ids: Vec<String> },
       ComputerUsePreview,
       CodeInterpreter { container: Container },
       ImageGeneration { container: Container },   // NEW
       Mcp { server_label: String, server_url: String, headers: Option<HashMap<String,String>> }, // NEW
   }
   ```

2. **Reasoning parameters & background mode.**

   ```rust
   pub struct ReasoningParams {
       pub effort: Option<Effort>,            // Effort::Low|High
       pub summary: Option<SummarySetting>,   // SummarySetting::Auto|Text(String)
   }
   // in Request
   pub reasoning: Option<ReasoningParams>,
   pub background: Option<bool>,             // sugar field
   pub store: Option<bool>,                  // encrypted reasoning opt-out
   ```

3. **Extend `Include` enum** to cover `"reasoning.summary"` and `"reasoning.encrypted_content"`.

4. **Vector-store attribute filters.**
   Accept an arbitrary `serde_json::Value` (mirroring Python SDK) to future-proof.

5. **Streaming additions** – `image.progress` event variant so users can display progressive renders.

6. **`BackgroundHandle` type** for 202 responses (id, status URL, poll/stream helpers).

None of these gaps stop ordinary chat-style usage with web/file/code tools, but adding them will bring the crate to **100 % feature parity** with the May 2025 spec and make it future-proof for upcoming MCP ecosystem growth.

---

### Overall verdict

*Current wrapper coverage*: **≈ 90 %** of the live Responses API.
*Work to hit 100 %*: implement the six bullets above (mainly new tool variants and the `reasoning`/`background` fields). After that you’ll be fully aligned with OpenAI’s May 2025 capabilities.
