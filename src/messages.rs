use crate::models::ModelInfo;
use crate::tool_choice::ToolChoice;
use mcp_protocol::tool::{Tool, ToolContent};
use serde::{Deserialize, Serialize};

/// Cache control configuration for system messages
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheControl {
    #[serde(rename = "type")]
    pub cache_type: String,
}

/// A single system message with optional cache control
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,
}

/// Different types of system messages that can be provided
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SystemMessageFormat {
    /// Simple string format
    String(String),
    /// Array of structured system messages
    Array(Vec<SystemMessage>),
}

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

    #[serde(rename = "document")]
    Document {
        source: DocumentSource,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        citations: Option<DocumentCitations>,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum DocumentSource {
    #[serde(rename = "text")]
    Text { media_type: String, data: String },

    #[serde(rename = "base64")]
    Base64 { media_type: String, data: String },

    #[serde(rename = "content")]
    Custom { content: Vec<ChunkedText> },

    #[serde(rename = "url")]
    Url { url: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChunkedText {
    #[serde(rename = "type")]
    pub r#type: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocumentCitations {
    pub enabled: bool,
}

/// A single message in a conversation with Claude
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    /// Role of the message sender (user, assistant, system)
    pub role: String,

    /// Content of the message - can be a string or vector of MessageContent objects
    pub content: MessageContentFormat,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MessageContentFormat {
    /// Simple string format
    String(String),
    /// Structured content format
    Structured(Vec<MessageContent>),
}

impl Message {
    /// Create a new message with structured content
    pub fn new_structured(role: impl Into<String>, content: Vec<MessageContent>) -> Self {
        Self {
            role: role.into(),
            content: MessageContentFormat::Structured(content),
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

    /// System prompt to use (can be a string or array of structured messages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemMessageFormat>,

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_completion_request_with_system_messages() {
        let json = r#"{
            "model": "claude-3-7-sonnet-20250219",
            "max_tokens": 1024,
            "system": [
              {
                "type": "text",
                "text": "You are an AI assistant tasked with analyzing literary works. Your goal is to provide insightful commentary on themes, characters, and writing style.\n"
              },
              {
                "type": "text",
                "text": "<the entire contents of Pride and Prejudice>",
                "cache_control": {"type": "ephemeral"}
              }
            ],
            "messages": [
              {
                "role": "user",
                "content": "Analyze the major themes in Pride and Prejudice."
              }
            ]
          }"#;

        serde_json::from_str::<CompletionRequest>(json).expect("Failed to deserialize request");
    }

    #[test]
    fn test_deserialize_completion_request_with_system_message_string() {
        let json = r#"{
            "model": "claude-3-7-sonnet-20250219",
            "max_tokens": 1024,
            "system": "You are an AI assistant tasked with analyzing literary works. Your goal is to provide insightful commentary on themes, characters, and writing style.",
            "messages": [
              {
                "role": "user",
                "content": "Analyze the major themes in Pride and Prejudice."
              }
            ]
        }"#;

        serde_json::from_str::<CompletionRequest>(json).expect("Failed to deserialize request");
    }

    #[test]
    fn test_deserialize_completion_request_with_document() {
        let json = r#"{
    "model": "claude-3-7-sonnet-20250219",
    "max_tokens": 1024,
    "messages": [
      {
        "role": "user",
        "content": [
          {
            "type": "document",
            "source": {
              "type": "text",
              "media_type": "text/plain",
              "data": "The grass is green. The sky is blue."
            },
            "title": "My Document",
            "context": "This is a trustworthy document.",
            "citations": {"enabled": true}
          },
          {
            "type": "text",
            "text": "What color is the grass and sky?"
          }
        ]
      }
    ]
  }"#;

        serde_json::from_str::<CompletionRequest>(json).expect("Failed to deserialize request");
    }

    #[test]
    fn test_deserialize_message_content_with_document_source_custom() {
        let json = r#"{
    "type": "document",
    "source": {
        "type": "content",
        "content": [
            {"type": "text", "text": "First chunk"},
            {"type": "text", "text": "Second chunk"}
        ]
    },
    "title": "Document Title",
    "context": "Context about the document that will not be cited from",
    "citations": {"enabled": true}
}"#;

        serde_json::from_str::<MessageContent>(json).expect("Failed to deserialize request");
    }

    #[test]
    fn test_deserialize_message_content_with_document_source_text() {
        let json = r#"{
    "type": "document",
    "source": {
        "type": "text",
        "media_type": "text/plain",
        "data": "Plain text content..."
    },
    "title": "Document Title",
    "context": "Context about the document that will not be cited from",
    "citations": {"enabled": true}
}"#;

        serde_json::from_str::<MessageContent>(json).expect("Failed to deserialize request");
    }

    #[test]
    fn test_deserialize_message_content_with_document_source_base64() {
        let json = r#"{
    "type": "document",
    "source": {
        "type": "base64",
        "media_type": "application/pdf",
        "data": "base64-encoded-pdf-content"
    },
    "title": "Document Title",
    "context": "Context about the document that will not be cited from",
    "citations": {"enabled": true}
}"#;

        serde_json::from_str::<MessageContent>(json).expect("Failed to deserialize request");
    }
}
