use serde::Deserialize;
use shuru::version_manager::{NodeVersionManager, ShuruVersionManager};
use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
pub enum VersionedCommand {
    Node,
}

impl VersionedCommand {
    pub fn get_version_manager(&self, version_info: &VersionInfo) -> ShuruVersionManager {
        match self {
            VersionedCommand::Node => {
                ShuruVersionManager::Node(NodeVersionManager::with_version_info(version_info))
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
    let map: Option<HashMap<String, VersionInfo>> = Option::deserialize(deserializer)?;

    let mut result = HashMap::new();

    if let Some(map) = map {
        for (key, value) in map {
            match key.as_str() {
                "node" => {
                    result.insert(VersionedCommand::Node, value);
                }
                _ => {
                    return Err(serde::de::Error::custom(format!(
                        "Unknown version command: {}",
                        key
                    )));
                }
            }
        }
    }

    Ok(result)
}
