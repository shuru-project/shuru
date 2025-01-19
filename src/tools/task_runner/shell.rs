use std::process::Command;

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
        command.args(shell_args);
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
