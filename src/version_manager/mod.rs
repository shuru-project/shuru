use shuru::error::Error;

mod node_version_manager;
pub use node_version_manager::NodeVersionManager;

mod versioned_command;
pub use versioned_command::{deserialize_versions, VersionInfo, VersionedCommand};

pub trait VersionManager {
    fn download(
        &self,
        version: &str,
        platform: Option<&String>,
    ) -> Result<std::path::PathBuf, Error>;
    fn command_exists(&self, version: &str, platform: Option<&String>) -> bool;
    fn get_command_dir(
        &self,
        version: &str,
        platform: Option<&String>,
    ) -> Result<std::path::PathBuf, Error>;
}

#[derive(Debug)]
pub enum ShuruVersionManager {
    Node(NodeVersionManager),
}

impl ShuruVersionManager {
    pub fn download(
        &self,
        version: &str,
        platform: Option<&String>,
    ) -> Result<std::path::PathBuf, Error> {
        match self {
            ShuruVersionManager::Node(manager) => manager.download(version, platform),
        }
    }

    pub fn command_exists(&self, version: &str, platform: Option<&String>) -> bool {
        match self {
            ShuruVersionManager::Node(manager) => manager.command_exists(version, platform),
        }
    }

    pub fn get_command_dir(
        &self,
        version: &str,
        platform: Option<&String>,
    ) -> Result<std::path::PathBuf, Error> {
        match self {
            ShuruVersionManager::Node(manager) => manager.get_command_dir(version, platform),
        }
    }
}
