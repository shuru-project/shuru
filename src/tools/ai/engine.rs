use console::style;
use serde::Deserialize;
use shuru::{
    config::{Config, TaskConfig, VersionInfo, VersionedCommand},
    tools::ai::{
        context::{Context, ContextError},
        progress_tracker::ProgressTracker,
    },
};
use std::{collections::HashMap, path::PathBuf, process::Stdio};
use thiserror::Error;
use tokio;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse TOML: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("Failed to serialize TOML: {0}")]
    TomlSerializationFailed(#[from] toml::ser::Error),

    #[error("Failed to execute command: {0}")]
    CommandExecution(String),

    #[error("Context Error: {0}")]
    ContextError(ContextError),

    #[error("Invalid action: {0}")]
    InvalidAction(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("AI service error: {status_code} - {message}")]
    AIService { status_code: u16, message: String },
}

pub type Result<T> = std::result::Result<T, EngineError>;

#[derive(Debug, Deserialize)]
pub enum ActionType {
    #[serde(rename = "ProjectSetup")]
    ProjectSetup,
    #[serde(rename = "AddCommand")]
    AddCommand,
    #[serde(rename = "ModifyConfig")]
    ModifyConfig,
    #[serde(rename = "CreateWorkflow")]
    CreateWorkflow,
}

#[derive(Debug, Deserialize)]
pub struct AIPlan {
    pub action_type: ActionType,
    pub description: String,
    pub actions: Vec<Action>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    CreateFile {
        path: String,
        content: String,
    },
    CreateDirectory {
        path: String,
    },
    InstallPackage {
        name: String,
        version: Option<String>,
        dev: Option<bool>,
    },
    AddShuruCommand {
        name: String,
        command: String,
        description: Option<String>,
    },
    ModifyShuruConfig {
        node_version: Option<String>,
        commands: Option<HashMap<String, TaskConfig>>,
    },
    RunCommand {
        command: String,
        args: Vec<String>,
    },
    ChangeWorkDir {
        path: String,
    },
}

pub struct ActionEngine {
    context: Context,
}

impl ActionEngine {
    pub fn new(context: Context) -> Self {
        Self { context }
    }

    pub async fn execute_action(&mut self, action: &Action) -> Result<()> {
        match action {
            Action::CreateFile { path, content } => {
                println!("📝 Creating file: {}", path);
                self.create_file(path, content)?;
                println!("✓ File created successfully");
                Ok(())
            }
            Action::CreateDirectory { path } => {
                println!("📁 Creating directory: {}", path);
                self.create_directory(path)?;
                println!("✓ Directory created successfully");
                Ok(())
            }
            Action::InstallPackage { name, version, dev } => {
                let pkg_spec = match version {
                    Some(v) => format!("{}@{}", name, v),
                    None => name.to_string(),
                };
                println!(
                    "📦 Installing package: {} ({})",
                    pkg_spec,
                    if dev.unwrap_or(false) { "dev" } else { "prod" }
                );
                self.install_package(name, version, dev.unwrap_or(false))
                    .await?;
                println!("✓ Package installed successfully");
                Ok(())
            }
            Action::AddShuruCommand {
                name,
                command,
                description,
            } => {
                println!("⚡ Adding Shuru command: {}", name);
                self.add_command(name, command, description)?;
                println!("✓ Command added successfully");
                Ok(())
            }
            Action::ModifyShuruConfig {
                node_version,
                commands,
            } => {
                println!("⚙️  Modifying Shuru config");
                self.modify_config(node_version, commands)?;
                println!("✓ Config modified successfully");
                Ok(())
            }
            Action::RunCommand { command, args } => {
                println!("▶️  Running command: {} {}", command, args.join(" "));
                self.run_command(command, args).await?;
                println!("✓ Command completed successfully");
                Ok(())
            }
            Action::ChangeWorkDir { path } => {
                self.change_work_dir(path)?;
                println!(
                    "✓ Changed working directory to {}",
                    self.context.work_dir.display()
                );
                Ok(())
            }
        }
    }

    fn change_work_dir(&mut self, path: &str) -> Result<()> {
        let new_dir = PathBuf::from(path);
        if new_dir.is_absolute() {
            self.context.work_dir = new_dir;
        } else {
            self.context.work_dir = self.context.work_dir.join(new_dir);
        }

        if !self.context.work_dir.exists() {
            std::fs::create_dir_all(&self.context.work_dir).map_err(EngineError::Io)?;
        }

        Ok(())
    }

