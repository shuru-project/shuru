use shuru_core::{
    error::{Error, VersionManagerError},
    version_config::VersionInfo,
};

use shuru_tools::version_manager::{VersionManager, VersionValidator};

#[derive(Debug)]
pub struct NodeVersionManager {
    pub version: String,
    pub platform: String,
}

impl NodeVersionManager {
    pub fn with_version_info(version_info: &VersionInfo) -> Self {
        let (version, platform) = match version_info {
            VersionInfo::Simple(version) => {
                let platform = format!(
                    "{}-{}",
                    shuru_core::utils::os_type(),
                    shuru_core::utils::get_architecture()
                );

                (version.to_string(), platform)
            }
            VersionInfo::Complex { version, platform } => {
                (version.to_string(), platform.to_string())
            }
        };

        NodeVersionManager { version, platform }
    }

    fn get_download_dir(&self) -> Result<std::path::PathBuf, VersionManagerError> {
        let version = self.version.trim_start_matches('v');
        let home_dir = match dirs::home_dir() {
            Some(path) => path,
            None => return Err(VersionManagerError::UnableHomeDirectory {}),
        };
        Ok(home_dir.join(format!(".shuru/node/{}", version)))
    }

    fn get_install_dir(&self, download_dir: &std::path::Path) -> Result<std::path::PathBuf, Error> {
        Ok(
            download_dir.join(NodeVersionManager::format_node_version_with_platform(
                &self.version,
                &self.platform,
            )),
        )
    }

    fn get_download_url(&self) -> String {
        format!(
            "https://nodejs.org/dist/{}/{}.tar.gz",
            self.version,
            NodeVersionManager::format_node_version_with_platform(&self.version, &self.platform)
        )
    }

    // Example: node-v16.14.0-darwin-arm64
    fn format_node_version_with_platform(version: &str, platform: &str) -> String {
        format!("node-{}-{}", version, platform)
    }
}

impl VersionManager for NodeVersionManager {
    fn install_and_get_binary_path(&self) -> Result<std::path::PathBuf, Error> {
        let download_dir = self.get_download_dir()?;
        let install_dir = self.get_install_dir(&download_dir)?;
        let binary_dir = install_dir.join("bin");

        if binary_dir.exists() {
            return Ok(binary_dir);
        }

        std::fs::create_dir_all(&download_dir)?;

        let download_file_path = download_dir.join("node.tar.gz");
        self.download_node_archive(&download_file_path)?;

        self.extract_archive(&download_file_path, &download_dir)?;

        self.cleanup_downloaded_archive(&download_file_path)?;

        shuru_core::log!(
            "Node.js {} downloaded and installed successfully.",
            self.version
        );

        Ok(binary_dir)
    }
}

impl NodeVersionManager {
    fn download_node_archive(
        &self,
        download_file_path: &std::path::Path,
    ) -> Result<(), VersionManagerError> {
        let url = self.get_download_url();
        shuru_core::log!("Downloading Node.js {} from {}...", self.version, url);

        let mut response = match reqwest::blocking::get(&url) {
            Ok(response) => response,
            Err(source) => return Err(VersionManagerError::DownloadError { url, source }),
        };

        if !response.status().is_success() {
            return Err(VersionManagerError::FailedDownloadPackage {
                package: "Node.js".to_string(),
                url,
                status: response.status().to_string(),
            });
        }

        let mut file = match std::fs::File::create(download_file_path) {
            Ok(file) => file,
            Err(source) => {
                return Err(VersionManagerError::FailedCreateFile {
                    file: download_file_path.to_string_lossy().to_string(),
                    source,
                })
            }
        };

        response
            .copy_to(&mut file)
            .map_err(|source| VersionManagerError::FailedWriteFile {
                file: download_file_path.to_string_lossy().to_string(),
                source,
            })?;

        shuru_core::log!("Download complete.");
        Ok(())
    }

    fn extract_archive(
        &self,
        download_file_path: &std::path::Path,
        download_dir: &std::path::Path,
    ) -> Result<(), VersionManagerError> {
        shuru_core::log!("Extracting Node.js version {}...", self.version);
        shuru_core::utils::extract_tar_gz(download_file_path, download_dir).map_err(|error| {
            VersionManagerError::FailedExtractArchive {
                file: download_file_path.to_string_lossy().to_string(),
                target: download_dir.to_string_lossy().to_string(),
                error: error.to_string(),
            }
        })
    }

    fn cleanup_downloaded_archive(
        &self,
        download_file_path: &std::path::Path,
    ) -> Result<(), VersionManagerError> {
        shuru_core::log!("Cleaning up the downloaded archive...");
        std::fs::remove_file(download_file_path).map_err(|source| {
            VersionManagerError::FailedDeleteFile {
                file: download_file_path.to_string_lossy().to_string(),
                source,
            }
        })
    }
}

impl VersionValidator for NodeVersionManager {
    fn validate_version(version: &str) -> Result<(), VersionManagerError> {
        let version = version.strip_prefix('v').ok_or_else(|| {
            VersionManagerError::InvalidVersion(format!(
                "Node version must start with 'v'. Provided: {}\n    Hint: Use the format vX.Y.Z (e.g., v14.17.0).",
                version
            ))
        })?;

        let parts: Vec<&str> = version.split('.').collect();

        if parts.len() != 3 || !parts.iter().all(|part| part.chars().all(char::is_numeric)) {
            let hint = match parts.len() {
                1 => "Please include minor and patch versions (e.g., v14.17.0).",
                2 => "Please include the patch version (e.g., v14.17.0).",
                _ => "Please use the format major.minor.patch (e.g., v14.17.0).",
            };

            return Err(VersionManagerError::InvalidVersion(format!(
                "Invalid Node version format: {}\n    Hint: {}",
                version, hint
            )));
        }

        Ok(())
    }
}
