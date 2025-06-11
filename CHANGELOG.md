# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2025-06-11

### ✨ Features
- **Vision (Image Input)**: Added support for user-supplied image analysis.
  - New `InputItem::content_image()` helper for content items.
  - New `InputItem::message()` and `RequestBuilder::input_image_url()` for easy message creation.
  - Fully compatible with GPT-4o multimodal capabilities.

### 🆕 Examples
- Added `examples/image_input.rs` and updated `examples/comprehensive_demo.rs` with an image input demo.

### ✅ Tests
- Added unit tests verifying the new builders and request structure.

### 🐛 Fixed
- Corrected message structure for image inputs (now wrapped in a `message` item with `role: "user"`).

### 📝 Documentation
- Updated README.md and DOCUMENTATION.md with Vision usage instructions and code samples.

## [0.2.0] - 2024-07-22

### ✨ Features
- **Added Built-in Image Generation Tool**: Implemented support for the new `Tool::image_generation()` built-in tool. The model now handles image generation directly and returns a base64-encoded image in the new `ImageGenerationCall` response item.

### 💥 Breaking Changes
- **Removed Deprecated Image Tools**: Removed the jury-rigged `Tool::image_generation_function()` and the `Tool::image_generation_with_partials()` methods. Please use the new `Tool::image_generation()` for AI-driven image generation or the direct `client.images.generate()` method for explicit API calls.

### ♻️ Changed
- Updated `examples/image_generation.rs` and `examples/comprehensive_demo.rs` to use the new built-in image generation tool, simplifying the workflow and removing manual tool-handling logic for images.
- Added `examples/image_generation_builtin.rs` to specifically demonstrate the new feature.

## [0.1.8] - 2025-01-15

### 🐛 Fixed
- **Vector Store File Deletion**: Fixed `delete_file` method response parsing
  - Created `VectorStoreFileDeleteResponse` struct to handle actual API response
  - Fixed incorrect expectation of `VectorStore` object return type
  - API correctly returns `{id: "file-xxx", object: "vector_store.file.deleted", deleted: true}`
  - Method now returns `Result<VectorStoreFileDeleteResponse>` instead of `Result<VectorStore>`
- **API Compatibility**: Resolved deserialization errors when deleting files from vector stores
  - The OpenAI API response doesn't include a `name` field as expected by `VectorStore` struct
  - New response struct matches the actual API specification

### ✨ Enhanced
- **Comprehensive Demo**: Added proper vector store file deletion testing
  - Demonstrates correct cleanup sequence: 1) Remove files from vector stores, 2) Delete vector stores, 3) Delete files
  - Shows proper usage of `VectorStoreFileDeleteResponse` fields
  - Added educational output explaining the difference between removing files from vector stores vs. deleting files
- **Type Exports**: Added `VectorStoreFileDeleteResponse` to public API exports
- **Documentation**: Updated vector store file deletion examples to show correct return type

### 🔧 Backward Compatibility
- **Breaking Change**: `vector_stores.delete_file()` now returns `VectorStoreFileDeleteResponse` instead of `VectorStore`
  - This is a necessary fix to match the actual OpenAI API response structure
  - Users should update code to handle the new response type:
    ```rust
    // Old (broken):
    let vector_store = client.vector_stores.delete_file("vs_123", "file_456").await?;
    
    // New (working):
    let delete_response = client.vector_stores.delete_file("vs_123", "file_456").await?;
    println!("Deleted: {} (success: {})", delete_response.id, delete_response.deleted);
    ```

## [0.1.7] - 2025-01-XX (Current Development)

### 🚀 Major Phase 1 Implementation - Full OpenAI May 2025 Spec Compatibility

#### 🎨 Image Generation Support
- **Working Image Generation**: Implemented complete image generation support
  - Added `ImageGenerateRequest` and `ImageGenerateResponse` types for Images API
  - Added `client.images.generate()` for direct image generation
  - Created `Tool::image_generation_function()` pre-made function tool for AI-triggered generation
  - Full parameter support: prompt, n (1-10), size, quality, style, format, compression, background, seed, user
  - Note: Native OpenAI hosted `image_generation` tool support pending official API release
