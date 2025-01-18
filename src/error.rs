use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigValidationError {
    #[error("Description: Command cannot be empty for task: '{0}'.")]
    EmptyCommandError(String),

    #[error("Description: Directory cannot be empty for task: '{0}'.")]
    EmptyDirError(String),
}

#[derive(Debug, Error)]
pub enum VersionManagerError {
    #[error("Invalid version\n    Description: {0}")]
    InvalidVersion(String),

    #[error("Description: Unable to find home directory")]
    UnableHomeDirectory {},

    #[error("Description: Failed to download version from '{url}'\n    Technical: {source}")]
    DownloadError {
        url: String,
        #[source]
        source: reqwest::Error,
    },

    #[error(
        "Description: Failed to download {package} from '{url}'\n    Technical: Status: {status}"
    )]
    FailedDownloadPackage {
        package: String,
        url: String,
        status: String,
    },

    #[error("Description: Failed to create download file '{file}'\n    Technical: {source}")]
    FailedCreateFile {
        file: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Description: Failed to write a compressed file '{file}'\n    Technical: {source}")]
    FailedWriteFile {
        file: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("Description: Failed to remove downloaded archive '{file}'\n    Technical: {source}")]
    FailedDeleteFile {
        file: String,
        #[source]
        source: std::io::Error,
    },

    #[error(
        "Description: Failed to extract archive '{file}' to '{target}'\n    Technical: {error}"
    )]
    FailedExtractArchive {
        file: String,
        target: String,
        error: String,
    },

    #[error("Description: Failed to run command '{command}'\n    Technical: {source}")]
    FailedRunCommand {
        command: String,
        #[source]
        source: std::io::Error,
    },

    #[error(
        "Description: {package} build command failed\n    Technical: Exit code: {status}{error}"
    )]
    FailedPackageBuildCommand {
        package: String,
        status: i32,
        error: String,
    },
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Configuration loading error\n    {0}")]
    ConfigLoadError(String),

    #[error("Configuration validation error\n    {0}")]
    ConfigValidationError(#[from] ConfigValidationError),

    #[error("Configuration file not found.")]
    ConfigFileNotFound,

    #[error("Command execution error\n    {0}")]
    CommandExecutionError(String),

    #[error("No default command found.")]
    DefaultCommandNotFound,

    #[error("Command '{0}' not found.")]
    CommandNotFound(String),

    #[error("Command '{0}' not found. Did you mean: {1}?")]
    CommandNotFoundWithSuggestions(String, String),

    #[error("Version manager error\n    {0}")]
    VersionManagerError(#[from] VersionManagerError),

    #[error("IO error\n    Technical: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unable to find home directory.")]
    HomeDirectoryNotFound,

    #[error("Failed to clear cache directory at '{0}'\n    Technical: {1}")]
    CacheClearError(String, #[source] std::io::Error),

    #[error("Shuru AI Error: {0}")]
    AIReplError(#[from] shuru::tools::ai::repl::ReplError),
}
