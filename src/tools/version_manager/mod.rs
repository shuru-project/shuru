use shuru::error::Error;

mod version_validator;

pub use version_validator::VersionValidator;

mod node_version_manager;
pub use node_version_manager::NodeVersionManager;

mod python_version_manager;
pub use python_version_manager::PythonVersionManager;

mod versioned_command;
pub use versioned_command::{deserialize_versions, VersionInfo, VersionedCommand};

pub trait VersionManager {
    fn install_and_get_binary_path(&self) -> Result<std::path::PathBuf, Error>;
}

#[derive(Debug)]
pub enum ShuruVersionManager {
    Node(NodeVersionManager),
    Python(PythonVersionManager),
}

impl ShuruVersionManager {
    pub fn install_and_get_binary_path(&self) -> Result<std::path::PathBuf, Error> {
        match self {
            ShuruVersionManager::Node(manager) => manager.install_and_get_binary_path(),
            ShuruVersionManager::Python(manager) => manager.install_and_get_binary_path(),
        }
    }
}
