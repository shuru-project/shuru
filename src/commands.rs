use clap::ValueEnum;
use shuru::{config::Config, error::Error};

#[derive(ValueEnum, Clone)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

pub fn generate_completions(shell: Shell) -> Result<std::process::ExitStatus, Error> {
    let completion_script = match shell {
        Shell::Bash => include_str!("completions/bash.sh"),
        Shell::Zsh => include_str!("completions/zsh.sh"),
        Shell::Fish => include_str!("completions/shuru.fish"),
    };
    println!("{}", completion_script);
    std::process::exit(0);
}

pub fn update_versions(config: &Config) -> Result<std::process::ExitStatus, Error> {
    for (versioned_command, version_info) in &config.versions {
        let version_manager = versioned_command.get_version_manager(version_info)?;
        let _ = version_manager.install_and_get_binary_path()?;
    }
    println!("All versioned commands updated successfully.");
    std::process::exit(0);
}

pub fn list_commands(config: Option<Config>) -> Result<std::process::ExitStatus, Error> {
    if let Some(config) = config {
        for task_name in config.tasks.keys() {
            println!("{}", task_name);
        }
    }
    std::process::exit(0);
}

pub fn clear_cache() -> Result<std::process::ExitStatus, Error> {
    let home_dir = dirs::home_dir().ok_or_else(|| Error::HomeDirectoryNotFound)?;
    let cache_dir = home_dir.join(".shuru");

    if cache_dir.exists() {
        std::fs::remove_dir_all(&cache_dir)
            .map_err(|e| Error::CacheClearError(cache_dir.display().to_string(), e))?;
        println!("Successfully cleared cache at {:?}", cache_dir);
    } else {
        println!(
            "Cache directory {:?} does not exist. Nothing to clear.",
            cache_dir
        );
    }
    std::process::exit(0);
}
