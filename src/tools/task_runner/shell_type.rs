use std::process::Command;

pub enum ShellType {
    Bash,
    Fish,
    PowerShell,
    Zsh,
    Unknown,
}

impl ShellType {
    pub fn create_command(&self) -> Command {
        let (shell_cmd, shell_arg) = match self {
            ShellType::Bash => ("bash", "-c"),
            ShellType::Fish => ("fish", "-c"),
            ShellType::PowerShell => ("powershell", "-Command"),
            ShellType::Zsh => ("zsh", "-c"),
            ShellType::Unknown => ("/bin/sh", "-c"),
        };

        let mut command = Command::new(shell_cmd);
        command.arg(shell_arg);
        command
    }

    pub fn from_env() -> Self {
        if cfg!(target_os = "windows") {
            return ShellType::PowerShell;
        }

        match std::env::var("SHELL") {
            Ok(shell) if shell.contains("bash") => ShellType::Bash,
            Ok(shell) if shell.contains("fish") => ShellType::Fish,
            Ok(shell) if shell.contains("zsh") => ShellType::Zsh,
            _ => ShellType::Unknown,
        }
    }
}
