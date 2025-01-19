use serde::{Deserialize, Serialize};
pub use shuru::{
    shuru::error::Error,
    tools::{
        task_runner::TaskConfig,
        version_manager::{deserialize_versions, VersionInfo, VersionedCommand},
    },
};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub tasks: HashMap<String, TaskConfig>,
    #[serde(default, deserialize_with = "deserialize_versions")]
    pub versions: HashMap<VersionedCommand, VersionInfo>,
}

impl Config {
    pub fn validate_tasks(&self) -> Result<(), Error> {
        for (task_name, task_config) in &self.tasks {
            task_config.validate(task_name)?;
            for dep in &task_config.depends {
                if !self.tasks.contains_key(dep) {
                    return Err(Error::CommandNotFound(dep.to_string()));
                }
            }
        }
        Ok(())
    }

    pub fn build_env_path(&self) -> Result<String, Error> {
        let env_path = self.versions.iter().try_fold(
            String::new(),
            |env_path, (versioned_command, version_info)| {
                let version_manager = versioned_command.get_version_manager(version_info)?;
                let binary_path = version_manager.install_and_get_binary_path()?;

                Ok::<_, Error>(format!("{}:{}", binary_path.to_string_lossy(), env_path))
            },
        )?;

        Ok(format!(
            "{}{}",
            env_path,
            std::env::var("PATH").unwrap_or_default()
        ))
    }
}
