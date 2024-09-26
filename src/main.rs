use clap::Parser;
use shuru::{command_runner::CommandRunner, config::Config, error::Error};

#[derive(Parser)]
struct Cli {
    command: Option<String>,
}

fn load_config() -> Result<Config, Error> {
    let config_str = std::fs::read_to_string("shuru.toml")
        .map_err(|e| Error::ConfigLoadError(format!("Unable to read config file: {}", e)))?;

    toml::from_str(&config_str)
        .map_err(|e| Error::ConfigLoadError(format!("Invalid config file format: {}", e)))
}

fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();

    let config = load_config().map_err(|e| {
        eprintln!("Error loading configuration: {}", e);
        e
    })?;

    let runner = CommandRunner::new(config);
    let cli = Cli::parse();

    if let Some(command_name) = cli.command {
        runner.run_command(&command_name)?;
    } else {
        runner.run_default()?;
    }

    Ok(())
}
