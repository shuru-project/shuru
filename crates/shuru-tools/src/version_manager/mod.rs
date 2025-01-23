use shuru_core::{
    config::Config,
    error::{Error, VersionManagerError},
    version_config::{VersionInfo, VersionedCommand},
};

pub trait VersionValidator {
    fn validate_version(version: &str) -> Result<(), VersionManagerError>;
}

mod node_version_manager;
pub use node_version_manager::NodeVersionManager;

pub trait VersionManager {
    fn install_and_get_binary_path(&self) -> Result<std::path::PathBuf, Error>;
}

#[derive(Debug)]
pub enum ShuruVersionManager {
    Node(NodeVersionManager),
}

impl ShuruVersionManager {
    pub fn install_and_get_binary_path(&self) -> Result<std::path::PathBuf, Error> {
        match self {
            ShuruVersionManager::Node(manager) => manager.install_and_get_binary_path(),
        }
    }
}

pub trait VersionManagerResolver {
    fn resolve_version_manager(
        &self,
        version_info: &VersionInfo,
    ) -> Result<ShuruVersionManager, Error>;
}

impl VersionManagerResolver for VersionedCommand {
    fn resolve_version_manager(
        &self,
        version_info: &VersionInfo,
    ) -> Result<ShuruVersionManager, Error> {
        let version = version_info.get_version();

        match self {
            VersionedCommand::Node => {
                NodeVersionManager::validate_version(version)?;
                Ok(ShuruVersionManager::Node(
                    NodeVersionManager::with_version_info(version_info),
                ))
            }
        }
    }
}

pub trait EnvPathBuilder {
    fn build_env_path(&self) -> Result<String, Error>;
}

impl EnvPathBuilder for Config {
    fn build_env_path(&self) -> Result<String, Error> {
        let env_path = self.versions.iter().try_fold(
            String::new(),
            |env_path, (versioned_command, version_info)| {
                let version_manager = versioned_command.resolve_version_manager(version_info)?;
                let binary_path = version_manager.install_and_get_binary_path()?;

                Ok::<_, Error>(format!("{}:{}", binary_path.to_string_lossy(), env_path))
            },
        )?;

        Ok(format!(
            "{}{}",
            env_path,
            std::env::var("PATH").unwrap_or_default()
        ))
    }
}
