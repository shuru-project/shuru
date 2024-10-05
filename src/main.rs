use clap::{Parser, ValueEnum};
use shuru::{command_runner::CommandRunner, config::Config, error::Error};
use std::os::unix::process::ExitStatusExt as _;

#[derive(ValueEnum, Clone)]
enum Shell {
    Bash,
    Zsh,
    Fish,
}

#[derive(Parser)]
#[clap(version, about = "Shuru task runner", long_about = None)]
struct Cli {
    command: Option<String>,

    #[clap(
        long = "completions",
        help = "The shell to generate completions for (e.g., bash, zsh, fish)"
    )]
    completions: Option<Shell>,

    #[clap(long = "list-commands", help = "List available commands")]
    list_commands: bool,
}

fn load_config() -> Result<Config, Error> {
    let config_str = std::fs::read_to_string("shuru.toml").map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => Error::ConfigFileNotFound,
        _ => Error::ConfigLoadError(format!("Unable to read config file: {}", e)),
    })?;

    let config: Config = toml::from_str(&config_str)
        .map_err(|e| Error::ConfigLoadError(format!("Invalid config file format: {}", e)))?;

    config.validate_tasks()?;

    Ok(config)
}

fn run() -> Result<std::process::ExitStatus, Error> {
    let cli = Cli::parse();
    let config = load_config()?;

    if cli.list_commands {
        for task in &config.tasks {
            println!("{}", task.name);
        }
        return Ok(std::process::ExitStatus::from_raw(0));
    }

    if let Some(shell) = cli.completions {
        let completion_script = match shell {
            Shell::Bash => include_str!("completions/bash.sh"),
            Shell::Zsh => include_str!("completions/zsh.sh"),
            Shell::Fish => include_str!("completions/shuru.fish"),
        };
        println!("{}", completion_script);
        return Ok(std::process::ExitStatus::from_raw(0));
    }

    let runner = CommandRunner::new(config);

    match cli.command {
        Some(command_name) => runner.run_command(&command_name),
        None => runner.run_default(),
    }
}

fn main() {
    dotenvy::dotenv().ok();

    match run() {
        Ok(status) => std::process::exit(status.code().unwrap_or(shuru::util::EXIT_SUCCESS)),
        Err(e) => {
            eprintln!("\x1b[31mError:\x1b[0m {}", e);
            std::process::exit(shuru::util::get_error_code(e));
        }
    }
}