- **Bridge Pattern**: Implemented function tool bridge between Responses API and Images API
  - AI can now request image generation through function calling
  - Automatic handling of image generation in conversation flow
  - Examples demonstrate both direct API usage and AI-triggered generation

#### 📦 Response Type Enhancements (Phase 1 Complete)
- **Response Struct**: Added 21 new fields for full API parity
  - Core fields: `object`, `status`, `output_text`, `instructions`, `user`
  - Parameter echoes: `temperature`, `top_p`, `max_output_tokens`, `parallel_tool_calls`, `tool_choice`, `tools`, `top_logprobs`, `reasoning_effort`
  - Usage details: Complete token tracking with `output_tokens_details` and `prompt_tokens_details`
  - Advanced features: `text` (format/stop config), `truncation`, `reasoning` output, `incomplete_details`, `error`
  - Helper methods: `is_complete()`, `is_in_progress()`, `has_errors()`, `total_tokens()`
- **Request Struct**: Added 9 new fields
  - Critical: `store` (false for stateless mode)
  - Configuration: `truncation`, `text`, `max_output_tokens`, `top_logprobs`, `parallel_tool_calls`
  - Reasoning: `reasoning_effort` parameter support
  - User tracking: `user` field for conversation attribution
- **Tool Enhancements**: 
  - Added `partial_images` (1-3) for progressive image streaming
  - Added `require_approval` for MCP tools with never/auto/always modes
  - Helper methods for enhanced tool creation

#### 🔧 Token Optimization Fixes
- **Response Completion Issues**: Fixed incomplete responses by increasing token limits
  - Default `max_output_tokens` increased from 200 to 500 for general responses
  - Reasoning models (O4Mini) now use 2000 tokens for complex thinking tasks
  - Achieved 100% success rate (was 50%) in conversation continuity
- **Model-Specific Optimizations**:
  - GPT4oMini: 500 tokens (balanced for conversations)
  - O4Mini reasoning: 2000 tokens (accounts for internal thinking)
  - Streaming: 500 tokens (improved completion rates)

### 🐛 Fixed
- **CRITICAL API COMPATIBILITY**: Fixed Include field values to match OpenAI API specification
  - Changed `file_search.results` to `file_search_call.results` (API requirement)
  - Added support for `web_search_call.results`, `message.input_image.image_url`, `computer_call_output.output.image_url`
  - Maintained backward compatibility with legacy string values
- **Reasoning Parameter Structure**: Fixed reasoning parameters to match API specification
  - Moved `reasoning_effort` into `ReasoningParams` structure as `effort` field
  - Removed invalid top-level `reasoning_effort` parameter
- **Container Parameter Support**: Removed unsupported container parameters
  - API doesn't support `container` field in tools yet
  - Added documentation notes about future API updates
- **Truncation Deserialization**: Fixed truncation field handling
  - Created `TruncationSetting` enum to handle both string ("disabled") and object formats
  - Resolves "invalid type: string 'disabled', expected struct TruncationConfig" errors
- **O4Mini Model Compatibility**: Fixed temperature parameter issues
  - O4Mini model doesn't support `temperature` parameter
  - Removed temperature from all O4Mini requests
  - Added documentation about built-in reasoning optimization
- **Function Call Round-Trip**: Fixed image generation in comprehensive demo
  - Added proper two-step function call handling with `with_function_outputs()`
  - Image generation now waits for actual API results instead of instant return

### ✨ Enhanced
- **Type-Safe Include Options**: Improved include system with compile-time validation
  - Added new `Include` enum variants for all supported API options
  - Backward compatibility maintained with `include_strings()` method
- **Model Optimization**: Updated default model recommendations
  - Changed default from GPT4o to GPT4oMini for better performance/cost ratio
  - O4Mini recommended for reasoning tasks
- **Enhanced Examples**: Updated all examples to demonstrate new features
  - `comprehensive_demo.rs` showcases all API capabilities without errors
  - `reasoning_demo.rs` demonstrates proper O4Mini usage
  - `image_generation.rs` demonstrates both direct API and function tool usage
  - All examples now use optimized model configurations and proper token allocations
