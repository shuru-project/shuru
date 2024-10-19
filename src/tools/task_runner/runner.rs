use std::path::{Path, PathBuf};
use std::process::ExitStatus;

use shuru::{
    config::Config,
    error::Error,
    tools::task_runner::{shell_type::ShellType, TaskConfig},
};

use simsearch::SimSearch;

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

        let matches = self.search_similar_tasks(name);

        if matches.is_empty() {
            return Err(Error::CommandNotFound(name.to_string()));
        }

        let suggestions = self.generate_suggestions(&matches);

        Err(Error::CommandNotFoundWithSuggestions(
            name.to_string(),
            suggestions,
        ))
    }

    fn search_similar_tasks(&self, name: &str) -> Vec<u32> {
        let mut engine: SimSearch<u32> = SimSearch::new();
        let task_keys: Vec<_> = self.config.tasks.keys().collect();

        task_keys.iter().enumerate().for_each(|(id, task_name)| {
            engine.insert(id as u32, task_name);
        });

        engine.search(name)
    }

    fn generate_suggestions(&self, matches: &[u32]) -> String {
        let task_keys: Vec<_> = self.config.tasks.keys().collect();

        matches
            .iter()
            .filter_map(|&index| task_keys.get(index as usize).map(|s| s.to_string()))
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn run_command(&self, name: &str) -> Result<ExitStatus, Error> {
        let task = self.find_task(name)?;
        self.run_dependencies(task)?;
        let work_dir = self.resolve_work_directory(task)?;
        let env_path = self.build_env_path()?;
        let shell_type = ShellType::from_env();
        self.execute_command(task, work_dir, env_path, &shell_type)
    }

    fn run_dependencies(&self, task: &TaskConfig) -> Result<(), Error> {
        for dep in &task.depends {
            self.run_command(dep)?;
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

    fn build_env_path(&self) -> Result<String, Error> {
        let env_path = self.config.versions.iter().try_fold(
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

    fn execute_command(
        &self,
        task: &TaskConfig,
        work_dir: PathBuf,
        env_path: String,
        shell_type: &ShellType,
    ) -> Result<ExitStatus, Error> {
        let venv_activate_path = self.detect_venv(&work_dir, shell_type)?;

        let mut command = shell_type.create_command();

        let full_command = if let Some(activate_path) = venv_activate_path {
            self.build_venv_command(&activate_path, &task.command, shell_type)?
        } else {
            task.command.clone()
        };

        command
            .current_dir(work_dir)
            .env("PATH", env_path)
            .envs(&task.env)
            .arg(&full_command);

        command.status().map_err(|e| {
            Error::CommandExecutionError(format!("Description: Failed to execute command: {}", e))
        })
    }

    fn build_venv_command(
        &self,
        activate_path: &Path,
        task_command: &str,
        shell_type: &ShellType,
    ) -> Result<String, Error> {
        let activate_str = activate_path.to_str().ok_or_else(|| {
            Error::CommandExecutionError(
                "Description: Failed to convert activate path to string".to_string(),
            )
        })?;

        self.shell_command_format(shell_type, activate_str, task_command)
    }

    fn shell_command_format(
        &self,
        shell_type: &ShellType,
        activate_str: &str,
        task_command: &str,
    ) -> Result<String, Error> {
        match shell_type {
            ShellType::Bash | ShellType::Zsh | ShellType::Unknown => {
                Ok(format!("source {} && {}", activate_str, task_command))
            }
            ShellType::Fish => Ok(format!("source {}; and {}", activate_str, task_command)),
            ShellType::PowerShell => Ok(format!("& '{}'; {}", activate_str, task_command)),
        }
    }

    fn detect_venv(
        &self,
        work_dir: &Path,
        shell_type: &ShellType,
    ) -> Result<Option<PathBuf>, Error> {
        let venv_dir = work_dir.join("venv");
        if venv_dir.is_dir() {
            let venv_bin_dir = venv_dir.join("bin");
            let activate_script = match shell_type {
                ShellType::Fish => venv_bin_dir.join("activate.fish"),
                ShellType::PowerShell => venv_bin_dir.join("Activate.ps1"),
                _ => venv_bin_dir.join("activate"),
            };

            if activate_script.is_file() {
                return Ok(Some(activate_script));
            }

            return Err(Error::CommandExecutionError(
                "Description: Virtual environment detected but activate script not found"
                    .to_string(),
            ));
        }

        Ok(None)
    }

    pub fn run_default(&self) -> Result<ExitStatus, Error> {
        self.config
            .tasks
            .iter()
            .find(|(_, task_config)| task_config.default.unwrap_or(false))
            .map(|(task_name, _)| self.run_command(task_name))
            .unwrap_or(Err(Error::DefaultCommandNotFound))
    }
}
