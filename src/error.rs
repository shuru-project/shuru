use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigValidationError {
    #[error("Invalid command name: '{0}'. Reason: {1}")]
    CommandNameValidationError(String, String),

    #[error("Command cannot be empty for task: '{0}'.")]
    EmptyCommandError(String),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Configuration loading error: {0}")]
    ConfigLoadError(String),

    #[error("Configuration validation error: {0}")]
    ConfigValidationError(#[from] ConfigValidationError),

    #[error("Configuration file not found.")]
    ConfigFileNotFound,

    #[error("Command execution error: {0}")]
    CommandExecutionError(String),

    #[error("No default command found.")]
    DefaultCommandNotFound,

    #[error("Command '{0}' not found.")]
    CommandNotFound(String),

    #[error("Version manager error: '{0}'")]
    VersionManagerError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
}
