use std::error::Error;
use std::fmt;

/// Error type for Anthropic API operations
#[derive(Debug)]
pub enum AnthropicError {
    /// HTTP request failed
    HttpError(String),

    /// Failed to serialize/deserialize JSON
    JsonError(String),

    /// API returned an error
    ApiError { status: u16, message: String },

    /// Unexpected response format
    InvalidResponse(String),

    /// Rate limit exceeded
    RateLimitExceeded { retry_after: Option<u64> },

    /// Authentication error
    AuthenticationError(String),
}

impl fmt::Display for AnthropicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnthropicError::HttpError(msg) => write!(f, "HTTP error: {}", msg),
            AnthropicError::JsonError(msg) => write!(f, "JSON error: {}", msg),
            AnthropicError::ApiError { status, message } => {
                write!(f, "API error ({}): {}", status, message)
            }
            AnthropicError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
            AnthropicError::RateLimitExceeded { retry_after } => {
                if let Some(seconds) = retry_after {
                    write!(f, "Rate limit exceeded. Retry after {} seconds", seconds)
                } else {
                    write!(f, "Rate limit exceeded")
                }
            }
            AnthropicError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
        }
    }
}

impl Error for AnthropicError {}

impl From<serde_json::Error> for AnthropicError {
    fn from(error: serde_json::Error) -> Self {
        AnthropicError::JsonError(error.to_string())
    }
}
