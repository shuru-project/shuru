use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigValidationError {
    #[error("Command cannot be empty for task: '{0}'.")]
    EmptyCommandError(String),

    #[error("Directory cannot be empty for task: '{0}'.")]
    EmptyDirError(String),
}

#[derive(Debug, Error)]
pub enum VersionManagerError {
    #[error("Invalid version: {0}")]
    InvalidVersion(String),

    #[error("Unable to find home directory")]
    UnableHomeDirectory,

    #[error("Failed to download version from '{url}' | {source}")]
    DownloadError {
        url: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("Failed to download {package} from '{url}' | Status: {status}")]
    FailedDownloadPackage {
        package: String,
        url: String,
        status: String,
    },

    #[error("Failed to create download file '{file}' | {source}")]
    FailedCreateFile {
        file: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write a compressed file '{file}' | {source}")]
    FailedWriteFile {
        file: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("Failed to remove downloaded archive '{file}' | {source}")]
    FailedDeleteFile {
        file: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to extract archive '{file}' to '{target}' | {error}")]
    FailedExtractArchive {
        file: String,
        target: String,
        error: String,
    },

    #[error("Failed to run command '{command}' | {source}")]
    FailedRunCommand {
        command: String,
        #[source]
        source: std::io::Error,
    },

    #[error("{package} build command failed | Exit code: {status}{error}")]
    FailedPackageBuildCommand {
        package: String,
        status: i32,
        error: String,
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

    #[error("Unable to find home directory.")]
    HomeDirectoryNotFound,

    #[error("Failed to clear cache directory at '{0}': {1}")]
    CacheClearError(String, #[source] std::io::Error),
}
