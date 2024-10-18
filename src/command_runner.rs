use std::path::PathBuf;
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
        self.run_dependencies(task)?;
        let work_dir = self.resolve_work_directory(task)?;
        let env_path = self.build_env_path()?;
        self.execute_command(task, work_dir, env_path)
    }

    fn run_dependencies(&self, task: &TaskConfig) -> Result<(), Error> {
        for dep in &task.depends {
            self.run_command(dep)?;
        }
        Ok(())
    }

    fn resolve_work_directory(&self, task: &TaskConfig) -> Result<PathBuf, Error> {
        let current_dir = std::env::current_dir().map_err(|e| {
            Error::CommandExecutionError(format!("Failed to get current directory: {}", e))
        })?;

        if let Some(dir) = &task.dir {
            let resolved_dir = current_dir.join(dir);
            self.validate_directory(&resolved_dir, &current_dir)?;
            Ok(resolved_dir)
        } else {
            Ok(current_dir)
        }
    }

    fn validate_directory(
        &self,
        resolved_dir: &PathBuf,
        current_dir: &PathBuf,
    ) -> Result<(), Error> {
        if !resolved_dir.exists() {
            return Err(Error::CommandExecutionError(format!(
                "Specified directory does not exist: '{}'",
                resolved_dir.display()
            )));
        }

        let canonical_dir = std::fs::canonicalize(resolved_dir).map_err(|e| {
            Error::CommandExecutionError(format!(
                "Failed to canonicalize directory '{}': {}",
                resolved_dir.display(),
                e
            ))
        })?;

        if !canonical_dir.starts_with(current_dir) {
            return Err(Error::CommandExecutionError(format!(
                "Invalid directory '{}'. Cannot navigate outside of the current directory.",
                resolved_dir.display()
            )));
        }

        Ok(())
    }

    fn build_env_path(&self) -> Result<String, Error> {
        let env_path = self.config.versions.iter().try_fold(
            String::new(),
            |env_path, (versioned_command, version_info)| {
                let version_manager = versioned_command.get_version_manager(version_info);
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

    fn execute_command(
        &self,
        task: &TaskConfig,
        work_dir: PathBuf,
        env_path: String,
    ) -> Result<ExitStatus, Error> {
        let mut command = if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            cmd.arg("/C");
            cmd
        } else {
            let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
            let mut cmd = Command::new(shell);
            cmd.arg("-c");
            cmd
        };

        command
            .current_dir(work_dir)
            .env("PATH", env_path)
            .envs(&task.env)
            .arg(&task.command);

        command
            .status()
            .map_err(|e| Error::CommandExecutionError(format!("Failed to execute command: {}", e)))
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
