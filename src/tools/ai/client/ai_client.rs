use async_trait::async_trait;
use shuru::tools::ai::plan::AIPlan;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AIClientError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Failed to parse response: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("API error (status {status}): {message}")]
    APIError { status: u16, message: String },

    #[error("Rate limit exceeded, retry after {retry_after} seconds")]
    RateLimit { retry_after: u64 },

    #[error("Invalid API key")]
    InvalidAPIKey,

    #[error("Model not available: {0}")]
    ModelNotAvailable(String),

    #[error("Invalid prompt: {0}")]
    InvalidPrompt(String),

    #[error("Response validation failed: {0}")]
    ValidationError(String),
}

pub type Result<T> = std::result::Result<T, AIClientError>;

#[async_trait]
pub trait AIClient {
    async fn generate_plan(&self, user_prompt: &str) -> Result<AIPlan>;
}
