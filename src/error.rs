use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Configuration loading error: {0}")]
    ConfigLoadError(String),

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
