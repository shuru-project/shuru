use async_trait::async_trait;
use reqwest;
use serde::Deserialize;
use shuru_core::global_config::ProviderConfig;

use shuru_ai::{
    client::ai_client::{AIClientError, Result},
    client::AIClient,
    context::Context,
    plan::AIPlan,
};

#[derive(Debug, Deserialize)]
struct OpenAIError {
    error: OpenAIErrorDetails,
}

#[derive(Debug, Deserialize)]
struct OpenAIErrorDetails {
    message: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
}

pub struct OpenAIClient {
    api_key: String,
    client: reqwest::Client,
    model: String,
    max_tokens: u32,
    temperature: f32,
}

impl OpenAIClient {
    /// Creates a new client with default settings for OpenAI.
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            model: "gpt-4".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
        }
    }

    /// Creates a new client with custom provider and model configurations.
    pub fn new_with_config(provider: &ProviderConfig) -> Self {
        Self {
            api_key: provider.api_key.clone(),
            client: reqwest::Client::new(),
            model: provider.model.clone(),
            max_tokens: provider.max_tokens.unwrap_or(4096),
            temperature: provider.temperature.unwrap_or(0.7),
        }
    }

    /// Handles error responses from the OpenAI API.
    async fn handle_error_response(&self, status: u16, response_text: String) -> AIClientError {
        match status {
            401 => AIClientError::InvalidAPIKey,
            429 => AIClientError::RateLimit { retry_after: 60 },
            _ => match serde_json::from_str::<OpenAIError>(&response_text) {
                Ok(error) => AIClientError::APIError {
                    status,
                    message: error.error.message,
                },
                Err(_) => AIClientError::APIError {
                    status,
                    message: response_text,
                },
            },
        }
    }

    /// Validates the generated AI plan.
    fn validate_plan(&self, _plan: &AIPlan) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl AIClient for OpenAIClient {
    /// Generates a plan based on the user prompt using OpenAI's API.
    async fn generate_plan(&self, context: &Context, user_prompt: &str) -> Result<AIPlan> {
        if user_prompt.trim().is_empty() {
            return Err(AIClientError::InvalidPrompt("Empty prompt".to_string()));
        }

        let system_prompt = include_str!("../assets/prompts/system_prompt.txt");

        let full_system_prompt = system_prompt.replace("{context}", &context.to_string());

        let request_body = serde_json::json!({
            "model": self.model,
            "messages": [
                { "role": "system", "content": full_system_prompt },
                { "role": "user", "content": user_prompt }
            ],
            "max_tokens": self.max_tokens,
            "temperature": self.temperature
        });

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await
            .map_err(AIClientError::Network)?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let response_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to retrieve response text".to_string());
            return Err(self.handle_error_response(status, response_text).await);
        }

        let openai_response: OpenAIResponse = response.json().await.map_err(|err| {
            AIClientError::ValidationError(format!("Failed to parse OpenAI response: {}", err))
        })?;

        let content = openai_response
            .choices
            .first()
            .ok_or_else(|| {
                AIClientError::ValidationError("No choices in OpenAI response".to_string())
            })?
            .message
            .content
            .clone();

        let plan: AIPlan = serde_json::from_str(&content).map_err(|err| {
            AIClientError::ValidationError(format!("Failed to parse AIPlan: {}", err))
        })?;

        self.validate_plan(&plan)?;

        Ok(plan)
    }
}
