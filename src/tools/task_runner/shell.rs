use std::process::{Command, Stdio};

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

    pub fn from_env() -> Self {
        match std::env::var("SHELL") {
            Ok(shell) if shell.contains("bash") => Shell::Bash,
            Ok(shell) if shell.contains("fish") => Shell::Fish,
            Ok(shell) if shell.contains("zsh") => Shell::Zsh,
            _ => Shell::Unknown,
        }
    }
}
