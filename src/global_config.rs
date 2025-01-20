use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GlobalConfigError {
    #[error("Unable to determine the user's home directory")]
    UserHomeDirNotFound,

    #[error("Failed to read configuration file at `{0}`: {1}")]
    ConfigReadError(std::path::PathBuf, #[source] std::io::Error),

    #[error("Failed to parse configuration file: {0}")]
    ConfigParseError(#[from] toml::de::Error),

    #[error("No default provider configured")]
    NoDefaultProvider,

    #[error("Provider '{0}' not found")]
    ProviderNotFound(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProviderConfig {
    pub api_key: String,
    pub model: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct AIConfig {
    pub default_provider: String,
    pub providers: std::collections::HashMap<String, ProviderConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ShuruGlobalConfig {
    pub ai: AIConfig,
}

impl ShuruGlobalConfig {
    pub fn get_global_config_path() -> Result<std::path::PathBuf, GlobalConfigError> {
        let home_dir = dirs::home_dir().ok_or(GlobalConfigError::UserHomeDirNotFound)?;
        Ok(home_dir.join(".shuru").join("config.toml"))
    }

    pub fn load() -> Result<ShuruGlobalConfig, GlobalConfigError> {
        let config_path = Self::get_global_config_path()?;
        let config_content = std::fs::read_to_string(&config_path)
            .map_err(|err| GlobalConfigError::ConfigReadError(config_path.clone(), err))?;
        let parsed_config: ShuruGlobalConfig = toml::from_str(&config_content)?;
        Ok(parsed_config)
    }

    pub fn get_default_provider(&self) -> Result<&ProviderConfig, GlobalConfigError> {
        let default_provider = &self.ai.default_provider;
        self.ai
            .providers
            .get(default_provider)
            .ok_or_else(|| GlobalConfigError::ProviderNotFound(default_provider.clone()))
    }

    pub fn get_provider(&self, provider_name: &str) -> Result<&ProviderConfig, GlobalConfigError> {
        self.ai
            .providers
            .get(provider_name)
            .ok_or_else(|| GlobalConfigError::ProviderNotFound(provider_name.to_string()))
    }
}
