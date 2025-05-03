use crate::models::ModelInfo;
use crate::tool_choice::ToolChoice;
use mcp_protocol::tool::{Tool, ToolContent};
use serde::{Deserialize, Serialize};

/// Different types of content that can be in a message
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum MessageContent {
    #[serde(rename = "text")]
    Text { text: String },

    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },

    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        content: Vec<ToolContent>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_error: Option<bool>,
    },
}

/// A single message in a conversation with Claude
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    /// Role of the message sender (user, assistant, system)
    pub role: String,

    /// Content of the message as vector of MessageContent objects
    pub content: Vec<MessageContent>,
}

impl Message {
    /// Create a new message with structured content
    pub fn new_structured(role: impl Into<String>, content: Vec<MessageContent>) -> Self {
        Self {
            role: role.into(),
            content,
        }
    }
}

/// Request to generate a completion from Claude
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionRequest {
    /// The Claude model to use
    pub model: String,

    /// List of messages in the conversation
    pub messages: Vec<Message>,

    /// Maximum number of tokens to generate
    pub max_tokens: u32,

    /// Temperature parameter (0.0 to 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// System prompt to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// Tools to make available to Claude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    /// Tool choice configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// Whether to disable parallel tool use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_parallel_tool_use: Option<bool>,
}

/// Information about token usage
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    pub input_tokens: u32,

    pub output_tokens: u32,

    pub cache_read_input_tokens: Option<u32>,

    pub cache_creation_input_tokens: Option<u32>,
}

/// Response from a completion request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionResponse {
    /// Generated content blocks
    pub content: Vec<MessageContent>,

    /// ID of the message
    pub id: String,

    /// Model used for generation
    pub model: String,

    // always "assistant"
    pub role: String,

    /// Reason why generation stopped
    /// can be "end_turn", "max_tokens", "stop_sequence", "tool_use", null
    pub stop_reason: StopReason,

    /// Stop sequence if applicable (deprecated - kept for backward compatibility)
    pub stop_sequence: Option<String>,

    /// Message type
    #[serde(rename = "type")]
    pub message_type: String,

    /// Token usage information
    pub usage: Usage,
}

/// Reason why generation stopped
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StopReason {
    /// Generation stopped because the end of a turn was reached
    #[serde(rename = "end_turn")]
    EndTurn,

    /// Generation stopped because the maximum token limit was reached
    #[serde(rename = "max_tokens")]
    MaxTokens,

    /// Generation stopped because a stop sequence was encountered
    #[serde(rename = "stop_sequence")]
    StopSequence,

    /// Generation stopped because a tool was used
    #[serde(rename = "tool_use")]
    ToolUse,
}

/// Request format for the anthropic-proxy actor
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AnthropicRequest {
    ListModels,

    GenerateCompletion { request: CompletionRequest },
}

/// Response status
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResponseStatus {
    /// Operation succeeded
    #[serde(rename = "Success")]
    Success,

    /// Operation failed
    #[serde(rename = "Error")]
    Error,
}

/// Response format from the anthropic-proxy actor
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AnthropicResponse {
    /// List of available models
    ListModels { models: Vec<ModelInfo> },

    /// Generated completion
    Completion { completion: CompletionResponse },

    /// Error response
    Error { error: String },
}
