use shuru::config::Config;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContextError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML serialization/deserialization error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("TOML serialization/deserialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),
}

pub struct Context {
    pub work_dir: PathBuf,
    pub config: Option<Config>,
    pub npm_client: String,
}

impl Context {
    pub fn new(work_dir: PathBuf, config: Option<Config>) -> Self {
        Self {
            work_dir,
            config,
            npm_client: "npm".to_string(),
        }
    }

    pub fn ensure_config_file(&mut self) -> Result<(Config, PathBuf), ContextError> {
        let config_path = self.work_dir.join("shuru.toml");

        let config = match &self.config {
            Some(config) => config.clone(),
            None => {
                if !config_path.exists() {
                    let config = Config::default();
                    std::fs::write(&config_path, toml::to_string(&config)?).map(|_| {
                        self.config = Some(config.clone());
                    })?;
                    config
                } else {
                    let content = std::fs::read_to_string(&config_path)?;
                    let config: Config = toml::from_str(&content)?;
                    self.config = Some(config.clone());
                    config
                }
            }
        };

        Ok((config, config_path))
    }
}
