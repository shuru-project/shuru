use serde::Deserialize;
use shuru::tools::version_manager::{
    NodeVersionManager, PythonVersionManager, ShuruVersionManager,
};
use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
pub enum VersionedCommand {
    Node,
    Python,
}

impl VersionedCommand {
    pub fn get_version_manager(&self, version_info: &VersionInfo) -> ShuruVersionManager {
        match self {
            VersionedCommand::Node => {
                ShuruVersionManager::Node(NodeVersionManager::with_version_info(version_info))
            }
            VersionedCommand::Python => {
                ShuruVersionManager::Python(PythonVersionManager::with_version_info(version_info))
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum VersionInfo {
    Simple(String),
    Complex { version: String, platform: String },
}

pub fn deserialize_versions<'de, D>(
    deserializer: D,
) -> Result<HashMap<VersionedCommand, VersionInfo>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let map: HashMap<String, VersionInfo> = HashMap::deserialize(deserializer)?;

    let mut result = HashMap::new();

    for (key, value) in map {
        match key.as_str() {
            "node" => {
                result.insert(VersionedCommand::Node, value);
            }
            "python" => {
                result.insert(VersionedCommand::Python, value);
            }
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Unknown version command: {}",
                    key
                )));
            }
        }
    }

    Ok(result)
}
