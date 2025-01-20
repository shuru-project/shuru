use std::{
    ffi::OsString,
    process::{Command, Stdio},
};

use shell_quote::Quote;

pub enum Shell {
    Bash,
    Fish,
    Zsh,
    Unknown,
}

impl Shell {
    pub fn create_command(&self) -> Command {
        let (shell_cmd, shell_args): (&str, &[&str]) = match self {
            Shell::Bash => ("bash", &["-c"]),
            Shell::Fish => ("fish", &["-c"]),
            Shell::Zsh => ("zsh", &["-c"]),
            Shell::Unknown => ("/bin/sh", &["-cu"]),
        };

        let mut command = Command::new(shell_cmd);
        command
            .args(shell_args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit());
        command
    }

    pub fn create_async_command(&self) -> tokio::process::Command {
        let (shell_cmd, shell_args): (&str, &[&str]) = match self {
            Shell::Bash => ("bash", &["-c"]),
            Shell::Fish => ("fish", &["-c"]),
            Shell::Zsh => ("zsh", &["-c"]),
            Shell::Unknown => ("/bin/sh", &["-cu"]),
        };

        let mut command = tokio::process::Command::new(shell_cmd);
        command
            .args(shell_args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit());
        command
    }

    pub fn escape_argument(&self, argument: &str) -> OsString {
        match self {
            Shell::Bash => shell_quote::Bash::quote(argument),
            Shell::Fish => shell_quote::Fish::quote(argument),
            Shell::Zsh => shell_quote::Zsh::quote(argument),
            Shell::Unknown => shell_quote::Sh::quote(argument),
        }
    }

    pub fn from_env() -> Self {
        match std::env::var("SHELL") {
            Ok(shell) if shell.contains("bash") => Shell::Bash,
            Ok(shell) if shell.contains("fish") => Shell::Fish,
            Ok(shell) if shell.contains("zsh") => Shell::Zsh,
            _ => Shell::Unknown,
        }
    }
}
