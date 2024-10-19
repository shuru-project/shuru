use shuru::{
    config::Config,
    tools::version_manager::{VersionInfo, VersionedCommand},
};

fn get_nvmrc_version() -> Option<VersionInfo> {
    if let Ok(version_str) = std::fs::read_to_string(".nvmrc") {
        let version = version_str.trim();
        if version.is_empty() {
            return None;
        }

        // Ensure version starts with 'v' as per the requirement
        let version = if version.starts_with('v') {
            version.to_string()
        } else {
            format!("v{}", version)
        };

        Some(VersionInfo::Simple(version))
    } else {
        None
    }
}

fn get_python_version() -> Option<VersionInfo> {
    if let Ok(version_str) = std::fs::read_to_string(".python-version") {
        let version = version_str.trim();
        if version.is_empty() {
            return None;
        }

        Some(VersionInfo::Simple(version.to_string()))
    } else {
        None
    }
}

pub fn update_versions_from_files(config: &mut Config) {
    if let Some(node_version) = get_nvmrc_version() {
        shuru::log!("Detected Node.js version from .nvmrc: {}. You can add this to shuru.toml under [versions] as `node = \"{}\"`", node_version.get_version(), node_version.get_version());

        config.versions.insert(VersionedCommand::Node, node_version);
    }

    if let Some(python_version) = get_python_version() {
        shuru::log!("Detected Python version from .python-version: {}. You can add this to shuru.toml under [versions] as `python = \"{}\"`", python_version.get_version(), python_version.get_version());

        config
            .versions
            .insert(VersionedCommand::Python, python_version);
    }
}
