use std::path::PathBuf;
use std::process::ExitStatus;

use shuru_core::{config::Config, error::Error};

use shuru_tools::{
    task_runner::{shell::Shell, TaskConfig},
    version_manager::EnvPathBuilder,
};

pub struct TaskRunner {
    config: Config,
}

impl TaskRunner {
    pub fn new(config: Config) -> Self {
        TaskRunner { config }
    }

    fn find_task(&self, name: &str) -> Result<&TaskConfig, Error> {
        if let Some(task) = self.config.tasks.get(name) {
            return Ok(task);
        }

        let matches = self.search_similar_tasks(name, 0.5);

        if matches.is_empty() {
            return Err(Error::CommandNotFound(name.to_string()));
        }

        let suggestions = matches.join(", ");

        Err(Error::CommandNotFoundWithSuggestions(
            name.to_string(),
            suggestions,
        ))
    }

    fn search_similar_tasks(&self, name: &str, min_score: f64) -> Vec<String> {
        let task_keys: Vec<String> = self.config.tasks.keys().cloned().collect();

        shuru_core::utils::fuzzy_match::filter_matches(name, task_keys, min_score)
            .iter()
            .map(|(key, _score)| key.to_owned())
            .collect()
    }

    pub fn run_task(&self, name: &str) -> Result<ExitStatus, Error> {
        let task = self.find_task(name)?;
        self.run_dependencies(task)?;
        let work_dir = self.resolve_work_directory(task)?;
        let env_path = self.config.build_env_path()?;
        let shell = Shell::from_env();
        self.execute_command(task, work_dir, &env_path, &shell)
    }

    fn run_dependencies(&self, task: &TaskConfig) -> Result<(), Error> {
        for dep in &task.depends {
            self.run_task(dep)?;
        }
        Ok(())
    }

    fn resolve_work_directory(&self, task: &TaskConfig) -> Result<PathBuf, Error> {
        let current_dir = std::env::current_dir().map_err(|e| {
            Error::CommandExecutionError(format!(
                "Description: Failed to get current directory: {}",
                e
            ))
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
                "Description: Specified directory does not exist: '{}'",
                resolved_dir.display()
            )));
        }

        let canonical_dir = std::fs::canonicalize(resolved_dir).map_err(|e| {
            Error::CommandExecutionError(format!(
                "Description: Failed to canonicalize directory '{}': {}",
                resolved_dir.display(),
                e
            ))
        })?;

        if !canonical_dir.starts_with(current_dir) {
            return Err(Error::CommandExecutionError(format!(
                "Description: Invalid directory '{}'. Cannot navigate outside of the current directory.",
                resolved_dir.display()
            )));
        }

        Ok(())
    }

    fn execute_command(
        &self,
        task: &TaskConfig,
        work_dir: PathBuf,
        env_path: &str,
        shell: &Shell,
    ) -> Result<ExitStatus, Error> {
        let mut command = shell.create_command();

        let task_command_with_args = if let Some((command, args)) = task.command.split_once(' ') {
            let escaped_args: Vec<std::ffi::OsString> = args
                .split_whitespace()
                .map(String::from)
                .map(|arg| shell.escape_argument(&arg))
                .collect();

            format!(
                "{} {}",
                command,
                escaped_args
                    .iter()
                    .map(|arg| arg.to_string_lossy())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        } else {
            task.command.clone()
        };

        command
            .current_dir(work_dir)
            .env("PATH", env_path)
            .envs(&task.env)
            .arg(&task_command_with_args);

        command.status().map_err(|e| {
            Error::CommandExecutionError(format!("Description: Failed to execute command: {}", e))
        })
    }

    pub fn run_default(&self) -> Result<ExitStatus, Error> {
        self.config
            .tasks
            .iter()
            .find(|(_, task_config)| task_config.default.unwrap_or(false))
            .map(|(task_name, _)| self.run_task(task_name))
            .unwrap_or(Err(Error::DefaultCommandNotFound))
    }
}
