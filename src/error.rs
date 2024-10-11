use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigValidationError {
    #[error("Invalid command name: '{0}'. Reason: {1}")]
    CommandNameValidationError(String, String),

    #[error("Command cannot be empty for task: '{0}'.")]
    EmptyCommandError(String),
}

#[derive(Debug, Error)]
pub enum VersionManagerError {
    #[error("Unable to find home directory")]
    UnableHomeDirectory {},

    #[error("Failed to download version from '{url}' | {error}")]
    DownloadError {
        url: String,
        error: reqwest::Error,
    },

    #[error("Failed to download {package} from '{url}' | Status: {status}")]
    FailedDownloadPackage {
        package: String,
        url: String,
        status: String,
    },

    #[error("Failed to create download file '{file}' | {error}")]
    FailedCreateFile {
        file: String,
        error: std::io::Error
    },

    #[error("Failed to write a compressed file '{file}' | {error}")]
    FailedWriteFile {
        file: String,
        error: reqwest::Error
    },


    #[error("Failed to remove downloaded archive '{file}' | {error}")]
    FailedDeleteFile {
        file: String,
        error: std::io::Error
    },

    #[error("Failed to extract archive '{file}' to '{target}' | {error}")]
    FailedExtractArchive {
        file: String,
        target: String,
        error: String
    },

    #[error("Failed to run command '{command}' | {error}")]
    FailedRunCommand {
        command: String,
        error: std::io::Error
    },

    #[error("{package} build command failed | Exit code: {status}{error}")]
    FailedPackageBuildCommand {
        package: String,
        status: i32,
        error: String
    },
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

    #[error("Version manager error | {0}")]
    VersionManagerError(#[from] VersionManagerError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Unable to find home directory.")]
    HomeDirectoryNotFound,

    #[error("Failed to clear cache directory at '{0}': {1}")]
    CacheClearError(String, #[source] std::io::Error),
}
