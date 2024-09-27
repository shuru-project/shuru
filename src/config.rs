use serde::Deserialize;
use shuru::version_manager::{deserialize_versions, VersionedCommand};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct TaskConfig {
    pub name: String,
    pub command: String,
    pub default: Option<bool>,
    pub aliases: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "task")]
    pub tasks: Vec<TaskConfig>,
    #[serde(default, deserialize_with = "deserialize_versions")]
    pub versions: HashMap<VersionedCommand, String>,
}
