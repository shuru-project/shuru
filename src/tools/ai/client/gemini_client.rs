use async_trait::async_trait;
use reqwest;
use serde::Deserialize;
use serde_json::json;

use shuru::{
    global_config::ProviderConfig,
    tools::ai::{
        client::ai_client::{AIClientError, Result},
        client::AIClient,
        context::Context,
        plan::AIPlan,
    },
};

#[derive(Debug, Deserialize)]
struct GeminiError {
    error: GeminiErrorDetails,
}

#[derive(Debug, Deserialize)]
struct GeminiErrorDetails {
    message: String,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: Content,
}

#[derive(Debug, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Deserialize)]
struct Part {
    text: String,
}

#[allow(dead_code)]
pub struct GeminiClient {
    api_key: String,
    client: reqwest::Client,
    model: String,
    max_tokens: u32,
    temperature: f32,
}

impl GeminiClient {
    /// Creates a new client with default settings for Gemini.
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            model: "gemini-4".to_string(),
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

    /// Handles error responses from the Gemini API.
    async fn handle_error_response(&self, status: u16, response_text: String) -> AIClientError {
        match status {
            401 => AIClientError::InvalidAPIKey,
            429 => AIClientError::RateLimit { retry_after: 60 },
            _ => match serde_json::from_str::<GeminiError>(&response_text) {
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

    /// Prepares the request body for the Gemini API call.
    fn prepare_request_body(&self, context: &Context, user_prompt: &str) -> serde_json::Value {
        let system_prompt = include_str!("../assets/prompts/system_prompt.txt");

        let full_system_prompt = system_prompt.replace("{context}", &context.to_string());

        json!({
            "contents": [{
                "parts": [{ "text": full_system_prompt }, { "text": format!("USER PROMPT: {}", user_prompt) }]
            }],
            "generationConfig": {
                "response_mime_type": "application/json"
            }
        })
    }
}

#[async_trait]
impl AIClient for GeminiClient {
    /// Generates a plan based on the user prompt using Gemini's API.
    async fn generate_plan(&self, context: &Context, user_prompt: &str) -> Result<AIPlan> {
        if user_prompt.trim().is_empty() {
            return Err(AIClientError::InvalidPrompt("Empty prompt".to_string()));
        }

        let api_endpoint = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={api_key}",
            model = self.model,
            api_key = self.api_key
        );

        let request_body = self.prepare_request_body(context, user_prompt);

        let response = self
            .client
            .post(&api_endpoint)
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(self
                .handle_error_response(
                    response.status().as_u16(),
                    response.text().await.unwrap_or_default(),
                )
                .await);
        }

        let gemini_response: GeminiResponse = response.json().await.map_err(|err| {
            AIClientError::ValidationError(format!("Failed to parse Gemini response: {}", err))
        })?;

        let content = gemini_response
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .ok_or_else(|| {
                AIClientError::ValidationError("No valid content in Gemini response".to_string())
            })?;

        let plan: AIPlan = serde_json::from_str(&content).map_err(|err| {
            AIClientError::ValidationError(format!("Failed to parse AIPlan: {}", err))
        })?;

        self.validate_plan(&plan)?;

        Ok(plan)
    }
}
