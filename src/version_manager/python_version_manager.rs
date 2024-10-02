use std::env;
use std::process::{Command, Stdio};

use shuru::{
    error::Error,
    version_manager::{VersionInfo, VersionManager},
};

#[derive(Debug)]
pub struct PythonVersionManager {
    pub version: String,
}

impl PythonVersionManager {
    pub fn with_version_info(version_info: &VersionInfo) -> Self {
        let version = match version_info {
            VersionInfo::Simple(version) => version.to_string(),
            VersionInfo::Complex { version, .. } => version.to_string(),
        };

        PythonVersionManager { version }
    }

    fn get_download_dir(&self) -> Result<std::path::PathBuf, Error> {
        let home_dir = dirs::home_dir().ok_or_else(|| {
            Error::VersionManagerError("Unable to find home directory".to_string())
        })?;
        Ok(home_dir.join(format!(".shuru/python/{}", self.version)))
    }

    fn get_install_dir(&self, download_dir: &std::path::Path) -> Result<std::path::PathBuf, Error> {
        Ok(download_dir.join("install"))
    }

    fn get_download_url(&self) -> String {
        format!(
            "https://www.python.org/ftp/python/{}/Python-{}.tgz",
            self.version, self.version
        )
    }
}

impl VersionManager for PythonVersionManager {
    fn install_and_get_binary_path(&self) -> Result<std::path::PathBuf, Error> {
        let download_dir = self.get_download_dir()?;
        let install_dir = self.get_install_dir(&download_dir)?;
        let binary_dir = install_dir.join("bin");

        if binary_dir.exists() {
            return Ok(binary_dir);
        }

        std::fs::create_dir_all(&download_dir)?;

        let download_file_path = download_dir.join("python.tgz");
        self.download_python_archive(&download_file_path)?;

        self.extract_archive(&download_file_path, &download_dir)?;

        self.build_python(&download_dir, &install_dir)?;

        self.cleanup_downloaded_archive(&download_file_path)?;

        shuru::log!(
            "Python {} downloaded and installed successfully.",
            self.version
        );

        Ok(binary_dir)
    }
}

impl PythonVersionManager {
    fn download_python_archive(&self, download_file_path: &std::path::Path) -> Result<(), Error> {
        let url = self.get_download_url();
        shuru::log!("Downloading Python {} from {}...", self.version, url);

        let response = reqwest::blocking::get(&url)
            .map_err(|e| Error::VersionManagerError(format!("Failed to download Python: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::VersionManagerError(format!(
                "Failed to download Python, status: {}",
                response.status()
            )));
        }

        let mut file = std::fs::File::create(download_file_path).map_err(|e| {
            Error::VersionManagerError(format!("Failed to create download file: {}", e))
        })?;

        std::io::copy(&mut response.bytes()?.as_ref(), &mut file)
            .map_err(|e| Error::VersionManagerError(format!("Failed to write to file: {}", e)))?;

        shuru::log!("Download complete.");
        Ok(())
    }

    fn extract_archive(
        &self,
        download_file_path: &std::path::Path,
        download_dir: &std::path::Path,
    ) -> Result<(), Error> {
        shuru::log!("Extracting Python version {}...", self.version);
        shuru::util::extract_tar_gz(download_file_path, download_dir)
            .map_err(|e| Error::VersionManagerError(format!("Failed to extract archive: {}", e)))
    }

    fn build_python(
        &self,
        download_dir: &std::path::Path,
        install_dir: &std::path::Path,
    ) -> Result<(), Error> {
        shuru::log!("Building Python {} from source...", self.version);

        let verbose = env::var("SHURU_BUILD_PYTHON_VERBOSE").is_ok();
        let python_source_dir = download_dir.join(format!("Python-{}", self.version));

        self.configure_python(&python_source_dir, install_dir, verbose)?;
        self.compile_python(&python_source_dir, verbose)?;
        self.install_python(&python_source_dir, verbose)?;

        shuru::log!("Python {} built and installed successfully.", self.version);
        Ok(())
    }

    fn configure_python(
        &self,
        source_dir: &std::path::Path,
        install_dir: &std::path::Path,
        verbose: bool,
    ) -> Result<(), Error> {
        shuru::log!("Configuring Python...");

        let mut configure_cmd = Command::new("./configure");
        configure_cmd
            .arg(format!("--prefix={}", install_dir.to_string_lossy()))
            .current_dir(source_dir);

        PythonVersionManager::run_command(&mut configure_cmd, verbose)?;
        Ok(())
    }

    fn compile_python(&self, source_dir: &std::path::Path, verbose: bool) -> Result<(), Error> {
        shuru::log!("Compiling Python...");

        let mut make_cmd = Command::new("make");
        make_cmd.current_dir(source_dir);

        PythonVersionManager::run_command(&mut make_cmd, verbose)?;
        Ok(())
    }

    fn install_python(&self, source_dir: &std::path::Path, verbose: bool) -> Result<(), Error> {
        shuru::log!("Installing Python...");

        let mut make_install_cmd = Command::new("make");
        make_install_cmd.arg("install").current_dir(source_dir);

        PythonVersionManager::run_command(&mut make_install_cmd, verbose)?;
        Ok(())
    }

    fn run_command(cmd: &mut Command, verbose: bool) -> Result<(), Error> {
        if verbose {
            let status = cmd
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .map_err(|e| Error::VersionManagerError(format!("Failed to run command: {}", e)))?;

            if !status.success() {
                return Err(Error::VersionManagerError(
                    "Python build command failed".to_string(),
                ));
            }
        } else {
            let output = cmd
                .output()
                .map_err(|e| Error::VersionManagerError(format!("Failed to run command: {}", e)))?;

            if !output.status.success() {
                let error_message = String::from_utf8_lossy(&output.stderr);
                return Err(Error::VersionManagerError(format!(
                    "Python build failed: {}",
                    error_message
                )));
            }
        }
        Ok(())
    }

    fn cleanup_downloaded_archive(
        &self,
        download_file_path: &std::path::Path,
    ) -> Result<(), Error> {
        shuru::log!("Cleaning up the downloaded archive...");
        std::fs::remove_file(download_file_path).map_err(|e| {
            Error::VersionManagerError(format!("Failed to remove downloaded archive: {}", e))
        })
    }
}