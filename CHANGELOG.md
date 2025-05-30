# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.7] - 2025-01-XX (Current Development)

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
  - All examples now use optimized model configurations

### 🔧 Code Quality
- **Clippy Fixes**: Resolved duplicate match arms warning
  - Merged `file_search.results` and `file_search_call.results` patterns
- **Formatting**: Applied `cargo fmt` across codebase
- **Error Handling**: Enhanced error messages and documentation

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