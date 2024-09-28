use shuru::{error::Error, version_manager::VersionManager};

use super::VersionInfo;

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
                    shuru::util::os_type(),
                    shuru::util::get_architecture()
                );

                (version.to_string(), platform)
            }
            VersionInfo::Complex { version, platform } => {
                (version.to_string(), platform.to_string())
            }
        };

        NodeVersionManager { version, platform }
    }

    fn get_download_dir(&self) -> Result<std::path::PathBuf, Error> {
        let version = self.version.trim_start_matches('v');
        let home_dir = dirs::home_dir().ok_or_else(|| {
            Error::VersionManagerError("Unable to find home directory".to_string())
        })?;
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

        println!(
            "Node.js {} downloaded and installed successfully.",
            self.version
        );

        Ok(binary_dir)
    }
}

impl NodeVersionManager {
    fn download_node_archive(&self, download_file_path: &std::path::Path) -> Result<(), Error> {
        let url = self.get_download_url();
        println!("Downloading Node.js {} from {}...", self.version, url);

        let response = reqwest::blocking::get(&url).map_err(|e| {
            Error::VersionManagerError(format!("Failed to download Node.js: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(Error::VersionManagerError(format!(
                "Failed to download Node.js, status: {}",
                response.status()
            )));
        }

        let mut file = std::fs::File::create(download_file_path).map_err(|e| {
            Error::VersionManagerError(format!("Failed to create download file: {}", e))
        })?;

        std::io::copy(&mut response.bytes()?.as_ref(), &mut file)
            .map_err(|e| Error::VersionManagerError(format!("Failed to write to file: {}", e)))?;

        println!("Download complete.");
        Ok(())
    }

    fn extract_archive(
        &self,
        download_file_path: &std::path::Path,
        download_dir: &std::path::Path,
    ) -> Result<(), Error> {
        println!("Extracting Node.js version {}...", self.version);
        shuru::util::extract_tar_gz(download_file_path, download_dir)
            .map_err(|e| Error::VersionManagerError(format!("Failed to extract archive: {}", e)))
    }

    fn cleanup_downloaded_archive(
        &self,
        download_file_path: &std::path::Path,
    ) -> Result<(), Error> {
        println!("Cleaning up the downloaded archive...");
        std::fs::remove_file(download_file_path).map_err(|e| {
            Error::VersionManagerError(format!("Failed to remove downloaded archive: {}", e))
        })
    }
}
