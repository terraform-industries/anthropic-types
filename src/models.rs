use serde::{Deserialize, Serialize};

/// Information about a model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelInfo {
    /// Model ID
    pub id: String,

    /// Display name
    pub display_name: String,

    /// Maximum context window size
    pub max_tokens: u32,

    /// Provider name
    pub provider: String,

    /// Optional pricing information
    pub pricing: Option<ModelPricing>,
}

/// Pricing information for a model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelPricing {
    /// Cost per million input tokens
    pub input_cost_per_million_tokens: f64,

    /// Cost per million output tokens
    pub output_cost_per_million_tokens: f64,
}

impl ModelInfo {
    /// Get maximum tokens for a given model ID
    pub fn get_max_tokens(model_id: &str) -> u32 {
        match model_id {
            // Claude 3.7 models
            "claude-3-7-sonnet-20250219" => 200000,

            // Claude 3.5 models
            "claude-3-5-sonnet-20241022"
            | "claude-3-5-haiku-20241022"
            | "claude-3-5-sonnet-20240620" => 200000,

            // Claude 3 models
            "claude-3-opus-20240229" => 200000,
            "claude-3-sonnet-20240229" => 200000,
            "claude-3-haiku-20240307" => 200000,

            // Claude 2 models
            "claude-2.1" | "claude-2.0" => 100000,

            // Default case
            _ => 100000, // Conservative default
        }
    }

    /// Get pricing information for a given model ID
    pub fn get_pricing(model_id: &str) -> ModelPricing {
        match model_id {
            // Claude 3.7 models
            "claude-3-7-sonnet-20250219" => ModelPricing {
                input_cost_per_million_tokens: 3.00,
                output_cost_per_million_tokens: 15.00,
            },

            // Claude 3.5 models
            "claude-3-5-sonnet-20241022" | "claude-3-5-sonnet-20240620" => ModelPricing {
                input_cost_per_million_tokens: 3.00,
                output_cost_per_million_tokens: 15.00,
            },
            "claude-3-5-haiku-20241022" => ModelPricing {
                input_cost_per_million_tokens: 0.80,
                output_cost_per_million_tokens: 4.00,
            },

            // Claude 3 models
            "claude-3-opus-20240229" => ModelPricing {
                input_cost_per_million_tokens: 15.00,
                output_cost_per_million_tokens: 75.00,
            },
            "claude-3-haiku-20240307" => ModelPricing {
                input_cost_per_million_tokens: 0.25,
                output_cost_per_million_tokens: 1.25,
            },
            "claude-3-sonnet-20240229" => ModelPricing {
                input_cost_per_million_tokens: 3.00,
                output_cost_per_million_tokens: 15.00,
            },

            // Default for older or unknown models
            _ => ModelPricing {
                input_cost_per_million_tokens: 8.00,
                output_cost_per_million_tokens: 24.00,
            },
        }
    }
}