    fn create_file(&self, path: &str, content: &str) -> Result<()> {
        let full_path = self.context.work_dir.join(path);

        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent).map_err(EngineError::Io)?;
        }

        std::fs::write(full_path, content).map_err(EngineError::Io)
    }

    fn create_directory(&self, path: &str) -> Result<()> {
        std::fs::create_dir_all(self.context.work_dir.join(path)).map_err(EngineError::Io)
    }

    async fn install_package(&self, name: &str, version: &Option<String>, dev: bool) -> Result<()> {
        let mut cmd = tokio::process::Command::new(&self.context.npm_client);
        cmd.current_dir(&self.context.work_dir);

        let install_cmd = match self.context.npm_client.as_str() {
            "yarn" => "add",
            _ => "install",
        };

        cmd.arg(install_cmd);

        if dev {
            match self.context.npm_client.as_str() {
                "yarn" => cmd.arg("--dev"),
                _ => cmd.arg("--save-dev"),
            };
        }

        let package_spec = if let Some(v) = version {
            format!("{}@{}", name, v)
        } else {
            name.to_string()
        };

        cmd.arg(package_spec);

        // Use stdio inheritance for interactive commands
        let mut child = cmd
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit())
            .spawn()
            .map_err(EngineError::Io)?;

        let status = child.wait().await.map_err(EngineError::Io)?;

        if !status.success() {
            return Err(EngineError::CommandExecution(format!(
                "Package installation failed with exit code: {}",
                status.code().unwrap_or(-1)
            )));
        }

        Ok(())
    }

    fn add_command(
        &mut self,
        name: &str,
        command: &str,
        description: &Option<String>,
    ) -> Result<()> {
        let (mut config, config_path) = self
            .context
            .ensure_config_file()
            .map_err(EngineError::ContextError)?;

        config.tasks.insert(
            name.to_string(),
            TaskConfig {
                command: command.to_string(),
                description: description.clone(),
                ..Default::default()
            },
        );

        self.save_config(&config_path, &config)
    }

    fn modify_config(
        &mut self,
        node_version: &Option<String>,
        commands: &Option<HashMap<String, TaskConfig>>,
    ) -> Result<()> {
        let (mut config, config_path) = self
            .context
            .ensure_config_file()
            .map_err(EngineError::ContextError)?;

        if let Some(version) = node_version {
            config
                .versions
                .insert(VersionedCommand::Node, VersionInfo::Simple(version.clone()));
        }

        if let Some(cmds) = commands {
            config.tasks.extend(cmds.clone());
        }

        self.save_config(&config_path, &config)
    }

    fn save_config(&self, path: &PathBuf, config: &Config) -> Result<()> {
        let content =
            toml::to_string_pretty(config).map_err(EngineError::TomlSerializationFailed)?;
        std::fs::write(path, content).map_err(EngineError::Io)
    }

    async fn run_command(&self, command: &str, args: &[String]) -> Result<()> {
        let mut child = tokio::process::Command::new(command)
            .args(args)
            .current_dir(&self.context.work_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit())
            .spawn()
            .map_err(EngineError::Io)?;

        let status = child.wait().await.map_err(EngineError::Io)?;

        if !status.success() {
            return Err(EngineError::CommandExecution(format!(
                "Command failed with exit code: {}",
                status.code().unwrap_or(-1)
            )));
        }

        Ok(())
    }

    pub async fn execute_plan_with_progress(&mut self, plan: AIPlan) -> Result<()> {
        let mut progress = ProgressTracker::new(plan.actions.len());

        for action in plan.actions.iter() {
            let (action_desc, is_interactive) = match action {
                Action::ChangeWorkDir { path, .. } => {
                    (format!("Changing work directory: {}", path), false)
                }
                Action::CreateFile { path, .. } => (format!("Creating file: {}", path), false),
                Action::CreateDirectory { path } => {
                    (format!("Creating directory: {}", path), false)
                }
                Action::InstallPackage { name, .. } => {
                    (format!("Installing package: {}", name), true)
                }
                Action::AddShuruCommand { name, .. } => {
                    (format!("Adding command: {}", name), false)
                }
                Action::ModifyShuruConfig { .. } => ("Modifying configuration".to_string(), false),
                Action::RunCommand { command, args } => {
                    (format!("Running: {} {}", command, args.join(" ")), true)
                }
            };

            progress.update(&action_desc, is_interactive);

            match self.execute_action(action).await {
                Ok(_) => {
                    progress.complete_action(true, None, is_interactive);
                }
                Err(e) => {
                    progress.complete_action(false, Some(&e.to_string()), is_interactive);
                    return Err(e);
                }
            }
        }

        println!(
            "\n{}",
            style("✨ All actions completed successfully!")
                .bold()
                .green()
        );
        Ok(())
    }

    fn _detect_package_manager(&self) -> String {
        if self.context.work_dir.join("yarn.lock").exists() {
            "yarn".to_string()
        } else if self.context.work_dir.join("pnpm-lock.yaml").exists() {
            "pnpm".to_string()
        } else {
            "npm".to_string()
        }
    }
}
