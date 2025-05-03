// Anthropic API Types Library
//
// This crate provides type definitions for interacting with the Anthropic API
// and is intended to be used by Theater actors that need to communicate with Claude.

pub mod errors;
pub mod messages;
pub mod models;
pub mod tool_choice;

// Re-export main types for convenience
pub use errors::AnthropicError;
pub use messages::{
    AnthropicRequest, AnthropicResponse, CompletionRequest, CompletionResponse, Message,
    MessageContent, ResponseStatus, Usage,
};
pub use models::{ModelInfo, ModelPricing};
pub use tool_choice::ToolChoice;
