use async_trait::async_trait;
use reqwest;
use serde::Deserialize;

use shuru::tools::ai::{
    client::ai_client::{AIClientError, Result},
    client::AIClient,
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
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    async fn handle_error_response(&self, status: u16, response_text: String) -> AIClientError {
        match status {
            401 => AIClientError::InvalidAPIKey,
            429 => {
                // Parse retry-after header if available
                AIClientError::RateLimit {
                    retry_after: 60, // Default to 60 seconds if not specified
                }
            }
            _ => {
                // Try to parse OpenAI error format
                match serde_json::from_str::<OpenAIError>(&response_text) {
                    Ok(error) => AIClientError::APIError {
                        status,
                        message: error.error.message,
                    },
                    Err(_) => AIClientError::APIError {
                        status,
                        message: response_text,
                    },
                }
            }
        }
    }

    fn validate_plan(&self, _plan: &AIPlan) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl AIClient for OpenAIClient {
    async fn generate_plan(&self, user_prompt: &str) -> Result<AIPlan> {
        if user_prompt.trim().is_empty() {
            return Err(AIClientError::InvalidPrompt("Empty prompt".to_string()));
        }

        let system_prompt = include_str!("../prompts/system_prompt.txt");

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "model": "gpt-4o-mini",
                "messages": [
                    {
                        "role": "system",
                        "content": system_prompt
                    },
                    {
                        "role": "user",
                        "content": user_prompt
                    }
                ]
            }))
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

        // Deserialize response into OpenAIResponse struct
        let openai_response: OpenAIResponse = response.json().await.map_err(|err| {
            AIClientError::ValidationError(format!("Failed to parse OpenAI response: {}", err))
        })?;

        // Extract the content field
        let content = openai_response
            .choices
            .first()
            .ok_or_else(|| {
                AIClientError::ValidationError("No choices in OpenAI response".to_string())
            })?
            .message
            .content
            .clone();

        // Deserialize content into AIPlan
        let plan: AIPlan = serde_json::from_str(&content).map_err(|err| {
            AIClientError::ValidationError(format!("Failed to parse AIPlan: {}", err))
        })?;

        // Validate the generated plan
        self.validate_plan(&plan)?;

        Ok(plan)
    }
}
