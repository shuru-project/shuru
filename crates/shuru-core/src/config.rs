use serde::{Deserialize, Serialize};

use shuru_core::error::Error;
use shuru_core::task_config::TaskConfig;
use shuru_core::version_config::{deserialize_versions, VersionInfo, VersionedCommand};

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
}
