use clap::Parser;
use shuru::{command_runner::CommandRunner, config::Config, error::Error};

#[derive(Parser)]
struct Cli {
    command: Option<String>,
}

fn load_config() -> Result<Config, Error> {
    let config_str = std::fs::read_to_string("shuru.toml").map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => Error::ConfigFileNotFound,
        _ => Error::ConfigLoadError(format!("Unable to read config file: {}", e)),
    })?;

    toml::from_str(&config_str)
        .map_err(|e| Error::ConfigLoadError(format!("Invalid config file format: {}", e)))
}

fn run() -> Result<std::process::ExitStatus, Error> {
    let config = load_config()?;

    let runner = CommandRunner::new(config);
    let cli = Cli::parse();

    match cli.command {
        Some(command_name) => runner.run_command(&command_name),
        None => runner.run_default(),
    }
}

fn main() {
    match run() {
        Ok(status) => std::process::exit(status.code().unwrap_or(shuru::util::EXIT_SUCCESS)),
        Err(e) => {
            eprintln!("\x1b[31mError:\x1b[0m {}", e);
            std::process::exit(shuru::util::get_error_code(e));
        }
    }
}
