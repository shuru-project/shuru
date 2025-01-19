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

    #[error("API key not found in the configuration file")]
    ApiKeyNotFound,
}

#[derive(Deserialize)]
pub struct AIConfig {
    pub api_key: String,
}

#[derive(Deserialize)]
pub struct ShuruGlobalConfig {
    pub ai: Option<AIConfig>,
}

impl ShuruGlobalConfig {
    pub fn get_global_config_path() -> Result<std::path::PathBuf, GlobalConfigError> {
        let home_dir = dirs::home_dir().ok_or(GlobalConfigError::UserHomeDirNotFound)?;
        Ok(home_dir.join(".shuru").join("config.toml"))
    }

    pub fn load() -> Result<ShuruGlobalConfig, GlobalConfigError> {
        let config_path = ShuruGlobalConfig::get_global_config_path()?;

        let config_content = std::fs::read_to_string(&config_path)
            .map_err(|err| GlobalConfigError::ConfigReadError(config_path.clone(), err))?;
        let parsed_config: ShuruGlobalConfig = toml::from_str(&config_content)?;

        Ok(parsed_config)
    }
}
