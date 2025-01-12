use serde::Deserialize;
use shuru::error::ConfigValidationError;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct TaskConfig {
    pub command: String,
    pub dir: Option<String>,
    pub default: Option<bool>,
    #[serde(default)]
    pub depends: Vec<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

impl TaskConfig {
    pub fn validate(&self, task_name: &str) -> Result<(), ConfigValidationError> {
        self.validate_command(task_name)?;
        self.validate_dir(task_name)?;
        Ok(())
    }

    fn validate_command(&self, task_name: &str) -> Result<(), ConfigValidationError> {
        if self.command.is_empty() {
            return Err(ConfigValidationError::EmptyCommandError(
                task_name.to_string(),
            ));
        }
        Ok(())
    }

    fn validate_dir(&self, task_name: &str) -> Result<(), ConfigValidationError> {
        if let Some(dir) = &self.dir {
            if dir.is_empty() {
                return Err(ConfigValidationError::EmptyDirError(task_name.to_string()));
            }
        }
        Ok(())
    }
}
