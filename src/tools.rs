use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Definition of a tool that Claude can use
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolDefinition {
    /// Name of the tool
    pub name: String,
    
    /// Description of the tool
    pub description: String,
    
    /// Input parameters schema
    pub input_schema: ToolParameters,
}

/// Parameters for a tool
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolParameters {
    /// Type of the tool parameters
    #[serde(rename = "type")]
    pub param_type: String,
    
    /// Properties of the tool parameters
    pub properties: HashMap<String, ParameterProperty>,
    
    /// Required parameters
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
}

/// Property of a parameter
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParameterProperty {
    /// Type of the parameter
    #[serde(rename = "type")]
    pub param_type: String,
    
    /// Description of the parameter
    pub description: String,
    
    /// Enum values (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
    
    /// Minimum value (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    
    /// Maximum value (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
}

/// Tool choice configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ToolChoice {
    /// Model decides whether to use tools
    #[serde(rename = "auto")]
    Auto,
    
    /// Force model to use a specific tool
    #[serde(rename = "tool")]
    Tool {
        /// Name of the tool to use
        name: String,
    },
    
    /// Force model to use any available tool
    #[serde(rename = "any")]
    Any,
    
    /// Force model not to use tools
    #[serde(rename = "none")]
    None,
}

impl ToolChoice {
    /// Create a new auto tool choice
    pub fn auto() -> Self {
        Self::Auto
    }
    
    /// Create a new tool-specific choice
    pub fn specific(name: impl Into<String>) -> Self {
        Self::Tool { name: name.into() }
    }
    
    /// Create a new any tool choice
    pub fn any() -> Self {
        Self::Any
    }
    
    /// Create a new none tool choice
    pub fn none() -> Self {
        Self::None
    }
}

/// Helper function to create a simple calculator tool definition
/// Evaluates a mathematical expression
pub fn evaluate_expression(expression: &str) -> Result<f64, String> {
    // This is a very simple evaluator that handles basic operations
    // In a real implementation, you'd use a proper expression parser
    // For now, we'll just check if it's a simple number
    expression.parse::<f64>().map_err(|_| {
        format!("Failed to evaluate expression: '{}'. Only simple numeric values are supported in this implementation.", expression)
    })
}

/// Helper function to create a simple calculator tool definition
pub fn calculator_tool() -> ToolDefinition {
    ToolDefinition {
        name: "calculator".to_string(),
        description: "Evaluates mathematical expressions".to_string(),
        input_schema: ToolParameters {
            param_type: "object".to_string(),
            properties: {
                let mut map = HashMap::new();
                map.insert(
                    "expression".to_string(),
                    ParameterProperty {
                        param_type: "string".to_string(),
                        description: "The mathematical expression to evaluate".to_string(),
                        enum_values: None,
                        minimum: None,
                        maximum: None,
                    },
                );
                map
            },
            required: vec!["expression".to_string()],
        },
    }
}
