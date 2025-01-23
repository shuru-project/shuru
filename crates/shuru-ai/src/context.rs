use shuru_core::{config::Config, error::ContextError};
use std::fmt;
use std::path::PathBuf;

pub struct Context {
    pub work_dir: PathBuf,
    pub config: Option<Config>,
    pub npm_client: String,
}

impl Context {
    pub fn new(work_dir: PathBuf, config: Option<Config>, npm_client: String) -> Self {
        Self {
            work_dir,
            config,
            npm_client,
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

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = format!("Work Directory: {}\n", self.work_dir.display());

        if let Some(config) = &self.config {
            result.push_str("Configuration:\n");

            if let Some(node_version) = config
                .versions
                .get(&shuru_core::version_config::VersionedCommand::Node)
            {
                result.push_str(&format!("  - Node.js version: {}\n", node_version));
            }

            if !config.tasks.is_empty() {
                result.push_str("  - Existing project tasks (for reference only):\n");
                for (task_name, task_config) in &config.tasks {
                    result.push_str(&format!(
                        "    - {}:\n        command: {}\n        description: \"{}\"\n",
                        task_name,
                        task_config.command,
                        task_config.description.as_deref().unwrap_or("")
                    ));
                }
            }
        } else {
            result.push_str("Configuration: None\n");
        }

        write!(f, "{}", result)
    }
}
