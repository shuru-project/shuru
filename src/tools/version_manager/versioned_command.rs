use serde::{Deserialize, Serialize};
use shuru::{
    error::Error,
    tools::version_manager::{NodeVersionManager, ShuruVersionManager, VersionValidator},
};
use std::collections::HashMap;
use strum::EnumString;

#[derive(Debug, Hash, Eq, PartialEq, Serialize, Deserialize, Clone, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum VersionedCommand {
    Node,
}

impl VersionedCommand {
    pub fn get_version_manager(
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum VersionInfo {
    Simple(String),
    Complex { version: String, platform: String },
}

impl VersionInfo {
    pub fn get_version(&self) -> &str {
        match self {
            VersionInfo::Simple(version) => version,
            VersionInfo::Complex { version, .. } => version,
        }
    }
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
        if value.get_version().is_empty() {
            return Err(serde::de::Error::custom(format!(
                "Missing version information for {}",
                key
            )));
        }

        match key.parse::<VersionedCommand>() {
            Ok(command) => {
                result.insert(command, value);
            }
            Err(_) => {
                return Err(serde::de::Error::custom("Unknown version command"));
            }
        }
    }

    Ok(result)
}
