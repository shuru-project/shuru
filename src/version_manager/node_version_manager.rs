use shuru::{error::Error, version_manager::VersionManager};

#[derive(Debug)]
pub struct NodeVersionManager;

impl VersionManager for NodeVersionManager {
    fn download(
        &self,
        version: &str,
        platform: Option<&String>,
    ) -> Result<std::path::PathBuf, Error> {
        if self.command_exists(version, platform) {
            return self.get_command_dir(version, platform);
        }

        let platform = get_platform_value(platform);

        let trimmed_version = version.trim_start_matches('v');

        let url = format!(
            "https://nodejs.org/dist/{}/{}.tar.gz",
            version,
            format_node_version_with_platform(version, &platform)
        );

        let home_dir = dirs::home_dir().ok_or_else(|| {
            Error::CommandExecutionError("Unable to find home directory".to_string())
        })?;
        let install_dir = home_dir.join(get_install_dir_name(version, Some(&platform)));

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

        self.get_command_dir(version, Some(&platform))
    }

    fn command_exists(&self, version: &str, platform: Option<&String>) -> bool {
        let home_dir = dirs::home_dir().unwrap();
        let install_dir = get_install_dir_name(version, platform);
        home_dir.join(install_dir).exists()
    }

    fn get_command_dir(
        &self,
        version: &str,
        platform: Option<&String>,
    ) -> Result<std::path::PathBuf, Error> {
        let trimmed_version = version.trim_start_matches('v');
        let platform = get_platform_value(platform);

        let command_path = format!(
            ".shuru/node/{}/{}/bin",
            trimmed_version,
            format_node_version_with_platform(version, &platform),
        );

        let home_dir = dirs::home_dir().ok_or_else(|| {
            Error::CommandExecutionError("Unable to find home directory".to_string())
        })?;

        Ok(home_dir.join(command_path))
    }
}

fn get_platform_value(platform: Option<&String>) -> String {
    match platform {
        Some(platform) => platform.to_string(),
        None => format!(
            "{}-{}",
            shuru::util::os_type(),
            shuru::util::get_architecture()
        ),
    }
}

// Ex: node-v16.14.0-darwin-arm64
fn format_node_version_with_platform(version: &str, platform: &str) -> String {
    format!("node-{}-{}", version, platform)
}

fn get_install_dir_name(version: &str, platform: Option<&String>) -> String {
    let trimmed_version = version.trim_start_matches('v');

    match platform {
        Some(platform) => format!(
            ".shuru/node/{}/{}",
            trimmed_version,
            format_node_version_with_platform(version, platform)
        ),
        None => format!(".shuru/node/{}/", trimmed_version),
    }
}
