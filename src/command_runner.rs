use std::process::{Command, ExitStatus};

use shuru::{
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
            .find_map(|(task_name, task_config)| {
                if task_name == name {
                    Some(task_config)
                } else {
                    None
                }
            })
            .ok_or_else(|| Error::CommandNotFound(name.to_string()))
    }

    pub fn run_command(&self, name: &str) -> Result<ExitStatus, Error> {
        let task = self.find_task(name)?;

        for dep in &task.depends {
            self.run_command(dep)?;
        }

        let current_dir = std::env::current_dir().map_err(|e| {
            Error::CommandExecutionError(format!("Failed to get current directory: {}", e))
        })?;

        let work_dir = if let Some(dir) = &task.dir {
            let resolved_dir = current_dir.join(dir);

            if !resolved_dir.exists() {
                return Err(Error::CommandExecutionError(format!(
                    "Specified directory does not exist: '{}'",
                    resolved_dir.display()
                )));
            }

            let canonical_dir = std::fs::canonicalize(&resolved_dir).map_err(|e| {
                Error::CommandExecutionError(format!(
                    "Failed to canonicalize directory '{}': {}",
                    resolved_dir.display(),
                    e
                ))
            })?;

            if !canonical_dir.starts_with(&current_dir) {
                return Err(Error::CommandExecutionError(format!(
                    "Invalid directory '{}'. Cannot navigate outside of the current directory.",
                    dir
                )));
            }
            canonical_dir
        } else {
            current_dir
        };

        let env_path = self.config.versions.iter().try_fold(
            String::new(),
            |env_path, (versioned_command, version_info)| {
                let version_manager = versioned_command.get_version_manager(version_info);
                let binary_path = version_manager.install_and_get_binary_path()?;

                Ok::<_, Error>(format!("{}:{}", binary_path.to_string_lossy(), env_path))
            },
        )?;

        let final_env_path = format!("{}{}", env_path, std::env::var("PATH").unwrap_or_default());

        let status = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .current_dir(work_dir)
                .env("PATH", final_env_path)
                .args(["/C", &task.command])
                .status()
                .map_err(|e| {
                    Error::CommandExecutionError(format!("Failed to execute command: {}", e))
                })?
        } else {
            let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());

            Command::new(shell)
                .current_dir(work_dir)
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
        if let Some((task_name, _)) = self
            .config
            .tasks
            .iter()
            .find(|(_, task_config)| task_config.default.unwrap_or(false))
        {
            return self.run_command(task_name);
        }

        Err(Error::DefaultCommandNotFound)
    }
}
