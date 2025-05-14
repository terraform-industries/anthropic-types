# Anthropic Types

A Rust library containing type definitions for communicating with the Anthropic API.

## Purpose

This library provides a standardized set of types for working with the Anthropic API in Theater actors. It defines the structures needed for:

- Sending requests to the Anthropic API
- Parsing responses from the Anthropic API
- Working with messages, models, and tools

### Basic Example

```rust
use anthropic_types::{
    AnthropicRequest, CompletionRequest, Message, OperationType,
};

// Create a simple chat message
let message = Message::new_text("user", "Hello, Claude!");

// Build a completion request
let request = AnthropicRequest {
    version: "1.0".to_string(),
    operation_type: OperationType::ChatCompletion,
    request_id: "req-12345".to_string(),
    completion_request: Some(CompletionRequest {
        model: "claude-3-7-sonnet-20250219".to_string(),
        messages: vec![message],
        max_tokens: Some(1024),
        temperature: Some(0.7),
        system: Some("You are a helpful assistant.".to_string()),
        top_p: None,
        anthropic_version: None,
        tools: None,
        tool_choice: None,
        disable_parallel_tool_use: None,
        additional_params: None,
    }),
    params: None,
};

// Serialize to JSON
let json = serde_json::to_string(&request).unwrap();
```

### Using With Claude-Chat Actor

In your claude-chat actor, you can now use these types to communicate with the anthropic-proxy:

```rust
fn send_to_anthropic(
    proxy_id: &str,
    conversation_id: &str,
    messages: &[ChatMessage],
    system: Option<String>,
) -> Result<ChatMessage, String> {
    // Convert our ChatMessage format to the AnthropicMessage format
    let anthropic_messages: Vec<anthropic_types::Message> = messages
        .iter()
        .map(|msg| anthropic_types::Message::new_text(
            &msg.role,
            &msg.content
        ))
        .collect();

    // Build a properly structured request
    let req = anthropic_types::AnthropicRequest {
        version: "1.0".to_string(),
        operation_type: anthropic_types::OperationType::ChatCompletion,
        request_id: format!("req-{}", conversation_id),
        completion_request: Some(anthropic_types::CompletionRequest {
            model: "claude-3-7-sonnet-20250219".to_string(),
            messages: anthropic_messages,
            max_tokens: Some(1024),
            temperature: Some(0.7),
            system: system,
            top_p: None,
            anthropic_version: None,
            tools: None,
            tool_choice: None,
            disable_parallel_tool_use: None,
            additional_params: None,
        }),
        params: None,
    };

    // Serialize and send the request
    // ...
}
```

## Features

- **Message Types**: Definitions for messages and content blocks
- **Model Information**: Information about Claude models, including context limits and pricing
- **Tool Support**: Definitions for tools, tool choice, and parameters
- **Error Handling**: Comprehensive error types for Anthropic API interactions

## Structure

- `messages.rs`: Message types, requests, and responses
- `models.rs`: Model information and pricing
- `tools.rs`: Tool definitions and parameters
- `errors.rs`: Error types for Anthropic API operations

## License

MIT
