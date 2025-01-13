use clap::Parser;
use shuru::{commands, config::Config, error::Error, tools::task_runner::TaskRunner};

#[derive(Parser)]
#[clap(version, about = "Shuru task runner", long_about = None)]
struct Cli {
    command: Option<String>,

    #[clap(
        long = "completions",
        help = "The shell to generate completions for (e.g., bash, zsh, fish)"
    )]
    completions: Option<commands::Shell>,

    #[clap(long = "list-commands", help = "List available commands")]
    list_commands: bool,

    #[clap(
        long = "update-versions",
        help = "Update all commands to versions in shuru.toml"
    )]
    update_versions: bool,

    #[clap(long = "clear-cache", help = "Clear all cached versions")]
    clear_cache: bool,

    #[clap(long = "ai", help = "Prompt Shuru AI")]
    ai: bool,
}

fn load_config() -> Result<Config, Error> {
    let config_str = std::fs::read_to_string("shuru.toml").map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => Error::ConfigFileNotFound,
        _ => Error::ConfigLoadError(format!(
            "Description: Unable to read config file\n    Technical: {}",
            e
        )),
    })?;

    let config: Config = toml::from_str(&config_str).map_err(|e| {
        Error::ConfigLoadError(format!(
            "Description: Invalid config file format\n    Technical: {}",
            e
        ))
    })?;

    config.validate_tasks()?;

    Ok(config)
}

fn run() -> Result<std::process::ExitStatus, Error> {
    let cli = Cli::parse();

    if cli.ai {
        return commands::ai();
    }

    if let Some(shell) = cli.completions {
        return commands::generate_completions(shell);
    }

    if cli.list_commands {
        let config = load_config().ok();
        return commands::list_commands(config);
    }

    let config = load_config()?;

    if cli.update_versions {
        return commands::update_versions(&config);
    }

    if cli.clear_cache {
        return commands::clear_cache();
    }

    let runner = TaskRunner::new(config);

    match cli.command {
        Some(command_name) => runner.run_command(&command_name),
        None => runner.run_default(),
    }
}

fn main() {
    dotenvy::dotenv().ok();

    if let Err(e) = run() {
        eprintln!("\x1b[31mError:\x1b[0m {}", e);
        std::process::exit(shuru::utils::core_utils::get_error_code(e));
    }
}