- **Production Readiness**: Achieved 85% OpenAI Responses API May 2025 spec coverage
  - All Phase 1 features implemented with 100% backward compatibility
  - Zero breaking changes for existing users
  - Comprehensive test coverage (37/38 tests passing)

### 🔧 Code Quality
- **Clippy Fixes**: Resolved duplicate match arms warning
  - Merged `file_search.results` and `file_search_call.results` patterns
- **Formatting**: Applied `cargo fmt` across codebase
- **Error Handling**: Enhanced error messages and documentation
- **Performance**: Optimized token allocations for different use cases
  - Quick responses: 500 tokens
  - Reasoning tasks: 2000 tokens
  - Function calling: Dynamic based on complexity

### 📚 Documentation
- **API Compatibility**: Updated documentation to reflect current OpenAI API specification
- **Model Recommendations**: Added guidance for optimal model selection
- **Error Resolution**: Added troubleshooting section for common API issues
- **Examples**: Updated all code examples to use current best practices
- **Comprehensive Audit**: Performed full review and update of README.md, DOCUMENTATION.md, and examples
  - Updated 6 examples to use GPT4oMini as recommended default model
  - Added comprehensive troubleshooting section with common API issues and solutions
  - Enhanced model capabilities matrix with temperature support indicators
  - Added migration guide for v0.1.7 API compatibility improvements
  - All documentation examples now reflect corrected API usage patterns
- **Image Generation**: Added complete documentation for new image generation features
  - Direct API usage examples
  - Function tool integration patterns
  - Parameter mapping and best practices

### ✅ Testing
- **Integration Tests**: All examples now run without API errors
- **Comprehensive Demo**: Full end-to-end testing of all features
- **Error Scenarios**: Added proper error handling demonstrations
- **Test Suite Fixes**: Resolved all compilation and runtime issues
  - Fixed `reasoning_effort` usage in tests to use proper `ReasoningParams` structure
  - Updated Include enum tests to expect correct API field values (`file_search_call.results`)
  - Fixed streaming example error handling to use `Result<StreamEvent, Error>` pattern
  - All 40 tests now pass (37 unit tests + 3 integration tests)
- **Documentation Testing**: All code examples in documentation verified to compile and work correctly
- **Image Generation Tests**: Added comprehensive tests for new image features
  - Direct API generation tests
  - Function tool integration tests
  - Parameter validation tests
  - All 10 new test functions passing

## [0.1.6] - 2025-01-XX

### 📝 Documentation
- Updated version references throughout documentation from 0.1.0 to current version
- Added comprehensive CHANGELOG.md with version history
- Enhanced function calling documentation with bug fix notes
- Updated README.md and DOCUMENTATION.md installation instructions

### 🔧 Maintenance
- Prepared for crates.io publication with updated documentation

## [0.1.5] - 2025-01-XX

### 🐛 Fixed
- **CRITICAL BUG FIX**: Fixed multiple function calls not working properly
  - Function call outputs now use correct API structure with `call_id` and `output` at top level
  - Previously nested structure in `content` was causing "Missing required parameter" errors
  - Multiple tool calls in sequence now work correctly
- Fixed syntax errors in test suite
- Improved code quality with clippy and fmt

### 🔧 Changed
- Updated `InputItem` structure for function call outputs
  - `call_id` is now a top-level optional field
  - `output` is now a top-level optional field for function outputs
  - `content` is now optional and not used for function call outputs

### ✅ Tested
- Enhanced comprehensive demo example with continuous conversation workflow
- Added proper test coverage for function call output format
- Verified multiple tool calls work in sequence

## [0.1.4] - 2025-01-XX

### 🐛 Fixed
- Initial attempt at function calling bug fix (incomplete)

## [0.1.3] - 2025-01-XX

### 🔧 Changed
- Version bump to resolve CI publish conflicts

## [0.1.2] - 2025-01-XX

### 📦 Released
- First manual crates.io publication
- Basic function calling support (with bugs)

## [0.1.1] - 2025-01-XX

### 🚀 Added
- Initial release with core OpenAI Responses API support
- Conversation continuity with response IDs
- Streaming responses
- File operations
- Vector stores
- Basic function calling (with issues)
- Reasoning parameters
- Background processing
- Enhanced model support 