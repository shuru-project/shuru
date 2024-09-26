use flate2::read::GzDecoder;
use shuru::error::Error;
use std::fs::File;
use std::path::Path;
use tar::Archive;

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
