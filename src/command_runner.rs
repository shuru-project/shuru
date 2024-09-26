use std::process::{Command, ExitStatus};

use crate::{
    config::{Config, TaskConfig},
    error::Error,
};

pub struct CommandRunner {
    config: Config,
}

impl CommandRunner {
    pub fn new(config: Config) -> Self {
        CommandRunner { config }
    }

    fn find_task(&self, name: &str) -> Result<&TaskConfig, Error> {
        self.config
            .tasks
            .iter()
            .find(|task| {
                task.name == name
                    || task
                        .aliases
                        .as_ref()
                        .map_or(false, |aliases| aliases.contains(&name.to_string()))
            })
            .ok_or_else(|| Error::CommandNotFound(name.to_string()))
    }

    pub fn run_command(&self, name: &str) -> Result<ExitStatus, Error> {
        let task = self.find_task(name)?;

        let env_path = self.config.versions.iter().try_fold(
            String::new(),
            |env_path, (command_type, version)| {
                let arch = shuru::util::get_architecture();
                let version_manager = command_type.get_version_manager();

                let command_dir = if version_manager.command_exists(version) {
                    version_manager.get_command_dir(version)?
                } else {
                    version_manager.download(version, &arch)?
                };

                Ok::<_, Error>(format!("{}:{}", command_dir.to_string_lossy(), env_path))
            },
        )?;

        let final_env_path = format!("{}{}", env_path, std::env::var("PATH").unwrap_or_default());

        let status = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .env("PATH", final_env_path)
                .args(["/C", &task.command])
                .status()
                .map_err(|e| {
                    Error::CommandExecutionError(format!("Failed to execute command: {}", e))
                })?
        } else {
            let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());

            Command::new(shell)
                .env("PATH", final_env_path)
                .arg("-c")
                .arg(&task.command)
                .status()
                .map_err(|e| {
                    Error::CommandExecutionError(format!("Failed to execute command: {}", e))
                })?
        };

        Ok(status)
    }

    pub fn run_default(&self) -> Result<ExitStatus, Error> {
        if let Some(task) = self
            .config
            .tasks
            .iter()
            .find(|task| task.default.unwrap_or(false))
        {
            self.run_command(&task.name)
        } else {
            Err(Error::DefaultCommandNotFound)
        }
    }
}
