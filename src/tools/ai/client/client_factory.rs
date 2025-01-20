use std::sync::Arc;

use shuru::{
    global_config::{GlobalConfigError, ShuruGlobalConfig},
    tools::ai::client::{openai_client::OpenAIClient, AIClient},
};

use super::gemini_client::GeminiClient;

pub struct AIClientFactory {
    config: Arc<ShuruGlobalConfig>,
}

impl AIClientFactory {
    pub fn new(config: ShuruGlobalConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    pub fn create_client(
        &self,
        provider_name: Option<String>,
    ) -> Result<Box<dyn AIClient>, GlobalConfigError> {
        let provider_name = provider_name
            .or_else(|| Some(self.config.ai.default_provider.clone()))
            .ok_or(GlobalConfigError::NoDefaultProvider)?;

        let provider = self
            .config
            .ai
            .providers
            .get(&provider_name)
            .ok_or_else(|| GlobalConfigError::ProviderNotFound(provider_name.clone()))?;

        match provider_name.as_str() {
            "openai" => Ok(Box::new(OpenAIClient::new_with_config(provider))),
            // "anthropic" => Ok(Box::new(AnthropicClient::new_with_config(provider.clone()))),
            "gemini" => Ok(Box::new(GeminiClient::new_with_config(provider))),
            // "ollama" => Ok(Box::new(OllamaClient::new_with_config(provider.clone()))),
            _ => Err(GlobalConfigError::ProviderNotFound(provider_name)),
        }
    }
}
