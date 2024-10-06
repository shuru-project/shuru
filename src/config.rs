use serde::Deserialize;
use shuru::{
    error::{ConfigValidationError, Error},
    version_manager::{deserialize_versions, VersionedCommand},
};
use std::collections::{HashMap, HashSet};

use crate::version_manager::VersionInfo;

#[derive(Debug, Deserialize)]
pub struct TaskConfig {
    pub name: String,
    pub command: String,
    pub default: Option<bool>,
    pub aliases: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "task", default)]
    pub tasks: Vec<TaskConfig>,
    #[serde(default, deserialize_with = "deserialize_versions")]
    pub versions: HashMap<VersionedCommand, VersionInfo>,
}

impl TaskConfig {
    pub fn validate(&self) -> Result<(), ConfigValidationError> {
        self.validate_name()?;

        self.validate_command()?;

        Ok(())
    }

    fn validate_name(&self) -> Result<(), ConfigValidationError> {
        if self.name.is_empty() {
            return Err(ConfigValidationError::CommandNameValidationError(
                self.name.clone(),
                "Command name cannot be empty.".to_string(),
            ));
        }

        let mut chars = self.name.chars();
        if let Some(first_char) = chars.next() {
            if !first_char.is_alphabetic() && first_char != '_' {
                return Err(ConfigValidationError::CommandNameValidationError(
                    self.name.clone(),
                    "Must start with a letter or underscore.".to_string(),
                ));
            }
        }

        for c in chars {
            if !c.is_alphanumeric() && c != '_' && c != '-' {
                return Err(ConfigValidationError::CommandNameValidationError(
                    self.name.clone(),
                    "Only letters, digits, underscores, and hyphens are allowed.".to_string(),
                ));
            }
        }

        Ok(())
    }

    fn validate_command(&self) -> Result<(), ConfigValidationError> {
        if self.command.is_empty() {
            return Err(ConfigValidationError::EmptyCommandError(self.name.clone()));
        }
        Ok(())
    }
}

impl Config {
    pub fn validate_tasks(&self) -> Result<(), Error> {
        let mut task_names = HashSet::new();

        for task in &self.tasks {
            task.validate()?;

            if !task_names.insert(task.name.clone()) {
                return Err(ConfigValidationError::CommandNameValidationError(
                    task.name.clone(),
                    "Task name must be unique.".to_string(),
                )
                .into());
            }

            task_names.insert(task.name.clone());
        }

        Ok(())
    }
}
