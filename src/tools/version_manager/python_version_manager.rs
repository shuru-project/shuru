use std::env;
use std::process::{Command, Stdio};

use shuru::{
    error::{Error, VersionManagerError},
    tools::version_manager::{VersionInfo, VersionManager, VersionValidator},
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

    fn get_download_dir(&self) -> Result<std::path::PathBuf, VersionManagerError> {
        let home_dir = match dirs::home_dir() {
            Some(path) => path,
            None => return Err(VersionManagerError::UnableHomeDirectory {}),
        };
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
    fn download_python_archive(
        &self,
        download_file_path: &std::path::Path,
    ) -> Result<(), VersionManagerError> {
        let url = self.get_download_url();
        shuru::log!("Downloading Python {} from {}...", self.version, url);

        let mut response = match reqwest::blocking::get(&url) {
            Ok(response) => response,
            Err(source) => return Err(VersionManagerError::DownloadError { url, source }),
        };

        if !response.status().is_success() {
            return Err(VersionManagerError::FailedDownloadPackage {
                package: "Python".to_string(),
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

        shuru::log!("Download complete.");
        Ok(())
    }

    fn extract_archive(
        &self,
        download_file_path: &std::path::Path,
        download_dir: &std::path::Path,
    ) -> Result<(), VersionManagerError> {
        shuru::log!("Extracting Python version {}...", self.version);
        shuru::util::extract_tar_gz(download_file_path, download_dir).map_err(|error| {
            VersionManagerError::FailedExtractArchive {
                file: download_file_path.to_string_lossy().to_string(),
                target: download_dir.to_string_lossy().to_string(),
                error: error.to_string(),
            }
        })
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
        self.link_names_python(install_dir, verbose)?;

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

    fn link_names_python(&self, install_dir: &std::path::Path, verbose: bool) -> Result<(), Error> {
        shuru::log!("Creating link names for Python...");
        let binary_dir = format!("{}/bin", install_dir.to_string_lossy());

        let mut link_name_python_cmd = Command::new("ln");
        link_name_python_cmd
            .arg("-s")
            .arg("python3")
            .arg("python")
            .current_dir(&binary_dir);

        PythonVersionManager::run_command(&mut link_name_python_cmd, verbose)?;

        let mut link_name_python_config_cmd = Command::new("ln");
        link_name_python_config_cmd
            .arg("-s")
            .arg("python3-config")
            .arg("python-config")
            .current_dir(binary_dir);

        PythonVersionManager::run_command(&mut link_name_python_config_cmd, verbose)?;

        Ok(())
    }

    fn run_command(cmd: &mut Command, verbose: bool) -> Result<(), VersionManagerError> {
        if verbose {
            let status = match cmd
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
            {
                Ok(status) => status,
                Err(source) => {
                    return Err(VersionManagerError::FailedRunCommand {
                        command: format!("{:?}", cmd),
                        source,
                    })
                }
            };

            if !status.success() {
                return Err(VersionManagerError::FailedPackageBuildCommand {
                    package: "Python".to_string(),
                    status: status.code().unwrap(),
                    error: "".to_string(),
                });
            }
        } else {
            let output = match cmd.output() {
                Ok(output) => output,
                Err(source) => {
                    return Err(VersionManagerError::FailedRunCommand {
                        command: format!("{:?}", cmd),
                        source,
                    })
                }
            };

            if !output.status.success() {
                let error_message = String::from_utf8_lossy(&output.stderr).to_string();
                return Err(VersionManagerError::FailedPackageBuildCommand {
                    package: "Python".to_string(),
                    status: output.status.code().unwrap(),
                    error: format!(" > {}", error_message),
                });
            }
        }
        Ok(())
    }

    fn cleanup_downloaded_archive(
        &self,
        download_file_path: &std::path::Path,
    ) -> Result<(), VersionManagerError> {
        shuru::log!("Cleaning up the downloaded archive...");
        std::fs::remove_file(download_file_path).map_err(|source| {
            VersionManagerError::FailedDeleteFile {
                file: download_file_path.to_string_lossy().to_string(),
                source,
            }
        })
    }
}

impl VersionValidator for PythonVersionManager {
    fn validate_version(version: &str) -> Result<(), VersionManagerError> {
        let parts: Vec<&str> = version.split('.').collect();

        if !parts.iter().all(|part| part.chars().all(char::is_numeric)) {
            return Err(VersionManagerError::InvalidVersion(format!(
                "Invalid Python version format: {}. Hint: All parts must be numeric (e.g., 3.10.0).",
                version
            )));
        }

        if parts.len() != 3 {
            let hint = match parts.len() {
                1 => "Please include minor and patch versions (e.g., 3.10.0).",
                2 => "Please include the patch version (e.g., 3.10.0).",
                _ => "Please use the format major.minor.patch (e.g., 3.10.0).",
            };

            return Err(VersionManagerError::InvalidVersion(format!(
                "Invalid Python version format: {}. Hint: {}",
                version, hint
            )));
        }

        Ok(())
    }
}
