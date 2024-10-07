use flate2::read::GzDecoder;
use shuru::error::Error;
use std::fs::File;
use std::path::Path;
use tar::Archive;

const EXIT_CONFIG_ERROR: i32 = 166;
const EXIT_CONFIG_FILE_NOT_FOUND: i32 = 167;
const EXIT_CONFIG_LOAD_ERROR: i32 = 168;

const EXIT_COMMAND_ERROR: i32 = 170;
const EXIT_COMMAND_NOT_FOUND: i32 = 171;

const EXIT_VERSION_MANAGER_ERROR: i32 = 175;
const EXIT_NO_DEFAULT_COMMAND_FOUND: i32 = 180;

pub fn get_error_code(error: Error) -> i32 {
    match error {
        Error::ConfigLoadError(_) => EXIT_CONFIG_LOAD_ERROR,
        Error::ConfigValidationError(_) => EXIT_CONFIG_ERROR,
        Error::ConfigFileNotFound => EXIT_CONFIG_FILE_NOT_FOUND,
        Error::CommandExecutionError(_) => EXIT_COMMAND_ERROR,
        Error::CommandNotFound(_) => EXIT_COMMAND_NOT_FOUND,
        Error::VersionManagerError(_) => EXIT_VERSION_MANAGER_ERROR,
        Error::DefaultCommandNotFound => EXIT_NO_DEFAULT_COMMAND_FOUND,
        _ => 1,
    }
}

pub fn os_type() -> &'static str {
    if cfg!(target_os = "macos") {
        "darwin"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "win"
    } else {
        panic!("Unsupported OS type")
    }
}

pub fn get_architecture() -> String {
    match std::env::consts::ARCH {
        "x86" => "x86".to_string(),
        "x86_64" => "x64".to_string(),
        "aarch64" => "arm64".to_string(),
        "arm" => "arm".to_string(),
        _ => "unknown".to_string(),
    }
}

pub fn extract_tar_gz<P: AsRef<Path>>(tar_gz_path: P, dest_dir: P) -> Result<(), Error> {
    let tar_gz = File::open(tar_gz_path)
        .map_err(|e| Error::CommandExecutionError(format!("Failed to open tar.gz file: {}", e)))?;

    let mut archive = Archive::new(GzDecoder::new(tar_gz));
    archive.unpack(dest_dir).map_err(|e| {
        Error::CommandExecutionError(format!("Failed to extract tar.gz file: {}", e))
    })?;

    Ok(())
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        println!("\x1b[90m{}\x1b[0m", format!($($arg)*));
    };
}
