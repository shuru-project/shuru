use serde::Deserialize;
use shuru::config::TaskConfig;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub enum PlanType {
    #[serde(rename = "ProjectSetup")]
    ProjectSetup,
    #[serde(rename = "AddCommand")]
    AddCommand,
    #[serde(rename = "ModifyConfig")]
    ModifyConfig,
    #[serde(rename = "CreateWorkflow")]
    CreateWorkflow,
}

#[derive(Debug, Deserialize)]
pub struct AIPlan {
    pub plan_type: PlanType,
    pub description: String,
    pub actions: Vec<Action>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    CreateFile {
        path: String,
        content: String,
    },
    CreateDirectory {
        path: String,
    },
    InstallPackage {
        name: String,
        version: Option<String>,
        dev: Option<bool>,
    },
    AddShuruCommand {
        name: String,
        command: String,
        description: Option<String>,
    },
    ModifyShuruConfig {
        node_version: Option<String>,
        commands: Option<HashMap<String, TaskConfig>>,
    },
    RunCommand {
        command: String,
        args: Vec<String>,
    },
    ChangeWorkDir {
        path: String,
    },
}
