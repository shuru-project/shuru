use shuru::{error::Error, version_manager::VersionManager};

#[derive(Debug)]
pub struct NodeVersionManager;

impl VersionManager for NodeVersionManager {
    fn download(&self, version: &str, arch: &str) -> Result<std::path::PathBuf, Error> {
        if self.command_exists(version) {
            return self.get_command_dir(version);
        }

        let trimmed_version = version.trim_start_matches('v');
        let os_type = shuru::util::os_type();

        let url = format!(
            "https://nodejs.org/dist/v{}/node-v{}-{}-{}.tar.gz",
            trimmed_version, trimmed_version, os_type, arch
        );

        let home_dir = dirs::home_dir().ok_or_else(|| {
            Error::CommandExecutionError("Unable to find home directory".to_string())
        })?;
        let install_dir = home_dir.join(format!(".shuru/node/{}", trimmed_version));

        std::fs::create_dir_all(&install_dir)?;

        let download_path = install_dir.join("node.tar.gz");

        println!(
            "Downloading Node.js version {} from {}...",
            trimmed_version, url
        );

        let response = reqwest::blocking::get(&url).map_err(|e| {
            // Cleanup if the directory was created but download failed
            let _ = std::fs::remove_dir_all(&install_dir);
            Error::CommandExecutionError(format!("Failed to download Node.js: {}", e))
        })?;

        println!("Response Status: {}", response.status());

        if !response.status().is_success() {
            // Cleanup if the download was not successful
            let _ = std::fs::remove_dir_all(&install_dir);
            return Err(Error::CommandExecutionError(format!(
                "Failed to download Node.js, status: {}",
                response.status()
            )));
        }

        let mut file = std::fs::File::create(&download_path).map_err(|e| {
            let _ = std::fs::remove_dir_all(&install_dir);
            Error::CommandExecutionError(format!("Failed to create file for download: {}", e))
        })?;

        std::io::copy(&mut response.bytes()?.as_ref(), &mut file).map_err(|e| {
            let _ = std::fs::remove_dir_all(&install_dir);
            Error::CommandExecutionError(format!("Failed to write to file: {}", e))
        })?;

        println!("Extracting Node.js version {}...", trimmed_version);
        shuru::util::extract_tar_gz(&download_path, &install_dir)?;

        println!("Cleaning up the downloaded archive...");
        std::fs::remove_file(&download_path).map_err(|e| {
            Error::CommandExecutionError(format!("Failed to remove downloaded archive: {}", e))
        })?;

        println!(
            "Node.js version {} downloaded and installed successfully.",
            trimmed_version
        );

        self.get_command_dir(version)
    }

    fn command_exists(&self, version: &str) -> bool {
        let trimmed_version = version.trim_start_matches('v');
        let home_dir = dirs::home_dir().unwrap();
        let install_dir = home_dir.join(format!(".shuru/node/{}", trimmed_version));
        install_dir.exists()
    }

    fn get_command_dir(&self, version: &str) -> Result<std::path::PathBuf, Error> {
        let trimmed_version = version.trim_start_matches('v');
        let os_type = shuru::util::os_type();
        let arch = shuru::util::get_architecture();

        let command_path = format!(
            ".shuru/node/{}/node-v{}-{}-{}/bin",
            trimmed_version, trimmed_version, os_type, arch
        );

        let home_dir = dirs::home_dir().ok_or_else(|| {
            Error::CommandExecutionError("Unable to find home directory".to_string())
        })?;

        Ok(home_dir.join(command_path))
    }
}
